use crate::units::PixelTransform;

#[derive(Debug, Copy, Clone)]
pub struct Scissor {
    pub xform: PixelTransform,
    pub extent: [f32; 2],
}
