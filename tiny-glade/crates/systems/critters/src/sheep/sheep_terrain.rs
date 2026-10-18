//! sheep_terrain（B 主基线，fresh 反编译证据）。
//!
//! 证据：`evidence/restorer-eco-critters/decomp/14231bd80.c`（sheep_walk_on_terrain）+ 反汇编
//! 常量复核：smooth = `1-exp(-3.0*dt)`（-3.0=0x142ad58e8）；行走增益 10.0（0x1429345a8）、
//! 浮空增益 20.0（0x142a7fa30）、下陷 ×3；clamp 行走 [-2,2]/[-5,5]、浮空 [-0.3,0.3]
//! （0x142925900/0x1429258d8）；阈值 -0.05（0x1429ddce8）；+4.0 身位偏移；bilinear 参数 130.0。
//! 状态码：2=地面吸附、4=过渡、6=浮空、3=panic 不可达、0/1/5=行走。
//!
//! fresh 判定要点（相对旧重建）：
//! - 水面 bilinear 只在 case 4（过渡）采样；`<=0` → state=2（非"在水里"→地面，旧实现反了）。
//! - case 4 的 lerp 为内联 `time_continuous_lerp`（rate 常量 -0.5 存在，符号约定未取证，按 0.5）。
//! - 行走分支有效高度 `h_eff = (state&1) ? state+4(cached_h) : h`（8B 记录 +4 为缓存高度）。

use bevy_ecs::prelude::*;
use country_core::resources::terrain_height::TerrainHeightsData;
use country_core::resources::time::GameTime;
use country_core::resources::world_raster::WorldRaster;
use glam::Vec3;

use crate::boids::time_continuous_lerp;
use crate::components::Transform;

/// 羊地形状态（8B 记录：state u32@+0、cached_h f32@+4——行走态 bit0 置位时作为有效高度）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct SheepTerrainState {
    pub state: u32,
    /// +4：缓存地形高度（state&1 时作为 h_eff；O 14231bd80 default 分支 `(float)puVar3[1]`）。
    pub cached_h: f32,
}

impl SheepTerrainState {
    pub fn new(state: u32) -> Self {
        Self {
            state,
            cached_h: 0.0,
        }
    }
}

/// 竖向速度/差记录（12B 槽；仅 @+4 被读写）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct SheepVerticalDelta(pub f32);

pub const STATE_GROUNDED: u32 = 2;
pub const STATE_TRANSITION: u32 = 4;
pub const STATE_FLOATING: u32 = 6;

/// sheep_walk_on_terrain（0x14231bd80）。
///
/// 逐羊（O，fresh 14231bd80.c）：
/// - 顶部 `smooth = 1-exp(-3.0*dt)`；`h = TerrainHeightsData::sample_at_world_xz(pos.x,pos.z)`。
/// - switch(state@+0)：
///   - 2 → `pos.y = h`；
///   - 3 → panic（不可达）；
///   - 4 → `pos.y = time_continuous_lerp(pos.y, h, rate, dt)`；`bilinear(pos)<=0` → state=2；
///   - 6 → `dy=h-pos.y`；`c=clamp(dy,-0.3,0.3)`；`v += (c*20-v)*smooth`；`dy>-0.05` → state=2、v=0；
///   - default（行走 0/1/5）→ `h_eff=(state&1)?cached_h:h`；`dy=(h_eff+4.0)-pos.y`；
///     `h<=pos.y` → `c=clamp(dy,-2,2)`、`v+=(c*10-v)*smooth`；否则 `pos.y=h`、`c=clamp(dy,-5,5)`、
///     `v+=(c*10-v)*smooth*3`。
#[allow(clippy::type_complexity)]
pub fn sheep_walk_on_terrain(
    time: Res<GameTime>,
    terrain: Res<TerrainHeightsData>,
    water: Res<WorldRaster<512>>,
    mut query: Query<(&mut Transform, &mut SheepTerrainState, &mut SheepVerticalDelta)>,
) {
    let dt = time.delta;
    let w = (*water).as_ref();
    for (mut transform, mut state, mut vertical) in &mut query {
        let pos = transform.translation;
        let h = terrain.sample_at_world_xz(pos.x, pos.z);
        let (y, v, s) = apply_terrain_follow(
            *state,
            h,
            pos,
            vertical.0,
            dt,
            |x, z| w.bilinear_sample_at_world_xz(x, z, 130.0, 130.0),
        );
        transform.translation.y = y;
        vertical.0 = v;
        *state = s;
    }
}

/// 竖向弹簧-阻尼核心（fresh 14231bd80 逐分支还原；bevy-free）。
/// `water` 回调只在 state==4 分支被调用（与二进制一致：水面采样仅过渡态发生）。
pub fn apply_terrain_follow(
    state: SheepTerrainState,
    h: f32,
    pos: Vec3,
    vel: f32,
    dt: f32,
    water: impl Fn(f32, f32) -> f32,
) -> (f32, f32, SheepTerrainState) {
    let smooth = 1.0 - (-3.0 * dt).exp();
    match state.state {
        STATE_GROUNDED => (h, vel, state),
        STATE_TRANSITION => {
            // O：内联 time_continuous_lerp；rate 常量 -0.5 存在，符号未取证，按 0.5 趋近。
            let y = time_continuous_lerp(pos.y, h, 0.5, dt);
            let mut st = state;
            if water(pos.x, pos.z) <= 0.0 {
                st.state = STATE_GROUNDED;
            }
            (y, vel, st)
        }
        STATE_FLOATING => {
            let dy = h - pos.y;
            let c = dy.clamp(-0.3, 0.3);
            let v = vel + (c * 20.0 - vel) * smooth;
            let mut st = state;
            if dy > -0.05 {
                st.state = STATE_GROUNDED;
                (pos.y, 0.0, st)
            } else {
                (pos.y, v, st)
            }
        }
        _ => {
            let h_eff = if state.state & 1 != 0 { state.cached_h } else { h };
            let dy = (h_eff + 4.0) - pos.y;
            if h <= pos.y {
                let c = dy.clamp(-2.0, 2.0);
                (pos.y, vel + (c * 10.0 - vel) * smooth, state)
            } else {
                let c = dy.clamp(-5.0, 5.0);
                (h, vel + (c * 10.0 - vel) * smooth * 3.0, state)
            }
        }
    }
}
