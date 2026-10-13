//! `startup/load_textures.rs` — texture asset loading.
//!
//! Source: B skeleton `crates/country-core/src/startup/load_textures.rs`.
//! O evidence (Ghidra, B patched EXE):
//! - `load_textures` — parallel (`bevy_tasks::ParallelSlice::par_chunk_map`)
//!   load of game textures into `AssetTextureLibrary` + `TextureWatch`; two
//!   filters: `LoadAllGameTextures` vs `LoadOnlyBootupTextures` (boot subset).
//! - `TextureLoadingFilter::should_load` @0x1409e6950.
//! - `load_glade_texture_mappings` @0x1409e6b60 →
//!   `GladeTextureMappings::create_mappings` (per-glade texture overrides).

#![allow(dead_code, unused_imports)]

use crate::Gap;

/// `GladeTextureMappings` (O named entity; per-glade texture override map).
#[derive(Clone, Debug, Default)]
pub struct GladeTextureMappings(pub Gap);

impl GladeTextureMappings {
    /// `create_mappings` (O inline in `load_glade_texture_mappings`).
    pub fn create_mappings(_session: &Gap) -> Self {
        GladeTextureMappings(Gap)
    }
}

/// `TextureLoadingFilter` (O trait; `should_load` @0x1409e6950).
pub trait TextureLoadingFilter {
    fn should_load(&self, _path: &str) -> bool;
}

/// `LoadAllGameTextures` (O named).
pub struct LoadAllGameTextures;
impl TextureLoadingFilter for LoadAllGameTextures {
    fn should_load(&self, _path: &str) -> bool {
        true
    }
}

/// `LoadOnlyBootupTextures` (O named).
pub struct LoadOnlyBootupTextures;
impl TextureLoadingFilter for LoadOnlyBootupTextures {
    fn should_load(&self, _path: &str) -> bool {
        false
    }
}

pub fn load_textures(_gpu: &Gap, _config: &Gap, _library: &mut Gap, _watch: &mut Gap) {
    todo!("recover load_textures (par_chunk_map texture load)")
}

pub fn load_glade_texture_mappings(_session: &Gap, _mappings: &mut GladeTextureMappings) {
    todo!("recover load_glade_texture_mappings")
}
