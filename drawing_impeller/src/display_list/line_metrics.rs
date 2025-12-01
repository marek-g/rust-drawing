pub struct LineMetrics {
    pub(crate) line_metrics: impellers::LineMetrics,
}

impl drawing_api::LineMetrics for LineMetrics {
    fn get_unscaled_ascent(&self, line: usize) -> f64 {
        self.line_metrics.get_unscaled_ascent(line)
    }

    fn get_ascent(&self, line: usize) -> f64 {
        self.line_metrics.get_ascent(line)
    }

    fn get_descent(&self, line: usize) -> f64 {
        self.line_metrics.get_descent(line)
    }

    fn get_baseline(&self, line: usize) -> f64 {
        self.line_metrics.get_baseline(line)
    }

    fn is_hardbreak(&self, line: usize) -> bool {
        self.line_metrics.is_hardbreak(line)
    }

    fn get_width(&self, line: usize) -> f64 {
        self.line_metrics.get_width(line)
    }

    fn get_height(&self, line: usize) -> f64 {
        self.line_metrics.get_height(line)
    }

    fn get_left(&self, line: usize) -> f64 {
        self.line_metrics.get_left(line)
    }

    fn get_code_unit_start_index_utf16(&self, line: usize) -> usize {
        self.line_metrics.get_code_unit_start_index_utf16(line)
    }

    fn get_code_unit_end_index_utf16(&self, line: usize) -> usize {
        self.line_metrics.get_code_unit_end_index_utf16(line)
    }

    fn get_code_unit_end_index_excluding_whitespace_utf16(&self, line: usize) -> usize {
        self.line_metrics
            .get_code_unit_end_index_excluding_whitespace_utf16(line)
    }

    fn get_code_unit_end_index_including_newline_utf16(&self, line: usize) -> usize {
        self.line_metrics
            .get_code_unit_end_index_including_newline_utf16(line)
    }
}
