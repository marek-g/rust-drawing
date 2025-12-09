use crate::{GlContext, GlTexture};

use super::Primitive;

#[derive(Default)]
pub struct Paragraph {
    pub(crate) primitives: Vec<Primitive<GlTexture, crate::Fonts<GlContext>>>,
}

impl drawing_api::Paragraph for Paragraph {
    type GlyphInfo = crate::display_list::GlyphInfo;

    type LineMetrics = crate::display_list::LineMetrics;

    fn get_max_width(&self) -> f32 {
        todo!()
    }

    fn get_height(&self) -> f32 {
        todo!()
    }

    fn get_longest_line_width(&self) -> f32 {
        todo!()
    }

    fn get_min_intrinsic_width(&self) -> f32 {
        todo!()
    }

    fn get_max_intrinsic_width(&self) -> f32 {
        todo!()
    }

    fn get_ideographic_baseline(&self) -> f32 {
        todo!()
    }

    fn get_alphabetic_baseline(&self) -> f32 {
        todo!()
    }

    fn get_line_count(&self) -> u32 {
        todo!()
    }

    fn get_line_metrics(&self) -> Option<Self::LineMetrics> {
        todo!()
    }

    fn get_word_boundary_utf16(&self, code_unit_index: usize) -> drawing_api::Range {
        todo!()
    }

    fn create_glyph_info_at_code_unit_index_utf16(
        &self,
        code_unit_index: usize,
    ) -> Option<Self::GlyphInfo> {
        todo!()
    }

    fn create_glyph_info_at_paragraph_coordinates(
        &self,
        x: f64,
        y: f64,
    ) -> Option<Self::GlyphInfo> {
        todo!()
    }
}
