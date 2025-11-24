use super::{FontStyle, FontWeight};

pub struct ParagraphStyle<T: crate::Texture, P: crate::Paint<Texture = T>> {
    pub foreground: Option<P>,
    pub background: Option<P>,
    pub weight: FontWeight,
    pub style: FontStyle,
    pub family: String,
    pub size: f32,
    pub height: f32,
}
