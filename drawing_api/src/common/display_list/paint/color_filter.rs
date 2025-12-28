use super::{BlendMode, Color, ColorMatrix};

#[derive(Clone, PartialEq)]
pub enum ColorFilter {
    Blend(Color, BlendMode),
    Matrix(ColorMatrix),
}
