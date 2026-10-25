//! TreeStorage (0x140a38460 get / 0x140a38560 insert / 0x140a385a0 remove).
//! Evidence: tree-process-cmds-analysis.md "TreeStorage 布局"; reset: tree-tool-extra-analysis.md §2.

use bevy_ecs::prelude::*;
// std HashMap: the original uses hashbrown (random KEYS refreshed on reset; not modelled).
use std::collections::HashMap;

/// 8-byte map value: low32 = prefab table index, high32 = entity index.
/// The full `Entity` (with generation) is kept so lookups address the right entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TreeEntry {
    pub prefab_index: u32,
    pub entity: Entity,
}

impl TreeEntry {
    pub fn new(prefab_index: u32, entity: Entity) -> Self {
        Self { prefab_index, entity }
    }
    pub fn entity_index(&self) -> u32 {
        self.entity.index()
    }
    /// The packed 8-byte value as stored/returned by the original.
    pub fn raw(&self) -> u64 {
        ((self.entity.index() as u64) << 32) | self.prefab_index as u64
    }
}

/// u64 index -> entry, plus `next_index` (u64).
#[derive(Resource, Debug, Clone, Default, PartialEq)]
pub struct TreeStorage {
    map: HashMap<u64, TreeEntry>,
    next_index: u64,
}

impl TreeStorage {
    pub fn new() -> Self {
        Self::default()
    }

    /// 0x140a38460. Original returns None early when the map is empty.
    pub fn get(&self, index: u64) -> Option<TreeEntry> {
        self.map.get(&index).copied()
    }

    /// 0x140a38560: hash insert then `next_index = max(next_index, index+1)`.
    /// Returns the previous value if the slot was occupied.
    pub fn insert(&mut self, index: u64, entry: TreeEntry) -> Option<TreeEntry> {
        let old = self.map.insert(index, entry);
        self.next_index = self.next_index.max(index.saturating_add(1));
        old
    }

    /// 0x140a385a0. Does not touch `next_index`.
    pub fn remove(&mut self, index: u64) -> Option<TreeEntry> {
        self.map.remove(&index)
    }

    pub fn next_index(&self) -> u64 {
        self.next_index
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn e(p: u32, ent: u32) -> TreeEntry {
        TreeEntry::new(p, Entity::from_raw(ent))
    }

    #[test]
    fn next_index_is_max_plus_one() {
        let mut s = TreeStorage::new();
        assert_eq!(s.next_index(), 0);
        s.insert(5, e(0, 1));
        assert_eq!(s.next_index(), 6);
        s.insert(2, e(0, 2));
        assert_eq!(s.next_index(), 6);
        s.insert(u64::MAX, e(0, 3));
        assert_eq!(s.next_index(), u64::MAX);
    }

    #[test]
    fn insert_returns_old_and_remove_keeps_next() {
        let mut s = TreeStorage::new();
        assert_eq!(s.insert(1, e(0, 7)), None);
        assert_eq!(s.insert(1, e(0, 8)), Some(e(0, 7)));
        assert_eq!(s.get(1), Some(e(0, 8)));
        assert_eq!(s.remove(1), Some(e(0, 8)));
        assert_eq!(s.remove(1), None);
        assert_eq!(s.next_index(), 2);
        assert!(s.is_empty());
    }

    #[test]
    fn packing_layout() {
        let t = e(3, 9);
        assert_eq!(t.raw(), (9u64 << 32) | 3);
        assert_eq!(t.entity, Entity::from_raw(9));
        assert_eq!(t.entity_index(), 9);
    }

    #[test]
    fn default_is_reset_state() {
        let mut s = TreeStorage::new();
        s.insert(4, e(0, 1));
        s = TreeStorage::default();
        assert_eq!((s.len(), s.next_index()), (0, 0));
    }
}
