// physical pixel (in range 0 .. window width - 1, 0 .. window height - 1)
#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PixelUnit;

pub type PixelPoint = euclid::Point2D<f32, PixelUnit>;
pub type PixelSize = euclid::Size2D<f32, PixelUnit>;
pub type PixelRect = euclid::Rect<f32, PixelUnit>;

pub type Matrix = euclid::Transform3D<f32, euclid::UnknownUnit, euclid::UnknownUnit>;
