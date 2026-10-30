//! 原 B `populate_autoclutter_regions`（1422d6ed0）。
//!
//! 恢复过期区域移除、全部订阅合并、脏格失效、候选冲刷、普通/固定/默认
//! mesh 选择、真实落点、墙/碰撞拒绝及完整候选写入。区域来源是必填 typed
//! owner；不按脏格凭空构造区域，也不从丢失的来源或 mesh 填默认值。
//!
//! 证据：140eecf00、1422d6ed0、1408b8e90；详见既有 M01 自动杂物报告。

use bevy_ecs::prelude::*;
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::render::gpu_readback::GpuReadbacks;
use country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo;
use country_core::resources::terrain_height::{TerrainHeightSsbo, TerrainHeightsData};
use country_core::resources::world_raster_defs::{GardenRaster, PathRaster, WaterRaster};
use country_core::resources::clutter_meshes::{ClutterMesh, ClutterMeshes, ClutterMeshTy};
use country_core::systems::collision::world::RaycastWorld;
use country_core::utils::dirty_tiles::DirtyTiles;
use glam::{Vec2, Vec3};
use tabloid::Subscriber;

use super::{
    AutoClutterCandidate, AutoClutterRegion, AutoClutterState, ClutterCandidateIndex,
    ClutterRegionId, GlobalId, LazyGridClutterCell,
};

/// 脏格订阅（build 里 insert_resource AutoClutterDirtySubscriptions）。
/// Subscriber 含 Cell 非 Sync，故以 Mutex 包裹（资源要求 Send+Sync）。
/// O（0x1422d6930）：13 个订阅句柄入 Vec——8× `RaycastWorld::subscribe_to_changes`、
/// `BushGrid` 与 `TerrainHeightsData` 各 1× `Publisher<DirtyTiles>::subscribe`、`PathRaster`/
/// `WaterRaster`/`GardenRaster` 各 1× `WorldRaster::subscribe`；填充见
/// [`autoclutter_subscribe_to_tabloids`]。
#[derive(Resource, Default)]
pub struct AutoClutterDirtySubscriptions {
    pub subscriber: std::sync::Mutex<Vec<Subscriber<DirtyTiles>>>,
}

/// populate 局部状态（PDB impl$0::init_state 0x1422e1b40）。
#[derive(Resource, Clone, Debug, Default)]
pub struct PopulateLocal {
    /// 已处理名集合（BTreeSet；pick_clutter_mesh 去重）。
    pub processed_names: std::collections::BTreeSet<String>,
    /// Reused region snapshot, retained in native table order.
    pub pending_regions: Vec<(ClutterRegionId, AutoClutterRegion)>,
}

/// `autoclutter_subscribe_to_tabloids`（Startup，0x1422d6930）。
///
/// 机器码证据：FunctionSystem 的 `validate_param_unsafe`（0x141ced3b0；错误串自 0x142f32706 起）依序给出
/// 七个 `ResMut` 参数：`AutoClutterDirtySubscriptions`、`RaycastWorld`、`BushGrid`、`PathRaster`、
/// `WaterRaster`、`GardenRaster`、`TerrainHeightsData<TerrainHeightSsbo>`。函数体按 O 顺序把 13 个
/// `Subscriber<DirtyTiles>` 句柄 push 入第一个资源的 Vec：
/// 8× `RaycastWorld::subscribe_to_changes(world, slot)`（0x1408cdc20，槽序 0,1,3,4,5,6,7,2）→
/// `BushGrid` 内联发布者（`*grid + 0x18`）`Publisher<DirtyTiles>::subscribe` →
/// `PathRaster`/`WaterRaster`/`GardenRaster` 各经 `TypedStorageBuffer::buffer()` 取内层
/// `WorldRaster<512>` 再 `WorldRaster::subscribe`（0x140a158e0）→
/// `TerrainHeightsData` 内联发布者（`*data + 0x18`）`Publisher<DirtyTiles>::subscribe`。
pub fn autoclutter_subscribe_to_tabloids(
    mut sub: ResMut<AutoClutterDirtySubscriptions>,
    mut raycast: ResMut<RaycastWorld>,
    mut bush: ResMut<BushGrid>,
    mut path: ResMut<PathRaster>,
    mut water: ResMut<WaterRaster>,
    mut garden: ResMut<GardenRaster>,
    mut terrain: ResMut<TerrainHeightsData<TerrainHeightSsbo>>,
) {
    let mut guard = sub.subscriber.lock().unwrap();
    for slot in [0u8, 1, 3, 4, 5, 6, 7, 2] {
        guard.push(raycast.subscribe_to_changes(slot));
    }
    guard.push(bush.publisher.subscribe());
    guard.push(path.subscribe());
    guard.push(water.subscribe());
    guard.push(garden.subscribe());
    guard.push(terrain.subscribe());
}

/// populate_autoclutter_regions（0x1422d6ed0；9 参数 SystemParam 面）。
#[allow(clippy::too_many_arguments)]
pub fn populate_autoclutter_regions(
    mut state: ResMut<AutoClutterState>,
    _dirty: ResMut<DirtyTilesResource>,
    water: Res<country_core::resources::world_raster_defs::WaterRaster>,
    (ssbos, readbacks): (Res<AssetSsboLibrary>, Option<Res<GpuReadbacks>>),
    mut sub: ResMut<AutoClutterDirtySubscriptions>,
    mut local: ResMut<PopulateLocal>,
    session: Res<country_core::resources::session::ActiveSession>,
    meshes: Res<ClutterMeshes>,
    bush: Res<BushGrid>,
    raycast: Res<RaycastWorld>,
    terrain: Res<TerrainHeightsData<TerrainHeightSsbo>>,
    paths: country_core::systems::paths::PathRastersResources,
    garden: Res<GardenRaster>,
    (balconies, gates, wall_grid, walls): (Query<&country_core::components::drawable::BalconyDoor>, Query<&country_core::components::WoodenGate>, Res<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>, Res<country_core::resources::walls::public_walls::PublicWalls>),
    mut _lazy: ResMut<LazyGridClutterCell>,
    _mesh_groups: ResMut<MeshGroupWeights>,
) {
    // 1. 过期区域清理（O：HashMap::retain，epoch != state+0x90 删除）。
    state.remove_stale_regions();

    // 2. 冲刷候选（2 处中的第一处）。
    state.flush_candidates_pending_removal();

    // 4. 脏格合入（O：new_clear → Subscriber::take → extend → 位集 mark_dirty）。
    let mut merged = DirtyTiles::new_clear();
    {
        let mut guard = sub.subscriber.lock().unwrap();
        for subscriber in guard.iter_mut() {
            if let Some(reader) = subscriber.take() {
                merged.extend(&reader);
            }
        }
    }
    // B 1422d6ed0：订阅的并集仅用于使既有区域候选失效。
    for cell in merged.tile_indices() {
        state.mark_dirty(cell);
    }
    // 3. 第二处冲刷。
    state.flush_candidates_pending_removal();

    // B 1422d6ed0 遍历真实区域表，不按脏格凭空新建区域。
    if state.region_candidates.len() < 0xfffd {
        // 仅扫描一次区域表；保留当前表的遍历顺序，避免逐项重复扫描。
        let mut pending = std::mem::take(&mut local.pending_regions);
        pending.clear();
        pending.extend(state.regions.iter()
            .filter(|(_, region)| region.candidate_index.0 == 0xffff)
            .map(|(&key, region)| (key, region.clone())));
        'regions: for index in 0..pending.len() {
            let (key, region) = &pending[index];
            if state.region_candidates.len() >= 0xfffd { break; }
                let (selected, mut point) = match region.b38 {
                    0 | 1 => {
                    let (mesh, mut rng) = pick_normal_mesh(&meshes, *key, region.b38, region.b39, session.meta.is_halloween2024());
                    (mesh, weighted_region_landing(&region, mesh.radius_xz, &mut rng))
                    }
                    2..=4 => {
                        let ty = match region.b38 {
                            2 => ClutterMeshTy::OldenRock,
                            3 => ClutterMeshTy::OldenTreeLog,
                            4 => ClutterMeshTy::OldenTreeStump,
                            _ => unreachable!(),
                        };
                        (meshes.mesh(ty), region.rectangle2d().center)
                    }
                    _ => (
                        pick_normal_mesh(
                            &meshes,
                            *key,
                            region.b38,
                            region.b39,
                            session.meta.is_halloween2024(),
                        )
                        .0,
                        region.rectangle2d().center,
                    ),
                };
                // B 1422d8107..8190：从区域高度向下射出，保留负零方向位。
                let hit = raycast.raycast_with_max_toi(
                    Vec3::new(point.x, region.center.y, point.y),
                    Vec3::new(-0.0, -1.0, -0.0), f32::MAX, 0x30fb);
                let building = hit.is_hit() && hit.handle[0] as u8 == 2 && ((hit.handle[0] >> 8) & 1) != 0;
                if building && region.b38 >= 2 {
                    state.regions.get_mut(key).expect("autoclutter region disappeared")
                        .candidate_index = ClutterCandidateIndex(0xfffe);
                    continue;
                }
                if hit.is_miss() || (hit.is_hit() && hit.is_terrain()) || building {
                    // B 1422d89a7：地形命中使用命中的 y；未命中才采地形。
                    let mut height = if hit.is_miss() {
                        terrain.sample_at_world_xz(point.x, point.y)
                    } else { hit.position.y };
                    if building { point = Vec2::new(hit.position.x, hit.position.z); }
                    // 原调用对四个路径栅格在同一世界坐标采样，而非水面四角。
                    let grids = paths.as_float_grid();
                    let path = if building { 0.0 } else { (grids[0].bilinear_sample_at_world_xz(point.x, point.y, 130.0, 130.0)
                        + grids[1].bilinear_sample_at_world_xz(point.x, point.y, 130.0, 130.0)
                        + grids[2].bilinear_sample_at_world_xz(point.x, point.y, 130.0, 130.0)
                        + grids[3].bilinear_sample_at_world_xz(point.x, point.y, 130.0, 130.0)).clamp(0.0, 1.0) };
                    // 1422d84df..8507：mode 1 且 selector != 1 时需要墙朝向。
                    if path > 0.0 && region.b38 >= 2 {
                        state.regions.get_mut(key).expect("autoclutter region disappeared")
                            .candidate_index = ClutterCandidateIndex(0xfffe);
                        continue;
                    }
                    let wall_forward = if selected.native_flag_47 || ((building || path > 0.0) && region.b38 != 1) {
                        // 1422d8b4f..8b62 查询区域矩形；没有墙段时原分支拒绝。
                        let mut closest = None;
                        let mut best = f32::MAX;
                        for segment in wall_grid.query_with_rectangle2d(region.rectangle2d()) {
                            let a = Vec2::from_array(segment.endpoints[0]);
                            let b = Vec2::from_array(segment.endpoints[1]);
                            let d = b - a;
                            let length2 = d.y * d.y + d.x * d.x;
                            let length2 = if length2 > f32::from_bits(0x1e3c_e508) {
                                length2
                            } else {
                                f32::from_bits(0x1e3c_e508)
                            };
                            let t = ((point.y - a.y) * d.y + (point.x - a.x) * d.x) / length2;
                            // MAXSS 0,t / MINSS 1,t 在无序时都取源 t。
                            let t = if 0.0 > t { 0.0 } else { t };
                            let t = if 1.0 < t { 1.0 } else { t };
                            let dx = point.x - (t * b.x + (1.0 - t) * a.x);
                            let dz = point.y - (t * b.y + (1.0 - t) * a.y);
                            let distance2 = dz * dz + dx * dx;
                            if distance2 < best {
                                let wall = walls.get_unchecked(segment.wall).expect("自动杂物墙段引用了不存在的墙");
                                let top = wall.max_y_gable();
                                if wall.authority.elevation() <= height + 0.99 && height + 0.99 < top {
                                    closest = Some(segment);
                                    best = distance2;
                                }
                            }
                        }
                        let Some(segment) = closest else {
                            state.regions.get_mut(key).expect("autoclutter region disappeared")
                                .candidate_index = ClutterCandidateIndex(0xfffe);
                            continue;
                        };
                        let a = Vec2::from_array(segment.endpoints[0]);
                        let b = Vec2::from_array(segment.endpoints[1]);
                        let (landing, normal, projected_height) = wall_landing(point, height, a, b, selected.bounds.footprint.size.y);
                        point = landing;
                        height = if building { projected_height } else { terrain.sample_at_world_xz(point.x, point.y) };
                        Some(normal)
                    } else { None };
                    // B 1422d8468：重新以输出键初始化随机流取得方向。
                    let output_key = utils::calculate_hash(&(key.0, 0u32));
                    let mut orientation_rng = utils::rng::rng_from_seed_u64(output_key);
                    let r = orientation_rng.f32();
                    let theta = r * f32::from_bits(0x40c90fdb) + (1.0 - r) * 0.0;
                    let forward = wall_forward.unwrap_or_else(|| if region.b38 >= 2 {
                        Vec2::new(-region.basis[0].y, region.basis[0].x)
                    } else { Vec2::new(theta.cos(), theta.sin()) });
                    let inv = 1.0 / (forward.x * forward.x + forward.y * forward.y).sqrt();
                    let mut rect = selected.bounds;
                    rect.translate(Vec3::new(point.x, height, point.y));
                    rect.set_orientation_xz(Vec2::new(-forward.y * inv, forward.x * inv));
                    if region.center.y - height > 4.0 || (!building && !terrain_slope_allowed(&terrain, rect)) {
                        // B 在进入放置尝试前写 0xfffe；拒绝保留此状态直到重新标脏。
                        state.regions.get_mut(key).expect("autoclutter region disappeared")
                            .candidate_index = ClutterCandidateIndex(0xfffe);
                        continue;
                    }
                    // B 1422d9001..999a：先取得 canonical 岩石资源，再按
                    // water、garden、rock 的顺序拒绝；无序比较不视为正数。
                    let rock = published_rock_mask(&ssbos, readbacks.as_deref());
                    if !building && ((*water).as_ref().bilinear_sample_at_world_xz(point.x, point.y, 130.0, 130.0) > 0.0
                        || (*garden).as_ref().bilinear_sample_at_world_xz(point.x, point.y, 130.0, 130.0) > 0.0
                        || TerrainRockMaskSsbo::sample_mapped_at_world_xz(rock, point.x, point.y) != 0) {
                        state.regions.get_mut(key).expect("autoclutter region disappeared")
                            .candidate_index = ClutterCandidateIndex(0xfffe);
                        continue;
                    }
                    if !building && selected.native_flag_46 {
                        if bushes_near_point(&bush, point, selected.radius_xz) {
                            state.regions.get_mut(key).expect("autoclutter region disappeared")
                                .candidate_index = ClutterCandidateIndex(0xfffe);
                            continue;
                        }
                    }
                    // 原 1422da3ee..a4a9：真实 BalconyDoor 的位置和方向
                    // 构造 1.5×2×2 矩形，与选中 mesh 的真实包围矩形比较。
                    if building {
                        let wall_id = country_core::resources::walls::WallId(hit.handle[1]);
                        if let Some(wall) = walls.get(wall_id) {
                            let inside = if let Some(mut circle) = wall.is_circle() {
                                circle.radius += -0.28;
                                circle.contains_point(point)
                            } else if let Some(rectangle) = wall.is_rectangle() {
                                rectangle.contains_point_padding(point, Vec2::splat(-0.56))
                            } else { true };
                            if !inside {
                                state.regions.get_mut(key).expect("autoclutter region disappeared")
                                    .candidate_index = ClutterCandidateIndex(0xfffe);
                                continue;
                            }
                        }
                    }
                    for door in balconies.iter().filter(|_| !building) {
                        let f = door.forward;
                        let inv = 1.0 / (f.z * f.z + f.x * f.x).sqrt();
                        let door_center = Vec3::new(
                            (f.x + f.x) * 0.5 + door.translation.x,
                            (f.y + f.y) * 0.5 + door.translation.y,
                            (f.z + f.z) * 0.5 + door.translation.z,
                        );
                        let door_rect = utils::geometry::rectangle::Rectangle3d::new(
                            door_center, Vec3::new(1.5, 2.0, 2.0), Vec2::new(-f.z * inv, f.x * inv));
                        if rect.overlaps_rectangle3d(door_rect) {
                            state.regions.get_mut(key).expect("autoclutter region disappeared")
                                .candidate_index = ClutterCandidateIndex(0xfffe);
                            continue 'regions;
                        }
                    }
                    // B 1422da589..daa66：使用真实门顶边首尾及其 3D 长度。
                    for gate in gates.iter().filter(|_| !building) {
                        let first = Vec3::from_array(*gate.top_positions.first().expect("门顶边为空"));
                        let last = Vec3::from_array(*gate.top_positions.last().expect("门顶边为空"));
                        let delta = first - last;
                        let planar = Vec2::new(last.x - first.x, last.z - first.z);
                        let inv = 1.0 / (planar.y * planar.y + planar.x * planar.x).sqrt();
                        let center = Vec3::new(last.x * 0.5 + first.x * 0.5 + 0.0,
                            last.y * 0.5 + first.y * 0.5 + 1.0,
                            last.z * 0.5 + first.z * 0.5 + 0.0);
                        let width = ((delta.y * delta.y + delta.x * delta.x) + delta.z * delta.z).sqrt() - 0.5;
                        let gate_rect = utils::geometry::rectangle::Rectangle3d::new(center,
                            Vec3::new(width, 2.0, 3.0), planar * inv);
                        if rect.overlaps_rectangle3d(gate_rect) {
                            state.regions.get_mut(key).expect("autoclutter region disappeared")
                                .candidate_index = ClutterCandidateIndex(0xfffe);
                            continue 'regions;
                        }
                    }
                    // B 1422dadf0..daf1b：先比较墙高区间，再检查墙段矩形。
                    let aabb = rect.footprint.aabb();
                    let min = aabb.center() - aabb.half_dims();
                    let max = aabb.center() + aabb.half_dims();
                    for segment in wall_grid.query_segments_with_aabb_clamped(min.to_array(), max.to_array()) {
                        let wall = walls.get(segment.wall).expect("自动杂物墙段引用了不存在的墙");
                        let bottom = wall.authority.elevation();
                        let top = wall.max_y_gable();
                        if (rect.bottom + 0.1 <= top) && (bottom <= rect.top) {
                            let a = Vec2::from_array(segment.endpoints[0]);
                            let b = Vec2::from_array(segment.endpoints[1]);
                            let delta = b - a;
                            let length = (delta.y * delta.y + delta.x * delta.x).sqrt();
                            let inv = 1.0 / length;
                            let axis = Vec2::new(-delta.y * inv, delta.x * inv);
                            let center = Vec2::new((a.x + b.x) * 0.5, (a.y + b.y) * 0.5);
                            let wall_rect = utils::geometry::Rectangle2d::new(center, Vec2::new(0.56, length), axis);
                            if rect.footprint.overlaps_rectangle2d(wall_rect) {
                            state.regions.get_mut(key).expect("autoclutter region disappeared")
                                    .candidate_index = ClutterCandidateIndex(0xfffe);
                                continue 'regions;
                            }
                        }
                    }
                    // 原 1422db2cd：真实物理世界对 mesh 矩形的最后一次拒绝。
                    if raycast.shape_overlaps_with(rect, 0x308c) {
                                state.regions.get_mut(key).expect("autoclutter region disappeared")
                            .candidate_index = ClutterCandidateIndex(0xfffe);
                        continue;
                    }
                    let source = region.source.clone();
                    assert!(selected.mesh.is_some(), "自动杂物选中的真实 mesh 尚未加载");
                    let index = state.region_candidates.len();
                    assert!(index <= 0xfffd, "自动杂物候选超过原版索引上限");
                    // 来源 union 克隆真实 owner；普通组的目的地是地形，
                    // marker 3 经过正常候选重叠比较，不走 marker 6 强制接受。
                    let candidate = AutoClutterCandidate {
                        kind: source.native_tag(),
                        resolved: Some(super::ResolvedAutoClutterObject {
                            source,
                            destination: if building {
                                let unchanged = point.x == hit.position.x && height == hit.position.y && point.y == hit.position.z;
                                country_core::components::drawable::AutoClutterDestination::OnBuilding {
                                    native_u32_04: if unchanged { hit.discriminant } else { 0 },
                                    coordinates: Vec2::from_array(hit.extra),
                                    wall: country_core::resources::walls::WallId(hit.handle[1]),
                                }
                            } else { country_core::components::drawable::AutoClutterDestination::Terrain },
                            metadata: selected.into(),
                        }),
                        forced: if matches!(region.b38, 2..=4) { 6 } else { 3 },
                        // B output +0x10 receives the generated tuple hash.
                        // +0x40 remains its independent source payload word.
                        key: output_key,
                        f48: u64::from(point.x.to_bits()) | (u64::from(height.to_bits()) << 32),
                        f50: point.y.to_bits(),
                        f54: u64::from(forward.x.to_bits()),
                        f5c: forward.y.to_bits(),
                        f60: 1.0f32.to_bits(),
                        rect,
                        region_key: key.0,
                        priority: region.f34 as u32,
                        ..Default::default()
                    };
                    state.regions.get_mut(key).expect("autoclutter region disappeared")
                        .candidate_index = ClutterCandidateIndex(index as u16);
                    state.region_candidates.push(candidate);
                    continue;
                }
                // B 143043840 跳表。The table is keyed by the hit source byte:
                // 0/1/3/6/7/8/9/10/13/14 reject, 2 accepts only the flat-roof
                // byte tested above, 11 is the terrain path already handled,
                // and 4/5/12/15/16 reach the original unreachable-code panic.
                let tag = hit.handle[0] as u8;
                match tag {
                    0 | 1 | 3 | 6 | 7 | 8 | 9 | 10 | 13 | 14 => {
                        state.regions.get_mut(key).expect("autoclutter region disappeared")
                            .candidate_index = ClutterCandidateIndex(0xfffe);
                        continue;
                    }
                    2 => {
                        debug_assert_eq!((hit.handle[0] >> 8) & 1, 0);
                        state.regions.get_mut(key).expect("autoclutter region disappeared")
                            .candidate_index = ClutterCandidateIndex(0xfffe);
                        continue;
                    }
                    4 | 5 | 12 | 15 | 16 => {
                        unreachable!("populate_autoclutter_regions: unreachable hit source {tag}")
                    }
                    11 => unreachable!("terrain hit source 11 must be handled before the jump table"),
                    _ => unreachable!("populate_autoclutter_regions: source byte {tag} is outside the 17-slot jump table"),
                }
            }
        local.pending_regions = pending;
    }
}

/// B 1422d952b..96d8：墙段投影、法向和 mesh 半宽偏移。
fn wall_landing(point: Vec2, height: f32, a: Vec2, b: Vec2, size: f32) -> (Vec2, Vec2, f32) {
    let delta = b - a;
    let length = (delta.y * delta.y + delta.x * delta.x).sqrt();
    let inv = 1.0 / length;
    let normal = Vec2::new(-inv * delta.y, inv * delta.x);
    // 原 xvy 将两个端点都扩成同一个候选高度。
    let vertical = height - height;
    let length2 = (delta.x * delta.x + vertical * vertical) + delta.y * delta.y;
    let length2 = if length2 > f32::from_bits(0x1e3c_e508) {
        length2
    } else {
        f32::from_bits(0x1e3c_e508)
    };
    let t = (((point.x - a.x) * delta.x + (height - height) * vertical)
        + (point.y - a.y) * delta.y) / length2;
    // Native conditional order: `0.0 <= t ? t : 0.0`, then `t <= 1.0 ?
    // t : 1.0`; NaN therefore becomes 0.0 and is not preserved.
    let t = if 0.0 <= t { t } else { 0.0 };
    let t = if t <= 1.0 { t } else { 1.0 };
    let offset = size * 0.5 + 0.28 + 0.05;
    (Vec2::new(normal.x * offset + (b.x * t + a.x * (1.0 - t)),
        normal.y * offset + (b.y * t + a.y * (1.0 - t))), normal,
        offset * 0.0 + (height * (1.0 - t) + height * t))
}

/// B 141cf3900：扫描真实 BushGrid 覆盖格，严格比较距离与半径+0.7。
fn bushes_near_point(grid: &BushGrid, point: Vec2, radius: f32) -> bool {
    // Aabb2::from_center_radius@141240070 对负数和 NaN 半径都会 panic。
    assert!(radius >= 0.0, "自动杂物灌木查询半径无效");
    let (minimum, end) = super::read_bush_positions_from_gpu::bush_rect_cells(
        point - Vec2::splat(radius), point + Vec2::splat(radius));
    let mut found = false;
    for y in minimum[1]..end[1] {
        for x in minimum[0]..end[0] {
            let cell = &grid.cells[x as usize + y as usize * 64];
            let mut row_hit = false;
            for bush in &cell.positions {
                let dx = bush.x - point.x;
                let dz = bush.y - point.y;
                if (dz * dz + dx * dx).sqrt() < radius + 0.7 {
                    found = true;
                    row_hit = true;
                    break;
                }
            }
            if row_hit { break; }
        }
    }
    found
}

/// B 1422d86ae..88ea：沿包围矩形两条轴的正负端点采地形高度。
fn terrain_slope_allowed(terrain: &TerrainHeightsData<TerrainHeightSsbo>, rect: utils::geometry::rectangle::Rectangle3d) -> bool {
    let axis = rect.footprint.axis;
    let inv = 1.0 / (axis.y * axis.y + axis.x * axis.x).sqrt();
    let x = axis.x * inv;
    let z = axis.y * inv;
    let center = rect.footprint.center;
    let half_width_x = (x * rect.footprint.size.x) * 0.5;
    let half_width_z = (z * rect.footprint.size.x) * 0.5;
    let half_depth_x = (-z * rect.footprint.size.y) * 0.5;
    let half_depth_z = (x * rect.footprint.size.y) * 0.5;
    let heights = [
        terrain.sample_at_world_xz(center.x + half_width_x, center.y + half_width_z),
        terrain.sample_at_world_xz(center.x - half_width_x, center.y - half_width_z),
        terrain.sample_at_world_xz(center.x + half_depth_x, center.y + half_depth_z),
        terrain.sample_at_world_xz(center.x - half_depth_x, center.y - half_depth_z),
    ];
    let min = heights[0].min(heights[1]).min(heights[2]).min(heights[3]);
    let max = heights[0].max(heights[1]).max(heights[2]).max(heights[3]);
    // B 1422d86ae..88ea: `scaled = (top - bottom) * 0.75`, then
    // `if 0.6 <= scaled { scaled = 0.6 }`. The raw comparison leaves NaN
    // untouched (NaN <= range is false), unlike `f32::min`.
    let scaled = (rect.top - rect.bottom) * 0.75;
    let threshold = if 0.6 <= scaled { 0.6 } else { scaled };
    // Native rejects when `threshold <= max - min`; NaN/unordered stays
    // accepted because the comparison is false.
    !(threshold <= (max - min))
}

/// B 1422d7755..1422d8090：普通组的九个真实 mesh 和对应权重。
/// 此函数只恢复选择，调用者仍须执行原始落点、碰撞和候选写入。
fn pick_normal_mesh(meshes: &ClutterMeshes, region: ClutterRegionId, selector: u8, group: u8, halloween: bool) -> (&ClutterMesh, fastrand::Rng) {
    let group_zero = [
        (ClutterMeshTy::WateringCan, 1.2), (ClutterMeshTy::BoxWStuff, 1.2),
        (ClutterMeshTy::PlantPotV1, 2.5), (ClutterMeshTy::Ladder, 1.0),
        (ClutterMeshTy::Wheel, 1.0), (ClutterMeshTy::Pitchfork, 1.0),
        (ClutterMeshTy::Shovel, 1.0),
        (ClutterMeshTy::Firewood, if selector == 1 { 0.0 } else { 1.5 }),
        (ClutterMeshTy::Bucket, 1.0),
    ];
    // 1422dd1d7..dd79b 的 b39=1；槽顺序及权重来自原写入顺序。
    let group_one = [
        (ClutterMeshTy::Firewood, if selector == 1 { 0.0 } else { 1.5 }),
        (ClutterMeshTy::Sunflower, 0.75), (ClutterMeshTy::RoadSign, 0.5),
        (ClutterMeshTy::PotatoSack, 1.0), (ClutterMeshTy::BasketWFood, 0.75),
        (ClutterMeshTy::Basket, 0.75), (ClutterMeshTy::Birdhouse, 0.5),
        (ClutterMeshTy::Haystack, 0.75), (ClutterMeshTy::BarrelWPlantpot, 0.75),
        (ClutterMeshTy::BarrelWFoodBasket, 0.75), (ClutterMeshTy::BenchWBasket, 1.0),
        (ClutterMeshTy::BenchShort, 1.0), (ClutterMeshTy::WateringCan, 1.2),
        (ClutterMeshTy::BoxWStuff, 1.2), (ClutterMeshTy::PlantPotV1, 2.0),
        (ClutterMeshTy::PlantPotV2, 1.5), (ClutterMeshTy::Ladder, 1.0),
        (ClutterMeshTy::Wheel, 1.0), (ClutterMeshTy::Pitchfork, 1.0),
        (ClutterMeshTy::Shovel, 1.0), (ClutterMeshTy::Bucket, 1.0),
    ];
    let group_two = [
        (ClutterMeshTy::Firewood, if selector == 1 { 0.0 } else { 1.5 }),
        (ClutterMeshTy::Sunflower, 0.75), (ClutterMeshTy::RoadSign, 0.5),
        (ClutterMeshTy::PotatoSack, 1.0), (ClutterMeshTy::BasketWFood, 0.75),
        (ClutterMeshTy::Basket, 0.75), (ClutterMeshTy::Birdhouse, 0.5),
        (ClutterMeshTy::Haystack, 0.75), (ClutterMeshTy::BarrelWPlantpot, 0.75),
        (ClutterMeshTy::BarrelWFoodBasket, 0.75), (ClutterMeshTy::BenchWBasket, 1.0),
        (ClutterMeshTy::BenchShort, 1.0), (ClutterMeshTy::WateringCan, 1.2),
        (ClutterMeshTy::BoxWStuff, 1.2), (ClutterMeshTy::PlantPotV1, 2.0),
        (ClutterMeshTy::PlantPotV2, 1.5), (ClutterMeshTy::Ladder, 1.0),
        (ClutterMeshTy::Wheel, 1.0), (ClutterMeshTy::Pitchfork, 1.0),
        (ClutterMeshTy::Shovel, 1.0), (ClutterMeshTy::Bucket, 1.0),
        (ClutterMeshTy::CratesWTools, 0.5), (ClutterMeshTy::CratesWFlowers, 0.5),
        (ClutterMeshTy::Cart, 0.5), (ClutterMeshTy::Cart2, 0.5),
    ];
    let group_three = [
        (ClutterMeshTy::Sunflower, 1.0), (ClutterMeshTy::RoadSign, 1.0),
        (ClutterMeshTy::PotatoSack, 1.0), (ClutterMeshTy::BasketWFood, 1.0),
        (ClutterMeshTy::Basket, 1.0), (ClutterMeshTy::Birdhouse, 1.0),
        (ClutterMeshTy::Haystack, 1.0), (ClutterMeshTy::BarrelWPlantpot, 0.75),
        (ClutterMeshTy::BarrelWFoodBasket, 0.75), (ClutterMeshTy::BenchWBasket, 0.75),
        (ClutterMeshTy::BenchShort, 0.75), (ClutterMeshTy::PlantPotV2, 1.0),
        (ClutterMeshTy::CratesWTools, 0.75), (ClutterMeshTy::CratesWFlowers, 0.75),
        (ClutterMeshTy::Cart, 0.75), (ClutterMeshTy::Cart2, 0.75),
        (ClutterMeshTy::WateringCan, 1.0), (ClutterMeshTy::BoxWStuff, 1.0),
        (ClutterMeshTy::PlantPotV1, 0.9), (ClutterMeshTy::Ladder, 1.0),
        (ClutterMeshTy::Wheel, 1.0), (ClutterMeshTy::Pitchfork, 1.0),
        (ClutterMeshTy::Shovel, 1.0),
        (ClutterMeshTy::Firewood, if selector == 1 { 0.0 } else { 1.0 }),
        (ClutterMeshTy::Bucket, 1.0),
    ];
    let group_four = [
        (ClutterMeshTy::BarrelWCandles, 1.0), (ClutterMeshTy::CartWPumpkinsRegular, 1.0),
        (ClutterMeshTy::CratesWPumpkinsRegular, 1.0), (ClutterMeshTy::Pumpkin, 1.0),
        (ClutterMeshTy::Pumpkin2, 1.0), (ClutterMeshTy::CrateFish, 1.0),
        (ClutterMeshTy::StallBread, 1.0), (ClutterMeshTy::StallVeggies, 1.0),
        (ClutterMeshTy::RackBrooms, 1.0), (ClutterMeshTy::Flowerbox, 0.75),
        (ClutterMeshTy::Anvil, 1.0), (ClutterMeshTy::Sunflower, 1.0),
        (ClutterMeshTy::RoadSign, 1.0), (ClutterMeshTy::PotatoSack, 1.0),
        (ClutterMeshTy::BasketWFood, 1.0), (ClutterMeshTy::Basket, 1.0),
        (ClutterMeshTy::Birdhouse, 1.0), (ClutterMeshTy::Haystack, 1.0),
        (ClutterMeshTy::BarrelWPlantpot, 0.75), (ClutterMeshTy::BarrelWFoodBasket, 0.75),
        (ClutterMeshTy::Bench, 0.75), (ClutterMeshTy::BenchWBasket, 0.25),
        (ClutterMeshTy::BenchShort, 0.75), (ClutterMeshTy::PlantPotV2, 1.0),
        (ClutterMeshTy::CratesWTools, 0.75), (ClutterMeshTy::CratesWFlowers, 0.75),
        (ClutterMeshTy::Cart, 0.75), (ClutterMeshTy::Cart2, 0.75),
        (ClutterMeshTy::WateringCan, 1.0), (ClutterMeshTy::BoxWStuff, 1.0),
        (ClutterMeshTy::PlantPotV1, 0.9), (ClutterMeshTy::Ladder, 1.0),
        (ClutterMeshTy::Wheel, 1.0), (ClutterMeshTy::Pitchfork, 1.0),
        (ClutterMeshTy::Shovel, 1.0),
        (ClutterMeshTy::Firewood, if selector == 1 { 0.0 } else { 1.0 }),
        (ClutterMeshTy::Bucket, 1.0),
    ];
    // 默认组在组4第十个元素之后插入 FabricRolls，B 1422dd7ca。
    let group_default: [(ClutterMeshTy, f32); 38] = std::array::from_fn(|i| {
        if i < 10 { group_four[i] } else if i == 10 { (ClutterMeshTy::FabricRolls, 1.0) } else { group_four[i - 1] }
    });
    let halloween_group = [
        (ClutterMeshTy::Pumpkin, 3.0), (ClutterMeshTy::Pumpkin2, 3.0),
        (ClutterMeshTy::PumpkinWFace, 3.0), (ClutterMeshTy::CartWPumpkins, 3.0),
        (ClutterMeshTy::CratesWPumpkins, 3.0), (ClutterMeshTy::OldenTreeStumpAutumn, 3.0),
        (ClutterMeshTy::BarrelWCandles, 2.0), (ClutterMeshTy::RoadSign, 1.0),
        (ClutterMeshTy::Haystack, 1.0), (ClutterMeshTy::BenchWBasket, 0.75),
        (ClutterMeshTy::BenchShort, 0.75), (ClutterMeshTy::CratesWTools, 0.75),
        (ClutterMeshTy::Wheel, 1.0), (ClutterMeshTy::Pitchfork, 1.0),
        (ClutterMeshTy::Shovel, 1.0),
        (ClutterMeshTy::Firewood, if selector == 1 { 0.0 } else { 1.0 }),
        (ClutterMeshTy::Bucket, 1.0),
    ];
    let entries: &[(ClutterMeshTy, f32)] = if halloween { &halloween_group } else { match group {
        0 => &group_zero, 1 => &group_one, 2 => &group_two, 3 => &group_three, 4 => &group_four,
        _ => &group_default,
    }};
    let mut weights = [0.0; 38];
    for (slot, &(ty, weight)) in entries.iter().enumerate() {
        weights[slot] = if meshes.mesh(ty).mesh.is_some() { weight } else { 0.0 };
    }
    let key = utils::calculate_hash(&(region.0, 0u32));
    let mut rng = utils::rng::rng_from_seed_u64(key);
    let random = rng.f32();
    let index = weighted_index(&weights[..entries.len()], random);
    (meshes.mesh(entries[index].0), rng)
}

/// 原始加权选择：有序累加，耗尽回到首槽，空列表触发边界失败。
fn weighted_index(weights: &[f32], random: f32) -> usize {
    let total = weights.iter().fold(0.0f32, |sum, weight| sum + weight);
    let mut sum = 0.0f32;
    for (index, weight) in weights.iter().enumerate() {
        sum += *weight / total;
        // UCOMISS/JBE：无序时也继续，最终可能触发原边界 panic。
        if sum > random { return index; }
    }
    // 列表耗尽时原 1422d7f14 将索引重置为 0，再执行边界检查。
    assert!(!weights.is_empty(), "populate_autoclutter_regions: weighted mesh pick bounds check failed");
    0
}

/// B 1422d7e..：按 mesh 半径收缩区域，以同一随机流生成候选 XZ。
fn weighted_region_landing(region: &AutoClutterRegion, radius: f32, rng: &mut fastrand::Rng) -> Vec2 {
    weighted_region_landing_samples(region, radius, rng.f32(), rng.f32())
}

fn weighted_region_landing_samples(region: &AutoClutterRegion, radius: f32, random_x: f32, random_z: f32) -> Vec2 {
    let axis = region.basis[0];
    let size = region.basis[1];
    let radius2 = radius + radius;
    let width = (size.x - radius2).max(0.01);
    let depth = (size.y - radius2).max(0.01);
    let base_x = (region.center.x + radius * axis.x) - radius * axis.y;
    let base_z = region.center.z + radius * axis.y + radius * axis.x;
    let offset_x = random_x * width - base_x;
    let offset_z = random_z * depth - base_z;
    let u = (offset_z * axis.y + offset_x * axis.x) / width;
    let v = (offset_z * axis.x + offset_x * -axis.y) / depth;
    let x = (u - u.floor()) * width;
    let z = (v - v.floor()) * depth;
    Vec2::new(z * -axis.y + x * axis.x + base_x,
        z * axis.x + x * axis.y + base_z)
}

/// 加权随机挑网格 + 位面落点（O：type 0/1 分支；权重累和 + fastrand f32；
/// 落点 = maxps(0.01, Δ) → floorf → xvy；0.01 = 0x1429cd300 packed 最小钳）。
pub fn weighted_mesh_landing(
    key: ClutterRegionId,
    groups: &MeshGroupWeights,
    mesh_type: u8,
    wx: f32,
    wz: f32,
) -> Vec3 {
    let entries = if mesh_type == 0 {
        &groups.group_0
    } else {
        &groups.group_1
    };
    let total: f32 = entries.iter().map(|e| e.weight).sum();
    let mut rng = utils::rng::rng_from_seed_u64(utils::rng::init_rng_u64(key.0));
    let r = rng.f32();
    let mut acc = 0.0f32;
    let mut chosen = Vec2::ZERO;
    for e in entries {
        acc += if total > 0.0 { e.weight / total } else { 0.0 };
        if r <= acc {
            chosen = e.offset;
            break;
        }
    }
    // 位面落点（O：maxps(0.01, Δ) → floorf 网格化 → xvy swizzle）。
    let delta = Vec2::new(wx, wz) + chosen;
    let clamped = Vec2::new(delta.x.max(0.01), delta.y.max(0.01));
    let cell = Vec2::new(clamped.x.floor(), clamped.y.floor());
    Vec3::new(cell.x, 0.0, cell.y)
}

/// 圆周落点（O：type>=3；θ = rng×2π（0x142934674），种子 = 键哈希 → cos/sin 单位圆）。
pub fn circular_position(key: ClutterRegionId) -> Vec3 {
    let mut rng = utils::rng::rng_from_seed_u64(utils::rng::init_rng_u64(key.0));
    let theta = rng.f32() * std::f32::consts::TAU;
    Vec3::new(theta.cos(), 0.0, theta.sin())
}

/// 脏格坐标列表（DirtyTiles 0x200B 位集；cell 语义）。
pub fn dirty_tiles_cells(dirty: &DirtyTiles) -> Vec<(i32, i32)> {
    let mut out = Vec::new();
    for idx in dirty.tile_indices() {
        let x = (idx % 64) as i32;
        let y = (idx / 64) as i32;
        out.push((x, y));
    }
    out
}

/// 格中心世界坐标（格 8 世界单位；世界半宽 65 → 格域 [-65,65]；I：机器码为 ×2 后
/// trunc 的位面中心流，近似对齐）。
pub fn cell_center_world(x: i32, y: i32) -> (f32, f32) {
    ((x as f32 + 0.5) * 8.0 - 65.0, (y as f32 + 0.5) * 8.0 - 65.0)
}

/// 格哈希键（`calculate_hash<tuple<GlobalId,u32>>` 0x140fabc80 语义；固定种子）。
pub fn hash_cell_key(x: i32, y: i32) -> u64 {
    utils::calculate_hash(&((x as i64), (y as u32)))
}

/// kind → 网格槽表（O：kind 跳表 0x143043818 各 case 写块，wave2 全量反编译逐槽提取；
/// 槽 = {网格表 `local_100` 下标, f32 权重}。网格表下标 → 具体 ClutterMeshTy 资产 =
/// U（①登记，需 load_clutter_meshes 资产清单对照））。
///
/// 各槽权重源码形如 `[entry+0x30] == -1 → 权重=0`（Option None 哨兵）。case 0/2/3 的
/// 权重经写块直接提取（O）；case 4 经 memmove 打包循环（0x18..0x278 stride 0x20）写出，
/// 槽序确定、部分权重寄存器复用歧义（④登记，取提取值）。
const KIND_MESH_SLOTS: &[(u8, &[(u8, f32)])] = &[
    // case '\0'：9 槽（+0x00..+0x40；槽 8（mesh 10）权重 = `(float)local_108 * fVar53`，
    // local_108 写前经 `CONCAT44(...,local_108*1.5)` 更新（行 768）→ scale×1.5 链，
    // scale=1.0（mesh-type≠1）→ 1.5）。
    (
        0,
        &[
            (9, 1.2),
            (8, 1.2),
            (3, 2.5),
            (4, 1.0),
            (5, 1.0),
            (6, 1.0),
            (7, 1.0),
            (10, 1.5),
            (11, 1.0),
        ],
    ),
    // case '\x02'：25 槽（+0x00..+0xc0；槽 0 权重 = scale*1.5，槽 14 = fVar61+fVar61=2.0）。
    (
        2,
        &[
            (10, 1.5),
            (25, 0.75),
            (24, 0.5),
            (23, 1.0),
            (21, 0.75),
            (20, 0.75),
            (19, 0.5),
            (17, 0.75),
            (16, 0.75),
            (22, 0.75),
            (14, 1.0),
            (15, 1.0),
            (9, 1.2),
            (8, 1.2),
            (3, 2.0),
            (18, 1.5),
            (4, 1.0),
            (5, 1.0),
            (6, 1.0),
            (7, 1.0),
            (11, 1.0),
            (28, 0.5),
            (29, 0.5),
            (26, 0.5),
            (27, 0.5),
        ],
    ),
    // case '\x03'：25 槽（+0x00..+0xc0；末槽权重 = scale*1.0）。
    (
        3,
        &[
            (25, 1.0),
            (24, 1.0),
            (23, 1.0),
            (21, 1.0),
            (20, 1.0),
            (19, 1.0),
            (17, 0.75),
            (16, 0.75),
            (22, 0.5),
            (14, 0.75),
            (15, 0.75),
            (18, 0.75),
            (28, 1.0),
            (29, 1.0),
            (26, 1.2),
            (27, 1.0),
            (9, 1.0),
            (8, 1.0),
            (3, 1.0),
            (4, 1.0),
            (5, 1.0),
            (6, 1.0),
            (7, 1.0),
            (10, 1.0),
            (11, 1.0),
        ],
    ),
    // case '\x04'：37 槽（+0x00..+0x238；memmove 打包循环写出，权重为提取值/1.0 主）。
    (
        4,
        &[
            (45, 1.0),
            (56, 1.0),
            (57, 1.0),
            (39, 1.0),
            (40, 1.0),
            (50, 1.0),
            (48, 1.0),
            (49, 1.0),
            (51, 1.0),
            (52, 1.0),
            (46, 1.0),
            (25, 1.0),
            (24, 1.0),
            (23, 1.0),
            (21, 1.0),
            (20, 1.0),
            (19, 1.0),
            (17, 1.0),
            (16, 1.0),
            (22, 1.0),
            (13, 1.0),
            (14, 1.0),
            (15, 1.0),
            (18, 1.0),
            (28, 1.0),
            (29, 1.0),
            (26, 1.0),
            (27, 1.0),
            (9, 1.0),
            (8, 1.0),
            (3, 1.0),
            (4, 1.0),
            (5, 1.0),
            (6, 1.0),
            (7, 1.0),
            (10, 1.0),
            (11, 1.0),
        ],
    ),
];

/// kind → 网格槽表（None = case 1 独立处理器 / default kind 5–25，body U（②登记））。
pub fn mesh_slots_for_kind(kind: u8) -> Option<&'static [(u8, f32)]> {
    KIND_MESH_SLOTS
        .iter()
        .find(|(k, _)| *k == kind)
        .map(|(_, slots)| *slots)
}

/// mesh-type 跳表分派（O：DAT_14304382c）：
/// - 0/1：加权随机挑网格（`init_rng_u64(键哈希)` + `fastrand::f32` + 权重/total 累和选中）；
/// - 2：跳过；3/4：固定候选槽 +0x25/+0x26；
/// - 5/6/8/11/12/13/14/15/18/19：透传（code_r0x0001422d7d8e，交下游）；
/// - 7：计数累积 [cand+0xad]++；9/0xa/0x11/0x14/0x15/0x10/0x18：错误/panic 路径（原为
///   `halt_baddata`/`slice_error_fail`，本重建以 None 跳过并登记）。
///
/// 返回选中槽的网格表下标（加权随机时含权重）。网格表下标 → ClutterMeshTy = U（①登记）。
///
/// 加权选中（O：行 2042-2065）：`do { acc += w/total; idx++ } while (acc <= r)`；
/// 下标从 -1 起步；原 UCOMISS/JBE 在等号和无序时继续，耗尽重置索引0；
/// 空列表仍在后续边界检查失败。
pub fn select_mesh(mesh_type: u8, kind: u8, key: ClutterRegionId) -> Option<u8> {
    match mesh_type {
        0 | 1 => {
            let slots = mesh_slots_for_kind(kind)?;
            let total: f32 = slots.iter().map(|s| s.1).sum();
            // O：`init_rng_u64(键哈希)` → `fastrand::Rng::f32()`。
            let mut rng = utils::rng::rng_from_seed_u64(utils::rng::init_rng_u64(key.0));
            let r = rng.f32();
            let mut acc = 0.0f32;
            let mut idx = 0usize;
            while idx < slots.len() {
                acc += slots[idx].1 / total;
                // 原 UCOMISS/JBE 对无序值也继续。
                if acc > r {
                    break;
                }
                idx += 1;
            }
            if idx == slots.len() && !slots.is_empty() { idx = 0; }
            // O：空列表的索引0仍会在边界检查处失败。
            assert!(
                idx < slots.len(),
                "populate_autoclutter_regions: weighted mesh pick bounds check failed"
            );
            Some(slots[idx].0)
        }
        2 => None, // O：跳过（case 2 break）。
        3 => Some(0x25),
        4 => Some(0x26),
        _ => None, // 透传/错误路径 U（本重建不虚构网格选择）。
    }
}

/// 网格组权重表（O：kind 跳表 0x143043818 预设槽组 + 权重；
/// 槽记录 +0x30 == -1（Option None 哨兵）→ 权重 0）。
/// 本重建以 KIND_MESH_SLOTS 静态表承载（见上），资源字段保留为装配层接口（兼容）。
#[derive(Resource, Clone, Debug, Default)]
pub struct MeshGroupWeights {
    /// mesh-type 0 的权重组（O：type 0/1 分支的加权随机挑网格）。
    pub group_0: Vec<MeshEntry>,
    /// mesh-type 1 的权重组。
    pub group_1: Vec<MeshEntry>,
    /// 万圣节组（O：is_halloween2024 → 槽序 +0x70..+0x168）。
    pub halloween: Vec<MeshEntry>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeshEntry {
    /// 权重（+0x30 == -1 哨兵 → 0）。
    pub weight: f32,
    /// 落点偏移（位面数学输入）。
    pub offset: Vec2,
}

impl MeshGroupWeights {
    /// 网格组选择（mesh-type/kind；本重建以 KIND_MESH_SLOTS + select_mesh 为准——
    /// 原 F3 近似 select_group 弃用，保留签名防装配层依赖）。
    #[deprecated(note = "wave2 以 KIND_MESH_SLOTS + select_mesh 取代；此近似勿用于判等")]
    pub fn select_group(&self, _key: ClutterRegionId, halloween: bool) -> (u8, u8) {
        if halloween {
            return (0, 4);
        }
        if !self.group_0.is_empty() {
            (0, 0)
        } else if !self.group_1.is_empty() {
            (1, 1)
        } else {
            (2, 2)
        }
    }
}

/// 注册表核类型身份，数据只借用已完成回读；首份回读前使用原缓冲零初始化内容。
fn published_rock_mask<'a>(
    library: &AssetSsboLibrary,
    readbacks: Option<&'a GpuReadbacks>,
) -> &'a [TerrainRockMaskSsbo] {
    library.get::<TerrainRockMaskSsbo>()
        .expect("autoclutter requires the canonical TerrainRockMaskSsbo");
    TerrainRockMaskSsbo::published(readbacks)
}

/// 会话元（is_halloween2024 门控 0x140b46860）。
#[derive(Resource, Clone, Debug, Default)]
pub struct SessionMeta {
    pub is_halloween2024: bool,
}

/// GPU 灌木网格：build 里 insert_resource 的唯一 `BushGrid`（read_bush 域）。此前这里有一个
/// 并行占位类型（`Vec<[f32;2]>`），ClutterPlugin 插入的是 read_bush 版，导致本系统的参数
/// 校验永远失败；统一为同一个类型。
pub use super::read_bush_positions_from_gpu::BushGrid;

/// 全局 id 包装（hash_one<&GlobalId>）。
pub fn global_id(cell: [i32; 2]) -> GlobalId {
    hash_cell_key(cell[0], cell[1])
}

/// 水面 4 角采样偏移（O：0x1422d8234.. 四角 bilinear，半径 130.0）。
pub const WATER_CORNER_SAMPLES: [(f32, f32); 4] = [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 1.0)];

/// DirtyTiles 资源包装（core `DirtyTiles` 非 Resource；装配层构造）。
#[derive(Resource, Clone, Debug, Default)]
pub struct DirtyTilesResource {
    pub inner: DirtyTiles,
}

/// 射线查询缝（legacy_raycast 掩码 0x30fb；非 native 壳返回 None）。
pub mod raycast_seam {
    /// legacy_raycast（O 0x1408cc870：掩码 0x30fb；结果 ==2 才走水面采样）。
    pub fn legacy_raycast_mask(_key: u64, _wx: f32, _wz: f32) -> bool {
        false
    }
}

#[cfg(test)]
mod subscribe_tests {
    use super::*;
    use country_core::resources::world_raster::DirtyTilePublisher;
    use country_core::resources::terrain_height::GRID_SIDE;

    #[test]
    fn weighted_mesh_choice_matches_original_x86() {
        let cases: &[([u32; 9], u32, usize)] = include!(concat!(env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m06-clutter-pick-native-cases.rs"));
        for (weights, random, expected) in cases {
            assert_eq!(weighted_index(&weights.map(f32::from_bits), f32::from_bits(*random)), *expected,
                "权重={weights:?} 随机数={random:#x}");
        }
    }

    #[test]
    fn terrain_slope_threshold_is_strict_even_on_flat_ground() {
        let terrain = TerrainHeightsData::<TerrainHeightSsbo>::new();
        let flat = utils::geometry::rectangle::Rectangle3d::new(Vec3::ZERO, Vec3::ONE, Vec2::X);
        assert!(terrain_slope_allowed(&terrain, flat));
        let zero_height = utils::geometry::rectangle::Rectangle3d::new(Vec3::ZERO,
            Vec3::new(1.0, 0.0, 1.0), Vec2::X);
        assert!(!terrain_slope_allowed(&terrain, zero_height));
        // Native `0.6 <= scaled` is false for NaN, so the threshold keeps NaN
        // and `NaN > range` is false: the placement is accepted.
        let nan_height = utils::geometry::rectangle::Rectangle3d::new(
            Vec3::new(0.0, f32::NAN, 0.0),
            Vec3::ONE,
            Vec2::X,
        );
        assert!(terrain_slope_allowed(&terrain, nan_height));
    }

    #[test]
    fn bush_rejection_uses_strict_distance_and_native_grid() {
        let mut grid = BushGrid::default();
        let (min, _) = super::super::read_bush_positions_from_gpu::bush_rect_cells(Vec2::splat(-1.0), Vec2::ONE);
        let index = min[0] as usize + min[1] as usize * 64;
        grid.cells[index].positions.push(Vec2::new(1.7, 0.0));
        assert!(!bushes_near_point(&grid, Vec2::ZERO, 1.0));
        grid.cells[index].positions[0].x = 1.6;
        assert!(bushes_near_point(&grid, Vec2::ZERO, 1.0));
    }

    #[test]
    fn weighted_landing_matches_original_x86_bits() {
        let cases: &[([u32; 9], [u32; 2])] = include!(concat!(env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m06-clutter-landing-native-cases.rs"));
        for (input, expected) in cases {
            let f = input.map(f32::from_bits);
            let region = AutoClutterRegion { center: Vec3::new(f[0], 0.0, f[1]),
                basis: [Vec2::new(f[2], f[3]), Vec2::new(f[4], f[5])], ..Default::default() };
            let point = weighted_region_landing_samples(&region, f[6], f[7], f[8]);
            assert_eq!([point.x.to_bits(), point.y.to_bits()], *expected, "输入 {input:?}");
        }
    }

    #[test]
    fn wall_landing_matches_original_x86_bits() {
        let cases: &[([u32; 8], [u32; 2], [u32; 3])] = include!(concat!(env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m06-clutter-wall-landing-native-cases.rs"));
        for (input, expected, direction) in cases {
            let [ax, az, bx, bz, x, y, z, size] = input.map(f32::from_bits);
            let (point, normal, _) = wall_landing(Vec2::new(x, z), y, Vec2::new(ax, az), Vec2::new(bx, bz), size);
            assert_eq!([point.x.to_bits(), point.y.to_bits()], *expected, "输入 {input:?}");
            assert_eq!([normal.x.to_bits(), 0, normal.y.to_bits()], *direction, "输入 {input:?}");
        }
        let roof_cases: &[([u32; 8], [u32; 3], [u32; 3])] = include!(concat!(env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m06-clutter-wall-roof-native-cases.rs"));
        for (input, expected, direction) in roof_cases {
            let [ax, az, bx, bz, x, y, z, size] = input.map(f32::from_bits);
            let (point, normal, height) = wall_landing(Vec2::new(x, z), y, Vec2::new(ax, az), Vec2::new(bx, bz), size);
            assert_eq!([point.x.to_bits(), point.y.to_bits(), height.to_bits()], *expected, "输入 {input:?}");
            assert_eq!([normal.x.to_bits(), 0, normal.y.to_bits()], *direction, "输入 {input:?}");
        }
    }

    #[test]
    fn wall_landing_clamps_nan_projection_to_zero_like_native_branches() {
        // Native conditional clamps: `0.0 <= t ? t : 0.0` and
        // `t <= 1.0 ? t : 1.0`, so a NaN projection becomes 0.0.
        let (point, normal, height) = wall_landing(
            Vec2::new(f32::NAN, 0.0),
            2.0,
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            1.0,
        );
        let offset = 1.0 * 0.5 + 0.28 + 0.05;
        assert_eq!(normal, Vec2::new(0.0, 1.0));
        assert_eq!(point, Vec2::new(0.0, offset));
        assert_eq!(height, 2.0);
    }

    #[test]
    fn normal_group_selects_the_available_real_mesh() {
        use country_core::asset_libraries::Handle;
        let mut fixed = std::array::from_fn(|_| ClutterMesh::omitted_in_demo("test/omitted", 0, None, false, false));
        fixed[6] = ClutterMesh::omitted_in_demo("test/watering-can", 0, None, false, false);
        fixed[6].mesh = Some(Handle::new());
        let meshes = ClutterMeshes::from_parts(fixed, vec![]);
        for key in [0, 1, u64::MAX, 0xfff779ed8e53fbdb] {
            assert_eq!(pick_normal_mesh(&meshes, ClutterRegionId(key), 0, 0, false).0.mesh_name,
                "test/watering-can");
        }
        let mut absent = std::array::from_fn(|_| ClutterMesh::omitted_in_demo("test/omitted", 0, None, false, false));
        absent[6].mesh_name = "test/first-zero-weight";
        let omitted = ClutterMeshes::from_parts(absent, vec![]);
        assert_eq!(pick_normal_mesh(&omitted, ClutterRegionId(0), 0, 0, false).0.mesh_name, "test/first-zero-weight");
    }

    #[test]
    fn group_one_mesh_selection_matches_original_weight_tables() {
        let cases: &[(u32, u8, [u32; 21])] = include!(concat!(env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m06-clutter-group1-native-cases.rs"));
        let types = [ClutterMeshTy::Firewood, ClutterMeshTy::Sunflower, ClutterMeshTy::RoadSign,
            ClutterMeshTy::PotatoSack, ClutterMeshTy::BasketWFood, ClutterMeshTy::Basket,
            ClutterMeshTy::Birdhouse, ClutterMeshTy::Haystack, ClutterMeshTy::BarrelWPlantpot,
            ClutterMeshTy::BarrelWFoodBasket, ClutterMeshTy::BenchWBasket, ClutterMeshTy::BenchShort,
            ClutterMeshTy::WateringCan, ClutterMeshTy::BoxWStuff, ClutterMeshTy::PlantPotV1,
            ClutterMeshTy::PlantPotV2, ClutterMeshTy::Ladder, ClutterMeshTy::Wheel,
            ClutterMeshTy::Pitchfork, ClutterMeshTy::Shovel, ClutterMeshTy::Bucket];
        for &(mask, selector, bits) in cases {
            let mut fixed = std::array::from_fn(|_| ClutterMesh::omitted_in_demo("test/mesh", 0, None, false, false));
            for (slot, &ty) in types.iter().enumerate() {
                if mask & (1 << slot) != 0 { fixed[ty as usize].mesh = Some(country_core::asset_libraries::Handle::new()); }
            }
            let meshes = ClutterMeshes::from_parts(fixed, vec![]);
            let key = ClutterRegionId(u64::from(mask));
            let mut rng = utils::rng::rng_from_seed_u64(utils::calculate_hash(&(key.0, 0u32)));
            let expected = weighted_index(&bits.map(f32::from_bits), rng.f32());
            let (selected, _) = pick_normal_mesh(&meshes, key, selector, 1, false);
            assert!(std::ptr::eq(selected, meshes.mesh(types[expected])), "mask {mask:#x}");
        }

    }

    #[test]
    fn group_two_and_three_selection_matches_original_weight_tables() {
        let cases: &[(u8, u32, u8, [u32; 25])] = include!(concat!(env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m06-clutter-group23-native-cases.rs"));
        let types2 = [ClutterMeshTy::Firewood, ClutterMeshTy::Sunflower, ClutterMeshTy::RoadSign,
            ClutterMeshTy::PotatoSack, ClutterMeshTy::BasketWFood, ClutterMeshTy::Basket, ClutterMeshTy::Birdhouse,
            ClutterMeshTy::Haystack, ClutterMeshTy::BarrelWPlantpot, ClutterMeshTy::BarrelWFoodBasket,
            ClutterMeshTy::BenchWBasket, ClutterMeshTy::BenchShort, ClutterMeshTy::WateringCan,
            ClutterMeshTy::BoxWStuff, ClutterMeshTy::PlantPotV1, ClutterMeshTy::PlantPotV2,
            ClutterMeshTy::Ladder, ClutterMeshTy::Wheel, ClutterMeshTy::Pitchfork, ClutterMeshTy::Shovel,
            ClutterMeshTy::Bucket, ClutterMeshTy::CratesWTools, ClutterMeshTy::CratesWFlowers,
            ClutterMeshTy::Cart, ClutterMeshTy::Cart2];
        let types3 = [ClutterMeshTy::Sunflower, ClutterMeshTy::RoadSign, ClutterMeshTy::PotatoSack,
            ClutterMeshTy::BasketWFood, ClutterMeshTy::Basket, ClutterMeshTy::Birdhouse, ClutterMeshTy::Haystack,
            ClutterMeshTy::BarrelWPlantpot, ClutterMeshTy::BarrelWFoodBasket, ClutterMeshTy::BenchWBasket,
            ClutterMeshTy::BenchShort, ClutterMeshTy::PlantPotV2, ClutterMeshTy::CratesWTools,
            ClutterMeshTy::CratesWFlowers, ClutterMeshTy::Cart, ClutterMeshTy::Cart2,
            ClutterMeshTy::WateringCan, ClutterMeshTy::BoxWStuff, ClutterMeshTy::PlantPotV1,
            ClutterMeshTy::Ladder, ClutterMeshTy::Wheel, ClutterMeshTy::Pitchfork, ClutterMeshTy::Shovel,
            ClutterMeshTy::Firewood, ClutterMeshTy::Bucket];
        for &(group, mask, selector, bits) in cases {
            let types = if group == 2 { &types2 } else { &types3 };
            let mut fixed = std::array::from_fn(|_| ClutterMesh::omitted_in_demo("test/mesh", 0, None, false, false));
            for (slot, &ty) in types.iter().enumerate() {
                if mask & (1 << slot) != 0 { fixed[ty as usize].mesh = Some(country_core::asset_libraries::Handle::new()); }
            }
            let meshes = ClutterMeshes::from_parts(fixed, vec![]);
            let key = ClutterRegionId(u64::from(mask));
            let mut rng = utils::rng::rng_from_seed_u64(utils::calculate_hash(&(key.0, 0u32)));
            let expected = weighted_index(&bits.map(f32::from_bits), rng.f32());
            let (selected, _) = pick_normal_mesh(&meshes, key, selector, group, false);
            assert!(std::ptr::eq(selected, meshes.mesh(types[expected])), "group {group}, mask {mask:#x}");
        }
    }

    #[test]
    fn group_four_selection_matches_original_weight_tables() {
        let cases: &[(u64, u8, [u32; 37])] = include!(concat!(env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m06-clutter-group4-native-cases.rs"));
        let types = [ClutterMeshTy::BarrelWCandles, ClutterMeshTy::CartWPumpkinsRegular,
            ClutterMeshTy::CratesWPumpkinsRegular, ClutterMeshTy::Pumpkin, ClutterMeshTy::Pumpkin2,
            ClutterMeshTy::CrateFish, ClutterMeshTy::StallBread, ClutterMeshTy::StallVeggies,
            ClutterMeshTy::RackBrooms, ClutterMeshTy::Flowerbox, ClutterMeshTy::Anvil,
            ClutterMeshTy::Sunflower, ClutterMeshTy::RoadSign, ClutterMeshTy::PotatoSack,
            ClutterMeshTy::BasketWFood, ClutterMeshTy::Basket, ClutterMeshTy::Birdhouse,
            ClutterMeshTy::Haystack, ClutterMeshTy::BarrelWPlantpot, ClutterMeshTy::BarrelWFoodBasket,
            ClutterMeshTy::Bench, ClutterMeshTy::BenchWBasket, ClutterMeshTy::BenchShort,
            ClutterMeshTy::PlantPotV2, ClutterMeshTy::CratesWTools, ClutterMeshTy::CratesWFlowers,
            ClutterMeshTy::Cart, ClutterMeshTy::Cart2, ClutterMeshTy::WateringCan,
            ClutterMeshTy::BoxWStuff, ClutterMeshTy::PlantPotV1, ClutterMeshTy::Ladder,
            ClutterMeshTy::Wheel, ClutterMeshTy::Pitchfork, ClutterMeshTy::Shovel,
            ClutterMeshTy::Firewood, ClutterMeshTy::Bucket];
        for &(mask, selector, bits) in cases {
            let mut fixed = std::array::from_fn(|_| ClutterMesh::omitted_in_demo("test/mesh", 0, None, false, false));
            for (slot, &ty) in types.iter().enumerate() {
                let tag = ty as usize;
                let index = if tag < 29 { tag } else { tag - 1 };
                if mask & (1 << slot) != 0 { fixed[index].mesh = Some(country_core::asset_libraries::Handle::new()); }
            }
            let meshes = ClutterMeshes::from_parts(fixed, vec![]);
            let key = ClutterRegionId(mask);
            let mut rng = utils::rng::rng_from_seed_u64(utils::calculate_hash(&(key.0, 0u32)));
            let expected = weighted_index(&bits.map(f32::from_bits), rng.f32());
            let (selected, _) = pick_normal_mesh(&meshes, key, selector, 4, false);
            assert!(std::ptr::eq(selected, meshes.mesh(types[expected])), "mask {mask:#x}");
        }
        let default_cases: &[(u64, u8, [u32; 38])] = include!(concat!(env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m06-clutter-groupdefault-native-cases.rs"));
        let default_types: [ClutterMeshTy; 38] = std::array::from_fn(|i|
            if i < 10 { types[i] } else if i == 10 { ClutterMeshTy::FabricRolls } else { types[i - 1] });
        for &(mask, selector, bits) in default_cases {
            let mut fixed = std::array::from_fn(|_| ClutterMesh::omitted_in_demo("test/mesh", 0, None, false, false));
            for (slot, &ty) in default_types.iter().enumerate() {
                let tag = ty as usize;
                let index = if tag < 29 { tag } else { tag - 1 };
                if mask & (1 << slot) != 0 { fixed[index].mesh = Some(country_core::asset_libraries::Handle::new()); }
            }
            let meshes = ClutterMeshes::from_parts(fixed, vec![]);
            let key = ClutterRegionId(mask);
            let mut rng = utils::rng::rng_from_seed_u64(utils::calculate_hash(&(key.0, 0u32)));
            let expected = weighted_index(&bits.map(f32::from_bits), rng.f32());
            let (selected, _) = pick_normal_mesh(&meshes, key, selector, 5, false);
            assert!(std::ptr::eq(selected, meshes.mesh(default_types[expected])), "default mask {mask:#x}");
        }
    }

    #[test]
    fn halloween_selection_matches_original_weight_tables() {
        let cases: &[(u32, u8, [u32; 17])] = include!(concat!(env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m06-clutter-halloween-native-cases.rs"));
        let types = [ClutterMeshTy::Pumpkin, ClutterMeshTy::Pumpkin2, ClutterMeshTy::PumpkinWFace,
            ClutterMeshTy::CartWPumpkins, ClutterMeshTy::CratesWPumpkins, ClutterMeshTy::OldenTreeStumpAutumn,
            ClutterMeshTy::BarrelWCandles, ClutterMeshTy::RoadSign, ClutterMeshTy::Haystack,
            ClutterMeshTy::BenchWBasket, ClutterMeshTy::BenchShort, ClutterMeshTy::CratesWTools,
            ClutterMeshTy::Wheel, ClutterMeshTy::Pitchfork, ClutterMeshTy::Shovel,
            ClutterMeshTy::Firewood, ClutterMeshTy::Bucket];
        for &(mask, selector, bits) in cases {
            let mut fixed = std::array::from_fn(|_| ClutterMesh::omitted_in_demo("test/mesh", 0, None, false, false));
            for (slot, &ty) in types.iter().enumerate() {
                let tag = ty as usize;
                let index = if tag < 29 { tag } else { tag - 1 };
                if mask & (1 << slot) != 0 { fixed[index].mesh = Some(country_core::asset_libraries::Handle::new()); }
            }
            let meshes = ClutterMeshes::from_parts(fixed, vec![]);
            let key = ClutterRegionId(u64::from(mask));
            let mut rng = utils::rng::rng_from_seed_u64(utils::calculate_hash(&(key.0, 0u32)));
            let expected = weighted_index(&bits.map(f32::from_bits), rng.f32());
            let (selected, _) = pick_normal_mesh(&meshes, key, selector, 0, true);
            assert!(std::ptr::eq(selected, meshes.mesh(types[expected])), "halloween mask {mask:#x}");
        }
    }

    #[test]
    fn population_reads_the_canonical_completed_rock_mask_without_a_local_mirror() {
        use country_core::render::ssbo::{MappedStorageBuffer, rhapsody_boundary::Buffer};
        let mut world = world();
        // 仅注册CPU夹具句柄；系统必须使用完成反馈，不可采样空mapping或在途GPU页。
        let library = AssetSsboLibrary::new();
        library.register::<TerrainRockMaskSsbo>(MappedStorageBuffer::from_mapping(
            &mut [], Buffer { id: 73, generation: 1, byte_size: 0x8000, ..Default::default() },
        ));
        assert_eq!(published_rock_mask(&library, None).len(), 0x2000);
        assert_eq!(TerrainRockMaskSsbo::sample_mapped_at_world_xz(
            published_rock_mask(&library, None), 65.0, 65.0), 0);
        world.insert_resource(library);
        let mut readbacks = GpuReadbacks::default();
        readbacks.publish_slice(&vec![TerrainRockMaskSsbo(u32::MAX); 0x2000], 3);
        world.insert_resource(readbacks);
        world.resource_mut::<GpuReadbacks>().publish_slice(
            &vec![TerrainRockMaskSsbo(0); 0x2000], 4,
        );
        assert_eq!(TerrainRockMaskSsbo::sample_mapped_at_world_xz(
            published_rock_mask(world.resource::<AssetSsboLibrary>(), Some(world.resource::<GpuReadbacks>())),
            65.0, 65.0), 0, "新完成读回必须可供下一次采样");
    }

    fn world() -> World {
        let mut world = World::new();
        world.init_resource::<country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash>();
        world.init_resource::<country_core::resources::walls::public_walls::PublicWalls>();
        world.insert_resource(AutoClutterDirtySubscriptions::default());
        world.insert_resource(RaycastWorld::new());
        world.insert_resource(BushGrid::default());
        world.insert_resource(PathRaster::new(DirtyTilePublisher::unique()));
        world.insert_resource(country_core::resources::world_raster_defs::Cobblestone1Raster::new(DirtyTilePublisher::unique()));
        world.insert_resource(country_core::resources::world_raster_defs::Cobblestone2Raster::new(DirtyTilePublisher::unique()));
        world.insert_resource(country_core::resources::world_raster_defs::Cobblestone3Raster::new(DirtyTilePublisher::unique()));
        world.insert_resource(WaterRaster::new(DirtyTilePublisher::unique()));
        world.insert_resource(GardenRaster::new(DirtyTilePublisher::unique()));
        world.insert_resource(TerrainHeightsData::<TerrainHeightSsbo>::new());
        world.init_resource::<country_core::systems::ui::signifiers::UiSignifierStream>();
        world.init_resource::<country_core::resources::gui::DebugGui>();
        world
    }

    #[test]
    fn dirty_tiles_do_not_manufacture_regions_or_mesh_candidates() {
        let mut world = world();
        let fixture = crate::autoclutter::window_test_helpers::world(vec![]);
        world.insert_resource(fixture.resource::<ClutterMeshes>().clone());
        world.insert_resource(country_core::resources::session::ActiveSession::new(
            country_core::resources::session::SessionId::new(), Default::default()));
        world.insert_resource(AutoClutterState { lazy_grid: vec![Vec::new(); 4096], ..Default::default() });
        let library = AssetSsboLibrary::new();
        library.register::<TerrainRockMaskSsbo>(country_core::render::ssbo::MappedStorageBuffer::from_mapping(
            &mut [], country_core::render::ssbo::rhapsody_boundary::Buffer { id: 74, generation: 1, byte_size: 0x8000, ..Default::default() }));
        world.insert_resource(library);
        world.init_resource::<PopulateLocal>();
        world.init_resource::<SessionMeta>();
        world.init_resource::<LazyGridClutterCell>();
        world.init_resource::<MeshGroupWeights>();
        let mut dirty = DirtyTilesResource::default();
        dirty.inner.set_cell(31, 27);
        world.insert_resource(dirty);
        let mut schedule = Schedule::default();
        schedule.add_systems(populate_autoclutter_regions);
        schedule.run(&mut world);
        let state = world.resource::<AutoClutterState>();
        assert!(state.regions.is_empty());
        assert!(state.region_candidates.is_empty());
    }

    #[test]
    fn fixed_mesh_selectors_reach_real_nani_from_regions() {
        let mut world = world();
        let mut fixed = std::array::from_fn(|_| ClutterMesh::omitted_in_demo("test/omitted", 0, None, false, false));
        for ty in [ClutterMeshTy::OldenRock, ClutterMeshTy::OldenTreeLog, ClutterMeshTy::OldenTreeStump] {
            let mesh = &mut fixed[ty as usize - 1];
            mesh.mesh = Some(country_core::asset_libraries::Handle::new());
            mesh.bounds = utils::geometry::rectangle::Rectangle3d::new(Vec3::ZERO, Vec3::ONE, Vec2::X);
        }
        world.insert_resource(ClutterMeshes::from_parts(fixed, vec![]));
        world.insert_resource(country_core::resources::session::ActiveSession::new(
            country_core::resources::session::SessionId::new(), Default::default()));
        let mut state = AutoClutterState { lazy_grid: vec![Vec::new(); 4096], ..Default::default() };
        for selector in 2..=4u8 {
            let key = ClutterRegionId(u64::from(selector));
            state.add_region(key, &AutoClutterRegion { key,
                source: country_core::components::drawable::AutoClutterSource::None, kind: 5,
                center: Vec3::new(f32::from(selector) * 5.0, 1.0, 0.0),
                basis: [Vec2::X, Vec2::splat(2.0)], b38: selector, b39: 0, ..Default::default() });
        }
        world.insert_resource(state);
        let library = AssetSsboLibrary::new();
        library.register::<TerrainRockMaskSsbo>(country_core::render::ssbo::MappedStorageBuffer::from_mapping(
            &mut [], country_core::render::ssbo::rhapsody_boundary::Buffer { id: 76, generation: 1, byte_size: 0x8000, ..Default::default() }));
        world.insert_resource(library);
        world.init_resource::<PopulateLocal>();
        world.init_resource::<LazyGridClutterCell>();
        world.init_resource::<MeshGroupWeights>();
        world.init_resource::<DirtyTilesResource>();
        world.init_resource::<country_core::resources::time::GameTime>();
        world.init_resource::<country_core::systems::ui_clutter::DeletedAutoClutter>();
        let mut schedule = Schedule::default();
        schedule.add_systems((populate_autoclutter_regions,
            super::super::process_autoclutter_candidates::process_autoclutter_candidates,
            super::super::manage_autoclutter_entities::manage_autoclutter_entities).chain());
        schedule.run(&mut world);
        let state = world.resource::<AutoClutterState>();
        assert_eq!(state.region_candidates.len(), 3);
        assert_eq!(state.spawn_candidates.len(), 3);
        assert!(state.spawn_candidates.iter().all(|spawn| spawn.marker == 6));
        let mut query = world.query::<&country_core::components::drawable::AutoClutterConvertableToDecoClutter>();
        assert_eq!(query.iter(&world).count(), 3);
    }

    #[test]
    fn default_weighted_selector_uses_rectangle_centre_and_marker_three() {
        let mut world = world();
        let ty = ClutterMeshTy::BarrelWCandles;
        let tag = ty as usize;
        let index = if tag < 29 { tag } else { tag - 1 };
        let mut fixed = std::array::from_fn(|_| {
            ClutterMesh::omitted_in_demo("test/omitted", 0, None, false, false)
        });
        fixed[index] = ClutterMesh::omitted_in_demo("test/default-selector", 0, None, false, false);
        fixed[index].mesh = Some(country_core::asset_libraries::Handle::new());
        fixed[index].bounds =
            utils::geometry::rectangle::Rectangle3d::new(Vec3::ZERO, Vec3::ONE, Vec2::X);
        world.insert_resource(ClutterMeshes::from_parts(fixed, vec![]));
        world.insert_resource(country_core::resources::session::ActiveSession::new(
            country_core::resources::session::SessionId::new(),
            Default::default(),
        ));
        let key = ClutterRegionId(0x5eed);
        let center = Vec3::new(2.0, 1.0, -3.0);
        let mut state =
            AutoClutterState { lazy_grid: vec![Vec::new(); 4096], ..Default::default() };
        state.add_region(
            key,
            &AutoClutterRegion {
                key,
                source: country_core::components::drawable::AutoClutterSource::None,
                kind: 5,
                center,
                basis: [Vec2::X, Vec2::splat(2.0)],
                b38: 5,
                b39: 5,
                ..Default::default()
            },
        );
        world.insert_resource(state);
        let library = AssetSsboLibrary::new();
        library.register::<TerrainRockMaskSsbo>(
            country_core::render::ssbo::MappedStorageBuffer::from_mapping(
                &mut [],
                country_core::render::ssbo::rhapsody_boundary::Buffer {
                    id: 78,
                    generation: 1,
                    byte_size: 0x8000,
                    ..Default::default()
                },
            ),
        );
        world.insert_resource(library);
        world.init_resource::<PopulateLocal>();
        world.init_resource::<LazyGridClutterCell>();
        world.init_resource::<MeshGroupWeights>();
        world.init_resource::<DirtyTilesResource>();
        world.init_resource::<country_core::resources::time::GameTime>();
        world.init_resource::<country_core::systems::ui_clutter::DeletedAutoClutter>();
        let mut schedule = Schedule::default();
        schedule.add_systems(
            (
                populate_autoclutter_regions,
                super::super::process_autoclutter_candidates::process_autoclutter_candidates,
                super::super::manage_autoclutter_entities::manage_autoclutter_entities,
            )
                .chain(),
        );
        schedule.run(&mut world);
        let state = world.resource::<AutoClutterState>();
        assert_eq!(state.region_candidates.len(), 1);
        assert_eq!(state.spawn_candidates.len(), 1);
        assert_eq!(state.spawn_candidates[0].marker, 3);
        let rectangle_centre = state.regions[&key].rectangle2d().center;
        assert_eq!(state.spawn_candidates[0].position.x, rectangle_centre.x);
        assert_eq!(state.spawn_candidates[0].position.z, rectangle_centre.y);
        let mut query = world.query::<&country_core::components::drawable::AutoClutterConvertableToDecoClutter>();
        assert_eq!(query.iter(&world).count(), 1);
    }

    #[test]
    fn flat_roof_hit_preserves_real_wall_destination_to_manager() {
        use country_core::systems::collision::world::{upsert_collider, RaycastGeoSource, RaycastMeshData};
        use country_core::components::drawable::{AutoClutterConvertableToDecoClutter, AutoClutterDestination, AutoClutterSource};
        let mut world = world();
        let mut fixed = std::array::from_fn(|_| ClutterMesh::omitted_in_demo("test/omitted", 0, None, false, false));
        fixed[6].mesh = Some(country_core::asset_libraries::Handle::new());
        fixed[6].bounds = utils::geometry::rectangle::Rectangle3d::new(Vec3::ZERO, Vec3::ONE, Vec2::X);
        world.insert_resource(ClutterMeshes::from_parts(fixed, vec![]));
        world.insert_resource(country_core::resources::session::ActiveSession::new(
            country_core::resources::session::SessionId::new(), Default::default()));
        let wall = country_core::resources::walls::WallId(91);
        let mut colliders = std::collections::HashMap::new();
        {
            let mut raycast = world.resource_mut::<RaycastWorld>();
            upsert_collider(&mut raycast, &mut colliders, 0u8, RaycastGeoSource::roof(wall, true, false),
                std::sync::Arc::new(RaycastMeshData::new_cuboid_at(Vec3::new(10.0, 0.5, 10.0), Vec3::ZERO)),
                Vec3::new(0.0, 2.0, 0.0), glam::Quat::IDENTITY);
            raycast.flush_colliders();
        }
        let mut state = AutoClutterState { lazy_grid: vec![Vec::new(); 4096], ..Default::default() };
        let key = ClutterRegionId(51);
        state.add_region(key, &AutoClutterRegion { key,
            source: AutoClutterSource::None, kind: 5,
            center: Vec3::new(0.0, 4.0, 0.0), basis: [Vec2::X, Vec2::ONE],
            b38: 1, b39: 0, ..Default::default() });
        world.insert_resource(state);
        let library = AssetSsboLibrary::new();
        library.register::<TerrainRockMaskSsbo>(country_core::render::ssbo::MappedStorageBuffer::from_mapping(
            &mut [], country_core::render::ssbo::rhapsody_boundary::Buffer { id: 77, generation: 1, byte_size: 0x8000, ..Default::default() }));
        world.insert_resource(library);
        world.init_resource::<PopulateLocal>();
        world.init_resource::<LazyGridClutterCell>();
        world.init_resource::<MeshGroupWeights>();
        world.init_resource::<DirtyTilesResource>();
        world.init_resource::<country_core::resources::time::GameTime>();
        world.init_resource::<country_core::systems::ui_clutter::DeletedAutoClutter>();
        let mut schedule = Schedule::default();
        schedule.add_systems((populate_autoclutter_regions,
            super::super::process_autoclutter_candidates::process_autoclutter_candidates,
            super::super::manage_autoclutter_entities::manage_autoclutter_entities).chain());
        schedule.run(&mut world);
        let mut query = world.query::<&AutoClutterConvertableToDecoClutter>();
        let [object] = query.iter(&world).collect::<Vec<_>>()[..] else { panic!("原屋顶命中应生成一个真实杂物") };
        assert!(matches!(object.destination, AutoClutterDestination::OnBuilding { wall: id, .. } if id == wall));
    }

    #[test]
    fn real_region_reaches_nani_from_a_complete_typed_source() {
        let mut world = world();
        let mut fixed = std::array::from_fn(|_| ClutterMesh::omitted_in_demo("test/omitted", 0, None, false, false));
        fixed[6].mesh = Some(country_core::asset_libraries::Handle::new());
        fixed[6].bounds = utils::geometry::rectangle::Rectangle3d::new(Vec3::ZERO, Vec3::ONE, Vec2::X);
        world.insert_resource(ClutterMeshes::from_parts(fixed, vec![]));
        world.insert_resource(country_core::resources::session::ActiveSession::new(
            country_core::resources::session::SessionId::new(), Default::default()));
        let mut state = AutoClutterState { lazy_grid: vec![Vec::new(); 4096], ..Default::default() };
        let key = ClutterRegionId(51);
        state.add_region(key, &AutoClutterRegion { key,
            source: country_core::components::drawable::AutoClutterSource::None, kind: 5,
            center: Vec3::new(0.0, 1.0, 0.0), basis: [Vec2::X, Vec2::ONE],
            b38: 0, b39: 0, ..Default::default() });
        world.insert_resource(state);
        let library = AssetSsboLibrary::new();
        library.register::<TerrainRockMaskSsbo>(country_core::render::ssbo::MappedStorageBuffer::from_mapping(
            &mut [], country_core::render::ssbo::rhapsody_boundary::Buffer { id: 75, generation: 1, byte_size: 0x8000, ..Default::default() }));
        world.insert_resource(library);
        world.init_resource::<PopulateLocal>();
        world.init_resource::<SessionMeta>();
        world.init_resource::<LazyGridClutterCell>();
        world.init_resource::<MeshGroupWeights>();
        world.init_resource::<DirtyTilesResource>();
        world.init_resource::<country_core::resources::time::GameTime>();
        world.init_resource::<country_core::systems::ui_clutter::DeletedAutoClutter>();
        let mut schedule = Schedule::default();
        schedule.add_systems((populate_autoclutter_regions,
            super::super::process_autoclutter_candidates::process_autoclutter_candidates,
            super::super::manage_autoclutter_entities::manage_autoclutter_entities).chain());
        schedule.run(&mut world);
        let state = world.resource::<AutoClutterState>();
        assert_eq!(state.region_candidates.len(), 1);
        assert_eq!(state.spawn_candidates.len(), 1);
        assert_eq!(state.spawn_candidates[0].marker, 3);
        assert_eq!(state.spawn_candidates[0].key, utils::calculate_hash(&(key.0, 0u32)));
        assert_eq!(state.regions[&key].candidate_index.0, 0);
        let mut query = world.query::<(&country_core::components::drawable::AutoClutter,
            &country_core::components::drawable::AutoClutterConvertableToDecoClutter,
            &country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::SolidVertexColorInstanceData>)>();
        assert_eq!(query.iter(&world).count(), 1);
    }

    fn pending(world: &World) -> Vec<bool> {
        let subs = world.resource::<AutoClutterDirtySubscriptions>();
        let mut guard = subs.subscriber.lock().unwrap();
        guard.iter_mut().map(|subscriber| subscriber.take().is_some()).collect()
    }

    /// 真实 World 上：七个参数资源全部存在 → 参数校验通过，且按机器码顺序订阅 13 个句柄；
    /// 之后向各源发布，只有对应序号的句柄收到（8 = BushGrid、9/10/11 = Path/Water/Garden 栅格、
    /// 12 = TerrainHeightsData）。
    #[test]
    fn subscribes_thirteen_handles_in_machine_order_and_validates_on_a_real_world() {
        let mut world = world();
        let mut schedule = Schedule::default();
        schedule.add_systems(autoclutter_subscribe_to_tabloids);
        schedule.run(&mut world);
        assert_eq!(pending(&world).len(), 13);
        assert!(pending(&world).iter().all(|got| !got));

        let mut tiles = DirtyTiles::new_clear();
        tiles.set_cell(1, 2);
        world.resource_mut::<BushGrid>().publisher.publish(tiles).unwrap();
        let got = pending(&world);
        assert_eq!(got.iter().position(|g| *g), Some(8));
        assert_eq!(got.iter().filter(|g| **g).count(), 1);

        world.resource_mut::<PathRaster>().copy_to_staging(0, &mut []);
        assert_eq!(pending(&world).iter().position(|g| *g), Some(9));
        world.resource_mut::<WaterRaster>().copy_to_staging(0, &mut []);
        assert_eq!(pending(&world).iter().position(|g| *g), Some(10));
        world.resource_mut::<GardenRaster>().copy_to_staging(0, &mut []);
        assert_eq!(pending(&world).iter().position(|g| *g), Some(11));

        let mut gpu = vec![0u16; GRID_SIDE * GRID_SIDE];
        gpu[0] = 7;
        world.resource_mut::<TerrainHeightsData<TerrainHeightSsbo>>().copy_from_gpu(&gpu);
        let got = pending(&world);
        assert_eq!(got.iter().position(|g| *g), Some(12));
        assert_eq!(got.iter().filter(|g| **g).count(), 1);
    }

    /// 缺任一资源 → 系统参数校验失败，不产生任何订阅。
    #[test]
    fn missing_resource_fails_validation_without_subscribing() {
        use bevy_ecs::system::{RunSystemError, RunSystemOnce};
        let mut world = world();
        world.remove_resource::<WaterRaster>();
        let result = world.run_system_once(autoclutter_subscribe_to_tabloids);
        assert!(matches!(result, Err(RunSystemError::InvalidParams { .. })), "{result:?}");
        assert_eq!(pending(&world).len(), 0);
    }
}
