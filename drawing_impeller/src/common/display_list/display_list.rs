#[derive(Clone)]
pub struct DisplayList {
    pub(crate) display_list: impellers::DisplayList,
}

impl drawing_api::DisplayList for DisplayList {}
