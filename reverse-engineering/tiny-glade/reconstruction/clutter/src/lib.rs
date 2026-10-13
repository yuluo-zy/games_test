//! system-clutter 自动杂物（autoclutter）模块恢复草稿（B 主基线，反汇编级证据）
//!
//! 依据：evidence/multi-baseline/clutter-placement-analysis.md。
//! 原模块：crates/systems/clutter/src/（lib.rs + autoclutter/mod.rs，B SHA 119507C2）。
//! A 前身目录 clutter/（ClutterState/ClutterRegion/ClutterCandidateIndex），B 改名 autoclutter/
//! （AutoClutterState/AutoClutterRegion/ClutterCandidateIndex），SHA 随版本演进。
//!
//! 证据链（B VA）：
//! build 0x140652050 → populate_autoclutter_regions 0x1422d6ed0(44KB)
//!   → process_autoclutter_candidates 0x1422ce260 → add_autoclutter_around_gates 0x1422cc390
//!   → add_autoclutter_around_windows 0x1422cefe0 → add_autoclutter_on_windows 0x1422cd7d0
//!   → add_birdnests 0x1422c9c70 → add_lantern_water_setdresssing 0x1422d2fa0
//!   → add_tree_setdressing 0x1422d3800 → manage_autoclutter_entities 0x1422d04e0
//! host_geometry：load_clutter_meshes 0x1422c0760 / inter_roof_merlons 0x1422d4870+0x141ce3a70 /
//! inter_shape_stitches 0x141cd1a90+0x141ce65b0+0x141ce7190（见 clutter-extra-analysis.md，
//! 第二批补全语义）。
//!
//! 本草稿非可编译还原：依赖类型（WaterRaster/SessionMeta/DirtyTiles/BushGrid/Window/Gate/
//! TerrainDecorator/TreeEntry/ColorId 等）字段与组件绑定为 country_core 边界点，待组件模块
//! 恢复后接线；大量字段保留 unknown。不以孤立 crate 交付（遵循"不新建孤立 Cargo"）。

pub mod autoclutter;
pub mod auto_clutter_edit;
pub mod merlons;
pub mod stitches;
