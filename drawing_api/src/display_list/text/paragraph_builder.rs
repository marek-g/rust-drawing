use super::ParagraphStyle;

pub trait ParagraphBuilder<T: crate::Texture, P: crate::Paint<Texture = T>> {
    type Paragraph;

    fn push_style(&mut self, style: ParagraphStyle<T, P>);

    fn pop_style(&mut self);

    fn add_text(&mut self, text: &str);

    fn build(self) -> Result<Self::Paragraph, &'static str>;
}
