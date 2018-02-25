extern crate winit;

use color::*;
use units::*;

pub trait Backend {
	type Texture: Texture;
	type RenderTarget;

	fn get_device_transform(size: PhysPixelSize) -> PhysPixelToDeviceTransform;

	fn create_texture(&mut self, memory: &[u8], width: u16, height: u16) -> Self::Texture;

	fn get_main_render_target(&mut self)-> Self::RenderTarget;
	fn create_render_target(&mut self, width: u16, height: u16) -> (Self::Texture, Self::RenderTarget);

	fn begin(&mut self);

	fn clear(&mut self, target: &Self::RenderTarget, color: &Color);

	fn triangles_colored(&mut self, target: &Self::RenderTarget,
		color: &Color, vertices: &[Point],
		transform: UnknownToDeviceTransform);

	fn triangles_textured(&mut self, target: &Self::RenderTarget,
		color: &Color, texture: &Self::Texture,
		filtering: bool,
		vertices: &[Point], uv: &[Point],
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
			color,
			&[
				Point::new(p1[0], p1[1]), Point::new(p2[0], p1[1]), Point::new(p1[0], p2[1]),
				Point::new(p2[0], p1[1]), Point::new(p2[0], p2[1]), Point::new(p1[0], p2[1]),
			], transform);
	}

	fn rect_textured(&mut self, target: &Self::RenderTarget,
		color: &Color, texture: &Self::Texture,
		filtering: bool,
		rect: Rect, transform: UnknownToDeviceTransform) {
        let p1 = [ rect.origin.x, rect.origin.y ];
        let p2 = [ rect.origin.x + rect.size.width, rect.origin.y + rect.size.height ];

		self.triangles_textured(target,
			color, texture, filtering,
			&[
				Point::new(p1[0], p1[1]), Point::new(p2[0], p1[1]), Point::new(p1[0], p2[1]),
				Point::new(p2[0], p1[1]), Point::new(p2[0], p2[1]), Point::new(p1[0], p2[1]),
			],
			&[
				Point::new(0.0, 0.0), Point::new(1.0, 0.0), Point::new(0.0, 1.0),
				Point::new(1.0, 0.0), Point::new(1.0, 1.0), Point::new(0.0, 1.0),
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

	fn create(factory: &mut Self::Factory, memory: &[u8],
		width: u16, height: u16) -> Result<Self, Self::Error>;

	fn update(&mut self, encoder: &mut Self::Encoder, memory: &[u8],
		offset_x: u16, offset_y: u16, width: u16, height: u16) -> Result<(), Self::Error2>;

	fn get_size(&self) -> (u16, u16);
}
