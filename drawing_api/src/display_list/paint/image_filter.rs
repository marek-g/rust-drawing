use crate::Matrix;

use super::{ImageFilterFragment, TextureSampling, TileMode};

#[derive(Clone)]
pub enum ImageFilter<F: ImageFilterFragment> {
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

    Fragment {
        image_filter: F,
    },

    Compose {
        outer: Box<ImageFilter<F>>,
        inner: Box<ImageFilter<F>>,
    },
}
