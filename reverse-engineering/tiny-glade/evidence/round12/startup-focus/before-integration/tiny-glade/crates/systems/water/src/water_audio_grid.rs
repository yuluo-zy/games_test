//! water_audio_grid.rs — 相机周围水声采样格更新（B 0x141baefe0，O；fresh 2026-10-17
//! 逐语句重建）。
//!
//! 语义（O，`_scratch/uwag_full.txt` 反汇编逐条）：每音频栅格单元维护 **64 帧滚动活动
//! 位集**——每帧把当前帧位 `1 << (frame_bit & 0x3f)` 置/清：
//! - 相机→采样点距离² ≥ 6400.0（0x142eed900 = 80²）→ 清位；
//! - 栅格水位双线性采样（`bilinear_sample_at_world_xz`，extent 130.0 / 窗 260.0）≤ 0.5
//!   （0x142925870）→ 清位；
//! - 相机→格点遮挡射线（`intersects_any`，mask=0x32a3）命中 → 清位；否则置位并累加
//!   抖动增量对。
//!
//! 帧位哈希：`frame_bit = f + (f+1)*10 + 1 = 11*(f+1)`；抖动增量对（PAND 0x000e000e +
//! PADDW -7）= `{((2*fb)&0xe)-7, ((fb>>2)&0xe)-7}`；位置抖动 = `d * 0.0625 * 13.0`
//! （0x142eed8e0 / 0x142eed8f0）。
//!
//! 契约差（登记 RESTORE.md）：原版 24 B 栅格单元 = {f32 x, f32 z, u64 窗口位 @+8,
//! u16×2 计数器 @+0x10}，`update_water_audio_grid` 只写窗口位/计数器；core 当前
//! `WaterAudioGrid` 为语义模型（`water_level`/`position`），本 crate 把滚动窗口/计数器
//! 移入 [`WaterAudioGridUpdateState`]（Local）并按语义模型投影写入 core 资源。

use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

use country_core::resources::terrain_height::{TerrainHeightSsbo, TerrainHeightsData};
use country_core::resources::world_raster_defs::WaterRaster;
use country_core::systems::audio::water_ambience_audio::WaterAudioGrid;
use country_core::systems::camera_rig::main_camera::MainCamera;
use country_core::systems::collision::world::Raycasting;

use crate::water_systems_enabled;

/// 栅格维数（O：掩码 0x0e → 每轴 8 格；core `WATER_GRID_EXTENT` = 8 一致）。
pub const WATER_GRID_EXTENT: usize = 8;
/// 格坐标 → 世界 / 抖动幅度（O：`x * 0x0625(0x142eed8e0) * 13.0(0x142eed8f0)` = x*0.8125）。
pub const GRID_CELL_SPACING: f32 = 0.0625 * 13.0;
/// 相机距离² 阈值（O：0x142eed900 = 6400.0）。
pub const MAX_CAMERA_DIST_SQ: f32 = 6400.0;
/// 水位阈值（O：`<= 0.5` 清位，0x142925870）。
pub const WATER_LEVEL_THRESHOLD: f32 = 0.5;
/// 遮挡射线掩码（O：0x32a3；含地形 0x20 位）。
pub const OCCLUSION_RAY_MASK: u32 = 0x32a3;
/// 采样 extent / 窗（O：0x142a7b658 = 130.0 / 0x43020000 = 260.0）。
pub const SAMPLE_EXTENT: f32 = 130.0;
pub const SAMPLE_WINDOW: f32 = 260.0;

/// 滚动更新状态（O：`*param_1 = f + 1` 帧计数；窗口位 = `1 << ((11*(f+1)) & 0x3f)`，
/// 11 与 64 互质 → 全窗口轮转）。
pub struct WaterAudioGridUpdateState {
    /// 帧计数。
    pub frame: u64,
    /// 每格 64 位滚动窗口（原版在 24 B 栅格条目 +0x08；语义模型移入 Local，差登记）。
    pub windows: [u64; WATER_GRID_EXTENT * WATER_GRID_EXTENT],
    /// 每格可见帧抖动增量累加对（原版 24 B 条目 +0x10 u16×2，PADDW/PSUBW）。
    pub sums: [[i16; 2]; WATER_GRID_EXTENT * WATER_GRID_EXTENT],
}

impl Default for WaterAudioGridUpdateState {
    fn default() -> Self {
        Self {
            frame: 0,
            windows: [0; WATER_GRID_EXTENT * WATER_GRID_EXTENT],
            sums: [[0; 2]; WATER_GRID_EXTENT * WATER_GRID_EXTENT],
        }
    }
}

/// 格索引 → 轴坐标（O：`((2*(i&7))-7, (2*((i>>2)&7))-7)`；格基址 = 坐标 × spacing。
/// I：原版由栅格构建方（音频域 init）预写单元 +0/+4 基址）。
fn cell_coord(index: usize) -> Vec2 {
    let i = index as u32;
    Vec2::new(
        2.0 * (i & 7) as f32 - 7.0,
        2.0 * ((i >> 2) & 7) as f32 - 7.0,
    )
}

/// `update_water_audio_grid` @0x141baefe0（O，fresh 重建）。
///
/// 参数面（O，BASELINE）：`(Local<WaterAudioGridUpdateState>, Res<WaterRaster>,
/// Raycasting, Res<MainCamera>, ResMut<WaterAudioGrid>)`。core 侧 `Raycasting` 原为
/// 普通结构（SystemParam 待接线）+ 地形数据资源分离（`intersects_any` 显式传
/// `TerrainHeightsData`），本 crate 以 `Res<Raycasting>` + `Res<TerrainHeightsData>` 落地
/// （差登记 RESTORE.md）。
pub fn update_water_audio_grid(
    mut state: Local<WaterAudioGridUpdateState>,
    water: Res<WaterRaster>,
    raycasting: Res<Raycasting>,
    terrain: Res<TerrainHeightsData<TerrainHeightSsbo>>,
    camera: Res<MainCamera>,
    mut grid: ResMut<WaterAudioGrid>,
) {
    if !water_systems_enabled() {
        return;
    }
    // O：帧计数（pre-increment）：`state.frame = f + 1`（0x141baf318）。
    let frame = state.frame;
    state.frame = frame.wrapping_add(1);
    // O：frame_bit = f + (f+1)*10 + 1 = 11*(f+1)（0x141baf32b..333）。
    let frame_bit = frame
        .wrapping_add(frame.wrapping_add(1).wrapping_mul(10))
        .wrapping_add(1);
    // O：滚动窗位 = `1 << (frame_bit & 63)`（0x141baf3a0 SHL R13,CL）。
    let bit = 1u64 << (frame_bit & 63);
    // O：抖动增量对（0x141baf33f..353：PAND 0x000e000e → PADDW -7）：
    //     d = {((2*fb)&0xe)-7, ((fb>>2)&0xe)-7}。
    let delta_x = (frame_bit.wrapping_mul(2) & 0xe) as i16 - 7;
    let delta_z = ((frame_bit >> 2) & 0xe) as i16 - 7;
    // O：位置抖动 = d * 0.0625 * 13.0（0x141baf371/379）。
    let jitter = Vec2::new(
        delta_x as f32 * GRID_CELL_SPACING,
        delta_z as f32 * GRID_CELL_SPACING,
    );
    // O：`MainCamera::eye_position`（0x140911dd0）。
    let eye = camera.eye_position();

    for (index, cell) in grid.cells.iter_mut().enumerate() {
        // O：单元基址 + 抖动 → `utils::swizzlers::x0y`（0x140c9cdc0）→ 采样点 {x,0,z}。
        let coord = cell_coord(index);
        let base = Vec2::new(coord.x * GRID_CELL_SPACING, coord.y * GRID_CELL_SPACING);
        let sample = base + jitter;
        let point = Vec3::new(sample.x, 0.0, sample.y);
        let delta = eye - point;
        // O 三重剔除（0x141baf44d 起）：`UCOMISS 6400,dist; JBE` → 距离² ≥ 6400
        //   **或 unordered(NaN)** → 清位；水位 `UCOMISS level,0.5; JBE` → ≤ 0.5 或
        //   NaN → 清位；遮挡射线命中 → 清位。`!(x<y)`/`!(x>y)` 形式对齐 JBE 的
        //   unordered-NaN 亦清位语义（输入有限时不可达，记录 RESTORE.md）。
        let visible = if !(delta.length_squared() < MAX_CAMERA_DIST_SQ) {
            false
        } else {
            let level = (**water)
                .as_ref()
                .bilinear_sample_at_world_xz(point.x, point.z, SAMPLE_EXTENT, SAMPLE_WINDOW);
            if !(level > WATER_LEVEL_THRESHOLD) {
                false
            } else {
                !raycasting.intersects_any(&terrain, point, delta, OCCLUSION_RAY_MASK)
            }
        };

        // O：滚动窗置位/清位；0→1 累加增量对（PADDW），1→0 累减（PSUBW）
        //     （0x141baf4b3..4da / 0x141baf4e8..50c）。
        let was_set = state.windows[index] & bit != 0;
        if visible {
            state.windows[index] |= bit;
            if !was_set {
                state.sums[index][0] = state.sums[index][0].wrapping_add(delta_x);
                state.sums[index][1] = state.sums[index][1].wrapping_add(delta_z);
            }
        } else {
            state.windows[index] &= !bit;
            if was_set {
                state.sums[index][0] = state.sums[index][0].wrapping_sub(delta_x);
                state.sums[index][1] = state.sums[index][1].wrapping_sub(delta_z);
            }
        }

        // O 语义模型投影（I：原版此处不写 water_level/position——窗口/计数器由消费方
        //   推导；core WaterAudioGrid 以 `water_level` + `position` 表达，本实现把
        //   真实窗口占比投影为 water_level、把计数器质心投影为 position，差登记）。
        let window = state.windows[index];
        let sum = state.sums[index];
        let visible_count = window.count_ones();
        let level = visible_count as f32 / 64.0;
        let centroid = if visible_count == 0 {
            point
        } else {
            Vec3::new(
                base.x + sum[0] as f32 / visible_count as f32 * GRID_CELL_SPACING,
                0.0,
                base.y + sum[1] as f32 / visible_count as f32 * GRID_CELL_SPACING,
            )
        };
        cell.water_level = level;
        cell.position = centroid;
    }
}

/// Retained CPU projection; its unit-counter limitation is documented in the module.
pub mod bit_projection;
