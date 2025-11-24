use super::{BlendMode, Color, ColorMatrix};

#[derive(Clone)]
pub enum ColorFilter {
    Blend(Color, BlendMode),
    Matrix(ColorMatrix),
}
