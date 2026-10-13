//! `utils/mod.rs` — game-owned util helpers.
//!
//! Source: B skeleton `crates/country-core/src/utils/mod.rs`.
//! O evidence (Ghidra, B patched EXE):
//! - `get_res` / `get_res_mut` (O inline into rhapsody compile / render loop) —
//!   bevy-world resource access helpers.
//! - `get_last_event<E>` @0x1402b4910 / `drain_events<E>` @0x1402b47f0 —
//!   drain bevy `Events<E>` buffers (return all / last event).
//! - `world_space_to_screen_space` @0x140ac8780 — world point → screen space.
//! - `is_inside_map_bounds` (O inline into wall spatial hash).
//! - `convert_function_expr_to_title` (O).
//! - `calculate_hash` — used by `WidgetId::from_name` (O call site).
//!
//! `Mesh` here is a placeholder for the render-domain `render::mesh::Mesh`
//! (wp4); the owner integration re-points it. `Triangle` mirrors
//! `utils::geometry::intersect::Triangle` (O: used by debug volume viz).

#![allow(dead_code, unused_imports)]

pub mod audio_utils;
pub mod child_wall;
pub mod debug_viz;
pub mod dirty_tiles;
pub mod geo;
pub mod load_json;
pub mod project_cursor_onto_ray;

use crate::Gap;

/// Placeholder for the render-domain `render::mesh::Mesh` (wp4 owns the real
/// type). Game-owned `Mesh` consumers (geometry::*, utils::geo) are typed here.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Mesh {
    pub _gap: Gap,
}

/// `utils::geometry::intersect::Triangle` (O: `utils::geometry` namespace).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle(pub [glam::Vec3; 3]);

impl Triangle {
    pub fn new(a: glam::Vec3, b: glam::Vec3, c: glam::Vec3) -> Self {
        Triangle([a, b, c])
    }
}

/// `utils::calculate_hash` (O call site in `WidgetId::from_name`).
/// Exact hash algorithm U → DefaultHasher marked **I** (integration must
/// confirm the original hash so WidgetId cross-matching holds).
pub fn calculate_hash(name: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    name.hash(&mut h);
    h.finish()
}

/// `get_res` (O inline into rhapsody compile): world resource access.
pub fn get_res<T>(_world: &bevy_ecs::world::World) -> Option<&T> {
    todo!("recover utils::get_res")
}

/// `get_res_mut` (O inline into render loop).
pub fn get_res_mut<T>(_world: &mut bevy_ecs::world::World) -> Option<&mut T> {
    todo!("recover utils::get_res_mut")
}

/// `drain_events<E>` @0x1402b47f0 (O): collect + clear all events of type E.
pub fn drain_events<E>(_world: &mut bevy_ecs::world::World) -> Vec<E> {
    todo!("recover utils::drain_events")
}

/// `get_last_event<E>` @0x1402b4910 (O): last event of type E.
pub fn get_last_event<E>(_world: &mut bevy_ecs::world::World) -> Option<E> {
    todo!("recover utils::get_last_event")
}

/// `world_space_to_screen_space` @0x140ac8780 (O; body is a tiny camera
/// transform — typed gap).
pub fn world_space_to_screen_space(_world_pos: glam::Vec3) -> glam::Vec2 {
    glam::Vec2::ZERO
}

/// `is_inside_map_bounds` (O inline into wall spatial hash).
pub fn is_inside_map_bounds(_pos: glam::Vec2) -> bool {
    false
}

/// `convert_function_expr_to_title` (O).
pub fn convert_function_expr_to_title(_expr: &str) -> String {
    String::new()
}

/// `paths::config()` (O call site in `profile_file_path`): game config dir.
pub fn paths_config() -> std::path::PathBuf {
    std::path::PathBuf::from(".")
}
