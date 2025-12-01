use crate::{GlContextData, GlTexture};

use super::Primitive;

#[derive(Default)]
pub struct Paragraph {
    pub(crate) primitives: Vec<Primitive<GlTexture, crate::Fonts<GlContextData>>>,
}

impl drawing_api::Paragraph for Paragraph {}
