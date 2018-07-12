extern crate winit;

use color::*;
use units::*;

#[repr(C, packed)]
#[derive(Clone)]
pub struct ColoredVertex {
    pub pos: [f32; 2],
    pub color: [f32; 4],
}

impl ColoredVertex {
	pub fn new(pos: [f32; 2], color: [f32; 4]) -> Self {
		ColoredVertex { pos, color }
	}
}

#[repr(C, packed)]
#[derive(Clone)]
pub struct TexturedVertex {
    pub pos: [f32; 2],
    pub tex_coords: [f32; 2],
}

impl TexturedVertex {
	pub fn new(pos: [f32; 2], tex_coords: [f32; 2]) -> Self {
		TexturedVertex { pos, tex_coords }
	}
}

#[repr(C, packed)]
#[derive(Clone)]
pub struct TexturedY8Vertex {
    pub pos: [f32; 2],
    pub tex_coords: [f32; 2],
    pub color: [f32; 4],
}

impl TexturedY8Vertex {
	pub fn new(pos: [f32; 2], tex_coords: [f32; 2], color: [f32; 4]) -> Self {
		TexturedY8Vertex { pos, tex_coords, color }
	}
}

pub trait Backend {
	type Factory;
	type Texture: Texture;
	type RenderTarget;

	fn get_device_transform(size: PhysPixelSize) -> PhysPixelToDeviceTransform;

	/// Device specific factory. Can be used by extensions to create shaders etc.
	fn get_factory(&self) -> Self::Factory;

	fn create_texture(&mut self, memory: Option<&[u8]>, width: u16, height: u16, format: ColorFormat, updatable: bool) -> Self::Texture;

	fn get_main_render_target(&mut self)-> Self::RenderTarget;
	fn create_render_target(&mut self, width: u16, height: u16) -> (Self::Texture, Self::RenderTarget);

	fn begin(&mut self);

	fn clear(&mut self, target: &Self::RenderTarget, color: &Color);

	fn triangles_colored(&mut self, target: &Self::RenderTarget,
		vertices: &[ColoredVertex],
		transform: UnknownToDeviceTransform);

	fn triangles_textured(&mut self, target: &Self::RenderTarget,
		texture: &Self::Texture, filtering: bool,
		vertices: &[TexturedVertex],
		transform: UnknownToDeviceTransform);

	fn triangles_textured_y8(&mut self, target: &Self::RenderTarget,
		texture: &Self::Texture, filtering: bool,
		vertices: &[TexturedY8Vertex],
		transform: UnknownToDeviceTransform);

	fn end(&mut self);

	fn line(&mut self, target: &Self::RenderTarget,
		color: &Color, thickness: DeviceThickness,
		start_point: Point, end_point: Point,
		transform: UnknownToDeviceTransform);

	fn rect_colored(&mut self, target: &Self::RenderTarget,
		color: &Color, rect: Rect,
        transform: UnknownToDeviceTransform) {
        let p1 = [ rect.origin.x, rect.origin.y ];
        let p2 = [ rect.origin.x + rect.size.width, rect.origin.y + rect.size.height ];

		self.triangles_colored(target,
			&[
				ColoredVertex::new([p1[0], p1[1]], *color),
				ColoredVertex::new([p2[0], p1[1]], *color),
				ColoredVertex::new([p1[0], p2[1]], *color),

				ColoredVertex::new([p2[0], p1[1]], *color),
				ColoredVertex::new([p2[0], p2[1]], *color),
				ColoredVertex::new([p1[0], p2[1]], *color),
			], transform);
	}

	fn rect_textured(&mut self, target: &Self::RenderTarget,
		texture: &Self::Texture, filtering: bool,
		rect: Rect, transform: UnknownToDeviceTransform) {
        self.rect_textured_sub(target, texture, filtering, rect, &[0.0, 0.0], &[1.0, 1.0], transform)
	}

	fn rect_textured_sub(&mut self, target: &Self::RenderTarget,
		texture: &Self::Texture, filtering: bool,
		rect: Rect, uv1: &[f32; 2], uv2: &[f32; 2],
		transform: UnknownToDeviceTransform) {
        let p1 = [ rect.origin.x, rect.origin.y ];
        let p2 = [ rect.origin.x + rect.size.width, rect.origin.y + rect.size.height ];

		self.triangles_textured(target,
			texture, filtering,
			&[
				TexturedVertex::new([p1[0], p1[1]], [uv1[0], uv1[1]]),
				TexturedVertex::new([p2[0], p1[1]], [uv2[0], uv1[1]]),
				TexturedVertex::new([p1[0], p2[1]], [uv1[0], uv2[1]]),

				TexturedVertex::new([p2[0], p1[1]], [uv2[0], uv1[1]]),
				TexturedVertex::new([p2[0], p2[1]], [uv2[0], uv2[1]]),
				TexturedVertex::new([p1[0], p2[1]], [uv1[0], uv2[1]]),
			],
			transform);
	}

	fn rect_textured_y8(&mut self, target: &Self::RenderTarget,
		texture: &Self::Texture, filtering: bool, color: &Color,
		rect: Rect,
		transform: UnknownToDeviceTransform) {
        self.rect_textured_y8_sub(target, texture, filtering, color, rect, &[0.0, 0.0], &[1.0, 1.0], transform)
	}

	fn rect_textured_y8_sub(&mut self, target: &Self::RenderTarget,
		texture: &Self::Texture, filtering: bool, color: &Color,
		rect: Rect, uv1: &[f32; 2], uv2: &[f32; 2],
		transform: UnknownToDeviceTransform) {
        let p1 = [ rect.origin.x, rect.origin.y ];
        let p2 = [ rect.origin.x + rect.size.width, rect.origin.y + rect.size.height ];

		self.triangles_textured_y8(target,
			texture, filtering,
			&[
				TexturedY8Vertex::new([p1[0], p1[1]], [uv1[0], uv1[1]], *color),
				TexturedY8Vertex::new([p2[0], p1[1]], [uv2[0], uv1[1]], *color),
				TexturedY8Vertex::new([p1[0], p2[1]], [uv1[0], uv2[1]], *color),

				TexturedY8Vertex::new([p2[0], p1[1]], [uv2[0], uv1[1]], *color),
				TexturedY8Vertex::new([p2[0], p2[1]], [uv2[0], uv2[1]], *color),
				TexturedY8Vertex::new([p1[0], p2[1]], [uv1[0], uv2[1]], *color),
			],
			transform);
	}
}

pub trait WindowBackend : Backend {
	fn create_window_backend(window_builder: winit::WindowBuilder,
		events_loop: &winit::EventsLoop) -> Self;

	fn update_window_size(&mut self, width: u16, height: u16);
}

pub trait Texture : Sized {
	type Factory;
	type Encoder;
	type Error;
	type Error2;

	fn create(factory: &mut Self::Factory, memory: Option<&[u8]>,
		width: u16, height: u16, format: ColorFormat, updatable: bool) -> Result<Self, Self::Error>;

	fn update(&mut self, encoder: &mut Self::Encoder, memory: &[u8],
		offset_x: u16, offset_y: u16, width: u16, height: u16) -> Result<(), Self::Error2>;

	fn get_size(&self) -> (u16, u16);
}
