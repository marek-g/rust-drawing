use crate::{smart_pointers::OptRef, PixelSize};

#[derive(Debug, Clone, PartialEq)]
pub struct RoundingRadii {
    pub top_left: PixelSize,
    pub bottom_left: PixelSize,
    pub top_right: PixelSize,
    pub bottom_right: PixelSize,
}

impl RoundingRadii {
    pub fn single_radii(radius: f32) -> Self {
        RoundingRadii {
            top_left: PixelSize::new(radius, radius),
            bottom_left: PixelSize::new(radius, radius),
            top_right: PixelSize::new(radius, radius),
            bottom_right: PixelSize::new(radius, radius),
        }
    }

    pub fn axis_radii(x_radius: f32, y_radius: f32) -> Self {
        RoundingRadii {
            top_left: PixelSize::new(x_radius, y_radius),
            bottom_left: PixelSize::new(x_radius, y_radius),
            top_right: PixelSize::new(x_radius, y_radius),
            bottom_right: PixelSize::new(x_radius, y_radius),
        }
    }
}

impl<'a> From<&'a RoundingRadii> for OptRef<'a, RoundingRadii> {
    fn from(value: &'a RoundingRadii) -> Self {
        OptRef::Borrowed(value)
    }
}

impl<'a> From<RoundingRadii> for OptRef<'a, RoundingRadii> {
    fn from(value: RoundingRadii) -> Self {
        OptRef::Owned(value)
    }
}

impl<'a> From<f32> for OptRef<'a, RoundingRadii> {
    fn from(value: f32) -> Self {
        OptRef::Owned(RoundingRadii::single_radii(value))
    }
}

impl<'a> From<(f32, f32)> for OptRef<'a, RoundingRadii> {
    fn from(value: (f32, f32)) -> Self {
        OptRef::Owned(RoundingRadii::axis_radii(value.0, value.1))
    }
}
