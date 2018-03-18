use units::*;
use color::*;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct FontParams {
    pub size: u16
}

pub trait Font {
    fn add_text(&mut self,
		color: &Color,
		text: &str,
		pos: Point,
		transform: UnknownToDeviceTransform);

    fn get_dimensions(&mut self,  text: &str) -> (u16, u16);
}