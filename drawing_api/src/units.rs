// unknown units - doesn't matter what it is as long as the transform matrix to/from something else is provided
pub type Length = euclid::Length<f32, euclid::UnknownUnit>;
pub type Point = euclid::Point2D<f32, euclid::UnknownUnit>;
pub type Size = euclid::Size2D<f32, euclid::UnknownUnit>;
pub type Rect = euclid::Rect<f32, euclid::UnknownUnit>;

// backend specific unit (for example in range -1.0 .. 1.0 for OpenGL and Direct3D)
#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DeviceUnit;

pub type DeviceLength = euclid::Length<f32, DeviceUnit>;
pub type DevicePoint = euclid::Point2D<f32, DeviceUnit>;
pub type DeviceSize = euclid::Size2D<f32, DeviceUnit>;
pub type DeviceRect = euclid::Rect<f32, DeviceUnit>;

// physical pixel (in range 0 .. window width - 1, 0 .. window height - 1)
#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PixelUnit;

pub type PixelLength = euclid::Length<f32, PixelUnit>;
pub type PixelPoint = euclid::Point2D<f32, PixelUnit>;
pub type PixelSize = euclid::Size2D<f32, PixelUnit>;
pub type PixelRect = euclid::Rect<f32, PixelUnit>;

// density independent pixel (unit that is based on the physical density of the screen)
#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DipUnit;

pub type DipLength = euclid::Length<f32, DipUnit>;
pub type DipPoint = euclid::Point2D<f32, DipUnit>;
pub type DipSize = euclid::Size2D<f32, DipUnit>;
pub type DipRect = euclid::Rect<f32, DipUnit>;

pub type UnknownToDeviceTransform = euclid::Transform2D<f32, euclid::UnknownUnit, DeviceUnit>;
pub type DeviceTransform = euclid::Transform2D<f32, DeviceUnit, DeviceUnit>;
pub type PixelTransform = euclid::Transform2D<f32, PixelUnit, PixelUnit>;
pub type DipTransform = euclid::Transform2D<f32, DipUnit, DipUnit>;
pub type DipToPixelTransform = euclid::Transform2D<f32, DipUnit, PixelUnit>;
pub type DipToDeviceTransform = euclid::Transform2D<f32, DipUnit, DeviceUnit>;
pub type PixelToDeviceTransform = euclid::Transform2D<f32, PixelUnit, DeviceUnit>;

pub type Matrix = euclid::Transform3D<f32, euclid::UnknownUnit, euclid::UnknownUnit>;
