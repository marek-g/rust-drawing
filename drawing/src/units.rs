use euclid::*;


// unknown units - doesn't matter what it is as long as the transform matrix to something else is provided
pub type Thickness = Length<f32, UnknownUnit>;
pub type Point = TypedPoint2D<f32, UnknownUnit>;
pub type Size = TypedSize2D<f32, UnknownUnit>;
pub type Rect = TypedRect<f32, UnknownUnit>;


// backend specific unit (for example in range -1.0 .. 1.0 for OpenGL and Direct3D)
#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DeviceUnit;

pub type DeviceThickness = Length<f32, DeviceUnit>;
pub type DevicePoint = TypedPoint2D<f32, DeviceUnit>;
pub type DeviceSize = TypedSize2D<f32, DeviceUnit>;
pub type DeviceRect = TypedRect<f32, DeviceUnit>;


// physical pixel (in range 0 .. window width - 1, 0 .. window height - 1)
#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PhysPixelUnit;

pub type PhysPixelThickness = Length<f32, PhysPixelUnit>;
pub type PhysPixelPoint = TypedPoint2D<f32, PhysPixelUnit>;
pub type PhysPixelSize = TypedSize2D<f32, PhysPixelUnit>;
pub type PhysPixelRect = TypedRect<f32, PhysPixelUnit>;


// user pixel (usually the same as physical pixel but user may be able to change it with DPI preferences)
#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct UserPixelUnit;

pub type UserPixelThickness = Length<f32, UserPixelUnit>;
pub type UserPixelPoint = TypedPoint2D<f32, UserPixelUnit>;
pub type UserPixelSize = TypedSize2D<f32, UserPixelUnit>;
pub type UserPixelRect = TypedRect<f32, UserPixelUnit>;


pub type UnknownToDeviceTransform = TypedTransform2D<f32, UnknownUnit, DeviceUnit>;
pub type DeviceTransform = TypedTransform2D<f32, DeviceUnit, DeviceUnit>;
pub type UserPixelTransform = TypedTransform2D<f32, UserPixelUnit, UserPixelUnit>;
pub type UserPixelToPhysPixelTransform = TypedTransform2D<f32, UserPixelUnit, PhysPixelUnit>;
pub type UserPixelToDeviceTransform = TypedTransform2D<f32, UserPixelUnit, DeviceUnit>;
pub type PhysPixelToDeviceTransform = TypedTransform2D<f32, PhysPixelUnit, DeviceUnit>;
