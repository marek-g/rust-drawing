use crate::generic::device::RenderTarget;
use drawing_api::{ColorFormat, PixelToDeviceTransform};
use gl::types::GLuint;

pub struct GlSurface {
    pub(crate) framebuffer_id: GLuint,
    pub(crate) width: u16,
    pub(crate) height: u16,
    pub(crate) color_format: ColorFormat,
    pub(crate) is_owner: bool,
}

/*impl Drop for GlSurface {
    fn drop(&mut self) {
        if self.is_owner {
            unsafe {
                gl::DeleteFramebuffers(1, &self.framebuffer_id);
            }
        }
    }
}*/

impl RenderTarget for GlSurface {
    fn update_size(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    fn get_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    fn get_aspect_ratio(&self) -> f32 {
        1.0f32
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

impl drawing_api::Surface for GlSurface {
    type DisplayList = Vec<crate::generic::renderer::Primitive>;

    fn draw(&self, display_list: &Self::DisplayList) -> Result<(), &'static str> {
        //todo!()
        Ok(())
    }
}
