use units::*;
use color::*;
use backend::Device;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct FontParams {
    pub size: u8
}

pub trait Font<D: Device> {
	fn create(device: &mut D, bytes: Vec<u8>) -> Self;

	fn draw(&mut self, device: &mut D, target: &D::RenderTarget,
		color: &Color,
		text: &str,
		pos: Point,
		font_params: FontParams,
		transform: UnknownToDeviceTransform);

    fn get_dimensions(&mut self, device: &mut D, params: FontParams, text: &str) -> (u16, u16);
}
