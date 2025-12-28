use crate::smart_pointers::OptRef;

use super::ParagraphStyle;

pub trait ParagraphBuilder: Sized + 'static {
    type Paragraph: crate::Paragraph;
    type Paint: crate::Paint<Texture = Self::Texture>;
    type Fonts: crate::Fonts;
    type Texture: crate::Texture;

    fn new(fonts: &Self::Fonts) -> Result<Self, &'static str>;

    fn push_style<'a>(&mut self, style: impl Into<OptRef<'a, ParagraphStyle<Self::Paint>>>)
    where
        Self::Paint: 'a;

    fn pop_style(&mut self);

    fn add_text(&mut self, text: &str);

    fn build(self) -> Result<Self::Paragraph, &'static str>;
}
