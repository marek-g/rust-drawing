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
    pub locale: Option<String>,
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
            locale: None,
        }
    }
}

impl<T, P> ParagraphStyle<T, P>
where
    T: crate::Texture,
    P: crate::Paint<Texture = T>,
{
    pub fn simple(family: impl Into<String>, size: f32, paint: P) -> Self {
        ParagraphStyle {
            foreground: Some(paint),
            family: family.into(),
            size,
            ..Default::default()
        }
    }

    pub fn with_foreground(mut self, paint: Option<P>) -> Self {
        self.foreground = paint;
        self
    }

    pub fn with_background(mut self, paint: Option<P>) -> Self {
        self.background = paint;
        self
    }

    pub fn with_weight(mut self, weight: FontWeight) -> Self {
        self.weight = weight;
        self
    }

    pub fn with_style(mut self, style: FontStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_family(mut self, family: String) -> Self {
        self.family = family;
        self
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn with_height_factor(mut self, height_factor: Option<f32>) -> Self {
        self.height_factor = height_factor;
        self
    }

    pub fn with_text_alignment(mut self, text_alignment: TextAlignment) -> Self {
        self.text_alignment = text_alignment;
        self
    }
    pub fn with_text_direction(mut self, text_direction: TextDirection) -> Self {
        self.text_direction = text_direction;
        self
    }
    pub fn with_text_decoration(mut self, text_decoration: Option<TextDecoration>) -> Self {
        self.text_decoration = text_decoration;
        self
    }
    pub fn with_max_lines(mut self, max_lines: Option<u32>) -> Self {
        self.max_lines = max_lines;
        self
    }
    pub fn with_ellipsis(mut self, ellipsis: Option<String>) -> Self {
        self.ellipsis = ellipsis;
        self
    }
    pub fn with_locale(mut self, locale: Option<String>) -> Self {
        self.locale = locale;
        self
    }
}
