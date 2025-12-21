use drawing_api::OptRef;

use crate::ImpellerTexture;

use super::convert_paragraph_style;

pub struct ParagraphBuilder {
    pub(crate) paragraph_builder: impellers::ParagraphBuilder,
}

impl drawing_api::ParagraphBuilder for ParagraphBuilder {
    type Paragraph = crate::Paragraph;
    type Paint = crate::Paint;
    type Fonts = crate::Fonts;
    type Texture = crate::ImpellerTexture;

    fn new(fonts: &crate::Fonts) -> Result<Self, &'static str> {
        Ok(ParagraphBuilder {
            paragraph_builder: impellers::ParagraphBuilder::new(&fonts.typography_context.borrow())
                .ok_or("Couldn't create impeller ParagraphBuilder")?,
        })
    }

    fn push_style<'a>(
        &mut self,
        style: impl Into<OptRef<'a, drawing_api::ParagraphStyle<ImpellerTexture, crate::Paint>>>,
    ) {
        let paragraph_style = convert_paragraph_style(&style.into());
        self.paragraph_builder.push_style(&paragraph_style);
    }

    fn pop_style(&mut self) {
        self.paragraph_builder.pop_style();
    }

    fn add_text(&mut self, text: &str) {
        self.paragraph_builder.add_text(text);
    }

    fn build(self) -> Result<Self::Paragraph, &'static str> {
        let paragraph = self
            .paragraph_builder
            .build(600.0f32)
            .ok_or("Impeller couldn't build the paragraph")?;
        Ok(crate::Paragraph { paragraph })
    }
}
