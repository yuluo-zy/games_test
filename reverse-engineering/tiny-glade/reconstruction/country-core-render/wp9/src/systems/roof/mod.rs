//! systems/roof/mod.rs — system_roof crate 屋顶系统族（WP9）。
//!
//! 来源：PDB 骨架 `crates/systems/roof/src/*.rs`（B）。本树镜像 `reconstruction/
//! country-core` 装配布局；集成 owner 接线时把本族并入 country-core（如 WP6 模式）。
//! 业务逻辑合同见 `contracts/roof-systems.md`。

#![allow(dead_code, unused_imports)]

pub mod create_roof;
pub mod deferred_record_roof_history;
pub mod delete_roof;
pub mod edit_roof;
pub mod edit_rules;
pub mod highlight_roof;
pub mod replay_roof;
pub mod roof_trigger_volume;
pub mod simulate_roof_tile_for_audio;
pub mod ui_edit_roof;
pub mod ui_edit_roof_ridge_dir;
pub mod ui_edit_roof_ridge_length;
pub mod update_roof_spatial;
pub mod utils;
pub mod visual;
pub mod wall_height_dt;
