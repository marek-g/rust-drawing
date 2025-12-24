pub trait LineMetrics: 'static {
    fn get_unscaled_ascent(&self, line: usize) -> f64;

    fn get_ascent(&self, line: usize) -> f64;

    fn get_descent(&self, line: usize) -> f64;

    fn get_baseline(&self, line: usize) -> f64;

    fn is_hardbreak(&self, line: usize) -> bool;

    fn get_width(&self, line: usize) -> f64;

    fn get_height(&self, line: usize) -> f64;

    fn get_left(&self, line: usize) -> f64;

    fn get_code_unit_start_index_utf16(&self, line: usize) -> usize;

    fn get_code_unit_end_index_utf16(&self, line: usize) -> usize;

    fn get_code_unit_end_index_excluding_whitespace_utf16(&self, line: usize) -> usize;

    fn get_code_unit_end_index_including_newline_utf16(&self, line: usize) -> usize;
}
