#[derive(Clone)]
pub struct ImpellerTexture {}

impl drawing_api::Texture for ImpellerTexture {
    fn get_size(&self) -> (u16, u16) {
        todo!()
    }

    fn get_native_handle(&self) -> usize {
        todo!()
    }
}
