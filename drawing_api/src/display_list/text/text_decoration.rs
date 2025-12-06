use crate::Color;

pub struct TextDecoration {
    pub types: TextDecorationType,
    pub color: Color,
    pub style: TextDecorationStyle,
    pub thickness_multiplier: f32,
}

impl Default for TextDecoration {
    fn default() -> Self {
        Self {
            types: Default::default(),
            color: Default::default(),
            style: TextDecorationStyle::Solid,
            thickness_multiplier: 1.0f32,
        }
    }
}

pub enum TextDecorationStyle {
    Solid,
    Double,
    Dotted,
    Dashed,
    Wavy,
}

pub struct TextDecorationType {
    pub underline: bool,
    pub overline: bool,
    pub line_through: bool,
}

impl Default for TextDecorationType {
    fn default() -> Self {
        Self {
            underline: false,
            overline: false,
            line_through: false,
        }
    }
}
