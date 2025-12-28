use crate::generic::device::RenderTarget;
use crate::generic::renderer::Renderer;
use crate::units::PixelToDeviceTransform;
use crate::GlContext;
use drawing_api::{euclid::Vector2D, ColorFormat};
use gl::types::GLuint;

pub struct GlSurface {
    pub(crate) context: GlContext,

    pub(crate) framebuffer_id: GLuint,
    pub(crate) width: u16,
    pub(crate) height: u16,
    pub(crate) color_format: ColorFormat,
    pub(crate) is_owner: bool,
}

impl Drop for GlSurface {
    fn drop(&mut self) {
        if self.is_owner {
            unsafe {
                gl::DeleteFramebuffers(1, &self.framebuffer_id);
            }
        }
    }
}

impl RenderTarget for GlSurface {
    type Device = crate::GlContext;

    fn get_device(&self) -> Self::Device {
        self.context.clone()
    }

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
        PixelToDeviceTransform::identity()
            .then_scale(2.0f32 / self.width as f32, -2.0f32 / self.height as f32)
            .then_translate(Vector2D::new(-1.0f32, 1.0f32))
    }
}

impl drawing_api::Surface for GlSurface {
    type DisplayList = crate::display_list::DisplayList;

    fn draw(&mut self, display_list: &Self::DisplayList) -> Result<(), &'static str> {
        let mut renderer = Renderer::new();
        renderer.draw::<GlContext>(&self, &display_list.display_list, true)?;
        Ok(())
    }

    fn present(self) -> Result<(), &'static str> {
        Ok(())
    }
}
