use drawing_api::{OptRef, PixelPoint};

use crate::{GlContext, GlTexture};

use super::Primitive;

pub struct ParagraphBuilder {
    fonts: crate::Fonts<GlContext>,
    paragraph: crate::display_list::Paragraph,
    styles: Vec<drawing_api::ParagraphStyle<GlTexture, crate::display_list::Paint>>,
}

impl drawing_api::ParagraphBuilder for ParagraphBuilder {
    type Paragraph = crate::display_list::Paragraph;
    type Paint = crate::display_list::Paint;
    type Fonts = crate::Fonts<GlContext>;
    type Texture = GlTexture;

    fn new(fonts: &crate::Fonts<GlContext>) -> Result<Self, &'static str> {
        Ok(ParagraphBuilder {
            fonts: fonts.clone(),
            paragraph: crate::display_list::Paragraph::default(),
            styles: Vec::new(),
        })
    }

    fn push_style<'a>(
        &mut self,
        style: impl Into<OptRef<'a, drawing_api::ParagraphStyle<GlTexture, crate::display_list::Paint>>>,
    ) {
        let style = style.into();
        self.styles.push(style.to_owned());
    }

    fn pop_style(&mut self) {
        self.styles.pop();
    }

    fn add_text(&mut self, text: &str) {
        let style = &self.styles.last();
        self.paragraph.primitives.push(Primitive::Text {
            fonts: self.fonts.clone(),
            family_name: style
                .map(|s| s.family.clone())
                .unwrap_or("default".to_string()),
            size: style.map(|s| s.size).unwrap_or(24.0f32),
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
