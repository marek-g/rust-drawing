use anyhow::Result;

use crate::backend::Device;
use crate::color::*;
use crate::units::*;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct FontParams {
    pub size: u8,
}

pub trait Font<D: Device> {
    fn create(device: &mut D, bytes: Vec<u8>) -> Result<Self>
    where
        Self: Sized;

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

    fn get_dimensions(
        &mut self,
        device: &mut D,
        params: FontParams,
        text: &str,
    ) -> Result<(u16, u16)>;

    fn get_dimensions_each_char(
        &mut self,
        device: &mut D,
        params: FontParams,
        text: &str,
    ) -> Result<(Vec<i16>, u16)>;
}
