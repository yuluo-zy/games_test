//! `signifier_mark_for_delete.rs` — 指示符删除（B 0x140929160 `signfier_mark_for_delete`
//! / 0x140929a90 `despawn_marked`，O；PDB 拼写 `signfier`）。
//!
//! 证据（2026-10-17 fresh 反编译/机器码 0x140929160）：
//! - 系统签名（PDB）：
//!   `Res<UiState>`,
//!   `Query<(Entity, &SignifierId, Option<&SignifierMinimumLifespan>),
//!           Without<SignifierMarkedForDeletion>>`,
//!   `Commands`；
//! - `UiState+0x18 != 0` 时，遍历 `*(UiState+0x20)+0x18` 的 0x28 B 条目，逐条
//!   `calculate_hash<(WidgetId, Option<&str>)>` 并 `insert<WallId,()>` 建 hash 表；
//! - 遍历查询实体：把实体 `SignifierId`（`&GlobalId`）hash 后做 hashbrown
//!   `contains`——**不在表中**且（无 `SignifierMinimumLifespan`，或
//!   `min_lifespan@+4 <= t@+0`）时 `insert(SignifierMarkedForDeletion)`；
//! - `despawn_marked` @0x140929a90：遍历带标记实体 `Commands::despawn`（已判等）。
//!
//! GAP（登记）：crate `UiState` 未落库 signifier 配置表（`*(+0x20)` 的
//! `(WidgetId, Option<&str>)` 列表），因此 live-hash 集合只能以空集参与判等；
//! 解锁条件：UiState 暴露该配置表的 hash 迭代面。
use bevy_ecs::prelude::*;

use crate::systems::ui::resources::ui_state::UiState;

use super::signifier_create::SignifierId;

/// 已标记删除的指示符实体（O：`SignifierMarkedForDeletion` 组件标记）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct SignifierMarkedForDeletion;

/// 指示符最短寿命（O：组件 8 B；`min_lifespan @+4`、`t @+0`）。
///
/// 删除条件（机器码：`movss xmm0,[rdi+4]; ucomiss xmm0,[rdi]; ja 跳过`）：
/// `min_lifespan <= t` 时打删除标记。
#[repr(C)]
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct SignifierMinimumLifespan {
    /// +0x00：计时/已存活时间（U：字段名按比较语义取）。
    pub t: f32,
    /// +0x04：最短寿命。
    pub min_lifespan: f32,
}

/// `signfier_mark_for_delete` @0x140929160（O）：按 live-hash 表 + 寿命对匹配实体打标记。
pub fn signfier_mark_for_delete(
    ui_state: Res<UiState>,
    query: Query<
        (Entity, &SignifierId, Option<&SignifierMinimumLifespan>),
        Without<SignifierMarkedForDeletion>,
    >,
    mut commands: Commands,
) {
    // O：`UiState+0x18 != 0` 时重建 live-hash 表。
    // GAP：UiState 的 `(WidgetId, Option<&str>)` 配置表未落库 → 集合恒为空（见模块文档）。
    let rebuild = ui_state.interaction().native_tag != 0;
    let live: std::collections::HashSet<u64> = if rebuild {
        std::collections::HashSet::new()
    } else {
        std::collections::HashSet::new()
    };

    for (entity, id, lifespan) in &query {
        // O：`contains(hash(&SignifierId))`；不在表中（或表为空）才继续。
        let is_live = live.contains(&id.0);
        // O：`RDI == None`（无最短寿命组件）→ 命中；否则 `min_lifespan <= t` 才命中。
        let expired = lifespan.map_or(true, |l| l.min_lifespan <= l.t);
        if !is_live && expired {
            commands.entity(entity).insert(SignifierMarkedForDeletion);
        }
    }
}

/// `despawn_marked` @0x140929a90（O）：销毁所有带删除标记的指示符实体。
pub fn despawn_marked(
    query: Query<(Entity, &SignifierMarkedForDeletion)>,
    mut commands: Commands,
) {
    for (entity, _) in &query {
        commands.entity(entity).despawn();
    }
}
