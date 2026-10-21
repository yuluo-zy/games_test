//! 指示符回收：本帧不再请求且已满足最短寿命时，标记并销毁实体。
//! 活动集合由 captured WidgetParams 的完整工厂列表导出；保留原延迟命令边界。
use bevy_ecs::prelude::*;

use super::{signifier_create::SignifierId, animation::SignifierAnimation};
use crate::systems::ui::resources::ui_state::UiState;
use std::collections::HashSet;

/// 已标记删除的指示符实体（O：`SignifierMarkedForDeletion` 组件标记）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct SignifierMarkedForDeletion;

/// 指示符最短寿命（O：组件 8 B；`min_lifespan @+4`、`t @+0`）。
///
/// 删除条件（机器码：`movss xmm0,[rdi+4]; ucomiss xmm0,[rdi]; ja 跳过`）：
/// `!(min_lifespan > t)` 时打删除标记；unordered 也不跳过。
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
    let mut live = HashSet::new();
    if ui.hover_byte() != 0 {
        let widget = ui.active_widget();
        let params = ui.active_params_ref().expect("active UI has captured WidgetParams");
        live.extend(params.signifiers.iter().map(|factory| {
            crate::utils::calculate_hash(&(widget, factory.name()))
        }));
    }
    for (entity, id, lifespan) in &query {
        // B14092980a UCOMISS min,t; JA skips only ordered min > t.
        let expired = lifespan.is_none_or(|v| !(v.min_lifespan > v.t));
        if !live.contains(&id.0) && expired {
            commands.entity(entity).insert(SignifierMarkedForDeletion);
        }
    }
}

/// `despawn_marked` @0x140929a90：等待退场动画 t 精确归零再销毁。
pub fn despawn_marked(
    query: Query<(Entity, &SignifierMarkedForDeletion, Option<&SignifierAnimation>)>,
    mut commands: Commands,
) {
    for (entity, _, animation) in &query {
        // B140929f5f/62/68: exactly ordered t == 0, including negative zero.
        // Negative, positive, infinite and NaN t all wait for the animation.
        if animation.is_none_or(|animation| animation.t == 0.0) {
            commands.entity(entity).despawn();
        }
    }
}
