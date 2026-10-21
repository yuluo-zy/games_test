//! 原1408153d0：墙、屋顶、地形与楼梯装饰的唯一逐帧变换生产者。
//! 保留当前/上一帧双缓冲、原必需读取次序与地形/楼梯即时脏标记。

#[path = "cache_roof_orientation.rs"]
mod roof_orientation;
#[path = "cache_terrain_support.rs"]
mod terrain_support;

use bevy_ecs::prelude::*;

use country_core::resources::clutter_meshes::{ClutterMeshTy, ClutterMeshes, clutter_mesh_ty_data};
use country_core::resources::roofs::roof_shape::Roof;
use country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume;
use country_core::resources::stairs::{
    stairs_assembly_params::PlayermadeStairsAssemblyRequests, stairs_state::StairsState,
};
use country_core::resources::terrain_height::TerrainHeightsData;
use country_core::resources::walls::WallId;
use country_core::resources::walls::cached_decorator_transforms::{
    CachedDecoratorTransform, CachedDecoratorTransformStorage, CachedDecoratorTransforms,
};
use country_core::resources::walls::decorator::{
    DecoratorDst, DecoratorId, DecoratorInfo, DecoratorRotationXZ,
};
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_storage::DecoratorStorageAddr;
use country_core::resources::walls::public_walls::PublicWalls;
use glam::{Vec2, Vec3};

/// 原九项系统输入；上一帧缓存只属于本系统的 Local。
pub fn cache_decorator_transforms(
    mut storage: ResMut<DecoratorStorage>,
    mut cache: ResMut<CachedDecoratorTransforms>,
    mut previous: Local<CachedDecoratorTransformStorage>,
    walls: Res<PublicWalls>,
    terrain: Res<TerrainHeightsData>,
    roof_trigger_volumes: Query<(&Roof, &RoofTriggerVolume)>,
    stairs_requests: Res<PlayermadeStairsAssemblyRequests>,
    meshes: Res<ClutterMeshes>,
    stairs: Res<StairsState>,
) {
    cache.swap_buffers_and_clear(&mut previous);
    for (wall, bucket) in storage.iter_walls() {
        for (id, info) in bucket.iter_maybe_mut() {
            // 原墙域只允许墙/屋顶；不能把误放入墙桶的地形或楼梯静默跳过。
            assert!(
                !matches!(info.dst_tag(), 3 | 4),
                "invalid destination in wall storage"
            );
            let transform = compute_surface_transform(
                id,
                info,
                Some(wall),
                &walls,
                &roof_trigger_volumes,
                &terrain,
                &meshes,
                &stairs_requests,
                previous.get(id),
            );
            cache.insert(id, transform);
        }
    }
    // get_mut 的顶层 immutable 门先于桶存在性；逐项 insert 成功后立即记脏。
    if let Some(bucket) = storage.get_mut(DecoratorStorageAddr::Terrain) {
        let (decorators, modified) = (&bucket.decorators, &mut bucket.modified_ids);
        for (&id, info) in decorators {
            assert_eq!(
                info.dst_tag(),
                3,
                "terrain storage contains a non-terrain decorator"
            );
            let transform = compute_surface_transform(
                id,
                info,
                None,
                &walls,
                &roof_trigger_volumes,
                &terrain,
                &meshes,
                &stairs_requests,
                previous.get(id),
            );
            if insert_compared_transform(&mut cache, id, transform, previous.get(id)) {
                modified.insert(id);
            }
        }
    }
    if let Some(bucket) = storage.get_mut(DecoratorStorageAddr::Stair) {
        let (decorators, modified) = (&bucket.decorators, &mut bucket.modified_ids);
        for (&id, info) in decorators {
            info.dst_as_stair();
            let transform = compute_stair_transform(info, &stairs_requests, &stairs);
            if insert_compared_transform(&mut cache, id, transform, previous.get(id)) {
                modified.insert(id);
            }
        }
    }
}

/// 近似相等时复用旧值全部位；新值校验失败不能先写 modified。
fn insert_compared_transform(
    cache: &mut CachedDecoratorTransforms,
    id: u32,
    transform: CachedDecoratorTransform,
    previous: Option<&CachedDecoratorTransform>,
) -> bool {
    let to_transform = |t: &CachedDecoratorTransform| utils::components::transform::Transform {
        r: t.rotation,
        t: t.translation,
        s: t.scale,
    };
    if let Some(old) = previous.filter(|old| {
        to_transform(old).abs_diff_eq(&to_transform(&transform), f32::from_bits(0x3a83126f))
    }) {
        cache.insert(id, *old);
        false
    } else {
        cache.insert(id, transform);
        true
    }
}

/// 保留 GraphMap 的第一个有向出邻接，不排序，也不查反向边。
fn adjusted_outgoing(requests: &PlayermadeStairsAssemblyRequests, id: u32) -> Option<Vec2> {
    let id = DecoratorId(id);
    let neighbor = requests
        .adjusted_edges
        .neighbors_directed(id, petgraph::Direction::Outgoing)
        .next()?;
    Some(
        requests
            .adjusted_edges
            .edge_weight(id, neighbor)
            .expect("adjusted stair neighbor has no edge")
            .outgoing_dir,
    )
}

fn stair_extra_height(info: &DecoratorInfo) -> f32 {
    let extra = info.extra().as_stair();
    f32::from_le_bytes(extra.0[..4].try_into().unwrap())
}

/// 复用现有几何内核；仅真实墙分支读取 PublicWallState。
#[allow(clippy::too_many_arguments)]
fn compute_surface_transform(
    id: u32,
    info: &DecoratorInfo,
    wall: Option<WallId>,
    walls: &PublicWalls,
    roof_trigger_volumes: &Query<(&Roof, &RoofTriggerVolume)>,
    terrain: &TerrainHeightsData,
    meshes: &ClutterMeshes,
    stairs_requests: &PlayermadeStairsAssemblyRequests,
    previous_transform: Option<&CachedDecoratorTransform>,
) -> CachedDecoratorTransform {
    match info.dst_tag() {
        // B140815875..894 normalizes every raw tag outside 2..=4 to the
        // wall branch. Keep the original parity below, even at the raw boundary.
        tag if tag.wrapping_sub(2) >= 3 => {
            let wall_state = walls
                .get(wall.expect("wall storage must provide its wall"))
                .expect("wall decorator refers to a missing public wall");
            let corner = crate::decorator_on_rectangle_edited::rectangle_corner_id(info.u20 as u8)
                .and_then(|corner| wall_state.corner_pos_and_normal_xz(corner as usize));
            let coord = info.wall_space_coord();
            let (translation, normal) = if let Some((position, normal)) = corner {
                (Vec3::new(position.x, coord.y, position.y), normal)
            } else {
                let curve = wall_state.simplified_curve_2d();
                let tangent = coord.get_tangent(curve);
                let normal = if info.dst_tag() & 1 == 0 {
                    Vec2::new(tangent.y, -tangent.x)
                } else {
                    Vec2::new(-tangent.y, tangent.x)
                };
                (coord.to_world_space(curve), normal)
            };
            CachedDecoratorTransform {
                rotation: utils::xz_rotation_bad(normal),
                translation,
                scale: Vec3::ONE,
            }
        }
        2 => {
            // B140815b22..c00. A missing roof association/UV uses the last
            // transform, or the original -100 sentinel on its first frame.
            // A stale/mismatched ECS roof entity is an unwrap failure instead.
            let fallback = || {
                previous_transform
                    .copied()
                    .unwrap_or(CachedDecoratorTransform {
                        rotation: glam::Quat::IDENTITY,
                        translation: Vec3::splat(-100.0),
                        scale: Vec3::ONE,
                    })
            };
            let Some(entity) = walls.roof_entity(wall.expect("roof storage must be a wall")) else {
                return fallback();
            };
            let (roof, volume) = roof_trigger_volumes
                .get(entity)
                .expect("roof association does not resolve to Roof and RoofTriggerVolume");
            let coord = info.wall_space_coord();
            let Some(mut translation) =
                volume.get_position_from_mesh_uv(Vec2::new(coord.u, coord.y), roof.floor_y())
            else {
                return fallback();
            };
            let tag = info.decorator_type().0;
            let mut rotation = if roof_orientation::roof_uses_yaw(tag) {
                DecoratorRotationXZ(
                    f32::from_bits(info.dst_secondary as u32) + roof.rotation_yaw_radians(),
                )
                .to_quat()
            } else {
                glam::Quat::IDENTITY
            };
            if !roof.is_flat() || (tag.wrapping_sub(0x38) < 8 && tag != 62) {
                if tag == 63 {
                    if let Some(direction) = adjusted_outgoing(stairs_requests, id) {
                        rotation = utils::xz_rotation_bad(direction);
                    }
                } else {
                    let pivot = roof.pivot_3d();
                    let normal = roof_orientation::roof_decorator_normal(
                        info.rank,
                        translation,
                        roof.observed()[8] & 1 != 0,
                        Vec2::new(pivot.x, pivot.z),
                        |query, position| {
                            let section = roof.get_roof_shape_from_pos_ws_clamped(query);
                            let country_core::resources::roofs::geometry::surface::RoofSection::Rectangle { frame, dimensions } = section else {
                            panic!("rectangular roof produced a non-rectangle section");
                        };
                            utils::geometry::Rectangle2d {
                                axis: Vec2::new(frame[0], frame[1]),
                                center: Vec2::new(frame[2], frame[3]),
                                size: Vec2::from_array(dimensions),
                            }
                            .closest_edge_normal(position)
                        },
                    );
                    rotation = roof_orientation::compose_roof_orientation(
                        rotation,
                        utils::xz_rotation_bad(normal),
                    );
                }
            }
            // 非平屋顶不读取 StairExtra；平屋顶63才叠加真实节点高度。
            if roof.is_flat() && tag == 63 {
                translation.y += stair_extra_height(info);
            }
            CachedDecoratorTransform {
                rotation,
                translation,
                scale: Vec3::ONE,
            }
        }
        3 => {
            let (x, z) = decorator_terrain_xz(info);
            let mut translation = Vec3::new(x, terrain.sample_at_world_xz(x, z), z);
            let mut rotation =
                DecoratorRotationXZ(f32::from_bits(info.dst_secondary as u32)).to_quat();
            if info.decorator_type().0 == 63 {
                translation.y += stair_extra_height(info);
                if let Some(direction) = adjusted_outgoing(stairs_requests, id) {
                    rotation = utils::xz_rotation_from_forward(direction);
                }
            } else if info.decorator_type().0.wrapping_sub(0x38) >= 8
                || info.decorator_type().0 == 62
            {
                let ty = ClutterMeshTy::from_native_tag(info.decorator_type().0)
                    .expect("terrain clutter type is outside the recovered native enum");
                let mut bounds = clutter_mesh_ty_data(ty, meshes).mesh.bounds;
                bounds.set_orientation_xz(terrain_support::terrain_clutter_axis(rotation));
                bounds.translate(translation);
                translation.y = terrain_support::terrain_clutter_support_height(
                    bounds.footprint.axis,
                    bounds.footprint.center,
                    bounds.footprint.size,
                    bounds.bottom,
                    bounds.top,
                    |point| terrain.sample_at_world_xz(point.x, point.y),
                );
            }
            CachedDecoratorTransform {
                rotation,
                translation,
                scale: Vec3::ONE,
            }
        }
        _ => unreachable!("surface transform requires a surface destination"),
    }
}

/// 原 SSE 源操作数政策：无序时取 source；相等时保留左值，含正负零。
fn numeric_min_left(left: f32, right: f32) -> f32 {
    if left.is_nan() || right < left {
        right
    } else {
        left
    }
}
fn numeric_max_left(left: f32, right: f32) -> f32 {
    if left.is_nan() || right > left {
        right
    } else {
        left
    }
}
fn max_source(value: f32, source: f32) -> f32 {
    if value > source { value } else { source }
}

fn compute_stair_transform(
    info: &DecoratorInfo,
    requests: &PlayermadeStairsAssemblyRequests,
    stairs: &StairsState,
) -> CachedDecoratorTransform {
    let DecoratorDst::Stair(destination) = info.destination() else {
        unreachable!("stair transform requires a stair destination");
    };
    let Some(params) = requests.requests.iter().find(|params| {
        let (a, b) = params.segment.nodes();
        (a.0, b.0) == (destination.first_id, destination.second_id)
    }) else {
        // 未命中先于 U 校验；负零与零缩放均是原业务值，仍写缓存并参与比较。
        return CachedDecoratorTransform {
            rotation: glam::Quat::IDENTITY,
            translation: Vec3::new(-0.0, -100.0, -0.0),
            scale: Vec3::ZERO,
        };
    };
    let u =
        utils::curve::CurveU::<utils::curve::CurveCoordSemantic2D>::new(destination.coordinate.y)
            .get();
    let coordinate = params.curve.get_coord_at_u(u);
    let i = coordinate.segment;
    let t = coordinate.t;
    let mut start = 0usize;
    let mut selected = 0usize;
    for segment in &params.sub_segments {
        let end = start.wrapping_add(usize::from(segment.count));
        if end > i {
            break;
        }
        start = end;
        selected += 1;
    }
    // 超过末段时仅夹索引，不重新计算 start；空列表保持原必需索引失败。
    selected = selected.min(params.sub_segments.len().saturating_sub(1));
    let segment = &params.sub_segments[selected];
    let tangent0 = params.tangents[i];
    let tangent1 = params.tangents[i + 1];
    let tangent = ((tangent1 - tangent0) * t + tangent0).normalize_or_zero();
    let normal = Vec2::new(-tangent.y, tangent.x);
    let interpolated = params.interpolated_curve.as_ref().unwrap();
    let p0 = interpolated.points[i];
    let p1 = interpolated.points[i + 1];
    let one_minus_t = 1.0 - t;
    let mut position = Vec3::new(
        p1.x * t + p0.x * one_minus_t,
        p1.y * t + p0.y * one_minus_t,
        p1.z * t + one_minus_t * p0.z,
    );
    if segment.kind == 2 {
        let end = start.wrapping_add(usize::from(segment.count));
        let coordinates = &interpolated.points_u[start..=end];
        let span = (*coordinates.last().unwrap() - coordinates[0]) * interpolated.length;
        let fraction = 0.5 / max_source(span, f32::from_bits(0x322bcc77));
        let fraction = if fraction < 1.0 { fraction } else { 1.0 };
        let steps = ((1.0 / fraction).round() as usize).max(1);
        let points = &interpolated.points[start..=end];
        let first_y = points[0].y;
        let last_y = points.last().unwrap().y;
        let low = numeric_min_left(first_y, last_y);
        let high = numeric_max_left(first_y, last_y);
        let step = (high - low) / steps as f32;
        position.y =
            ((position.y - low) / max_source(step, f32::from_bits(0x3727c5ac))).ceil() * step + low;
    } else if segment.kind == 3 {
        let end = start.wrapping_add(usize::from(segment.count));
        let points = &interpolated.points[start..=end];
        assert_eq!(
            points.len(),
            2,
            "a ladder subsegment must contain two points"
        );
        let (a, b) = params.segment.nodes();
        let [from, to] =
            crate::decorator_visual::stairs_assemble::calculate_adjusted_ladder_positions(
                &params.nodes,
                points[0],
                points[1],
                stairs,
                &requests.previous.sub_segments,
                [a, b],
            );
        let downhill = if to.y > from.y {
            Vec2::new(from.x - to.x, from.z - to.z)
        } else {
            Vec2::new(to.x - from.x, to.z - from.z)
        }
        .normalize_or_zero();
        let difference = to - from;
        let distance = (difference.z * difference.z
            + (difference.y * difference.y + difference.x * difference.x))
            .sqrt();
        let steps = ((distance / f32::from_bits(0x3f266666)).ceil() as usize).max(3);
        let step = (to.y - from.y) / (steps - 1) as f32;
        position.y = ((position.y - from.y) / max_source(step, f32::from_bits(0x3727c5ac))).ceil()
            * step
            + from.y;
        position += Vec3::new(downhill.x, 0.0, downhill.y) * f32::from_bits(0x3e4ccccd);
    }
    // widths 的必需读取在量化/梯形调整之后，不前移到无条件几何前段。
    let width0 = params.widths[i];
    let width1 = params.widths[i + 1];
    let width = ((width1 - width0) * t + width0) * 0.5;
    let offset = destination.coordinate.x * width;
    position += Vec3::new(normal.x, 0.0, normal.y) * offset;
    CachedDecoratorTransform {
        rotation: DecoratorRotationXZ::from_xz_forward_vector(normal)
            .offset(destination.yaw.0)
            .to_quat(),
        translation: position,
        scale: Vec3::ONE,
    }
}

/// 地形装饰位置（O：`DecoratorInfo+0x00` 前 8B = Vec2(x,z)）。
pub fn decorator_terrain_xz(
    info: &country_core::resources::walls::decorator::DecoratorInfo,
) -> (f32, f32) {
    (
        f32::from_bits(info.dst_wall as u32),
        f32::from_bits((info.dst_wall >> 32) as u32),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::clutter_meshes::ClutterMesh;
    use country_core::resources::stairs::stairs_assembly_params::{
        AdjustedStairEdge, PlayermadeStairsAssemblyParams, StairNodeAnchor, StairSubSegment,
    };
    use country_core::resources::stairs::stairs_state::StairSegment;
    use country_core::resources::walls::decorator::{
        DecoratorExtraInfo, RoofDecoratorDst, StairDecoratorDst, StairExtra, TerrainDecoratorDst,
    };
    use std::panic::{AssertUnwindSafe, catch_unwind};

    fn world() -> World {
        let mut world = World::new();
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<CachedDecoratorTransforms>();
        world.init_resource::<PublicWalls>();
        world.insert_resource(TerrainHeightsData::new());
        world.init_resource::<PlayermadeStairsAssemblyRequests>();
        world.init_resource::<StairsState>();
        // 本组只覆盖不读取 clutter 的56/63与楼梯；仍装入真实资源类型。
        world.insert_resource(ClutterMeshes::from_parts(
            std::array::from_fn(|_| ClutterMesh::omitted_in_demo("unused", 0, None, false, false)),
            Vec::new(),
        ));
        world
    }

    fn schedule() -> Schedule {
        let mut schedule = Schedule::default();
        schedule.add_systems(cache_decorator_transforms);
        schedule
    }

    fn terrain_info(ty: u8, height: f32) -> DecoratorInfo {
        let mut info = DecoratorInfo {
            ty,
            rank: 1,
            ..Default::default()
        };
        info.set_destination(DecoratorDst::Terrain(TerrainDecoratorDst {
            position_xz: Vec2::ZERO,
            yaw: DecoratorRotationXZ(0.0),
        }));
        let mut extra = [0; 20];
        extra[..4].copy_from_slice(&height.to_le_bytes());
        info.set_extra(DecoratorExtraInfo::Stair(StairExtra(extra)));
        info
    }

    fn stair_info(a: u32, b: u32, u: f32, lateral: f32) -> DecoratorInfo {
        let mut info = DecoratorInfo::default();
        info.set_destination(DecoratorDst::Stair(StairDecoratorDst {
            first_id: a,
            second_id: b,
            coordinate: Vec2::new(lateral, u),
            yaw: DecoratorRotationXZ(0.0),
        }));
        info
    }

    fn params(kind: u8) -> PlayermadeStairsAssemblyParams {
        PlayermadeStairsAssemblyParams {
            curve: utils::curve::Curve2::from_cached_parts(
                vec![Vec2::ZERO, Vec2::new(2.0, 0.0)],
                vec![0.0, 1.0],
                2.0,
            ),
            heights: vec![0.0, 1.0],
            widths: vec![2.0, 4.0],
            tangents: vec![Vec2::X, Vec2::X],
            auxiliary_samples: Vec::new(),
            interpolated_curve: Some(utils::curve::Curve3::from_cached_parts(
                vec![Vec3::ZERO, Vec3::new(2.0, 1.0, 0.0)],
                vec![0.0, 1.0],
                5.0f32.sqrt(),
            )),
            nodes: [(StairNodeAnchor::Terrain { height: 0.0 }, 1); 2],
            terrain_contacts: Default::default(),
            segment: StairSegment::new(DecoratorId(1), DecoratorId(2)),
            sub_segments: [StairSubSegment {
                kind,
                flag: 0,
                count: 1,
            }]
            .into_iter()
            .collect(),
            dirty: false,
        }
    }

    fn insert(world: &mut World, address: DecoratorStorageAddr, id: u32, info: DecoratorInfo) {
        world
            .resource_mut::<DecoratorStorage>()
            .get_or_insert_default(address)
            .decorators
            .insert(id, info);
    }

    fn panic_text(f: impl FnOnce()) -> String {
        let error = catch_unwind(AssertUnwindSafe(f)).expect_err("required read must fail");
        if let Some(text) = error.downcast_ref::<String>() {
            text.clone()
        } else if let Some(text) = error.downcast_ref::<&str>() {
            text.to_string()
        } else {
            String::new()
        }
    }

    fn install_roof(world: &mut World, flat: bool, info: DecoratorInfo) {
        let wall = WallId(123);
        let mut bytes = [0; 88];
        bytes[8..12].copy_from_slice(&1u32.to_le_bytes());
        bytes[0xc..0x10].copy_from_slice(&1.0f32.to_le_bytes());
        bytes[0x30..0x34].copy_from_slice(&(if flat { 0.0f32 } else { 1.0 }).to_le_bytes());
        bytes[0x4c..0x50].copy_from_slice(&2.0f32.to_le_bytes());
        let mut volume = RoofTriggerVolume::default();
        volume.set_triangles(vec![[Vec3::ZERO, Vec3::X, Vec3::Z]]);
        volume.set_uvs(vec![[Vec2::ZERO, Vec2::X, Vec2::Y]]);
        let entity = world.spawn((Roof::from_observed(bytes), volume)).id();
        world
            .resource_mut::<PublicWalls>()
            .associate_roof(wall, entity);
        insert(world, DecoratorStorageAddr::Wall(wall), 7, info);
    }

    fn roof_info(height: f32) -> DecoratorInfo {
        let mut info = terrain_info(63, height);
        info.set_destination(DecoratorDst::Roof(RoofDecoratorDst {
            coord: Vec2::splat(0.25),
            yaw: DecoratorRotationXZ(0.3),
        }));
        info
    }

    #[test]
    fn terrain_xz_reads_packed_dst_wall() {
        let info = DecoratorInfo {
            dst_wall: u64::from(1.5f32.to_bits()) | (u64::from((-2.25f32).to_bits()) << 32),
            ..DecoratorInfo::default()
        };
        assert_eq!(decorator_terrain_xz(&info), (1.5, -2.25));
    }

    #[test]
    fn actual_world_terrain63_reads_height_and_only_first_outgoing() {
        let mut world = world();
        insert(
            &mut world,
            DecoratorStorageAddr::Terrain,
            7,
            terrain_info(63, 2.0),
        );
        let base_height = world
            .resource::<TerrainHeightsData>()
            .sample_at_world_xz(0.0, 0.0);
        let mut run = schedule();
        run.run(&mut world);
        let first = world.resource::<CachedDecoratorTransforms>().transform(7);
        assert_eq!(first.translation, Vec3::new(0.0, base_height + 2.0, 0.0));
        assert_eq!(first.rotation, glam::Quat::IDENTITY);
        world
            .resource_mut::<PlayermadeStairsAssemblyRequests>()
            .adjusted_edges
            .add_edge(
                DecoratorId(9),
                DecoratorId(7),
                AdjustedStairEdge {
                    position: Vec2::ZERO,
                    outgoing_dir: Vec2::Y,
                },
            );
        run.run(&mut world);
        assert_eq!(
            world.resource::<CachedDecoratorTransforms>().transform(7),
            first
        );
        world
            .resource_mut::<PlayermadeStairsAssemblyRequests>()
            .adjusted_edges
            .add_edge(
                DecoratorId(7),
                DecoratorId(9),
                AdjustedStairEdge {
                    position: Vec2::ZERO,
                    outgoing_dir: Vec2::ZERO,
                },
            );
        run.run(&mut world);
        let actual = world.resource::<CachedDecoratorTransforms>().transform(7);
        assert_eq!(actual.rotation, utils::xz_rotation_from_forward(Vec2::ZERO));
        assert_eq!(actual.translation, first.translation);
    }

    #[test]
    fn actual_world_reuses_previous_bits_and_deletes_missing_item_next_frame() {
        let mut world = world();
        insert(
            &mut world,
            DecoratorStorageAddr::Terrain,
            7,
            terrain_info(56, 0.0),
        );
        let height = world
            .resource::<TerrainHeightsData>()
            .sample_at_world_xz(0.0, 0.0);
        let old = CachedDecoratorTransform {
            rotation: glam::Quat::from_xyzw(-0.0, 0.0, -0.0, 1.0),
            translation: Vec3::new(-0.0, height + 0.0005, -0.0),
            scale: Vec3::ONE,
        };
        world
            .resource_mut::<CachedDecoratorTransforms>()
            .insert(7, old);
        let mut run = schedule();
        run.run(&mut world);
        assert_eq!(
            world
                .resource::<CachedDecoratorTransforms>()
                .transform(7)
                .to_components()
                .map(f32::to_bits),
            old.to_components().map(f32::to_bits)
        );
        assert!(
            world
                .resource::<DecoratorStorage>()
                .terrain
                .as_ref()
                .unwrap()
                .modified_ids
                .is_empty()
        );
        world
            .resource_mut::<DecoratorStorage>()
            .terrain
            .as_mut()
            .unwrap()
            .decorators
            .remove(&7);
        run.run(&mut world);
        assert!(
            world
                .resource::<CachedDecoratorTransforms>()
                .try_transform(7)
                .is_none()
        );
    }

    #[test]
    fn actual_world_marks_prior_success_before_later_insert_panics() {
        let mut world = world();
        insert(
            &mut world,
            DecoratorStorageAddr::Terrain,
            1,
            terrain_info(63, 1.0),
        );
        insert(
            &mut world,
            DecoratorStorageAddr::Terrain,
            2,
            terrain_info(63, 1.0),
        );
        let order: Vec<_> = world
            .resource::<DecoratorStorage>()
            .terrain
            .as_ref()
            .unwrap()
            .decorators
            .keys()
            .copied()
            .collect();
        world
            .resource_mut::<DecoratorStorage>()
            .terrain
            .as_mut()
            .unwrap()
            .decorators
            .insert(order[1], terrain_info(63, f32::NAN));
        let message = panic_text(|| schedule().run(&mut world));
        assert!(message.contains("translation.is_finite"), "{message}");
        let bucket = world
            .resource::<DecoratorStorage>()
            .terrain
            .as_ref()
            .unwrap();
        assert!(bucket.modified_ids.contains(&order[0]));
        assert!(!bucket.modified_ids.contains(&order[1]));
        assert!(
            world
                .resource::<CachedDecoratorTransforms>()
                .try_transform(order[0])
                .is_some()
        );
        assert!(
            world
                .resource::<CachedDecoratorTransforms>()
                .try_transform(order[1])
                .is_none()
        );
    }

    #[test]
    fn actual_world_top_immutable_gate_runs_even_without_terrain_or_stair_bucket() {
        let mut world = world();
        world.resource_mut::<DecoratorStorage>().immutable = true;
        assert!(panic_text(|| schedule().run(&mut world)).contains("!self.immutable"));
    }

    #[test]
    fn actual_world_roof63_is_lazy_about_wall_and_extra_and_preserves_direct_insert() {
        for flat in [false, true] {
            let mut world = world();
            let mut info = roof_info(0.75);
            if !flat {
                info.u40 = 6 << 8;
            }
            install_roof(&mut world, flat, info);
            let mut run = schedule();
            run.run(&mut world);
            let transform = world.resource::<CachedDecoratorTransforms>().transform(7);
            assert_eq!(
                transform.translation,
                Vec3::new(0.25, if flat { 2.75 } else { 2.0 }, 0.25)
            );
            assert_eq!(transform.rotation, DecoratorRotationXZ(0.3).to_quat());
            assert!(
                world.resource::<DecoratorStorage>().per_wall[&WallId(123)]
                    .modified_ids
                    .is_empty()
            );
            world
                .resource_mut::<PlayermadeStairsAssemblyRequests>()
                .adjusted_edges
                .add_edge(
                    DecoratorId(7),
                    DecoratorId(8),
                    AdjustedStairEdge {
                        position: Vec2::ZERO,
                        outgoing_dir: Vec2::Y,
                    },
                );
            run.run(&mut world);
            assert_eq!(
                world
                    .resource::<CachedDecoratorTransforms>()
                    .transform(7)
                    .rotation,
                utils::xz_rotation_bad(Vec2::Y)
            );
        }
    }

    #[test]
    fn actual_world_flat_roof63_requires_extra_after_uv_but_nonflat_does_not() {
        let mut world = world();
        let mut info = roof_info(0.0);
        info.u40 = 6 << 8;
        install_roof(&mut world, true, info);
        assert!(panic_text(|| schedule().run(&mut world)).contains("not a stair"));
    }

    #[test]
    fn actual_world_roof_missing_association_reuses_previous_without_public_wall() {
        let mut world = world();
        let wall = WallId(44);
        insert(
            &mut world,
            DecoratorStorageAddr::Wall(wall),
            7,
            roof_info(0.0),
        );
        let mut run = schedule();
        run.run(&mut world);
        assert_eq!(
            world
                .resource::<CachedDecoratorTransforms>()
                .transform(7)
                .translation,
            Vec3::splat(-100.0)
        );
        let old = CachedDecoratorTransform {
            rotation: glam::Quat::IDENTITY,
            translation: Vec3::new(3.0, 4.0, 5.0),
            scale: Vec3::ONE,
        };
        world
            .resource_mut::<CachedDecoratorTransforms>()
            .insert(7, old);
        run.run(&mut world);
        assert_eq!(
            world.resource::<CachedDecoratorTransforms>().transform(7),
            old
        );
    }

    #[test]
    fn actual_world_unmatched_stair_keeps_order_and_negative_zero_before_invalid_u() {
        let mut world = world();
        world
            .resource_mut::<PlayermadeStairsAssemblyRequests>()
            .requests
            .push(params(4));
        insert(
            &mut world,
            DecoratorStorageAddr::Stair,
            7,
            stair_info(2, 1, f32::NAN, f32::NAN),
        );
        schedule().run(&mut world);
        let transform = world.resource::<CachedDecoratorTransforms>().transform(7);
        assert_eq!(
            transform.translation.to_array().map(f32::to_bits),
            [0x80000000, (-100.0f32).to_bits(), 0x80000000]
        );
        assert_eq!(transform.scale, Vec3::ZERO);
        assert!(
            world
                .resource::<DecoratorStorage>()
                .stairs
                .as_ref()
                .unwrap()
                .modified_ids
                .contains(&7)
        );
    }

    #[test]
    fn actual_world_all_stair_modes_use_real_curve_and_lateral_width() {
        for (kind, x, y) in [
            (0, 0.4, 0.2),
            (2, 0.4, 0.25),
            (3, 0.2, 1.0 / 3.0),
            (4, 0.4, 0.2),
            (255, 0.4, 0.2),
        ] {
            let mut world = world();
            world
                .resource_mut::<PlayermadeStairsAssemblyRequests>()
                .requests
                .push(params(kind));
            insert(
                &mut world,
                DecoratorStorageAddr::Stair,
                7,
                stair_info(1, 2, 0.2, 0.5),
            );
            schedule().run(&mut world);
            let transform = world.resource::<CachedDecoratorTransforms>().transform(7);
            assert!(
                (transform.translation - Vec3::new(x, y, 0.6)).length() < 1e-6,
                "kind={kind}: {transform:?}"
            );
            assert_eq!(transform.scale, Vec3::ONE);
        }
    }

    #[test]
    fn actual_world_stair_uses_first_exact_request_and_marks_only_changed_items() {
        let mut world = world();
        let first = params(4);
        let mut second = first.clone();
        second.interpolated_curve.as_mut().unwrap().points[1].y = 9.0;
        world
            .resource_mut::<PlayermadeStairsAssemblyRequests>()
            .requests
            .extend([first, second]);
        insert(
            &mut world,
            DecoratorStorageAddr::Stair,
            7,
            stair_info(1, 2, 0.2, 0.0),
        );
        let mut run = schedule();
        run.run(&mut world);
        assert!((world.resource::<CachedDecoratorTransforms>().position(7).y - 0.2).abs() < 1e-6);
        world
            .resource_mut::<DecoratorStorage>()
            .stairs
            .as_mut()
            .unwrap()
            .modified_ids
            .clear();
        run.run(&mut world);
        assert!(
            world
                .resource::<DecoratorStorage>()
                .stairs
                .as_ref()
                .unwrap()
                .modified_ids
                .is_empty()
        );
    }

    #[test]
    fn exhausted_subsegment_keeps_cumulative_start_instead_of_rewinding_last_segment() {
        let stairs = StairsState::default();
        let mut requests = PlayermadeStairsAssemblyRequests::default();
        let mut p = params(2);
        p.curve = utils::curve::Curve2::from_cached_parts(
            vec![Vec2::ZERO, Vec2::X, Vec2::new(2.0, 0.0)],
            vec![0.0, 0.5, 1.0],
            2.0,
        );
        p.interpolated_curve = Some(utils::curve::Curve3::from_cached_parts(
            vec![
                Vec3::ZERO,
                Vec3::new(1.0, 1.0, 0.0),
                Vec3::new(2.0, 10.0, 0.0),
            ],
            vec![0.0, 0.5, 1.0],
            10.0,
        ));
        p.tangents = vec![Vec2::X; 3];
        p.widths = vec![2.0; 3];
        requests.requests.push(p);
        let result = compute_stair_transform(&stair_info(1, 2, 0.625, 0.0), &requests, &stairs);
        // i=1已超过唯一count1；原start保总计1，量化片段为点1..=2。
        assert!((result.translation.y - 3.7).abs() < 1e-5, "{result:?}");
    }

    #[test]
    fn stair_required_read_order_is_u_then_subsegment_then_tangent_then_curve_then_point_then_width()
     {
        let stairs = StairsState::default();
        let mut requests = PlayermadeStairsAssemblyRequests::default();
        let mut p = params(4);
        p.sub_segments.clear();
        p.tangents.clear();
        p.interpolated_curve = None;
        p.widths.clear();
        requests.requests.push(p);
        assert!(
            panic_text(|| {
                compute_stair_transform(&stair_info(1, 2, f32::NAN, 0.0), &requests, &stairs);
            })
            .contains("CurveU")
        );
        assert!(
            panic_text(|| {
                compute_stair_transform(&stair_info(1, 2, 0.2, 0.0), &requests, &stairs);
            })
            .contains("index out of bounds")
        );
        requests.requests[0].sub_segments.push(StairSubSegment {
            kind: 4,
            flag: 0,
            count: 1,
        });
        assert!(
            panic_text(|| {
                compute_stair_transform(&stair_info(1, 2, 0.2, 0.0), &requests, &stairs);
            })
            .contains("index out of bounds")
        );
        requests.requests[0].tangents = vec![Vec2::X; 2];
        assert!(
            panic_text(|| {
                compute_stair_transform(&stair_info(1, 2, 0.2, 0.0), &requests, &stairs);
            })
            .contains("Option::unwrap")
        );
        requests.requests[0].interpolated_curve = Some(utils::curve::Curve3::from_cached_parts(
            Vec::new(),
            Vec::new(),
            0.0,
        ));
        assert!(
            panic_text(|| {
                compute_stair_transform(&stair_info(1, 2, 0.2, 0.0), &requests, &stairs);
            })
            .contains("index out of bounds")
        );
        requests.requests[0].interpolated_curve = params(4).interpolated_curve;
        assert!(
            panic_text(|| {
                compute_stair_transform(&stair_info(1, 2, 0.2, 0.0), &requests, &stairs);
            })
            .contains("index out of bounds")
        );
    }

    #[test]
    fn ladder_validation_precedes_width_read_and_descending_uses_signed_step() {
        let stairs = StairsState::default();
        let mut requests = PlayermadeStairsAssemblyRequests::default();
        let mut p = params(3);
        p.sub_segments[0].count = 0;
        p.widths.clear();
        requests.requests.push(p);
        assert!(
            panic_text(|| {
                compute_stair_transform(&stair_info(1, 2, 0.2, 0.0), &requests, &stairs);
            })
            .contains("ladder subsegment")
        );
        let mut p = params(3);
        p.interpolated_curve.as_mut().unwrap().points = vec![Vec3::Y, Vec3::new(2.0, 0.0, 0.0)];
        requests.requests[0] = p;
        let result = compute_stair_transform(&stair_info(1, 2, 0.2, 0.0), &requests, &stairs);
        assert!(
            result.translation.y > 1000.0,
            "original signed negative step: {result:?}"
        );
    }

    #[test]
    fn quantization_numeric_minmax_keep_left_zero_and_ignore_one_nan() {
        assert_eq!(numeric_min_left(-0.0, 0.0).to_bits(), (-0.0f32).to_bits());
        assert_eq!(numeric_max_left(-0.0, 0.0).to_bits(), (-0.0f32).to_bits());
        assert_eq!(numeric_min_left(f32::NAN, 3.0), 3.0);
        assert_eq!(numeric_max_left(3.0, f32::NAN), 3.0);
        assert_eq!(max_source(f32::NAN, 1e-8), 1e-8);
    }
}
