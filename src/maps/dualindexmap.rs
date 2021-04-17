use core::hash::Hash;
use std::collections::HashMap;
use std::rc::Rc;

/// This is an example of a DualIndexMap that won't compile. For more
/// see this blog post:
/// http://deepmesa.com/blog/rust-storing-data-in-multiple-containers
///
pub mod compileerr {

    use super::*;

    pub struct DualIndexMap<K, V> {
        kv_map: HashMap<K, V>,
        vk_map: HashMap<V, K>,
    }

    impl<K: Hash + Eq, V: Hash + Eq> DualIndexMap<K, V> {
        pub fn new(capacity: usize) -> DualIndexMap<K, V> {
            DualIndexMap {
                kv_map: HashMap::with_capacity(capacity),
                vk_map: HashMap::with_capacity(capacity),
            }
        }

        pub fn put(&mut self, key: K, val: V) {
            self.kv_map.insert(key, val);

            //Uncommenting the following statement results in a compile error. For more see:
            //http://deepmesa.com/blog/rust-storing-data-in-multiple-containers

            //self.vk_map.insert(val, key);
        }

        pub fn get_by_key(&self, key: &K) -> Option<&V> {
            self.kv_map.get(key)
        }

        pub fn get_by_val(&self, val: &V) -> Option<&K> {
            self.vk_map.get(val)
        }
    }
}

/// This is an example of a DualIndexMap that requires the `Copy`
/// trait on the Key and Value. For more see this blog post:
/// http://deepmesa.com/blog/rust-storing-data-in-multiple-containers
///
pub mod copytrait {
    use super::*;

    pub struct DualIndexMap<K, V> {
        kv_map: HashMap<K, V>,
        vk_map: HashMap<V, K>,
    }

    //K and V must implement the Copy trait
    impl<K: Copy + Hash + Eq, V: Copy + Hash + Eq> DualIndexMap<K, V> {
        pub fn new(capacity: usize) -> DualIndexMap<K, V> {
            DualIndexMap {
                kv_map: HashMap::with_capacity(capacity),
                vk_map: HashMap::with_capacity(capacity),
            }
        }

        pub fn put(&mut self, key: K, val: V) {
            self.kv_map.insert(key, val);
            self.vk_map.insert(val, key);
        }

        pub fn get_by_key(&self, key: &K) -> Option<&V> {
            self.kv_map.get(key)
        }

        pub fn get_by_val(&self, val: &V) -> Option<&K> {
            self.vk_map.get(val)
        }
    }
}

/// This is an example of a DualIndexMap that requires the `Clone`
/// trait on the Key and Value. For more see this blog post:
/// http://deepmesa.com/blog/rust-storing-data-in-multiple-containers
///
pub mod clonetrait {
    use super::*;

    pub struct DualIndexMap<K, V> {
        kv_map: HashMap<K, V>,
        vk_map: HashMap<V, K>,
    }

    //K and V must implement the Clone trait
    impl<K: Clone + Hash + Eq, V: Clone + Hash + Eq> DualIndexMap<K, V> {
        pub fn new(capacity: usize) -> DualIndexMap<K, V> {
            DualIndexMap {
                kv_map: HashMap::with_capacity(capacity),
                vk_map: HashMap::with_capacity(capacity),
            }
        }

        pub fn put(&mut self, key: K, val: V) {
            //Key and Value are explicitly copied into kv_map using clone()
            self.kv_map.insert(key.clone(), val.clone());
            //Key and Value are moved into the vk_map
            self.vk_map.insert(val, key);
        }

        pub fn get_by_key(&self, key: &K) -> Option<&V> {
            self.kv_map.get(key)
        }

        pub fn get_by_val(&self, val: &V) -> Option<&K> {
            self.vk_map.get(val)
        }
    }
}

/// This is an example of a DualIndexMap that uses `Rc` for the key
/// and value. For more see this blog post:
/// http://deepmesa.com/blog/rust-storing-data-in-multiple-containers
///
pub mod rcversion {
    use super::*;

    pub struct DualIndexMap<K, V> {
        kv_map: HashMap<Rc<K>, Rc<V>>,
        vk_map: HashMap<Rc<V>, Rc<K>>,
    }

    //K and V must implement the Clone trait
    impl<K: Hash + Eq, V: Hash + Eq> DualIndexMap<K, V> {
        pub fn new(capacity: usize) -> DualIndexMap<K, V> {
            DualIndexMap {
                kv_map: HashMap::with_capacity(capacity),
                vk_map: HashMap::with_capacity(capacity),
            }
        }

        pub fn put(&mut self, key: K, val: V) {
            let k = Rc::new(key);
            let v = Rc::new(val);
            //Key and Value are explicitly copied into kv_map using clone()
            self.kv_map.insert(Rc::clone(&k), Rc::clone(&v));
            //Key and Value are moved into the vk_map
            self.vk_map.insert(v, k);
        }

        pub fn get_by_key(&self, key: &K) -> Option<&V> {
            if let Some(r) = self.kv_map.get(key) {
                return Some(&r);
            }
            None
        }

        pub fn get_by_val(&self, val: &V) -> Option<&K> {
            if let Some(r) = self.vk_map.get(val) {
                return Some(&r);
            }
            None
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_copy_version() {
        let mut dim = copytrait::DualIndexMap::<u8, u16>::new(10);
        dim.put(100, 1000);
        assert_eq!(dim.get_by_key(&100), Some(&1000));
        assert_eq!(dim.get_by_val(&1000), Some(&100));
        assert_eq!(dim.get_by_key(&101), None);
    }

    #[test]
    fn test_clone_version() {
        let mut dim = clonetrait::DualIndexMap::<u8, u16>::new(10);
        dim.put(100, 1000);
        assert_eq!(dim.get_by_key(&100), Some(&1000));
        assert_eq!(dim.get_by_val(&1000), Some(&100));
        assert_eq!(dim.get_by_key(&101), None);
    }

    #[test]
    fn test_rc_version() {
        let mut dim = rcversion::DualIndexMap::<u8, u16>::new(10);
        dim.put(100, 1000);
        assert_eq!(dim.get_by_key(&100), Some(&1000));
        assert_eq!(dim.get_by_val(&1000), Some(&100));
        assert_eq!(dim.get_by_key(&101), None);
    }
}
