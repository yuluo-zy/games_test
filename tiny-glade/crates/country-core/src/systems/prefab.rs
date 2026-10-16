//! prefab.rs — 预制体实例契约（组 A restorer 接线；B `systems/prefab/` 的
//! `SpawnPrefabCmd`/`DespawnPrefabCmd`/`PrefabInstanceData`/`PrefabInstance`，
//! forest/tree-tool 的树生成/回收消费面）。
//!
//! 原模块为大型 prefab 系统（资产加载/实例化/回收）；本处只落地本组 crate 消费的
//! 类型合同（组件 + 命令事件），实例化宿主接线待续。字段布局 U（typed gap）。

use bevy_ecs::{component::Component, entity::Entity, event::Event};

/// 预制体实例引用（权威定义在 `resources::prefab`）。
pub use crate::resources::prefab::PrefabInstance;
use glam::Vec3;

/// 预制体实例数据组件（O：`systems::prefab::PrefabInstanceData`；forest
/// handplaced_tree_animation 0x142279500 / system_tree 各系统以 `&mut` 改位置
/// 缩放。字段 = {位置/缩放/旋转/预制体名…}，语义 I，布局 U）。
#[derive(Component, Clone, Debug, Default)]
pub struct PrefabInstanceData {
    /// 世界位置（I）。
    pub position: Vec3,
    /// 缩放（I）。
    pub scale: f32,
    /// 度制旋转（I）。
    pub rotation_degrees: f32,
    /// typed gap：其余实例字段（U）。
    pub _gap: u64,
}

/// 生成预制体命令（O：`systems::prefab::SpawnPrefabCmd`；forest manage_tree_prefabs
/// 0x141bf16c0 与 system_tree process_tree_cmds 发射。载荷语义 U）。
#[derive(Event, Clone, Debug, Default)]
pub struct SpawnPrefabCmd {
    /// typed gap。
    pub _gap: u64,
}

/// 回收预制体命令（O：`systems::prefab::DespawnPrefabCmd`；despawn_trees
/// 0x1422686d0 发射。载荷语义 U）。
#[derive(Event, Clone, Debug, Default)]
pub struct DespawnPrefabCmd {
    /// 目标实体（I：命令载荷族 {实体,…}）。
    pub entity: Option<Entity>,
    /// typed gap。
    pub _gap: u64,
}
