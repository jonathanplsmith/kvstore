use crate::parse::Command;
use std::{collections::HashMap, sync::Arc};

#[derive(PartialEq, Eq, Debug)]
pub enum Reponse {
    Ok,
    Err,
    Value(Arc<[u8]>),
}

impl Reponse {
    pub fn unwrap(self) -> Arc<[u8]> {
        if let Self::Value(v) = self {
            v
        } else {
            panic!("Cannot unwrap Ok or Err!");
        }
    }
}

pub struct KVStore {
    data: HashMap<Box<[u8]>, Arc<[u8]>>,
}

impl KVStore {
    pub fn new() -> Self {
        KVStore {
            data: HashMap::new(),
        }
    }

    fn lookup(&self, key: &[u8]) -> Option<Arc<[u8]>> {
        self.data.get(key).cloned()
    }

    fn insert(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.data.insert(key.into(), value.into());
    }

    pub fn exec_command(&mut self, command: Command) -> Reponse {
        match command {
            Command::Get(key) => self.exec_get(key),
            Command::Set(key, value) => self.exec_set(key, value),
            Command::Delete(key) => self.exec_delete(key),
            Command::Clear => self.exec_clear(),
        }
    }

    fn exec_get(&self, key: Vec<u8>) -> Reponse {
        match self.lookup(&key) {
            Some(val) => Reponse::Value(val),
            None => Reponse::Err,
        }
    }

    fn exec_set(&mut self, key: Vec<u8>, value: Vec<u8>) -> Reponse {
        self.insert(key, value);

        Reponse::Ok
    }

    fn exec_delete(&mut self, key: Vec<u8>) -> Reponse {
        match self.data.remove(key.as_slice()) {
            Some(_) => Reponse::Ok,
            None => Reponse::Err,
        }
    }

    fn exec_clear(&mut self) -> Reponse {
        self.data.clear();

        Reponse::Ok
    }
}

impl Default for KVStore {
    fn default() -> Self {
        Self::new()
    }
}
