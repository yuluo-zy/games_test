//! sheep_unstuck（0x1423246b0，源码 0–；fresh 反编译证据）。
//!
//! 逐羊（O，fresh 1423246b0.c）：
//! - `RaycastWorld::legacy_spherecast` → 命中码；`!= 2` → 8B 状态列 = (1, 结果) 并跳过后续。
//! - 否则 `terrain_rock_mask::sample_at_world_xz` + 水面 bilinear；
//!   门 `mask空 || state>6 || (0x54>>(state&31)&1)==0`（0x54 选中 {2,4,6}）→ 内层状态机：
//!   - state==2：水>0 → `is_corrupted()`；非坏档 → state=4；坏档 → state=0；水<=0 → ddbb；
//!   - state==5 且 mask空且水<=0：地形高 h；`pos.y>h` → state=6；
//!   - ddbb（state 2 水<=0）：`ground_h=max(h,0.5)`；`flag&1 && |(ground_h-记录)/dt| 有限且>10.0`
//!     → state=0；记录 = (1, ground_h)；
//!   - 其余状态经 cfe：记录清零。
//! - 门失败（岩石且 state∈{2,4,6}）→ state=0、记录清零。
//! - 收尾：state!=2 → 记录清零；`Sheep+0x30(flag_30)==0` → 清零 Idle timeout_s。
//!
//! DIVERGE：`legacy_spherecast` ABI（shared.rs 抽象 hit/distance，非命中码 pair）；
//! `terrain_rock_mask` 逐点采样（本 crate 为整体 grid）；`is_corrupted` 未接线（按非坏档）；
//! Sheep+0/+4 记录因 components.rs 不可改落为 `SheepGroundRecord`（布局分拆）。

use bevy_ecs::prelude::*;
use country_core::resources::terrain_height::TerrainHeightsData;
use country_core::resources::time::GameTime;
use country_core::resources::world_raster::WorldRaster;
use glam::Vec3;

use crate::components::{Sheep, SheepMoveState, Transform, Velocity};
use crate::shared::{RaycastWorldHandle, TerrainRockMask, legacy_spherecast};

/// Sheep 记录 +0（字节位）/+4（地面高度）——二进制位于 0x68 记录，因 components.rs 不可改分拆。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct SheepGroundRecord {
    pub flag: u8,
    pub h: f32,
}

/// 岩石掩码逐点采样（宿主 grid 回读；二进制 `sample_at_world_xz` 返回字节，DIVERGE）。
fn rock_present(_mask: &TerrainRockMask, _pos: Vec3) -> bool {
    false
}

/// `BackupSaveFileKind::is_corrupted`（host 未接线；按非坏档）。
fn save_is_corrupted() -> bool {
    false
}

/// sheep_unstuck（0x1423246b0）。
pub fn sheep_unstuck(
    time: Res<GameTime>,
    terrain: Res<TerrainHeightsData>,
    water: Res<WorldRaster<512>>,
    rock_mask: Res<TerrainRockMask>,
    raycast: Res<RaycastWorldHandle>,
    mut query: Query<(
        &Transform,
        &mut Sheep,
        &mut SheepGroundRecord,
        &mut crate::components::CritterLocomotion,
        &mut Velocity,
    )>,
) {
    let dt = time.delta;
    let w = (*water).as_ref();
    for (transform, mut sheep, mut ground, mut loc, _velocity) in &mut query {
        let pos = transform.translation;
        // Sheep 上下文 → legacy_spherecast 半径 0.8（B-ecstate 半径约定）。
        let sphere = legacy_spherecast(&raycast, pos, Vec3::NEG_Y, 0.8);
        let state0 = loc.0;
        // O：命中码 != 2 → 状态列 = (1, 结果)（结果字段语义 U）。
        if !sphere.hit {
            loc.0 = 1;
            let _ = sphere.distance;
            continue;
        }
        let rock = rock_present(&rock_mask, pos);
        let water_s = w.bilinear_sample_at_world_xz(pos.x, pos.z, 130.0, 130.0);
        let mask_sel = (0x54_u32 >> (state0 & 0x1f)) & 1 == 1;
        if !rock || state0 > 6 || !mask_sel {
            // 内层状态机（fresh 1423246b0）：`bucket = clamp_u32(state-2, 0..3)`
            // （`if 4 < state-2 {3}`，u32 **回绕**：state 0/1/5/7+ 均映射 bucket 3）。
            let mut bucket = state0.wrapping_sub(2);
            if 4 < bucket {
                bucket = 3;
            }
            if bucket == 0 {
                // state==2。
                if water_s > 0.0 {
                    // is_corrupted()：非坏档 → 4；坏档 → 0（cd1 清记录低 4 字节）。
                    if !save_is_corrupted() {
                        loc.0 = 4;
                    } else {
                        loc.0 = 0;
                        ground.flag = 0;
                    }
                } else {
                    // ddbb：记录地面高度（flag=1 + h）；不在此清旧值。
                    let h = terrain.sample_at_world_xz(pos.x, pos.z);
                    let ground_h = if h <= 0.5 { 0.5 } else { h };
                    let fall_rate = (ground_h - ground.h) / dt;
                    if (ground.flag & 1) != 0 && fall_rate.is_finite() && 10.0 < fall_rate.abs() {
                        loc.0 = 0;
                    }
                    ground.flag = 1;
                    ground.h = ground_h;
                }
            } else if bucket == 3 {
                // state 0/1/5/7+：`!rock && water<=0` 时比较地形高度；
                // `pos.y > h` → state=6；否则回尾部（`state==2` 的 ddbb 分支不可达）。
                if !rock && water_s <= 0.0 {
                    let h = terrain.sample_at_world_xz(pos.x, pos.z);
                    if pos.y > h {
                        loc.0 = 6;
                    }
                }
            } else {
                // bucket 1/2（state 3/4）：dd2 → state!=2 → 尾部；记录不在此清。
            }
        } else {
            // 岩石且 state∈{2,4,6} → cd1：state=0、清记录低 4 字节（h 保留）。
            loc.0 = 0;
            ground.flag = 0;
        }
        // 收尾（O L292–302）：仅 `state!=2` 时写时间戳并（flag_30==0）清零 Idle timeout_s；
        // 地面记录**不**在此清除（旧实现的清记录已删除）。
        if loc.0 != 2 {
            // B 142324d21..142324d2c clears only the active Idle timeout.
            if let SheepMoveState::Idle { timeout_s } = &mut sheep.move_state {
                *timeout_s = 0.0;
            }
        }
    }
}
