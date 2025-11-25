use drawing_api::{
    DeviceRect, DipPoint, DipRect, FillType, PixelLength, PixelPoint, PixelRect, PixelSize,
    PixelTransform, TextureSampling,
};
use euclid::rect;

use crate::{generic::device::convert_color, GlContextData, GlTexture};

use super::{PathElement, Primitive};

struct DisplayListLayer {
    bounds: DipRect,
    paint: Option<crate::Paint>,
    filter: Option<drawing_api::ImageFilter>,
    display_list: Vec<Primitive<GlTexture, crate::Fonts<GlContextData>>>,
}

impl DisplayListLayer {
    pub fn new(
        bounds: impl Into<DipRect>,
        paint: Option<&crate::Paint>,
        filter: Option<drawing_api::ImageFilter>,
    ) -> Self {
        DisplayListLayer {
            bounds: bounds.into(),
            paint: paint.cloned(),
            filter,
            display_list: Vec::new(),
        }
    }
}

pub struct DisplayListBuilder {
    display_list: Vec<DisplayListLayer>,
}

impl DisplayListBuilder {
    pub fn new() -> Self {
        Self {
            display_list: vec![DisplayListLayer::new(DipRect::zero(), None, None)],
        }
    }

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
                    vertical_tile_mode,
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

impl Default for DisplayListBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl drawing_api::DisplayListBuilder for DisplayListBuilder {
    type DisplayList = Vec<Primitive<GlTexture, crate::Fonts<GlContextData>>>;
    type Paint = crate::Paint;
    type Paragraph = Vec<Primitive<GlTexture, crate::Fonts<GlContextData>>>;
    type Path = (Vec<PathElement>, FillType);
    type Texture = crate::GlTexture;

    fn save_layer(
        &mut self,
        bounds: impl Into<DipRect>,
        paint: Option<&Self::Paint>,
        filter: Option<drawing_api::ImageFilter>,
    ) {
        self.display_list
            .push(DisplayListLayer::new(bounds, paint, filter));
    }

    fn restore(&mut self) {
        let layer = self.display_list.pop().unwrap();
        self.display_list
            .last_mut()
            .unwrap()
            .display_list
            .push(Primitive::Composite {
                color: layer
                    .paint
                    .map(|p| p.color)
                    .unwrap_or([0.0f32, 0.0f32, 0.0f32, 1.0f32]),
                primitives: layer.display_list,
            });
    }

    fn draw_paint(&mut self, paint: &Self::Paint) {
        // TODO: handle other cases
        self.display_list
            .last_mut()
            .unwrap()
            .display_list
            .push(Primitive::Clear { color: paint.color });
    }

    fn draw_line(
        &mut self,
        from: impl Into<DipPoint>,
        to: impl Into<DipPoint>,
        paint: &Self::Paint,
    ) {
        let from = from.into();
        let to = to.into();
        self.display_list
            .last_mut()
            .unwrap()
            .display_list
            .push(Primitive::Line {
                color: paint.color,
                thickness: PixelLength::new(1.0f32),
                // TODO: convert
                start_point: PixelPoint::new(from.x, from.y),
                end_point: PixelPoint::new(to.x, to.y),
            });
    }

    fn draw_rect(&mut self, rect: impl Into<DipRect>, paint: &Self::Paint) {
        let rect = rect.into();
        self.display_list
            .last_mut()
            .unwrap()
            .display_list
            .push(Primitive::Rectangle {
                color: paint.color,
                rect: PixelRect::new(
                    PixelPoint::new(rect.origin.x, rect.origin.y),
                    PixelSize::new(rect.size.width, rect.size.height),
                ),
            });
    }

    fn draw_path(&mut self, path: &Self::Path, paint: &Self::Paint) {
        match paint.draw_style {
            drawing_api::DrawStyle::Fill => self
                .display_list
                .last_mut()
                .unwrap()
                .display_list
                .push(Primitive::Fill {
                    path: path.0.to_vec(),
                    brush: DisplayListBuilder::paint_to_brush(paint),
                }),
            drawing_api::DrawStyle::Stroke => self
                .display_list
                .last_mut()
                .unwrap()
                .display_list
                .push(Primitive::Stroke {
                    path: path.0.to_vec(),
                    thickness: PixelLength::new(paint.stroke_width.max(1.0f32)),
                    brush: DisplayListBuilder::paint_to_brush(paint),
                }),
            drawing_api::DrawStyle::StrokeAndFill => {
                self.display_list
                    .last_mut()
                    .unwrap()
                    .display_list
                    .push(Primitive::Fill {
                        path: path.0.to_vec(),
                        brush: DisplayListBuilder::paint_to_brush(paint),
                    });
                self.display_list
                    .last_mut()
                    .unwrap()
                    .display_list
                    .push(Primitive::Stroke {
                        path: path.0.to_vec(),
                        thickness: PixelLength::new(paint.stroke_width.max(1.0f32)),
                        brush: DisplayListBuilder::paint_to_brush(paint),
                    })
            }
        }
    }

    fn draw_texture_rect(
        &mut self,
        texture: &Self::Texture,
        src_rect: impl Into<DeviceRect>,
        dst_rect: impl Into<DipRect>,
        sampling: TextureSampling,
        paint: Option<&Self::Paint>,
    ) {
        let src_rect = src_rect.into();
        let dst_rect = dst_rect.into();
        self.display_list
            .last_mut()
            .unwrap()
            .display_list
            .push(Primitive::Image {
                texture: texture.clone(),
                rect: rect(
                    dst_rect.origin.x,
                    dst_rect.origin.y,
                    dst_rect.size.width,
                    dst_rect.size.height,
                ),
                uv: [
                    src_rect.origin.x,
                    src_rect.origin.y,
                    src_rect.origin.x + src_rect.size.width,
                    src_rect.origin.y + src_rect.size.height,
                ],
            });
    }

    fn draw_paragraph(&mut self, location: impl Into<DipPoint>, paragraph: &Self::Paragraph) {
        let location = location.into();
        let location = PixelPoint::new(location.x, location.y);
        for el in paragraph {
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
                    self.display_list
                        .last_mut()
                        .unwrap()
                        .display_list
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

    fn build(mut self) -> Result<Self::DisplayList, &'static str> {
        Ok(self.display_list.pop().unwrap().display_list)
    }
}
