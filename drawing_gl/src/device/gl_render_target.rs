use crate::generic::device::RenderTarget;
use drawing_api::PixelToDeviceTransform;
use gl::types::*;

pub struct GlRenderTarget {
    pub(crate) framebuffer_id: GLuint,
    pub(crate) width: u16,
    pub(crate) height: u16,
    pub(crate) aspect_ratio: f32,
}

impl GlRenderTarget {
    pub fn new(framebuffer_id: GLuint, width: u16, height: u16, aspect_ratio: f32) -> Self {
        Self {
            framebuffer_id,
            width,
            height,
            aspect_ratio,
        }
    }
}

impl Drop for GlRenderTarget {
    fn drop(&mut self) {
        if self.framebuffer_id > 0 {
            unsafe {
                gl::DeleteFramebuffers(1, &self.framebuffer_id);
            }
        }
    }
}

impl RenderTarget for GlRenderTarget {
    fn update_size(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    fn get_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    fn get_aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    fn get_device_transform(&self) -> PixelToDeviceTransform {
        PixelToDeviceTransform::new(
            2.0f32 / self.width as f32,
            0.0f32,
            0.0f32,
            -2.0f32 / self.height as f32,
            -1.0f32,
            1.0f32,
        )
    }
}
