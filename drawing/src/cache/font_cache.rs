use font::FontParams;
use std::collections::HashMap;

pub struct FontCache<'a, F> {
    hash_map: HashMap<(&'a str, FontParams), F>
}

impl<'a, F> FontCache<'a, F> {
    pub fn new() -> FontCache<'a, F> {
        FontCache {
            hash_map: HashMap::new()
        }
    }

    pub fn insert(&mut self, key: &'a str, params: FontParams, font: F) {
        self.hash_map.insert((key, params), font);
    }

    pub fn get(&self, key: &'a str, params: FontParams) -> Option<&F> {
        self.hash_map.get(&(key, params))
    }
}