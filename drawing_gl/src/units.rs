use drawing_api::{PixelUnit, Transform2D};

// backend device specific unit (for example in range -1.0 .. 1.0 for OpenGL and Direct3D)
#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DeviceUnit;

// backend texture specific unit (for example in range 0.0 .. 1.0 for OpenGL and Direct3D)
#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct UvUnit;

pub type PixelTransform = Transform2D<f32, PixelUnit, PixelUnit>;
pub type PixelToDeviceTransform = Transform2D<f32, PixelUnit, DeviceUnit>;
pub type PixelToUvTransform = Transform2D<f32, PixelUnit, UvUnit>;
