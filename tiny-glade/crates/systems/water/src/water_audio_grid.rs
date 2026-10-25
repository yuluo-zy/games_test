//! water_audio_grid.rs — 相机周围水声采样格更新（B 0x141baefe0，O；fresh 2026-10-17
//! 逐语句重建）。
//!
//! 语义（O，`_scratch/uwag_full.txt` 反汇编逐条）：每音频栅格单元维护 **64 帧滚动活动
//! 位集**——每帧把当前帧位 `1 << (frame_bit & 0x3f)` 置/清：
//! - 相机→采样点距离² ≥ 6400.0（0x142eed900 = 80²）→ 清位；
//! - 栅格水位双线性采样（`bilinear_sample_at_world_xz`，两个世界跨度均为130.0）≤ 0.5
//!   （0x142925870）→ 清位；
//! - 相机→格点遮挡射线（`intersects_any`，mask=0x32a3）命中 → 清位；否则置位并累加
//!   抖动增量对。
//!
//! 帧位哈希：`frame_bit = f + (f+1)*10 + 1 = 11*(f+1)`；抖动增量对（PAND 0x000e000e +
//! PADDW -7）= `{((2*fb)&0xe)-7, ((fb>>2)&0xe)-7}`；位置抖动 = `d * 0.0625 * 13.0`
//! （0x142eed8e0 / 0x142eed8f0）。
//!
//! core 的唯一 `WaterAudioGrid` 持有100个格子的基址、窗口位和两路 i16 抖动和；
//! 本系统仅在 Local 中保留帧号，并借用游戏实际安装的 `RaycastWorld`，不持有第二个
//! 碰撞世界。原遮挡查询从采样点指向相机，最大参数为1，不能延伸到相机之后。

use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

use country_core::resources::terrain_height::{TerrainHeightSsbo, TerrainHeightsData};
use country_core::resources::world_raster_defs::WaterRaster;
use country_core::systems::audio::water_ambience_audio::WaterAudioGrid;
use country_core::systems::camera_rig::main_camera::MainCamera;
use country_core::systems::collision::world::RaycastWorld;

use crate::water_systems_enabled;

/// 栅格维数（O：初始化1409ebf20构造10×10个基址；抖动掩码不表示格数）。
pub const WATER_GRID_EXTENT: usize = 10;
/// 格坐标 → 世界 / 抖动幅度（O：`x * 0x0625(0x142eed8e0) * 13.0(0x142eed8f0)` = x*0.8125）。
pub const GRID_CELL_SPACING: f32 = 0.0625 * 13.0;
/// 相机距离² 阈值（O：0x142eed900 = 6400.0）。
pub const MAX_CAMERA_DIST_SQ: f32 = 6400.0;
/// 水位阈值（O：`<= 0.5` 清位，0x142925870）。
pub const WATER_LEVEL_THRESHOLD: f32 = 0.5;
/// 遮挡射线掩码（O：0x32a3；含地形 0x20 位）。
pub const OCCLUSION_RAY_MASK: u32 = 0x32a3;
/// 两轴采样跨度（O：0x142a7b658及第五参数0x43020000均为130.0）。
pub const SAMPLE_EXTENT: f32 = 130.0;
pub const SAMPLE_WINDOW: f32 = 130.0;

/// 滚动更新状态（O：`*param_1 = f + 1` 帧计数；窗口位 = `1 << ((11*(f+1)) & 0x3f)`，
/// 11 与 64 互质 → 全窗口轮转）。
#[derive(Default)]
pub struct WaterAudioGridUpdateState {
    /// 原141baefe0仅将帧号放在Local；活动位与抖动和由网格持有，并在卸载时清空。
    pub frame: u64,
}

/// `update_water_audio_grid` @0x141baefe0（O，fresh 重建）。
///
/// 原参数中的借用 `Raycasting` 在当前工程拆为实际 `RaycastWorld` 和地形资源。
/// 二者与相机共用游戏已有owner，保留原先“地形先判、再判碰撞体”的有限线段查询。
pub fn update_water_audio_grid(
    mut state: Local<WaterAudioGridUpdateState>,
    water: Res<WaterRaster>,
    raycasting: Res<RaycastWorld>,
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

    for cell in &mut grid.cells {
        // O：单元基址 + 抖动 → `utils::swizzlers::x0y`（0x140c9cdc0）→ 采样点 {x,0,z}。
        let base = Vec2::from_array(cell.position);
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
                // 原141baf49c向1408cc320传入1.0：方向为eye-point，
                // 所以这里只检查采样点到相机之间，不能把相机后方算作遮挡。
                !raycasting.intersects_any_w_terrain(
                    &terrain, point, delta, 1.0, OCCLUSION_RAY_MASK,
                )
            }
        };

        // O：滚动窗置位/清位；0→1 累加增量对（PADDW），1→0 累减（PSUBW）
        //     （0x141baf4b3..4da / 0x141baf4e8..50c）。
        cell.set_sample(bit, visible, [delta_x, delta_z]);
    }
}

/// 早期CPU投影仅用于历史对照；生产更新直接写canonical网格，不调用该近似。
pub mod bit_projection;


#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::world_raster::DirtyTilePublisher;
    use country_core::systems::collision::world::{
        ColliderUpsert, RaycastGeoSource, RaycastMeshData, upsert_collider,
    };
    use glam::{Mat4, Quat};
    use std::{collections::HashMap, sync::Arc};

    const CENTER_CELL: usize = 55;

    fn world_with_water(collision: RaycastWorld, eye: Vec3, level: u8) -> World {
        let mut world = World::new();
        let mut water = WaterRaster::new(DirtyTilePublisher::unique());
        water.composite_mut().fill(level);
        water.scratch_mut().fill(level);
        let mut camera = MainCamera::default();
        camera.set_camera_matrices(Mat4::from_translation(eye));
        world.insert_resource(water);
        // 明确令整个地形低于水面，避免fixture把Y=0采样起点算作地形内部。
        let mut terrain = TerrainHeightsData::<TerrainHeightSsbo>::new();
        terrain.set_height_range(-2.5, -2.5);
        world.insert_resource(terrain);
        world.insert_resource(collision);
        world.insert_resource(camera);
        world.insert_resource(WaterAudioGrid::new());
        world
    }

    fn first_center_sample() -> Vec3 {
        let base = WaterAudioGrid::new().cells[CENTER_CELL].position;
        Vec3::new(base[0] - 0.8125, 0.0, base[1] - 4.0625)
    }

    fn update_schedule() -> Schedule {
        let mut schedule = Schedule::default();
        schedule.add_systems(update_water_audio_grid);
        schedule
    }

    #[test]
    fn canonical_grid_accumulates_rolls_clears_and_continues() {
        let mut world = world_with_water(RaycastWorld::new(), Vec3::Y * 10.0, 255);
        let mut schedule = update_schedule();
        schedule.run(&mut world);
        let grid = world.resource::<WaterAudioGrid>();
        assert_eq!(grid.cells.len(), 100);
        assert_eq!(grid.cells[CENTER_CELL].bits, 1 << 11);
        assert_eq!(grid.cells[CENTER_CELL].sums, [-1, -5]);
        for _ in 1..64 { schedule.run(&mut world); }
        let center = world.resource::<WaterAudioGrid>().cells[CENTER_CELL];
        assert_eq!(center.bits, u64::MAX);
        assert_eq!(center.sums, [0, 0]);

        world.resource_mut::<WaterRaster>().clear_all();
        for _ in 0..64 { schedule.run(&mut world); }
        assert!(world.resource::<WaterAudioGrid>().cells.iter()
            .all(|cell| cell.bits == 0 && cell.sums == [0, 0]));
        let bases = world.resource::<WaterAudioGrid>().cells.iter()
            .map(|cell| cell.position).collect::<Vec<_>>();
        world.resource_mut::<WaterAudioGrid>().clear_activity();
        {
            let mut water = world.resource_mut::<WaterRaster>();
            water.composite_mut().fill(255);
            water.scratch_mut().fill(255);
        }
        schedule.run(&mut world);
        let grid = world.resource::<WaterAudioGrid>();
        assert_eq!(grid.cells.iter().map(|cell| cell.position).collect::<Vec<_>>(), bases);
        assert_eq!(grid.cells[CENTER_CELL].bits, 1 << 11);
        assert_eq!(grid.cells[CENTER_CELL].sums, [-1, -5]);
    }

    #[test]
    fn occluders_after_the_camera_do_not_silence_water() {
        let point = first_center_sample();
        let eye = Vec3::Y * 10.0;
        let delta = eye - point;
        for (factor, visible) in [(0.5, false), (1.5, true)] {
            let mut collision = RaycastWorld::new();
            let mut instances = HashMap::new();
            assert!(matches!(upsert_collider(
                &mut collision, &mut instances, 1u32, RaycastGeoSource::Slot0,
                Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::ZERO)),
                point + delta * factor, Quat::IDENTITY,
            ), ColliderUpsert::Inserted { .. }));
            collision.flush_colliders();
            // 两个场景的无限射线都会命中；业务只应采纳有限线段内的碰撞体。
            assert!(collision.intersects_any(point, delta, OCCLUSION_RAY_MASK));
            let mut world = world_with_water(collision, eye, 255);
            update_schedule().run(&mut world);
            let cell = world.resource::<WaterAudioGrid>().cells[CENTER_CELL];
            assert_eq!(cell.bits != 0, visible, "遮挡参数={factor}");
            assert_eq!(cell.sums, if visible { [-1, -5] } else { [0, 0] });
        }
    }

    #[test]
    fn raster_threshold_distance_boundary_and_nan_keep_original_rejection() {
        let point = first_center_sample();
        for (level, eye, visible) in [
            (127, point + Vec3::Y * 10.0, false),
            (128, point + Vec3::Y * 10.0, true),
            (255, point + Vec3::Y * 80.0, false),
            (255, point + Vec3::Y * f32::from_bits(80.0f32.to_bits() - 1), true),
            (255, Vec3::new(f32::NAN, 10.0, 0.0), false),
        ] {
            let mut world = world_with_water(RaycastWorld::new(), eye, level);
            update_schedule().run(&mut world);
            assert_eq!(world.resource::<WaterAudioGrid>().cells[CENTER_CELL].bits != 0, visible);
        }
    }
}
