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

impl<T: crate::Texture, P: crate::Paint<Texture = T>> Default for ParagraphStyle<T, P> {
    fn default() -> Self {
        Self {
            foreground: Some(P::default()),
            background: None,
            weight: FontWeight::Regular,
            style: FontStyle::Normal,
            family: "default".to_string(),
            size: 24.0f32,
            height: 24.0f32,
        }
    }
}
