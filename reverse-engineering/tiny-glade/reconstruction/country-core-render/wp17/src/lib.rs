//! WP17 — Tiny Glade country-core 历史回放域（history replay domain）骨架对齐还原。
//!
//! B 版本（EXE sha256 cf048e27…，PDB sha256 2d9686b8…）。范围：
//!   - `systems::history::replay_*`（11 个回放系统 skeleton 单元）：
//!     finish_replay / replay_mode / signifier_replay_active /
//!     replay_wall / replay_terrain / replay_auto_clutter / replay_stairs /
//!     replay_raster(+path/water/garden) / replay_pop_bucket。
//!   - `resources::history::replay_cmd_buffer` 与 10 个编辑历史容器
//!     （wall_history / wall_compound_edit / auto_clutter_edit / color_edit /
//!     decorator_edit / path_edit / stairs_history_edit / terrain_edit /
//!     tree_edit + replay_cmd_buffer）。
//!
//! 业务逻辑合同见 `contracts/{replay-orchestration,replay-systems,
//! edit-history-types}.md`（Observed/Inferred/Unknown + typed gap）。
//! 本 crate 是独立离线 `cargo check` 门禁；不 patch 共享 `country-core` crate
//! （集成 owner 整合）。wp13 的 `History` 核心（edit_chain/snapshots/pending/
//! cursor/history_start/nonce）+ `reconstruction/history` + `tg-history`
//! 作为已恢复边界复用，本域只扩展回放面。

pub mod resources;
pub mod systems;

/// Typed-gap 标记：机器码/PDB 未恢复布局与语义的字段。从不臆造字段。
#[derive(
    Clone, Copy, Default, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct Gap;

/// 别名模块，使各模块可写 `crate::typed_gap::Gap`。
pub mod typed_gap {
    pub use crate::Gap;
}
