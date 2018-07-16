extern crate image;

use ::Result;
use font::FontParams;
use backend::Device;
use color::Color;
use font::Font;
use primitive::Primitive;
use resources::*;
use units::*;

pub struct Renderer;

impl Renderer {
	pub fn new() -> Self {
		Renderer { }
	}

	pub fn draw<D: Device, F: Font<D>>(&mut self,
		device: &mut D,
		render_target: &D::RenderTarget,
		size: PhysPixelSize,
		primitives: Vec<Primitive>,
		resources: &mut Resources<D, F>) -> Result<()> {
		let physical_pixel_to_device_transform = D::get_device_transform(size);
		let user_pixel_to_physical_pixel_transform = UserPixelToPhysPixelTransform::identity();
		let user_pixel_to_device_transform = user_pixel_to_physical_pixel_transform
			.post_mul(&physical_pixel_to_device_transform);
		let unknown_to_device_transform = UnknownToDeviceTransform::from_row_major_array(
			user_pixel_to_device_transform.to_row_major_array()
		);

		let mut pushed_render_target: Option<(D::Texture, D::RenderTarget, Color)> = None;

		device.begin(render_target);
		device.clear(render_target, &[0.5f32, 0.4f32, 0.3f32, 1.0f32]);

		for primitive in &primitives {
            match primitive {
				&Primitive::Line { ref color, thickness, start_point, end_point } => {
					let thickness = user_pixel_to_device_transform.transform_point(
						&UserPixelPoint::new(thickness.get(), thickness.get())
					).x_typed();

					let target_view = if let Some(ref pushed_render_target) = pushed_render_target { &pushed_render_target.1 } else { render_target };

					device.line(&target_view, color, thickness,
						start_point.to_untyped(), end_point.to_untyped(),
						unknown_to_device_transform);
				},
				
				&Primitive::Rectangle { ref color, rect } => {
					let target_view = if let Some(ref pushed_render_target) = pushed_render_target { &pushed_render_target.1 } else { render_target };

					device.rect_colored(&target_view, color, rect.to_untyped(),
						unknown_to_device_transform)
				},

				&Primitive::Text { ref resource_key, ref color, position, size, ref text } => {
					let target_view = if let Some(ref pushed_render_target) = pushed_render_target { &pushed_render_target.1 } else { render_target };

					if let Some(font) = resources.fonts_mut().get_mut(&resource_key.to_string()) {
						font.draw(device, &target_view, color, text,
							position.to_untyped(), FontParams { size: size as u8 }, unknown_to_device_transform)?;
					}
				},

				&Primitive::Image { resource_key, rect } => {
					let target_view = if let Some(ref pushed_render_target) = pushed_render_target { &pushed_render_target.1 } else { render_target };

					if let Some(texture) = resources.textures_mut().get(&resource_key) {
						device.rect_textured(&target_view,
							&texture, false, &[1.0f32, 1.0f32, 1.0f32, 1.0f32],
							rect.to_untyped(), unknown_to_device_transform);
					}
				},

				&Primitive::PushLayer { ref color } => {
					let (texture2, texture2_view) = device.create_render_target(size.width as u16, size.height as u16)?;
					pushed_render_target = Some((texture2, texture2_view, *color));

					if let Some(ref pushed_render_target) = pushed_render_target {
						device.begin(&pushed_render_target.1);
						device.clear(&pushed_render_target.1, &[0.0f32, 0.0f32, 0.0f32, 0.0f32]);
					}
				},

				&Primitive::PopLayer { .. } => {
					if let Some(ref pushed_render_target) = pushed_render_target {
						device.end(&pushed_render_target.1);
						device.rect_textured(render_target,
							&pushed_render_target.0, false, &pushed_render_target.2,
							Rect::new(Point::new(0.0f32, 0.0f32), Size::new(size.width, size.height)),
							unknown_to_device_transform);
					}
					pushed_render_target = None;
				}
			}
		}

		device.end(render_target);

		Ok(())
	}
}
