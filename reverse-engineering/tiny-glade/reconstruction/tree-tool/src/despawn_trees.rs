//! tree-tool：despawn_trees 业务恢复草稿（B 主基线，反汇编级证据）
//!
//! 依据：evidence/multi-baseline/tree-tool-rest-analysis.md（2026-10-13 reva/radare2
//! 精读 0x1422686d0）。这是 process_tree_cmds Delete 分支的实际销毁方：
//! Delete 分支只打 TreeMarkedForDeletion 标记 + 播删除音效，本系统逐帧扫描
//! 标记树，动画进度低于阈值时发送 DespawnPrefabCmd。
//!
//! 硬证据：
//! - bevy system_type 包装器 0x141bb2110 签名 =
//!   Query<(Entity, &TreeAnimation, &TreeMarkedForDeletion)> + EventWriter<DespawnPrefabCmd>。
//! - 反汇编：UCOMISS XMM6(0.01), f@TreeAnimation+4 ; JBE 跳过 → 发送。
//!   阈值常量 0.01f = 0x3c23d70a @ 0x142960674（radare2 确认）。
//! - 事件槽 stride 0x10：slot+0 = bevy 事件 id（内部），slot+8 = Entity u64。
//!
//! 依赖复用：bevy_ecs 0.16 直接复用。TreeAnimation/TreeMarkedForDeletion 为
//! country_core::components 组件，内部字段 unknown 处不伪造。

use bevy_ecs::event::EventWriter;
use bevy_ecs::prelude::*;
use bevy_ecs::query::Query;

/// 树动画组件（country_core::components::TreeAnimation）。
/// 本轮证据：size 0x10；f32 @+4 与 0.01 阈值比较（despawn 门控）。
/// 其余字段 unknown。
pub struct TreeAnimation {
    /// [evidence] f32 @+4：动画进度/淡出值，< 0.01 触发销毁。字段名 unknown。
    pub despawn_progress: f32,
    /// @+0 及 @+8..+0x10 未确认（unknown）。
    pub unknown: [u8; 12],
}

/// 标记待删除组件（country_core::components::TreeMarkedForDeletion）。
/// Delete 分支经 Commands 推入；疑似零尺寸标记组件（unknown）。
pub struct TreeMarkedForDeletion {
    pub _private: [u8; 0],
}

/// 销毁预制体命令（country_core::systems::prefab::DespawnPrefabCmd）。
/// 事件槽 stride 0x10；payload u64 @slot+8（@slot+0 为 bevy 事件 id）。
pub struct DespawnPrefabCmd {
    /// [evidence] payload = 被销毁树的 Entity（查询实体列读取，u64）。
    pub entity: Entity,
}

/// 扫描标记删除的树，动画进度 < 0.01 时发送 DespawnPrefabCmd。
///
/// 签名来自 bevy system_type 包装器（硬证据）。PDB size 751（0x2EF），
/// 源文件 process_tree_cmds.rs 203–214 行。
pub fn despawn_trees(
    mut marked_trees: Query<(Entity, &TreeAnimation, &TreeMarkedForDeletion)>,
    mut despawn_prefab: EventWriter<DespawnPrefabCmd>,
) {
    for (entity, tree_animation, _marked) in &mut marked_trees {
        // [evidence] UCOMISS XMM6(0.01), f32@TreeAnimation+4 ; JBE 跳过。
        // 发送当且仅当 f < 0.01（严格小于，NaN 不发送）。
        if tree_animation.despawn_progress < 0.01 {
            despawn_prefab.send(DespawnPrefabCmd { entity });
        }
    }
}
