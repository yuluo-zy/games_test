//! Shared archive owner; reuse the existing storage/restore implementation.
pub use crate::recovered_building::wall_decorator_archivist::{
    restore_archived_decorators, DecoratorArchivist, DecoratorStorage,
};
use std::collections::HashSet;

/// B +0x30 is the permitted decorator-ID set; +0x1e0 permits all IDs.
/// Explicit state is required until original startup construction is connected.
#[derive(Clone, Debug)]
pub struct DecoratorCullPolicy {
    pub allowed_ids: HashSet<u32>,
    pub allow_any: bool,
}

impl<C: Send + Sync + 'static> DecoratorArchivist<C> {
    /// B 0x140b2bbf0 changes this flag only, preserving the ID set.
    pub fn decorator_cull_allow_any(&mut self) {
        self.culling.allow_any = true;
    }

    /// B 0x140b2bd90: flag short-circuit, otherwise ID membership.
    pub fn is_decorator_cull_allowed(&self, id: u32) -> bool {
        self.culling.allow_any || self.culling.allowed_ids.contains(&id)
    }
}
