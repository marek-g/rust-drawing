use core::marker::Sized;

/// Represents an image whose data is resident in GPU memory.
/// Reference counted, thread safe, immutable object.
pub trait Texture: Sized + Sync + Send + Clone {
    /// Returns size of the texture.
    fn get_size(&self) -> (u16, u16);

    /// Gets the native (OpenGL) handle associated with this texture.
    fn get_native_handle(&self) -> usize;
}
