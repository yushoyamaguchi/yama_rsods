use std::collections::LinkedList;
const INITIAL_SIZE: usize = 128;


#[derive( Eq, PartialEq, Clone, Debug)]
struct Entry<K, V> {
    key: K,
    value: V,
}

struct ChainedHashMap<K, V> {
    buckets: Vec<LinkedList<Entry<K, V>>>,
}

impl<K,V> ChainedHashMap<K,V>
where
    K: Eq,
    V: Clone,
{
    fn new() -> ChainedHashMap<K,V>{
        let mut buckets = Vec::with_capacity(INITIAL_SIZE);
        for _ in 0..INITIAL_SIZE {
            buckets.push(LinkedList::new());
        }
        ChainedHashMap { buckets }
    }
    fn hash(&self,key:&K) -> usize {
        0
    }

    fn insert(&self, key: K, value: V) -> Option<V> {
        let hash = self.hash(&key);
        let bucket = &self.buckets[hash];
        for entry in bucket {
            if entry.key == key {
                let old_value = entry.value.clone();
                entry.value = value;
                return Some(old_value);
            }
        }
        None
    }
}