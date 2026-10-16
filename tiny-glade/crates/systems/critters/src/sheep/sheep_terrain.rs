//! sheep_terrain（B 主基线，反汇编级证据；reconstruction/critters/src/sheep_terrain.rs 一致）。
//!
//! 证据：sheep_walk_on_terrain 0x14231bd80（源码 15–80）。
//! 查询列：Transform(0x30)、8B 状态（state u32@+0, f32@+4）、3×4B、12B 竖向速度记录。
//! 常量（RNE hexdump 证实）：smooth = 1-exp(-3.0*dt)；+4.0 身位偏移；行走增益 10.0、
//! 浮空增益 20.0、下陷 ×3；clamp 行走 [-2,2]/[-5,5]、浮空 [-0.3,0.3]；阈值 -0.05 转地面态。
//! 状态码：2=地面吸附、4=过渡、6=浮空、3=panic 不可达、0/1/5=行走。

use bevy_ecs::prelude::*;
use country_core::resources::terrain_height::TerrainHeightsData;
use country_core::resources::time::GameTime;
use country_core::resources::world_raster::WorldRaster;

use crate::components::{Sheep, Transform};

/// 羊地形状态（8B：state u32@+0、f32@+4——@+4 语义 unknown）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SheepTerrainState(pub u32);

/// 竖向速度/差记录（12B 槽；仅 @+4 被读写）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct SheepVerticalDelta(pub f32);

pub const STATE_GROUNDED: u32 = 2;
pub const STATE_TRANSITION: u32 = 4;
pub const STATE_FLOATING: u32 = 6;

/// sheep_walk_on_terrain（0x14231bd80）。
#[allow(clippy::type_complexity)]
pub fn sheep_walk_on_terrain(
    time: Res<GameTime>,
    terrain: Res<TerrainHeightsData>,
    water: Res<WorldRaster<512>>,
    mut query: Query<(
        &mut Transform,
        &mut SheepTerrainState,
        &mut SheepVerticalDelta,
    )>,
) {
    let dt = time.delta;
    let w = (*water).as_ref();
    for (mut transform, mut state, mut vertical) in &mut query {
        let pos = transform.translation;
        let h = terrain.sample_at_world_xz(pos.x, pos.z);
        let in_water = w.bilinear_sample_at_world_xz(pos.x, pos.z, 130.0, 130.0) > 0.0;
        let (y, v, s) = apply_terrain_follow(
            *state,
            h,
            pos.y,
            *vertical,
            dt,
            pos.y,
            in_water,
        );
        transform.translation.y = y;
        *vertical = v;
        *state = s;
    }
}

/// 竖向弹簧-阻尼核心（reconstruction/critters::sheep_terrain::apply_terrain_follow 一致）。
pub fn apply_terrain_follow(
    state: SheepTerrainState,
    terrain_h: f32,
    pos_y: f32,
    vel: SheepVerticalDelta,
    dt: f32,
    lerped_y: f32,
    in_water: bool,
) -> (f32, SheepVerticalDelta, SheepTerrainState) {
    let smooth = 1.0 - (-3.0 * dt).exp();
    match state.0 {
        STATE_GROUNDED => (terrain_h, vel, state),
        STATE_TRANSITION => {
            let y = lerped_y;
            if in_water {
                (y, vel, SheepTerrainState(STATE_GROUNDED))
            } else {
                (y, vel, state)
            }
        }
        STATE_FLOATING => {
            let dy = terrain_h - pos_y;
            let c = dy.clamp(-0.3, 0.3);
            let v = vel.0 + (c * 20.0 - vel.0) * smooth;
            if dy > -0.05 {
                (pos_y, SheepVerticalDelta(0.0), SheepTerrainState(STATE_GROUNDED))
            } else {
                (pos_y, SheepVerticalDelta(v), state)
            }
        }
        _ => {
            let dy = (terrain_h + 4.0) - pos_y;
            if terrain_h <= pos_y {
                let c = dy.clamp(-2.0, 2.0);
                let v = vel.0 + (c * 10.0 - vel.0) * smooth;
                (pos_y, SheepVerticalDelta(v), state)
            } else {
                let c = dy.clamp(-5.0, 5.0);
                let v = vel.0 + (c * 10.0 - vel.0) * smooth * 3.0;
                (terrain_h, SheepVerticalDelta(v), state)
            }
        }
    }
}
