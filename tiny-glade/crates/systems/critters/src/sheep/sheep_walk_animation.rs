//! sheep_walk_animation（0x14231a630，源码 0–；closure$0 L15）。
//!
//! 证据（skeleton PDB 依赖注解）：`Sheep::being_petted`（0x14089d4c0 = f32@+0x58 > 0.01）、
//! fmodf（0x142923f10）。语义：非抚摸时按速度推进行走周期（fmod 相位），
//! 抚摸时冻结为站立（being_petted 门控）。

use bevy_ecs::prelude::*;
use country_core::resources::time::GameTime;

use crate::components::{Sheep, Velocity};

/// 羊行走动画相位（组件；宿主渲染域消费）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct SheepWalkPhase {
    pub phase: f32,
}

/// sheep_walk_animation（0x14231a630）：`being_petted → 跳过行走动画`（O）。
pub fn sheep_walk_animation(
    time: Res<GameTime>,
    mut query: Query<(&Sheep, &Velocity, &mut SheepWalkPhase)>,
) {
    let dt = time.delta;
    for (sheep, velocity, mut phase) in &mut query {
        if sheep.being_petted() {
            continue;
        }
        // fmodf 相位推进（O：行走周期 = 速度驱动）。
        let speed = velocity.0.length();
        phase.phase = (phase.phase + speed * dt * 2.0) % 1.0;
    }
}
