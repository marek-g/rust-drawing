use crate::units::PixelToDeviceTransform;
use core::marker::Sized;

pub trait RenderTarget: Sized {
    type Device: crate::generic::device::Device;

    fn get_device(&self) -> Self::Device;

    fn update_size(&mut self, width: u16, height: u16);

    fn get_size(&self) -> (u16, u16);

    fn get_aspect_ratio(&self) -> f32;

    fn get_device_transform(&self) -> PixelToDeviceTransform;
}
