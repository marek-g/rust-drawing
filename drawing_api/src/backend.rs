pub trait Backend {
    type DisplayListBuilder;
    type Paint;
    type Surface;

    fn create_display_list_builder(&self) -> Result<Self::DisplayListBuilder, &'static str>;
    fn create_paint(&self) -> Result<Self::Paint, &'static str>;
}
