pub struct GlyphInfo {}

impl drawing_api::GlyphInfo for GlyphInfo {
    fn get_grapheme_cluster_code_unit_range_begin_utf16(&self) -> usize {
        todo!()
    }

    fn get_grapheme_cluster_code_unit_range_end_utf16(&self) -> usize {
        todo!()
    }

    fn get_grapheme_cluster_bounds(&self) -> drawing_api::DipRect {
        todo!()
    }

    fn is_ellipsis(&self) -> bool {
        todo!()
    }

    fn get_text_direction(&self) -> drawing_api::TextDirection {
        todo!()
    }
}
