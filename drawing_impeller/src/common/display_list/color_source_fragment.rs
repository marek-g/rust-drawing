#[derive(Clone)]
pub struct ColorSourceFragment {
    pub(crate) color_source: impellers::ColorSource,
}

impl drawing_api::ColorSourceFragment for ColorSourceFragment {}
