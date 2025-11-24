use crate::ColorFormat;

/// An abstraction over graphics context (like OpenGL context).
///
/// It is reference counted, single threaded object.
pub trait Context: Clone {
    type DisplayListBuilder;
    type DisplayList;
    type Fonts;
    type Paint;
    type PathBuilder;
    type Surface;
    type Texture;

    fn create_texture(
        &self,
        contents: &[u8],
        width: u16,
        height: u16,
        format: ColorFormat,
    ) -> Result<Self::Texture, &'static str>;

    fn draw(
        &self,
        surface: &Self::Surface,
        display_list: &Self::DisplayList,
    ) -> Result<(), &'static str>;
}
