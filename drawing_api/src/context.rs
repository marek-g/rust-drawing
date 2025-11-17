use crate::ColorFormat;

pub trait Context {
    type DisplayList;
    type DisplayListBuilder;
    type Paint;
    type Surface;
    type Texture;

    fn create_display_list_builder(&self) -> Result<Self::DisplayListBuilder, &'static str>;
    fn create_paint(&self) -> Result<Self::Paint, &'static str>;
    fn create_texture(
        &self,
        contents: &[u8],
        width: u16,
        height: u16,
        format: ColorFormat,
    ) -> Result<Self::Texture, &'static str>;

    fn draw(
        &mut self,
        surface: &Self::Surface,
        display_list: &Self::DisplayList,
    ) -> Result<(), &'static str>;
}
