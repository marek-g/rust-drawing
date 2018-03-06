use units::*;
use color::*;

pub struct FontParams {
    pub size: UserPixelThickness
}

pub trait Font {
    fn add_text(&mut self,
		color: &Color,
		text: &str,
		pos: Point,
		transform: UnknownToDeviceTransform);

    fn get_dimensions(&mut self,  text: &str) -> (u16, u16);
}