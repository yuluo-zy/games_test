//! 墙↔屋顶关联事件（systems/wall/wall_state/wall_roof_hashmap.rs）。
//!
//! 原 B 模块路径（skeleton 富化/PDB）：`country_core::systems::wall::
//! wall_state::wall_roof_hashmap`，宿主 `RoofIsInvalid`/
//! `RecalculateGableRoofWallHeights`/`RecalculateWallBricks` 等事件。
//! 重建把事件归入 `resources::roofs::commands`；本模块按原路径再导出，
//! 供 system-color 等原路径消费方接线。
pub use crate::resources::roofs::commands::{
    RecalculateWallBricks, RecalculateWallHoles, RoofIsInvalid, WallGableChanged,
};
