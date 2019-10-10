use crate::units::*;
use crate::primitive::*;

pub trait PrimitiveTransformations {
    fn translate(&mut self, offset: UserPixelPoint);
}

impl PrimitiveTransformations for Vec<Primitive> {
    fn translate(&mut self, offset: UserPixelPoint) {
        for primitive in self.iter_mut() {
            match primitive {
                Primitive::Line { ref mut start_point, ref mut end_point, .. } => {
                    start_point.x += offset.x; start_point.y += offset.y;
                    end_point.x += offset.x; end_point.y += offset.y;
                },
                Primitive::Rectangle { ref mut rect, .. } => {
                    rect.origin.x += offset.x; rect.origin.y += offset.y;
                },
                Primitive::Text { ref mut position, .. } => {
                    position.x += offset.x; position.y += offset.y;
                },
                Primitive::Image { ref mut rect, .. } => {
                    rect.origin.x += offset.x; rect.origin.y += offset.y;
                },
                Primitive::PushLayer { .. } => (),
                Primitive::PopLayer => (),
            }
        }
    }
}
