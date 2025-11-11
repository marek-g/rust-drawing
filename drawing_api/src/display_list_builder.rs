use crate::Point;

pub trait DisplayListBuilder {
    type DisplayList;
    type Paint: crate::Paint;

    fn draw_line(&self, from: Point, to: Point, paint: &Self::Paint);

    fn build(&self) -> Result<Self::DisplayList, &'static str>;
}
