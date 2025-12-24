use core::marker::Sized;

use crate::ColorFormat;

/// Represents an image whose data is resident in GPU memory.
/// Reference counted, thread safe, immutable object.
pub trait Texture: Sized + Sync + Send + Clone + 'static {
    /// Returns descriptor of the texture.
    fn get_descriptor(&self) -> TextureDescriptor;

    /// Gets the native OpenGL handle associated with this texture.
    fn get_gl_handle(&self) -> usize;
}

#[derive(Clone)]
pub struct TextureDescriptor {
    pub width: u32,
    pub height: u32,
    pub color_format: ColorFormat,
    pub mip_count: u32,
}

impl Default for TextureDescriptor {
    fn default() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
            color_format: Default::default(),
            mip_count: Default::default(),
        }
    }
}
