use super::BlurStyle;

pub enum MaskFilter {
    Blur { style: BlurStyle, sigma: f32 },
}
