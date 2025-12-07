use crate::{FragmentShader, Matrix, Texture};

use super::{TextureSampling, TileMode};

#[derive(Clone)]
pub enum ImageFilter<T: Texture, S: FragmentShader> {
    Blur {
        x_sigma: f32,
        y_sigma: f32,
        tile_mode: TileMode,
    },

    Dilate {
        x_radius: f32,
        y_radius: f32,
    },

    Erode {
        x_radius: f32,
        y_radius: f32,
    },

    Matrix {
        matrix: Matrix,
        sampling: TextureSampling,
    },

    FragmentShader {
        program: S,
        samplers: Vec<T>,
        data: Vec<u8>,
    },

    Compose {
        outer: Box<ImageFilter<T, S>>,
        inner: Box<ImageFilter<T, S>>,
    },
}
