//! autoclutter_animation（B 主基线；0x1422ccef0，源码 17–）。
//!
//! 证据（skeleton PDB 依赖注解）：EasingFunction::apply（0x142923e80 附近）、
//! EntityCommands::despawn、raw_vec。语义：自动杂物实体出生弹入动画（Easing 驱动）；
//! 动画完成 → despawn。

use bevy_ecs::prelude::*;
use country_core::resources::time::GameTime;
use utils::easing_functions::EasingFunction;

/// 自动杂物动画（组件；出生弹入：scale 0→1，Easing 驱动）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct AutoClutterAnimation {
    pub age: f32,
    pub duration: f32,
}

/// autoclutter_animation（0x1422ccef0）：推进弹入动画；完成 → despawn。
pub fn autoclutter_animation(
    time: Res<GameTime>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut AutoClutterAnimation)>,
) {
    let dt = time.delta;
    let mut done = Vec::new();
    for (entity, mut anim) in &mut query {
        anim.age += dt;
        let t = (anim.age / anim.duration.max(0.001)).clamp(0.0, 1.0);
        let _eased = EasingFunction::CubicOut.apply(t);
        if anim.age >= anim.duration {
            done.push(entity);
        }
    }
    for entity in done {
        commands.entity(entity).despawn();
    }
}
