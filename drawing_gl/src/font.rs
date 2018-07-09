extern crate drawing;

use self::drawing::font::*;
use self::drawing::color::*;
use self::drawing::units::*;

use ::backend::GlWindowBackend;

pub struct GlFont { }


impl Font<GlWindowBackend> for GlFont {
    fn create(_backend: &mut GlWindowBackend, bytes: Vec<u8>) -> Self {
        GlFont {
        }
    }

    fn draw(&mut self, backend: &mut GlWindowBackend, target: &(),
		color: &Color,
		text: &str,
		pos: Point,
		font_params: FontParams,
		transform: UnknownToDeviceTransform) {
    }

    fn get_dimensions(&mut self, backend: &mut GlWindowBackend, params: FontParams, text: &str) -> (u16, u16) {
        (10 as u16, 10 as u16)
    }
}
