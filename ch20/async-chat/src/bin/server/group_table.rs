use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::group::Group;

pub struct GroupTable(Mutex<HashMap<Arc<String>, Arc<Group>>>);

impl GroupTable {
    pub fn new() -> Self {
        GroupTable(Mutex::new(HashMap::new()))
    }

    pub fn get(&self, name: &String) -> Option<Arc<Group>> {
        // self.0.lock().unwrap().get(name).cloned()
        let map_guard = self.0.lock().unwrap();
        let group = map_guard.get(name);
        group.cloned()
    }

    pub fn get_or_create(&self, name: Arc<String>) -> Arc<Group> {
        let mut map_guard = self.0.lock().unwrap();
        let group = map_guard
            .entry(name.clone())
            .or_insert_with(|| Arc::new(Group::new(name)));
        group.clone()
    }
}
