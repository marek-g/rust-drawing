extern crate drawing;
extern crate winit;
extern crate glutin;
extern crate gl;
extern crate std;

use self::drawing::color::*;
use self::drawing::units::*;
use self::glutin::GlContext;
use self::gl::types::*;
use ::utils::Program;
use ::utils::Shader;
use ::pipelines::*;
use backend::drawing::backend::*;

pub struct GlRenderTarget {
    framebuffer_id: GLuint,
    width: u16,
    height: u16,
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

#[derive(Clone, Debug, PartialEq)]
pub struct GlTexture {
    id: GLuint,
    width: u16,
    height: u16,
    gl_format: GLuint,
    gl_type: GLuint,
    flipped_y: bool,
}

impl drawing::backend::Texture for GlTexture {
    type Factory = ();
    type Encoder = ();
	type Error = ();
    type Error2 = ();

	fn create(factory: &mut Self::Factory, memory: Option<&[u8]>,
		width: u16, height: u16, format: ColorFormat, updatable: bool) -> Result<Self, Self::Error> {
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

        let mut texture = GlTexture {
            id: texture_id,
            width, height,
            gl_format, gl_type,
            flipped_y: false,
        };

        unsafe {
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl_internal_format as GLint,
                width as GLsizei, height as GLsizei, 0, gl_format, gl_type,
                match memory {
                    Some(memory) => memory.as_ptr() as *const GLvoid,
                    None => std::ptr::null(),
                });
        }

        Ok(texture)
    }

	fn update(&mut self, encoder: &mut Self::Encoder, memory: &[u8],
		offset_x: u16, offset_y: u16, width: u16, height: u16) -> Result<(), Self::Error2> {
        unsafe {
            gl::TexSubImage2D(gl::TEXTURE_2D, 0, offset_x as GLint, offset_y as GLint,
                width as GLsizei, height as GLsizei, self.gl_format, self.gl_type,
                memory.as_ptr() as *const GLvoid);
        }
        Ok(())
    }

    fn get_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}

impl Drop for GlTexture {
    fn drop(&mut self) {
        if self.id > 0 {
            unsafe { gl::DeleteTextures(1, &self.id); }
        }
    }
}

pub struct GlBackend {
    factory: (),
    width: u16,
    height: u16,
    colored_pipeline: ColoredPipeline,
    textured_pipeline: TexturedPipeline,
    textured_y8_pipeline: TexturedY8Pipeline,
}

pub struct GlWindowBackend {
    window: glutin::GlWindow,
    gl_backend: GlBackend
}

impl drawing::backend::WindowBackend for GlWindowBackend {
    fn create_window_backend(window_builder: winit::WindowBuilder,
		events_loop: &winit::EventsLoop) -> Self {

        // create OpenGl context
        // context can be shared between windows (doesn't have to)
        // cannot be shared between threads (until shared with other context)
        let context = glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 2)))
            .with_vsync(true);

        let gl_window = glutin::GlWindow::new(window_builder, context, &events_loop).unwrap();

        // make context current
        unsafe {
            gl_window.make_current().unwrap();
        }

        // tell gl crate how to forward gl function calls to the driver
        unsafe {
            gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        }

        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

		GlWindowBackend {
            window: gl_window,
            gl_backend: GlBackend {
                factory: (),
                width: 0, height: 0,
                colored_pipeline: ColoredPipeline::new(),
                textured_pipeline: TexturedPipeline::new(),
                textured_y8_pipeline: TexturedY8Pipeline::new(),
            }
        }
	}

    fn update_window_size(&mut self, width: u16, height: u16) {
        unsafe {
            self.gl_backend.width = width;
            self.gl_backend.height = height;
            gl::Viewport(0, 0, width as i32, height as i32);
        }
    }
}

impl drawing::backend::Backend for GlWindowBackend {
    type Factory = ();
    type Texture = GlTexture;
    type RenderTarget = GlRenderTarget;

    fn get_device_transform(size: PhysPixelSize) -> PhysPixelToDeviceTransform {
        GlBackend::get_device_transform(size)
    }

    fn get_factory(&self) -> Self::Factory {
        self.gl_backend.get_factory()
    }

    fn create_texture(&mut self, memory: Option<&[u8]>, width: u16, height: u16, format: ColorFormat, updatable: bool) -> Self::Texture {
        self.gl_backend.create_texture(memory, width, height, format, updatable)
    }

    fn get_main_render_target(&mut self)-> Self::RenderTarget {
        self.gl_backend.get_main_render_target()
    }

    fn create_render_target(&mut self, width: u16, height: u16) -> (Self::Texture, Self::RenderTarget) {
        self.gl_backend.create_render_target(width, height)
    }

    fn begin(&mut self) {
        self.gl_backend.begin()
    }

    fn clear(&mut self, target: &Self::RenderTarget,
        color: &Color) {
        self.gl_backend.clear(target, color)
    }

    fn triangles_colored(&mut self, target: &Self::RenderTarget,
        vertices: &[ColoredVertex],
        transform: UnknownToDeviceTransform) {
        self.gl_backend.triangles_colored(target, vertices, transform)
    }

    fn triangles_textured(&mut self, target: &Self::RenderTarget,
        texture: &Self::Texture, filtering: bool,
		vertices: &[TexturedVertex],
		transform: UnknownToDeviceTransform) {
        self.gl_backend.triangles_textured(target, texture, filtering, vertices, transform)
    }

    fn triangles_textured_y8(&mut self, target: &Self::RenderTarget,
		texture: &Self::Texture, filtering: bool,
		vertices: &[TexturedY8Vertex],
		transform: UnknownToDeviceTransform) {
        self.gl_backend.triangles_textured_y8(target, texture, filtering, vertices, transform)
    }

    fn line(&mut self, target: &Self::RenderTarget,
        color: &Color, thickness: DeviceThickness,
		start_point: Point, end_point: Point,
		transform: UnknownToDeviceTransform) {
        self.gl_backend.line(target, color, thickness, start_point, end_point, transform)
    }

    fn end(&mut self) {
        self.gl_backend.end();
        self.window.swap_buffers().unwrap();
    }
}

impl drawing::backend::Backend for GlBackend {
    type Factory = ();
    type Texture = GlTexture;
    type RenderTarget = GlRenderTarget;

    fn get_device_transform(size: PhysPixelSize) -> PhysPixelToDeviceTransform {
        PhysPixelToDeviceTransform::column_major(
            2.0f32 / size.width, 0.0f32, -1.0f32,
            0.0f32, -2.0f32 / size.height, 1.0f32,
        )
    }

    fn get_factory(&self) -> Self::Factory {
    }

    fn create_texture(&mut self, memory: Option<&[u8]>, width: u16, height: u16, format: ColorFormat, updatable: bool) -> Self::Texture {
        Self::Texture::create(&mut self.factory, memory, width, height, format, updatable).unwrap()
    }

    fn get_main_render_target(&mut self)-> Self::RenderTarget {
        GlRenderTarget {
            framebuffer_id: 0,
            width: self.width,
            height: self.height,
        }
    }

    fn create_render_target(&mut self, width: u16, height: u16) -> (Self::Texture, Self::RenderTarget) {
        let mut framebuffer_id: GLuint = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut framebuffer_id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer_id);
        }
        let mut texture = GlTexture::create(&mut (), None, width, height, ColorFormat::RGBA, false).unwrap();
        texture.flipped_y = true;
        unsafe {
            gl::FramebufferTexture(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, texture.id, 0);
            let draw_buffers = gl::COLOR_ATTACHMENT0;
            gl::DrawBuffers(1, &draw_buffers);
        }
        (texture, GlRenderTarget { framebuffer_id, width, height })
    }

	fn begin(&mut self) {
	}

    fn clear(&mut self, target: &Self::RenderTarget, color: &Color) {
        self.set_render_target(&target);
        unsafe {
            gl::ClearColor(color[0], color[1], color[2], color[3]);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
        }
    }

    fn triangles_colored(&mut self, target: &Self::RenderTarget,
        vertices: &[ColoredVertex],
        transform: UnknownToDeviceTransform) {
        self.set_render_target(&target);
        let transform = [[transform.m11, transform.m12, 0.0, 0.0],
            [transform.m21, transform.m22, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [transform.m31, transform.m32, 0.0, 1.0]];

        self.colored_pipeline.apply();
        self.colored_pipeline.set_transform(&transform);
        self.colored_pipeline.draw(&vertices);
    }

    fn triangles_textured(&mut self, target: &Self::RenderTarget,
        texture: &Self::Texture, filtering: bool,
		vertices: &[TexturedVertex],
		transform: UnknownToDeviceTransform) {
        self.set_render_target(&target);
        unsafe {
            gl::Enable(gl::TEXTURE_2D);
            gl::BindTexture (gl::TEXTURE_2D, texture.id);
            if filtering {
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            } else {
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
            }
        }

        let transform = [[transform.m11, transform.m12, 0.0, 0.0],
            [transform.m21, transform.m22, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [transform.m31, transform.m32, 0.0, 1.0]];

        self.textured_pipeline.apply();
        self.textured_pipeline.set_transform(&transform);
        self.textured_pipeline.set_flipped_y(texture.flipped_y);
        self.textured_pipeline.draw(&vertices);
    }

    fn triangles_textured_y8(&mut self, target: &Self::RenderTarget,
		texture: &Self::Texture, filtering: bool,
		vertices: &[TexturedY8Vertex],
		transform: UnknownToDeviceTransform) {
        self.set_render_target(&target);
        unsafe {
            gl::Enable(gl::TEXTURE_2D);
            gl::BindTexture (gl::TEXTURE_2D, texture.id);
            if filtering {
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            } else {
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
            }
        }

        let transform = [[transform.m11, transform.m12, 0.0, 0.0],
            [transform.m21, transform.m22, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [transform.m31, transform.m32, 0.0, 1.0]];

        self.textured_y8_pipeline.apply();
        self.textured_y8_pipeline.set_transform(&transform);
        self.textured_y8_pipeline.set_flipped_y(texture.flipped_y);
        self.textured_y8_pipeline.draw(&vertices);
    }

    fn line(&mut self, target: &Self::RenderTarget,
        color: &Color, thickness: DeviceThickness,
		start_point: Point, end_point: Point,
		transform: UnknownToDeviceTransform) {
        self.set_render_target(&target);
        // TODO:
		//if thickness == 1.0f32 {
            self.line_native(color, start_point, end_point, transform);
        //} else {
            //self.line_triangulated(color, thickness, start_point, end_point, transform);
        //} 
	}

	fn end(&mut self) {
	}
}

impl GlBackend {
    fn set_render_target(&mut self, target: &GlRenderTarget) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, target.framebuffer_id);
            if target.framebuffer_id == 0 {
                gl::Viewport(0, 0, self.width as GLint, self.height as GLint);
            } else {
                gl::Viewport(0, 0, target.width as GLint, target.height as GLint);
            }
        }
    }

	fn line_native(&mut self,
        color: &Color, start_point: Point, end_point: Point,
		transform: UnknownToDeviceTransform) {
        let transform = [[transform.m11, transform.m12, 0.0, 0.0],
            [transform.m21, transform.m22, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [transform.m31, transform.m32, 0.0, 1.0]];

        let v1 = ColoredVertex::new([start_point.x, start_point.y], *color);
        let v2 = ColoredVertex::new([end_point.x, end_point.y], *color);
        let v3 = ColoredVertex::new([start_point.x, start_point.y], *color);

        self.colored_pipeline.apply();
        self.colored_pipeline.set_transform(&transform);
        self.colored_pipeline.draw_lines(&[v1, v2, v3]);
    }

    fn line_triangulated(&mut self,
        color: &Color, thickness: DeviceThickness,
        start_point: Point, end_point: Point,
		transform: UnknownToDeviceTransform) {
    }
}
