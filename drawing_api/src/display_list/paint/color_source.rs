use crate::{Matrix, Point, Texture};

use super::{Color, TextureSampling, TileMode};

pub enum ColorSource<'a, T: Texture> {
    LinearGradient {
        start: Point,
        end: Point,
        colors: &'a [Color],
        stops: &'a [f32],
        tile_mode: TileMode,
        transformation: Option<&'a Matrix>,
    },

    RadialGradient {
        center: Point,
        radius: f32,
        colors: &'a [Color],
        stops: &'a [f32],
        tile_mode: TileMode,
        transformation: Option<&'a Matrix>,
    },

    ConicalGradient {
        start_center: Point,
        start_radius: f32,
        end_center: Point,
        end_radius: f32,
        colors: &'a [Color],
        stops: &'a [f32],
        tile_mode: TileMode,
        transformation: Option<&'a Matrix>,
    },

    SweepGradient {
        center: Point,
        start: Point,
        end: Point,
        colors: &'a [Color],
        stops: &'a [f32],
        tile_mode: TileMode,
        transformation: Option<&'a Matrix>,
    },

    Image {
        image: T,
        horizontal_tile_mode: TileMode,
        vertical_tile_mode: TileMode,
        sampling: TextureSampling,
        transformation: Option<&'a Matrix>,
    },
}
