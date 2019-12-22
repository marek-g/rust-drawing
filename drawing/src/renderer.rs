use crate::backend::Device;
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
		primitives: &Vec<Primitive>,
		resources: &mut Resources<D, F>,
	) -> Result<()> {
		let physical_pixel_to_device_transform = D::get_device_transform(size);
		let user_pixel_to_physical_pixel_transform = UserPixelToPhysPixelTransform::identity();
		let user_pixel_to_device_transform = user_pixel_to_physical_pixel_transform
			.post_transform(&physical_pixel_to_device_transform);
		let unknown_to_device_transform = UnknownToDeviceTransform::from_row_major_array(
			user_pixel_to_device_transform.to_row_major_array(),
		);

		for primitive in primitives {
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

					device.line(
						&render_target,
						color,
						DeviceThickness::new(thickness),
						start_point.to_untyped(),
						end_point.to_untyped(),
						unknown_to_device_transform,
					);
				}

				&Primitive::Rectangle { ref color, rect } => device.rect_colored(
					&render_target,
					color,
					rect.to_untyped(),
					unknown_to_device_transform,
				),

				&Primitive::Image {
					resource_key,
					rect,
					uv,
				} => {
					if let Some(texture) = resources.textures_mut().get(&resource_key) {
						device.rect_textured(
							&render_target,
							&texture,
							false,
							&[1.0f32, 1.0f32, 1.0f32, 1.0f32],
							rect.to_untyped(),
							&uv,
							unknown_to_device_transform,
						);
					}
				}

				&Primitive::Text {
					ref resource_key,
					ref color,
					position,
					clipping_rect,
					size,
					ref text,
				} => {
					if let Some(font) = resources.fonts_mut().get_mut(&resource_key.to_string()) {
						font.draw(
							device,
							&render_target,
							color,
							text,
							position.to_untyped(),
							clipping_rect.to_untyped(),
							FontParams { size: size as u8 },
							unknown_to_device_transform,
						)?;
					}
				}

				&Primitive::Stroke {
					ref path,
					ref thickness,
					ref brush,
				} => device.stroke(path, *thickness, brush),

				&Primitive::StrokeStyled {
					ref path,
					ref thickness,
					ref brush,
					ref style,
				} => device.stroke_styled(path, *thickness, brush, style),

				&Primitive::Fill {
					ref path,
					ref brush,
				} => device.fill(path, brush),

				&Primitive::ClipRect {
					ref rect,
					ref primitives,
				} => {
					device.save_state();

					device.set_clip_rect(*rect);
					self.draw(device, render_target, size, primitives, resources)?;

					device.restore_state();
				}

				&Primitive::ClipPath {
					ref path,
					ref primitives,
				} => {
					device.save_state();

					device.set_clip_path(path);
					self.draw(device, render_target, size, primitives, resources)?;

					device.restore_state();
				}

				&Primitive::Transform {
					ref transform,
					ref primitives,
				} => {
					device.save_state();

					device.transform(*transform);
					self.draw(device, render_target, size, primitives, resources)?;

					device.restore_state();
				}

				&Primitive::Composite {
					ref color,
					ref primitives,
				} => {
					let (texture2, texture2_view) =
						device.create_render_target(size.width as u16, size.height as u16)?;

					device.clear(&texture2_view, &[0.0f32, 0.0f32, 0.0f32, 0.0f32]);

					self.draw(device, &texture2_view, size, &primitives, resources)?;

					device.rect_textured(
						render_target,
						&texture2,
						false,
						&color,
						Rect::new(
							Point::new(0.0f32, 0.0f32),
							Size::new(size.width, size.height),
						),
						&[0.0, 0.0, 1.0, 1.0],
						unknown_to_device_transform,
					);
				}
			}
		}

		Ok(())
	}
}
