use std::borrow::Cow;

use crate::ColorFormat;

/// An abstraction over graphics context (like OpenGL context).
///
/// It is reference counted, single threaded object.
pub trait Context: Clone {
    type DisplayListBuilder: crate::DisplayListBuilder<
        Paint = Self::Paint,
        ParagraphBuilder = Self::ParagraphBuilder,
        PathBuilder = Self::PathBuilder,
        Texture = Self::Texture,
    >;
    type Fonts: crate::Fonts;
    type Paint: crate::Paint<Texture = Self::Texture>;
    type ParagraphBuilder: crate::ParagraphBuilder<
        Texture = Self::Texture,
        Paint = Self::Paint,
        Fonts = Self::Fonts,
    >;
    type PathBuilder: crate::PathBuilder;
    type Surface: crate::Surface;
    type Texture: crate::Texture;

    fn wrap_gl_framebuffer(
        &mut self,
        framebuffer_id: u32,
        width: u16,
        height: u16,
        color_format: ColorFormat,
    ) -> Result<Self::Surface, &'static str>;

    fn create_texture(
        &self,
        contents: Cow<'static, [u8]>,
        width: u16,
        height: u16,
        color_format: ColorFormat,
    ) -> Result<Self::Texture, &'static str>;

    fn draw(
        &mut self,
        surface: &mut Self::Surface,
        display_list: &<Self::DisplayListBuilder as crate::DisplayListBuilder>::DisplayList,
    ) -> Result<(), &'static str>;
}
