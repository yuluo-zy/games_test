//! `AssetShaderLibrary` — shader program cache (WP4-owned, render-facing surface).
//!
//! Skeleton bundle: `skeleton/B/crates/country-core/src/asset_libraries/shader_library.rs`
//! (source_sha256 `16e9fe00…`, 6 procs). Decompiles:
//! `evidence/building-gates/B-mesh-helpers/140936c60.c`, `140936ef0.c`,
//! `140937aa0.c`; `_analysis/decomp/C_0x140937560.c`, `C_0x1409378e0.c`,
//! `C_0x140937cb0.c`.
//!
//! Layout (O): raster/compute FrozenMaps + two name maps:
//!   raster_names: RwLock<HashMap<String, Handle<RasterShaderProgram>>>
//!                 (lock +0x80, buckets +0x88, mask +0x90, items +0xa0)
//!   compute_names: RwLock<HashMap<ComputeShaderSourceHash, Handle<ComputeShaderProgram>>>
//!                 (NoHashHasher; lock +0xb8, buckets +0xc0, mask +0xc8, items +0xd8)
//! Containers: `frozen::FrozenMap` / `parking_lot::RwLock` are REUSED deps.

use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};

use parking_lot::RwLock;

use crate::asset_libraries::Handle;
use crate::render::shader::{ComputeShaderProgram, RasterShaderProgram};

/// Hash of the compute-shader source used as the cache key (O: a u64 —
/// `ComputeShaderSourceHash` keyed map with `NoHashHasher` in `add_compute`).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ComputeShaderSourceHash(pub u64);

/// No-op hasher for pre-hashed keys (matches the observed NoHashHasher usage).
#[derive(Default, Clone, Copy)]
pub struct NoHashHasher(u64);

impl Hasher for NoHashHasher {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, _bytes: &[u8]) {
        // Unused for integer keys.
    }
    fn write_u64(&mut self, i: u64) {
        self.0 = i;
    }
}

pub type NoHashBuildHasher = BuildHasherDefault<NoHashHasher>;

/// Shader program library (O layout above).
pub struct AssetShaderLibrary {
    /// Raster programs keyed by handle.
    pub raster_shaders: HashMap<Handle<RasterShaderProgram>, Box<RasterShaderProgram>>,
    /// Compute programs keyed by handle.
    pub compute_shaders: HashMap<Handle<ComputeShaderProgram>, Box<ComputeShaderProgram>>,
    /// Raster name → handle (O).
    raster_names: RwLock<HashMap<String, Handle<RasterShaderProgram>>>,
    /// Compute source-hash → handle (O, NoHashHasher).
    compute_names: RwLock<HashMap<ComputeShaderSourceHash, Handle<ComputeShaderProgram>>>,
}

impl AssetShaderLibrary {
    /// `new` (O: 0x140936c60) — empty library with seeded maps.
    pub fn new() -> Self {
        AssetShaderLibrary {
            raster_shaders: HashMap::new(),
            compute_shaders: HashMap::new(),
            raster_names: RwLock::new(HashMap::new()),
            compute_names: RwLock::new(HashMap::new()),
        }
    }

    /// `add_raster(name, program)` (O: 0x140936ef0).
    ///
    /// Dedup: if `name` is already registered, the existing program must be
    /// identical (source name + compiled source + version); equal → no-op,
    /// different → panic `"AssetShaderLibrary: the shader … already in the
    /// library"`. Otherwise allocates a handle, registers name→handle and stores
    /// the Boxed program.
    pub fn add_raster(
        &mut self,
        name: &str,
        program: RasterShaderProgram,
    ) -> Handle<RasterShaderProgram> {
        let mut names = self.raster_names.write();
        if let Some(&existing) = names.get(name) {
            // O: dedup compare — identical sources return the existing handle.
            if let Some(cur) = self.raster_shaders.get(&existing) {
                let same = cur.source_name == program.source_name
                    && cur.compiled_variants.base_pipeline
                        == program.compiled_variants.base_pipeline;
                if same {
                    return existing;
                }
            }
            panic!(
                "AssetShaderLibrary: the shader {:?} is already in the library",
                name
            );
        }
        let handle = Handle::new();
        names.insert(name.to_owned(), handle);
        self.raster_shaders.insert(handle, Box::new(program));
        handle
    }

    /// `add_compute(source_hash, program)` (O: 0x140937560).
    ///
    /// Duplicate source hash → panic `"the shader … already in library"`.
    pub fn add_compute(
        &mut self,
        source_hash: ComputeShaderSourceHash,
        program: ComputeShaderProgram,
    ) -> Handle<ComputeShaderProgram> {
        let mut names = self.compute_names.write();
        if names.contains_key(&source_hash) {
            panic!(
                "AssetShaderLibrary: the shader {:016x} is already in the library",
                source_hash.0
            );
        }
        let handle = Handle::new();
        names.insert(source_hash, handle);
        self.compute_shaders.insert(handle, Box::new(program));
        handle
    }

    /// `get_compute_by_name(hash) -> Handle<ComputeShaderProgram>` (O:
    /// 0x1409378e0) — shared-lock lookup by precomputed source hash; returns the
    /// cached handle (caller `ComputePass::new` caches by name-hash).
    pub fn get_compute_by_name(
        &self,
        hash: ComputeShaderSourceHash,
    ) -> Option<Handle<ComputeShaderProgram>> {
        self.compute_names.read().get(&hash).copied()
    }

    /// `get_compute_handle_by_name` (O name; thin wrapper).
    pub fn get_compute_handle_by_name(
        &self,
        hash: ComputeShaderSourceHash,
    ) -> Handle<ComputeShaderProgram> {
        self.get_compute_by_name(hash)
            .expect("compute shader not registered")
    }

    /// `get_compute(hash) -> &ComputeShaderProgram` (O name).
    pub fn get_compute(&self, hash: ComputeShaderSourceHash) -> Option<&ComputeShaderProgram> {
        self.get_compute_by_name(hash)
            .and_then(|h| self.compute_shaders.get(&h))
            .map(|b| &**b)
    }

    /// `get_compute_shader_name` (O name; U body).
    pub fn get_compute_shader_name(&self, _handle: Handle<ComputeShaderProgram>) -> Option<String> {
        // U
        None
    }

    /// `get_raster_handle_by_name(name) -> Handle<RasterShaderProgram>` (O:
    /// 0x140937aa0) — shared-lock string lookup.
    pub fn get_raster_handle_by_name(&self, name: &str) -> Option<Handle<RasterShaderProgram>> {
        self.raster_names.read().get(name).copied()
    }

    /// `get_raster_by_name(name) -> Option<&RasterShaderProgram>` (O name).
    pub fn get_raster_by_name(&self, name: &str) -> Option<&RasterShaderProgram> {
        self.get_raster_handle_by_name(name)
            .and_then(|h| self.raster_shaders.get(&h))
            .map(|b| &**b)
    }

    /// `get_raster(handle)` (O name).
    pub fn get_raster(&self, handle: Handle<RasterShaderProgram>) -> Option<&RasterShaderProgram> {
        self.raster_shaders.get(&handle).map(|b| &**b)
    }

    /// `get_raster_mut(handle)` (O name).
    pub fn get_raster_mut(
        &mut self,
        handle: Handle<RasterShaderProgram>,
    ) -> Option<&mut RasterShaderProgram> {
        self.raster_shaders.get_mut(&handle).map(|b| &mut **b)
    }

    /// `debug_raster_get_name(handle) -> Option<&str>` (O: 0x140937cb0) — returns
    /// the registered source name for a raster handle. Returns an owned `String`
    /// in the skeleton (the original hands out a borrowed name from the map).
    pub fn debug_raster_get_name(&self, handle: Handle<RasterShaderProgram>) -> Option<String> {
        self.raster_names
            .read()
            .iter()
            .find(|(_, h)| **h == handle)
            .map(|(name, _)| name.clone())
    }

    /// `extract_raster_shader_names` (O name; U body).
    pub fn extract_raster_shader_names(&self) -> Vec<String> {
        // U
        self.raster_names.read().keys().cloned().collect()
    }
}

impl Default for AssetShaderLibrary {
    fn default() -> Self {
        Self::new()
    }
}
