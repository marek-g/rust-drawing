#[derive(Clone)]
pub struct ImpellerTexture {
    pub(crate) texture: impellers::Texture,
    pub(crate) size: (u16, u16),
}

impl drawing_api::Texture for ImpellerTexture {
    fn get_size(&self) -> (u16, u16) {
        self.size
    }

    fn get_native_handle(&self) -> usize {
        self.texture.get_opengl_handle() as usize
    }
}
