use crate::parse::{self, Command};
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

    fn insert(&mut self, key: &[u8], value: &[u8]) {
        // into possibly bad?
        self.data.insert(key.into(), Arc::from(value));
    }

    /// Tries to execute the given command.
    /// Returns `None` if the byte sequence does not represent a valid command.
    pub fn exec_command(&mut self, command: &[u8]) -> Option<Reponse> {
        let res = parse::parse_command(command)?;
        dbg!(&res);

        Some(match res {
            Command::Get(key) => self.exec_get(key),
            Command::Set(key, value) => self.exec_set(key, value),
        })
    }

    fn exec_get(&self, key: &[u8]) -> Reponse {
        match self.lookup(key) {
            Some(val) => Reponse::Value(val),
            None => Reponse::Err,
        }
    }

    fn exec_set(&mut self, key: &[u8], value: &[u8]) -> Reponse {
        self.insert(key, value);

        Reponse::Ok
    }
}

impl Default for KVStore {
    fn default() -> Self {
        Self::new()
    }
}
