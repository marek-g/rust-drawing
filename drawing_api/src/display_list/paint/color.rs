#[derive(Debug, Clone)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
    pub color_space: ColorSpace,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            red: 0.0f32,
            green: 0.0f32,
            blue: 0.0f32,
            alpha: 1.0f32,
            color_space: ColorSpace::SRGB,
        }
    }
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

    pub fn from_num(num: u32) -> Self {
        let a = num >> 24;
        let r = (num >> 16) & 0xFF;
        let g = (num >> 8) & 0xFF;
        let b = num & 0xFF;
        Self {
            red: r as f32 / 255.0f32,
            green: g as f32 / 255.0f32,
            blue: b as f32 / 255.0f32,
            alpha: a as f32 / 255.0f32,
            color_space: ColorSpace::SRGB,
        }
    }
}

impl From<(f32, f32, f32)> for Color {
    fn from(rgb: (f32, f32, f32)) -> Self {
        Color::rgb(rgb.0, rgb.1, rgb.2)
    }
}

impl From<[f32; 3]> for Color {
    fn from(rgb: [f32; 3]) -> Self {
        Color::rgb(rgb[0], rgb[1], rgb[2])
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from(rgba: (f32, f32, f32, f32)) -> Self {
        Color::rgba(rgba.0, rgba.1, rgba.2, rgba.3)
    }
}

impl From<[f32; 4]> for Color {
    fn from(rgba: [f32; 4]) -> Self {
        Color::rgba(rgba[0], rgba[1], rgba[2], rgba[3])
    }
}

impl From<u32> for Color {
    fn from(num: u32) -> Self {
        Color::from_num(num)
    }
}

#[derive(Debug, Clone)]
pub enum ColorSpace {
    SRGB,
    ExtendedSRGB,
    DisplayP3,
}

#[derive(PartialEq, Copy, Clone)]
pub enum ColorFormat {
    // for color images, 24-bit color with 8-bit alpha channel
    RGBA,

    // 8-bit channel, for use with monochromatic textures (like fonts)
    Y8,
}

impl Default for ColorFormat {
    fn default() -> Self {
        ColorFormat::RGBA
    }
}
