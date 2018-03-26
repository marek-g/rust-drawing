use backend::Texture;
use cache::*;

pub struct Resources<T: Texture, Font> {
    fonts: FontCache<Font>,
    textures: IdCache<T> 
}

impl<'a, T: Texture, Font> Resources<T, Font> {
    pub fn new() -> Resources<T, Font> {
        Resources {
            fonts: FontCache::new(),
            textures: IdCache::new()
        }
    }

    pub fn fonts(&self) -> &FontCache<Font> {
        &self.fonts
    }

    pub fn fonts_mut(&mut self) -> &mut FontCache<Font> {
        &mut self.fonts
    }

    pub fn textures(&self) -> &IdCache<T> {
        &self.textures
    }

    pub fn textures_mut(&mut self) -> &mut IdCache<T> {
        &mut self.textures
    }
}