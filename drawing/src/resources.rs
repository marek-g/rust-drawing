use font::Font;
use backend::Device;
use std::collections::HashMap;

pub struct Resources<D: Device, F: Font<D>> {
    fonts: HashMap<String, F>,
    textures: HashMap<i32, D::Texture>,
    last_texture_id: i32,
}

impl<D: Device, F: Font<D>> Resources<D, F> {
    pub fn new() -> Resources<D, F> {
        Resources {
            fonts: HashMap::new(),
            textures: HashMap::new(),
            last_texture_id: 0
        }
    }

    pub fn fonts_mut(&mut self) -> &mut HashMap<String, F> {
        &mut self.fonts
    }

    pub fn get_next_texture_id(&mut self) -> i32 {
        self.last_texture_id += 1;
        self.last_texture_id
    }

    pub fn textures(&self) -> &HashMap<i32, D::Texture> {
        &self.textures
    }

    pub fn textures_mut(&mut self) -> &mut HashMap<i32, D::Texture> {
        &mut self.textures
    }
}