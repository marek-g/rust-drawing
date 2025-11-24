use super::BlurStyle;

#[derive(Clone)]
pub enum MaskFilter {
    Blur { style: BlurStyle, sigma: f32 },
}
