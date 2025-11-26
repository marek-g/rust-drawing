use crate::{ImpellerSurface, ImpellerTexture};

#[derive(Clone)]
pub struct ImpellerContext {}

impl drawing_api::Context for ImpellerContext {
    type DisplayListBuilder = crate::DisplayListBuilder;

    type Fonts = crate::Fonts;

    type Paint = crate::Paint;

    type ParagraphBuilder = crate::ParagraphBuilder;

    type PathBuilder = crate::PathBuilder;

    type Surface = ImpellerSurface;

    type Texture = ImpellerTexture;

    fn create_texture(
        &self,
        contents: &[u8],
        width: u16,
        height: u16,
        format: drawing_api::ColorFormat,
    ) -> Result<Self::Texture, &'static str> {
        todo!()
    }

    fn draw(
        &self,
        surface: &Self::Surface,
        display_list: &<Self::DisplayListBuilder as drawing_api::DisplayListBuilder>::DisplayList,
    ) -> Result<(), &'static str> {
        todo!()
    }
}
