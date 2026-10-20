//! 指示符回收：本帧不再请求且已满足最短寿命时，标记并销毁实体。
//! 活动集合由创建系统统一维护；保留历史拼写 signfier 以兼容现有调用。
use bevy_ecs::prelude::*;

use super::signifier_create::{SignifierId, UiSignifierStream};

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
    stream: Res<UiSignifierStream>,
    query: Query<
        (Entity, &SignifierId, Option<&SignifierMinimumLifespan>),
        Without<SignifierMarkedForDeletion>,
    >,
    mut commands: Commands,
) {
    for (entity, id, lifespan) in &query {
        let is_live = stream.is_active(*id);
        // O：`RDI == None`（无最短寿命组件）→ 命中；否则 `min_lifespan <= t` 才命中。
        let expired = lifespan.is_none_or(|lifespan| lifespan.min_lifespan <= lifespan.t);
        if !is_live && expired {
            commands.entity(entity).insert(SignifierMarkedForDeletion);
        }
    }
}

/// `despawn_marked` @0x140929a90（O）：销毁所有带删除标记的指示符实体。
pub fn despawn_marked(query: Query<(Entity, &SignifierMarkedForDeletion)>, mut commands: Commands) {
    for (entity, _) in &query {
        commands.entity(entity).despawn();
    }
}
