use std::collections::HashMap;

pub struct FontCache<F> {
    hash_map: HashMap<String, F>
}

impl<F> FontCache<F> {
    pub fn new() -> FontCache<F> {
        FontCache {
            hash_map: HashMap::new()
        }
    }

    pub fn insert(&mut self, key: String, font: F) {
        self.hash_map.insert(key, font);
    }

    pub fn get_mut(&mut self, key: &String) -> Option<&mut F> {
        self.hash_map.get_mut(key)
    }
}