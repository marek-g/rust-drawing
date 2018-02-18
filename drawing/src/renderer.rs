extern crate image;

use backend::WindowBackend;
use primitive::Primitive;
use units::*;

pub struct Renderer<B: WindowBackend> {
	backend: B
}

impl<B: WindowBackend> Renderer<B> {
	pub fn new(backend: B) -> Renderer<B> {
		Renderer { backend: backend }
	}

	pub fn update_window_size(&mut self, width: u16, height: u16) {
		self.backend.update_window_size(width, height)
	}

	pub fn draw(&mut self, size: PhysPixelSize, primitives: Vec<Primitive>) {
		let physical_pixel_to_device_transform = B::get_device_transform(size);
		let user_pixel_to_physical_pixel_transform = UserPixelToPhysPixelTransform::identity();
		let user_pixel_to_device_transform = user_pixel_to_physical_pixel_transform
			.post_mul(&physical_pixel_to_device_transform);
		let unknown_to_device_transform = UnknownToDeviceTransform::from_row_major_array(
			user_pixel_to_device_transform.to_row_major_array()
		);

		self.backend.begin();

		for primitive in &primitives {
            match primitive {
				&Primitive::Line { ref color, thickness, start_point, end_point } => {
					let thickness = user_pixel_to_device_transform.transform_point(
						&UserPixelPoint::new(thickness.get(), thickness.get())
					).x_typed();
					self.backend.line(color, thickness,
						start_point.to_untyped(), end_point.to_untyped(),
						unknown_to_device_transform);
				},
				
				&Primitive::Rectangle { ref color, rect } => {
					self.backend.rect(color, rect.to_untyped(),
						unknown_to_device_transform)
				},

				&Primitive::Text { .. } => {

				},

				&Primitive::Image { rect, path } => {
					let img = image::open(path).unwrap().to_rgba();
					let (w, h) = img.dimensions();
					let data: &[u8] = &img;
					let texture = self.backend.create_texture(data, w as u16, h as u16);
					self.backend.texture(&[1.0f32, 1.0f32, 1.0f32, 1.0f32], &texture, rect.to_untyped(),
						unknown_to_device_transform)
				},

				&Primitive::PushLayer { .. } => {

				},

				&Primitive::PopLayer { .. } => {

				}
			}
		}

		self.backend.end();
	}
}