use std::collections::HashMap;
// pub use kv::KvStore;

pub struct KvStore {
    map: HashMap<String, String>
}

impl KvStore {

    pub fn new() -> KvStore {
        panic!();
    }
    pub fn get(&self, _key: String) -> Option<String> {
        panic!();
    }

    pub fn set(&mut self, _key: String, _val: String) {
       panic!()
    }

    pub fn remove(&mut self, _key: String) {
        panic!()
    }
}
