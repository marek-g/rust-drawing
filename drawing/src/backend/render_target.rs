use core::marker::Sized;
use crate::units::PixelToDeviceTransform;

pub trait RenderTarget: Sized {
    fn get_size(&self) -> (u16, u16);

    fn get_aspect_ratio(&self) -> f32;

    fn get_device_transform(&self) -> PixelToDeviceTransform;
}
