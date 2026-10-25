//! WP16 — country-core "remaining misc resources + systems" recovery
//! (Tiny Glade, version B, `ljxsj_92385`; EXE sha256 `cf048e27…`).
//!
//! Business-logic contracts (O/I/U + typed gaps) live in `contracts/*.md`.
//! This crate is the standalone offline `cargo check` gate for the skeleton-
//! aligned Rust surface delivered to the country-core integration owner.
//!
//! Scope recovered here (each module doc names its PDB bundle + RVAs):
//!   - `resources::ivy`      ivy growth state (Ivy polyline storage,
//!                           IvyDirectionProposer noise field, WallCoord).
//!   - `resources::rt`       software-BVH world (RtWorld / DirtyCow<RtMesh> /
//!                           RtMeshHandle, RtBvhBuilderGlue resource + worker,
//!                           GpuBlBvh builder, the three ray-tracing SSBOs).
//!   - `resources::{window_lights, color_id}`  lantern/window light + color-id
//!                           CPU state (extend the wp6 placeholders).
//!   - `systems::backend`    config/mesh/texture watch, vao_update, grow_grass,
//!                           icc profile, prepare_frame_constants, screen-log,
//!                           frame-index, panic-reporter + transient-mesh drain.
//!   - `systems::collision`  rapier-glue RaycastWorld + mesh-mesh intersection.
//!   - `systems::{debug,paths,steam,ivy}`  debug viz + sheep controls, path
//!                           drawing + history + brush-size, steam integration,
//!                           ivy grower/spawner/pruner/leaf systems.
//!   - `asset_libraries`     non-render registries: MeshWatch / ConfigWatch /
//!                           TextureWatch (hotwatch), AssetVAOLibrary,
//!                           AssetPlacementDataLibrary, Asset/Handle core.
//!   - `components`          TransientMesh + marker components residual, plus
//!                           `geometry` (Plane/Cube/Sphere/InstancedWall) and
//!                           `utils` (DirtyTiles / DebugViz / load_json / geo /
//!                           audio_loop / project_cursor / child_wall) residual.
//!
//! Reused-dependency surfaces (hotwatch+notify, rapier3d, steamworks,
//! bracket-noise, perchance, obvhs, tabloid, append-only-vec,
//! conservative-line-raster, frozen/nohash-hasher, priority-queue) are NOT
//! re-implemented — `reused` defines the exact game-side call surfaces the
//! skeleton bundles reference.
//!
//! All recovered code is `implemented_unverified`: offline `cargo check` proves
//! the skeleton-aligned surface closes, not original-behaviour equivalence.

extern crate alloc;

pub mod asset_libraries;
pub mod boundary;
pub mod components;
pub mod resources;
pub mod systems;

mod reused;
mod typed_gap;

pub use typed_gap::Gap;

// Re-export the game-facing reused-dep surface so game modules can spell
// `crate::reused::…` exactly as the integration owner wires the real deps.
pub mod rp {
    pub use crate::reused::*;
}
