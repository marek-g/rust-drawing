use crate::Matrix;

use super::{TextureSampling, TileMode};

pub enum ImageFilter {
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

    Compose {
        outer: Box<ImageFilter>,
        inner: Box<ImageFilter>,
    },
}
