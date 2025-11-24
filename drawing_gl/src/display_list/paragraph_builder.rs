use drawing_api::{PixelLength, PixelPoint};

use crate::{GlContextData, GlTexture};

use super::Primitive;

pub struct ParagraphBuilder {
    fonts: crate::Fonts<GlContextData>,
    paragraph: Vec<Primitive<GlTexture, crate::Fonts<GlContextData>>>,
    styles: Vec<drawing_api::ParagraphStyle<GlTexture, crate::display_list::Paint>>,
}

impl ParagraphBuilder {
    pub fn new(fonts: &crate::Fonts<GlContextData>) -> Self {
        ParagraphBuilder {
            fonts: fonts.clone(),
            paragraph: Vec::new(),
            styles: Vec::new(),
        }
    }
}

impl drawing_api::ParagraphBuilder<GlTexture, crate::display_list::Paint> for ParagraphBuilder {
    type Paragraph = Vec<Primitive<GlTexture, crate::Fonts<GlContextData>>>;

    fn push_style(
        &mut self,
        style: drawing_api::ParagraphStyle<GlTexture, crate::display_list::Paint>,
    ) {
        self.styles.push(style);
    }

    fn pop_style(&mut self) {
        self.styles.pop();
    }

    fn add_text(&mut self, text: &str) {
        let style = &self.styles.last();
        self.paragraph.push(Primitive::Text {
            fonts: self.fonts.clone(),
            family_name: style
                .map(|s| s.family.clone())
                .unwrap_or("default".to_string()),
            size: PixelLength::new(style.map(|s| s.size).unwrap_or(24.0f32)),
            color: style
                .map(|s| s.foreground.clone().map(|p| p.color))
                .flatten()
                .unwrap_or([0.0f32, 0.0f32, 0.0f32, 1.0f32]),
            position: PixelPoint::new(0.0f32, 0.0f32),
            clipping_rect: None,
            text: text.to_owned(),
        });
    }

    fn build(self) -> Result<Self::Paragraph, &'static str> {
        Ok(self.paragraph)
    }
}
