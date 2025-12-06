use super::convert_to_rect;

pub struct Path {
    pub(crate) path: impellers::Path,
}

impl drawing_api::Path for Path {
    fn get_bounds(&self) -> drawing_api::DipRect {
        let bounds = self.path.get_bounds();
        convert_to_rect(&bounds)
    }
}
