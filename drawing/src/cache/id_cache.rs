use std::collections::HashMap;

pub struct IdCache<T> {
    hash_map: HashMap<i32, T>,
    last_id: i32
}

impl<T> IdCache<T> {
    pub fn new() -> IdCache<T> {
        IdCache {
            hash_map: HashMap::new(),
            last_id: 0
        }
    }
}