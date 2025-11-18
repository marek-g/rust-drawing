use crate::{generic::renderer::*, PathElement, Primitive};
use drawing_api::*;
use euclid::Point2D;

pub trait Transformation<T>
where
    T: Texture,
{
    fn translate(&mut self, offset: PixelPoint);
}

impl<T: Texture> Transformation<T> for PixelPoint {
    fn translate(&mut self, offset: PixelPoint) {
        self.x += offset.x;
        self.y += offset.y;
    }
}

impl<T: Texture> Transformation<T> for PixelRect {
    fn translate(&mut self, offset: PixelPoint) {
        <Point2D<f32, PixelUnit> as Transformation<T>>::translate(&mut self.origin, offset);
    }
}

impl<T: Texture, F: Fonts> Transformation<T> for Vec<Primitive<T, F>> {
    fn translate(&mut self, offset: PixelPoint) {
        for primitive in self.iter_mut() {
            match primitive {
                Primitive::Clear { .. } => {}

                Primitive::Line {
                    start_point,
                    end_point,
                    ..
                } => {
                    <Point2D<f32, PixelUnit> as Transformation<T>>::translate(start_point, offset);
                    <Point2D<f32, PixelUnit> as Transformation<T>>::translate(end_point, offset);
                }

                Primitive::Rectangle { rect, .. } => {
                    <euclid::Rect<f32, drawing_api::PixelUnit> as Transformation<T>>::translate(
                        rect, offset,
                    );
                }

                Primitive::Image { rect, .. } => {
                    <euclid::Rect<f32, drawing_api::PixelUnit> as Transformation<T>>::translate(
                        rect, offset,
                    );
                }

                Primitive::Text {
                    position,
                    clipping_rect,
                    ..
                } => {
                    <Point2D<f32, drawing_api::PixelUnit> as Transformation<T>>::translate(
                        position, offset,
                    );
                    <euclid::Rect<f32, drawing_api::PixelUnit> as Transformation<T>>::translate(
                        clipping_rect,
                        offset,
                    );
                }

                Primitive::Stroke { path, .. } => {
                    <Vec<PathElement> as Transformation<T>>::translate(path, offset)
                }

                Primitive::StrokeStyled { path, .. } => {
                    <Vec<PathElement> as Transformation<T>>::translate(path, offset)
                }

                Primitive::Fill { path, .. } => {
                    <Vec<PathElement> as Transformation<T>>::translate(path, offset)
                }

                Primitive::ClipRect { rect, primitives } => {
                    <euclid::Rect<f32, drawing_api::PixelUnit> as Transformation<T>>::translate(
                        rect, offset,
                    );
                    primitives.translate(offset);
                }

                Primitive::ClipPath { path, primitives } => {
                    <Vec<PathElement> as Transformation<T>>::translate(path, offset);
                    primitives.translate(offset);
                }

                Primitive::Transform { transform, .. } => {
                    *transform =
                        transform.then_translate(euclid::Vector2D::new(offset.x, offset.y));
                }

                Primitive::Composite { primitives, .. } => primitives.translate(offset),
            }
        }
    }
}

impl<T: Texture> Transformation<T> for Vec<PathElement> {
    fn translate(&mut self, offset: PixelPoint) {
        for path_element in self.iter_mut() {
            match path_element {
                PathElement::MoveTo(point) => {
                    <Point2D<f32, drawing_api::PixelUnit> as Transformation<T>>::translate(
                        point, offset,
                    )
                }

                PathElement::LineTo(point) => {
                    <Point2D<f32, drawing_api::PixelUnit> as Transformation<T>>::translate(
                        point, offset,
                    )
                }

                PathElement::BezierTo(c1, c2, point) => {
                    <Point2D<f32, drawing_api::PixelUnit> as Transformation<T>>::translate(
                        c1, offset,
                    );
                    <Point2D<f32, drawing_api::PixelUnit> as Transformation<T>>::translate(
                        c2, offset,
                    );
                    <Point2D<f32, drawing_api::PixelUnit> as Transformation<T>>::translate(
                        point, offset,
                    );
                }

                PathElement::ClosePath => (),

                PathElement::Solidity(..) => (),
            }
        }
    }
}
