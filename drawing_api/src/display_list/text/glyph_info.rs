use crate::DipRect;

use super::TextDirection;

pub trait GlyphInfo {
    fn get_grapheme_cluster_code_unit_range_begin_utf16(&self) -> usize;

    fn get_grapheme_cluster_code_unit_range_end_utf16(&self) -> usize;

    fn get_grapheme_cluster_bounds(&self) -> DipRect;

    fn is_ellipsis(&self) -> bool;

    fn get_text_direction(&self) -> TextDirection;
}
