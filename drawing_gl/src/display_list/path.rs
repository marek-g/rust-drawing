use drawing_api::FillType;

use super::PathElement;

pub struct Path {
    pub(crate) path: Vec<PathElement>,
    pub(crate) fill_type: FillType,
}

impl drawing_api::Path for Path {
    fn get_bounds(&self) -> drawing_api::PixelRect {
        todo!()
    }
}
