use units::*;
use color::*;
use backend::Backend;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct FontParams {
    pub size: u8
}

pub trait Font<B: Backend> {
	fn create(backend: &mut B, bytes: Vec<u8>) -> Self;

	fn draw(&mut self, backend: &mut B, target: &B::RenderTarget,
		color: &Color,
		text: &str,
		pos: Point,
		font_params: FontParams,
		transform: UnknownToDeviceTransform);

    fn get_dimensions(&mut self, backend: &mut B, params: FontParams, text: &str) -> (u16, u16);
}
