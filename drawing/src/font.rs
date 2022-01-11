use anyhow::Result;

use crate::backend::Device;
use crate::color::*;
use crate::units::*;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct FontParams {
    pub size: u8,
}

pub trait Font<D: Device> {
    /// Safe to call from any thread for any device type (even for OpenGL).
    /// To achieve it the device specific resources creation (like texture)
    /// is delayed to the first draw() call.
    fn create(bytes: Vec<u8>) -> Result<Self>
    where
        Self: Sized;

    /// Safe to call from any thread for any device type (even for OpenGL).
    /// To achieve it the device specific resources creation (like texture)
    /// is delayed to the first draw() call.
    fn get_dimensions(&mut self, params: FontParams, text: &str) -> Result<(u16, u16)>;

    /// Safe to call from any thread for any device type (even for OpenGL).
    /// To achieve it the device specific resources creation (like texture)
    /// is delayed to the first draw() call.
    fn get_dimensions_each_char(
        &mut self,
        params: FontParams,
        text: &str,
    ) -> Result<(Vec<i16>, u16)>;

    // Not safe to call from any thread for some device types (like OpenGL).
    fn draw(
        &mut self,
        device: &mut D,
        target: &D::RenderTarget,
        color: &Color,
        text: &str,
        pos: Point,
        clipping_rect: Rect,
        font_params: FontParams,
        transform: UnknownToDeviceTransform,
    ) -> Result<()>;
}
