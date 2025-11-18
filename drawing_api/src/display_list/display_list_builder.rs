use crate::PixelPoint;

pub trait DisplayListBuilder {
    type DisplayList;
    type Paint: crate::Paint;

    fn draw_line(&mut self, from: PixelPoint, to: PixelPoint, paint: &Self::Paint);

    fn build(self) -> Result<Self::DisplayList, &'static str>;
}
