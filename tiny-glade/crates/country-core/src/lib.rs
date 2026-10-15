//! Game-facing rendering declarations and recovered CPU domain rules.
//! Modules follow the original B functional source tree. GPU recording is
//! supplied through explicit backend traits; see README.md for unresolved seams.
extern crate alloc;
pub mod components;

pub mod rhapsody_types;

pub mod utils_contract;

pub mod asset_libraries;
pub mod frozen_map;
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
    pub mod backend;
    pub mod camera_rig;
    pub mod render_loop;
    pub mod wall;
}

pub mod recovered_building;
pub mod recovered_systems;
