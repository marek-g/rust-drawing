extern crate winit;

use color::*;
use units::*;

pub trait Backend {
	type Texture: Texture;

	fn get_device_transform(size: PhysPixelSize) -> PhysPixelToDeviceTransform;

	fn begin(&mut self);

	fn triangles_colored(&mut self, color: &Color, vertices: &[Point],
		transform: UnknownToDeviceTransform);

	fn triangles_textured(&mut self, color: &Color, texture: &Self::Texture,
		vertices: &[Point], uv: &[Point],
		transform: UnknownToDeviceTransform);

	fn end(&mut self);

	fn line(&mut self, color: &Color, thickness: DeviceThickness,
		start_point: Point, end_point: Point,
		transform: UnknownToDeviceTransform);

	fn rect(&mut self, color: &Color, rect: Rect,
        transform: UnknownToDeviceTransform) {
        let p1 = [ rect.origin.x, rect.origin.y ];
        let p2 = [ rect.origin.x + rect.size.width, rect.origin.y + rect.size.height ];

		self.triangles_colored(color, &[
			Point::new(p1[0], p1[1]), Point::new(p2[0], p1[1]), Point::new(p1[0], p2[1]),
			Point::new(p2[0], p1[1]), Point::new(p2[0], p2[1]), Point::new(p1[0], p2[1]),
			], transform);
	}
}

pub trait WindowBackend : Backend {
	fn create_backend_window(window_builder: winit::WindowBuilder,
		events_loop: &winit::EventsLoop) -> Self;

	fn update_window_size(&mut self, width: u16, height: u16);
}

pub trait TextureBackend : Backend {
	fn create_texure_backend(width: u16, height: u16) -> Self;
}

pub trait Texture {

}