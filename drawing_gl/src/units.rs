use drawing_api::PixelUnit;

// backend specific unit (for example in range -1.0 .. 1.0 for OpenGL and Direct3D)
#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DeviceUnit;

pub type PixelTransform = euclid::Transform2D<f32, PixelUnit, PixelUnit>;
pub type PixelToDeviceTransform = euclid::Transform2D<f32, PixelUnit, DeviceUnit>;
