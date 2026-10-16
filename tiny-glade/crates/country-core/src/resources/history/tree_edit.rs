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
    /// 删除（O：0x58B 树命令变体 2；reviewer-A 审查结论——删除语义不得以
    /// Move 静默冒充，组 A 修复补入）。
    Delete = 2,
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
    /// 删除（O：变体 2）。
    Delete(TreeEditData),
}
impl Default for TreeEdit {
    fn default() -> Self {
        Self::Place(TreeEditData::default())
    }
}

// ---------------------------------------------------------------- 组 A 接线

/// 树 canopy 最大剔除距离（O：`tree_edit::TreeCanopyMaxCullDistance` 载荷
/// `(pub f32)`；render 域 `draw_callbacks_b` 记为无 Resource 的 f32 载荷——本处
/// 按 BASELINE 参数面 `ResMut<TreeCanopyMaxCullDistance>`（forest::tree_animation
/// 0x142279500）落地为 Resource，差异登记（render 侧视图待同步）。
#[derive(bevy_ecs::resource::Resource, Clone, Copy, Debug, PartialEq)]
pub struct TreeCanopyMaxCullDistance(pub f32);

impl Default for TreeCanopyMaxCullDistance {
    fn default() -> Self {
        Self(0.0)
    }
}

/// 树编辑命令事件族（O，BASELINE 参数面；`system_tree`/`forest` 消费）。
/// 载荷语义 U（0x58B 树命令记录族见 re-analyst-A 契约 §2），按类型合同登记。

/// 放置树命令（O：`tree_edit::InsertTreeCmd`；ui_place_tree 0x142269170 发射、
/// process_tree_cmds 0x142267430 消费。载荷 = 树 id + 位置 + 缩放 + prefab 名
/// （0x18B/0x48B 记录族，组 A 修复按真实载荷落地）。
#[derive(bevy_ecs::event::Event, Clone, Debug, Default)]
pub struct InsertTreeCmd {
    /// 树 id（O：TreeStorage::next_index 产出）。
    pub id: u64,
    /// 世界位置。
    pub position: glam::Vec3,
    /// 缩放（O：lerp(0.4922, 1.0) 族）。
    pub scale: f32,
    /// prefab 名（O：random_tree_prefab 分派）。
    pub prefab_name: std::string::String,
}

/// 删除树命令（O：`tree_edit::DeleteTreeCmd`；ui_delete_tree 0x141bd2510 /
/// hide_trees / convert_preplaced_trees 发射、process_tree_cmds 消费。
/// 载荷 = 真实树 id——组 A 修复：删除链按 TreeStorage 遍历/索引，禁止假 id）。
#[derive(bevy_ecs::event::Event, Clone, Debug, Default)]
pub struct DeleteTreeCmd {
    /// 树 id（O：TreeStorage 索引）。
    pub id: u64,
}

/// 预置树批量生成命令（O：`tree_edit::SpawnPreplacedTrees`；convert_preplaced_trees
/// 0x14226be70 消费）。
#[derive(bevy_ecs::event::Event, Clone, Debug, Default)]
pub struct SpawnPreplacedTrees {
    /// typed gap。
    pub _gap: u64,
}

/// 树回放命令（O：`tree_edit::TreeReplayCmd`；`ReplayCommand::Tree` 载荷，
/// `system_tree::replay_tree` 0x141bc34f0 消费。命令记录 = 0x58B
/// {1, 1, 变体, 树 id, …} 族，re-analyst-A 契约 §2；变体 0=放置/1=移动/2=删除）。
#[derive(Clone, Debug, Default)]
pub struct TreeReplayCmd {
    /// 回放编辑数据。
    pub edit: TreeEdit,
    /// 撤销方向。
    pub undo: bool,
    /// 即时模式（时间动画跳过）。
    pub instant: bool,
}

impl TreeReplayCmd {
    /// 桶展开（O 面：变体 0/1/2 → 命令分派由 replay_tree 执行）。
    pub fn expanded(edit: &TreeEdit, undo: bool, instant: bool) -> Self {
        Self {
            edit: edit.clone(),
            undo,
            instant,
        }
    }

    /// `make_instant`（O：ReplayBuffer::make_animations_instant 遍历面）。
    pub fn make_instant(&mut self) {
        self.instant = true;
    }
}

impl TreeEdit {
    /// 编辑种类（O：变体 0=Place / 1=Move / 2=Delete）。
    pub fn kind(&self) -> TreeEditKind {
        match self {
            Self::Place(_) => TreeEditKind::Place,
            Self::Move(_) => TreeEditKind::Move,
            Self::Delete(_) => TreeEditKind::Delete,
        }
    }

    /// 树 id（O：载荷字段）。
    pub fn id(&self) -> u64 {
        match self {
            Self::Place(data) | Self::Move(data) | Self::Delete(data) => data.id,
        }
    }
}

impl TreeEdit {
    /// 回放位置（I：TreeEditData 载荷面；`system_tree::replay_tree` 消费）。
    pub fn edit_position(&self) -> glam::Vec3 {
        match self {
            Self::Place(d) | Self::Move(d) | Self::Delete(d) => d.position,
        }
    }
    /// 回放缩放（I）。
    pub fn edit_scale(&self) -> f32 {
        match self {
            Self::Place(d) | Self::Move(d) | Self::Delete(d) => d.scale,
        }
    }
    /// 回放 prefab 名（I）。
    pub fn prefab_name(&self) -> &str {
        match self {
            Self::Place(d) | Self::Move(d) | Self::Delete(d) => &d.prefab_name,
        }
    }
}
