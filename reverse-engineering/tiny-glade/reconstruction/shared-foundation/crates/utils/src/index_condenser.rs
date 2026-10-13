//! Dense index compaction, recovered from B remove. HashMap supplies the generic
//! container; we recover key/cache repair and swap-remove ordering only.
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
    /// Producer adapter for the dense-vector/hash-index invariants observed by
    /// remove; the original generic insert signature was inline and is unresolved.
    pub fn insert(&mut self, key: K) -> IndexHandle<K> {
        let cached_index = *self.indices.entry(key.clone()).or_insert_with(|| {
            let index = u32::try_from(self.keys.len()).expect("index overflow");
            self.keys.push(key.clone());
            index
        });
        IndexHandle {
            key,
            cached_index: Cell::new(cached_index),
        }
    }
    pub fn keys(&self) -> &[K] {
        &self.keys
    }
    pub fn remove(&mut self, handle: &mut IndexHandle<K>) -> bool {
        // Original remove inlines the same private get interface (IPI0x74E3).
        let Some(index) = self.get(handle) else {
            return false;
        };
        let index = index as usize;
        let removed = self.keys.swap_remove(index);
        self.indices.remove(&removed);
        if let Some(moved) = self.keys.get(index) {
            *self.indices.get_mut(moved).expect("moved key missing") = index as u32;
        }
        true
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
        let mut c = IndexCondenser::default();
        let mut a = c.insert(10);
        let mut b = c.insert(20);
        let mut d = c.insert(30);
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
#[path = "../../../../../evidence/shared-foundation-2026-10-13/utils-base/interface-batch-02/index_tests.rs"]
mod interface_oracle;
