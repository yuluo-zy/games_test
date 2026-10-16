//! 屋顶动画（visual/roof_animation.rs）。
//!
//! `roof_animation`（0x1421a2da0）。候选参数面（skeleton 富化）：
//! `(Query<(Entity, &Roof, &mut RoofAnimation, Option<&RoofMarkedForDeletion>)>,
//!  Res<GameTime>)`。
use country_core::resources::{
    roofs::{Roof, RoofAnimation, RoofMarkedForDeletion},
    time::GameTime,
};
use bevy_ecs::prelude::*;

pub fn roof_animation(
    mut roofs: Query<(Entity, &Roof, &mut RoofAnimation, Option<&RoofMarkedForDeletion>)>,
    time: Res<GameTime>,
) {
    for (_entity, roof, mut animation, marked) in &mut roofs {
        // 未变化的 delta 不得仅因可变查询行被访问而设置 Bevy changed tick；
        // 恢复叶只在某通道 delta 非零（含无序 NaN）时写该标记。
        let mut wrote = 0;
        animation
            .bypass_change_detection()
            .advance_observed(roof.observed(), marked.is_some(), time.delta, &mut wrote, 1);
        if wrote != 0 {
            animation.set_changed();
        }
    }
}
