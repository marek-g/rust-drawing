use crate::PixelRect;

use super::TextDirection;

pub trait GlyphInfo: 'static {
    fn get_grapheme_cluster_code_unit_range_begin_utf16(&self) -> usize;

    fn get_grapheme_cluster_code_unit_range_end_utf16(&self) -> usize;

    fn get_grapheme_cluster_bounds(&self) -> PixelRect;

    fn is_ellipsis(&self) -> bool;

    fn get_text_direction(&self) -> TextDirection;
}
