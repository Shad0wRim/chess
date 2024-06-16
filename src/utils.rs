use std::error::Error;
use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Clone)]
pub struct Counter<Key: Hash + Eq> {
    map: HashMap<Key, usize>,
}

#[allow(dead_code)]
impl<Key: Hash + Eq> Counter<Key> {
    pub fn new() -> Counter<Key> {
        Counter {
            map: HashMap::new(),
        }
    }
    pub fn add(&mut self, key: Key) {
        if let Some(count) = self.map.get_mut(&key) {
            *count += 1;
        } else {
            self.map.insert(key, 1);
        }
    }
    pub fn from(collection: impl IntoIterator<Item = Key>) -> Counter<Key> {
        let mut counter = Counter::new();
        for item in collection {
            counter.add(item);
        }
        counter
    }
    pub fn counts(&self) -> impl Iterator<Item = &usize> {
        self.map.values()
    }
    pub fn get(&self, key: &Key) -> Option<&usize> {
        self.map.get(key)
    }
}

impl<Key: Hash + Eq> Default for Counter<Key> {
    fn default() -> Self {
        Counter::new()
    }
}

/// Utility function that prints out errors, including their source
pub fn print_all_errors<T: Error + ?Sized>(err: &T) {
    println!("{}", err);
    let mut next = err.source();
    while let Some(e) = next {
        println!("{}", e);
        next = e.source();
    }
}
