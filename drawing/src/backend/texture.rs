use anyhow::Result;
use core::marker::Sized;

pub trait Texture: Sized {
    /// Safe to call from any thread for any device type (even for OpenGL).
    fn get_size(&self) -> (u16, u16);

    // Not safe to call from any thread for some device types (like OpenGL).
    fn update(
        &mut self,
        memory: &[u8],
        offset_x: u16,
        offset_y: u16,
        width: u16,
        height: u16,
    ) -> Result<()>;
}
