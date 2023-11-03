use std::collections::LinkedList;
const INITIAL_SIZE: usize = 128;


#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Entry<K, V> {
    key: K,
    value: V,
}

struct ChainedHashMap<K, V> {
    buckets: Vec<LinkedList<Entry<K, V>>>,
}