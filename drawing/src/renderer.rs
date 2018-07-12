extern crate image;

use font::FontParams;
use backend::WindowBackend;
use font::Font;
use primitive::Primitive;
use resources::*;
use units::*;

pub struct Renderer<B: WindowBackend> {
	backend: B
}

impl<B: WindowBackend> Renderer<B> {
	pub fn new(backend: B) -> Renderer<B> {
		Renderer {
			backend: backend
		}
	}

	pub fn backend(&mut self) -> &mut B {
		&mut self.backend
	}

	pub fn update_window_size(&mut self, width: u16, height: u16) {
		self.backend.update_window_size(width, height)
	}

	pub fn draw<F: Font<B>>(&mut self, size: PhysPixelSize,
		primitives: Vec<Primitive>,
		resources: &mut Resources<B::Texture, F>) {
		let physical_pixel_to_device_transform = B::get_device_transform(size);
		let user_pixel_to_physical_pixel_transform = UserPixelToPhysPixelTransform::identity();
		let user_pixel_to_device_transform = user_pixel_to_physical_pixel_transform
			.post_mul(&physical_pixel_to_device_transform);
		let unknown_to_device_transform = UnknownToDeviceTransform::from_row_major_array(
			user_pixel_to_device_transform.to_row_major_array()
		);

		self.backend.begin();
		let mut target_view = self.backend.get_main_render_target();
		let mut target_texture: Option<B::Texture> = None;
		let mut target_color = [1.0f32, 1.0f32, 1.0f32, 1.0f32];
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

				&Primitive::Text { ref resource_key, ref color, position, size, ref text } => {
					if let Some(font) = resources.fonts_mut().get_mut(&resource_key.to_string()) {
						font.draw(&mut self.backend, &target_view, color, text,
							position.to_untyped(), FontParams { size: size as u8 }, unknown_to_device_transform);
					}
				},

				&Primitive::Image { resource_key, rect } => {
					if let Some(texture) = resources.textures_mut().get(&resource_key) {
						self.backend.rect_textured(&target_view,
							&texture, false, &[1.0f32, 1.0f32, 1.0f32, 1.0f32],
							rect.to_untyped(), unknown_to_device_transform);
					}
				},

				&Primitive::PushLayer { ref color } => {
					let (texture2, texture2_view) = self.backend.create_render_target(size.width as u16, size.height as u16);
					target_view = texture2_view;
					target_texture = Some(texture2);
					target_color = *color;
					self.backend.clear(&target_view, &[0.0f32, 0.0f32, 0.0f32, 0.0f32]);
				},

				&Primitive::PopLayer { .. } => {
					let main_view = self.backend.get_main_render_target();
					if let Some(ref target_texture) = target_texture {
						self.backend.rect_textured(&main_view,
							&target_texture, false, &target_color,
							Rect::new(Point::new(0.0f32, 0.0f32), Size::new(size.width, size.height)),
							unknown_to_device_transform);
					}
					target_view = main_view;
					target_texture = None;
				}
			}
		}

		self.backend.end();
	}
}