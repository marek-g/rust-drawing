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
