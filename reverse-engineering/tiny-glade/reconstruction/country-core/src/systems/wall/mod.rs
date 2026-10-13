//! country_core::systems::wall — the wall-authoring ECS systems (WP12).
//!
//! Mirrors the B skeleton `crates/country-core/src/systems/wall/` (57 source units):
//!   - root: calc_wall_bend, deferred_wall_history, delete_wall, draw_wall, finish_wall,
//!     merge_walls, set_wall_style, structure_outline, ui_continue_wall, ui_draw_wall,
//!     upload_instanced_wall_ssbo, wall_highlight, wall_splitter
//!   - adjust_wall_height/, eraser/, shapes/, snapping/, valid_enclosure/, wall_state/
//!
//! State types (InnerWalls / PublicWalls / WallState / ...) are WP11 scope and referenced
//! via `crate::resources::walls::*` typed-gap facades. All systems are `implemented_unverified`.

pub mod adjust_wall_height;
pub mod calc_wall_bend;
pub mod deferred_wall_history;
pub mod delete_wall;
pub mod draw_wall;
pub mod eraser;
pub mod finish_wall;
pub mod merge_walls;
pub mod set_wall_style;
pub mod shapes;
pub mod snapping;
pub mod structure_outline;
pub mod ui_continue_wall;
pub mod ui_draw_wall;
pub mod upload_instanced_wall_ssbo;
pub mod valid_enclosure;
pub mod wall_highlight;
pub mod wall_splitter;
pub mod wall_state;
