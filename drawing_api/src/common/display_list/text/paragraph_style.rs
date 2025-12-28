use crate::smart_pointers::{OptRef, Owned};

use super::{FontStyle, FontWeight, TextAlignment, TextDecoration, TextDirection};

#[derive(Clone)]
pub struct ParagraphStyle<P> {
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

impl<P: Default> Default for ParagraphStyle<P> {
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

impl<P: Default> ParagraphStyle<P> {
    pub fn simple(family: impl Into<String>, size: f32, paint: impl Into<Owned<P>>) -> Self {
        ParagraphStyle {
            foreground: Some(paint.into().0),
            family: family.into(),
            size,
            ..Default::default()
        }
    }

    pub fn with_foreground(mut self, paint: impl Into<Owned<P>>) -> Self {
        self.foreground = Some(paint.into().0);
        self
    }

    pub fn without_foreground(mut self) -> Self {
        self.foreground = None;
        self
    }

    pub fn with_background(mut self, paint: impl Into<Owned<P>>) -> Self {
        self.background = Some(paint.into().0);
        self
    }

    pub fn without_background(mut self) -> Self {
        self.background = None;
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

impl<'a, P: Default> From<&'a ParagraphStyle<P>> for OptRef<'a, ParagraphStyle<P>> {
    fn from(value: &'a ParagraphStyle<P>) -> Self {
        OptRef::Borrowed(value)
    }
}

impl<'a, P: Default> From<ParagraphStyle<P>> for OptRef<'a, ParagraphStyle<P>> {
    fn from(value: ParagraphStyle<P>) -> Self {
        OptRef::Owned(value)
    }
}

impl<'a, P: Default, S: Into<String>, P2: Into<Owned<P>>> From<(S, f32, P2)>
    for OptRef<'a, ParagraphStyle<P>>
{
    fn from(value: (S, f32, P2)) -> Self {
        OptRef::Owned(ParagraphStyle::simple(value.0, value.1, value.2))
    }
}
