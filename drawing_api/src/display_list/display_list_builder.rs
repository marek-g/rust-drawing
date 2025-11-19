use crate::DipPoint;

pub trait DisplayListBuilder {
    type DisplayList;
    type Paint: crate::Paint;

    /// Fills the current clip with the specified paint.
    fn draw_paint(&mut self, paint: &Self::Paint);

    fn draw_line(
        &mut self,
        from: impl Into<DipPoint>,
        to: impl Into<DipPoint>,
        paint: &Self::Paint,
    );

    fn build(self) -> Result<Self::DisplayList, &'static str>;
}
