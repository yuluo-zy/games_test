//! Game-facing rendering declarations and recovered CPU domain rules.
//! Modules follow the original B functional source tree. GPU recording is
//! supplied through explicit backend traits; see README.md for unresolved seams.
extern crate alloc;
pub mod components;

pub mod rhapsody_types;

pub mod utils_contract;

pub mod asset_libraries;
pub mod frozen_map;
pub mod game_config;
pub mod app_loading_state;
pub mod game_profile;
pub mod geometry;
pub mod input;
pub mod meta_commander;
pub mod name_hash;
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct Gap;
pub mod render;
pub mod reused;

pub mod rhapsody_contract;
pub mod typed_gap {
    pub use crate::Gap;
}
pub use systems::render_loop as render_loop_ctx;

pub mod resources;
pub mod startup;
pub mod utils;

pub mod systems {
    pub mod audio;
    pub mod backend;
    pub mod brush_preview;
    pub mod debug;
    pub mod camera_rig;
    pub mod collision;
    pub mod ivy;
    pub mod main_camera;
    pub mod mode_manager;
    pub mod paths;
    pub mod render_loop;
    pub mod weather_system;
    pub mod world_raster;
    pub mod history;
    pub mod garden;
    pub mod ui;
    pub mod ui_clutter;
    pub mod ui_edit_focus;
    pub mod prefab;
    pub mod terrain;
    pub mod time_of_day;
    pub mod time_update;
    pub mod save;
    pub mod steam;
    pub mod wall;
}

