use drawing_api::Point;

use super::Primitive;

pub struct DisplayListBuilder;

impl drawing_api::DisplayListBuilder for DisplayListBuilder {
    type DisplayList = Vec<Primitive>;
    type Paint = crate::generic::device::Paint;

    fn draw_line(&self, from: Point, to: Point, paint: &Self::Paint) {}

    fn build(&self) -> Result<Self::DisplayList, &'static str> {
        todo!()
    }
}
