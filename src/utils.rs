use std::error::Error;
use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Clone)]
/// Struct that counts the number of times an object is added to it
pub struct Counter<Key: Hash + Eq> {
    map: HashMap<Key, usize>,
}

#[allow(dead_code)]
impl<Key: Hash + Eq> Counter<Key> {
    /// Creates a new empty counter
    pub fn new() -> Counter<Key> {
        Counter {
            map: HashMap::new(),
        }
    }
    /// Adds an element to the counter, incrementing the count if it was seen before and setting
    /// the count to 1 if it hasn't
    pub fn add(&mut self, key: Key) {
        if let Some(count) = self.map.get_mut(&key) {
            *count += 1;
        } else {
            self.map.insert(key, 1);
        }
    }
    /// Creates a counter from an iterable, where each element of the iterator will be counted
    pub fn from(collection: impl IntoIterator<Item = Key>) -> Counter<Key> {
        let mut counter = Counter::new();
        for item in collection {
            counter.add(item);
        }
        counter
    }
    /// Returns an iterator over the counts of the Counter
    pub fn counts(&self) -> impl Iterator<Item = &usize> {
        self.map.values()
    }
    /// Returns the count of the provided key, returns 0 if the element was not seen yet
    pub fn get(&self, key: &Key) -> usize {
        *self.map.get(key).unwrap_or(&0)
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
/// Utility function that returns a string of all errors, including their source
pub fn all_errors_string<T: Error + ?Sized>(err: &T) -> String {
    let mut error_string = format!("{err}");
    let mut next = err.source();
    while let Some(e) = next {
        error_string += &format!("\n{e}");
        next = e.source();
    }
    error_string
}
