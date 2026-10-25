//! 原 B `sheep_walk_on_terrain`（0x14231bd80），唯一 Locomotion/Velocity 消费者。
//!
//! 原指令夹具见 `toolbar-coaster-sheep-terrain-native.json`；过渡态目标是 h-0.5、
//! 速率4，而非 h/0.5。奇数行走态使用同一 Locomotion 的 +4 缓存高度。

use crate::{
    boids::time_continuous_lerp,
    components::{CritterLocomotion, Sheep, Transform, Velocity},
};
use bevy_ecs::prelude::*;
use country_core::{
    render::extract::ComponentWrite,
    resources::{terrain_height::TerrainHeightsData, time::GameTime, world_raster::WorldRaster},
};
use glam::Vec3;

/// 保留既有调用名称，但它和 CritterLocomotion 具有同一个 Component TypeId。
/// 不创建第二份地形状态或竖向速度。
pub type SheepTerrainState = CritterLocomotion;
pub const STATE_GROUNDED: u32 = 2;
pub const STATE_TRANSITION: u32 = 4;
pub const STATE_FLOATING: u32 = 6;

#[allow(clippy::type_complexity)]
pub fn sheep_walk_on_terrain(
    time: Res<GameTime>,
    terrain: Res<TerrainHeightsData>,
    water: Res<country_core::resources::world_raster_defs::WaterRaster>,
    mut query: Query<
        (
            ComponentWrite<Transform>,
            &mut CritterLocomotion,
            &mut Velocity,
        ),
        With<Sheep>,
    >,
) {
    let w = (*water).as_ref();
    for (mut transform, mut state, mut velocity) in &mut query {
        let pos = transform.translation;
        let h = terrain.sample_at_world_xz(pos.x, pos.z);
        let old_tag = state.0;
        let (y, v, next) =
            apply_terrain_follow(*state, h, pos, velocity.0.y, time.delta, |x, z| {
                w.bilinear_sample_at_world_xz(x, z, 130.0, 130.0)
            });
        // 原函数每次标记 Locomotion；Transform/Velocity 的 changed tick 则按分支写。
        *state = next;
        match old_tag {
            STATE_GROUNDED | STATE_TRANSITION => transform.translation.y = y,
            STATE_FLOATING => velocity.0.y = v,
            _ => {
                if h > pos.y {
                    transform.translation.y = y;
                }
                velocity.0.y = v;
            }
        }
    }
}

/// 原 SSE MAXSS/MINSS 顺序：无序比较选择输入，不把 NaN 偷换成有限 clamp 边界。
fn native_clamp(value: f32, lower: f32, upper: f32) -> f32 {
    let value = if lower > value { lower } else { value };
    if upper < value { upper } else { value }
}

/// 逐分支执行原竖向弹簧；water 仅在过渡态调用一次，状态3保留原不可达 panic。
pub fn apply_terrain_follow(
    state: SheepTerrainState,
    h: f32,
    pos: Vec3,
    vel: f32,
    dt: f32,
    water: impl Fn(f32, f32) -> f32,
) -> (f32, f32, SheepTerrainState) {
    let smooth = 1.0 - (dt * -3.0).exp();
    match state.0 {
        STATE_GROUNDED => (h, vel, state),
        3 => panic!("flying sheep cannot follow terrain"),
        STATE_TRANSITION => {
            let y = time_continuous_lerp(pos.y, h + -0.5, 4.0, dt);
            let mut next = state;
            // 原 ucomiss+ja：只有严格正值保持4，NaN也转为地面态。
            if !(water(pos.x, pos.z) > 0.0) {
                next.0 = STATE_GROUNDED;
            }
            (y, vel, next)
        }
        STATE_FLOATING => {
            let dy = h - pos.y;
            let v = vel + (native_clamp(dy, -0.3, 0.3) * 20.0 - vel) * smooth;
            let mut next = state;
            if dy > -0.05 {
                next.0 = STATE_GROUNDED;
                (pos.y, 0.0, next)
            } else {
                (pos.y, v, next)
            }
        }
        _ => {
            let effective = if state.0 & 1 != 0 { state.1 } else { h };
            let dy = (effective + 4.0) - pos.y;
            if h > pos.y {
                (
                    h,
                    vel + ((native_clamp(dy, -5.0, 5.0) * 10.0 - vel) * smooth) * 3.0,
                    state,
                )
            } else {
                (
                    pos.y,
                    vel + (native_clamp(dy, -2.0, 2.0) * 10.0 - vel) * smooth,
                    state,
                )
            }
        }
    }
}
