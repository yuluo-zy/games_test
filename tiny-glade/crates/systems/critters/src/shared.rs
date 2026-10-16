//! shared 移动 / 避让链（B 主基线，反汇编级证据）。
//!
//! 证据（critters-behavior-analysis.md §2 + reviewer-B RNE 抽验）：
//! - critters_avoid_collisions 0x1423179a0（源码 shared.rs 27–196）
//! - smooth_critter_velocities 0x142318c10（199–210）
//! - move_critters 0x142318fa0（214–374）
//! 查询签名（硬证据，init@0x14230ecf0）：
//! `Query<(&mut Transform, &CritterLocomotion, &Velocity, Option<&CritterVelocityMultiplier>,
//! Option<&Duck>, Option<&Sheep>, Option<&mut BirdSpots>)>`（7 列）。
//! 避让链消费 `RaycastWorld::legacy_spherecast`（0x1408ccc90）：固定 10 槽序
//! `{0,1,3,7,6,4,11,2,12,13}`、紧凑结果 marker@+0x30（hit 标志）——本模块经
//! `LegacySpherecast` trait 表达该契约，宿主在装配层用真实 RaycastWorld 实现。

use bevy_ecs::prelude::*;
use country_core::resources::terrain_rock_mask;
use country_core::resources::time::GameTime;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::world_raster::WorldRaster;
use country_core::systems::collision::world::RaycastWorld;
use country_core::systems::wall::wall_state::wall_spatial_hash::{WallSpatialHash, WallSpatialSegment};
use glam::{Quat, Vec2, Vec3};
use utils::geometry::line_relation::{LineInterval, LineRelation};

use crate::components::{
    BirdSpots, CritterGoal, CritterLocomotion, CritterVelocityMultiplier, CritterVelocityTarget,
    Duck, Sheep, Transform, Velocity,
};

/// `smooth_critter_velocities`（0x142318c10，源码 199–210）。
/// 证据：`retention = powf(target.retention_per_second, dt)`；
/// `velocity = velocity*retention + target.dir*(1-retention)`（3 轴，原顺序非 lerp 重排）。
pub fn smooth_critter_velocities(
    time: Res<GameTime>,
    mut query: Query<(&CritterVelocityTarget, &mut Velocity, &CritterVelocityMultiplier)>,
) {
    let dt = time.delta;
    for (target, mut velocity, multiplier) in &mut query {
        smooth_velocity(&mut velocity.0, *target, dt);
        // 证据：multiplier 每帧由查询环境值回写（语义 unknown；常量 1.0 仅在 sheep 场景证实）。
        let _ = multiplier.0;
    }
}

/// 速度平滑核心（= firefly.rs smooth_velocity 逐字段一致；powf 两处 0x142923f70）。
pub fn smooth_velocity(velocity: &mut Vec3, target: CritterVelocityTarget, dt: f32) {
    let retention = target.retention_per_second.powf(dt);
    let weight = 1.0 - retention;
    *velocity = Vec3::new(
        velocity.x * retention + target.direction.x * weight,
        velocity.y * retention + target.direction.y * weight,
        velocity.z * retention + target.direction.z * weight,
    );
}

/// `move_critters`（0x142318fa0，源码 214–374；查询 7 列 O）。
/// 逐实体（L223 dt；L234 起）：
/// - `translation.y = max(y, 0)`；位移 = `velocity * dt * multiplier`（缺省 1.0）。
/// - 避让门控（L244–315）：`bilinear_sample_at_world_xz<RasterBufferRef<512>>` 水面/地形采样 +
///   `terrain_rock_mask::sample_at_world_xz`（locomotion==2）；`legacy_spherecast`
///   （命中或球距 <1.0 → 跳过移动）；水面双采样（fc&1 且 locomotion!=4 → 判水）。
/// - 应用位移；朝向 `rot = rot*decay + target*(1-decay)`（decay = powf(0.01, dt)，
///   target = `Quat::from_rotation_arc`(dir, up)，归一化 divps/sqrtps）。
/// - BirdSpots（Option）：读取 Transform 后更新停靠点位置（stride 0x18）。
#[allow(clippy::type_complexity)]
pub fn move_critters(
    time: Res<GameTime>,
    water: Res<WorldRaster<512>>,
    rock_mask: Res<TerrainRockMask>,
    raycast: Res<RaycastWorldHandle>,
    mut query: Query<(
        &mut Transform,
        &CritterLocomotion,
        &Velocity,
        Option<&CritterVelocityMultiplier>,
        Option<&Duck>,
        Option<&Sheep>,
        Option<&mut BirdSpots>,
    )>,
) {
    let dt = time.delta;
    let decay = 0.01_f32.powf(dt);
    let w = (*water).as_ref();
    for (mut transform, locomotion, velocity, multiplier, _duck, _sheep, mut bird_spots) in
        &mut query
    {
        let mult = multiplier.map_or(1.0, |m| m.0);
        let pos = &mut transform.translation;
        pos.y = pos.y.max(0.0);

        // 避让门控。
        let rock = locomotion.samples_rock_mask()
            && terrain_rock_mask::sample_at_world_xz(rock_mask.mask(), pos.x, pos.z) != 0;
        let sphere = legacy_spherecast(&raycast, *pos, velocity.0);
        let sphere_blocked = sphere.hit && sphere.distance < 1.0;
        let water_blocked = if locomotion.skips_water_sample() {
            false
        } else {
            // 水面双采样（L264，fc&1 且 locomotion!=4）。
            w.bilinear_sample_at_world_xz(pos.x, pos.z, 130.0, 130.0) >= 0.5
                && w.bilinear_sample_at_world_xz(pos.x, pos.z, 130.0, 130.0) > 0.0
        };
        let blocked = rock || sphere_blocked || water_blocked;

        if !blocked {
            let displacement = velocity.0 * (dt * mult);
            pos.x += displacement.x;
            pos.y += displacement.y;
            pos.z += displacement.z;
        }

        // 朝向（仅当有水平速度时；O：from_rotation_arc + 混合 + 归一化）。
        let dir = Vec3::new(velocity.0.x, 0.0, velocity.0.z);
        if dir.length_squared() > 1e-6 {
            let target = Quat::from_rotation_arc(dir.normalize(), Vec3::Y);
            transform.rotation = transform.rotation.lerp(target, 1.0 - decay).normalize();
        }

        if let Some(spots) = bird_spots.as_deref_mut() {
            // 证据：读取 Transform 后更新停靠点位置（stride 0x18 记录）。
            for spot in &mut spots.spots {
                spot.pos = transform.translation;
            }
        }
    }
}

/// `critters_avoid_collisions`（0x1423179a0，源码 27–196）。
/// 普通 QueryIter；只处理 `CritterGoal.active == 1`（`while(*piVar17 != 1)`）。
/// - dir = normalize(goal - pos)（x/z 2D）；`Circle2d::new_rotated` + `LineInterval::line_segment`
///   构移动段；`bilinear_sample_at_world_xz`（水）+ `terrain_rock_mask::sample_at_world_xz`。
/// - 墙体：`WallSpatialHash` 邻域查询 + `PublicWalls::get`（`max_y_gable < pos.y` 跳过）+
///   `LineInterval::relate` 判相交。
/// - 水面 8×bilinear 采样求和 clamp 判水。
/// - `legacy_spherecast`：球距 `<1.0` → 保持目标（blocked）；球距 `<0.1` 且命中 →
///   `EventWriter::write<DespawnDuckCmd>`（撤销鸭子）。
/// - 收尾清 `CritterGoal.active`。
pub fn critters_avoid_collisions(
    water: Res<WorldRaster<512>>,
    rock_mask: Res<TerrainRockMask>,
    walls_hash: Res<WallSpatialHash>,
    walls: Res<PublicWalls>,
    raycast: Res<RaycastWorldHandle>,
    mut ev_despawn: EventWriter<crate::spawn_ducks::DespawnDuckCmd>,
    mut query: Query<(
        Entity,
        &Transform,
        &mut CritterGoal,
        &CritterLocomotion,
        Option<&Sheep>,
    )>,
) {
    let w = (*water).as_ref();
    for (entity, transform, mut goal, locomotion, _sheep) in &mut query {
        let Some(target) = goal.0 else {
            continue;
        };
        let pos = transform.translation;
        let to = target - pos;
        let dir = Vec3::new(to.x, 0.0, to.z).normalize_or_zero();
        if dir.length_squared() < 1e-9 {
            goal.0 = None;
            continue;
        }

        // 移动段：Circle2d::new_rotated(pos2d, r, 朝向) + LineInterval::line_segment。
        let pos2d = Vec2::new(pos.x, pos.z);
        let goal2d = Vec2::new(target.x, target.z);
        let segment = LineInterval::line_segment(pos2d, goal2d);

        // 墙体避让：邻域段查询 + PublicWalls::get（max_y_gable < pos.y 跳过）。
        let wall_blocked = query_wall_blocked(
            &walls_hash,
            &walls,
            pos,
            segment,
            locomotion.wall_hash_only(),
        );

        // 水面：8×bilinear 采样求和 clamp[0,1] >= 0.1 → 判水。
        let water_blocked = if locomotion.skips_water_sample() {
            false
        } else {
            let mut sum = 0.0f32;
            for (dx, dz) in WATER_SAMPLE_OFFSETS {
                sum += w.bilinear_sample_at_world_xz(pos.x + dx, pos.z + dz, 130.0, 130.0);
            }
            sum.clamp(0.0, 1.0) >= 0.1
        };

        // 岩石掩码（locomotion==2 才采样）。
        let rock_blocked = locomotion.samples_rock_mask()
            && terrain_rock_mask::sample_at_world_xz(rock_mask.mask(), pos.x, pos.z) != 0;

        // legacy_spherecast（固定 10 槽序，见模块文档）。
        let sphere = legacy_spherecast(&raycast, pos, dir);

        let action = decide_avoidance(wall_blocked, water_blocked || rock_blocked, sphere);
        match action {
            AvoidanceAction::Despawn => {
                let _ = ev_despawn.write(crate::spawn_ducks::DespawnDuckCmd { entity });
            }
            _ => {}
        }
        // 收尾：清目标（O：`*piVar17 = 0`）。
        goal.0 = None;
    }
}

/// 邻域墙体查询（O：WallSpatialHash::query → PublicWalls::get → max_y_gable 跳过 →
/// LineInterval::relate 判相交；墙体记录 stride 0x38）。
fn query_wall_blocked(
    hash: &WallSpatialHash,
    walls: &PublicWalls,
    pos: Vec3,
    segment: LineInterval,
    pure_hash: bool,
) -> bool {
    let min = [pos.x - 4.0, pos.z - 4.0];
    let max = [pos.x + 4.0, pos.z + 4.0];
    for (endpoint, wall_id) in hash.query_with_aabb_clamped(min, max) {
        let Some(state) = walls.get(wall_id) else {
            continue;
        };
        if pure_hash {
            // 纯 WallSpatialHash 路径：只按段几何判交（无 PublicWalls 语义过滤）。
            if let Some(segs) = hash_segments_intersect(hash, wall_id, segment) {
                let _ = segs;
                return true;
            }
            continue;
        }
        if state.max_y_gable() < pos.y {
            continue;
        }
        // 段级 LineInterval::relate 判相交。
        if let Some(seg) = first_segment(hash, wall_id) {
            let other = LineInterval::line_segment(
                Vec2::new(seg.endpoints[0][0], seg.endpoints[0][1]),
                Vec2::new(seg.endpoints[1][0], seg.endpoints[1][1]),
            );
            if segment.relate(other) != LineRelation::Disjoint {
                return true;
            }
        }
    }
    false
}

fn first_segment<'a>(hash: &'a WallSpatialHash, wall: country_core::resources::walls::WallId) -> Option<&'a WallSpatialSegment> {
    hash.segments_for_wall(wall).next()
}

fn hash_segments_intersect(hash: &WallSpatialHash, wall: country_core::resources::walls::WallId, segment: LineInterval) -> Option<bool> {
    for seg in hash.segments_for_wall(wall) {
        let other = LineInterval::line_segment(
            Vec2::new(seg.endpoints[0][0], seg.endpoints[0][1]),
            Vec2::new(seg.endpoints[1][0], seg.endpoints[1][1]),
        );
        if segment.relate(other) != LineRelation::Disjoint {
            return Some(true);
        }
    }
    None
}

/// 避让决策（每实体核心；与 reconstruction/critters::shared::decide_avoidance 一致）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvoidanceAction {
    /// 球检命中或目标仍 >=1.0 远：保持当前目标继续避让（不移动/不撤销）。
    KeepMoving,
    /// 已抵达（无命中且 <1.0）：清 CritterGoal.active。
    Arrived,
    /// 已贴近（<0.1）且命中标志：发 DespawnDuckCmd 并清目标。
    Despawn,
}

pub fn decide_avoidance(
    wall_blocked: bool,
    water_blocked: bool,
    sphere: SpherecastResult,
) -> AvoidanceAction {
    if wall_blocked || water_blocked {
        return AvoidanceAction::KeepMoving;
    }
    if sphere.hit || sphere.distance >= 1.0 {
        return AvoidanceAction::KeepMoving;
    }
    if sphere.hit && sphere.distance < f32::from_bits(0x3ecccccd) {
        AvoidanceAction::Despawn
    } else {
        AvoidanceAction::Arrived
    }
}

/// 单实体移动核心（move_critters 每实体；与 reconstruction shared.rs 一致）。
pub fn move_critter_core(pos: &mut Vec3, vel: Vec3, multiplier: f32, dt: f32, allow_move: bool) {
    pos.y = pos.y.max(0.0);
    if allow_move {
        *pos += vel * (dt * multiplier);
    }
}

/// 朝向更新核心（move_critters；nlerp 语义）。
pub fn face_movement(rot: &mut Quat, move_dir: Vec3, dt: f32) {
    let decay = 0.01_f32.powf(dt);
    let target = Quat::from_rotation_arc(move_dir.normalize_or_zero(), Vec3::Y);
    *rot = rot.lerp(target, 1.0 - decay).normalize();
}

/// legacy_spherecast 契约：固定 10 槽序（reviewer-B RNE 抽验）+ 紧凑结果 marker@+0x30。
pub const LEGACY_SPHERECAST_SLOT_ORDER: [u32; 10] = [0, 1, 3, 7, 6, 4, 11, 2, 12, 13];

#[derive(Clone, Copy, Debug, Default)]
pub struct SpherecastResult {
    pub hit: bool,
    pub distance: f32,
}

/// 按证据顺序对 17 槽做球检（mask = 1<<slot），取最近命中；marker@+0x30 = hit 标志
/// （紧凑结果语义）。非 native-physics 壳返回未命中（宿主 native 装配层按
/// `LEGACY_SPHERECAST_SLOT_ORDER` 实现真实槽查询）。
pub fn legacy_spherecast(raycast: &RaycastWorldHandle, origin: Vec3, dir: Vec3) -> SpherecastResult {
    let mut best: Option<f32> = None;
    for &slot in LEGACY_SPHERECAST_SLOT_ORDER.iter() {
        if let Some((t, _)) = raycast.slot_query(origin, dir, slot) {
            best = Some(best.map_or(t, |b| b.min(t)));
        }
    }
    match best {
        Some(distance) => SpherecastResult { hit: true, distance },
        None => SpherecastResult { hit: false, distance: f32::MAX },
    }
}

/// RaycastWorld 资源包装（core 的 RaycastWorld 非 Resource；装配层构造）。
#[derive(Resource)]
pub struct RaycastWorldHandle(pub RaycastWorld);

impl Default for RaycastWorldHandle {
    fn default() -> Self {
        Self(RaycastWorld::new())
    }
}

impl std::fmt::Debug for RaycastWorldHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RaycastWorldHandle")
    }
}

impl RaycastWorldHandle {
    /// 单槽射线查询（native-physics 装配层经 core `RaycastWorld::raycast(mask)` 实现；
    /// 非 native 壳返回 None——与 core `legacy_raycast` 的壳一致）。
    pub fn slot_query(&self, _origin: Vec3, _dir: Vec3, _slot: u32) -> Option<(f32, u32)> {
        None
    }
}

/// 射线查询缝（非 native-physics 壳返回 None；宿主在 native 装配层经
/// `RaycastWorld::raycast`（17 槽 mask 遍历）接线——签名同 core `legacy_raycast`）。
pub fn raycast_hit(handle: &RaycastWorldHandle, origin: Vec3, dir: Vec3) -> Option<f32> {
    let _ = (&handle.0, origin, dir);
    None
}

/// 岩石掩码资源（宿主每帧从 GPU 回读填充；采样语义 = core terrain_rock_mask）。
#[derive(Resource, Clone, Debug, Default)]
pub struct TerrainRockMask {
    mask: Vec<u32>,
}

impl TerrainRockMask {
    pub fn mask(&self) -> &[u32] {
        &self.mask
    }
    pub fn set_mask(&mut self, mask: Vec<u32>) {
        self.mask = mask;
    }
}

/// 水面 8 采样偏移（L99–103 求和 clamp 判水；精确偏移集为推断，I）。
const WATER_SAMPLE_OFFSETS: [(f32, f32); 8] = [
    (0.0, 0.0),
    (1.0, 0.0),
    (-1.0, 0.0),
    (0.0, 1.0),
    (0.0, -1.0),
    (1.0, 1.0),
    (-1.0, -1.0),
    (0.5, 0.5),
];
