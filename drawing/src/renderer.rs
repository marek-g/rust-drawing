use crate::backend::Device;
use crate::color::Color;
use crate::font::Font;
use crate::font::FontParams;
use crate::primitive::Primitive;
use crate::resources::*;
use crate::units::*;
use crate::Result;

pub struct Renderer;

impl Renderer {
	pub fn new() -> Self {
		Renderer {}
	}

	pub fn draw<D: Device, F: Font<D>>(
		&mut self,
		device: &mut D,
		render_target: &D::RenderTarget,
		size: PhysPixelSize,
		primitives: Vec<Primitive>,
		resources: &mut Resources<D, F>,
	) -> Result<()> {
		let physical_pixel_to_device_transform = D::get_device_transform(size);
		let user_pixel_to_physical_pixel_transform = UserPixelToPhysPixelTransform::identity();
		let user_pixel_to_device_transform = user_pixel_to_physical_pixel_transform
			.post_transform(&physical_pixel_to_device_transform);
		let unknown_to_device_transform = UnknownToDeviceTransform::from_row_major_array(
			user_pixel_to_device_transform.to_row_major_array(),
		);

		let mut pushed_render_target: Option<(D::Texture, D::RenderTarget, Color)> = None;

		for primitive in &primitives {
			match primitive {
				&Primitive::Line {
					ref color,
					thickness,
					start_point,
					end_point,
				} => {
					let thickness = user_pixel_to_device_transform
						.transform_point(UserPixelPoint::new(thickness.get(), thickness.get()))
						.x;

					let target_view = if let Some(ref pushed_render_target) = pushed_render_target {
						&pushed_render_target.1
					} else {
						render_target
					};

					device.line(
						&target_view,
						color,
						DeviceThickness::new(thickness),
						start_point.to_untyped(),
						end_point.to_untyped(),
						unknown_to_device_transform,
					);
				}

				&Primitive::Rectangle { ref color, rect } => {
					let target_view = if let Some(ref pushed_render_target) = pushed_render_target {
						&pushed_render_target.1
					} else {
						render_target
					};

					device.rect_colored(
						&target_view,
						color,
						rect.to_untyped(),
						unknown_to_device_transform,
					)
				}

				&Primitive::Text {
					ref resource_key,
					ref color,
					position,
					clipping_rect,
					size,
					ref text,
				} => {
					let target_view = if let Some(ref pushed_render_target) = pushed_render_target {
						&pushed_render_target.1
					} else {
						render_target
					};

					if let Some(font) = resources.fonts_mut().get_mut(&resource_key.to_string()) {
						font.draw(
							device,
							&target_view,
							color,
							text,
							position.to_untyped(),
							clipping_rect.to_untyped(),
							FontParams { size: size as u8 },
							unknown_to_device_transform,
						)?;
					}
				}

				&Primitive::Image {
					resource_key,
					rect,
					uv,
				} => {
					let target_view = if let Some(ref pushed_render_target) = pushed_render_target {
						&pushed_render_target.1
					} else {
						render_target
					};

					if let Some(texture) = resources.textures_mut().get(&resource_key) {
						device.rect_textured(
							&target_view,
							&texture,
							false,
							&[1.0f32, 1.0f32, 1.0f32, 1.0f32],
							rect.to_untyped(),
							&uv,
							unknown_to_device_transform,
						);
					}
				}

				&Primitive::PushLayer { ref color } => {
					let (texture2, texture2_view) =
						device.create_render_target(size.width as u16, size.height as u16)?;
					pushed_render_target = Some((texture2, texture2_view, *color));

					if let Some(ref pushed_render_target) = pushed_render_target {
						device.clear(&pushed_render_target.1, &[0.0f32, 0.0f32, 0.0f32, 0.0f32]);
					}
				}

				&Primitive::PopLayer { .. } => {
					if let Some(ref pushed_render_target) = pushed_render_target {
						device.rect_textured(
							render_target,
							&pushed_render_target.0,
							false,
							&pushed_render_target.2,
							Rect::new(
								Point::new(0.0f32, 0.0f32),
								Size::new(size.width, size.height),
							),
							&[0.0, 0.0, 1.0, 1.0],
							unknown_to_device_transform,
						);
					}
					pushed_render_target = None;
				}
			}
		}

		Ok(())
	}
}
