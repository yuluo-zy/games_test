//! B's insertion, cache repair and swap-removal policy. Original ID storage is
//! u32; K is a safe key adapter, not a claim about the author's generic ABI.
use std::{cell::Cell, collections::HashMap, hash::Hash};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IndexHandle<K> {
    pub key: K,
    pub cached_index: Cell<u32>,
}
/// Original private handle name; the cache can be refreshed through a shared
/// borrow, matching the original Cell and its get/update consumers.
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
    /// IPI 0x74ee, B consumers 0x140992280 and 0x14156d0d0: an occupied
    /// key fails before any append/map write; a new key gets the old Vec length.
    /// Both consumers retry their next ID on failure. Option is a semantic
    /// adapter for that branch; no independent Rust return ABI was emitted.
    pub fn insert(&mut self, key: K) -> Option<Condex<K>> {
        if self.indices.contains_key(&key) {
            return None;
        }
        let index = self.keys.len() as u32;
        self.keys.push(key.clone());
        self.indices.insert(key.clone(), index);
        Some(Condex {
            key,
            cached_index: Cell::new(index),
        })
    }
    pub fn keys(&self) -> &[K] {
        &self.keys
    }
    pub fn remove(&mut self, handle: &mut IndexHandle<K>) -> bool {
        self.remove_index(handle).is_some()
    }
    /// The complete B remove body returns the old dense index on success.
    /// Keep it available to the consumer that swaps its parallel payload arrays.
    pub fn remove_index(&mut self, handle: &Condex<K>) -> Option<u32> {
        // Original remove inlines the same private get interface (IPI0x74E3).
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
        // Original inline clear delegates to HashMap::clear and Vec::clear.
        // Retain allocation capacity; cached handles repair/fail through get.
        self.indices.clear();
        self.keys.clear();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stale_cached_index_is_repaired_before_swap_removal() {
        // Direct fixture setup exercises the recovered remove/get bodies without
        // claiming an original insertion API or its unknown producer policy.
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
