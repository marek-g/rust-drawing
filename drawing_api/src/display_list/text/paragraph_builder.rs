use super::ParagraphStyle;

pub trait ParagraphBuilder: Sized {
    type Paragraph;
    type Paint: crate::Paint<Texture = Self::Texture>;
    type Fonts: crate::Fonts;
    type Texture: crate::Texture;

    fn new(fonts: &Self::Fonts) -> Result<Self, &'static str>;

    fn push_style(&mut self, style: ParagraphStyle<Self::Texture, Self::Paint>);

    fn pop_style(&mut self);

    fn add_text(&mut self, text: &str);

    fn build(self) -> Result<Self::Paragraph, &'static str>;
}
