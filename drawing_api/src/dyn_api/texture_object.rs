use crate::{Texture, TextureDescriptor};

pub trait TextureObject {
    /// Returns descriptor of the texture.
    fn get_descriptor(&self) -> TextureDescriptor;

    /// Gets the native OpenGL handle associated with this texture.
    fn get_gl_handle(&self) -> usize;
}

impl<T: Texture> TextureObject for T {
    fn get_descriptor(&self) -> TextureDescriptor {
        self.get_descriptor()
    }

    fn get_gl_handle(&self) -> usize {
        self.get_gl_handle()
    }
}
