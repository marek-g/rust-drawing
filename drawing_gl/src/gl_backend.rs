use drawing_api::Backend;
use std::ffi::c_void;

use crate::{GlContextData, GlSurface};

pub struct GlBackend {
    gl_context_data: GlContextData,
}

impl Backend for GlBackend {
    type DisplayListBuilder = crate::generic::renderer::DisplayListBuilder;

    type Paint = crate::generic::device::Paint;

    type Surface = GlSurface;

    fn create_display_list_builder(&self) -> Result<Self::DisplayListBuilder, &'static str> {
        todo!()
    }

    fn create_paint(&self) -> Result<Self::Paint, &'static str> {
        todo!()
    }
}

impl GlBackend {
    pub fn new_gl_context<F>(&mut self, loadfn: F) -> Result<Self, &'static str>
    where
        F: FnMut(&'static str) -> *const c_void,
    {
	// tell gl crate how to forward gl function calls to the driver
        gl::load_with(loadfn);
	
        Ok(GlBackend {})
    }
}
