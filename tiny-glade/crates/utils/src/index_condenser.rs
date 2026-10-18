//! Dense indices for stable keys, with self-repairing cached handles.
use std::{
    cell::Cell,
    collections::{HashMap, hash_map::Entry},
    hash::Hash,
};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IndexHandle<K> {
    pub key: K,
    pub cached_index: Cell<u32>,
}
/// A handle whose cached index can be refreshed through a shared borrow.
pub type Condex<K> = IndexHandle<K>;
impl<K> IndexHandle<K> {
    pub fn last_known_index(&self) -> u32 {
        self.cached_index.get()
    }
}
pub struct IndexCondenser<K> {
    keys: Vec<K>,
    indices: HashMap<K, u32>,
}
impl<K: Eq + Hash + Clone> Default for IndexCondenser<K> {
    fn default() -> Self {
        Self {
            keys: Vec::new(),
            indices: HashMap::new(),
        }
    }
}
impl<K: Eq + Hash + Clone> IndexCondenser<K> {
    pub fn len(&self) -> usize {
        self.keys.len()
    }
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }
    pub fn contains_key(&self, key: &K) -> bool {
        self.indices.contains_key(key)
    }
    /// Inserts a new key, returning `None` when it already exists or no further
    /// index fits in `u32`. Existing handles follow keys through swap removal.
    pub fn insert(&mut self, key: K) -> Option<Condex<K>> {
        let index = u32::try_from(self.keys.len()).ok()?;
        match self.indices.entry(key) {
            Entry::Occupied(_) => None,
            Entry::Vacant(entry) => {
                let key = entry.key().clone();
                self.keys.push(key.clone());
                entry.insert(index);
                Some(Condex {
                    key,
                    cached_index: Cell::new(index),
                })
            }
        }
    }
    pub fn keys(&self) -> &[K] {
        &self.keys
    }
    pub fn remove(&mut self, handle: &mut IndexHandle<K>) -> bool {
        self.remove_index(handle).is_some()
    }
    /// Removes a key and returns its old index so parallel payload arrays can
    /// apply the same swap removal.
    pub fn remove_index(&mut self, handle: &Condex<K>) -> Option<u32> {
        let index = self.get(handle)?;
        let index = index as usize;
        let removed = self.keys.swap_remove(index);
        self.indices.remove(&removed);
        if let Some(moved) = self.keys.get(index) {
            *self.indices.get_mut(moved).expect("moved key missing") = index as u32;
        }
        Some(index as u32)
    }
    pub fn resolve(&self, handle: &mut IndexHandle<K>) -> Option<u32> {
        self.get(handle)
    }
    pub fn get(&self, handle: &Condex<K>) -> Option<u32> {
        let cached = handle.cached_index.get();
        if self.keys.get(cached as usize) == Some(&handle.key) {
            return Some(cached);
        }
        let index = *self.indices.get(&handle.key)?;
        handle.cached_index.set(index);
        Some(index)
    }
    pub fn clear(&mut self) {
        // Retain capacity. Cached handles repair or fail on their next lookup.
        self.indices.clear();
        self.keys.clear();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stale_cached_index_is_repaired_before_swap_removal() {
        // Arrange stale caches left behind by swap removal.
        let mut c = IndexCondenser {
            keys: vec![10, 20, 30],
            indices: [(10, 0), (20, 1), (30, 2)].into_iter().collect(),
        };
        let mut a = IndexHandle {
            key: 10,
            cached_index: Cell::new(0),
        };
        let mut b = IndexHandle {
            key: 20,
            cached_index: Cell::new(1),
        };
        let mut d = IndexHandle {
            key: 30,
            cached_index: Cell::new(2),
        };
        assert!(c.remove(&mut a));
        assert_eq!(c.keys(), [30, 20]);
        assert!(c.remove(&mut d));
        assert_eq!(d.last_known_index(), 0);
        assert_eq!(c.keys(), [20]);
        assert_eq!(c.resolve(&mut b), Some(0));
        assert!(!c.remove(&mut d));
    }
}

#[cfg(test)]
#[path = "oracle/index_tests.rs"]
mod interface_oracle;

#[cfg(test)]
#[path = "oracle/index_producer_tests.rs"]
mod producer_oracle;
