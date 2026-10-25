//! 指示符回收：本帧不再请求且已满足最短寿命时，标记并销毁实体。
//! 活动集合由创建系统统一维护；保留历史拼写 signfier 以兼容现有调用。
use bevy_ecs::prelude::*;

use super::signifier_create::{SignifierId, record_id};
use crate::systems::ui::resources::ui_state::UiState;
use std::collections::HashSet;

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
    ui: Res<UiState>,
    query: Query<
        (Entity, &SignifierId, Option<&SignifierMinimumLifespan>),
        Without<SignifierMarkedForDeletion>,
    >,
    mut commands: Commands,
) {
    // Original local set is rebuilt from the captured active widget's actual
    // factory list. No persistent Stream.active_ids table exists.
    let active: HashSet<u64> = if ui.hover_byte() != 0 {
        let widget = ui.active_widget();
        ui.active_params()
            .unwrap()
            .signifiers
            .iter()
            .map(|factory| record_id(widget, factory.name()))
            .collect()
    } else {
        HashSet::new()
    };
    for (entity, id, lifespan) in &query {
        let is_live = active.contains(&id.0);
        // Original UCOMISS/JA: a positive ordered lifespan comparison skips; unordered is expired.
        let expired = lifespan.is_none_or(|lifespan| !(lifespan.min_lifespan > lifespan.t));
        if !is_live && expired {
            commands.entity(entity).insert(SignifierMarkedForDeletion);
        }
    }
}

/// Original140929a90: no animation, or ordered t==±0 after reverse, permits deletion.
pub fn despawn_marked(
    query: Query<(
        Entity,
        &SignifierMarkedForDeletion,
        Option<&super::animation::SignifierAnimation>,
    )>,
    mut commands: Commands,
) {
    for (entity, _, animation) in &query {
        if animation.is_none_or(|animation| animation.t == 0.0) {
            commands.entity(entity).despawn();
        }
    }
}
