extern crate drawing;
extern crate app_units;
extern crate pathfinder_path_utils;
extern crate pathfinder_font_renderer;

use self::drawing::font::*;
use self::drawing::backend::Font;

use self::app_units::Au;
use self::pathfinder_font_renderer::{FontContext, FontInstance, FontKey, GlyphDimensions, GlyphKey, SubpixelOffset};

use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct PathfinderFont {
}

impl self::drawing::backend::Font for PathfinderFont {
    fn create(bytes: &[u8]) -> Self {
        let mut font_context = FontContext::new().unwrap();
        let font_key = FontKey::new();
        font_context.add_font_from_memory(&font_key, Arc::new(bytes.to_vec()), 0).unwrap();

        let font_instance = FontInstance::new(&font_key, Au(60 * 16)); // TEST_FONT_SIZE
        let glyph_key = GlyphKey::new(68, SubpixelOffset(0)); // 'a'
        let glyph_outline = font_context.glyph_outline(&font_instance, &glyph_key).unwrap();
        //let mut glyph_outline_buffer = PathBuffer::new();
        //glyph_outline_buffer.add_stream(glyph_outline);

        //info!("endpoints: {:#?}", glyph_outline_buffer.endpoints);
        //info!("control points: {:#?}", glyph_outline_buffer.control_points);

        PathfinderFont {}
    }

	fn get_dimensions(&mut self, font_params: FontParams, text: &str) -> (u16, u16) {
        (10, 10)
    }
}
