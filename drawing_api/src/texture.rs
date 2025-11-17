use core::marker::Sized;

/// Reference counted object.
pub trait Texture: Sized + Sync + Send + Clone {
    /// Safe to call from any thread for any device type (even for OpenGL).
    fn get_size(&self) -> (u16, u16);
}
