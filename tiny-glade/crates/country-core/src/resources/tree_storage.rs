//! Authoritative tree-index → entity owner (B 0x140a38460..0x140a38600).
//! The native table uses std::HashMap; its bucket layout is not game logic.
use bevy_ecs::{component::Component, entity::Entity, resource::Resource};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[repr(transparent)]
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TreeStorageIndex(pub u64);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TreeStorageEntry {
    pub entity: Entity,
}

#[derive(Resource, Debug, Default)]
pub struct TreeStorage {
    entries: HashMap<TreeStorageIndex, TreeStorageEntry>,
    next_index: u64,
}
impl TreeStorage {
    pub fn iter(&self) -> impl Iterator<Item = (&TreeStorageIndex, &TreeStorageEntry)> {
        self.entries.iter()
    }
    pub fn get(&self, index: TreeStorageIndex) -> Option<&TreeStorageEntry> {
        self.entries.get(&index)
    }
    /// Native insert returns the previous entry. The counter advances for an
    /// explicit restored index as well as an allocated one; replacement is valid.
    pub fn insert(&mut self, index: TreeStorageIndex, entry: TreeStorageEntry) -> Option<TreeStorageEntry> {
        let previous = self.entries.insert(index, entry);
        self.next_index = self.next_index.max(index.0.wrapping_add(1));
        previous
    }
    pub fn remove(&mut self, index: TreeStorageIndex) -> Option<TreeStorageEntry> {
        self.entries.remove(&index)
    }
    /// 0x140a38600 returns the old counter and increments it with release wrapping.
    pub fn next_index(&mut self) -> TreeStorageIndex {
        let index = self.next_index;
        self.next_index = index.wrapping_add(1);
        TreeStorageIndex(index)
    }
}
