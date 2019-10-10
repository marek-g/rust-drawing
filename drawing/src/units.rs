// unknown units - doesn't matter what it is as long as the transform matrix to something else is provided
pub type Thickness = euclid::Length<f32, euclid::UnknownUnit>;
pub type Point = euclid::Point2D<f32, euclid::UnknownUnit>;
pub type Size = euclid::Size2D<f32, euclid::UnknownUnit>;
pub type Rect = euclid::Rect<f32, euclid::UnknownUnit>;


// backend specific unit (for example in range -1.0 .. 1.0 for OpenGL and Direct3D)
#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DeviceUnit;

pub type DeviceThickness = euclid::Length<f32, DeviceUnit>;
pub type DevicePoint = euclid::Point2D<f32, DeviceUnit>;
pub type DeviceSize = euclid::Size2D<f32, DeviceUnit>;
pub type DeviceRect = euclid::Rect<f32, DeviceUnit>;


// physical pixel (in range 0 .. window width - 1, 0 .. window height - 1)
#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PhysPixelUnit;

pub type PhysPixelThickness = euclid::Length<f32, PhysPixelUnit>;
pub type PhysPixelPoint = euclid::Point2D<f32, PhysPixelUnit>;
pub type PhysPixelSize = euclid::Size2D<f32, PhysPixelUnit>;
pub type PhysPixelRect = euclid::Rect<f32, PhysPixelUnit>;


// user pixel (usually the same as physical pixel but user may be able to change it with DPI preferences)
#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct UserPixelUnit;

pub type UserPixelThickness = euclid::Length<f32, UserPixelUnit>;
pub type UserPixelPoint = euclid::Point2D<f32, UserPixelUnit>;
pub type UserPixelSize = euclid::Size2D<f32, UserPixelUnit>;
pub type UserPixelRect = euclid::Rect<f32, UserPixelUnit>;


pub type UnknownToDeviceTransform = euclid::Transform2D<f32, euclid::UnknownUnit, DeviceUnit>;
pub type DeviceTransform = euclid::Transform2D<f32, DeviceUnit, DeviceUnit>;
pub type UserPixelTransform = euclid::Transform2D<f32, UserPixelUnit, UserPixelUnit>;
pub type UserPixelToPhysPixelTransform = euclid::Transform2D<f32, UserPixelUnit, PhysPixelUnit>;
pub type UserPixelToDeviceTransform = euclid::Transform2D<f32, UserPixelUnit, DeviceUnit>;
pub type PhysPixelToDeviceTransform = euclid::Transform2D<f32, PhysPixelUnit, DeviceUnit>;
