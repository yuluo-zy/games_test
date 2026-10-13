//! WP20 — country-core camera / remaining-UI / time-weather / backend-input
//! systems (Tiny Glade, version B, `ljxsj_92385`; EXE sha256 `cf048e27…`,
//! PDB sha256 `2d9686b8…`).
//!
//! Business-logic contracts (O/I/U + typed gaps) live in `contracts/*.md`.
//! This crate is the standalone offline `cargo check` gate for the skeleton-
//! aligned Rust surface delivered to the country-core integration owner.
//!
//! Scope recovered here (each module doc names its PDB bundle + RVAs):
//!   - `systems::backend::input`    calculate_input_delta / input_record_replay /
//!                                  input_replay_for_trailers / reset_input_state /
//!                                  translate_input / translate_input_to_cursor_moved /
//!                                  update_cursor_position.
//!   - `systems::camera_rig`        CameraBody trait + FreeCamera / NormalCamera /
//!                                  WalkCamera / ButterflyCamera (dolly-based;
//!                                  business logic reconciled with the prior
//!                                  machine-code-verified `reconstruction/camera`).
//!   - `systems::main_camera` / `systems::look_at_camera`   the 0x4e0 B MainCamera
//!                                  resource + per-mode body dispatch + snapshot.
//!   - `systems::{time_of_day, time_update, weather_system, world_raster,
//!     timelapse_replay, mode_manager, prefab}`  game clock / sun / weather /
//!                                  raster upload / timelapse / mode FSM / prefabs.
//!   - `systems::ui_*` + `systems::ui_clutter/*`  remaining UI systems.
//!   - `systems::{arrow_gizmo_animation, brush_preview, clear_canvas, despawn,
//!     mesh_follow_terrain, mod_key, utils, utils_intersect}`  misc systems.
//!
//! Reused-dependency surfaces absent from the offline registry cache
//! (bevy_app/bevy_state/bevy_input/bevy_time at 0.16.0, puffin, enum-iterator,
//! append-only-vec, egui, suncalc, hexasphere) are declared in `reused` —
//! game-side call surfaces only, no re-implementation. wp15/16-owned types
//! (`input::*`, `systems::ui`, `resources::*`, `utils::components::transform`)
//! are declared as typed-gap integration markers in the `src/{input,resources,
//! utils,asset_libraries}.rs` facade files; the integration owner replaces
//! them with the real country-core modules. None of the facades is recovered
//! game logic.
//!
//! All recovered code is `implemented_unverified`: offline `cargo check` proves
//! the skeleton-aligned surface closes, not original-behaviour equivalence.

extern crate alloc;

pub mod app_loading_state;
pub mod asset_libraries;
pub mod game_config;
pub mod input;
pub mod meta_commander;
pub mod resources;
pub mod reused;
pub mod systems;
pub mod utils;

/// Typed-gap marker: a field whose layout/semantics were not recovered from the
/// original machine code. Never fabricated from PDB `void()` placeholders.
///
/// NOTE: in this standalone gate `Gap` also derives bevy `Resource` /
/// `Component` / `Event` so the many `Res<Gap>` placeholder system parameters
/// compile offline. The integration owner swaps each placeholder for the real
/// country-core type; the shared country-core `Gap` (lib.rs) stays unadorned.
#[derive(
    Clone, Copy, Default, Debug, PartialEq, Eq, Hash,
    bevy_ecs::resource::Resource,
    bevy_ecs::component::Component,
    bevy_ecs::event::Event,
    serde::Serialize, serde::Deserialize,
)]
pub struct Gap;

/// Alias module so modules can write `crate::typed_gap::Gap`.
pub mod typed_gap {
    pub use crate::Gap;
}
