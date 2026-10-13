//! `AssetMeshLibrary` — mesh registry (WP4-owned, render-facing surface).
//!
//! Skeleton bundle: `skeleton/B/crates/country-core/src/asset_libraries/mesh_library.rs`
//! (source_sha256 `312b9e63…`, 12 procs). Decompiles:
//! `evidence/building-gates/B-mesh-helpers/14092c920.c`, `14092cc60.c`;
//! `_analysis/decomp/D_0x14092d2e0.c`, `D_0x14092d300.c`, `D_0x14092d4f0.c`,
//! `C_0x14092d450.c`, `C_0x14092d570.c`, `C_0x14092d750.c`, `E_0x14092d8d0.c`.
//!
//! Layout (O): primary mesh map, name map
//! `RwLock<HashMap<String, Handle<Mesh>>>` (lock +0x60, buckets +0x68, mask
//! +0x70, items +0x80) and source-path map
//! `RwLock<HashMap<PathBuf, Handle<Mesh>>>` (lock +0x98, buckets +0xa0, mask
//! +0xa8, items +0xb8).

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use parking_lot::RwLock;

use crate::asset_libraries::Handle;
use crate::render::mesh::Mesh;

/// Mesh registry.
pub struct AssetMeshLibrary {
    /// Primary handle → mesh store.
    pub meshes: HashMap<Handle<Mesh>, Box<Mesh>>,
    /// Name → handle (O).
    name_to_handle: RwLock<HashMap<String, Handle<Mesh>>>,
    /// Source-path → handle, used for JSON mesh reload (O).
    path_to_handle: RwLock<HashMap<PathBuf, Handle<Mesh>>>,
    /// +0x28 disposed flag (O: checked by `remove`/`reload_mesh_src`).
    disposed: bool,
}

impl AssetMeshLibrary {
    /// `new` (O: 0x14092c920).
    pub fn new() -> Self {
        AssetMeshLibrary {
            meshes: HashMap::new(),
            name_to_handle: RwLock::new(HashMap::new()),
            path_to_handle: RwLock::new(HashMap::new()),
            disposed: false,
        }
    }

    /// `add(name, mesh) -> Handle<Mesh>` (O: 0x14092cc60): new global HandleId,
    /// Box the 120-byte Mesh, register name→handle and source-path→handle.
    pub fn add(&mut self, name: String, mesh: Mesh) -> Handle<Mesh> {
        let handle = Handle::new();
        let src = mesh_source_file(&mesh);
        self.meshes.insert(handle, Box::new(mesh));
        self.name_to_handle.write().insert(name, handle);
        if let Some(src) = src {
            self.path_to_handle.write().insert(src, handle);
        }
        handle
    }

    /// `get(handle) -> &Mesh` (O: 0x14092d2e0 — FrozenMap::get).
    pub fn get(&self, handle: Handle<Mesh>) -> Option<&Mesh> {
        self.meshes.get(&handle).map(|b| &**b)
    }

    /// `get_mut(handle) -> &mut Mesh` (O: 0x14092d300).
    pub fn get_mut(&mut self, handle: Handle<Mesh>) -> Option<&mut Mesh> {
        self.meshes.get_mut(&handle).map(|b| &mut **b)
    }

    /// `try_get_handle_by_name(name) -> Option<Handle<Mesh>>` (O: 0x14092d570).
    pub fn try_get_handle_by_name(&self, name: &str) -> Option<Handle<Mesh>> {
        self.name_to_handle.read().get(name).copied()
    }

    /// `get_handle_by_name(name)` (O: 0x14092d4f0) — panics when missing.
    pub fn get_handle_by_name(&self, name: &str) -> Handle<Mesh> {
        self.try_get_handle_by_name(name)
            .unwrap_or_else(|| panic!("mesh {:?} not found", name))
    }

    /// `get_by_name(name) -> &Mesh` (O: 0x14092d450) — panics when missing.
    pub fn get_by_name(&self, name: &str) -> &Mesh {
        let handle = self.get_handle_by_name(name);
        self.get(handle)
            .unwrap_or_else(|| panic!("mesh {:?} not found", name))
    }

    /// `remove(handle)` (O: 0x14092d750): drop the mesh and purge stale
    /// name mappings.
    pub fn remove(&mut self, handle: Handle<Mesh>) {
        assert!(!self.disposed, "mesh library already disposed");
        self.meshes.remove(&handle);
        let mut names = self.name_to_handle.write();
        names.retain(|_, h| *h != handle);
    }

    /// `reload_mesh_src(path)` (O: 0x14092d8d0): reload a mesh from its JSON
    /// source file (`utils::load_json::load_json_as_mesh`), replacing the stored
    /// Mesh in place. Non-`.json` extensions assert-fail. (The original performs
    /// the mutation through the mesh store's interior locks; modeled here as
    /// `&mut self` for the skeleton.)
    pub fn reload_mesh_src(&mut self, path: &Path) {
        let handle = {
            let paths = self.path_to_handle.read();
            paths
                .get(path)
                .copied()
                .expect("mesh source path not registered")
        };
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            panic!("mesh source extension must be json: {}", path.display());
        }
        // O: `get_mut` + `load_json_as_mesh<&str>(path)`.
        let _ = handle;
        let new_mesh = crate::utils_contract::load_json_as_mesh(path).ok();
        if let Some(new_mesh) = new_mesh {
            // O: replaces the stored Mesh contents field-by-field.
            if let Some(existing) = self.meshes.get_mut(&handle) {
                **existing = new_mesh;
            }
        }
    }

    /// `is_dirty` (O name; U body — reload watcher state).
    pub fn is_dirty(&self) -> bool {
        false
    }
}

impl Default for AssetMeshLibrary {
    fn default() -> Self {
        Self::new()
    }
}

/// O: the mesh's source file is stored in the +0x68/+0x70 region of `Mesh`
/// (`set_source_file`); helper to expose it to the path map.
fn mesh_source_file(_mesh: &Mesh) -> Option<PathBuf> {
    // U: source-file field not exposed as a public Mesh field in WP4.
    None
}
