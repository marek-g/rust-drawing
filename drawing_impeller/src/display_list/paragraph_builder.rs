use crate::ImpellerTexture;

pub struct ParagraphBuilder;

impl drawing_api::ParagraphBuilder<ImpellerTexture, crate::Paint, crate::Fonts>
    for ParagraphBuilder
{
    type Paragraph = ();

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
