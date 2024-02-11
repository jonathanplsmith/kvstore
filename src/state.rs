use std::{collections::HashMap, sync::Arc};

pub struct RuntimeState {
    data: HashMap<Box<[u8]>, Arc<[u8]>>,
}

impl RuntimeState {
    pub fn new() -> Self {
        RuntimeState {
            data: HashMap::new(),
        }
    }

    pub fn get(&self, key: &[u8]) -> Option<Arc<[u8]>> {
        self.data.get(key).cloned()
    }

    pub fn set(&mut self, key: &[u8], value: &[u8]) {
        // into possibly bad?
        self.data.insert(key.into(), Arc::from(value));
    }
}

impl Default for RuntimeState {
    fn default() -> Self {
        Self::new()
    }
}
