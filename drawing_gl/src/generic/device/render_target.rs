use core::marker::Sized;
use drawing_api::PixelToDeviceTransform;

pub trait RenderTarget: Sized {
    fn update_size(&mut self, width: u16, height: u16);

    fn get_size(&self) -> (u16, u16);

    fn get_aspect_ratio(&self) -> f32;

    fn get_device_transform(&self) -> PixelToDeviceTransform;
}
