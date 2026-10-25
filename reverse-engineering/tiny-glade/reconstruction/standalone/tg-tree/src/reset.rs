//! `reset_tree_tool` (startup_tree_tool.rs; fully inlined into
//! run_unsafe<FunctionSystem<ResMut<TreeStorage>>> 0x141bb19d0; registered on
//! OnExit<AppLoadingState>). Evidence: tree-tool-extra-analysis.md §2:
//! `*tree_storage = TreeStorage::default()` (empty map, next_index = 0).
//! The hashbrown KEYS random-key refresh is not modelled (UNKNOWN).

use crate::storage::TreeStorage;
use bevy_ecs::prelude::*;

/// `fn reset_tree_tool(mut tree_storage: ResMut<TreeStorage>)`.
pub fn reset_tree_tool(mut tree_storage: ResMut<TreeStorage>) {
    *tree_storage = TreeStorage::default();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{storage::TreeEntry, types::TreeComponent};
    use bevy_app::prelude::*;

    #[test]
    fn clears_map_and_next_index() {
        let mut app = App::new();
        let e = app.world_mut().spawn(TreeComponent).id();
        let mut s = TreeStorage::new();
        s.insert(9, TreeEntry::new(0, e));
        app.insert_resource(s).add_systems(Update, reset_tree_tool);
        app.update();
        let s = app.world().resource::<TreeStorage>();
        assert_eq!((s.len(), s.next_index(), s.get(9)), (0, 0, None));
    }
}
