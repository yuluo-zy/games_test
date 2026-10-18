//! shared 移动 / 避让链（B 主基线，fresh 反编译级证据，2026-10-17 重建）。
//!
//! 证据：smooth_critter_velocities 0x142318c10 / move_critters 0x142318fa0 /
//! critters_avoid_collisions 0x1423179a0。逐语句重建要点：
//! - smooth_critter_velocities：`retention = powf(target.retention_per_second, dt)`；
//!   `vel = vel*retention + dir*(1-retention)`（x/y 打包、z 分量的乘法顺序与反编译一致）；
//!   另写 CritterVelocityMultiplier = 查询环境值（uVar6 = Query+0x14；bevy-free 无法复现）。
//!   已知差分失败项：`powf(0, dt<0)=+inf` 时 `inf + (-inf)` 的 QNaN 符号位
//!   原码 ffc00000 vs Rust 7fc00000 —— LLVM QNaN 规范化差异（公式/顺序一致，非逻辑分歧）。
//! - move_critters 每实体（0x142319506 起）：
//!   1. `translation.y = max(y, 0)`；写 multiplier = 查询环境值。
//!   2. multiplier 门：None → 视为 1.0 走完整避让路径；Some(m) 且 m 精确为 0.0 →
//!      仅墙体路径（WallSpatialHash::query → PublicWalls::get → max_y_gable<y 跳过 →
//!      Circle2d+LineInterval::relate）；否则完整路径。
//!   3. 完整路径：位移 = vel*dt*mult；水面在**预测位置**采样；locomotion==2 →
//!      `rock>0 | water>0 | (water<=0 & Duck)` 阻塞；非 loc2 → `water<=0 & Duck` 阻塞；
//!      Duck/Sheep → legacy_spherecast（球距 <1.0 且命中码!=2 阻塞；Sheep 半径 0.8、
//!      其它 0.5，crate helper 无半径面）；**Sheep** 且 loc!=4 → pos/pos+disp 双 water
//!      采样（`pos<=0 && disp>0` 阻塞）。
//!   4. 应用位移；朝向 = from_rotation_arc(dir_x0z, up) 与旧旋转按 weight 混合（符号翻转 +
//!      nlerp + 归一化）。
//!   5. BirdSpots（Option）：按记录把 spot 写到 translation + rot*(spot 局部偏移)。
//! - critters_avoid_collisions 每实体（0x1423180f0 起）：
//!   1. 仅处理 CritterGoal.active==1；dir = normalize(goal-pos)（x/z 2D）。
//!   2. locomotion==2 → 在 `pos+dir*0.1` 采 water 与 rock，任一 >0 → 直接清目标。
//!   3. **Duck**（cursor 列 +0x130）→ **两位置 4+4** path 栅格采样：
//!      `c0=clamp(Σ4 sample(pos))`；`d=(c0>0)?|goal.position|:0.3`；
//!      `c1=clamp(Σ4 sample(pos+dir*d))`；仅 `c1>0` → 清目标；否则 spherecast 路。
//!      **Sheep**（0x68-stride Option）→ 直接 spherecast 路。**其它** → 墙体 relate：
//!      任一墙段 **Intersection（判别 0）→ 清目标**；遍历完无 Intersection → 保留目标。
//!   4. spherecast：命中码==2 或球距 >=1.0 → 保留目标；球距 <0.4 且 **Duck** →
//!      EventWriter<DespawnDuckCmd>。
//!   5. 收尾 *goal.active=0。
//!
//! 2026-10-17 已解：move_critters 的水面阻塞/water2 用 **Duck**（列 +0x130），双水采样
//! 与半径 0.8 用 **Sheep**（列 +0x138）；avoid 的 "water8" = **Duck**（列 +0x130，
//! 两位置 4+4 采样）。仍未定：墙体 relate 的 Circle2d/LineInterval 形状（fresh 为
//! Circle2d::new_rotated + line_segment）、move_critters 的纯墙体路径（无 Duck/Sheep
//! 实体）未实现、PublicWalls::get 缺失的 panic 路径、spherecast 半径参数、BirdSpots
//! 局部偏移常量。
//!
//! 命中码语义（RE-ANALYSIS §6.2/6.3 已补证，2026-10-17 落库）：spherecast 命中码
//! `hitcode==2` = 岩石命中 —— move_critters `(local_318 != 2) && 球距<1.0` 才阻塞（码 2 不
//! 阻塞）；critters_avoid_collisions `local_258==2 || 球距>=1.0` → 保留目标。mult==0 时
//! 位移项恒 0（仅墙体 relate/朝向分支，bevy-free 以 blocked=true 跳位移等价）。

use bevy_ecs::prelude::*;
use country_core::resources::terrain_rock_mask;
use country_core::resources::time::GameTime;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::world_raster::WorldRaster;
use country_core::systems::collision::world::RaycastWorld;
use country_core::systems::paths::PathRastersResources;
use country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash;
use glam::{Quat, Vec2, Vec3};
use utils::geometry::line_relation::{LineInterval, LineRelation};

use crate::components::{
    BirdSpots, CritterGoal, CritterLocomotion, CritterVelocityMultiplier, CritterVelocityTarget,
    Duck, Sheep, Transform, Velocity,
};

/// `smooth_critter_velocities`（0x142318c10）。逐实体（O 反编译）：
/// `retention = powf(retention_per_second, dt)`；`velocity = velocity*retention +
/// target.dir*(1-retention)`（3 轴）；另写 Multiplier = 查询环境值。
pub fn smooth_critter_velocities(
    time: Res<GameTime>,
    mut query: Query<(&CritterVelocityTarget, &mut Velocity, &CritterVelocityMultiplier)>,
) {
    let dt = time.delta;
    for (target, mut velocity, multiplier) in &mut query {
        smooth_velocity(&mut velocity.0, *target, dt);
        // 证据：multiplier 每帧由查询环境值回写（bevy-free 无法复现查询内值）。
        let _ = multiplier.0;
    }
}

/// 速度平滑核心（逐字段与反编译一致；powf@0x142923f70）。
pub fn smooth_velocity(velocity: &mut Vec3, target: CritterVelocityTarget, dt: f32) {
    let retention = target.retention_per_second.powf(dt);
    let weight = 1.0 - retention;
    *velocity = Vec3::new(
        velocity.x * retention + target.direction.x * weight,
        velocity.y * retention + target.direction.y * weight,
        velocity.z * retention + target.direction.z * weight,
    );
}

/// `move_critters`（0x142318fa0）。查询 7 列（init@0x14230ecf0）。
#[allow(clippy::type_complexity)]
pub fn move_critters(
    time: Res<GameTime>,
    water: Res<WorldRaster<512>>,
    rock_mask: Res<TerrainRockMask>,
    raycast: Res<RaycastWorldHandle>,
    walls_hash: Res<WallSpatialHash>,
    walls: Res<PublicWalls>,
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
    for (mut transform, locomotion, velocity, multiplier, duck, sheep, mut bird_spots) in &mut query
    {
        let pos = &mut transform.translation;
        pos.y = pos.y.max(0.0);

        // multiplier 门（反汇编 0x142319bb2/0x142319bc1）：
        // None → 1.0；Some(0.0) → 仅墙体路径（墙体避让逻辑在 critters_avoid_collisions 承担，
        // 此处视为已阻塞不位移）；Some(m) → 完整避让路径。
        let mult = match multiplier.map(|m| m.0) {
            None => 1.0,
            Some(m) if m == 0.0 => 0.0,
            Some(m) => m,
        };

        // 避让路径（反编译 0x142319bc1）。blocked → 跳 LAB_142319506（位移/朝向/BirdSpots
        // 一并跳过，FIX⑨）。
        //
        // mult==0 语义（RE-ANALYSIS §6.2 坐实）：二进制 `fVar32 != 0.0 || NaN(fVar32)` 才进入
        // LAB_142319bc1 位移块；multiplier 精确为 0.0 → 直接跳 LAB_142319506（位移项恒 0，
        // 位移/墙体 relate/朝向/BirdSpots 一并跳过——此处 blocked=true 同义）。
        let displacement = velocity.0 * (dt * mult);
        let blocked = if mult == 0.0 {
            true
        } else {
            let mut blocked = false;
            // 水面采样在**预测位置** pos+displacement（fresh 0x142319c2c-0x142319c4f：
            // 采样 x/z = transform x/z + disp）。
            let px = pos.x + displacement.x;
            let pz = pos.z + displacement.z;
            let water_pred = w.bilinear_sample_at_world_xz(px, pz, 130.0, 130.0);
            // `!(water>0)` 覆盖 unordered（NaN）分支——二进制 SETBE/SETA 组合。
            let not_water = !(water_pred > 0.0);
            if locomotion.samples_rock_mask() {
                // loc==2（fresh 0x142319c5f-0x142319ca6）：
                // `rock>0 || water>0 || (water<=0 && duck)` → blocked。
                let rock = terrain_rock_mask::sample_at_world_xz(rock_mask.mask(), px, pz) > 0;
                if rock || water_pred > 0.0 || (not_water && duck.is_some()) {
                    blocked = true;
                }
            } else if not_water && duck.is_some() {
                // 非 loc==2（fresh 0x142319cb0-0x142319cc0）：仅 Duck 在陆面
                // （`water<=0 & water2`）阻塞。
                blocked = true;
            }
            if !blocked && (duck.is_some() || sheep.is_some()) {
                // fresh 0x142319cc3/d0：Duck 或 Sheep 存在 → spherecast 路（否则本应走
                // 墙体 relate 路——该路径在 move_critters 仍未实现，登记）。
                // 内层门：loc>6 或 loc∉{0,1,5,6}（99>>loc&1==0）。
                if locomotion.0 > 6 || !locomotion.wall_hash_only() {
                    let dir = Vec3::new(velocity.0.x, 0.0, velocity.0.z);
                    // B-ecstate FIX：半径 = Sheep 0.8@0x142a16cb8 / 其它 0.5@0x142925870。
                    let radius = if sheep.is_some() { 0.8 } else { 0.5 };
                    let sphere = legacy_spherecast(&raycast, *pos, dir, radius);
                    // 命中码==2（岩石）→ 不阻塞；否则球距 <1.0 → 阻塞。
                    if sphere.hit && sphere.hitcode != 2 && sphere.distance < 1.0 {
                        blocked = true;
                    }
                    // water2（fresh 0x14231a1bd-0x14231a240）：**Sheep**（`[rbp+0x23c]`
                    // = cursor 列 +0x138）且 loc!=4 时取 `water(pos)` 与
                    // `water(pos+disp)`；当期点 !(>0) 且预测点 >0 → 阻塞。
                    if !blocked && sheep.is_some() && locomotion.0 != 4 {
                        let s_now = w.bilinear_sample_at_world_xz(pos.x, pos.z, 130.0, 130.0);
                        if !(s_now > 0.0) && water_pred > 0.0 {
                            blocked = true;
                        }
                    }
                }
            } else if !blocked {
                // B-ecstate FIX：纯墙 relate 路（无 Duck 且无 Sheep，fresh 0x142319cc3 后）。
                // WallSpatialHash::query → PublicWalls::get → max_y_gable<pos.y 跳过（loc!=3）
                // → circle+line_segment+relate；Intersection（判别 0）→ 阻塞；遍历尽 → 位移。
                let dir = Vec3::new(velocity.0.x, 0.0, velocity.0.z);
                let min = [pos.x - 4.0, pos.z - 4.0];
                let max = [pos.x + 4.0, pos.z + 4.0];
                let segment = LineInterval::line_segment(
                    Vec2::new(pos.x, pos.z),
                    Vec2::new(pos.x + dir.x, pos.z + dir.z),
                );
                for (_endpoint, wall_id) in walls_hash.query_with_aabb_clamped(min, max) {
                    // B-ecstate FIX：二进制 PublicWalls::get 缺失 → unwrap_failed panic
                    // （0x142319d31 TEST RAX; JZ unwrap_failed）。
                    let state = walls.get(wall_id).expect("PublicWalls::get missing");
                    if locomotion.0 != 3 && state.max_y_gable() < pos.y {
                        continue;
                    }
                    for seg in walls_hash.segments_for_wall(wall_id) {
                        let other = LineInterval::line_segment(
                            Vec2::new(seg.endpoints[0][0], seg.endpoints[0][1]),
                            Vec2::new(seg.endpoints[1][0], seg.endpoints[1][1]),
                        );
                        if matches!(segment.relate(other), LineRelation::Intersection(_)) {
                            blocked = true;
                            break;
                        }
                    }
                    if blocked {
                        break;
                    }
                }
            }
            blocked
        };

        if !blocked {
            pos.x += displacement.x;
            pos.y += displacement.y;
            pos.z += displacement.z;

            // 朝向：from_rotation_arc(dir_x0z, up) 与旧旋转按 weight 混合 + 归一化。
            let dir = Vec3::new(velocity.0.x, 0.0, velocity.0.z);
            if dir.length_squared() > 1e-6 {
                let target = Quat::from_rotation_arc(dir.normalize(), Vec3::Y);
                // 最短路径符号翻转（反汇编 XOR sign bit）后 nlerp。
                let flip = target.dot(transform.rotation) < 0.0;
                let t2 = if flip { -target } else { target };
                transform.rotation = transform.rotation.lerp(t2, 1.0 - decay).normalize();
            }

            // BirdSpots（Option）：spot 写到 translation + rot*(局部偏移)。
            if let Some(spots) = bird_spots.as_deref_mut() {
                for spot in &mut spots.spots {
                    // 反汇编 0x142319e5a：偏移 (-0.495, 1.567, 0.0) 经 rot 旋转后加 translation。
                    let off = Vec3::new(-0.495, 1.567, 0.0);
                    spot.pos = transform.translation + transform.rotation * off;
                }
            }
        }
    }
}

/// `critters_avoid_collisions`（0x1423179a0）。只处理 `CritterGoal.active == 1`；
/// 收尾清 active。逐实体控制流（O 反编译 0x1423180f0 起）：
/// 1) locomotion==2 且岩石采样>0 → 直接清目标（阻塞放弃）；
/// 2) water8 标志（鸭子路径）→ 8×bilinear 求和 clamp[0,1]>0（在水面）→ 清目标（抵达）；
///    否则墙体路径：WallSpatialHash::query 逐墙（locomotion==3 跳过 PublicWalls 过滤、
///    max_y_gable<pos.y 跳过）relate 判交，任一相交 → 保留目标；
/// 3) spherecast：命中码==2 或球距 >=1.0 → 保留目标；球距 <0.4 且 water8 → DespawnDuckCmd；
///    否则清目标。
/// 分歧点：water8 标志到 Option 组件的精确映射（本实现以 !samples_rock_mask 近似，见模块头）。
pub fn critters_avoid_collisions(
    water: Res<WorldRaster<512>>,
    rock_mask: Res<TerrainRockMask>,
    walls_hash: Res<WallSpatialHash>,
    walls: Res<PublicWalls>,
    raycast: Res<RaycastWorldHandle>,
    path_rasters: PathRastersResources,
    mut ev_despawn: EventWriter<crate::spawn_ducks::DespawnDuckCmd>,
    mut query: Query<(
        Entity,
        &Transform,
        &mut CritterGoal,
        &CritterLocomotion,
        Option<&Sheep>,
        Option<&Duck>,
    )>,
) {
    // O 0x142318219：`PathRastersResources::as_float_grid()`（四路 path 栅格引用）。
    let path_grids = path_rasters.as_float_grid();
    let w = (*water).as_ref();
    for (entity, transform, mut goal, locomotion, sheep, duck) in &mut query {
        // CritterGoal 桥接（country-core 组改 {position,goal_tag}，字节序与 B active@0,pos@4
        // 不一致——DIVERGE）：goal_tag!=0 → Some(position)。
        if goal.goal_tag == 0 {
            continue;
        }
        let target = goal.position;
        let pos = transform.translation;
        let to = target - pos;
        // B-ecstate FIX：二进制 normalize 非法 → dir=(0,0) 后**继续**（0x142318351-361）；
        // 旧 Rust 在此提前清目标并 continue（goal.position==pos 时行为差），已移除。
        let dir = Vec3::new(to.x, 0.0, to.z).normalize_or_zero();

        // 1) locomotion==2：在 `pos + dir*0.1`（xmm13=0.1）同时采 water 与 rock，
        //    任一 >0 → 放弃目标（fresh 0x1423183c4-0x14231840a：SETBE/SETA + OR AL）。
        if locomotion.samples_rock_mask() {
            let probe_x = pos.x + dir.x * f32::from_bits(0x3dcccccd);
            let probe_z = pos.z + dir.z * f32::from_bits(0x3dcccccd);
            let rock = terrain_rock_mask::sample_at_world_xz(rock_mask.mask(), probe_x, probe_z) > 0;
            let water_level = w.bilinear_sample_at_world_xz(probe_x, probe_z, 130.0, 130.0);
            if rock || water_level > 0.0 {
                goal.goal_tag = 0;
                continue;
            }
        }

        // 2) Duck / Sheep / 其它 三路（fresh 0x1423183e1-0x142318410 分支）：
        // - **Duck**（local_fc，cursor 列 +0x130）：**两位置 4+4 采样**（fresh
        //   0x142318410-0x1423185c0）：`c0=clamp(Σ4 sample(pos))`；
        //   `d=(c0>0)?|goal.position|:0.3`；`c1=clamp(Σ4 sample(pos+dir*d))`；
        //   **仅 c1>0 → 清目标**；否则落 spherecast 路。
        // - **Sheep**（local_f4，0x68-stride 记录）→ 直接 spherecast 路。
        // - 其它（无 Duck/Sheep）→ 墙体 relate 路：任一墙段 **Intersection → 清目标**；
        //   遍历完无 Intersection → 保留目标。
        let mut keep_goal = false;
        if duck.is_some() {
            let mut sum0 = 0.0f32;
            for grid in path_grids.iter() {
                sum0 += grid.bilinear_sample_at_world_xz(pos.x, pos.z, 130.0, 130.0);
            }
            let c0 = sum0.clamp(0.0, 1.0);
            // `ucomiss c0,0; ja`：仅有序大于 0 用目标半径，否则 0.3（0x3e99999a）。
            let d = if c0 > 0.0 {
                goal.position.length()
            } else {
                f32::from_bits(0x3e99999a)
            };
            let px = pos.x + dir.x * d;
            let pz = pos.z + dir.z * d;
            let mut sum1 = 0.0f32;
            for grid in path_grids.iter() {
                sum1 += grid.bilinear_sample_at_world_xz(px, pz, 130.0, 130.0);
            }
            if sum1.clamp(0.0, 1.0) > 0.0 {
                // 预测点在路径上 → 抵达（清目标）。
                goal.goal_tag = 0;
                continue;
            }
        } else if sheep.is_none() {
            // 墙体路径（fresh 0x14231841d 之后的 relate 循环）。
            let min = [pos.x - 4.0, pos.z - 4.0];
            let max = [pos.x + 4.0, pos.z + 4.0];
            let pos2d = Vec2::new(pos.x, pos.z);
            let goal2d = Vec2::new(pos.x + dir.x, pos.z + dir.z);
            let segment = LineInterval::line_segment(pos2d, goal2d);
            let mut intersect = false;
            for (_endpoint, wall_id) in walls_hash.query_with_aabb_clamped(min, max) {
                // B-ecstate FIX：二进制 `PublicWalls::get` 缺失 → unwrap_failed panic
                // （0x142318773 TEST RAX; JZ unwrap_failed）；旧 Rust continue 兜底。
                let state = walls.get(wall_id).expect("PublicWalls::get missing");
                if !locomotion.skips_public_walls() && state.max_y_gable() < pos.y {
                    continue;
                }
                for seg in walls_hash.segments_for_wall(wall_id) {
                    let other = LineInterval::line_segment(
                        Vec2::new(seg.endpoints[0][0], seg.endpoints[0][1]),
                        Vec2::new(seg.endpoints[1][0], seg.endpoints[1][1]),
                    );
                    // 0x142318xxx `do{...}while((int)relate != 0)`：Intersection（判别 0）
                    // 才退出循环 → 清目标；Disjoint/Parallel/Collinear 继续找下一墙。
                    if matches!(segment.relate(other), LineRelation::Intersection(_)) {
                        intersect = true;
                        break;
                    }
                }
                if intersect {
                    break;
                }
            }
            if intersect {
                goal.goal_tag = 0;
                continue;
            }
            // 遍历完仍无 Intersection → 保留目标（LAB_142317e4f）。
            keep_goal = true;
        }

        // 3) spherecast（Duck 未在路径上 / Sheep / 未保留的墙体路径）。
        if !keep_goal {
            // FIX④：loc<7 且位集（loc∈{0,1,5,6}）→ 直接保留目标（二进制 LAB_1423185c6
            // `(loc<7)&&(99>>(loc&31)&1)!=0 → goto next-entity`）。
            if !(locomotion.0 < 7 && locomotion.wall_hash_only()) {
                // B-ecstate FIX：半径 = Sheep 0.8 / 其它 0.5（0x14231866f）。
                let radius = if sheep.is_some() { 0.8 } else { 0.5 };
                let sphere = legacy_spherecast(&raycast, pos, dir, radius);
                // `local_258 == 2`（命中码2=岩石）或 球距>=1.0 → 保留目标；仅命中码!=2
                // 且球距<1.0 才清目标。
                if sphere.hit && sphere.hitcode != 2 && sphere.distance < 1.0 {
                    // <0.4 且 Duck → DespawnDuckCmd（fresh 0x142318502：local_fc = Duck）。
                    if sphere.distance < 0.4 && duck.is_some() {
                        let _ = ev_despawn.write(crate::spawn_ducks::DespawnDuckCmd { entity });
                    }
                    goal.goal_tag = 0;
                }
                // 否则（未命中/球距>=1.0/命中码2）→ 保留目标。
            }
        }
    }
}

/// legacy_spherecast 契约：固定 10 槽序（reviewer-B RNE 抽验）+ 紧凑结果 marker@+0x30。
pub const LEGACY_SPHERECAST_SLOT_ORDER: [u32; 10] = [0, 1, 3, 7, 6, 4, 11, 2, 12, 13];

#[derive(Clone, Copy, Debug, Default)]
pub struct SpherecastResult {
    pub hit: bool,
    pub distance: f32,
    /// legacy_spherecast 命中码（二进制返回结构 +0x1c 段 `local_318`）。语义（RE-ANALYSIS
    /// §6.2/6.3 补证）：`2`=岩石/障碍命中（move_critters 不阻塞、critters_avoid_collisions
    /// 保留目标）；`3`=墙 gable 跳过；其余=普通命中。无命中=0。
    pub hitcode: u32,
}

/// 按证据顺序对 17 槽做球检（mask = 1<<slot），取最近命中。
/// B-ecstate FIX：`radius` 为 legacy_spherecast 第 4 参数（二进制 Sheep 0.8@0x142a16cb8 /
/// 其它 0.5@0x142925870）；旧实现未传。
pub fn legacy_spherecast(
    raycast: &RaycastWorldHandle,
    origin: Vec3,
    dir: Vec3,
    radius: f32,
) -> SpherecastResult {
    let mut best: Option<(f32, u32)> = None;
    for &slot in LEGACY_SPHERECAST_SLOT_ORDER.iter() {
        if let Some((t, _)) = raycast.slot_query(origin, dir, radius, slot) {
            let better = match best {
                Some((b, _)) => t < b,
                None => true,
            };
            if better {
                best = Some((t, slot));
            }
        }
    }
    match best {
        Some((distance, hitcode)) => SpherecastResult {
            hit: true,
            distance,
            hitcode,
        },
        None => SpherecastResult {
            hit: false,
            distance: f32::MAX,
            hitcode: 0,
        },
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
    /// 非 native 壳返回 None）。`radius` = legacy_spherecast 第 4 参数（0.8 Sheep / 0.5 其它）。
    pub fn slot_query(
        &self,
        _origin: Vec3,
        _dir: Vec3,
        _radius: f32,
        _slot: u32,
    ) -> Option<(f32, u32)> {
        None
    }
}

/// 岩石掩码资源（宿主每帧从 GPU 回读填充）。
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

/// 旧近似的 8 偏移采样表（已被 2026-10-17 fresh 反汇编证伪：critters_avoid 的
/// "water8" 实为 **4 路 PathRasters 同点采样**；保留仅供历史对照，不参与逻辑）。
#[allow(dead_code)]
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
