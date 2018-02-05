extern crate drawing;
extern crate winit;
extern crate gfx;
extern crate gfx_core;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;

use self::drawing::color::*;
use self::drawing::units::*;
use self::glutin::GlContext;
use gfx::traits::FactoryExt;
use backend::gfx_core::Device;

pub struct GfxBackend {
	window: glutin::GlWindow,
	device: gfx_device_gl::Device,
	factory: gfx_device_gl::Factory,
	target_view: gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColorFormat>,
    stencil_view: gfx::handle::DepthStencilView<gfx_device_gl::Resources, DepthFormat>,
	encoder: gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>,
	color_pipeline_triangles: gfx::PipelineState<gfx_device_gl::Resources, ColorPipeline::Meta>,
	color_pipeline_lines: gfx::PipelineState<gfx_device_gl::Resources, ColorPipeline::Meta>,
	textured_pipeline_triangles: gfx::PipelineState<gfx_device_gl::Resources, TexturedPipeline::Meta>
}

impl drawing::backend::Backend for GfxBackend {
	fn create_backend_window(window_builder: winit::WindowBuilder,
		events_loop: &winit::EventsLoop) -> Self {
        let context = glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 2)))
            .with_vsync(true);
		let (window, mut device, mut factory, target_view, mut depth_stencil_view) =
            gfx_window_glutin::init::<ColorFormat, DepthFormat>(window_builder, context, &events_loop);

		let colored_vertex_shader = include_bytes!("shader/colored.glslv");
		let colored_pixel_shader = include_bytes!("shader/colored.glslf");
		let textured_vertex_shader = include_bytes!("shader/textured.glslv");
		let textured_pixel_shader = include_bytes!("shader/textured.glslf");

		let colored_shaderset = factory.create_shader_set(colored_vertex_shader, colored_pixel_shader).unwrap();
		let textured_shaderset = factory.create_shader_set(textured_vertex_shader, textured_pixel_shader).unwrap();

		let color_pipeline_triangles = factory.create_pipeline_state(
			&colored_shaderset,
			gfx::Primitive::TriangleList,
			gfx::state::Rasterizer::new_fill(),
			ColorPipeline::new()
		).unwrap();

		let color_pipeline_lines = factory.create_pipeline_state(
			&colored_shaderset,
			gfx::Primitive::LineList,
			gfx::state::Rasterizer::new_fill(),
			ColorPipeline::new()
		).unwrap();

		let textured_pipeline_triangles = factory.create_pipeline_state(
			&textured_shaderset,
			gfx::Primitive::TriangleList,
			gfx::state::Rasterizer::new_fill(),
			TexturedPipeline::new()
		).unwrap();

        let mut encoder = gfx::Encoder::from(factory.create_command_buffer());

		GfxBackend { window: window, device: device, factory: factory,
            target_view: target_view,
            stencil_view: depth_stencil_view,
			encoder: encoder,
			color_pipeline_triangles: color_pipeline_triangles,
			color_pipeline_lines: color_pipeline_lines,
			textured_pipeline_triangles: textured_pipeline_triangles }
	}

    fn update_window_size(&mut self, width: u16, height: u16) {
        gfx_window_glutin::update_views(&self.window, &mut self.target_view, &mut self.stencil_view);
        /*self.target_view = None;
        match gfx_window_glutin::update_views::<ColorFormat, gfx_device_gl::Device>(&mut self.window, &mut self.factory, &mut self.device, width, height) {
            Ok(target_view) => {
                self.target_view = Some(target_view);
            },
            Err(e) => println!("Resize failed: {}", e),
        }*/
    }

    fn get_device_transform(size: PhysPixelSize) -> PhysPixelToDeviceTransform {
        PhysPixelToDeviceTransform::column_major(
            2.0f32 / size.width, 0.0f32, -1.0f32,
            0.0f32, -2.0f32 / size.height, 1.0f32,
        )
    }

	fn begin(&mut self) {
        self.encoder.clear(&self.target_view, [0.5, 0.2, 0.3, 1.0]);
	}

    fn line(&mut self, color: &Color, thickness: DeviceThickness,
		start_point: Point, end_point: Point,
		transform: UnknownToDeviceTransform) {
        // TODO:
		//if thickness == 1.0f32 {
            self.line_native(color, start_point, end_point, transform);
        //} else {
            //self.line_triangulated(color, thickness, start_point, end_point, transform);
        //} 
	}
 
	fn rect(&mut self,
		color: &Color,
        rect: Rect,
        transform: UnknownToDeviceTransform) {
        let p1 = [ rect.origin.x, rect.origin.y ];
        let p2 = [ rect.origin.x + rect.size.width, rect.origin.y + rect.size.height ];

        let TRIANGLE: [ColorVertex; 6] = [
            ColorVertex { pos: [ p1[0], p1[1] ], color: *color },
            ColorVertex { pos: [ p2[0], p1[1] ], color: *color },
            ColorVertex { pos: [ p1[0], p2[1] ], color: *color },
            ColorVertex { pos: [ p2[0], p1[1] ], color: *color },
            ColorVertex { pos: [ p2[0], p2[1] ], color: *color },
            ColorVertex { pos: [ p1[0], p2[1] ], color: *color },
        ];
        let (vertex_buffer, slice) = self.factory.create_vertex_buffer_with_slice(&TRIANGLE, ());

        let transform = [[transform.m11, transform.m12, 0.0, 0.0],
            [transform.m21, transform.m22, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [transform.m31, transform.m32, 0.0, 1.0]];

        let mut data = ColorPipeline::Data {
            vbuf: vertex_buffer,
            locals: self.factory.create_constant_buffer(1),
            transform: transform,
            out: self.target_view.clone()
        };

        let locals = Locals { transform: transform };
        self.encoder.update_constant_buffer(&data.locals, &locals);

        self.encoder.draw(&slice, &self.color_pipeline_triangles, &data);
	}

	fn end(&mut self) {
        self.encoder.flush(&mut self.device);
        self.window.swap_buffers().unwrap();
        self.device.cleanup();
	}
}

impl GfxBackend {
	fn line_native(&mut self, color: &Color, start_point: Point, end_point: Point,
		transform: UnknownToDeviceTransform) {
        let LINE: [ColorVertex; 2] = [
            ColorVertex { pos: [ start_point.x, start_point.y ], color: *color },
            ColorVertex { pos: [ end_point.x, end_point.y ], color: *color },
        ];
        let (vertex_buffer, slice) = self.factory.create_vertex_buffer_with_slice(&LINE, ());

        let transform = [[transform.m11, transform.m12, 0.0, 0.0],
            [transform.m21, transform.m22, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [transform.m31, transform.m32, 0.0, 1.0]];

        let mut data = ColorPipeline::Data {
            vbuf: vertex_buffer,
            locals: self.factory.create_constant_buffer(1),
            transform: transform,
            out: self.target_view.clone()
        };

        let locals = Locals { transform: transform };
        self.encoder.update_constant_buffer(&data.locals, &locals);

        self.encoder.draw(&slice, &self.color_pipeline_lines, &data);
    }

    fn line_triangulated(&mut self, color: &Color, thickness: DeviceThickness,
        start_point: Point, end_point: Point,
		transform: UnknownToDeviceTransform) {
        let len = (((start_point.x - end_point.x)*(start_point.x - end_point.x) +
            (start_point.y - end_point.y)*(start_point.y - start_point.y))  as f32).sqrt();
        let normal_x = (end_point.y - start_point.y) / len;
        let normal_y = -(start_point.x - end_point.x) / len;

        let diff_x = normal_x * thickness.get() * 0.5f32;
        let diff_y = normal_y * thickness.get() * 0.5f32;
        let p1a_x = start_point.x - diff_x;
        let p1a_y = start_point.y - diff_y;
        let p1b_x = start_point.x + diff_x;
        let p1b_y = start_point.y + diff_y;
        let p2a_x = end_point.x - diff_x;
        let p2a_y = end_point.y - diff_y;
        let p2b_x = end_point.x + diff_x;
        let p2b_y = end_point.y + diff_y;;

        let TRIANGLE: [ColorVertex; 6] = [
            ColorVertex { pos: [ p1a_x, p1a_y ], color: *color },
            ColorVertex { pos: [ p2a_x, p2a_y ], color: *color },
            ColorVertex { pos: [ p1b_x, p1b_y ], color: *color },
            ColorVertex { pos: [ p1b_x, p1b_y ], color: *color },
            ColorVertex { pos: [ p2a_x, p2a_y ], color: *color },
            ColorVertex { pos: [ p2b_x, p2b_y ], color: *color },
        ];
        let (vertex_buffer, slice) = self.factory.create_vertex_buffer_with_slice(&TRIANGLE, ());

        let transform = [[transform.m11, transform.m12, 0.0, 0.0],
            [transform.m21, transform.m22, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [transform.m31, transform.m32, 0.0, 1.0]];

        let mut data = ColorPipeline::Data {
            vbuf: vertex_buffer,
            locals: self.factory.create_constant_buffer(1),
            transform: transform,
            out: self.target_view.clone()
        };

        let locals = Locals { transform: transform };
        self.encoder.update_constant_buffer(&data.locals, &locals);

        self.encoder.draw(&slice, &self.color_pipeline_triangles, &data);
    }
}

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines! {
    vertex ColorVertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 4] = "a_Color",
    }

    vertex TexturedVertex {
        pos: [f32; 2] = "a_Pos",
        tex_coord: [f32; 2] = "a_TexCoord",
    }

    constant Locals {
        transform: [[f32; 4]; 4] = "u_Transform",
    }

    pipeline ColorPipeline {
        vbuf: gfx::VertexBuffer<ColorVertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        transform: gfx::Global<[[f32; 4]; 4]> = "u_Transform",
        //out: gfx::RenderTarget<ColorFormat> = "Target0",
        out: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
    }

    pipeline TexturedPipeline {
        vbuf: gfx::VertexBuffer<TexturedVertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        transform: gfx::Global<[[f32; 4]; 4]> = "u_Transform",
        color: gfx::TextureSampler<[f32; 4]> = "t_Color",
        out: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
    }
}
