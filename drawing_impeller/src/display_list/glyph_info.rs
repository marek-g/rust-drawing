use drawing_api::{PixelPoint, PixelSize};

pub struct GlyphInfo {
    pub(crate) glyph_info: impellers::GlyphInfo,
}

impl drawing_api::GlyphInfo for GlyphInfo {
    fn get_grapheme_cluster_code_unit_range_begin_utf16(&self) -> usize {
        self.glyph_info
            .get_grapheme_cluster_code_unit_range_begin_utf16()
    }

    fn get_grapheme_cluster_code_unit_range_end_utf16(&self) -> usize {
        self.glyph_info
            .get_grapheme_cluster_code_unit_range_end_utf16()
    }

    fn get_grapheme_cluster_bounds(&self) -> drawing_api::PixelRect {
        let rect = self.glyph_info.get_grapheme_cluster_bounds();
        drawing_api::PixelRect::new(
            PixelPoint::new(rect.origin.x, rect.origin.y),
            PixelSize::new(rect.size.width, rect.size.height),
        )
    }

    fn is_ellipsis(&self) -> bool {
        self.glyph_info.is_ellipsis()
    }

    fn get_text_direction(&self) -> drawing_api::TextDirection {
        match self.glyph_info.get_text_direction() {
            impellers::TextDirection::RTL => drawing_api::TextDirection::RTL,
            impellers::TextDirection::LTR => drawing_api::TextDirection::LTR,
        }
    }
}
