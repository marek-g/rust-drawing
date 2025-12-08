use crate::{FragmentShader, Matrix, PixelPoint, Texture};

use super::{Color, TextureSampling, TileMode};

#[derive(Debug, Clone)]
pub enum ColorSource<T: Texture, S: FragmentShader> {
    LinearGradient {
        start: PixelPoint,
        end: PixelPoint,
        colors: Vec<Color>,
        stops: Vec<f32>,
        tile_mode: TileMode,
        transformation: Option<Matrix>,
    },

    RadialGradient {
        center: PixelPoint,
        radius: f32,
        colors: Vec<Color>,
        stops: Vec<f32>,
        tile_mode: TileMode,
        transformation: Option<Matrix>,
    },

    ConicalGradient {
        start_center: PixelPoint,
        start_radius: f32,
        end_center: PixelPoint,
        end_radius: f32,
        colors: Vec<Color>,
        stops: Vec<f32>,
        tile_mode: TileMode,
        transformation: Option<Matrix>,
    },

    SweepGradient {
        center: PixelPoint,
        start: f32,
        end: f32,
        colors: Vec<Color>,
        stops: Vec<f32>,
        tile_mode: TileMode,
        transformation: Option<Matrix>,
    },

    Image {
        image: T,
        horizontal_tile_mode: TileMode,
        vertical_tile_mode: TileMode,
        sampling: TextureSampling,
        transformation: Option<Matrix>,
    },

    FragmentShader {
        program: S,
        samplers: Vec<T>,
        data: Vec<u8>,
    },
}
