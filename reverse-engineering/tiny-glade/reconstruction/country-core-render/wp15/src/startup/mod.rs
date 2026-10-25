//! `startup/mod.rs` — the game startup plugin (`add_game_startup_systems`) and
//! the shared `load_mesh_into_library` / `load_shader_into_library` helpers.
//!
//! Source: B skeleton `crates/country-core/src/startup/mod.rs`.
//!
//! O evidence (Ghidra, B patched EXE):
//! - `add_game_startup_systems` — the plugin method that registers the ~30
//!   startup systems + `init_color_library`, and queues `ClearCanvasCmd`
//!   / auto-save state; scheduling uses `chain_ignore_deferred` +
//!   `in_set_inner` (bevy schedule, reused).
//! - `load_mesh_into_library` @0x140b1aca0 — `MeshWatch::watch` + a 0x90 B
//!   mesh payload + `AssetMeshLibrary::add()` (87 callers).
//! - `load_shader_into_library<&str>` @0x140b3fe40 — clone name +
//!   `compile_shader_variants()` + `AssetShaderLibrary::add_raster()`
//!   (97 callers); `load_shader_into_library_replace_previous` @0x140b405e0
//!   (same, replace semantics).
//!
//! `load_json_as_mesh` lives in `utils::load_json` (see `utils`).

#![allow(dead_code, unused_imports)]

pub mod clearing;
pub mod debug;
pub mod glade_themes;
pub mod init_color_library;
pub mod init_photomode_state;
pub mod load_audio;
pub mod load_glade_audio;
pub mod load_glade_grading;
pub mod load_glade_tod_and_weather;
pub mod load_mesh_atlases;
pub mod load_prefabs;
pub mod load_shaders;
pub mod load_textures;
pub mod panic_reporter_startup;
pub mod spawn_brush_previews;
pub mod spawn_human_ref;
pub mod spawn_preplaced_trees;
pub mod startup_blit;
pub mod startup_critters;
pub mod startup_deferred_raster;
pub mod startup_garden_brush;
pub mod startup_grass;
pub mod startup_ivy;
pub mod startup_meadow;
pub mod startup_nani;
pub mod startup_roof;
pub mod startup_session;
pub mod startup_sheep;
pub mod startup_sky_and_sun;
pub mod startup_terrain;
pub mod startup_time_of_day;
pub mod startup_walls;
pub mod startup_water;

use crate::Gap;

/// `startup::impl$0::add_game_startup_systems` (O; bevy app plugin method).
/// Registers the startup systems; ordering/`in_set` wiring uses bevy schedule
/// APIs (reused). Full registration list U (typed gap — the exact set is the
/// consumer that drives `load_game_session`, `init_color_library`, etc.).
pub fn add_game_startup_systems(_app: &mut Gap) {
    todo!("recover add_game_startup_systems plugin registration order")
}

/// `load_mesh_into_library` @0x140b1aca0 (O): watch + `AssetMeshLibrary::add()`.
pub fn load_mesh_into_library(_name: &str, _mesh: &[u8]) {
    todo!("recover load_mesh_into_library (MeshWatch::watch + AssetMeshLibrary::add)")
}

/// `load_shader_into_library<&str>` @0x140b3fe40 (O): compile variants + add.
pub fn load_shader_into_library(_name: &str, _source: &str) {
    todo!("recover load_shader_into_library (compile_shader_variants + AssetShaderLibrary::add_raster)")
}

/// `load_shader_into_library_replace_previous<&str>` @0x140b405e0 (O).
pub fn load_shader_into_library_replace_previous(_name: &str, _source: &str) {
    todo!("recover load_shader_into_library_replace_previous")
}
