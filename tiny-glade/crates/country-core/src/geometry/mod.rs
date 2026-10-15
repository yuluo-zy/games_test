//! `country_core::geometry` — 程序化几何与砖实例墙载体（B `geometry::*`）。
//!
//! 已做：`cube`、`plane`、`sphere`（汇编逐条还原；`sphere` 复用 `hexasphere 12.0.0`）、`instanced_wall`。
pub mod cube;
pub mod instanced_wall;
pub mod plane;
pub mod sphere;
