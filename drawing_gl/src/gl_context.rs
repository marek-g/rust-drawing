use drawing_api::{ColorFormat, Context, PixelTransform, Point, Texture, UnknownToDeviceTransform};
use euclid::Vector2D;
use gl::types::*;
use std::{borrow::Cow, cell::RefCell, ffi::c_void, ops::DerefMut, rc::Rc, sync::Arc};

use crate::{
    generic::{
        clipping::Scissor,
        device::{ColoredVertex, Device, Paint, TexturedVertex},
        renderer::Renderer,
    },
    pipelines::{
        ColoredPipeline, FragUniforms, ShaderType, TexturedPipeline, TexturedY8Pipeline,
        UniversalPipeline,
    },
    GlSurface, GlTexture, GlTextureData,
};

pub struct GlContextData {
    colored_pipeline: ColoredPipeline,
    colored_pipeline_buffers: (GLuint, GLuint),

    textured_pipeline: TexturedPipeline,
    textured_pipeline_buffers: (GLuint, GLuint),

    textured_y8_pipeline: TexturedY8Pipeline,
    textured_y8_pipeline_buffers: (GLuint, GLuint),

    universal_pipeline: UniversalPipeline,
    universal_pipeline_buffers: (GLuint, GLuint),
}

impl Drop for GlContextData {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.colored_pipeline_buffers.1);
            gl::DeleteBuffers(1, &self.colored_pipeline_buffers.0);

            gl::DeleteVertexArrays(1, &self.textured_pipeline_buffers.1);
            gl::DeleteBuffers(1, &self.textured_pipeline_buffers.0);

            gl::DeleteVertexArrays(1, &self.textured_y8_pipeline_buffers.1);
            gl::DeleteBuffers(1, &self.textured_y8_pipeline_buffers.0);

            gl::DeleteVertexArrays(1, &self.universal_pipeline_buffers.1);
            gl::DeleteBuffers(1, &self.universal_pipeline_buffers.0);
        }
    }
}

impl GlContextData {
    fn create_texture(
        &self,
        contents: Option<&[u8]>,
        width: u16,
        height: u16,
        format: drawing_api::ColorFormat,
        flipped_y: bool,
    ) -> Result<GlTexture, &'static str> {
        let mut texture_id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
        }

        let (gl_internal_format, gl_type, gl_format) = match format {
            //ColorFormat::RGBA => (gl::RGBA, gl::UNSIGNED_BYTE, gl::RGBA),
            ColorFormat::RGBA => (gl::SRGB8_ALPHA8, gl::UNSIGNED_BYTE, gl::RGBA),
            //ColorFormat::RGBA => (gl::RGBA, gl::UNSIGNED_INT_8_8_8_8_REV, gl::BGRA),
            ColorFormat::Y8 => (gl::R8, gl::UNSIGNED_BYTE, gl::RED),
        };

        let texture = GlTexture {
            data: Arc::new(GlTextureData {
                id: texture_id,
                is_owned: true,
                width,
                height,
                gl_format,
                gl_type,
                flipped_y,
            }),
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
                match contents {
                    Some(contents) => contents.as_ptr() as *const GLvoid,
                    None => std::ptr::null(),
                },
            );
        }

        Ok(texture)
    }

    pub fn set_render_target(&mut self, target: &GlSurface) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, target.framebuffer_id);
            gl::Viewport(0, 0, target.width as GLint, target.height as GLint);
        }
    }

    fn line_native(
        &mut self,
        color: &crate::generic::device::Color,
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

        self.colored_pipeline.apply();
        self.colored_pipeline.set_transform(&transform);
        self.colored_pipeline.draw_lines(&[v1, v2, v3]);
    }

    fn convert_paint(
        paint: &Paint<GlTexture>,
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
                    .unwrap_or_else(PixelTransform::identity),
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

            if texture.data.flipped_y {
                let m1 = paint
                    .xform
                    .pre_translate(Vector2D::new(0.0, frag.extent[1] * 0.5))
                    .pre_scale(1.0f32, -1.0f32)
                    .pre_translate(Vector2D::new(0.0, -frag.extent[1] * 0.5));
                invxform = m1.inverse().unwrap_or_else(PixelTransform::identity);
            } else {
                invxform = paint
                    .xform
                    .inverse()
                    .unwrap_or_else(PixelTransform::identity);
            };

            match texture.data.gl_format {
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
                .unwrap_or_else(PixelTransform::identity);
        }

        frag.paint_mat = xform_to_3x4(invxform);

        frag
    }
}

impl Device for GlContextData {
    type Texture = GlTexture;
    type RenderTarget = GlSurface;

    fn create_texture(
        &self,
        contents: &[u8],
        width: u16,
        height: u16,
        format: ColorFormat,
    ) -> Result<Self::Texture, &'static str> {
        self.create_texture(Some(contents), width, height, format, false)
    }

    fn create_render_target(
        &mut self,
        width: u16,
        height: u16,
    ) -> Result<(Self::Texture, Self::RenderTarget), &'static str> {
        let mut framebuffer_id: GLuint = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut framebuffer_id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer_id);
        }
        let texture = self.create_texture(None, width, height, ColorFormat::RGBA, true)?;
        unsafe {
            gl::FramebufferTexture(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, texture.data.id, 0);
            let draw_buffers = gl::COLOR_ATTACHMENT0;
            gl::DrawBuffers(1, &draw_buffers);
        }
        Ok((
            texture,
            GlSurface {
                framebuffer_id,
                width,
                height,
                color_format: ColorFormat::RGBA,
                is_owner: true,
            },
        ))
    }

    fn clear(&mut self, target: &Self::RenderTarget, color: &crate::generic::device::Color) {
        self.set_render_target(target);
        unsafe {
            gl::ClearColor(color[0], color[1], color[2], color[3]);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
        }
    }

    fn triangles_colored(
        &mut self,
        target: &Self::RenderTarget,
        vertices: &[crate::generic::device::ColoredVertex],
        transform: drawing_api::UnknownToDeviceTransform,
    ) {
        self.set_render_target(target);
        let transform = [
            [transform.m11, transform.m12, 0.0, 0.0],
            [transform.m21, transform.m22, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [transform.m31, transform.m32, 0.0, 1.0],
        ];

        self.colored_pipeline.apply();
        self.colored_pipeline.set_transform(&transform);
        self.colored_pipeline.draw(vertices);
    }

    fn triangles_textured(
        &mut self,
        target: &Self::RenderTarget,
        texture: &Self::Texture,
        filtering: bool,
        vertices: &[crate::generic::device::TexturedVertex],
        transform: drawing_api::UnknownToDeviceTransform,
    ) {
        self.set_render_target(target);
        unsafe {
            gl::Enable(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, texture.data.id);
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

        self.textured_pipeline.apply();
        self.textured_pipeline.set_transform(&transform);
        self.textured_pipeline.set_flipped_y(texture.data.flipped_y);
        self.textured_pipeline.draw(vertices);
    }

    fn triangles_textured_y8(
        &mut self,
        target: &Self::RenderTarget,
        texture: &Self::Texture,
        filtering: bool,
        vertices: &[crate::generic::device::TexturedY8Vertex],
        transform: drawing_api::UnknownToDeviceTransform,
    ) {
        self.set_render_target(target);
        unsafe {
            gl::Enable(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, texture.data.id);
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

        self.textured_y8_pipeline.apply();
        self.textured_y8_pipeline.set_transform(&transform);
        self.textured_y8_pipeline
            .set_flipped_y(texture.data.flipped_y);
        self.textured_y8_pipeline.draw(vertices);
    }

    fn line(
        &mut self,
        target: &Self::RenderTarget,
        color: &crate::generic::device::Color,
        thickness: drawing_api::DeviceLength,
        start_point: drawing_api::Point,
        end_point: drawing_api::Point,
        transform: drawing_api::UnknownToDeviceTransform,
    ) {
        self.set_render_target(target);
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
        paint: &crate::generic::device::Paint<Self::Texture>,
        texture: Option<&Self::Texture>,
        filtering: bool,
        paths: &[crate::generic::path::Path],
        thickness: f32,
        fringe_width: f32,
        antialiasing: bool,
        scissor: crate::generic::clipping::Scissor,
        composite_operation_state: crate::generic::renderer::CompositeOperationState,
        transform: drawing_api::UnknownToDeviceTransform,
    ) {
        self.set_render_target(target);
        let transform = [
            [transform.m11, transform.m12, 0.0, 0.0],
            [transform.m21, transform.m22, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [transform.m31, transform.m32, 0.0, 1.0],
        ];

        self.universal_pipeline.apply();
        self.universal_pipeline.set_transform(&transform);

        unsafe {
            gl::Enable(gl::STENCIL_TEST);
            gl::StencilMask(0xff);

            // Fill the stroke base without overlap
            gl::StencilFunc(gl::EQUAL, 0x0, 0xff);
            gl::StencilOp(gl::KEEP, gl::KEEP, gl::INCR);
            if antialiasing {
                self.universal_pipeline
                    .apply_frag_uniforms(&Self::convert_paint(
                        paint,
                        texture,
                        &scissor,
                        thickness,
                        fringe_width,
                        1.0 - 0.5 / 255.0,
                    ));
            } else {
                self.universal_pipeline
                    .apply_frag_uniforms(&Self::convert_paint(
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
                    self.universal_pipeline
                        .draw(stroke_vertices, gl::TRIANGLE_STRIP);
                }
            }

            // Draw anti-aliased pixels.
            if antialiasing {
                self.universal_pipeline
                    .apply_frag_uniforms(&Self::convert_paint(
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
                        self.universal_pipeline
                            .draw(stroke_vertices, gl::TRIANGLE_STRIP);
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
                    self.universal_pipeline
                        .draw(stroke_vertices, gl::TRIANGLE_STRIP);
                }
            }
            gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);

            gl::Disable(gl::STENCIL_TEST);
        }
    }

    fn fill(
        &mut self,
        target: &Self::RenderTarget,
        paint: &crate::generic::device::Paint<Self::Texture>,
        texture: Option<&Self::Texture>,
        filtering: bool,
        paths: &[crate::generic::path::Path],
        bounds: crate::generic::path::Bounds,
        fringe_width: f32,
        antialiasing: bool,
        scissor: crate::generic::clipping::Scissor,
        composite_operation_state: crate::generic::renderer::CompositeOperationState,
        transform: drawing_api::UnknownToDeviceTransform,
    ) {
        self.set_render_target(target);

        if let Some(ref texture) = paint.image {
            unsafe {
                gl::Enable(gl::TEXTURE_2D);
                gl::BindTexture(gl::TEXTURE_2D, texture.get_native_handle() as GLuint);
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

            let transform = [
                [transform.m11, transform.m12, 0.0, 0.0],
                [transform.m21, transform.m22, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [transform.m31, transform.m32, 0.0, 1.0],
            ];

            self.universal_pipeline.apply();
            self.universal_pipeline.set_transform(&transform);

            self.universal_pipeline.apply_frag_uniforms(&uniforms);

            // fill shape
            let fill_vertices = paths[0].get_fill();
            if !fill_vertices.is_empty() {
                self.universal_pipeline
                    .draw(fill_vertices, gl::TRIANGLE_FAN);
            }

            // antialias outline
            if antialiasing {
                let stroke_vertices = paths[0].get_stroke();
                if !stroke_vertices.is_empty() {
                    self.universal_pipeline
                        .draw(stroke_vertices, gl::TRIANGLE_STRIP);
                }
            }
        } else {
            let uniforms =
                Self::convert_paint(paint, texture, &scissor, fringe_width, fringe_width, -1.0);

            let transform = [
                [transform.m11, transform.m12, 0.0, 0.0],
                [transform.m21, transform.m22, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [transform.m31, transform.m32, 0.0, 1.0],
            ];

            self.universal_pipeline.apply();
            self.universal_pipeline.set_transform(&transform);

            unsafe {
                // Draw shapes on stencil buffer
                gl::Enable(gl::STENCIL_TEST);
                gl::StencilMask(0xff);
                gl::StencilFunc(gl::ALWAYS, 0, 0xff);
                gl::ColorMask(gl::FALSE, gl::FALSE, gl::FALSE, gl::FALSE);

                self.universal_pipeline.apply_frag_uniforms(&FragUniforms {
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
                        self.universal_pipeline
                            .draw(fill_vertices, gl::TRIANGLE_FAN);
                    }
                }
                gl::Enable(gl::CULL_FACE);
                gl::CullFace(gl::BACK);

                gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);

                self.universal_pipeline.apply_frag_uniforms(&uniforms);

                // Draw anti-aliased pixels
                if antialiasing {
                    gl::StencilFunc(gl::EQUAL, 0x00, 0xff);
                    gl::StencilOp(gl::KEEP, gl::KEEP, gl::KEEP);
                    for path in paths {
                        let stroke_vertices = path.get_stroke();
                        if !stroke_vertices.is_empty() {
                            self.universal_pipeline
                                .draw(stroke_vertices, gl::TRIANGLE_STRIP);
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
                self.universal_pipeline
                    .draw(&rect_verts, gl::TRIANGLE_STRIP);

                gl::Disable(gl::STENCIL_TEST);
            }
        }
    }
}

#[derive(Clone)]
pub struct GlContext {
    data: Rc<RefCell<GlContextData>>,
}

impl GlContext {
    pub fn new_gl_context<F>(loadfn: F) -> Result<Self, &'static str>
    where
        F: FnMut(&'static str) -> *const c_void,
    {
        // tell gl crate how to forward gl function calls to the driver
        gl::load_with(loadfn);

        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Disable(gl::CULL_FACE);
        }

        let mut colored_pipeline = ColoredPipeline::new();
        let colored_pipeline_buffers = colored_pipeline.create_vbo_and_vao();
        colored_pipeline.set_buffers(colored_pipeline_buffers);

        let mut textured_pipeline = TexturedPipeline::new();
        let textured_pipeline_buffers = textured_pipeline.create_vbo_and_vao();
        textured_pipeline.set_buffers(textured_pipeline_buffers);

        let mut textured_y8_pipeline = TexturedY8Pipeline::new();
        let textured_y8_pipeline_buffers = textured_y8_pipeline.create_vbo_and_vao();
        textured_y8_pipeline.set_buffers(textured_y8_pipeline_buffers);

        let mut universal_pipeline = UniversalPipeline::new();
        let universal_pipeline_buffers = universal_pipeline.create_vbo_and_vao();
        universal_pipeline.set_buffers(universal_pipeline_buffers);

        Ok(Self {
            data: Rc::new(RefCell::new(GlContextData {
                colored_pipeline,
                colored_pipeline_buffers,
                textured_pipeline,
                textured_pipeline_buffers,
                textured_y8_pipeline,
                textured_y8_pipeline_buffers,
                universal_pipeline,
                universal_pipeline_buffers,
            })),
        })
    }
}

impl Context for GlContext {
    type DisplayListBuilder = crate::DisplayListBuilder;
    type Fonts = crate::Fonts<GlContextData>;
    type Paint = crate::Paint;
    type ParagraphBuilder = crate::display_list::ParagraphBuilder;
    type PathBuilder = crate::PathBuilder;
    type Surface = GlSurface;
    type Texture = GlTexture;

    fn wrap_gl_framebuffer(
        &mut self,
        framebuffer_id: u32,
        width: u16,
        height: u16,
        color_format: ColorFormat,
    ) -> Result<GlSurface, &'static str> {
        Ok(GlSurface {
            framebuffer_id,
            width,
            height,
            color_format,
            is_owner: false,
        })
    }

    fn adopt_gl_texture(
        &self,
        texture_handle: u32,
        width: u16,
        height: u16,
        mip_count: u32,
        color_format: ColorFormat,
    ) -> Result<Self::Texture, &'static str> {
        Ok(GlTexture::from_external(
            texture_handle,
            width,
            height,
            color_format,
        ))
    }

    fn create_texture(
        &self,
        contents: Cow<'static, [u8]>,
        width: u16,
        height: u16,
        color_format: drawing_api::ColorFormat,
    ) -> Result<Self::Texture, &'static str> {
        self.data
            .borrow()
            .create_texture(Some(&contents), width, height, color_format, false)
    }

    fn draw(
        &mut self,
        surface: &mut Self::Surface,
        display_list: &<Self::DisplayListBuilder as drawing_api::DisplayListBuilder>::DisplayList,
    ) -> Result<(), &'static str> {
        let mut renderer = Renderer::new();
        let mut device = self.data.borrow_mut();
        renderer.draw(device.deref_mut(), surface, display_list, true)?;
        Ok(())
    }
}

#[inline]
fn premul_color(color: crate::generic::device::Color) -> crate::generic::device::Color {
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
