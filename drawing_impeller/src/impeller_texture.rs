#[derive(Clone)]
pub struct ImpellerTexture {
    pub(crate) texture: impellers::Texture,
    pub(crate) descriptor: drawing_api::TextureDescriptor,
}

impl drawing_api::Texture for ImpellerTexture {
    fn get_descriptor(&self) -> drawing_api::TextureDescriptor {
        self.descriptor.clone()
    }

    fn get_gl_handle(&self) -> usize {
        self.texture.get_opengl_handle() as usize
    }
}
