extern crate image;

use backend::WindowBackend;
use primitive::Primitive;
use units::*;
use font::*;

use std::io::prelude::*;
use std::fs::File;

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
		let target_view = self.backend.get_main_render_target();
		self.backend.clear(&target_view, &[0.5f32, 0.4f32, 0.3f32, 1.0f32]);

		for primitive in &primitives {
            match primitive {
				&Primitive::Line { ref color, thickness, start_point, end_point } => {
					let thickness = user_pixel_to_device_transform.transform_point(
						&UserPixelPoint::new(thickness.get(), thickness.get())
					).x_typed();
					self.backend.line(&target_view, color, thickness,
						start_point.to_untyped(), end_point.to_untyped(),
						unknown_to_device_transform);
				},
				
				&Primitive::Rectangle { ref color, rect } => {
					self.backend.rect_colored(&target_view, color, rect.to_untyped(),
						unknown_to_device_transform)
				},

				&Primitive::Text { font_path, ref color, position, size, text } => {
					let mut file = File::open(font_path).unwrap();
					let mut buffer = Vec::new();
					file.read_to_end(&mut buffer);

					let mut font = self.backend.create_font(&buffer, FontParams { size: size });
					font.add_text(color, text, position.to_untyped(), unknown_to_device_transform);
					self.backend.draw_font(&mut font, &target_view);
					
					let dims = font.get_dimensions(text);
					self.backend.rect_colored(&target_view, &[0.0f32, 0.0f32, 0.0f32, 0.5f32],
						UserPixelRect::new(
							UserPixelPoint::new(position.to_untyped().x, position.to_untyped().y),
							UserPixelSize::new(dims.0 as f32, dims.1 as f32),
						).to_untyped(),
						unknown_to_device_transform);
				},

				&Primitive::Image { rect, path } => {
					/*let img = image::open(path).unwrap().to_rgba();
					let (w, h) = img.dimensions();
					let data: &[u8] = &img;*/

					let w = rect.to_untyped().size.width as usize;
					let h = rect.to_untyped().size.height as usize;
					let mut data: Vec<u8> = Vec::with_capacity(w*h*4);
					for y in 0..h {
						for x in 0..w {
							let color: u8 = if ((x + y)/1 % 2) == 0 { 255 } else { 0 };
							data.push(color);
							data.push(color);
							data.push(color);
							data.push(255);
						}
					}


					let texture = self.backend.create_texture(&data, w as u16, h as u16);
					self.backend.rect_textured(&target_view,
						&[1.0f32, 1.0f32, 1.0f32, 1.0f32], &texture, false,
						rect.to_untyped(), unknown_to_device_transform);

					/*let (texture2, texture2_view) = self.backend.create_render_target(w as u16, h as u16);
					/*self.backend.line(&texture_view, &[1.0f32, 1.0f32, 0.3f32, 1.0f32],
						DeviceThickness::new(1.0f), )*/
					self.backend.clear(&texture2_view, &[1.0f32, 1.0f32, 0.3f32, 0.2f32]);
					self.backend.rect_colored(&texture2_view, &[1.0f32, 1.0f32, 0.3f32, 1.0f32], rect.to_untyped(),
						unknown_to_device_transform);
					self.backend.rect_textured(&target_view,
						&[1.0f32, 1.0f32, 1.0f32, 1.0f32], &texture2, false,
						rect.to_untyped(), unknown_to_device_transform);*/
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