use backend::Texture;
use std::collections::HashMap;

pub struct Resources<T: Texture, Font> {
    fonts: HashMap<String, Font>,
    textures: HashMap<i32, T>,
    last_texture_id: i32
}

impl<T: Texture, Font> Resources<T, Font> {
    pub fn new() -> Resources<T, Font> {
        Resources {
            fonts: HashMap::new(),
            textures: HashMap::new(),
            last_texture_id: 0
        }
    }

    pub fn fonts_mut(&mut self) -> &mut HashMap<String, Font> {
        &mut self.fonts
    }

    pub fn get_next_texture_id(&mut self) -> i32 {
        self.last_texture_id += 1;
        self.last_texture_id
    }

    pub fn textures(&self) -> &HashMap<i32, T> {
        &self.textures
    }

    pub fn textures_mut(&mut self) -> &mut HashMap<i32, T> {
        &mut self.textures
    }
}