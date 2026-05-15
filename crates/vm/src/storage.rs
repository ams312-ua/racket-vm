use common::value::{GCValue, Value};
use rapidhash::{HashMapExt, RapidHashMap};

/// Storage for values in the VM, used to store value variables
#[derive(Debug, Clone)]
pub struct ValueStorage {
    pub values: RapidHashMap<Box<str>, GCValue>,
}

impl ValueStorage {
    pub fn new() -> Self {
        Self {
            values: RapidHashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            values: RapidHashMap::with_capacity(capacity),
        }
    }

    pub fn get(&self, key: &str) -> Option<GCValue> {
        self.values.get(key).cloned()
    }

    pub fn insert(&mut self, key: Box<str>, value: GCValue) {
        self.values.insert(key, value);
    }

    pub fn remove(&mut self, key: &str) {
        self.values.remove(key);
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}
