use super::{BlendMode, Color, ColorMatrix};

pub enum ColorFilter {
    Blend(Color, BlendMode),
    Matrix(ColorMatrix),
}
