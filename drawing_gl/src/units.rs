use drawing_api::PixelUnit;

// backend device specific unit (for example in range -1.0 .. 1.0 for OpenGL and Direct3D)
#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DeviceUnit;

// backend texture specific unit (for example in range 0.0 .. 1.0 for OpenGL and Direct3D)
#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct UvUnit;

pub type PixelTransform = euclid::Transform2D<f32, PixelUnit, PixelUnit>;
pub type PixelToDeviceTransform = euclid::Transform2D<f32, PixelUnit, DeviceUnit>;
pub type PixelToUvTransform = euclid::Transform2D<f32, PixelUnit, UvUnit>;
