//! water_audio_grid.rs — 相机周围水声采样格更新（B 0x141baefe0，O；
//! re-analyst-A 契约 §2）。
//!
//! 语义（O）：每个音频栅格单元维护 **64 帧滚动活动位集**——每帧把当前帧位
//! `1 << (frame & 63)` 置/清：
//! - 相机距离² 超阈值（0x142eed900 = 6400.0，80 世界单位）→ 清位；
//! - 栅格水位采样（`bilinear_sample_at_world_xz`，extent 130.0 / 窗 260.0，
//!   0x142a7b658 / 0x43020000）≤ 0.5 → 清位；
//! - 相机→格点遮挡射线（`Raycasting::intersects_any(eye, cell-eye, mask=0x32a3)`，
//!   0x1408cc320）命中 → 清位；否则置位并累加格坐标。
//! 原版条目 24 B = {打包 u16×2 相对坐标, u64 窗口位, i16×2 坐标累加}（滚动环复用
//! 细节 I 待取证）；core `WaterAudioGrid` 语义模型以 `water_level`（窗口占比）+
//! `position`（格点世界位置）表达——本实现把滚动窗口移入
//! [`WaterAudioGridUpdateState`]（Local），按语义模型写入 core 资源。

use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

use country_core::resources::world_raster_defs::WaterRaster;
use country_core::systems::audio::water_ambience_audio::WaterAudioGrid;
use country_core::systems::camera_rig::main_camera::MainCamera;
use country_core::systems::collision::world::Raycasting;

use crate::water_systems_enabled;

/// 栅格维数（O：掩码 0x0e → 每轴 8 格；core `WATER_GRID_EXTENT` = 8 一致）。
pub const WATER_GRID_EXTENT: usize = 8;
/// 格坐标 → 世界（O：`coord * 0x0625(0x142eed8e0) * 13.0(0x142eed8f0)`）。
pub const GRID_CELL_SPACING: f32 = 0.0625 * 13.0;
/// 相机距离² 阈值（O：0x142eed900 = 6400.0）。
pub const MAX_CAMERA_DIST_SQ: f32 = 6400.0;
/// 水位阈值（O：`<= 0.5` 清位）。
pub const WATER_LEVEL_THRESHOLD: f32 = 0.5;
/// 遮挡射线掩码（O：0x32a3）。
pub const OCCLUSION_RAY_MASK: u64 = 0x32a3;
/// 采样 extent / 窗（O：0x142a7b658 = 130.0 / 0x43020000 = 260.0）。
pub const SAMPLE_EXTENT: f32 = 130.0;
pub const SAMPLE_WINDOW: f32 = 260.0;

/// 滚动更新状态（O：`*param_1 = f + 1` 帧计数；窗口位 = `1 << ((11*(f+1)) & 0x3f)`，
/// 11 与 64 互质 → 全窗口轮转）。
pub struct WaterAudioGridUpdateState {
    /// 帧计数。
    pub frame: u64,
    /// 每格 64 位滚动窗口（原版在 24 B 栅格条目内；语义模型移入 Local，I）。
    pub windows: [u64; WATER_GRID_EXTENT * WATER_GRID_EXTENT],
    /// 每格可见帧坐标累加 {x, z}（原版条目 u16×2 累加器；I）。
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

/// 格索引 → 轴坐标（O：`((idx 位交错) & 0x0e) - 7` ∈ {-7,-5,..,7}；
/// `2*(i&7)-7` / `2*((i>>2)&7)-7`）。
fn cell_coord(index: usize) -> Vec2 {
    let i = index as u32;
    Vec2::new(
        2.0 * (i & 7) as f32 - 7.0,
        2.0 * ((i >> 2) & 7) as f32 - 7.0,
    )
}

/// `update_water_audio_grid` @0x141baefe0（O）。
///
/// 参数面（O，BASELINE）：`(Local<WaterAudioGridUpdateState>, Res<WaterRaster>,
/// Raycasting, Res<MainCamera>, ResMut<WaterAudioGrid>)`——core 侧 `Raycasting`
/// 原为普通结构（SystemParam 待接线），组 A 以 `Res<Raycasting>` 落地（差登记
/// RESTORE.md）。
pub fn update_water_audio_grid(
    mut state: Local<WaterAudioGridUpdateState>,
    water: Res<WaterRaster>,
    raycasting: Res<Raycasting>,
    camera: Res<MainCamera>,
    mut grid: ResMut<WaterAudioGrid>,
) {
    if !water_systems_enabled() {
        return;
    }
    let frame = state.frame;
    state.frame = frame.wrapping_add(1);
    // O（reviewer FIX §3.4）：滚动窗口位 = `1 << ((11*(f+1)) & 0x3f)`
    // （11 与 64 互质 → 全窗口轮转；decomp-0x141baefe0 的 uVar16 面）。
    let bit = 1u64 << ((11 * (frame + 1)) & 63);
    let eye = camera.eye_position();

    for (index, cell) in grid.cells.iter_mut().enumerate() {
        let coord = cell_coord(index);
        let position = Vec3::new(coord.x * GRID_CELL_SPACING, 0.0, coord.y * GRID_CELL_SPACING);
        let delta = position - eye;
        let visible = if delta.length_squared() >= MAX_CAMERA_DIST_SQ {
            false
        } else {
            let level = (**water)
                .as_ref()
                .bilinear_sample_at_world_xz(position.x, position.z, SAMPLE_EXTENT, SAMPLE_WINDOW);
            if level <= WATER_LEVEL_THRESHOLD {
                false
            } else {
                !raycasting.intersects_any(eye, delta, OCCLUSION_RAY_MASK)
            }
        };

        let was_set = state.windows[index] & bit != 0;
        if visible {
            if !was_set {
                state.sums[index][0] = state.sums[index][0].wrapping_add(coord.x as i16);
                state.sums[index][1] = state.sums[index][1].wrapping_add(coord.y as i16);
            }
            state.windows[index] |= bit;
        } else {
            if was_set {
                state.sums[index][0] = state.sums[index][0].wrapping_sub(coord.x as i16);
                state.sums[index][1] = state.sums[index][1].wrapping_sub(coord.y as i16);
            }
            state.windows[index] &= !bit;
        }
        let window = state.windows[index];
        let sum = state.sums[index];

        // 语义模型写入（O 消费面：`water_level >= 0.5` 激活；位置 = 格点世界坐标。
        // y = 0 = 水面平面，与 draw_water 橡皮分支 xz 平面一致，I）。
        let level = window.count_ones() as f32 / 64.0;
        let visible_count = window.count_ones();
        let centroid = if visible_count == 0 {
            position
        } else {
            Vec3::new(
                sum[0] as f32 / visible_count as f32 * GRID_CELL_SPACING,
                0.0,
                sum[1] as f32 / visible_count as f32 * GRID_CELL_SPACING,
            )
        };
        cell.water_level = level;
        cell.position = centroid;
    }
}
