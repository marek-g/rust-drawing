use super::{FontStyle, FontWeight, TextAlignment, TextDecoration, TextDirection};

pub struct ParagraphStyle<T: crate::Texture, P: crate::Paint<Texture = T>> {
    pub foreground: Option<P>,
    pub background: Option<P>,
    pub weight: FontWeight,
    pub style: FontStyle,
    pub family: String,
    pub size: f32,
    pub height_factor: Option<f32>,
    pub text_alignment: TextAlignment,
    pub text_direction: TextDirection,
    pub text_decoration: Option<TextDecoration>,
    pub max_lines: Option<u32>,
    pub ellipsis: Option<String>,
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
            height_factor: None,
            text_alignment: TextAlignment::Start,
            text_direction: TextDirection::LTR,
            text_decoration: None,
            max_lines: None,
            ellipsis: None,
        }
    }
}
