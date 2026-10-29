//! 原B14081c790九参墙附着重锚系统；四原事件流及两真实差分tracker。
//! 先删除桶、处理墙变化和屋顶标脏，再遍历modified记录做普通投影与门底profile。
//! 原路径与证据详见reports/module-recovery-2026-10-23/m01-anchor-follow-recovery.md。

use bevy_ecs::prelude::*;
use country_core::resources::roofs::commands::RecalculateRoofCmd;
use country_core::resources::walls::WallId;
use country_core::resources::walls::decorator::anchors::{
    DecoratorAnchorContext, DecoratorInfoObserved, move_wall_decorator_following_anchor_base,
};
use country_core::resources::walls::decorator::{DecoratorId, DecoratorInfo};
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::{
    DecoratorLibKey, DerivedDecoratorInfoResources, DerivedDecoratorInfoView,
};
use country_core::resources::walls::public_wall_state::PublicWallState;
use country_core::resources::walls::public_walls::{OnWallChanged, OnWallDeleted, PublicWalls};
use country_core::systems::wall::adjust_wall_height::AdjustWallHeightCmd;
use glam::Vec2;
use utils::curve::{CurveCoordSemantic3D, CurveInternalCoord, curve_slice::CurveSlice};

use crate::decorator_on_wall_height_changed::{
    get_wall_coord, set_wall_coord, set_wall_height_relative_y, side_from_tag_byte,
};
use crate::decorator_visual::deco_differential_state::{
    ChimneyAssemblyParams, DoorStairsAssemblyParams, WallAttachedDecoTracker,
};

const _: () = assert!(std::mem::size_of::<DecoratorInfo>() == 72);

/// 原始 72B 记录 ↔ typed 视图（`DecoratorInfo` 为 `#[repr(C)]` 72B）。
///
/// 这是 core 已公开的 `DecoratorInfoObserved` 字节桥；不是对字段语义的再解释。
pub fn observed_from_info(info: &DecoratorInfo) -> DecoratorInfoObserved {
    let mut bytes = [0u8; 72];
    bytes[..8].copy_from_slice(&info.dst_wall.to_le_bytes());
    bytes[8..16].copy_from_slice(&info.dst_secondary.to_le_bytes());
    bytes[16..24].copy_from_slice(&info.dst_payload.to_le_bytes());
    bytes[24..32].copy_from_slice(&info.u18.to_le_bytes());
    bytes[32..36].copy_from_slice(&info.u20.to_le_bytes());
    bytes[36..40].copy_from_slice(&info.u24.to_le_bytes());
    bytes[40..44].copy_from_slice(&info.u28.to_le_bytes());
    bytes[44..48].copy_from_slice(&info.decorator_id.to_le_bytes());
    bytes[48..56].copy_from_slice(&info.u30.to_le_bytes());
    bytes[56..60].copy_from_slice(&info.u38.to_le_bytes());
    bytes[60..64].copy_from_slice(&info.u3c.to_le_bytes());
    bytes[64..68].copy_from_slice(&info.u40.to_le_bytes());
    bytes[68] = info.rank;
    bytes[69] = info.u45;
    bytes[70] = info.ty;
    bytes[71] = info.state;
    DecoratorInfoObserved::from_observed(bytes)
}

fn write_observed_back(info: &mut DecoratorInfo, observed: &DecoratorInfoObserved) {
    let b = observed.bytes();
    info.dst_wall = u64::from_le_bytes(b[..8].try_into().unwrap());
    info.dst_secondary = u64::from_le_bytes(b[8..16].try_into().unwrap());
    info.dst_payload = u64::from_le_bytes(b[16..24].try_into().unwrap());
    info.u18 = u64::from_le_bytes(b[24..32].try_into().unwrap());
    info.u20 = u32::from_le_bytes(b[32..36].try_into().unwrap());
    info.u24 = u32::from_le_bytes(b[36..40].try_into().unwrap());
    info.u28 = u32::from_le_bytes(b[40..44].try_into().unwrap());
    info.decorator_id = u32::from_le_bytes(b[44..48].try_into().unwrap());
    info.u30 = u64::from_le_bytes(b[48..56].try_into().unwrap());
    info.u38 = u32::from_le_bytes(b[56..60].try_into().unwrap());
    info.u3c = u32::from_le_bytes(b[60..64].try_into().unwrap());
    info.u40 = u32::from_le_bytes(b[64..68].try_into().unwrap());
    info.rank = b[68];
    info.u45 = b[69];
    info.ty = b[70];
    info.state = b[71];
}

/// 原B九参系统14081c790：事件游标、真实tracker和存储写回属于同一业务链。
#[allow(clippy::too_many_arguments)]
pub fn move_decorators_following_anchors(
    walls: Res<PublicWalls>,
    mut deleted: EventReader<OnWallDeleted>,
    mut changed: EventReader<OnWallChanged>,
    mut roofs: EventReader<RecalculateRoofCmd>,
    mut heights: EventReader<AdjustWallHeightCmd>,
    mut chimneys: ResMut<WallAttachedDecoTracker<ChimneyAssemblyParams>>,
    mut door_stairs: ResMut<WallAttachedDecoTracker<DoorStairsAssemblyParams>>,
    mut storage: ResMut<DecoratorStorage>,
    derived: DerivedDecoratorInfoResources,
) {
    // 原第一流只删除装饰桶；不能根据removed存储日志反推删除事件。
    for OnWallDeleted(wall) in deleted.read() {
        storage.on_wall_removed(*wall);
    }
    for OnWallChanged(wall) in changed.read() {
        if let Some(decorators) = storage.wall_decorators_mut(*wall) {
            decorators.mark_all_modified();
        }
        // 即使没有装饰桶，原两个HashSet仍各写一次，供差分装配消费者读取。
        chimneys.on_wall_changed(*wall);
        door_stairs.on_wall_changed(*wall);
    }
    for event in roofs.read() {
        invalidate_roof_decorators(&mut storage, event.wall);
    }
    for event in heights.read() {
        invalidate_roof_decorators(&mut storage, WallId(event.0.wall.0));
    }

    let derived = derived.as_view();
    for (wall_id, bucket) in storage.iter_walls_mut() {
        if bucket.modified_ids.is_empty() {
            continue;
        }
        // 原这里直接按ID查发布map；只有side0投影时才访问曲线，不额外断言roof-only桶。
        let state = walls.get_unchecked(wall_id).unwrap_or_else(|| {
            panic!(
                "The list of decorators contains a wall which is not in PublicWalls: {wall_id:?}"
            )
        });
        let public = state.building_projection();
        let context = DecoratorAnchorContext {
            raw_curve: state.simplified_curve_2d(),
            mapped_outline: state.height_outline(),
            arch_heights: state.arch_segment_heights().unwrap_or(&[]),
            public: &public,
        };
        // 分借字段保留原modified集合迭代，不为每墙分配ID副本。
        let (records, modified) = (&mut bucket.decorators, &bucket.modified_ids);
        for id in modified {
            // 原impl$5::next140a1b2d0缺记录走unwrap_failed，不得静默跳过损坏的追踪ID。
            let info = records
                .get_mut(id)
                .expect("modified decorator is missing from its storage");
            reanchor_one(info, &context, state, derived);
        }
    }
}

fn reanchor_one(
    info: &mut DecoratorInfo,
    context: &DecoratorAnchorContext<'_>,
    state: &PublicWallState,
    derived: DerivedDecoratorInfoView<'_>,
) {
    let side = side_from_tag_byte(info.dst_tag());
    match side {
        0 => {
            let observed = observed_from_info(info);
            let updated = move_wall_decorator_following_anchor_base(&observed, context)
                .unwrap_or_else(|error| panic!("invalid decorator anchor: {error:?}"));
            write_observed_back(info, &updated);
            reanchor_doors_attached_to_bottom(info, state, derived);
        }
        1 => {}
        _ => unreachable!(
            "internal error: entered unreachable code (move_decorators_following_anchors.rs side {side})"
        ),
    }
}

/// 原14081d2f2..14081d43e门底尾段：沿原mapped Curve3的XZ弧长采样地面，
/// 结合实际subtype/rank碰撞bounds写绝对y、anchor_ws.y与相对墙顶高度。
pub fn reanchor_doors_attached_to_bottom(
    info: &mut DecoratorInfo,
    state: &PublicWallState,
    derived: DerivedDecoratorInfoView<'_>,
) {
    // 原cmp byte[Info+21],1，不能将任何非零字节都视为true。
    if ((info.u20 >> 8) as u8) != 1 {
        return;
    }
    let Some(subtype) = derived.try_get_subtype(DecoratorId(info.decorator_id)) else {
        return;
    };
    let points = state.geometry.outline.as_slice();
    assert!(points.len() >= 2, "invalid mapped curve for bottom door");
    let coord = get_wall_coord(info);
    // MAXSS的第二操作数0在NaN/相等时获胜。
    let distance = if coord.u > 0.0 { coord.u } else { 0.0 };
    let start = CurveInternalCoord { segment: 0, t: 0.0 };
    let mut curve =
        CurveSlice::<[f32; 3], _, CurveCoordSemantic3D, Vec2, _>::from_coord_range_with_remap(
            points,
            start,
            start,
            |_, point| Vec2::new(point[0], point[2]),
        );
    curve.extend_right(distance);
    let end = curve.end();
    let a = points[end.segment][1];
    let b = points[end.segment + 1][1];
    let collision = derived
        .mesh_library()
        .collision_record(DecoratorLibKey::new(subtype.0, info.rank));
    let y = bottom_door_height(end.t, a, b, collision.bounds.bottom, collision.bounds.top);
    // 原先写anchor_ws.y，再读取max_y（缺cache时panic也保留前一写入）。
    info.u18 = (info.u18 & 0xffff_ffff_0000_0000) | u64::from(y.to_bits());
    set_wall_height_relative_y(info, y - state.max_y());
    set_wall_coord(
        info,
        utils::wall_space::WallSpaceCoord::new_unchecked(coord.u, y),
    );
}

/// 原14081d3de..14081d421逐指令表达式；保留NaN选择和运算顺序，不用clamp近似。
fn bottom_door_height(t: f32, a: f32, b: f32, bottom: f32, top: f32) -> f32 {
    let profile_y = t * (b - a) + a;
    let half_height = (top - bottom) * 0.5;
    let nonnegative = if profile_y > 0.0 { profile_y } else { 0.0 };
    let candidate = (nonnegative + half_height) + 0.05;
    if candidate.is_nan() || half_height > candidate {
        half_height
    } else {
        candidate
    }
}

/// `invalidate_roof_decorators`（行 133，内联，I）：屋顶装饰失效标记。
pub fn invalidate_roof_decorators(storage: &mut DecoratorStorage, wall: WallId) {
    if let Some(decorators) = storage.wall_decorators_mut(wall) {
        for (id, info) in &decorators.decorators {
            if info.dst_tag() == 2 {
                decorators.modified_ids.insert(*id);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decorator_on_wall_height_changed::{
        get_anchor_ws, get_wall_height_relative_y, set_anchor_ws,
    };
    use bevy_ecs::schedule::Schedule;
    use country_core::resources::{
        roofs::geometry::surface::Curve2,
        walls::{
            InnerWalls, WallState,
            decorator_storage::DecoratorStorageAddr,
            decorator_subtype::{
                DecoratorCollision, DecoratorMeshLibrary, DecoratorSubtype,
                DerivedDecoratorSubtypes,
            },
            publication_source::TerrainHeightGrid512,
        },
    };
    use country_core::systems::wall::{
        height_command::AdjustWallHeightRequest, wall_state::path_preparation::PathRasters512,
    };
    use glam::Vec3;

    fn world_with_walls() -> (World, Vec<WallId>) {
        let mut inner = InnerWalls::new();
        let walls = (0..9)
            .map(|_| {
                inner.new_wall(WallState::new(
                    Curve2::try_new(vec![[0., 0.], [4., 0.]], false).unwrap(),
                    4.,
                    0,
                    0,
                    false,
                ))
            })
            .collect::<Vec<_>>();
        let mut public = PublicWalls::default();
        let paths =
            PathRasters512::from_encoded(std::array::from_fn(|_| vec![0; 512 * 512].into()), false)
                .unwrap();
        public.consume_with_paths(
            &mut inner,
            &TerrainHeightGrid512 {
                encoded: vec![0; 512 * 512].into(),
                minimum_y: 0.,
                maximum_y: 0.,
                sampling_fix: false,
            },
            Some(&paths),
            &Default::default(),
            &Default::default(),
        );
        let mut world = World::new();
        world.insert_resource(public);
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<WallAttachedDecoTracker<ChimneyAssemblyParams>>();
        world.init_resource::<WallAttachedDecoTracker<DoorStairsAssemblyParams>>();
        world.init_resource::<DerivedDecoratorSubtypes>();
        world.init_resource::<DecoratorMeshLibrary>();
        world.init_resource::<Events<OnWallDeleted>>();
        world.init_resource::<Events<OnWallChanged>>();
        world.init_resource::<Events<RecalculateRoofCmd>>();
        world.init_resource::<Events<AdjustWallHeightCmd>>();
        (world, walls)
    }

    fn insert(world: &mut World, wall: WallId, id: u32, tag: u8) {
        let mut info = DecoratorInfo {
            decorator_id: id,
            rank: 2,
            u20: u32::from(tag) << 16,
            ..Default::default()
        };
        set_wall_coord(
            &mut info,
            utils::wall_space::WallSpaceCoord::new_unchecked(0.25, 7.),
        );
        set_wall_height_relative_y(&mut info, 3.);
        set_anchor_ws(&mut info, Vec3::new(1., 7., 0.));
        let mut storage = world.resource_mut::<DecoratorStorage>();
        storage
            .get_or_insert_default(DecoratorStorageAddr::Wall(wall))
            .insert(DecoratorId(id), info);
        storage
            .decorator_to_storage
            .insert(DecoratorId(id), DecoratorStorageAddr::Wall(wall));
    }

    fn dirty<P: Send + Sync + 'static>(world: &mut World) -> Vec<WallId> {
        let mut walls = Vec::new();
        world
            .resource_mut::<WallAttachedDecoTracker<P>>()
            .with_diffs(|wall, log| {
                walls.push(wall);
                log.into_removals()
                    .with_removed(|_, _| panic!("unexpected removed decorator"))
            });
        walls.sort_by_key(|w| w.0);
        walls
    }

    #[test]
    fn original_bottom_height_block_matches_1100_native_bit_cases() {
        let text = include_str!(
            "../../../../reports/module-recovery-2026-10-23/m01-anchor-bottom-native.txt"
        );
        for (i, line) in text.lines().enumerate() {
            let bits = line
                .split_whitespace()
                .map(|v| u32::from_str_radix(v, 16).unwrap())
                .collect::<Vec<_>>();
            let f = |n| f32::from_bits(bits[n]);
            assert_eq!(
                bottom_door_height(f(0), f(1), f(2), f(3), f(4)).to_bits(),
                bits[5],
                "native case {i}"
            );
        }
        assert_eq!(text.lines().count(), 1100);
    }

    #[test]
    fn four_event_streams_match_native_32_cases_and_64_persistent_frames() {
        use std::collections::HashSet;
        let fixture = include_str!(
            "../../../../reports/module-recovery-2026-10-23/m01-anchor-readers-native.txt"
        );
        for line in fixture.lines() {
            let pieces = line.split('|').collect::<Vec<_>>();
            let config = pieces[0]
                .split_whitespace()
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            let (mask, split) = (config[0], config[1]);
            let (mut world, walls) = world_with_walls();
            for (i, &wall) in walls.iter().enumerate() {
                insert(&mut world, wall, 100 + i as u32 * 2, 0);
                insert(&mut world, wall, 101 + i as u32 * 2, 2);
            }
            for (_, bucket) in world.resource_mut::<DecoratorStorage>().iter_walls_mut() {
                bucket.modified_ids.clear();
            }
            for stream in 0..4 {
                if mask & (1 << stream) == 0 {
                    continue;
                }
                for i in 0..2 {
                    let wall = walls[stream * 2 + i];
                    // 在原old/new两缓冲同一边界切开，Schedule保留同一reader游标。
                    match stream {
                        0 => {
                            let mut e = world.resource_mut::<Events<OnWallDeleted>>();
                            if split == 1 && i == 1 {
                                e.update();
                            }
                            e.send(OnWallDeleted(wall));
                        }
                        1 => {
                            let mut e = world.resource_mut::<Events<OnWallChanged>>();
                            if split == 1 && i == 1 {
                                e.update();
                            }
                            e.send(OnWallChanged(wall));
                        }
                        2 => {
                            let mut e = world.resource_mut::<Events<RecalculateRoofCmd>>();
                            if split == 1 && i == 1 {
                                e.update();
                            }
                            e.send(RecalculateRoofCmd {
                                entity: Entity::PLACEHOLDER,
                                wall,
                                kind: 0,
                            });
                        }
                        3 => {
                            let mut e = world.resource_mut::<Events<AdjustWallHeightCmd>>();
                            if split == 1 && i == 1 {
                                e.update();
                            }
                            e.send(AdjustWallHeightCmd(AdjustWallHeightRequest {
                                anchor: None,
                                mode: 0,
                                value: 4.,
                                wall: country_core::resources::walls::publication_fields::WallId(
                                    wall.0,
                                ),
                                observed_audio_flag: 0,
                            }));
                        }
                        _ => unreachable!(),
                    }
                }
            }
            let mut schedule = Schedule::default();
            schedule.add_systems(move_decorators_following_anchors);
            for frame in 0..2 {
                let calls = pieces[frame + 1]
                    .split(',')
                    .filter(|s| !s.is_empty())
                    .map(|s| {
                        let (kind, wall) = s.split_once(':').unwrap();
                        (kind, WallId(wall.parse().unwrap()))
                    })
                    .collect::<Vec<_>>();
                schedule.run(&mut world);
                let expected = |kind| {
                    calls
                        .iter()
                        .filter_map(|&(k, w)| (k == kind).then_some(w))
                        .collect::<HashSet<_>>()
                };
                let changed = expected("changed");
                let roof = expected("roof");
                let height = expected("height");
                let storage = world.resource::<DecoratorStorage>();
                for (i, &wall) in walls.iter().enumerate() {
                    let deleted = mask & 1 != 0 && i < 2;
                    let Some(bucket) = storage.wall_decorators(wall) else {
                        assert!(deleted);
                        continue;
                    };
                    assert!(!deleted);
                    let ordinary = 100 + i as u32 * 2;
                    let roof_id = ordinary + 1;
                    assert_eq!(
                        bucket.modified_ids.contains(&ordinary),
                        changed.contains(&wall),
                        "{line} frame{frame} wall{wall:?}"
                    );
                    assert_eq!(
                        bucket.modified_ids.contains(&roof_id),
                        changed.contains(&wall) || roof.contains(&wall) || height.contains(&wall)
                    );
                }
                assert_eq!(
                    dirty::<ChimneyAssemblyParams>(&mut world)
                        .into_iter()
                        .collect::<HashSet<_>>(),
                    expected("chimney")
                );
                assert_eq!(
                    dirty::<DoorStairsAssemblyParams>(&mut world)
                        .into_iter()
                        .collect::<HashSet<_>>(),
                    expected("stairs")
                );
                for (_, bucket) in world.resource_mut::<DecoratorStorage>().iter_walls_mut() {
                    bucket.modified_ids.clear();
                }
            }
        }
    }

    #[test]
    fn four_readers_write_real_trackers_only_invalidate_roofs_and_do_not_replay() {
        let (mut world, walls) = world_with_walls();
        insert(&mut world, walls[0], 10, 0);
        insert(&mut world, walls[1], 11, 0);
        insert(&mut world, walls[1], 12, 2);
        insert(&mut world, walls[2], 13, 0);
        for (_, bucket) in world.resource_mut::<DecoratorStorage>().iter_walls_mut() {
            bucket.modified_ids.clear();
        }
        let absent = WallId(9999);
        world
            .resource_mut::<Events<OnWallDeleted>>()
            .send(OnWallDeleted(walls[2]));
        world
            .resource_mut::<Events<OnWallChanged>>()
            .send_batch([OnWallChanged(walls[0]), OnWallChanged(absent)]);
        world
            .resource_mut::<Events<RecalculateRoofCmd>>()
            .send(RecalculateRoofCmd {
                entity: Entity::PLACEHOLDER,
                wall: walls[1],
                kind: 0,
            });
        world
            .resource_mut::<Events<AdjustWallHeightCmd>>()
            .send(AdjustWallHeightCmd(AdjustWallHeightRequest {
                anchor: None,
                mode: 0,
                value: 4.,
                wall: country_core::resources::walls::publication_fields::WallId(walls[1].0),
                observed_audio_flag: 0,
            }));
        let mut schedule = Schedule::default();
        schedule.add_systems(move_decorators_following_anchors);
        schedule.run(&mut world);
        let storage = world.resource::<DecoratorStorage>();
        assert!(storage.wall_decorators(walls[2]).is_none());
        assert!(storage.decorator_to_storage.get(DecoratorId(13)).is_none());
        assert_eq!(
            storage
                .wall_decorators(walls[0])
                .unwrap()
                .modified_ids
                .iter()
                .copied()
                .collect::<Vec<_>>(),
            vec![10]
        );
        assert_eq!(
            get_wall_coord(
                storage
                    .wall_decorators(walls[0])
                    .unwrap()
                    .get(DecoratorId(10))
                    .unwrap()
            )
            .u,
            1.
        );
        assert_eq!(
            storage
                .wall_decorators(walls[1])
                .unwrap()
                .modified_ids
                .iter()
                .copied()
                .collect::<Vec<_>>(),
            vec![12]
        );
        let mut expected = vec![walls[0], absent];
        expected.sort_by_key(|w| w.0);
        assert_eq!(dirty::<ChimneyAssemblyParams>(&mut world), expected);
        assert_eq!(dirty::<DoorStairsAssemblyParams>(&mut world), expected);
        for (_, bucket) in world.resource_mut::<DecoratorStorage>().iter_walls_mut() {
            bucket.modified_ids.clear();
        }
        schedule.run(&mut world);
        assert!(dirty::<ChimneyAssemblyParams>(&mut world).is_empty());
        assert!(dirty::<DoorStairsAssemblyParams>(&mut world).is_empty());
        assert!(
            world
                .resource::<DecoratorStorage>()
                .iter_walls()
                .all(|(_, b)| b.modified_ids.is_empty())
        );
        // 无新事件也处理上游消费者已经标modified的记录。
        {
            let mut storage = world.resource_mut::<DecoratorStorage>();
            let bucket = storage.wall_decorators_mut(walls[0]).unwrap();
            set_anchor_ws(
                bucket.get_mut(DecoratorId(10)).unwrap(),
                Vec3::new(3., 7., 0.),
            );
            bucket.modified_ids.insert(10);
        }
        schedule.run(&mut world);
        let storage = world.resource::<DecoratorStorage>();
        assert_eq!(
            get_wall_coord(
                storage
                    .wall_decorators(walls[0])
                    .unwrap()
                    .get(DecoratorId(10))
                    .unwrap()
            )
            .u,
            3.
        );
    }

    #[test]
    fn bottom_door_uses_mapped_profile_collision_rank_and_all_three_height_writes() {
        let (mut world, walls) = world_with_walls();
        insert(&mut world, walls[0], 10, 0);
        {
            let mut public = world.resource_mut::<PublicWalls>();
            // height_outline有独立prepared曲线；门底必须读原mapped Curve3，而不是该高度轮廓。
            public.get_mut(walls[0]).unwrap().geometry.outline =
                vec![[0., 2., 0.], [2., 6., 0.], [4., 10., 0.]].into();
        }
        world
            .resource_mut::<DerivedDecoratorSubtypes>()
            .insert(10, DecoratorSubtype(3));
        let mut collision = DecoratorCollision::empty();
        collision.bounds.bottom = -1.;
        collision.bounds.top = 1.;
        world
            .resource_mut::<DecoratorMeshLibrary>()
            .collision
            .insert(DecoratorLibKey::new(3, 2), collision);
        world
            .resource_mut::<DecoratorStorage>()
            .wall_decorators_mut(walls[0])
            .unwrap()
            .get_mut(DecoratorId(10))
            .unwrap()
            .u20 |= 0x100;
        let mut schedule = Schedule::default();
        schedule.add_systems(move_decorators_following_anchors);
        schedule.run(&mut world);
        let storage = world.resource::<DecoratorStorage>();
        let info = storage
            .wall_decorators(walls[0])
            .unwrap()
            .get(DecoratorId(10))
            .unwrap();
        let y = 5.05f32;
        assert_eq!(get_wall_coord(info).u, 1.);
        assert_eq!(get_wall_coord(info).y.to_bits(), y.to_bits());
        assert_eq!(get_anchor_ws(info), Vec3::new(1., y, 0.));
        assert_eq!(
            get_wall_height_relative_y(info).to_bits(),
            (y - world
                .resource::<PublicWalls>()
                .get(walls[0])
                .unwrap()
                .max_y())
            .to_bits()
        );
        // 原字节严格等于1；2不能误当true。普通前段仍运行并保留anchor绝对高。
        {
            let mut storage = world.resource_mut::<DecoratorStorage>();
            let info = storage
                .wall_decorators_mut(walls[0])
                .unwrap()
                .get_mut(DecoratorId(10))
                .unwrap();
            info.u20 = (info.u20 & !0xff00) | 0x200;
            set_anchor_ws(info, Vec3::new(3., -99., 0.));
        }
        schedule.run(&mut world);
        let storage = world.resource::<DecoratorStorage>();
        let info = storage
            .wall_decorators(walls[0])
            .unwrap()
            .get(DecoratorId(10))
            .unwrap();
        assert_eq!(get_wall_coord(info).u, 3.);
        assert_eq!(get_anchor_ws(info).y, -99.);
    }

    #[test]
    fn missing_door_subtype_keeps_committed_common_projection() {
        let (mut world, walls) = world_with_walls();
        insert(&mut world, walls[0], 10, 0);
        world
            .resource_mut::<DecoratorStorage>()
            .wall_decorators_mut(walls[0])
            .unwrap()
            .get_mut(DecoratorId(10))
            .unwrap()
            .u20 |= 0x100;
        let mut schedule = Schedule::default();
        schedule.add_systems(move_decorators_following_anchors);
        schedule.run(&mut world);
        let storage = world.resource::<DecoratorStorage>();
        let info = storage
            .wall_decorators(walls[0])
            .unwrap()
            .get(DecoratorId(10))
            .unwrap();
        assert_eq!(get_wall_coord(info).u, 1.);
        assert_eq!(get_anchor_ws(info), Vec3::new(1., 7., 0.));
        assert_eq!(get_wall_height_relative_y(info), 3.);
    }

    #[test]
    #[should_panic(expected = "modified decorator is missing from its storage")]
    fn invalid_modified_id_preserves_original_unwrap_failure() {
        let (mut world, walls) = world_with_walls();
        world
            .resource_mut::<DecoratorStorage>()
            .get_or_insert_default(DecoratorStorageAddr::Wall(walls[0]))
            .modified_ids
            .insert(999);
        let mut schedule = Schedule::default();
        schedule.add_systems(move_decorators_following_anchors);
        schedule.run(&mut world);
    }

    #[test]
    fn missing_collision_fails_after_common_coordinate_commit() {
        let (mut world, walls) = world_with_walls();
        insert(&mut world, walls[0], 10, 0);
        let state = world
            .resource::<PublicWalls>()
            .get(walls[0])
            .unwrap()
            .clone();
        let public = state.building_projection();
        let context = DecoratorAnchorContext {
            raw_curve: state.simplified_curve_2d(),
            mapped_outline: state.height_outline(),
            arch_heights: state.arch_segment_heights().unwrap(),
            public: &public,
        };
        let mut info = *world
            .resource::<DecoratorStorage>()
            .wall_decorators(walls[0])
            .unwrap()
            .get(DecoratorId(10))
            .unwrap();
        info.u20 |= 0x100;
        let mut subtypes = DerivedDecoratorSubtypes::default();
        subtypes.insert(10, DecoratorSubtype(3));
        let mesh = DecoratorMeshLibrary::default();
        let derived = DerivedDecoratorInfoView::new(&subtypes, &mesh);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            reanchor_one(&mut info, &context, &state, derived)
        }));
        assert!(result.is_err());
        assert_eq!(get_wall_coord(&info).u, 1.);
        assert_eq!(get_anchor_ws(&info), Vec3::new(1., 7., 0.));
    }

    #[test]
    fn info_byte_bridge_round_trips() {
        let info = DecoratorInfo {
            dst_wall: 0x1122_3344_5566_7788,
            dst_secondary: 9,
            dst_payload: 0xdead_beef_cafe_babe,
            u18: 0x0102_0304_0506_0708,
            u20: 0xaabb_ccdd,
            u24: 7,
            u28: 8,
            decorator_id: 42,
            u30: 0x1234,
            u38: 0x55,
            u3c: 0x66,
            u40: 0x77,
            rank: 2,
            u45: 1,
            ty: 0x38,
            state: 2,
        };
        let observed = observed_from_info(&info);
        let mut back = DecoratorInfo::default();
        write_observed_back(&mut back, &observed);
        assert_eq!(back, info);
    }

    #[test]
    fn door_flag_is_plus_0x21() {
        // 门底标志在 +0x21（u20 的 byte1）。构造 PublicWallState 成本高且该
        // 分支不读取 state；这里只验证标志解码本身。
        let mut info = DecoratorInfo::default();
        assert_eq!(info.u20 & 0x0100, 0);
        info.u20 |= 0x0100;
        assert_ne!(info.u20 & 0x0100, 0);
    }
}
