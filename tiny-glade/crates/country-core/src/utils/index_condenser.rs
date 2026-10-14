//! Consumer adapter for the shared recovered index contract. B insert inline
//! IPI0x74ee rejects occupied IDs; get/removal repair stale hints. The original
//! typed marker remains local, while ID/state policy lives in the utils crate.
use ::utils::index_condenser::{Condex, IndexCondenser as SharedCondenser};
use core::{cell::Cell, marker::PhantomData};

pub struct IndexCondenser<T> {
    shared: SharedCondenser<u32>,
    _marker: PhantomData<fn() -> T>,
}
impl<T> Default for IndexCondenser<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> IndexCondenser<T> {
    pub fn new() -> Self {
        Self {
            shared: SharedCondenser::default(),
            _marker: PhantomData,
        }
    }
    pub fn len(&self) -> usize {
        self.shared.len()
    }
    pub fn is_empty(&self) -> bool {
        self.shared.is_empty()
    }
    pub fn contains_id(&self, id: u32) -> bool {
        self.shared.contains_key(&id)
    }
    pub fn insert(&mut self, id: u32) -> Option<usize> {
        self.shared
            .insert(id)
            .map(|key| key.last_known_index() as usize)
    }
    /// Compatibility for callers that already established an unoccupied ID.
    pub fn push(&mut self, id: u32) -> usize {
        self.insert(id).expect("ID already occupied")
    }
    pub fn id_at(&self, dense: usize) -> Option<u32> {
        self.shared.keys().get(dense).copied()
    }
    pub fn resolve(&self, id: u32, hint: &mut u32) -> Option<usize> {
        let key = Condex {
            key: id,
            cached_index: Cell::new(*hint),
        };
        let resolved = self.shared.get(&key);
        *hint = key.last_known_index();
        resolved.map(|i| i as usize)
    }
    pub fn remove(&mut self, id: u32, hint: &mut u32) -> Option<usize> {
        let key = Condex {
            key: id,
            cached_index: Cell::new(*hint),
        };
        let removed = self.shared.remove_index(&key);
        *hint = key.last_known_index();
        removed.map(|i| i as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct K;

    #[test]
    fn push_resolve_and_fast_path() {
        let mut c = IndexCondenser::<K>::new();
        assert_eq!(c.push(10), 0);
        assert_eq!(c.push(11), 1);
        let mut hint = 1;
        assert_eq!(c.resolve(11, &mut hint), Some(1));
        // 提示错误 → 走哈希并回写提示。
        let mut bad = 0;
        assert_eq!(c.resolve(11, &mut bad), Some(1));
        assert_eq!(bad, 1);
        // 提示越界、id 不存在。
        let mut oob = 99;
        assert_eq!(c.resolve(77, &mut oob), None);
        assert!(c.contains_id(10) && !c.contains_id(77));
    }

    #[test]
    fn remove_swaps_last_into_hole_and_fixes_map() {
        let mut c = IndexCondenser::<K>::new();
        for id in [5, 6, 7] {
            c.push(id);
        }
        let mut h = 0;
        assert_eq!(c.remove(5, &mut h), Some(0));
        assert_eq!(c.len(), 2);
        assert_eq!(c.id_at(0), Some(7)); // 末元素补位
        let mut h7 = 99;
        assert_eq!(c.resolve(7, &mut h7), Some(0)); // 映射已改写
        assert!(!c.contains_id(5));
        // 移除末元素：不需要补位。
        let mut h6 = 1;
        assert_eq!(c.remove(6, &mut h6), Some(1));
        assert_eq!(c.len(), 1);
        // 未命中。
        let mut hm = 0;
        assert_eq!(c.remove(6, &mut hm), None);
    }

    #[test]
    fn empty_map_fails_fast() {
        let c = IndexCondenser::<K>::new();
        let mut h = 0;
        assert_eq!(c.resolve(1, &mut h), None);
    }
}
