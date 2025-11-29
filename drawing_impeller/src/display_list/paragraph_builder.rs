use crate::ImpellerTexture;

use super::convert_paragraph_style;

pub struct ParagraphBuilder {
    pub(crate) paragraph_builder: impellers::ParagraphBuilder,

    context: impellers::TypographyContext,
}

impl drawing_api::ParagraphBuilder for ParagraphBuilder {
    type Paragraph = impellers::Paragraph;
    type Paint = crate::Paint;
    type Fonts = crate::Fonts;
    type Texture = crate::ImpellerTexture;

    fn new(fonts: &crate::Fonts) -> Result<Self, &'static str> {
        let context_clone = fonts.typography_context.clone();
        Ok(ParagraphBuilder {
            paragraph_builder: impellers::ParagraphBuilder::new(&context_clone)
                .ok_or("Couldn't create impeller ParagraphBuilder")?,
            context: context_clone,
        })
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
        let mut pb = ParagraphBuilder {
            paragraph_builder: impellers::ParagraphBuilder::new(&self.context).unwrap(),
            context: self.context.clone(),
        };

        let mut paragraph_style = drawing_api::ParagraphStyle::default();
        paragraph_style.family = "F1".to_string();

        pb.push_style(paragraph_style);

        pb.add_text("ggg HELLO EVERYONE ąęśćżółĄĘŚŻŹ");

        Ok(pb
            .paragraph_builder
            .build(600.0f32)
            .ok_or("Impeller couldn't build the paragraph")?)

        // TODO: width
        /*Ok(self
        .paragraph_builder
        .build(600.0f32)
        .ok_or("Impeller couldn't build the paragraph")?)*/
    }
}
