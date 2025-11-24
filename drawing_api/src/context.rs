use crate::ColorFormat;

/// An abstraction over graphics context (like OpenGL context).
///
/// It is reference counted, single threaded object.
pub trait Context: Clone {
    type DisplayListBuilder: crate::DisplayListBuilder;
    type Fonts: crate::Fonts;
    type Paint: crate::Paint<Texture = Self::Texture>;
    type ParagraphBuilder: crate::ParagraphBuilder<Self::Texture, Self::Paint>;
    type PathBuilder: crate::PathBuilder;
    type Surface: crate::Surface;
    type Texture: crate::Texture;

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
        display_list: &<Self::DisplayListBuilder as crate::DisplayListBuilder>::DisplayList,
    ) -> Result<(), &'static str>;
}
