use crate::{GlContext, GlTexture};

use super::Primitive;

#[derive(Clone)]
pub struct DisplayList {
    pub(crate) display_list: Vec<Primitive<GlTexture, crate::Fonts<GlContext>>>,
}

impl DisplayList {
    pub(crate) fn new() -> Self {
        DisplayList {
            display_list: Vec::new(),
        }
    }
}

impl drawing_api::DisplayList for DisplayList {}
