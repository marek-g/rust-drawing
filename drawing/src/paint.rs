// file based on https://github.com/sunli829/nvg
// released on MIT license
// which was translated from https://github.com/memononen/nanovg (zlib license)

use crate::backend::Device;
use crate::backend::Texture;
use crate::color::Color;
use crate::font::Font;
use crate::primitive::Brush;
use crate::resources::Resources;
use crate::units::PixelRect;
use crate::units::PixelTransform;

#[derive(Debug, Copy, Clone)]
pub struct Paint {
    pub xform: PixelTransform,
    pub extent: [f32; 2],
    pub radius: f32,
    pub feather: f32,
    pub inner_color: Color,
    pub outer_color: Color,
    pub image: Option<i32>,
}

impl Paint {
    pub fn from_brush<'a, D: Device, F: Font<D>>(
        brush: &Brush,
        resources: &'a mut Resources<D, F>,
    ) -> (Self, Option<&'a D::Texture>) {
        match brush {
            Brush::Color { ref color } => (
                Paint {
                    xform: PixelTransform::identity(),
                    extent: [0.0, 0.0],
                    radius: 0.0,
                    feather: 1.0,
                    inner_color: *color,
                    outer_color: *color,
                    image: None,
                },
                None,
            ),

            Brush::LinearGradient {
                start_point,
                end_point,
                inner_color,
                outer_color,
            } => {
                const LARGE: f32 = 1e5;

                let mut dx = end_point.x - start_point.x;
                let mut dy = end_point.y - start_point.y;
                let d = (dx * dx + dy * dy).sqrt();

                if d > 0.0001 {
                    dx /= d;
                    dy /= d;
                } else {
                    dx = 0.0;
                    dy = 1.0;
                }

                (
                    Paint {
                        xform: PixelTransform::new(
                            dy,
                            -dx,
                            dx,
                            dy,
                            start_point.x - dx * LARGE,
                            start_point.y - dy * LARGE,
                        ),
                        extent: [LARGE, LARGE + d * 0.5],
                        radius: 0.0,
                        feather: d.max(1.0),
                        inner_color: *inner_color,
                        outer_color: *outer_color,
                        image: None,
                    },
                    None,
                )
            }

            Brush::RadialGradient {
                center_point,
                in_radius,
                out_radius,
                inner_color,
                outer_color,
            } => {
                let r = (in_radius + out_radius) * 0.5;
                let f = out_radius - in_radius;
                (
                    Paint {
                        xform: PixelTransform::new(
                            1.0,
                            0.0,
                            0.0,
                            1.0,
                            center_point.x,
                            center_point.y,
                        ),
                        extent: [r, r],
                        radius: r,
                        feather: f.max(1.0),
                        inner_color: *inner_color,
                        outer_color: *outer_color,
                        image: None,
                    },
                    None,
                )
            }

            Brush::ShadowGradient {
                rect,
                radius,
                feather,
                inner_color,
                outer_color,
            } => {
                let PixelRect { origin, size } = rect;
                (
                    Paint {
                        xform: PixelTransform::new(
                            1.0,
                            0.0,
                            0.0,
                            1.0,
                            origin.x + size.width * 0.5,
                            origin.y + size.height * 0.5,
                        ),
                        extent: [size.width * 0.5, size.height * 0.5],
                        radius: *radius,
                        feather: feather.max(1.0),
                        inner_color: *inner_color,
                        outer_color: *outer_color,
                        image: None,
                    },
                    None,
                )
            }

            Brush::ImagePattern {
                resource_key,
                transform,
                alpha,
            } => {
                let texture = resources.textures_mut().get(resource_key);
                let extent_size = if let Some(texture) = texture {
                    let size = texture.get_size();
                    [size.0 as f32, size.1 as f32]
                } else {
                    [1.0f32, 1.0f32]
                };
                (
                    Paint {
                        xform: *transform,
                        extent: extent_size,
                        radius: 0.0,
                        feather: 0.0,
                        inner_color: [1.0, 1.0, 1.0, *alpha],
                        outer_color: [1.0, 1.0, 1.0, *alpha],
                        image: Some(*resource_key),
                    },
                    texture,
                )
            }
        }
    }
}
