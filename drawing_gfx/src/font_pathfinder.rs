extern crate drawing;

use self::drawing::font::*;
use self::drawing::backend::Font;

#[derive(Clone, Debug, PartialEq)]
pub struct PathfinderFont {
}

impl self::drawing::backend::Font for PathfinderFont {
    fn create(bytes: &[u8]) -> Self {
        PathfinderFont {}
    }

	fn get_dimensions(&mut self, font_params: FontParams, text: &str) -> (u16, u16) {
        (10, 10)
    }
}
