use std::os::raw::c_void;

use crate::{ColorFormat, DrawingContext, TextureDescriptor};

pub trait ContextGl: DrawingContext + Clone + 'static {
    /// Creates an OpenGL context.
    unsafe fn new_gl<F>(loadfn: F) -> Result<Self, &'static str>
    where
        F: FnMut(&str) -> *mut c_void;

    /// Creates a new surface by wrapping an existing OpenGL framebuffer object.
    unsafe fn wrap_gl_framebuffer(
        &mut self,
        framebuffer_id: u32,
        width: u32,
        height: u32,
        color_format: ColorFormat,
    ) -> Result<Self::Surface, &'static str>;

    /// Creates a texture with an externally created OpenGL texture handle.
    unsafe fn adopt_gl_texture(
        &self,
        texture_handle: u32,
        descriptor: TextureDescriptor,
    ) -> Result<Self::Texture, &'static str>;
}
