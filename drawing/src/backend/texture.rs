use anyhow::Result;
use core::marker::Sized;

pub trait Texture: Sized {
    fn get_size(&self) -> (u16, u16);

    fn update(
        &mut self,
        memory: &[u8],
        offset_x: u16,
        offset_y: u16,
        width: u16,
        height: u16,
    ) -> Result<()>;
}
