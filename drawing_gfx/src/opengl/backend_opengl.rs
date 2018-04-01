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
use gfx::Factory;
use gfx::traits::FactoryExt;
use backend::gfx_core::Device;
use backend::drawing::backend::Texture;

pub type GfxResources = gfx_device_gl::Resources;
pub type GfxFactory = gfx_device_gl::Factory;

#[derive(Clone, Debug, PartialEq)]
pub struct GfxTexture {
    pub surface: gfx::handle::Texture<gfx_device_gl::Resources, gfx::format::R8_G8_B8_A8>,
    pub srv: gfx::handle::ShaderResourceView<gfx_device_gl::Resources, [f32; 4]>
}

impl drawing::backend::Texture for GfxTexture {
    type Factory = gfx_device_gl::Factory;
    type Encoder = gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>;
	type Error = gfx::CombinedError;
    type Error2 = gfx::UpdateError<[u16; 3]>;

	fn create(factory: &mut Self::Factory, memory: &[u8],
		width: u16, height: u16) -> Result<Self, Self::Error> {
        let kind = gfx::texture::Kind::D2(width, height, gfx::texture::AaMode::Single);
        let (surface, srv) = factory.create_texture_immutable_u8::<gfx::format::Rgba8>(
            kind, gfx::texture::Mipmap::Provided, &[&memory])?;
        Ok(GfxTexture {
            surface: surface,
            srv: srv
        })
    }

	fn update(&mut self, encoder: &mut Self::Encoder, memory: &[u8],
		offset_x: u16, offset_y: u16, width: u16, height: u16) -> Result<(), Self::Error2> {
        let img_info = gfx::texture::ImageInfoCommon {
            xoffset: offset_x,
            yoffset: offset_y,
            zoffset: 0,
            width: width,
            height: height,
            depth: 0,
            format: (),
            mipmap: 0,
        };
        let data = gfx::memory::cast_slice(memory);
        encoder.update_texture::<_, gfx::format::Rgba8>(&self.surface, None, img_info, data).map_err(Into::into)
    }

    fn get_size(&self) -> (u16, u16) {
        let (w, h, _, _) = self.surface.get_info().kind.get_dimensions();
        (w, h)
    }
}

pub struct GfxBackend {
	device: gfx_device_gl::Device,
	factory: gfx_device_gl::Factory,
	target_view: gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColorFormat>,
    stencil_view: gfx::handle::DepthStencilView<gfx_device_gl::Resources, DepthFormat>,
	encoder: gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>,
	color_pipeline_triangles: gfx::PipelineState<gfx_device_gl::Resources, ColorPipeline::Meta>,
	color_pipeline_lines: gfx::PipelineState<gfx_device_gl::Resources, ColorPipeline::Meta>,
	textured_pipeline_triangles: gfx::PipelineState<gfx_device_gl::Resources, TexturedPipeline::Meta>
}

pub struct GfxWindowBackend {
    window: glutin::GlWindow,
    gfx_backend: GfxBackend
}

pub trait GfxBackendExt<R> where
    R: gfx::Resources {
    type CommandBuffer: gfx::CommandBuffer<R>;
    fn get_encoder(&mut self) -> &mut gfx::Encoder<R, Self::CommandBuffer>;
}

impl GfxBackendExt<gfx_device_gl::Resources> for GfxBackend {
    type CommandBuffer = gfx_device_gl::CommandBuffer;
    fn get_encoder(&mut self) -> &mut gfx::Encoder<gfx_device_gl::Resources, Self::CommandBuffer> {
        &mut self.encoder
    }
}

impl GfxBackendExt<gfx_device_gl::Resources> for GfxWindowBackend {
    type CommandBuffer = gfx_device_gl::CommandBuffer;
    fn get_encoder(&mut self) -> &mut gfx::Encoder<gfx_device_gl::Resources, Self::CommandBuffer> {
        self.gfx_backend.get_encoder()
    }
}

impl drawing::backend::WindowBackend for GfxWindowBackend {
    fn create_window_backend(window_builder: winit::WindowBuilder,
		events_loop: &winit::EventsLoop) -> Self {
        let context = glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 2)))
            .with_vsync(true);
		let (window, device, mut factory, target_view, depth_stencil_view) =
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

        let encoder = gfx::Encoder::from(factory.create_command_buffer());

		GfxWindowBackend {
            window: window,
            gfx_backend: GfxBackend {
                device: device, factory: factory,
                target_view: target_view,
                stencil_view: depth_stencil_view,
			    encoder: encoder,
			    color_pipeline_triangles: color_pipeline_triangles,
			    color_pipeline_lines: color_pipeline_lines,
			    textured_pipeline_triangles: textured_pipeline_triangles
            }
        }
	}

    fn update_window_size(&mut self, _width: u16, _height: u16) {
        gfx_window_glutin::update_views(&self.window,
            &mut self.gfx_backend.target_view, &mut self.gfx_backend.stencil_view);
    }
}

impl drawing::backend::Backend for GfxWindowBackend {
    type Factory = gfx_device_gl::Factory;
    type Texture = GfxTexture;
    type RenderTarget = gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColorFormat>;

    fn get_device_transform(size: PhysPixelSize) -> PhysPixelToDeviceTransform {
        GfxBackend::get_device_transform(size)
    }

    fn get_factory(&self) -> Self::Factory {
        self.gfx_backend.get_factory()
    }

    fn create_texture(&mut self, memory: &[u8], width: u16, height: u16) -> Self::Texture {
        self.gfx_backend.create_texture(memory, width, height)
    }

    fn get_main_render_target(&mut self)-> Self::RenderTarget {
        self.gfx_backend.get_main_render_target()
    }

    fn create_render_target(&mut self, width: u16, height: u16) -> (Self::Texture, Self::RenderTarget) {
        self.gfx_backend.create_render_target(width, height)
    }

    fn begin(&mut self) {
        self.gfx_backend.begin()
    }

    fn clear(&mut self, target: &Self::RenderTarget,
        color: &Color) {
        self.gfx_backend.clear(target, color)
    }

    fn triangles_colored(&mut self, target: &Self::RenderTarget,
        color: &Color, vertices: &[Point],
        transform: UnknownToDeviceTransform) {
        self.gfx_backend.triangles_colored(target, color, vertices, transform)
    }

    fn triangles_textured(&mut self, target: &Self::RenderTarget,
        color: &Color, texture: &Self::Texture,
        filtering: bool,
		vertices: &[Point], uv: &[Point],
		transform: UnknownToDeviceTransform) {
        self.gfx_backend.triangles_textured(target, color, texture, filtering, vertices, uv, transform)
    }

    fn line(&mut self, target: &Self::RenderTarget,
        color: &Color, thickness: DeviceThickness,
		start_point: Point, end_point: Point,
		transform: UnknownToDeviceTransform) {
        self.gfx_backend.line(target, color, thickness, start_point, end_point, transform)
    }

    fn end(&mut self) {
        self.gfx_backend.end();
        self.window.swap_buffers().unwrap();
    }
}

impl drawing::backend::Backend for GfxBackend {
    type Factory = gfx_device_gl::Factory;
    type Texture = GfxTexture;
    type RenderTarget = gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColorFormat>;

    fn get_device_transform(size: PhysPixelSize) -> PhysPixelToDeviceTransform {
        PhysPixelToDeviceTransform::column_major(
            2.0f32 / size.width, 0.0f32, -1.0f32,
            0.0f32, -2.0f32 / size.height, 1.0f32,
        )
    }

    fn get_factory(&self) -> Self::Factory {
        self.factory.clone()
    }

    fn create_texture(&mut self, memory: &[u8], width: u16, height: u16) -> Self::Texture {
        Self::Texture::create(&mut self.factory, memory, width, height).unwrap()
    }

    fn get_main_render_target(&mut self)-> Self::RenderTarget {
        self.target_view.clone()
    }

    fn create_render_target(&mut self, width: u16, height: u16) -> (Self::Texture, Self::RenderTarget) {
        let (texture, srv, render_target) = self.factory.create_render_target(width, height).unwrap();
        (GfxTexture { surface: texture, srv: srv }, render_target)
    }

	fn begin(&mut self) {
	}

    fn clear(&mut self, target: &Self::RenderTarget, color: &Color) {
        self.encoder.clear(target, *color);
    }

    fn triangles_colored(&mut self, target: &Self::RenderTarget,
        color: &Color, vertices: &[Point],
        transform: UnknownToDeviceTransform) {
        let VERTICES: Vec<ColorVertex> = vertices.iter().map(|&point| ColorVertex {
            pos: [ point.x, point.y], color: *color
        }).collect();

        let (vertex_buffer, slice) = self.factory.create_vertex_buffer_with_slice(&VERTICES, ());

        let transform = [[transform.m11, transform.m12, 0.0, 0.0],
            [transform.m21, transform.m22, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [transform.m31, transform.m32, 0.0, 1.0]];

        let data = ColorPipeline::Data {
            vbuf: vertex_buffer,
            locals: self.factory.create_constant_buffer(1),
            out: target.clone()
        };

        let locals = Locals { transform: transform };
        self.encoder.update_constant_buffer(&data.locals, &locals);

        self.encoder.draw(&slice, &self.color_pipeline_triangles, &data);
    }

    fn triangles_textured(&mut self, target: &Self::RenderTarget,
        color: &Color, texture: &Self::Texture,
        filtering: bool,
		vertices: &[Point], uv: &[Point],
		transform: UnknownToDeviceTransform) {
        let VERTICES: Vec<TexturedVertex> = vertices.iter().zip(uv.iter()).map(|(&point, &uv)| TexturedVertex {
            pos: [ point.x, point.y], tex_coord: [uv.x, uv.y]
        }).collect();

        let (vertex_buffer, slice) = self.factory.create_vertex_buffer_with_slice(&VERTICES, ());

        let transform = [[transform.m11, transform.m12, 0.0, 0.0],
            [transform.m21, transform.m22, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [transform.m31, transform.m32, 0.0, 1.0]];

        let sampler = self.factory.create_sampler(gfx::texture::SamplerInfo::new(
            if filtering { gfx::texture::FilterMethod::Trilinear }
            else { gfx::texture::FilterMethod::Scale },
            gfx::texture::WrapMode::Tile));

        let data = TexturedPipeline::Data {
            vbuf: vertex_buffer,
            locals: self.factory.create_constant_buffer(1),
            texture: (texture.srv.clone(), sampler),
            out: target.clone()
        };

        let locals = Locals { transform: transform };
        self.encoder.update_constant_buffer(&data.locals, &locals);

        self.encoder.draw(&slice, &self.textured_pipeline_triangles, &data);
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
        self.encoder.flush(&mut self.device);
        self.device.cleanup();
	}
}

impl GfxBackend {
	fn line_native(&mut self, target: &gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColorFormat>,
        color: &Color, start_point: Point, end_point: Point,
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

        let data = ColorPipeline::Data {
            vbuf: vertex_buffer,
            locals: self.factory.create_constant_buffer(1),
            out: target.clone()
        };

        let locals = Locals { transform: transform };
        self.encoder.update_constant_buffer(&data.locals, &locals);

        self.encoder.draw(&slice, &self.color_pipeline_lines, &data);
    }

    fn line_triangulated(&mut self, target: &gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColorFormat>,
        color: &Color, thickness: DeviceThickness,
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

        let data = ColorPipeline::Data {
            vbuf: vertex_buffer,
            locals: self.factory.create_constant_buffer(1),
            out: target.clone()
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
        out: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
    }

    pipeline TexturedPipeline {
        vbuf: gfx::VertexBuffer<TexturedVertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        texture: gfx::TextureSampler<[f32; 4]> = "t_Color",
        out: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
    }
}
