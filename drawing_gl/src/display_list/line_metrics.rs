pub struct LineMetrics {}

impl drawing_api::LineMetrics for LineMetrics {
    fn get_unscaled_ascent(&self, line: usize) -> f64 {
        todo!()
    }

    fn get_ascent(&self, line: usize) -> f64 {
        todo!()
    }

    fn get_descent(&self, line: usize) -> f64 {
        todo!()
    }

    fn get_baseline(&self, line: usize) -> f64 {
        todo!()
    }

    fn is_hardbreak(&self, line: usize) -> bool {
        todo!()
    }

    fn get_width(&self, line: usize) -> f64 {
        todo!()
    }

    fn get_height(&self, line: usize) -> f64 {
        todo!()
    }

    fn get_left(&self, line: usize) -> f64 {
        todo!()
    }

    fn get_code_unit_start_index_utf16(&self, line: usize) -> usize {
        todo!()
    }

    fn get_code_unit_end_index_utf16(&self, line: usize) -> usize {
        todo!()
    }

    fn get_code_unit_end_index_excluding_whitespace_utf16(&self, line: usize) -> usize {
        todo!()
    }

    fn get_code_unit_end_index_including_newline_utf16(&self, line: usize) -> usize {
        todo!()
    }
}
