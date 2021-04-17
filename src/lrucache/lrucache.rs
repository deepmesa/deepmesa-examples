use core::hash::Hash;
use deepmesa::lists::linkedlist::Node;
use deepmesa::lists::FastLinkedList;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct CacheEntry<K, V> {
    key: Rc<K>,
    val: V,
}

pub struct LruCache<K, V> {
    size: usize,
    map: HashMap<Rc<K>, Node<CacheEntry<K, V>>>,
    list: FastLinkedList<CacheEntry<K, V>>,
}

impl<K, V> CacheEntry<K, V> {
    pub fn new(key: Rc<K>, val: V) -> CacheEntry<K, V> {
        CacheEntry { key, val }
    }
}

impl<K: Hash + Eq, V> LruCache<K, V> {
    pub fn new(size: usize) -> LruCache<K, V> {
        LruCache {
            size: size,
            map: HashMap::with_capacity(size),
            list: FastLinkedList::with_capacity(size),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(llnode) = self.map.get(key) {
            if !llnode.make_head(&mut self.list) {
                panic!("failed to make head!");
            }

            match llnode.val(&self.list) {
                None => panic!("List does not doesn't contain expected value!"),
                Some(ce) => {
                    return Some(&ce.val);
                }
            }
        }
        None
    }

    pub fn put(&mut self, key: K, val: V) {
        match self.map.get(&key) {
            None => {
                let map_key = Rc::new(key);
                let new_node = self
                    .list
                    .push_head(CacheEntry::new(Rc::clone(&map_key), val));
                self.map.insert(map_key, new_node);
            }
            Some(llnode) => {
                match llnode.val_mut(&mut self.list) {
                    None => panic!("value not found in linkedlist"),
                    Some(ce) => (*ce).val = val,
                };
                if !llnode.make_head(&mut self.list) {
                    panic!("failed to make head!");
                }
            }
        }

        if self.map.len() > self.size {
            match self.list.pop_tail() {
                None => panic!("pop tail unexpectedly returned None"),
                Some(ce) => {
                    self.map.remove(&ce.key);
                }
            }
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(llnode) = self.map.remove(key) {
            match llnode.pop(&mut self.list) {
                None => panic!("List doesn't have this node"),
                Some(ce) => return Some(ce.val),
            }
        }
        None
    }

    pub fn contains(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }
}

#[cfg(test)]
mod test {

    use super::LruCache;

    #[test]
    fn test_put_get() {
        let mut lru_cache = LruCache::<String, u8>::new(3);
        lru_cache.put("one".to_string(), 1);
        lru_cache.put("two".to_string(), 2);
        lru_cache.put("three".to_string(), 3);
        assert_eq!(lru_cache.size(), 3);

        //get the value for one
        let one = lru_cache.get(&"one".to_string());
        assert_eq!(one, Some(&1));

        //if we add another key then the 2 should be evicted
        lru_cache.put("four".to_string(), 4);
        assert_eq!(lru_cache.size(), 3);
        assert_eq!(lru_cache.contains(&"two".to_string()), false);
    }
}
