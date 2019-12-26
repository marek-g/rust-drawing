use crate::primitive::*;
use crate::units::*;
use crate::utils::clipping::clip_image;
use crate::utils::clipping::clip_line;
use crate::utils::clipping::clip_rect;

pub trait Transformation {
    fn translate(&mut self, offset: PixelPoint);
}

pub trait Clipping {
    fn clip(self, rect: PixelRect) -> Self;
}

impl Transformation for PixelPoint {
    fn translate(&mut self, offset: PixelPoint) {
        self.x += offset.x;
        self.y += offset.y;
    }
}

impl Transformation for PixelRect {
    fn translate(&mut self, offset: PixelPoint) {
        self.origin.translate(offset);
    }
}

impl Transformation for Vec<Primitive> {
    fn translate(&mut self, offset: PixelPoint) {
        for primitive in self.iter_mut() {
            match primitive {
                Primitive::Line {
                    ref mut start_point,
                    ref mut end_point,
                    ..
                } => {
                    start_point.translate(offset);
                    end_point.translate(offset);
                }

                Primitive::Rectangle { ref mut rect, .. } => {
                    rect.translate(offset);
                }

                Primitive::Image { ref mut rect, .. } => {
                    rect.translate(offset);
                }

                Primitive::Text {
                    ref mut position,
                    ref mut clipping_rect,
                    ..
                } => {
                    position.translate(offset);
                    clipping_rect.translate(offset);
                }

                Primitive::Stroke { ref mut path, .. } => path.translate(offset),

                Primitive::StrokeStyled { ref mut path, .. } => path.translate(offset),

                Primitive::Fill { ref mut path, .. } => path.translate(offset),

                Primitive::ClipRect {
                    ref mut rect,
                    ref mut primitives,
                } => {
                    rect.translate(offset);
                    primitives.translate(offset);
                }

                Primitive::ClipPath {
                    ref mut path,
                    ref mut primitives,
                } => {
                    path.translate(offset);
                    primitives.translate(offset);
                }

                Primitive::Transform {
                    ref mut transform, ..
                } => {
                    *transform =
                        transform.post_translate(euclid::Vector2D::new(offset.x, offset.y));
                }

                Primitive::Composite {
                    ref mut primitives, ..
                } => primitives.translate(offset),
            }
        }
    }
}

impl Clipping for Vec<Primitive> {
    fn clip(self, clipping_rect: PixelRect) -> Self {
        let mut res = Vec::new();
        for primitive in self.into_iter() {
            match primitive {
                Primitive::Line {
                    color,
                    thickness,
                    start_point,
                    end_point,
                } => {
                    if let Some(clipped) = clip_line(
                        start_point.x,
                        start_point.y,
                        end_point.x,
                        end_point.y,
                        clipping_rect.origin.x,
                        clipping_rect.origin.y,
                        clipping_rect.size.width,
                        clipping_rect.size.height,
                    ) {
                        res.push(Primitive::Line {
                            color,
                            thickness,
                            start_point: PixelPoint::new(clipped.0, clipped.1),
                            end_point: PixelPoint::new(clipped.2, clipped.3),
                        });
                    }
                }

                Primitive::Rectangle { color, rect } => {
                    if let Some(clipped) = clip_rect(
                        rect.origin.x,
                        rect.origin.y,
                        rect.size.width,
                        rect.size.height,
                        clipping_rect.origin.x,
                        clipping_rect.origin.y,
                        clipping_rect.size.width,
                        clipping_rect.size.height,
                    ) {
                        res.push(Primitive::Rectangle {
                            color,
                            rect: PixelRect::new(
                                PixelPoint::new(clipped.0, clipped.1),
                                PixelSize::new(clipped.2, clipped.3),
                            ),
                        });
                    }
                }

                Primitive::Image {
                    resource_key,
                    rect,
                    uv,
                } => {
                    if let Some(clipped) = clip_image(
                        rect.origin.x,
                        rect.origin.y,
                        rect.size.width,
                        rect.size.height,
                        clipping_rect.origin.x,
                        clipping_rect.origin.y,
                        clipping_rect.size.width,
                        clipping_rect.size.height,
                        &uv,
                    ) {
                        res.push(Primitive::Image {
                            resource_key,
                            rect: PixelRect::new(
                                PixelPoint::new(clipped.0, clipped.1),
                                PixelSize::new(clipped.2, clipped.3),
                            ),
                            uv: clipped.4,
                        });
                    }
                }

                Primitive::Text {
                    resource_key,
                    size,
                    color,
                    position,
                    clipping_rect: rect,
                    text,
                } => {
                    if let Some(clipped) = clip_rect(
                        rect.origin.x,
                        rect.origin.y,
                        rect.size.width,
                        rect.size.height,
                        clipping_rect.origin.x,
                        clipping_rect.origin.y,
                        clipping_rect.size.width,
                        clipping_rect.size.height,
                    ) {
                        res.push(Primitive::Text {
                            resource_key,
                            size,
                            color,
                            position,
                            clipping_rect: PixelRect::new(
                                PixelPoint::new(clipped.0, clipped.1),
                                PixelSize::new(clipped.2, clipped.3),
                            ),
                            text,
                        });
                    }
                }

                Primitive::Stroke {
                    path,
                    thickness,
                    brush,
                } => {
                    let clipped_path = path.clip(clipping_rect);
                    if clipped_path.len() > 0 {
                        res.push(Primitive::Stroke {
                            path: clipped_path,
                            thickness,
                            brush,
                        })
                    }
                }

                Primitive::StrokeStyled {
                    path,
                    thickness,
                    brush,
                    style,
                } => {
                    let clipped_path = path.clip(clipping_rect);
                    if clipped_path.len() > 0 {
                        res.push(Primitive::StrokeStyled {
                            path: clipped_path,
                            thickness,
                            brush,
                            style,
                        })
                    }
                }

                Primitive::Fill { path, brush } => {
                    let clipped_path = path.clip(clipping_rect);
                    if clipped_path.len() > 0 {
                        res.push(Primitive::Fill {
                            path: clipped_path,
                            brush,
                        })
                    }
                }

                Primitive::ClipRect { .. } => {
                    // TODO: implement!
                }

                Primitive::ClipPath { .. } => {
                    // TODO: implement!
                }

                Primitive::Transform { .. } => {
                    // TODO: implement!
                }

                Primitive::Composite { color, primitives } => {
                    let clipped_primitives = primitives.clip(clipping_rect);
                    if clipped_primitives.len() > 0 {
                        res.push(Primitive::Composite {
                            color,
                            primitives: clipped_primitives,
                        })
                    }
                }
            }
        }
        res
    }
}

impl Transformation for Vec<PathElement> {
    fn translate(&mut self, offset: PixelPoint) {
        for path_element in self.iter_mut() {
            match path_element {
                PathElement::MoveTo(ref mut point) => point.translate(offset),

                PathElement::LineTo(ref mut point) => point.translate(offset),

                PathElement::BezierTo(ref mut c1, ref mut c2, ref mut point) => {
                    c1.translate(offset);
                    c2.translate(offset);
                    point.translate(offset);
                }

                PathElement::ClosePath => (),

                PathElement::Solidity(..) => (),
            }
        }
    }
}

impl Clipping for Vec<PathElement> {
    fn clip(self, clipping_rect: PixelRect) -> Self {
        let mut res = Vec::new();
        for path_element in self.into_iter() {
            // TODO: implement!
            res.push(path_element);
        }
        res
    }
}
