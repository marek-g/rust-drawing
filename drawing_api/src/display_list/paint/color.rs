use std::str::FromStr;

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

    pub fn argb_u32(num: u32) -> Self {
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

    pub fn argb_u16(num: u16) -> Self {
        let a = num >> 12;
        let r = (num >> 8) & 0xF;
        let g = (num >> 4) & 0xF;
        let b = num & 0xF;
        Self {
            red: r as f32 / 15.0f32,
            green: g as f32 / 15.0f32,
            blue: b as f32 / 15.0f32,
            alpha: a as f32 / 15.0f32,
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
        Color::argb_u32(num)
    }
}

impl From<&str> for Color {
    fn from(text: &str) -> Self {
        text.parse().unwrap_or(Color::default())
    }
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.trim_start_matches('#');

        match hex.len() {
            3 => {
                // Single hex digit per channel, normalize by /15.0
                let r =
                    u8::from_str_radix(&hex[0..1], 16).map_err(|e| e.to_string())? as f32 / 15.0;
                let g =
                    u8::from_str_radix(&hex[1..2], 16).map_err(|e| e.to_string())? as f32 / 15.0;
                let b =
                    u8::from_str_radix(&hex[2..3], 16).map_err(|e| e.to_string())? as f32 / 15.0;
                Ok(Color::rgba(r, g, b, 1.0))
            }
            4 => {
                // Single hex digit per channel + alpha, normalize by /15.0
                let a =
                    u8::from_str_radix(&hex[0..1], 16).map_err(|e| e.to_string())? as f32 / 15.0;
                let r =
                    u8::from_str_radix(&hex[1..2], 16).map_err(|e| e.to_string())? as f32 / 15.0;
                let g =
                    u8::from_str_radix(&hex[2..3], 16).map_err(|e| e.to_string())? as f32 / 15.0;
                let b =
                    u8::from_str_radix(&hex[3..4], 16).map_err(|e| e.to_string())? as f32 / 15.0;
                Ok(Color::rgba(r, g, b, a))
            }
            6 => {
                // Two hex digits per channel, normalize by /255.0
                let r =
                    u8::from_str_radix(&hex[0..2], 16).map_err(|e| e.to_string())? as f32 / 255.0;
                let g =
                    u8::from_str_radix(&hex[2..4], 16).map_err(|e| e.to_string())? as f32 / 255.0;
                let b =
                    u8::from_str_radix(&hex[4..6], 16).map_err(|e| e.to_string())? as f32 / 255.0;
                Ok(Color::rgba(r, g, b, 1.0))
            }
            8 => {
                // Two hex digits per channel + alpha, normalize by /255.0
                let a =
                    u8::from_str_radix(&hex[0..2], 16).map_err(|e| e.to_string())? as f32 / 255.0;
                let r =
                    u8::from_str_radix(&hex[2..4], 16).map_err(|e| e.to_string())? as f32 / 255.0;
                let g =
                    u8::from_str_radix(&hex[4..6], 16).map_err(|e| e.to_string())? as f32 / 255.0;
                let b =
                    u8::from_str_radix(&hex[6..8], 16).map_err(|e| e.to_string())? as f32 / 255.0;
                Ok(Color::rgba(r, g, b, a))
            }
            _ => Err(format!("Invalid hex color length: {}", hex.len())),
        }
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
