//! `render/mesh_atlas.rs` -- 仅为兼容旧路径保留。
//!
//! 原版里 `AtlassedMeshName` 定义在 `resources::render::mesh_atlas_library`（O：`impl$10::fmt`
//! 0x140a3d2e0 的 PDB 路径），旧版在这里放了一份 `AtlassedMeshName(pub u64)` 占位。
//! 现统一成一份，这里只做 re-export。

pub use crate::resources::render::mesh_atlas_library::AtlassedMeshName;
