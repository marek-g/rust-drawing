use backend::Texture;
use font::FontParams;
use cache::*;

use std::collections::HashMap;

pub struct Resources<'a, T: Texture, Font> {
    fonts: FontCache<'a, Font>,
    textures: IdCache<T> 
}

impl<'a, T: Texture, Font> Resources<'a, T, Font> {
    pub fn new() -> Resources<'a, T, Font> {
        Resources {
            fonts: FontCache::new(),
            textures: IdCache::new()
        }
    }
}