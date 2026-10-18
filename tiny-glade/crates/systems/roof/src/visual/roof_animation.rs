//! 屋顶动画（visual/roof_animation.rs）。
//!
//! `roof_animation`（0x1421a2da0，1134B/251）。参数面（decomp 观测）：
//! `(Query<(Entity, &Roof, &mut RoofAnimation, Option<&RoofMarkedForDeletion>)>,
//!  Res<GameTime>)`。
//!
//! 判等（fresh decomp-0x1421a2da0）：逐 roof 双 f32 通道按 dt 推进——
//! 删除标记分支 `dt·−4.0`/`dt·−10.0`；正常分支因子 `is_flat?2.0:1.0`、
//! `dt·2.857143·factor`、`主>0.99→dt·3.3333333·factor`；clamp[0,1] 写
//! `RoofAnimation[0..4]`/`[4..8]`，各写 changed tick。`RoofAnimation::advance`
//! （core）与机器码逐项一致；本实现经 `advance_observed` 委托并仅在非零通道
//! 写 Changed（等价原 tick 写）。ECS 迭代形状语义等价。
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
        let mut wrote = 0;
        animation
            .bypass_change_detection()
            .advance_observed(roof.observed(), marked.is_some(), time.delta, &mut wrote, 1);
        if wrote != 0 {
            animation.set_changed();
        }
    }
}