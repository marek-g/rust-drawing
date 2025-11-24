use crate::{Matrix, Point, Texture};

use super::{Color, TextureSampling, TileMode};

#[derive(Debug, Clone)]
pub enum ColorSource<T: Texture> {
    LinearGradient {
        start: Point,
        end: Point,
        colors: Vec<Color>,
        stops: Vec<f32>,
        tile_mode: TileMode,
        transformation: Option<Matrix>,
    },

    RadialGradient {
        center: Point,
        radius: f32,
        colors: Vec<Color>,
        stops: Vec<f32>,
        tile_mode: TileMode,
        transformation: Option<Matrix>,
    },

    ConicalGradient {
        start_center: Point,
        start_radius: f32,
        end_center: Point,
        end_radius: f32,
        colors: Vec<Color>,
        stops: Vec<f32>,
        tile_mode: TileMode,
        transformation: Option<Matrix>,
    },

    SweepGradient {
        center: Point,
        start: Point,
        end: Point,
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
}
