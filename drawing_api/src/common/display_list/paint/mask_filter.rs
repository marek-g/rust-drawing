use super::BlurStyle;

#[derive(Clone, PartialEq)]
pub enum MaskFilter {
    Blur { style: BlurStyle, sigma: f32 },
}
