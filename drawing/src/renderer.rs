use crate::backend::ColoredVertex;
use crate::backend::Device;
use crate::backend::RenderTarget;
use crate::font::Font;
use crate::font::FontParams;
use crate::paint::Paint;
use crate::primitive::*;
use crate::resources::*;
use crate::scissor::Scissor;
use crate::units::*;
use crate::utils::path::FlattenedPath;
use crate::Result;

use std::convert::Into;

pub struct Renderer;

impl Renderer {
	pub fn new() -> Self {
		Renderer {}
	}

	pub fn draw<D: Device, F: Font<D>>(
		&mut self,
		device: &mut D,
		render_target: &D::RenderTarget,
		primitives: &Vec<Primitive>,
		resources: &mut Resources<D, F>,
		antialiasing: bool,
	) -> Result<()> {
		self.draw_internal(
			device,
			render_target,
			primitives,
			resources,
			antialiasing,
			PixelTransform::identity(),
			Scissor::empty(),
		)
	}

	fn draw_internal<D: Device, F: Font<D>>(
		&mut self,
		device: &mut D,
		render_target: &D::RenderTarget,
		primitives: &Vec<Primitive>,
		resources: &mut Resources<D, F>,
		antialiasing: bool,
		pixel_transform: PixelTransform,
		scissor: Scissor,
	) -> Result<()> {
		let pixel_to_device_transform = render_target
			.get_device_transform()
			.pre_transform(&pixel_transform);
		let unknown_to_device_transform = UnknownToDeviceTransform::from_row_major_array(
			pixel_to_device_transform.to_row_major_array(),
		);

		for primitive in primitives {
			match primitive {
				&Primitive::Line {
					ref color,
					thickness,
					start_point,
					end_point,
				} => {
					let thickness = pixel_to_device_transform
						.transform_point(PixelPoint::new(thickness.get(), thickness.get()))
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
							FontParams {
								size: size.get() as u8,
							},
							unknown_to_device_transform,
						)?;
					}
				}

				&Primitive::Stroke {
					ref path,
					ref thickness,
					ref brush,
				} => {
					let scale = 1.0f32; // TODO: take from transform? xform.average_scale()?
					let stroke_width = *thickness * scale; //.clamped(0.0, 200.0);
					let aspect_ratio = render_target.get_aspect_ratio();
					let flattened_path = Self::get_stroke_path(
						path,
						stroke_width,
						&Default::default(),
						aspect_ratio,
						antialiasing,
					);

					let (paint, texture) = Paint::from_brush(brush, resources);

					let stroke_width = pixel_to_device_transform
						.transform_point(PixelPoint::new(stroke_width.get(), stroke_width.get()))
						.x;

					device.stroke(
						&render_target,
						&paint,
						texture,
						true,
						&flattened_path.paths,
						stroke_width,
						1.0f32 / aspect_ratio,
						antialiasing,
						Scissor {
							xform: PixelTransform::identity(),
							extent: [-1.0, -1.0],
						},
						CompositeOperation::Basic(BasicCompositeOperation::SrcOver).into(),
						unknown_to_device_transform,
					);
				}

				&Primitive::StrokeStyled {
					ref path,
					ref thickness,
					ref brush,
					ref style,
				} => {
					let scale = 1.0f32; // TODO: take from transform? xform.average_scale()?
					let stroke_width = *thickness * scale; //.clamped(0.0, 200.0);
					let aspect_ratio = render_target.get_aspect_ratio();
					let flattened_path = Self::get_stroke_path(
						path,
						stroke_width,
						style,
						aspect_ratio,
						antialiasing,
					);

					let (paint, texture) = Paint::from_brush(brush, resources);

					let stroke_width = pixel_to_device_transform
						.transform_point(PixelPoint::new(stroke_width.get(), stroke_width.get()))
						.x;

					device.stroke(
						&render_target,
						&paint,
						texture,
						true,
						&flattened_path.paths,
						stroke_width,
						1.0f32 / aspect_ratio,
						antialiasing,
						Scissor {
							xform: PixelTransform::identity(),
							extent: [-1.0, -1.0],
						},
						CompositeOperation::Basic(BasicCompositeOperation::SrcOver).into(),
						unknown_to_device_transform,
					);
				}

				&Primitive::Fill {
					ref path,
					ref brush,
				} => {
					let aspect_ratio = render_target.get_aspect_ratio();
					let flattened_path = Self::get_fill_path(path, aspect_ratio, antialiasing);

					let (paint, texture) = Paint::from_brush(brush, resources);

					device.fill(
						&render_target,
						&paint,
						texture,
						true,
						&flattened_path.paths,
						flattened_path.bounds,
						1.0f32 / aspect_ratio,
						antialiasing,
						Scissor {
							xform: PixelTransform::identity(),
							extent: [-1.0, -1.0],
						},
						CompositeOperation::Basic(BasicCompositeOperation::SrcOver).into(),
						unknown_to_device_transform,
					);
				}

				&Primitive::ClipRect {
					ref rect,
					ref primitives,
				} => {
					self.draw_internal(
						device,
						render_target,
						primitives,
						resources,
						antialiasing,
						pixel_transform,
						scissor.intersect_with_rect(rect.clone(), &pixel_transform),
					)?;
				}

				&Primitive::ClipPath {
					ref path,
					ref primitives,
				} => {
					/*device.save_state();

					device.set_clip_path(path);
					self.draw(device, render_target, primitives, resources, antialiasing)?;

					device.restore_state();*/
				}

				&Primitive::Transform {
					ref transform,
					ref primitives,
				} => {
					self.draw_internal(
						device,
						render_target,
						primitives,
						resources,
						antialiasing,
						pixel_transform.pre_transform(transform),
						scissor.apply_transform(transform),
					)?;
				}

				&Primitive::Composite {
					ref color,
					ref primitives,
				} => {
					let size = render_target.get_size();
					let (texture2, texture2_view) =
						device.create_render_target(size.0 as u16, size.1 as u16)?;

					device.clear(&texture2_view, &[0.0f32, 0.0f32, 0.0f32, 0.0f32]);

					self.draw(device, &texture2_view, &primitives, resources, antialiasing)?;

					device.rect_textured(
						render_target,
						&texture2,
						false,
						&color,
						Rect::new(
							Point::new(0.0f32, 0.0f32),
							Size::new(size.0 as f32, size.1 as f32),
						),
						&[0.0, 0.0, 1.0, 1.0],
						unknown_to_device_transform,
					);
				}
			}
		}

		Ok(())
	}

	fn get_stroke_path(
		path: &Vec<PathElement>,
		stroke_width: PixelThickness,
		stroke_style: &StrokeStyle,
		aspect_ratio: f32,
		antialiasing: bool,
	) -> FlattenedPath {
		let mut flattened_path =
			FlattenedPath::new(path, 0.01f32 / aspect_ratio, 0.25f32 / aspect_ratio);
		let fringe_width = 1.0f32 / aspect_ratio;
		if antialiasing {
			flattened_path.expand_stroke(
				stroke_width.get() * 0.5f32,
				fringe_width,
				stroke_style.line_cap,
				stroke_style.line_join,
				stroke_style.miter_limit,
				0.25f32 / aspect_ratio,
			);
		} else {
			flattened_path.expand_stroke(
				stroke_width.get() * 0.5f32,
				0.0,
				stroke_style.line_cap,
				stroke_style.line_join,
				stroke_style.miter_limit,
				0.25f32 / aspect_ratio,
			);
		}
		flattened_path
	}

	fn get_fill_path(
		path: &Vec<PathElement>,
		aspect_ratio: f32,
		antialiasing: bool,
	) -> FlattenedPath {
		let mut flattened_path =
			FlattenedPath::new(path, 0.01f32 / aspect_ratio, 0.25f32 / aspect_ratio);
		let fringe_width = 1.0f32 / aspect_ratio;
		if antialiasing {
			flattened_path.expand_fill(fringe_width, LineJoin::Miter, 2.4f32, fringe_width);
		} else {
			flattened_path.expand_fill(0.0f32, LineJoin::Miter, 2.4f32, fringe_width);
		}
		flattened_path
	}
}
