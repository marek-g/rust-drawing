use crate::{ColorFormat, ContextGl, TextureDescriptor};

use super::{DrawingContextObject, SurfaceObject, TextureObject};

pub trait ContextGlObject: DrawingContextObject {
    /// Creates a new surface by wrapping an existing OpenGL framebuffer object.
    unsafe fn wrap_gl_framebuffer(
        &mut self,
        framebuffer_id: u32,
        width: u32,
        height: u32,
        color_format: ColorFormat,
    ) -> Result<Box<dyn SurfaceObject>, &'static str>;

    /// Creates a texture with an externally created OpenGL texture handle.
    unsafe fn adopt_gl_texture(
        &self,
        texture_handle: u32,
        descriptor: TextureDescriptor,
    ) -> Result<Box<dyn TextureObject>, &'static str>;
}

impl<C: ContextGl> ContextGlObject for C {
    unsafe fn wrap_gl_framebuffer(
        &mut self,
        framebuffer_id: u32,
        width: u32,
        height: u32,
        color_format: ColorFormat,
    ) -> Result<Box<dyn SurfaceObject>, &'static str> {
        unsafe {
            Ok(Box::new(self.wrap_gl_framebuffer(
                framebuffer_id,
                width,
                height,
                color_format,
            )?))
        }
    }

    unsafe fn adopt_gl_texture(
        &self,
        texture_handle: u32,
        descriptor: TextureDescriptor,
    ) -> Result<Box<dyn TextureObject>, &'static str> {
        unsafe { Ok(Box::new(self.adopt_gl_texture(texture_handle, descriptor)?)) }
    }
}
