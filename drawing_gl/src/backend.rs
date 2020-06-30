extern crate drawing;
extern crate gl;
extern crate glutin;
extern crate std;
extern crate winit;

use drawing::clipping::Scissor;
use drawing::composite_operation_state::CompositeOperationState;
use drawing::paint::Paint;
use euclid::Vector2D;

use crate::backend::drawing::backend::*;
use crate::pipelines::*;
use drawing::color::*;
use drawing::path::{Bounds, Path};
use drawing::units::*;
use drawing::Result;
use gl::types::*;

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
            gl::Disable(gl::CULL_FACE);
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

        self.aspect_ratio = windowed_context.window().scale_factor() as f32;

        let mut time_query: GLuint = 0;
        unsafe {
            gl::GenQueries(1, &mut time_query);
            gl::BeginQuery(gl::TIME_ELAPSED, time_query);
            gl::EndQuery(gl::TIME_ELAPSED);
        }
        print!("time_query: {}", time_query);

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
            
            time_query,
        })
    }

    pub fn begin(&mut self, window_target: &GlWindowTarget) -> Result<()> {
        unsafe {
            gl::BeginQuery(gl::TIME_ELAPSED, window_target.time_query);
        }

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

    pub fn end(&mut self, window_target: &GlWindowTarget) {
        unsafe {
            gl::EndQuery(gl::TIME_ELAPSED);

            // retrieving the recorded elapsed time
            // wait until the query result is available
            let mut done = 0i32;
            while done == 0 {
                gl::GetQueryObjectiv(
                    window_target.time_query,
                    gl::QUERY_RESULT_AVAILABLE,
                    &mut done,
                );
            }

            // get the query result
            let mut elapsed_time: GLuint64 = 0;
            gl::GetQueryObjectui64v(
                window_target.time_query,
                gl::QUERY_RESULT,
                &mut elapsed_time,
            );
            println!("GPU time: {} ms", elapsed_time as f64 / 1000000.0);
        }
    }

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

    fn convert_paint(
        paint: &Paint,
        texture: Option<&GlTexture>,
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
            frag.scissor_mat = xform_to_3x4(
                scissor
                    .xform
                    .inverse()
                    .unwrap_or_else(|| PixelTransform::identity()),
            );
            frag.scissor_ext[0] = scissor.extent[0];
            frag.scissor_ext[1] = scissor.extent[1];
            frag.scissor_scale[0] = (scissor.xform.m11 * scissor.xform.m11
                + scissor.xform.m21 * scissor.xform.m21)
                .sqrt()
                / fringe;
            frag.scissor_scale[1] = (scissor.xform.m12 * scissor.xform.m12
                + scissor.xform.m22 * scissor.xform.m22)
                .sqrt()
                / fringe;
        }

        frag.extent = [paint.extent[0], paint.extent[1]];
        frag.stroke_mult = (width * 0.5 + fringe * 0.5) / fringe;

        let invxform;

        if let Some(texture) = texture {
            frag.type_ = ShaderType::FillImage as i32;

            if texture.flipped_y {
                let m1 = paint
                    .xform
                    .pre_translate(Vector2D::new(0.0, frag.extent[1] * 0.5))
                    .pre_scale(1.0f32, -1.0f32)
                    .pre_translate(Vector2D::new(0.0, -frag.extent[1] * 0.5));
                invxform = m1.inverse().unwrap_or_else(|| PixelTransform::identity());
            } else {
                invxform = paint
                    .xform
                    .inverse()
                    .unwrap_or_else(|| PixelTransform::identity());
            };

            match texture.gl_format {
                gl::BGRA => frag.tex_type = 0,
                gl::RED => frag.tex_type = 1,
                _ => frag.tex_type = 0,
            }
        } else {
            frag.type_ = ShaderType::FillGradient as i32;
            frag.radius = paint.radius;
            frag.feather = paint.feather;
            invxform = paint
                .xform
                .inverse()
                .unwrap_or_else(|| PixelTransform::identity());
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
        _thickness: DeviceThickness,
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

    fn stroke(
        &mut self,
        target: &Self::RenderTarget,
        paint: &Paint,
        texture: Option<&Self::Texture>,
        _filtering: bool,
        paths: &[Path],
        thickness: f32,
        fringe_width: f32,
        antialiasing: bool,
        scissor: Scissor,
        _composite_operation_state: CompositeOperationState,
        transform: UnknownToDeviceTransform,
    ) {
        self.set_render_target(&target);
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

                // Fill the stroke base without overlap
                gl::StencilFunc(gl::EQUAL, 0x0, 0xff);
                gl::StencilOp(gl::KEEP, gl::KEEP, gl::INCR);
                if antialiasing {
                    pipeline.apply_frag_uniforms(&Self::convert_paint(
                        paint,
                        texture,
                        &scissor,
                        thickness,
                        fringe_width,
                        1.0 - 0.5 / 255.0,
                    ));
                } else {
                    pipeline.apply_frag_uniforms(&Self::convert_paint(
                        paint,
                        texture,
                        &scissor,
                        thickness,
                        fringe_width,
                        -1.0,
                    ));
                }
                for path in paths {
                    let stroke_vertices = path.get_stroke();
                    if !stroke_vertices.is_empty() {
                        pipeline.draw(&stroke_vertices, gl::TRIANGLE_STRIP);
                    }
                }

                // Draw anti-aliased pixels.
                if antialiasing {
                    pipeline.apply_frag_uniforms(&Self::convert_paint(
                        paint,
                        texture,
                        &scissor,
                        thickness,
                        fringe_width,
                        -1.0,
                    ));
                    gl::StencilFunc(gl::EQUAL, 0x0, 0xff);
                    gl::StencilOp(gl::KEEP, gl::KEEP, gl::KEEP);
                    for path in paths {
                        let stroke_vertices = path.get_stroke();
                        if !stroke_vertices.is_empty() {
                            pipeline.draw(&stroke_vertices, gl::TRIANGLE_STRIP);
                        }
                    }
                }

                // Clear stencil buffer.
                gl::ColorMask(gl::FALSE, gl::FALSE, gl::FALSE, gl::FALSE);
                gl::StencilFunc(gl::ALWAYS, 0x0, 0xff);
                gl::StencilOp(gl::ZERO, gl::ZERO, gl::ZERO);
                for path in paths {
                    let stroke_vertices = path.get_stroke();
                    if !stroke_vertices.is_empty() {
                        pipeline.draw(&stroke_vertices, gl::TRIANGLE_STRIP);
                    }
                }
                gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);

                gl::Disable(gl::STENCIL_TEST);
            }
        }
    }

    fn fill(
        &mut self,
        target: &Self::RenderTarget,
        paint: &Paint,
        texture: Option<&Self::Texture>,
        filtering: bool,
        paths: &[Path],
        bounds: Bounds,
        fringe_width: f32,
        antialiasing: bool,
        scissor: Scissor,
        _composite_operation_state: CompositeOperationState,
        transform: UnknownToDeviceTransform,
    ) {
        self.set_render_target(&target);

        if let Some(texture_id) = paint.image {
            unsafe {
                gl::Enable(gl::TEXTURE_2D);
                gl::BindTexture(gl::TEXTURE_2D, texture_id as GLuint);
                if filtering {
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
                } else {
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
                }
            }
        }

        if paths.len() == 1 && paths[0].convex {
            // convex fill
            let uniforms =
                Self::convert_paint(paint, texture, &scissor, fringe_width, fringe_width, -1.0);

            if let Some(ref mut pipeline) = self.universal_pipeline {
                let transform = [
                    [transform.m11, transform.m12, 0.0, 0.0],
                    [transform.m21, transform.m22, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [transform.m31, transform.m32, 0.0, 1.0],
                ];

                pipeline.apply();
                pipeline.set_transform(&transform);

                pipeline.apply_frag_uniforms(&uniforms);

                // fill shape
                let fill_vertices = paths[0].get_fill();
                if !fill_vertices.is_empty() {
                    pipeline.draw(&fill_vertices, gl::TRIANGLE_FAN);
                }

                // antialias outline
                if antialiasing {
                    let stroke_vertices = paths[0].get_stroke();
                    if !stroke_vertices.is_empty() {
                        pipeline.draw(&stroke_vertices, gl::TRIANGLE_STRIP);
                    }
                }
            }
        } else {
            let uniforms =
                Self::convert_paint(paint, texture, &scissor, fringe_width, fringe_width, -1.0);

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
                    // Draw shapes on stencil buffer
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
                    gl::CullFace(gl::BACK);

                    gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);

                    pipeline.apply_frag_uniforms(&uniforms);

                    // Draw anti-aliased pixels
                    if antialiasing {
                        gl::StencilFunc(gl::EQUAL, 0x00, 0xff);
                        gl::StencilOp(gl::KEEP, gl::KEEP, gl::KEEP);
                        for path in paths {
                            let stroke_vertices = path.get_stroke();
                            if !stroke_vertices.is_empty() {
                                pipeline.draw(&stroke_vertices, gl::TRIANGLE_STRIP);
                            }
                        }
                    }

                    // Draw fill
                    gl::Disable(gl::CULL_FACE);

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

    time_query: GLuint,
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

    fn get_device_transform(&self) -> PixelToDeviceTransform {
        PixelToDeviceTransform::column_major(
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
fn xform_to_3x4(xform: PixelTransform) -> [f32; 12] {
    let mut m = [0f32; 12];
    m[0] = xform.m11;
    m[1] = xform.m12;
    m[2] = 0.0;
    m[3] = 0.0;
    m[4] = xform.m21;
    m[5] = xform.m22;
    m[6] = 0.0;
    m[7] = 0.0;
    m[8] = xform.m31;
    m[9] = xform.m32;
    m[10] = 1.0;
    m[11] = 0.0;
    m
}
