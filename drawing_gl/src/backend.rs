extern crate drawing;
extern crate winit;
extern crate glutin;
extern crate gl;

use self::drawing::color::*;
use self::drawing::units::*;
use self::glutin::GlContext;
use backend::drawing::backend::Texture;

#[derive(Clone, Debug, PartialEq)]
pub struct GlTexture {
}

impl drawing::backend::Texture for GlTexture {
    type Factory = ();
    type Encoder = ();
	type Error = ();
    type Error2 = ();

	fn create(factory: &mut Self::Factory, memory: &[u8],
		width: u16, height: u16, updatable: bool) -> Result<Self, Self::Error> {
        Ok(GlTexture {
        })
    }

	fn update(&mut self, encoder: &mut Self::Encoder, memory: &[u8],
		offset_x: u16, offset_y: u16, width: u16, height: u16) -> Result<(), Self::Error2> {
        Ok(())
    }

    fn get_size(&self) -> (u16, u16) {
        (10, 10)
    }
}

pub struct GlBackend {
    factory: (),
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

		GlWindowBackend {
            window: gl_window,
            gl_backend: GlBackend {
                factory: (),
            }
        }
	}

    fn update_window_size(&mut self, _width: u16, _height: u16) {
    }
}

impl drawing::backend::Backend for GlWindowBackend {
    type Factory = ();
    type Texture = GlTexture;
    type RenderTarget = ();

    fn get_device_transform(size: PhysPixelSize) -> PhysPixelToDeviceTransform {
        GlBackend::get_device_transform(size)
    }

    fn get_factory(&self) -> Self::Factory {
        self.gl_backend.get_factory()
    }

    fn create_texture(&mut self, memory: &[u8], width: u16, height: u16, updatable: bool) -> Self::Texture {
        self.gl_backend.create_texture(memory, width, height, updatable)
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
        color: &Color, vertices: &[Point],
        transform: UnknownToDeviceTransform) {
        self.gl_backend.triangles_colored(target, color, vertices, transform)
    }

    fn triangles_textured(&mut self, target: &Self::RenderTarget,
        color: &Color, texture: &Self::Texture,
        filtering: bool,
		vertices: &[Point], uv: &[Point],
		transform: UnknownToDeviceTransform) {
        self.gl_backend.triangles_textured(target, color, texture, filtering, vertices, uv, transform)
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
    type RenderTarget = ();

    fn get_device_transform(size: PhysPixelSize) -> PhysPixelToDeviceTransform {
        PhysPixelToDeviceTransform::column_major(
            2.0f32 / size.width, 0.0f32, -1.0f32,
            0.0f32, -2.0f32 / size.height, 1.0f32,
        )
    }

    fn get_factory(&self) -> Self::Factory {
    }

    fn create_texture(&mut self, memory: &[u8], width: u16, height: u16, updatable: bool) -> Self::Texture {
        Self::Texture::create(&mut self.factory, memory, width, height, updatable).unwrap()
    }

    fn get_main_render_target(&mut self)-> Self::RenderTarget {
    }

    fn create_render_target(&mut self, width: u16, height: u16) -> (Self::Texture, Self::RenderTarget) {
        (GlTexture { }, ())
    }

	fn begin(&mut self) {
	}

    fn clear(&mut self, target: &Self::RenderTarget, color: &Color) {
        unsafe {
            gl::ClearColor(color[0], color[1], color[2], color[3]);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    fn triangles_colored(&mut self, target: &Self::RenderTarget,
        color: &Color, vertices: &[Point],
        transform: UnknownToDeviceTransform) {
    }

    fn triangles_textured(&mut self, target: &Self::RenderTarget,
        color: &Color, texture: &Self::Texture,
        filtering: bool,
		vertices: &[Point], uv: &[Point],
		transform: UnknownToDeviceTransform) {
    }

    fn line(&mut self, target: &Self::RenderTarget,
        color: &Color, thickness: DeviceThickness,
		start_point: Point, end_point: Point,
		transform: UnknownToDeviceTransform) {
        // TODO:
		//if thickness == 1.0f32 {
            self.line_native(target, color, start_point, end_point, transform);
        //} else {
            //self.line_triangulated(color, thickness, start_point, end_point, transform);
        //} 
	}

	fn end(&mut self) {
	}
}

impl GlBackend {
	fn line_native(&mut self, target: &(),
        color: &Color, start_point: Point, end_point: Point,
		transform: UnknownToDeviceTransform) {
    }

    fn line_triangulated(&mut self, target: (),
        color: &Color, thickness: DeviceThickness,
        start_point: Point, end_point: Point,
		transform: UnknownToDeviceTransform) {
    }
}
