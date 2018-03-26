use backend::Texture;
use std::collections::HashMap;

pub struct Resources<T: Texture, Font> {
    fonts: HashMap<String, Font>,
    textures: HashMap<i32, T> 
}

impl<T: Texture, Font> Resources<T, Font> {
    pub fn new() -> Resources<T, Font> {
        Resources {
            fonts: HashMap::new(),
            textures: HashMap::new()
        }
    }

    pub fn fonts_mut(&mut self) -> &mut HashMap<String, Font> {
        &mut self.fonts
    }

    pub fn textures_mut(&mut self) -> &mut HashMap<i32, T> {
        &mut self.textures
    }
}