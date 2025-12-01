pub struct LineMetrics {
    pub(crate) line_metrics: impellers::LineMetrics,
}

impl drawing_api::LineMetrics for LineMetrics {
    fn get_unscaled_ascent(&self, line: usize) -> f64 {
        self.get_unscaled_ascent(line)
    }

    fn get_ascent(&self, line: usize) -> f64 {
        self.get_ascent(line)
    }

    fn get_descent(&self, line: usize) -> f64 {
        self.get_descent(line)
    }

    fn get_baseline(&self, line: usize) -> f64 {
        self.get_baseline(line)
    }

    fn is_hardbreak(&self, line: usize) -> bool {
        self.is_hardbreak(line)
    }

    fn get_width(&self, line: usize) -> f64 {
        self.get_width(line)
    }

    fn get_height(&self, line: usize) -> f64 {
        self.get_height(line)
    }

    fn get_left(&self, line: usize) -> f64 {
        self.get_left(line)
    }

    fn get_code_unit_start_index_utf16(&self, line: usize) -> usize {
        self.get_code_unit_start_index_utf16(line)
    }

    fn get_code_unit_end_index_utf16(&self, line: usize) -> usize {
        self.get_code_unit_end_index_utf16(line)
    }

    fn get_code_unit_end_index_excluding_whitespace_utf16(&self, line: usize) -> usize {
        self.get_code_unit_end_index_excluding_whitespace_utf16(line)
    }

    fn get_code_unit_end_index_including_newline_utf16(&self, line: usize) -> usize {
        self.get_code_unit_end_index_including_newline_utf16(line)
    }
}
