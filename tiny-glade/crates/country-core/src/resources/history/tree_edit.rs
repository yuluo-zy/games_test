//! B `resources/history/tree_edit.rs` — 树编辑负载（章节 O；25 函数）。
//!
//! `TreeEdit`：{Place/Move} + tree{id, position, scale, rotation_degrees, prefab_name,
//! is_preplaced}；missing_prefab 0x1409e6210；TreeCanopyMaxCullDistance。
//! serde 0x14003d4e0/0x1417c2770/0x1417c2600/0x1417c2a50、deserialize 0x1417c2f60。
//! 树编辑为普通编辑（无独立 replay 文件；place/move 数据恢复）。
use glam::Vec3;

/// 树编辑操作。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[repr(u8)]
pub enum TreeEditKind {
    #[default]
    Place = 0,
    Move = 1,
}

/// 树数据载荷。
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TreeEditData {
    pub id: u64,
    pub position: Vec3,
    pub scale: f32,
    /// 度制旋转（字段序按章节）。
    pub rotation_degrees: f32,
    pub prefab_name: String,
    pub is_preplaced: bool,
}

/// 树编辑负载。
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TreeEdit {
    Place(TreeEditData),
    Move(TreeEditData),
}
impl Default for TreeEdit {
    fn default() -> Self {
        Self::Place(TreeEditData::default())
    }
}
