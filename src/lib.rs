use std::collections::HashMap;

pub struct KvStore {
    storage: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        KvStore {
            storage: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.storage.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        match self.storage.get(&key) {
            Some(val) => Some(val.clone()),
            None => None,
        }
    }

    pub fn remove(&mut self, key: String) {
        self.storage.remove(&key);
    }
}
