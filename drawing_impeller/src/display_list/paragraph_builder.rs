use crate::ImpellerTexture;

use super::{convert_paragraph_style, ttx};

pub struct ParagraphBuilder {
    pub(crate) paragraph_builder: impellers::ParagraphBuilder,
}

impl drawing_api::ParagraphBuilder for ParagraphBuilder {
    type Paragraph = impellers::Paragraph;
    type Paint = crate::Paint;
    type Fonts = crate::Fonts;
    type Texture = crate::ImpellerTexture;

    fn new(fonts: &crate::Fonts) -> Result<Self, &'static str> {
        /*Ok(ParagraphBuilder {
                    paragraph_builder: impellers::ParagraphBuilder::new(&fonts.typography_context.borrow())
                        .ok_or("Couldn't create impeller ParagraphBuilder")?,
        })*/
        unsafe {
            #[allow(static_mut_refs)]
            Ok(ParagraphBuilder {
                paragraph_builder: impellers::ParagraphBuilder::new(ttx.as_ref().unwrap())
                    .ok_or("Couldn't create impeller ParagraphBuilder")?,
            })
        }
    }

    fn push_style(&mut self, style: drawing_api::ParagraphStyle<ImpellerTexture, crate::Paint>) {
        let paragraph_style = convert_paragraph_style(&style);
        self.paragraph_builder.push_style(&paragraph_style);
    }

    fn pop_style(&mut self) {
        self.paragraph_builder.pop_style();
    }

    fn add_text(&mut self, text: &str) {
        self.paragraph_builder.add_text(text);
    }

    fn build(mut self) -> Result<Self::Paragraph, &'static str> {
        // TODO: width
        Ok(self
            .paragraph_builder
            .build(600.0f32)
            .ok_or("Impeller couldn't build the paragraph")?)
    }
}
