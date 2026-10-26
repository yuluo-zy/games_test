//! 原 B `sheep_unstuck`（0x1423246b0）：会话预热、真实碰撞/岩石/季节与唯一状态写回。
//! 140ae8320 为 ICF 共址；真实参数是 ActiveSession.glade，必须调用 is_winter，
//! 不能采用另一个共址符号 BackupSaveFileKind::is_corrupted 的名字猜业务。

use crate::components::{CritterLocomotion, Sheep, SheepMoveState, Transform};
use bevy_ecs::prelude::*;
use country_core::{
    asset_libraries::ssbo_library::AssetSsboLibrary,
    render::{extract::ComponentWrite, gpu_readback::GpuReadbacks},
    resources::{
        session::{ActiveSession, NewSessionStartedCmd, SessionLoadedCmd},
        terrain_height::TerrainHeightsData,
        terrain_rock_mask::TerrainRockMaskSsbo,
        time::GameTime,
        world_raster::WorldRaster,
        world_raster_defs::WaterRaster,
    },
    systems::{
        collision::world::RaycastWorld, ui::signifiers::signifier_create::UiSignifierStream,
    },
};
use glam::Vec3;

/// 原 Sheep +0/+4 的记录。保持现有拆分接口，但由实际出生路径附加，非第二份地形缓存。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct SheepGroundRecord {
    pub flag: u8,
    pub h: f32,
}

/// 原 Local 是一字节门：会话事件帧停止，随后再跳过一个无事件帧，然后正常执行。
#[derive(Default)]
pub struct FramesSinceNewSession(bool);

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn sheep_unstuck(
    time: Res<GameTime>,
    terrain: Res<TerrainHeightsData>,
    water: Res<WaterRaster>,
    buffers: Res<AssetSsboLibrary>,
    raycast: Res<RaycastWorld>,
    session: Res<ActiveSession>,
    _signifiers: Res<UiSignifierStream>,
    mut warmup: Local<FramesSinceNewSession>,
    mut new_sessions: EventReader<NewSessionStartedCmd>,
    mut loaded_sessions: EventReader<SessionLoadedCmd>,
    mut query: Query<(
        &Transform,
        ComponentWrite<Sheep>,
        &mut SheepGroundRecord,
        &mut CritterLocomotion,
    )>,
    readbacks: Option<Res<GpuReadbacks>>,
) {
    // 原分支按此顺序短路；第一批事件存在时不能提前消费第二个 reader。
    if new_sessions.read().count() != 0 || loaded_sessions.read().count() != 0 {
        warmup.0 = false;
        return;
    }
    if !warmup.0 {
        warmup.0 = true;
        return;
    }
    buffers
        .get::<TerrainRockMaskSsbo>()
        .expect("sheep_unstuck requires canonical mapped TerrainRockMaskSsbo");
    let rock = TerrainRockMaskSsbo::published(readbacks.as_deref());
    let water = water.0.as_ref();
    for (transform, mut sheep, mut ground, mut locomotion) in &mut query {
        let pos = transform.translation;
        // 原半径1，原点下移0.5；不能复用其它羊水平避障的0.8半径。
        let hit = raycast.legacy_spherecast(
            Vec3::new(pos.x, pos.y + -0.5, pos.z),
            Vec3::new(-0.0, -1.0, -0.0),
            1.0,
        );
        let hit_height = hit.is_hit().then_some(hit.position.y);
        let (rock, water) = if hit_height.is_none() {
            (
                TerrainRockMaskSsbo::sample_mapped_at_world_xz(rock, pos.x, pos.z) != 0,
                water.bilinear_sample_at_world_xz(pos.x, pos.z, 130.0, 130.0),
            )
        } else {
            (false, 0.0)
        };
        apply_unstuck(
            &mut locomotion,
            &mut ground,
            &mut sheep.move_state,
            pos.y,
            hit_height,
            rock,
            water,
            session.is_winter(),
            time.delta,
            || terrain.sample_at_world_xz(pos.x, pos.z),
        );
    }
}

/// 原每羊分支；外部射线/栅格读值已解析，所有状态写回仍在同一业务函数中决定。
#[allow(clippy::too_many_arguments)]
pub fn apply_unstuck(
    locomotion: &mut CritterLocomotion,
    ground: &mut SheepGroundRecord,
    move_state: &mut SheepMoveState,
    y: f32,
    hit_height: Option<f32>,
    rock: bool,
    water: f32,
    winter: bool,
    dt: f32,
    terrain_height: impl Fn() -> f32,
) {
    let mut keep_ground_record = false;
    if let Some(height) = hit_height {
        locomotion.0 = 1;
        locomotion.1 = height;
    } else {
        let state = locomotion.0;
        if rock && state <= 6 && (0x54_u32 >> state) & 1 != 0 {
            locomotion.0 = 0;
        } else if state == 2 {
            if water > 0.0 {
                locomotion.0 = if winter { 0 } else { 4 };
            } else {
                let height = terrain_height();
                // 原 MAXSS h,0.5 在NaN时选择0.5。
                let height = if height > 0.5 { height } else { 0.5 };
                let rate = (height - ground.h) / dt;
                if ground.flag & 1 != 0 && rate.is_finite() && rate.abs() > 10.0 {
                    locomotion.0 = 0;
                }
                ground.flag = 1;
                ground.h = height;
                keep_ground_record = true;
            }
        } else if (state.wrapping_sub(2) >= 5 || state == 5) && !rock && !(water > 0.0) {
            if y > terrain_height() {
                locomotion.0 = 6;
            }
        }
    }
    if !keep_ground_record {
        ground.flag = 0;
    }
    if locomotion.0 != 2 {
        if let SheepMoveState::Idle { timeout_s } = move_state {
            *timeout_s = 0.0;
        }
    }
}
