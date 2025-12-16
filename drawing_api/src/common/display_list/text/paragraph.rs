use super::Range;

pub trait Paragraph {
    type GlyphInfo: crate::GlyphInfo;
    type LineMetrics: crate::LineMetrics;

    fn get_max_width(&self) -> f32;

    fn get_height(&self) -> f32;

    fn get_longest_line_width(&self) -> f32;

    fn get_min_intrinsic_width(&self) -> f32;

    fn get_max_intrinsic_width(&self) -> f32;

    fn get_ideographic_baseline(&self) -> f32;

    fn get_alphabetic_baseline(&self) -> f32;

    fn get_line_count(&self) -> u32;

    fn get_line_metrics(&self) -> Option<Self::LineMetrics>;

    fn get_word_boundary_utf16(&self, code_unit_index: usize) -> Range;

    fn create_glyph_info_at_code_unit_index_utf16(
        &self,
        code_unit_index: usize,
    ) -> Option<Self::GlyphInfo>;

    fn create_glyph_info_at_paragraph_coordinates(&self, x: f64, y: f64)
        -> Option<Self::GlyphInfo>;
}
