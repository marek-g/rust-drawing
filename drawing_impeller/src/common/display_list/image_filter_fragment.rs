#[derive(Clone)]
pub struct ImageFilterFragment {
    pub(crate) image_filter: impellers::ImageFilter,
}

impl drawing_api::ImageFilterFragment for ImageFilterFragment {}
