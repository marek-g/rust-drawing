extern crate drawing;
extern crate gfx_text;

use self::drawing::font::*;
use self::drawing::backend::Font;

#[derive(Clone, Debug, PartialEq)]
pub struct GfxTextFont {
}

impl self::drawing::backend::Font for GfxTextFont {
    fn create(bytes: &[u8]) -> Self {
        GfxTextFont {}
    }

    fn get_dimensions(&mut self, font_params: FontParams, text: &str) -> (u16, u16) {
        (10, 10)
    }
}
