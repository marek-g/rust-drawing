extern crate drawing;
extern crate winit;
extern crate glutin;
extern crate gl;
extern crate std;

use self::drawing::color::*;
use self::drawing::units::*;
use self::glutin::GlContext;
use self::gl::types::*;
use ::pipelines::*;
use backend::drawing::backend::*;

pub struct GlDevice {
    //context: glutin::Context,
    colored_pipeline: Option<ColoredPipeline>,
    textured_pipeline: Option<TexturedPipeline>,
    textured_y8_pipeline: Option<TexturedY8Pipeline>,
}

impl GlDevice {  
    fn set_render_target(&mut self, target: &GlRenderTarget) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, target.framebuffer_id);
            gl::Viewport(0, 0, target.width as GLint, target.height as GLint);
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

        if let Some(ref mut pipeline) = self.colored_pipeline {
            pipeline.apply();
            pipeline.set_transform(&transform);
            pipeline.draw_lines(&[v1, v2, v3]);
        }
    }

    fn line_triangulated(&mut self,
        color: &Color, thickness: DeviceThickness,
        start_point: Point, end_point: Point,
		transform: UnknownToDeviceTransform) {
    }
}

impl drawing::backend::Device for GlDevice {
    type Texture = GlTexture;
    type RenderTarget = GlRenderTarget;
    type WindowTarget = GlWindowTarget;

    fn new() -> Self {
        GlDevice {
            colored_pipeline: None,
            textured_pipeline: None,
            textured_y8_pipeline: None,
        }
    }

    fn get_device_transform(size: PhysPixelSize) -> PhysPixelToDeviceTransform {
        PhysPixelToDeviceTransform::column_major(
            2.0f32 / size.width, 0.0f32, -1.0f32,
            0.0f32, -2.0f32 / size.height, 1.0f32,
        )
    }

    fn create_window_target(&mut self, window_builder: winit::WindowBuilder,
		events_loop: &winit::EventsLoop) -> Self::WindowTarget {
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
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        if self.colored_pipeline.is_none() {
            self.colored_pipeline = Some(ColoredPipeline::new());
        }
        else {
            panic!("Multiwindow mode for drawing_gl not supported by glutin yet!");
        }
        if self.textured_pipeline.is_none() {
            self.textured_pipeline = Some(TexturedPipeline::new());
        }
        if self.textured_y8_pipeline.is_none() {
            self.textured_y8_pipeline = Some(TexturedY8Pipeline::new());
        }

		GlWindowTarget {
            gl_window,
            gl_render_target: GlRenderTarget { framebuffer_id: 0, width: 0, height: 0, }
        }
	}

    fn create_texture(&mut self, memory: Option<&[u8]>, width: u16, height: u16, format: ColorFormat,
        _updatable: bool) -> Result<Self::Texture, ()> {
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

    fn create_render_target(&mut self, width: u16, height: u16) -> (Self::Texture, Self::RenderTarget) {
        let mut framebuffer_id: GLuint = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut framebuffer_id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, framebuffer_id);
        }
        let mut texture = self.create_texture(None, width, height, ColorFormat::RGBA, false).unwrap();
        texture.flipped_y = true;
        unsafe {
            gl::FramebufferTexture(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, texture.id, 0);
            let draw_buffers = gl::COLOR_ATTACHMENT0;
            gl::DrawBuffers(1, &draw_buffers);
        }
        (texture, GlRenderTarget { framebuffer_id, width, height })
    }

    fn begin(&mut self, _target: &Self::RenderTarget) {
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

        if let Some(ref mut pipeline) = self.colored_pipeline {
            pipeline.apply();
            pipeline.set_transform(&transform);
            pipeline.draw(&vertices);
        }
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

        if let Some(ref mut pipeline) = self.textured_pipeline {
            pipeline.apply();
            pipeline.set_transform(&transform);
            pipeline.set_flipped_y(texture.flipped_y);
            pipeline.draw(&vertices);
        }
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

        if let Some(ref mut pipeline) = self.textured_y8_pipeline {
            pipeline.apply();
            pipeline.set_transform(&transform);
            pipeline.set_flipped_y(texture.flipped_y);
            pipeline.draw(&vertices);
        }
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

	fn end(&mut self, _target: &Self::RenderTarget) {
	}
}

pub struct GlWindowTarget {
    gl_window: glutin::GlWindow,
    gl_render_target: GlRenderTarget,
}

impl drawing::backend::WindowTarget for GlWindowTarget {
    type RenderTarget = GlRenderTarget;

    fn get_window(&self) -> &winit::Window {
        &mut self.gl_window.window()
    }

	fn get_render_target(&mut self)-> &Self::RenderTarget {
        &self.gl_render_target
    }

	fn update_window_size(&mut self, width: u16, height: u16) {
        unsafe {
            self.gl_render_target.width = width;
            self.gl_render_target.height = height;
            gl::Viewport(0, 0, width as i32, height as i32);
        }
    }

	fn swap_buffers(&mut self) {
        self.gl_window.swap_buffers().unwrap();
    }
}

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
	type Error = ();

	fn update(&mut self, memory: &[u8],
		offset_x: u16, offset_y: u16, width: u16, height: u16) -> Result<(), Self::Error> {
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
