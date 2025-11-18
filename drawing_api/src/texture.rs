use core::marker::Sized;

/// Represents an image whose data is resident in GPU memory.
/// Reference counted, thread safe, immutable object.
pub trait Texture: Sized + Sync + Send + Clone {
    /// Safe to call from any thread for any device type (even for OpenGL).
    fn get_size(&self) -> (u16, u16);

    fn get_native_handle(&self) -> usize;
}
