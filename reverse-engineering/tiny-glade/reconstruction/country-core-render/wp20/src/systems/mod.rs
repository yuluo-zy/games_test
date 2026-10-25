//! `systems` — WP20 systems (camera / remaining-UI / time-weather /
//! backend-input). Skeleton-aligned to
//! `skeleton/B/crates/country-core/src/systems/`; contracts in `contracts/*.md`.
//!
//! - `backend::input`   WP20 input-backend systems (7 units).
//! - `camera_rig`       CameraBody trait + Free/Normal/Walk/Butterfly bodies.
//! - `main_camera` / `look_at_camera`   MainCamera resource + per-mode dispatch.
//! - `time_of_day`/`time_update`/`weather_system`/`world_raster`/
//!   `timelapse_replay`/`mode_manager`/`prefab`   time & world systems.
//! - `ui_2d_follow_mouse`/`ui_camera_mode_gizmos`/`ui_edit_focus`/... +
//!   `ui_clutter/*`   remaining UI systems.
//! - `arrow_gizmo_animation`/`brush_preview`/`clear_canvas`/`despawn`/
//!   `mesh_follow_terrain`/`mod_key`/`utils`/`utils_intersect`   misc.
//!
//! `ui` and `backend` are facade files (inline typed-gap stubs of the
//! wp15/wp16-owned surfaces); the real country-core `systems::ui`/`systems::
//! backend` modules replace them at integration. `collision` is wp16-owned and
//! intentionally NOT re-declared here.

pub mod arrow_gizmo_animation;
pub mod backend;
pub mod brush_preview;
pub mod camera_rig;
pub mod clear_canvas;
pub mod despawn;
pub mod look_at_camera;
pub mod main_camera;
pub mod mesh_follow_terrain;
pub mod mod_key;
pub mod mode_manager;
pub mod prefab;
pub mod time_of_day;
pub mod time_update;
pub mod timelapse_replay;
pub mod ui;
pub mod ui_2d_follow_mouse;
pub mod ui_camera_mode_gizmos;
pub mod ui_clutter;
pub mod ui_edit_focus;
pub mod ui_edit_freehand_walls;
pub mod ui_focus_mode_shape_height;
pub mod ui_move_freehand_walls;
pub mod ui_move_shapes;
pub mod ui_raise_shape;
pub mod utils;
pub mod utils_intersect;
pub mod weather_system;
pub mod world_raster;

/// `GameSystemSet` (O: enum-iterator driven `configure_sets(Update)` in
/// `systems::utils::add_stages`; variant set U — the bevy system-set labels
/// used across the WP20 systems).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GameSystemSet {
    Camera,
    Time,
    WorldRaster,
    BackendInput,
    Ui,
    Timelapse,
    Prefab,
    ModeManager,
    Misc,
}
