use drawing_api::{Either, PixelPoint, PixelRect, PixelSize, PixelUnit, TextureSampling};
use euclid::Angle;

use crate::{generic::device::convert_color, units::PixelTransform, GlContext, GlTexture};

use super::{ImageFilterFragment, PathElement, Primitive};

enum StackElement {
    Start,
    RestorePoint,
    Transform {
        transform: PixelTransform,
    },
    ClipRect {
        rect: PixelRect,
    },
    ClipPath {
        path: Vec<PathElement>,
    },
    Layer {
        bounds: PixelRect,
        paint: Option<crate::Paint>,
        filter: Option<drawing_api::ImageFilter<ImageFilterFragment>>,
    },
}

pub struct DisplayListBuilder {
    display_list_stack: Vec<(
        StackElement,
        Vec<Primitive<GlTexture, crate::Fonts<GlContext>>>,
    )>,
}

impl DisplayListBuilder {
    fn paint_to_brush(paint: &crate::Paint) -> super::Brush<GlTexture> {
        if let Some(color_source) = &paint.color_source {
            match color_source {
                drawing_api::ColorSource::LinearGradient {
                    start,
                    end,
                    colors,
                    stops,
                    tile_mode,
                    transformation,
                } => super::Brush::LinearGradient {
                    start_point: PixelPoint::new(start.x, start.y),
                    end_point: PixelPoint::new(end.x, end.y),
                    inner_color: convert_color(&colors[0]),
                    outer_color: convert_color(&colors[1]),
                },

                drawing_api::ColorSource::Image {
                    image,
                    horizontal_tile_mode,
                    qvertical_tile_mode,
                    sampling,
                    transformation,
                } => super::Brush::ImagePattern {
                    texture: image.clone(),
                    transform: transformation
                        .map(|t| PixelTransform::from_untyped(&t.to_2d()))
                        .unwrap_or_else(|| PixelTransform::identity()),
                    alpha: 1.0f32,
                },

                _ => super::Brush::Color { color: paint.color },
            }
        } else {
            super::Brush::Color { color: paint.color }
        }
    }
}

impl drawing_api::DisplayListBuilder for DisplayListBuilder {
    type DisplayList = Vec<Primitive<GlTexture, crate::Fonts<GlContext>>>;
    type ImageFilterFragment = crate::display_list::ImageFilterFragment;
    type Paint = crate::Paint;
    type ParagraphBuilder = crate::display_list::ParagraphBuilder;
    type PathBuilder = crate::PathBuilder;
    type Texture = crate::GlTexture;

    fn new(bounds: impl Into<Option<PixelRect>>) -> Self {
        Self {
            display_list_stack: vec![(StackElement::Start, Vec::new())],
        }
    }

    fn scale(&mut self, x_scale: f32, y_scale: f32) {
        self.display_list_stack.push((
            StackElement::Transform {
                transform: PixelTransform::identity().then_scale(x_scale, y_scale),
            },
            Vec::new(),
        ));
    }

    fn rotate(&mut self, angle_degrees: f32) {
        self.display_list_stack.push((
            StackElement::Transform {
                transform: PixelTransform::identity().then_rotate(Angle::degrees(angle_degrees)),
            },
            Vec::new(),
        ));
    }

    fn translate(&mut self, x_translation: f32, y_translation: f32) {
        self.display_list_stack.push((
            StackElement::Transform {
                transform: PixelTransform::identity().then_translate(euclid::Vector2D::<
                    f32,
                    PixelUnit,
                >::new(
                    x_translation,
                    y_translation,
                )),
            },
            Vec::new(),
        ));
    }

    fn transform(&mut self, transform: &drawing_api::Matrix) {
        todo!()
    }

    fn set_transform(&mut self, transform: &drawing_api::Matrix) {
        todo!()
    }

    fn get_transform(&self) -> drawing_api::Matrix {
        todo!()
    }

    fn reset_transform(&mut self) {
        todo!()
    }

    fn clip_rect(&mut self, rect: impl Into<PixelRect>, operation: drawing_api::ClipOperation) {
        let rect = rect.into();
        self.display_list_stack.push((
            StackElement::ClipRect {
                rect: PixelRect::new(
                    PixelPoint::new(rect.origin.x, rect.origin.y),
                    PixelSize::new(rect.size.width, rect.size.height),
                ),
            },
            Vec::new(),
        ));
    }

    fn clip_oval(
        &mut self,
        oval_bounds: impl Into<PixelRect>,
        operation: drawing_api::ClipOperation,
    ) {
        //todo!()
    }

    fn clip_rounded_rect(
        &mut self,
        rect: impl Into<PixelRect>,
        radii: &drawing_api::RoundingRadii,
        operation: drawing_api::ClipOperation,
    ) {
        //todo!()
    }

    fn clip_path(
        &mut self,
        path: &<Self::PathBuilder as drawing_api::PathBuilder>::Path,
        operation: drawing_api::ClipOperation,
    ) {
        self.display_list_stack.push((
            StackElement::ClipPath {
                path: path.path.to_vec(),
            },
            Vec::new(),
        ));
    }

    fn save(&mut self) {
        self.display_list_stack
            .push((StackElement::RestorePoint, Vec::new()));
    }

    fn save_layer(
        &mut self,
        bounds: impl Into<PixelRect>,
        paint: Option<&Self::Paint>,
        filter: Option<drawing_api::ImageFilter<ImageFilterFragment>>,
    ) {
        self.display_list_stack
            .push((StackElement::RestorePoint, Vec::new()));
        self.display_list_stack.push((
            StackElement::Layer {
                bounds: bounds.into(),
                paint: paint.cloned(),
                filter,
            },
            Vec::new(),
        ));
    }

    fn get_save_count(&mut self) -> usize {
        self.display_list_stack.len()
    }

    fn restore(&mut self) {
        loop {
            if self.display_list_stack.len() <= 1 {
                return;
            }

            let mut stack_el = self.display_list_stack.pop().unwrap();

            match stack_el.0 {
                StackElement::Start => unreachable!(),

                StackElement::RestorePoint => {
                    self.display_list_stack
                        .last_mut()
                        .unwrap()
                        .1
                        .append(&mut stack_el.1);
                    return;
                }

                StackElement::Transform { transform } => {
                    self.display_list_stack
                        .last_mut()
                        .unwrap()
                        .1
                        .push(Primitive::Transform {
                            transform,
                            primitives: stack_el.1,
                        });
                }

                StackElement::ClipRect { rect } => {
                    self.display_list_stack
                        .last_mut()
                        .unwrap()
                        .1
                        .push(Primitive::ClipRect {
                            rect,
                            primitives: stack_el.1,
                        });
                }

                StackElement::ClipPath { path } => {
                    self.display_list_stack
                        .last_mut()
                        .unwrap()
                        .1
                        .push(Primitive::ClipPath {
                            path,
                            primitives: stack_el.1,
                        });
                }

                StackElement::Layer {
                    bounds,
                    paint,
                    filter,
                } => {
                    self.display_list_stack
                        .last_mut()
                        .unwrap()
                        .1
                        .push(Primitive::Composite {
                            color: paint
                                .map(|p| p.color)
                                .unwrap_or([0.0f32, 0.0f32, 0.0f32, 1.0f32]),
                            primitives: stack_el.1,
                        });
                }
            }
        }
    }

    fn draw_paint<'a>(&mut self, paint: impl Into<Either<&'a Self::Paint, Self::Paint>>) {
        let paint = paint.into();
        let paint_ref = match &paint {
            Either::Left(paint) => *paint,
            Either::Right(paint) => paint,
        };

        // TODO: handle other cases
        self.display_list_stack
            .last_mut()
            .unwrap()
            .1
            .push(Primitive::Clear {
                color: paint_ref.color,
            });
    }

    fn draw_line(
        &mut self,
        from: impl Into<PixelPoint>,
        to: impl Into<PixelPoint>,
        paint: &Self::Paint,
    ) {
        let from = from.into();
        let to = to.into();
        self.display_list_stack
            .last_mut()
            .unwrap()
            .1
            .push(Primitive::Line {
                color: paint.color,
                thickness: 1.0f32,
                // TODO: convert
                start_point: PixelPoint::new(from.x, from.y),
                end_point: PixelPoint::new(to.x, to.y),
            });
    }

    fn draw_dashed_line(
        &mut self,
        from: impl Into<PixelPoint>,
        to: impl Into<PixelPoint>,
        on_length: f32,
        off_length: f32,
        paint: &Self::Paint,
    ) {
        todo!()
    }

    fn draw_rect(&mut self, rect: impl Into<PixelRect>, paint: &Self::Paint) {
        let rect = rect.into();
        self.display_list_stack
            .last_mut()
            .unwrap()
            .1
            .push(Primitive::Rectangle {
                color: paint.color,
                rect: PixelRect::new(
                    PixelPoint::new(rect.origin.x, rect.origin.y),
                    PixelSize::new(rect.size.width, rect.size.height),
                ),
            });
    }

    fn draw_rounded_rect(
        &mut self,
        rect: impl Into<PixelRect>,
        radii: &drawing_api::RoundingRadii,
        paint: &Self::Paint,
    ) {
        todo!()
    }

    fn draw_rounded_rect_difference(
        &mut self,
        outer_rect: impl Into<PixelRect>,
        outer_radii: &drawing_api::RoundingRadii,
        inner_rect: impl Into<PixelRect>,
        inner_radii: &drawing_api::RoundingRadii,
        paint: &Self::Paint,
    ) {
        todo!()
    }

    fn draw_oval(&mut self, oval_bounds: impl Into<PixelRect>, paint: &Self::Paint) {
        todo!()
    }

    fn draw_path(
        &mut self,
        path: &<Self::PathBuilder as drawing_api::PathBuilder>::Path,
        paint: &Self::Paint,
    ) {
        match paint.draw_style {
            drawing_api::DrawStyle::Fill => {
                self.display_list_stack
                    .last_mut()
                    .unwrap()
                    .1
                    .push(Primitive::Fill {
                        path: path.path.to_vec(),
                        brush: DisplayListBuilder::paint_to_brush(paint),
                    })
            }
            drawing_api::DrawStyle::Stroke => {
                self.display_list_stack
                    .last_mut()
                    .unwrap()
                    .1
                    .push(Primitive::Stroke {
                        path: path.path.to_vec(),
                        thickness: paint.stroke_width.max(1.0f32),
                        brush: DisplayListBuilder::paint_to_brush(paint),
                    })
            }
            drawing_api::DrawStyle::StrokeAndFill => {
                self.display_list_stack
                    .last_mut()
                    .unwrap()
                    .1
                    .push(Primitive::Fill {
                        path: path.path.to_vec(),
                        brush: DisplayListBuilder::paint_to_brush(paint),
                    });
                self.display_list_stack
                    .last_mut()
                    .unwrap()
                    .1
                    .push(Primitive::Stroke {
                        path: path.path.to_vec(),
                        thickness: paint.stroke_width.max(1.0f32),
                        brush: DisplayListBuilder::paint_to_brush(paint),
                    })
            }
        }
    }

    fn draw_shadow(
        &mut self,
        path: &<Self::PathBuilder as drawing_api::PathBuilder>::Path,
        color: &drawing_api::Color,
        elevation: f32,
        oocluder_is_transparent: bool,
        device_pixel_ratio: f32,
    ) {
        todo!()
    }

    fn draw_texture(
        &mut self,
        texture: &Self::Texture,
        point: impl Into<PixelPoint>,
        sampling: TextureSampling,
        paint: Option<&Self::Paint>,
    ) {
        todo!()
    }

    fn draw_texture_rect(
        &mut self,
        texture: &Self::Texture,
        src_rect: impl Into<PixelRect>,
        dst_rect: impl Into<PixelRect>,
        sampling: TextureSampling,
        paint: Option<&Self::Paint>,
    ) {
        let src_rect = src_rect.into();
        let dst_rect = dst_rect.into();
        self.display_list_stack
            .last_mut()
            .unwrap()
            .1
            .push(Primitive::Image {
                texture: texture.clone(),
                rect: dst_rect,
                src: src_rect,
            });
    }

    fn draw_paragraph(
        &mut self,
        location: impl Into<PixelPoint>,
        paragraph: &<Self::ParagraphBuilder as drawing_api::ParagraphBuilder>::Paragraph,
    ) {
        let location = location.into();
        let location = PixelPoint::new(location.x, location.y);
        for el in &paragraph.primitives {
            match el {
                Primitive::Text {
                    fonts,
                    family_name,
                    size,
                    color,
                    position,
                    clipping_rect,
                    text,
                } => {
                    let position =
                        PixelPoint::new(location.x + position.x, location.y + position.y);
                    self.display_list_stack
                        .last_mut()
                        .unwrap()
                        .1
                        .push(Primitive::Text {
                            fonts: fonts.clone(),
                            family_name: family_name.clone(),
                            size: size.clone(),
                            color: color.clone(),
                            position,
                            clipping_rect: clipping_rect.clone(),
                            text: text.clone(),
                        });
                }

                _ => (),
            }
        }
    }

    fn draw_display_list(&mut self, display_list: &Self::DisplayList, opacity: f32) {
        todo!()
    }

    fn build(mut self) -> Result<Self::DisplayList, &'static str> {
        Ok(self.display_list_stack.pop().unwrap().1)
    }
}
