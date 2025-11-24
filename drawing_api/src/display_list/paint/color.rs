#[derive(Debug, Clone)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
    pub color_space: ColorSpace,
}

impl Color {
    pub fn rgb(red: f32, green: f32, blue: f32) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: 1.0f32,
            color_space: ColorSpace::SRGB,
        }
    }

    pub fn rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
            color_space: ColorSpace::SRGB,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ColorSpace {
    SRGB,
    ExtendedSRGB,
    DisplayP3,
}

pub enum ColorFormat {
    // for color images, 24-bit color with 8-bit alpha channel
    RGBA,

    // 8-bit channel, for use with monochromatic textures (like fonts)
    Y8,
}
