//! `resources::history` — 历史编辑记录资源（WP17 扩展）。
//!
//! wp13 已恢复权威 `History` 核心（edit_chain / snapshots / pending /
//! history_start / nonce / cursor；undo/redo 内联进 history_process
//! 0x140af5b30）。本模块在其上补充回放域的编辑记录容器：
//!
//!   - `replay_cmd_buffer`：ReplayCmd / ReplayBucket / ReplayBuffer
//!   - `wall_history`：墙高度历史事件处理（ev_new_wall / ev_height_changed）
//!   - `wall_compound_edit`：`WallHistoryCompoundEdit` + `CompoundEditBuilder`
//!   - `auto_clutter_edit` / `color_edit` / `decorator_edit` / `path_edit` /
//!     `stairs_history_edit` / `terrain_edit` / `tree_edit`：各领域编辑记录
//!
//! 外层 `EditType` 判别与 `MultiEdit`（40B = edits Vec + 三个辅助计数）沿用
//! wp13；辅助容器（decorators/stairs）负载为 `Arc<DecoratorHistoryDiff>` /
//! `Arc<StairsHistoryDiff>`（O：replay_edit 0x140af6170 的 Arc 处理）。

pub mod auto_clutter_edit;
pub mod color_edit;
pub mod decorator_edit;
pub mod path_edit;
pub mod replay_cmd_buffer;
pub mod stairs_history_edit;
pub mod terrain_edit;
pub mod tree_edit;
pub mod wall_compound_edit;
pub mod wall_history;

pub use crate::typed_gap::Gap;

/// 回放相关判空谓词（O：PDB 名；wp13 已提供骨架）：
pub use crate::typed_gap::Gap as HistoryDiffGap;

/// 编辑命令类型（O：观察 JSON 13 类外层 tag + PDB constructor）。
/// 与 wp13 共享 crate 的定义一致（复用边界；不重复实现 History 核心）。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EditType {
    Wall,
    WallNewSystem,
    Roof,
    DecoratorV5,
    Terrain,
    Tree,
    Color,
    Stairs,
    Path,
    Water,
    Garden,
    Cobblestone1,
    Cobblestone2,
    Cobblestone3,
    /// 未证 tag（开放枚举；观察 schema 不封闭）。
    Unknown,
}

impl EditType {
    /// 由外层命令 tag 投影（O：观察 JSON 13 类外层 tag）。
    pub fn from_observed_tag(tag: &str) -> Self {
        match tag {
            "Wall" => EditType::Wall,
            "WallNewSystem" => EditType::WallNewSystem,
            "Roof" => EditType::Roof,
            "DecoratorV5" => EditType::DecoratorV5,
            "Terrain" => EditType::Terrain,
            "Tree" => EditType::Tree,
            "Color" => EditType::Color,
            "Stairs" => EditType::Stairs,
            "Path" => EditType::Path,
            "Water" => EditType::Water,
            "Garden" => EditType::Garden,
            "Cobblestone1" => EditType::Cobblestone1,
            "Cobblestone2" => EditType::Cobblestone2,
            "Cobblestone3" => EditType::Cobblestone3,
            _ => EditType::Unknown,
        }
    }
}

/// 墙 ID（O：`resources::walls::WallId`；此处为 skeleton 边界，布局 U）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct WallId(pub u64);

/// 装饰 ID（O：`resources::walls::decorator::DecoratorId`）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct DecoratorId(pub u64);

/// 楼梯图边（O：`stairs_state::StairGraphEdge` 两整数对）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct StairGraphEdge(pub (u64, u64));
