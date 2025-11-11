use crate::primitive::*;
use crate::units::*;

pub trait Transformation {
    fn translate(&mut self, offset: PixelPoint);
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
                    start_point,
                    end_point,
                    ..
                } => {
                    start_point.translate(offset);
                    end_point.translate(offset);
                }

                Primitive::Rectangle { rect, .. } => {
                    rect.translate(offset);
                }

                Primitive::Image { rect, .. } => {
                    rect.translate(offset);
                }

                Primitive::Text {
                    position,
                    clipping_rect,
                    ..
                } => {
                    position.translate(offset);
                    clipping_rect.translate(offset);
                }

                Primitive::Stroke { path, .. } => path.translate(offset),

                Primitive::StrokeStyled { path, .. } => path.translate(offset),

                Primitive::Fill { path, .. } => path.translate(offset),

                Primitive::ClipRect {
                    rect,
                    primitives,
                } => {
                    rect.translate(offset);
                    primitives.translate(offset);
                }

                Primitive::ClipPath {
                    path,
                    primitives,
                } => {
                    path.translate(offset);
                    primitives.translate(offset);
                }

                Primitive::Transform {
                    transform, ..
                } => {
                    *transform =
                        transform.then_translate(euclid::Vector2D::new(offset.x, offset.y));
                }

                Primitive::Composite {
                    primitives, ..
                } => primitives.translate(offset),
            }
        }
    }
}

impl Transformation for Vec<PathElement> {
    fn translate(&mut self, offset: PixelPoint) {
        for path_element in self.iter_mut() {
            match path_element {
                PathElement::MoveTo(point) => point.translate(offset),

                PathElement::LineTo(point) => point.translate(offset),

                PathElement::BezierTo(c1, c2, point) => {
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
