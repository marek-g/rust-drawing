use drawing_api::Context;
use std::ffi::c_void;

use crate::{generic::device::Device, GlContextData, GlDevice, GlRenderTarget, GlSurface};

pub struct GlContext {
    gl_device: GlDevice,
    gl_context_data: GlContextData,
}

impl Context for GlContext {
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

impl GlContext {
    pub fn new_gl_context<F>(loadfn: F) -> Result<Self, &'static str>
    where
        F: FnMut(&'static str) -> *const c_void,
    {
        let mut gl_device = GlDevice::new().unwrap();
        let gl_context_data = gl_device.init_context(loadfn);

        Ok(Self {
            gl_device,
            gl_context_data,
        })
    }

    pub fn draw(&mut self) {
        let render_target = GlRenderTarget::new(0, 100u16, 100u16, 1.0f32);

        //self.gl_device.begin(&self.gl_context_data).unwrap();

        self.gl_device.clear(
            //window_target.get_render_target(),
            &render_target,
            &[0.5f32, 0.4f32, 0.3f32, 1.0f32],
        );
    }
}
