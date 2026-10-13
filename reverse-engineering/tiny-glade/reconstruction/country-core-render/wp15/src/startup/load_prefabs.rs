//! `startup/load_prefabs.rs` — prefab (asset) loading + hotwatch.
//!
//! Source: B skeleton `crates/country-core/src/startup/load_prefabs.rs`.
//! O evidence (Ghidra, B patched EXE):
//! - `PrefabWatcher::new` @0x1412a8e00 / `watch_atlas` @0x1412a8fa0 /
//!   `watch_prefab` / `changed_world_subsets` (hotwatch file watching,
//!   reused `hotwatch` crate).
//! - `load_prefabs` (system: `Option<ResMut<PrefabWatcher>>` +
//!   `ResMut<PrefabLibrary>`) → `load_prefabs_impl` @0x1412a1df0
//!   → `load_prefab_mesh` / `recalculate_canopy_quad_attribs` /
//!   `filter_mesh_by_vertex_color` @0x140a08680.
//! - `reload_prefabs` @0x1412a1a60 (2 callers).
//! - `PrefabWatchPlugin` (bevy plugin; `build` @0x1408e4b00).

#![allow(dead_code, unused_imports)]

use crate::Gap;

/// `PrefabWatcher` (O named; file-watch handle).
#[derive(Clone, Debug, Default)]
pub struct PrefabWatcher(pub Gap);

impl PrefabWatcher {
    /// `PrefabWatcher::new` @0x1412a8e00 (O).
    pub fn new() -> Self {
        PrefabWatcher(Gap)
    }
    /// `PrefabWatcher::watch_atlas` @0x1412a8fa0 (O).
    pub fn watch_atlas(&mut self, _path: &std::path::Path) {
        todo!("recover PrefabWatcher::watch_atlas")
    }
    /// `PrefabWatcher::watch_prefab` (O).
    pub fn watch_prefab(&mut self, _path: &std::path::Path) {
        todo!("recover PrefabWatcher::watch_prefab")
    }
    /// `changed_world_subsets` (O inline in `reload_mesh_atlases`).
    pub fn changed_world_subsets(&mut self) -> Vec<u8> {
        Vec::new()
    }
}

/// `load_prefabs_impl` @0x1412a1df0 (O).
pub fn load_prefabs_impl(_library: &mut Gap, _watcher: Option<&mut PrefabWatcher>) {
    todo!("recover load_prefabs_impl")
}

/// `load_prefab_mesh` (O inline).
pub fn load_prefab_mesh(_library: &mut Gap, _path: &std::path::Path) {
    todo!("recover load_prefab_mesh")
}

/// `recalculate_canopy_quad_attribs` (O inline).
pub fn recalculate_canopy_quad_attribs() {
    todo!("recover recalculate_canopy_quad_attribs")
}

/// `filter_mesh_by_vertex_color` @0x140a08680 (O).
pub fn filter_mesh_by_vertex_color(_mesh: &mut Gap, _colors: &[f32]) {
    todo!("recover filter_mesh_by_vertex_color")
}

/// `reload_prefabs` @0x1412a1a60 (O).
pub fn reload_prefabs(_library: &mut Gap, _watcher: Option<&mut PrefabWatcher>) {
    todo!("recover reload_prefabs")
}
