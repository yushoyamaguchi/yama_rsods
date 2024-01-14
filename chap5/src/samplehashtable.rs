use std::collections::LinkedList;
use std::hash::{Hash, Hasher};
const INITIAL_SIZE: usize = 1024;


#[derive( Hash, Eq, PartialEq, Clone, Debug)]
struct Entry<K, V> {
    key: K,
    value: V,
}

struct ChainedHashMap<K, V> {
    buckets: Vec<Vec<Entry<K, V>>>,
}

impl<K,V> ChainedHashMap<K,V>
where
    K: Hash + Eq + Clone,
    V: Clone + PartialEq,
{
    fn new() -> ChainedHashMap<K,V>{
        let mut buckets = Vec::with_capacity(INITIAL_SIZE);
        for _ in 0..INITIAL_SIZE {
            buckets.push(Vec::new());
        }
        ChainedHashMap { buckets }
    }
    fn hash(&self, key: &K) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.buckets.len()
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let hash = self.hash(&key);
        let bucket: &mut Vec<Entry<K,V>> = &mut self.buckets[hash];
        for entry in bucket {
            if entry.key == key {
                let old_value = entry.value.clone();
                entry.value = value;
                return Some(old_value);
            }
        }
        None
    }

    fn get(&self, key: &K) -> Option<V> {
        let hash = self.hash(key);
        let bucket: &Vec<Entry<K,V>> = &self.buckets[hash];
        for entry in bucket {
            if entry.key == *key {
                return Some(entry.value.clone());
            }
        }
        None
    }

    fn remove(&mut self, key: &K, value: &V) -> Option<V> {
        let hash = self.hash(key);
        let bucket: &mut Vec<Entry<K,V>> = &mut self.buckets[hash];
        if let Some(index) = bucket.iter().position(|entry| entry.key == *key && entry.value == *value) { //iterator : referrence memo.md
            let removed_value = bucket.remove(index).value;
            return Some(removed_value);
        }
        None
    }
}