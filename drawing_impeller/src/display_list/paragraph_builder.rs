use crate::ImpellerTexture;

pub struct ParagraphBuilder;

impl drawing_api::ParagraphBuilder for ParagraphBuilder {
    type Paragraph = ();
    type Paint = crate::Paint;
    type Fonts = crate::Fonts;
    type Texture = crate::ImpellerTexture;

    fn new(fonts: &crate::Fonts) -> Self {
        ParagraphBuilder {}
    }

    fn push_style(&mut self, style: drawing_api::ParagraphStyle<ImpellerTexture, crate::Paint>) {
        todo!()
    }

    fn pop_style(&mut self) {
        todo!()
    }

    fn add_text(&mut self, text: &str) {
        todo!()
    }

    fn build(self) -> Result<Self::Paragraph, &'static str> {
        todo!()
    }
}
