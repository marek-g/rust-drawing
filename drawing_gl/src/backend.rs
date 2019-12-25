extern crate drawing;
extern crate gl;
extern crate glutin;
extern crate std;
extern crate winit;

use crate::backend::winit::dpi::PhysicalSize;
use drawing::composite_operation_state::CompositeOperationState;
use drawing::paint::Paint;

use self::drawing::color::*;
use self::drawing::units::*;
use self::drawing::utils::path::{Bounds, Path};
use self::drawing::Result;
use self::gl::types::*;
use crate::backend::drawing::backend::*;
use crate::pipelines::*;

use std::cell::{Ref, RefCell};

pub struct GlDevice {
    colored_pipeline: Option<ColoredPipeline>,
    textured_pipeline: Option<TexturedPipeline>,
    textured_y8_pipeline: Option<TexturedY8Pipeline>,
    universal_pipeline: Option<UniversalPipeline>,
    aspect_ratio: f32,
}

impl GlDevice {
    pub fn create_window_target(
        &mut self,
        window_builder: winit::window::WindowBuilder,
        events_loop: &winit::event_loop::EventLoop<()>,
        shared_window_target: Option<&GlWindowTarget>,
    ) -> Result<GlWindowTarget> {
        let context_builder = glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 2)))
            .with_vsync(true);

        let windowed_context = if let Some(ref shared_window_target) = shared_window_target {
            if let Some(ref gl_windowed_context) =
                shared_window_target.gl_windowed_context.borrow().as_ref()
            {
                unsafe {
                    context_builder
                        .with_shared_lists(gl_windowed_context.context())
                        .build_windowed(window_builder, &events_loop)
                        .unwrap()
                        .make_current()
                        .unwrap()
                }
            } else {
                unsafe {
                    context_builder
                        .build_windowed(window_builder, &events_loop)
                        .unwrap()
                        .make_current()
                        .unwrap()
                }
            }
        } else {
            unsafe {
                context_builder
                    .build_windowed(window_builder, &events_loop)
                    .unwrap()
                    .make_current()
                    .unwrap()
            }
        };

        // tell gl crate how to forward gl function calls to the driver
        gl::load_with(|symbol| windowed_context.context().get_proc_address(symbol) as *const _);

        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        if self.colored_pipeline.is_none() {
            self.colored_pipeline = Some(ColoredPipeline::new());
        }
        if self.textured_pipeline.is_none() {
            self.textured_pipeline = Some(TexturedPipeline::new());
        }
        if self.textured_y8_pipeline.is_none() {
            self.textured_y8_pipeline = Some(TexturedY8Pipeline::new());
        }
        if self.universal_pipeline.is_none() {
            self.universal_pipeline = Some(UniversalPipeline::new());
        }

        self.aspect_ratio = windowed_context.window().hidpi_factor() as f32;

        Ok(GlWindowTarget {
            gl_windowed_context: RefCell::new(Some(windowed_context)),
            gl_render_target: GlRenderTarget {
                framebuffer_id: 0,
                width: 0,
                height: 0,
                aspect_ratio: self.aspect_ratio,
            },
            colored_pipeline_buffers: self.colored_pipeline.as_ref().unwrap().create_vbo_and_vao(),
            textured_pipeline_buffers: self
                .textured_pipeline
                .as_ref()
                .unwrap()
                .create_vbo_and_vao(),
            textured_y8_pipeline_buffers: self
                .textured_y8_pipeline
                .as_ref()
                .unwrap()
                .create_vbo_and_vao(),
            universal_pipeline_buffers: self
                .universal_pipeline
                .as_ref()
                .unwrap()
                .create_vbo_and_vao(),
        })
    }

    pub fn begin(&mut self, window_target: &GlWindowTarget) -> Result<()> {
        unsafe {
            let context = window_target.gl_windowed_context.replace(None);
            let context = context.unwrap().make_current().unwrap();
            window_target.gl_windowed_context.replace(Some(context));
        }

        self.colored_pipeline
            .as_mut()
            .unwrap()
            .set_buffers(window_target.colored_pipeline_buffers);
        self.textured_pipeline
            .as_mut()
            .unwrap()
            .set_buffers(window_target.textured_pipeline_buffers);
        self.textured_y8_pipeline
            .as_mut()
            .unwrap()
            .set_buffers(window_target.textured_y8_pipeline_buffers);
        self.universal_pipeline
            .as_mut()
            .unwrap()
            .set_buffers(window_target.universal_pipeline_buffers);

        Ok(())
    }

    pub fn end(&mut self, _window_target: &GlWindowTarget) {}

    pub fn set_render_target(&mut self, target: &GlRenderTarget) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, target.framebuffer_id);
            gl::Viewport(0, 0, target.width as GLint, target.height as GLint);
        }
    }

    fn line_native(
        &mut self,
        color: &Color,
        start_point: Point,
        end_point: Point,
        transform: UnknownToDeviceTransform,
    ) {
        let transform = [
            [transform.m11, transform.m12, 0.0, 0.0],
            [transform.m21, transform.m22, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [transform.m31, transform.m32, 0.0, 1.0],
        ];

        let v1 = ColoredVertex::new([start_point.x, start_point.y], *color);
        let v2 = ColoredVertex::new([end_point.x, end_point.y], *color);
        let v3 = ColoredVertex::new([start_point.x, start_point.y], *color);

        if let Some(ref mut pipeline) = self.colored_pipeline {
            pipeline.apply();
            pipeline.set_transform(&transform);
            pipeline.draw_lines(&[v1, v2, v3]);
        }
    }

    fn line_triangulated(
        &mut self,
        color: &Color,
        thickness: DeviceThickness,
        start_point: Point,
        end_point: Point,
        transform: UnknownToDeviceTransform,
    ) {
    }

    fn convert_paint(
        &self,
        paint: &Paint,
        scissor: &Scissor,
        width: f32,
        fringe: f32,
        stroke_thr: f32,
    ) -> FragUniforms {
        let mut frag = FragUniforms {
            scissor_mat: Default::default(),
            paint_mat: Default::default(),
            inner_color: premul_color(paint.inner_color),
            outer_color: premul_color(paint.outer_color),
            scissor_ext: Default::default(),
            scissor_scale: Default::default(),
            extent: Default::default(),
            radius: 0.0,
            feather: 0.0,
            stroke_mult: 0.0,
            stroke_thr,
            tex_type: 0,
            type_: 0,
            //_padding: [0u8; 16],
        };

        if scissor.extent[0] < -0.5 || scissor.extent[1] < -0.5 {
            frag.scissor_ext[0] = 1.0;
            frag.scissor_ext[1] = 1.0;
            frag.scissor_scale[0] = 1.0;
            frag.scissor_scale[1] = 1.0;
        } else {
            frag.scissor_mat = xform_to_3x4(inverse(scissor.xform));
            frag.scissor_ext[0] = scissor.extent[0];
            frag.scissor_ext[1] = scissor.extent[1];
            frag.scissor_scale[0] =
                (scissor.xform[0] * scissor.xform[0] + scissor.xform[2] * scissor.xform[2]).sqrt()
                    / fringe;
            frag.scissor_scale[1] =
                (scissor.xform[1] * scissor.xform[1] + scissor.xform[3] * scissor.xform[3]).sqrt()
                    / fringe;
        }

        frag.extent = [paint.extent[0], paint.extent[1]];
        frag.stroke_mult = (width * 0.5 + fringe * 0.5) / fringe;

        let mut invxform;

        if let Some(img) = paint.image {
            // TODO: handle textures
            // remove below line - it was added to compile with the below code commented out
            invxform = inverse(paint.xform);
        /*if let Some(texture) = self.textures.get(img) {
            if texture.flags.contains(ImageFlags::FLIPY) {
                let m1 = Transform::translate(0.0, frag.extent[1] * 0.5) * paint.xform;
                let m2 = Transform::scale(1.0, -1.0) * m1;
                let m1 = Transform::translate(0.0, -frag.extent[1] * 0.5) * m2;
                invxform = m1.inverse();
            } else {
                invxform = paint.xform.inverse();
            };

            frag.type_ = ShaderType::FillImage as i32;
            match texture.texture_type {
                TextureType::RGBA => {
                    frag.tex_type = if texture.flags.contains(ImageFlags::PREMULTIPLIED) {
                        0
                    } else {
                        1
                    }
                }
                TextureType::Alpha => frag.tex_type = 2,
            }
        }*/
        } else {
            frag.type_ = ShaderType::FillGradient as i32;
            frag.radius = paint.radius;
            frag.feather = paint.feather;
            invxform = inverse(paint.xform);
        }

        frag.paint_mat = xform_to_3x4(invxform);

        frag
    }
}

impl drawing::backend::Device for GlDevice {
    type Texture = GlTexture;
    type RenderTarget = GlRenderTarget;

    fn new() -> Result<Self> {
        Ok(GlDevice {
            colored_pipeline: None,
            textured_pipeline: None,
            textured_y8_pipeline: None,
            universal_pipeline: None,
            aspect_ratio: 1.0f32,
        })
    }

    fn create_texture(
        &mut self,
        memory: Option<&[u8]>,
        width: u16,
        height: u16,
        format: ColorFormat,
        _updatable: bool,
    ) -> Result<Self::Texture> {
        let mut texture_id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
        }

        let (gl_internal_format, gl_type, gl_format) = match format {
            //ColorFormat::RGBA => (gl::RGBA, gl::UNSIGNED_BYTE, gl::RGBA),
            ColorFormat::RGBA => (gl::RGBA, gl::UNSIGNED_INT_8_8_8_8_REV, gl::BGRA),
            ColorFormat::Y8 => (gl::R8, gl::UNSIGNED_BYTE, gl::RED),
        };

        let texture = GlTexture {
            id: texture_id,
            is_owned: true,
            width,
            height,
            gl_format,
            gl_type,
            flipped_y: false,
        };

        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl_internal_format as GLint,
                width as GLsizei,
                height as GLsizei,
                0,
                gl_format,
                gl_type,
                match memory {
                    Some(memory) => memory.as_ptr() as *const GLvoid,
                    None => std::ptr::null(),
                },
            );
        }

        Ok(texture)
    }

    fn create_render_target(
        &mut self,
        width: u16,
        height: u16,
    ) -> Result<(Self::Texture, Self::RenderTarget)> {
        let mut framebuffer_id: GLuint = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut framebuffer_id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer_id);
        }
        let mut texture = self.create_texture(None, width, height, ColorFormat::RGBA, false)?;
        texture.flipped_y = true;
        unsafe {
            gl::FramebufferTexture(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, texture.id, 0);
            let draw_buffers = gl::COLOR_ATTACHMENT0;
            gl::DrawBuffers(1, &draw_buffers);
        }
        Ok((
            texture,
            GlRenderTarget {
                framebuffer_id,
                width,
                height,
                aspect_ratio: self.aspect_ratio,
            },
        ))
    }

    fn clear(&mut self, target: &Self::RenderTarget, color: &Color) {
        self.set_render_target(&target);
        unsafe {
            gl::ClearColor(color[0], color[1], color[2], color[3]);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
        }
    }

    fn triangles_colored(
        &mut self,
        target: &Self::RenderTarget,
        vertices: &[ColoredVertex],
        transform: UnknownToDeviceTransform,
    ) {
        self.set_render_target(&target);
        let transform = [
            [transform.m11, transform.m12, 0.0, 0.0],
            [transform.m21, transform.m22, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [transform.m31, transform.m32, 0.0, 1.0],
        ];

        if let Some(ref mut pipeline) = self.colored_pipeline {
            pipeline.apply();
            pipeline.set_transform(&transform);
            pipeline.draw(&vertices);
        }
    }

    fn triangles_textured(
        &mut self,
        target: &Self::RenderTarget,
        texture: &Self::Texture,
        filtering: bool,
        vertices: &[TexturedVertex],
        transform: UnknownToDeviceTransform,
    ) {
        self.set_render_target(&target);
        unsafe {
            gl::Enable(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
            if filtering {
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            } else {
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
            }
        }

        let transform = [
            [transform.m11, transform.m12, 0.0, 0.0],
            [transform.m21, transform.m22, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [transform.m31, transform.m32, 0.0, 1.0],
        ];

        if let Some(ref mut pipeline) = self.textured_pipeline {
            pipeline.apply();
            pipeline.set_transform(&transform);
            pipeline.set_flipped_y(texture.flipped_y);
            pipeline.draw(&vertices);
        }
    }

    fn triangles_textured_y8(
        &mut self,
        target: &Self::RenderTarget,
        texture: &Self::Texture,
        filtering: bool,
        vertices: &[TexturedY8Vertex],
        transform: UnknownToDeviceTransform,
    ) {
        self.set_render_target(&target);
        unsafe {
            gl::Enable(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
            if filtering {
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            } else {
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
            }
        }

        let transform = [
            [transform.m11, transform.m12, 0.0, 0.0],
            [transform.m21, transform.m22, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [transform.m31, transform.m32, 0.0, 1.0],
        ];

        if let Some(ref mut pipeline) = self.textured_y8_pipeline {
            pipeline.apply();
            pipeline.set_transform(&transform);
            pipeline.set_flipped_y(texture.flipped_y);
            pipeline.draw(&vertices);
        }
    }

    fn line(
        &mut self,
        target: &Self::RenderTarget,
        color: &Color,
        thickness: DeviceThickness,
        start_point: Point,
        end_point: Point,
        transform: UnknownToDeviceTransform,
    ) {
        self.set_render_target(&target);
        // TODO:
        //if thickness == 1.0f32 {
        self.line_native(color, start_point, end_point, transform);
        //} else {
        //self.line_triangulated(color, thickness, start_point, end_point, transform);
        //}
    }

    fn fill(
        &mut self,
        target: &Self::RenderTarget,
        paint: &Paint,
        paths: &[Path],
        bounds: Bounds,
        fringe_width: f32,
        scissor: Scissor,
        composite_operation_state: CompositeOperationState,
        transform: UnknownToDeviceTransform,
    ) {
        self.set_render_target(&target);

        if paths.len() == 1 && paths[0].convex {
            // convex fill
            let mut uniforms =
                self.convert_paint(paint, &scissor, fringe_width, fringe_width, -1.0);

            if let Some(ref mut pipeline) = self.universal_pipeline {
                let transform = [
                    [transform.m11, transform.m12, 0.0, 0.0],
                    [transform.m21, transform.m22, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [transform.m31, transform.m32, 0.0, 1.0],
                ];

                pipeline.apply();
                pipeline.set_transform(&transform);
                //pipeline.set_flipped_y(texture.flipped_y);
                pipeline.apply_frag_uniforms(&uniforms);

                // fill shape
                let fill_vertices = paths[0].get_fill();
                if !fill_vertices.is_empty() {
                    pipeline.draw(&fill_vertices, gl::TRIANGLE_FAN);
                }

                // antialias outline
                let stoke_vertices = paths[0].get_stroke();
                if !stoke_vertices.is_empty() {
                    pipeline.draw(&stoke_vertices, gl::TRIANGLE_STRIP);
                }
            }
        } else {
            let mut uniforms =
                self.convert_paint(paint, &scissor, fringe_width, fringe_width, -1.0);

            if let Some(ref mut pipeline) = self.universal_pipeline {
                let transform = [
                    [transform.m11, transform.m12, 0.0, 0.0],
                    [transform.m21, transform.m22, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [transform.m31, transform.m32, 0.0, 1.0],
                ];

                pipeline.apply();
                pipeline.set_transform(&transform);

                unsafe {
                    gl::Enable(gl::STENCIL_TEST);
                    gl::StencilMask(0xff);
                    gl::StencilFunc(gl::ALWAYS, 0, 0xff);
                    gl::ColorMask(gl::FALSE, gl::FALSE, gl::FALSE, gl::FALSE);

                    pipeline.apply_frag_uniforms(&FragUniforms {
                        stroke_thr: -1.0,
                        type_: ShaderType::Simple as i32,
                        ..FragUniforms::default()
                    });

                    gl::StencilOpSeparate(gl::FRONT, gl::KEEP, gl::KEEP, gl::INCR_WRAP);
                    gl::StencilOpSeparate(gl::BACK, gl::KEEP, gl::KEEP, gl::DECR_WRAP);
                    gl::Disable(gl::CULL_FACE);
                    for path in paths {
                        let fill_vertices = path.get_fill();
                        if !fill_vertices.is_empty() {
                            pipeline.draw(&fill_vertices, gl::TRIANGLE_FAN);
                        }
                    }
                    gl::Enable(gl::CULL_FACE);

                    gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);

                    pipeline.apply_frag_uniforms(&uniforms);

                    gl::StencilFunc(gl::EQUAL, 0x00, 0xff);
                    gl::StencilOp(gl::KEEP, gl::KEEP, gl::KEEP);
                    for path in paths {
                        let stoke_vertices = path.get_stroke();
                        if !stoke_vertices.is_empty() {
                            pipeline.draw(&stoke_vertices, gl::TRIANGLE_STRIP);
                        }
                    }

                    let rect_verts = vec![
                        TexturedVertex::new(
                            [bounds.max.x, bounds.max.y],
                            [0.5, 1.0],
                            [1.0, 1.0, 1.0, 1.0],
                        ),
                        TexturedVertex::new(
                            [bounds.max.x, bounds.min.y],
                            [0.5, 1.0],
                            [1.0, 1.0, 1.0, 1.0],
                        ),
                        TexturedVertex::new(
                            [bounds.min.x, bounds.max.y],
                            [0.5, 1.0],
                            [1.0, 1.0, 1.0, 1.0],
                        ),
                        TexturedVertex::new(
                            [bounds.min.x, bounds.min.y],
                            [0.5, 1.0],
                            [1.0, 1.0, 1.0, 1.0],
                        ),
                    ];

                    gl::StencilFunc(gl::NOTEQUAL, 0x00, 0xff);
                    gl::StencilOp(gl::ZERO, gl::ZERO, gl::ZERO);
                    pipeline.draw(&rect_verts, gl::TRIANGLE_STRIP);

                    gl::Disable(gl::STENCIL_TEST);
                }
            }
        }
    }
}

pub struct GlWindowTarget {
    gl_windowed_context:
        RefCell<Option<glutin::ContextWrapper<glutin::PossiblyCurrent, winit::window::Window>>>,
    gl_render_target: GlRenderTarget,

    colored_pipeline_buffers: (GLuint, GLuint),
    textured_pipeline_buffers: (GLuint, GLuint),
    textured_y8_pipeline_buffers: (GLuint, GLuint),
    universal_pipeline_buffers: (GLuint, GLuint),
}

impl GlWindowTarget {
    pub fn get_window(&self) -> Ref<winit::window::Window> {
        Ref::map(self.gl_windowed_context.borrow(), |context| {
            context.as_ref().unwrap().window()
        })
    }

    pub fn get_render_target(&self) -> &GlRenderTarget {
        &self.gl_render_target
    }

    pub fn update_size(&mut self, width: u16, height: u16) {
        unsafe {
            self.gl_render_target.width = width;
            self.gl_render_target.height = height;
            gl::Viewport(0, 0, width as i32, height as i32);
        }
    }

    pub fn swap_buffers(&mut self) {
        self.gl_windowed_context
            .borrow()
            .as_ref()
            .unwrap()
            .swap_buffers()
            .unwrap();
    }

    pub fn get_context(
        &self,
    ) -> Ref<glutin::ContextWrapper<glutin::PossiblyCurrent, winit::window::Window>> {
        Ref::map(self.gl_windowed_context.borrow(), |context| {
            context.as_ref().unwrap()
        })
    }
}

impl Drop for GlWindowTarget {
    fn drop(&mut self) {
        unsafe {
            let context = self.gl_windowed_context.replace(None);
            let context = context.unwrap().make_current().unwrap();
            self.gl_windowed_context.replace(Some(context));

            gl::DeleteVertexArrays(1, &mut self.colored_pipeline_buffers.1);
            gl::DeleteBuffers(1, &mut self.colored_pipeline_buffers.0);

            gl::DeleteVertexArrays(1, &mut self.textured_pipeline_buffers.1);
            gl::DeleteBuffers(1, &mut self.textured_pipeline_buffers.0);

            gl::DeleteVertexArrays(1, &mut self.textured_y8_pipeline_buffers.1);
            gl::DeleteBuffers(1, &mut self.textured_y8_pipeline_buffers.0);

            gl::DeleteVertexArrays(1, &mut self.universal_pipeline_buffers.1);
            gl::DeleteBuffers(1, &mut self.universal_pipeline_buffers.0);
        }
    }
}

pub struct GlRenderTarget {
    framebuffer_id: GLuint,
    width: u16,
    height: u16,
    aspect_ratio: f32,
}

impl Drop for GlRenderTarget {
    fn drop(&mut self) {
        if self.framebuffer_id > 0 {
            unsafe {
                gl::DeleteFramebuffers(1, &mut self.framebuffer_id);
            }
        }
    }
}

impl RenderTarget for GlRenderTarget {
    fn get_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    fn get_aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    fn get_device_transform(&self) -> PhysPixelToDeviceTransform {
        PhysPixelToDeviceTransform::column_major(
            2.0f32 / self.width as f32,
            0.0f32,
            -1.0f32,
            0.0f32,
            -2.0f32 / self.height as f32,
            1.0f32,
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GlTexture {
    id: GLuint,
    is_owned: bool,
    width: u16,
    height: u16,
    gl_format: GLuint,
    gl_type: GLuint,
    flipped_y: bool,
}

impl drawing::backend::Texture for GlTexture {
    fn update(
        &mut self,
        memory: &[u8],
        offset_x: u16,
        offset_y: u16,
        width: u16,
        height: u16,
    ) -> Result<()> {
        unsafe {
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                offset_x as GLint,
                offset_y as GLint,
                width as GLsizei,
                height as GLsizei,
                self.gl_format,
                self.gl_type,
                memory.as_ptr() as *const GLvoid,
            );
        }
        Ok(())
    }

    fn get_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}

impl Drop for GlTexture {
    fn drop(&mut self) {
        if self.is_owned && self.id > 0 {
            unsafe {
                gl::DeleteTextures(1, &self.id);
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////
//
// backend specific extensions
//
///////////////////////////////////////////////////////////////////////

impl GlTexture {
    pub fn from_external(id: GLuint, width: u16, height: u16, format: ColorFormat) -> GlTexture {
        let (gl_type, gl_format) = match format {
            ColorFormat::RGBA => (gl::UNSIGNED_INT_8_8_8_8_REV, gl::BGRA),
            ColorFormat::Y8 => (gl::UNSIGNED_BYTE, gl::RED),
        };
        GlTexture {
            id,
            is_owned: false,
            width,
            height,
            gl_format,
            gl_type,
            flipped_y: false,
        }
    }
}

#[inline]
fn premul_color(color: Color) -> Color {
    [
        color[0] * color[3],
        color[1] * color[3],
        color[2] * color[3],
        color[3],
    ]
}

#[inline]
fn xform_to_3x4(xform: [f32; 6]) -> [f32; 12] {
    let mut m = [0f32; 12];
    let t = &xform;
    m[0] = t[0];
    m[1] = t[1];
    m[2] = 0.0;
    m[3] = 0.0;
    m[4] = t[2];
    m[5] = t[3];
    m[6] = 0.0;
    m[7] = 0.0;
    m[8] = t[4];
    m[9] = t[5];
    m[10] = 1.0;
    m[11] = 0.0;
    m
}

pub fn inverse(transform: [f32; 6]) -> [f32; 6] {
    let t = &transform;
    let det = t[0] * t[3] - t[2] * t[1];
    if det > -1e-6 && det < 1e-6 {
        return [1.0, 0.0, 0.0, 1.0, 0.0, 0.0];
    }
    let invdet = 1.0 / det;
    let mut inv = [0f32; 6];
    inv[0] = t[3] * invdet;
    inv[2] = -t[2] * invdet;
    inv[4] = (t[2] * t[5] - t[3] * t[4]) * invdet;
    inv[1] = -t[1] * invdet;
    inv[3] = t[0] * invdet;
    inv[5] = (t[1] * t[4] - t[0] * t[5]) * invdet;
    inv
}
