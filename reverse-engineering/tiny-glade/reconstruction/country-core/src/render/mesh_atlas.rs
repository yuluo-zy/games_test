//! `render/mesh_atlas.rs` — `AtlassedMeshName` (referenced by wp3 `nani/mesh_atlas`).
//!
//! This module sits at the B-skeleton path `crates/country-core/src/render/mesh_atlas.rs`
//! (distinct from `render/nani/mesh_atlas`). Only the name-key type is referenced by the
//! recovered packages (`MeshAtlas::add(name: AtlassedMeshName, …)`); its recovered form is
//! a typed gap (in the original it is a ustr-style interned name).

/// Atlas part name key (O name; payload U — ustr-internd word in the original).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct AtlassedMeshName(pub u64);
