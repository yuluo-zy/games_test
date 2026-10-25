//! `systems/mesh_follow_terrain.rs` — make meshes follow the terrain height.
//!
//! PDB: `mesh_follow_terrain @0x140a68430` (1556 B).
//!
//! O: puffin scope; Query over `MeshFollowTerrain` entities (component +
//! mesh handle) samples the terrain height under each mesh and updates its
//! transform; panics with "MeshFollowTerrain got an invalid mesh handle" when
//! the handle is invalid.

#![allow(dead_code, unused_imports)]

use bevy_ecs::prelude::*;

use crate::utils::components::transform::Transform;
use crate::Gap;

/// `MeshFollowTerrain` (O named entity; component).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct MeshFollowTerrain {
    pub _gap: Gap,
}

/// `mesh_follow_terrain` @0x140a68430 (O).
pub fn mesh_follow_terrain(
    mut _meshes: Query<(&MeshFollowTerrain, &mut Transform)>,
    _terrain: Res<Gap>,
) {
    let _scope = crate::reused::puffin::Scope::new("mesh_follow_terrain");
    todo!("recover mesh_follow_terrain (terrain sample → Transform)")
}
