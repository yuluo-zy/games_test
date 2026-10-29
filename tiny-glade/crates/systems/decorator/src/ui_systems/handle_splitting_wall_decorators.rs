//! 原B墙装饰拆分：1412782b0完整picker、141279410完整rank/ID计划及14127a480锚点。
//! 沿原B文件归属恢复，不把箭缝竖向分支、附着墙角或余块恢复简化为删除。
//! 原碰撞库14125cf90是HashMap查找；几何、真实factory、归档和事件payload分别保留。
use crate::apply_decorator_op::{
    DecoratorChangePatch, DecoratorOp, LABEL_MOVE_DST, LABEL_MOVE_SRC, apply_decorator_op,
    apply_place_with_resources,
};
use crate::deferred_decorator_history::DeferredDecoratorEditHistory;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform;
use country_core::resources::walls::decorator::{
    DecoratorDst, DecoratorId, DecoratorInfo, DeferredDecoratorOpEvents, TransientDecoratorIdGen,
    WallDecoratorDst,
};
use country_core::resources::walls::decorator_storage::{DecoratorStorage, DecoratorStorageAddr};
use country_core::resources::walls::decorator_subtype::{
    DecoratorLibKey, DerivedDecoratorInfoView,
};
use country_core::resources::walls::{
    cached_decorator_transforms::CachedDecoratorTransforms,
    decorator_archivist::{DecoratorArchivalResources, LiveDecoratorArchivist},
    public_walls::PublicWalls,
};
use country_core::systems::collision::world::RaycastWorldResult;
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use country_core::systems::ui::signifiers::{SignifierEntityBuilder, SignifierFactory};
use glam::{Vec2, Vec3};
use std::sync::Arc;
use utils::{
    comparative_space::ComparativeSpace,
    wall_space::{WallSpace, WallSpaceCoord},
};

/// B1412782b0：把命中点投影到缓存装饰平面，显示完整rank并返回最近可选部件。
/// 原返回usize，无Option；初始选择0、距离上限100，三件组合的中间件只显示不参与选择。
#[allow(clippy::too_many_arguments)]
pub fn display_decorator_wall_parts_and_get_hovered_part_id(
    cached: &CachedDecoratorTransform,
    hit: &RaycastWorldResult,
    info: &DecoratorInfo,
    visual: &CachedDecoratorTransform,
    derived: DerivedDecoratorInfoView<'_>,
    id: DecoratorId,
    stream: &mut UiSignifierStream,
) -> usize {
    let normal = cached.rotation * Vec3::Z;
    let delta = hit.position - cached.translation;
    let dot = normal.z * delta.z + (delta.x * normal.x + normal.y * delta.y);
    let projected = hit.position - normal * dot;
    let mut best_id = 0;
    let mut best_position = cached.translation;
    let mut best_distance = 100.0;
    for part in 0..usize::from(info.rank) {
        let position = wall_part_position(info, part, visual, derived, id);
        let middle = info.rank == 3 && part == 1;
        if !middle {
            let delta = position - projected;
            let distance = (delta.x * delta.x + delta.y * delta.y + delta.z * delta.z).sqrt();
            if distance < best_distance {
                best_distance = distance;
                best_id = part;
                best_position = position;
            }
        }
        write_split_sphere(
            stream,
            format!("outline_split_{part}_{id}"),
            position,
            if middle { 0.05 } else { 0.2 },
        );
    }
    write_split_sphere(
        stream,
        format!("outline_split_{best_position:?}_{id}"),
        best_position,
        0.35,
    );
    best_id
}

/// 原几何内联；14125cf90实际是碰撞库HashMap::get，不是独立位置助手。
fn wall_part_position(
    info: &DecoratorInfo,
    part: usize,
    visual: &CachedDecoratorTransform,
    derived: DerivedDecoratorInfoView<'_>,
    id: DecoratorId,
) -> Vec3 {
    let bounds = derived
        .mesh_library()
        .collision_record(DecoratorLibKey::new(derived.subtype(id).0, info.rank))
        .bounds;
    let vertical = info.ty == 58;
    let dimension = if vertical {
        bounds.top - bounds.bottom
    } else {
        bounds.footprint.size.x
    };
    let offset = (part as f32 / (usize::from(info.rank).wrapping_sub(1) as f32) + -0.5)
        * (dimension * (1.0 - 1.0 / f32::from(info.rank)));
    let local = Vec3::new(
        offset * f32::from(u8::from(!vertical)),
        offset * f32::from(u8::from(vertical)),
        offset * 0.0,
    );
    let basis = glam::Mat3::from_quat(visual.rotation);
    // 原先按列乘scale，再按x/y/z三列相加；不换成先缩放local后的不同舍入顺序。
    let x = basis.x_axis * visual.scale.x;
    let y = basis.y_axis * visual.scale.y;
    let z = basis.z_axis * visual.scale.z;
    (local.z * z + (local.y * y + local.x * x)) + visual.translation
}

fn write_split_sphere(stream: &mut UiSignifierStream, name: String, position: Vec3, scale: f32) {
    let factory = SignifierFactory::from_closure(
        &name,
        Arc::new(move |meshes, shaders, commands| {
            vec![
                SignifierEntityBuilder::from_mesh_name("temp_sphere")
                    .animation_duration(0.1)
                    .scale(scale)
                    .position(position)
                    .build(meshes, shaders, commands),
            ]
        }),
    );
    stream.write_factory(&factory);
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct RankPart {
    rank: u8,
    offset: Option<Vec2>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct WallSplitPlan {
    remaining: [Option<RankPart>; 2],
    selected_offset: Option<Vec2>,
}

fn wall_split_plan(
    info: &DecoratorInfo,
    part: usize,
    derived: DerivedDecoratorInfoView<'_>,
    id: DecoratorId,
) -> WallSplitPlan {
    if info.dst_tag() > 1 {
        return WallSplitPlan {
            remaining: [None; 2],
            selected_offset: None,
        };
    }
    if info.destination().as_wall().corner_attachment != 0 {
        return WallSplitPlan {
            remaining: [
                Some(RankPart {
                    rank: info.rank.checked_sub(1).expect("原rank不能为0"),
                    offset: None,
                }),
                None,
            ],
            selected_offset: None,
        };
    }
    let subtype = derived.subtype(id).0;
    let get = |rank| {
        derived
            .mesh_library()
            .collision_record(DecoratorLibKey::new(subtype, rank))
            .bounds
    };
    let full = get(info.rank);
    let width = full.footprint.size.x;
    let height = full.top - full.bottom;
    let vertical = info.ty == 58;
    let weights = Vec2::new(
        f32::from(u8::from(!vertical)),
        f32::from(u8::from(vertical)),
    );
    let mut remaining = [None; 2];
    let mut count = 0;
    for (rank, sign) in [
        (part, -1.0),
        (
            usize::from(info.rank).wrapping_sub(1).wrapping_sub(part),
            1.0,
        ),
    ] {
        if rank == 0 {
            continue;
        }
        let bounds = get(rank as u8);
        let delta = Vec2::new(
            width - bounds.footprint.size.x,
            height - (bounds.top - bounds.bottom),
        );
        remaining[count] = Some(RankPart {
            rank: rank as u8,
            // 原竖向余块的tag0不激活offset；不能按对称视觉直觉擅自平移。
            offset: (!vertical).then(|| (delta * 0.5 * weights) * sign),
        });
        count += 1;
    }
    let atomic = get(1);
    assert_ne!(info.rank, 0, "原rank不能为0");
    let ratio = part as f32 / (usize::from(info.rank) - 1) as f32;
    let factor = ratio + ratio + -1.0;
    let offset = (Vec2::new(
        width - atomic.footprint.size.x,
        height - (atomic.top - atomic.bottom),
    ) * 0.5
        * weights)
        * factor;
    WallSplitPlan {
        remaining,
        selected_offset: Some(offset),
    }
}

/// 原返回32B：选中部件新ID与其余部件ID列表；+18是Vec长度，不是Option坐标。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WallDecoratorPartSplit {
    pub moving_id: DecoratorId,
    pub remaining_ids: Vec<DecoratorId>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct SplitAnchor {
    destination: Option<DecoratorDst>,
    pose: CachedDecoratorTransform,
}

/// 原14127a480：即使不偏移也投影坐标并刷新cache，只有offset活跃时写Move目的。
fn split_anchor_in(
    space: &WallSpace,
    source: WallDecoratorDst,
    offset: Option<Vec2>,
    mut to_world: impl FnMut(WallSpaceCoord) -> Vec3,
    mut tangent: impl FnMut(WallSpaceCoord) -> Vec2,
    mut height: impl FnMut(WallSpaceCoord) -> f32,
    compound: bool,
) -> SplitAnchor {
    let offset_value = offset.unwrap_or(Vec2::ZERO);
    let coordinate = WallSpaceCoord::new_unchecked(
        space
            .project(source.coord.u + offset_value.x)
            .expect("原拆分坐标超出墙空间"),
        source.coord.y + offset_value.y,
    );
    let attachment = height(coordinate);
    let position = to_world(coordinate);
    let direction = tangent(coordinate);
    let pose = CachedDecoratorTransform {
        rotation: utils::xz_rotation_bad(Vec2::new(-direction.y, direction.x)),
        translation: position,
        scale: Vec3::ONE,
    };
    let destination = offset.map(|_| {
        let wall = WallDecoratorDst {
            coord: coordinate,
            platform: source.platform,
            relative_y: coordinate.y - attachment,
            position,
            corner_attachment: 0,
            bottom_as_door: source.bottom_as_door,
        };
        if compound {
            DecoratorDst::WallCompound(wall)
        } else {
            DecoratorDst::Wall(wall)
        }
    });
    SplitAnchor { destination, pose }
}

fn split_anchor(
    walls: &PublicWalls,
    address: DecoratorStorageAddr,
    source: Option<WallDecoratorDst>,
    compound: bool,
    offset: Option<Vec2>,
    id: DecoratorId,
    cache: &mut CachedDecoratorTransforms,
) -> SplitAnchor {
    let wall = walls
        .get(address.try_as_wall().expect("原拆分存储必须为墙"))
        .expect("原拆分墙不存在");
    let source = source.expect("原拆分源必须有墙坐标");
    let space = crate::merge_proposal_wall_decorators::wall_space_of(wall);
    let anchor = split_anchor_in(
        &space,
        source,
        offset,
        |coord| coord.to_world_space(wall.simplified_curve_2d()),
        |coord| coord.get_tangent(wall.simplified_curve_2d()),
        |coord| wall.get_wall_attachment_height(coord.u).unwrap(),
        compound,
    );
    cache.insert(id.0, anchor.pose);
    anchor
}

/// 原第三参不是所有分支共用的世界点：Change带Move目的的首16B，Place带原ID/offset记录。
/// inactive payload只作安全初始化，不能将源地址低位伪装成真实world position。
fn anchor_event_data(address: DecoratorStorageAddr, moved: bool) -> [u8; 16] {
    let mut data = [0; 16];
    if moved {
        data[8..].copy_from_slice(&address.as_wall().0.to_le_bytes());
    } else {
        data[..8].copy_from_slice(&3u64.to_le_bytes());
    }
    data
}
fn part_event_data(id: DecoratorId, offset: Option<Vec2>) -> [u8; 16] {
    let mut data = [0; 16];
    data[..4].copy_from_slice(&id.0.to_le_bytes());
    data[4..8].copy_from_slice(&u32::from(offset.is_some()).to_le_bytes());
    if let Some(offset) = offset {
        data[8..12].copy_from_slice(&offset.x.to_le_bytes());
        data[12..].copy_from_slice(&offset.y.to_le_bytes());
    }
    data
}

/// B141279410：按原顺序保留第一余块ID、创建第二余块、最后生成选中atomic；
/// 每步执行真实cache、归档、权威存储、历史与延迟事件，不替换成remove。
#[allow(clippy::too_many_arguments)]
pub fn handle_moving_out_wall_decorator_part(
    part: usize,
    storage: &mut DecoratorStorage,
    address: DecoratorStorageAddr,
    id: DecoratorId,
    walls: &PublicWalls,
    cache: &mut CachedDecoratorTransforms,
    derived: DerivedDecoratorInfoView<'_>,
    ids: &mut TransientDecoratorIdGen<'_>,
    archivist: &mut LiveDecoratorArchivist,
    history: &mut DeferredDecoratorEditHistory,
    events: &DeferredDecoratorOpEvents,
    resources: &DecoratorArchivalResources<'_>,
    state: u8,
    demo: bool,
) -> WallDecoratorPartSplit {
    let source = *storage
        .get_mut(address)
        .and_then(|bucket| bucket.decorators.get(&id.0))
        .expect("原拆分装饰不存在");
    let source_wall = (source.dst_tag() < 2).then(|| *source.destination().as_wall());
    let compound = source.dst_tag() == 1;
    let plan = wall_split_plan(&source, part, derived, id);
    let mut remaining_ids = Vec::new();
    for (index, piece) in plan.remaining.into_iter().flatten().enumerate() {
        let piece_id = if index == 0 {
            id
        } else {
            DecoratorId(ids.generate())
        };
        remaining_ids.push(piece_id);
        let anchor = split_anchor(
            walls,
            address,
            source_wall,
            compound,
            piece.offset,
            piece_id,
            cache,
        );
        if index == 0 {
            let patch = DecoratorChangePatch {
                id: piece_id,
                destination: anchor.destination.map(|dst| (address, dst)),
                rank: Some(piece.rank),
                ty: None,
                word_2c_delta: None, seed_demo_mode: false,
                extra_low: None,
                extra_high: None,
                extra_byte: None,
                event_position: anchor.pose.translation,
                event_position_flag: 0,
            };
            let op = DecoratorOp::from_change_with_event_data(
                address,
                patch,
                anchor_event_data(address, anchor.destination.is_some()),
            );
            archivist.archive_current_decorators_at_with_resources(
                address,
                storage,
                resources,
                LABEL_MOVE_SRC,
            );
            if anchor.destination.is_some() {
                archivist.archive_current_decorators_at_with_resources(
                    address,
                    storage,
                    resources,
                    LABEL_MOVE_DST,
                );
            }
            apply_decorator_op(
                &op,
                state,
                address,
                storage,
                Some(&mut *archivist),
                Some(&mut *history),
                events,
            );
        } else {
            let mut info = source;
            if let Some(destination) = anchor.destination {
                info.set_destination(destination);
            }
            info.rank = piece.rank;
            let mut op =
                DecoratorOp::place_at(piece_id, info, anchor.pose.translation, 0, true, demo);
            op.extra[..16].copy_from_slice(&part_event_data(piece_id, piece.offset));
            apply_place_with_resources(
                &op,
                state,
                address,
                storage,
                archivist,
                Some(&mut *history),
                resources,
                events,
            );
        }
    }
    let moving_id = DecoratorId(ids.generate());
    let anchor = split_anchor(
        walls,
        address,
        source_wall,
        compound,
        plan.selected_offset,
        moving_id,
        cache,
    );
    let mut info = source;
    if let Some(destination) = anchor.destination {
        info.set_destination(destination);
    }
    info.rank = 1;
    let mut op = DecoratorOp::place_at(moving_id, info, anchor.pose.translation, 0, true, demo);
    op.extra[..16].copy_from_slice(&part_event_data(moving_id, plan.selected_offset));
    apply_place_with_resources(
        &op,
        state,
        address,
        storage,
        archivist,
        Some(history),
        resources,
        events,
    );
    WallDecoratorPartSplit {
        moving_id,
        remaining_ids,
    }
}

#[cfg(test)]
mod native_tests {
    use super::*;
    use country_core::resources::walls::decorator_subtype::{
        DecoratorCollision, DecoratorMeshLibrary, DecoratorSubtype, DerivedDecoratorSubtypes,
    };
    use country_core::systems::collision::world::RaycastMeshData;
    fn catalog() -> (DerivedDecoratorSubtypes, DecoratorMeshLibrary) {
        let mut subtypes = DerivedDecoratorSubtypes::default();
        subtypes.insert(42, DecoratorSubtype(7));
        let mut library = DecoratorMeshLibrary::default();
        let mesh = Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::ZERO));
        for rank in 1..=5 {
            let n = f64::from(rank);
            library.collision.insert(
                DecoratorLibKey::new(7, rank),
                DecoratorCollision {
                    triangles: Vec::new(),
                    mesh: mesh.clone(),
                    bounds: utils::geometry::Rectangle3d {
                        footprint: utils::geometry::Rectangle2d {
                            axis: Vec2::X,
                            center: Vec2::ZERO,
                            size: Vec2::new((n * 1.3 + n * n * 0.07) as f32, 1.),
                        },
                        bottom: -0.2,
                        top: (n * 1.7 + n * n * 0.11) as f32,
                    },
                },
            );
        }
        (subtypes, library)
    }
    fn numbers(s: &str) -> Vec<u32> {
        s.split(',').map(|v| v.parse().unwrap()).collect()
    }
    fn pose(s: &str) -> CachedDecoratorTransform {
        let v = numbers(s);
        let f = |i| f32::from_bits(v[i]);
        CachedDecoratorTransform {
            rotation: glam::Quat::from_xyzw(f(0), f(1), f(2), f(3)),
            translation: Vec3::new(f(4), f(5), f(6)),
            scale: Vec3::new(f(7), f(8), f(9)),
        }
    }
    #[test]
    fn original_split_plan_keeps_rank_offsets_and_vertical_inactive_payload() {
        let rows = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m01-split-plan-native.tsv"
        ));
        let (subtypes, library) = catalog();
        let view = DerivedDecoratorInfoView::new(&subtypes, &library);
        let mut count = 0;
        for line in rows.lines().filter(|s| !s.starts_with('#')) {
            let f: Vec<_> = line.split('\t').collect();
            let mut info = DecoratorInfo {
                ty: f[0].parse().unwrap(),
                rank: f[1].parse().unwrap(),
                ..Default::default()
            };
            info.u20 = f[3].parse::<u32>().unwrap() | ((f[4].parse::<u32>().unwrap()) << 16);
            let plan = wall_split_plan(&info, f[2].parse().unwrap(), view, DecoratorId(42));
            let expected: Vec<_> = f[5].split(';').map(numbers).collect();
            let actual: Vec<_> = plan
                .remaining
                .into_iter()
                .flatten()
                .chain(std::iter::once(RankPart {
                    rank: 1,
                    offset: plan.selected_offset,
                }))
                .collect();
            assert_eq!(actual.len(), expected.len(), "{line}");
            for (part, native) in actual.iter().zip(&expected) {
                assert_eq!(part.rank, u8::try_from(native[4]).unwrap(), "{line}");
                assert_eq!(part.offset.is_some(), native[1] == 1, "{line}");
                if let Some(offset) = part.offset {
                    assert_eq!(
                        offset.to_array().map(f32::to_bits),
                        [native[2], native[3]],
                        "{line}"
                    );
                }
            }
            count += 1;
        }
        assert_eq!(count, 252);
    }
    #[test]
    fn original_complete_picker_matches_positions_and_selection_with_real_owned_factories() {
        use bevy_ecs::{prelude::*, world::CommandQueue};
        use country_core::{
            asset_libraries::{
                mesh_library::{AssetMeshLibrary, MeshAsset},
                shader_library::AssetShaderLibrary,
            },
            render::{mesh::Mesh, shader::RasterShaderProgram, test_device::TestDevice},
            systems::ui::signifiers::{SignifierAnimation, UiSignifierEntry},
            utils::components::transform::Transform,
        };
        let (subtypes, library) = catalog();
        let view = DerivedDecoratorInfoView::new(&subtypes, &library);
        let meshes = AssetMeshLibrary::new();
        meshes.add(MeshAsset::new(
            Mesh::new("temp_sphere"),
            Some("temp_sphere".to_owned()),
        ));
        let shaders = AssetShaderLibrary::new();
        shaders.add_raster(
            RasterShaderProgram::new_draw(
                "vertex_color_shader",
                Default::default(),
                &TestDevice(1),
            ),
            false,
        );
        let rows = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m01-split-display-native.tsv"
        ));
        let mut count = 0;
        for line in rows.lines().filter(|s| !s.starts_with('#')) {
            let f: Vec<_> = line.split('\t').collect();
            let info = DecoratorInfo {
                ty: f[0].parse().unwrap(),
                rank: f[1].parse().unwrap(),
                ..Default::default()
            };
            let cached = pose(f[2]);
            let visual = pose(f[3]);
            let point = numbers(f[4]);
            let hit = RaycastWorldResult {
                handle: [4, 0, 0],
                position: Vec3::from_array(std::array::from_fn(|i| f32::from_bits(point[i]))),
                normal: Vec3::Z,
                toi: 0.,
                discriminant: 0,
                extra: [0.; 2],
            };
            let mut stream = UiSignifierStream::default();
            let selected = display_decorator_wall_parts_and_get_hovered_part_id(
                &cached,
                &hit,
                &info,
                &visual,
                view,
                DecoratorId(42),
                &mut stream,
            );
            assert_eq!(selected, f[5].parse::<usize>().unwrap(), "{line}");
            let native: Vec<_> = f[6].split(';').map(numbers).collect();
            let entries = stream.take_entries();
            assert_eq!(entries.len(), native.len());
            let mut world = World::new();
            let mut queue = CommandQueue::default();
            for (index, (entry, expected)) in entries.into_iter().zip(&native).enumerate() {
                let UiSignifierEntry::Factory(factory) = entry else {
                    panic!("必须是真实工厂")
                };
                let entities =
                    factory.wake(&meshes, &shaders, &mut Commands::new(&mut queue, &world));
                queue.apply(&mut world);
                assert_eq!(entities.len(), 1);
                let entity = entities[0];
                assert_eq!(
                    world
                        .get::<Transform>(entity)
                        .unwrap()
                        .translation
                        .to_array()
                        .map(f32::to_bits),
                    expected.as_slice(),
                    "{line} factory={index}"
                );
                let animation = world.get::<SignifierAnimation>(entity).unwrap();
                let scale: f32 = if index == usize::from(info.rank) {
                    0.35
                } else if info.rank == 3 && index == 1 {
                    0.05
                } else {
                    0.2
                };
                assert_eq!(animation.value.to_bits(), scale.to_bits());
                assert_eq!(animation.duration.to_bits(), 0.1f32.to_bits());
                assert!(factory.name().unwrap().starts_with("outline_split_"));
                world.despawn(entity);
            }
            count += 1;
        }
        assert_eq!(count, 288);
    }

    #[test]
    fn original_anchor_control_flow_projection_rotation_and_payload_match() {
        let rows = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m01-split-anchor-native.tsv"
        ));
        let mut checked = 0;
        let mut rejected = 0;
        for line in rows.lines().filter(|s| !s.starts_with('#')) {
            let f: Vec<_> = line.split('\t').collect();
            let scalar = |i: usize| f[i].parse::<f32>().unwrap();
            let space = WallSpace {
                is_loop: f[0] == "1",
                length: 20.,
            };
            let source = WallDecoratorDst {
                coord: WallSpaceCoord::new_unchecked(scalar(1), scalar(2)),
                platform: country_core::resources::walls::WallId(9),
                relative_y: 0.,
                position: Vec3::ZERO,
                corner_attachment: 3,
                bottom_as_door: true,
            };
            let offset = (f[5] == "1").then(|| Vec2::new(scalar(3), scalar(4)));
            let got = std::panic::catch_unwind(|| {
                split_anchor_in(
                    &space,
                    source,
                    offset,
                    |coord| Vec3::new(coord.u, coord.y, 0.),
                    |_| Vec2::new(scalar(7), scalar(8)),
                    |_| 1.25,
                    f[6] == "1",
                )
            });
            if f[9] == "1" {
                assert!(got.is_err(), "{line}");
                rejected += 1;
            } else {
                let got = got.unwrap();
                let bytes: Vec<_> = f[10]
                    .as_bytes()
                    .chunks_exact(2)
                    .map(|v| u8::from_str_radix(std::str::from_utf8(v).unwrap(), 16).unwrap())
                    .collect();
                let u64_at = |at| u64::from_le_bytes(bytes[at..at + 8].try_into().unwrap());
                let float = |at| f32::from_le_bytes(bytes[at..at + 4].try_into().unwrap());
                let expected = numbers(f[11]);
                assert_eq!(
                    got.pose.to_components().map(f32::to_bits),
                    expected.as_slice(),
                    "{line}"
                );
                if f[5] == "1" {
                    assert_eq!(u64_at(0), 0);
                    assert_eq!(u64_at(8), 88);
                    let destination = got.destination.unwrap();
                    assert_eq!(destination.tag(), f[6].parse::<u8>().unwrap());
                    let dst = destination.as_wall();
                    assert_eq!(
                        dst.coord.as_vec2().to_array().map(f32::to_bits),
                        [float(16).to_bits(), float(20).to_bits()]
                    );
                    assert_eq!(dst.platform.0, u64_at(24));
                    assert_eq!(dst.relative_y.to_bits(), float(32).to_bits());
                    assert_eq!(
                        dst.position.to_array().map(f32::to_bits),
                        [
                            float(36).to_bits(),
                            float(40).to_bits(),
                            float(44).to_bits()
                        ]
                    );
                    assert_eq!(dst.corner_attachment, bytes[48]);
                    assert_eq!(u8::from(dst.bottom_as_door), bytes[49]);
                    assert_eq!(destination.tag(), bytes[50]);
                    assert_eq!(bytes[56], 1);
                } else {
                    assert!(got.destination.is_none());
                    assert_eq!(u64_at(0), 3);
                }
            }
            checked += 1;
        }
        assert_eq!((checked, rejected), (144, 8));
    }

    #[test]
    fn real_world_split_archives_both_colliders_restores_rejected_preview_and_reuses_preview_ids() {
        use crate::{
            backup::{DecoratorBackup, create_or_restore_decorator_backup},
            blueprint::DecoratorBlueprints,
        };
        use bevy_ecs::{prelude::*, system::RunSystemOnce};
        use country_core::resources::{
            stairs::stairs_state::StairsState,
            walls::{
                InnerWalls, WallId, WallState,
                decorator::{DecoratorExtraInfo, DecoratorIdGen, WindowExtra},
                decorator_archivist::LiveColliderPayload,
                decorator_collision::DecoratorPhysicsColliders,
                public_walls::{WallPublicationDirty, WallSpatialHash},
                publication_fields::WallShapeSnapshot,
                publication_source::TerrainHeightGrid512,
            },
        };
        use country_core::systems::collision::world::{
            RaycastGeoSource, RaycastWorld, upsert_collider,
        };
        #[derive(Resource)]
        struct Request {
            address: DecoratorStorageAddr,
            id: DecoratorId,
            part: usize,
            commit: bool,
        }
        #[derive(Resource, Default)]
        struct Output(Option<WallDecoratorPartSplit>);
        fn apply(
            request: Res<Request>,
            mut storage: ResMut<DecoratorStorage>,
            walls: Res<PublicWalls>,
            mut cache: ResMut<CachedDecoratorTransforms>,
            derived: (Res<DerivedDecoratorSubtypes>, Res<DecoratorMeshLibrary>),
            mut ids: ResMut<DecoratorIdGen>,
            mut archive: ResMut<LiveDecoratorArchivist>,
            mut history: ResMut<DeferredDecoratorEditHistory>,
            events: Res<DeferredDecoratorOpEvents>,
            resources: DecoratorArchivalResources,
            mut output: ResMut<Output>,
        ) {
            let mut ids = ids.transient(request.commit);
            output.0 = Some(handle_moving_out_wall_decorator_part(
                request.part,
                &mut storage,
                request.address,
                request.id,
                &walls,
                &mut cache,
                DerivedDecoratorInfoView::new(&derived.0, &derived.1),
                &mut ids,
                &mut archive,
                &mut history,
                &events,
                &resources,
                u8::from(request.commit),
                false,
            ));
        }
        fn archive(
            request: Res<Request>,
            storage: Res<DecoratorStorage>,
            resources: DecoratorArchivalResources,
            mut archive: ResMut<LiveDecoratorArchivist>,
        ) {
            archive.archive_current_decorators_at_with_resources(
                request.address,
                &storage,
                &resources,
                "test initial valid frame",
            );
        }
        let points = (0..=32)
            .map(|i| {
                let a = i as f32 * core::f32::consts::TAU / 32.;
                [a.cos() * 8., a.sin() * 8.]
            })
            .collect();
        let curve =
            country_core::resources::roofs::geometry::surface::Curve2::try_new(points, false)
                .unwrap();
        let authority = WallState::new(curve, 7., 0, 0, false);
        let (mut snapshot, freehand) = authority.into_snapshot();
        let mut shape = [0; 28];
        shape[12..16].copy_from_slice(&8f32.to_le_bytes());
        snapshot.source.shape = WallShapeSnapshot::from_observed(shape);
        snapshot.source.terrain_floor = 0.;
        let mut inner = InnerWalls::new();
        let wall = inner.new_wall(WallState::from_snapshot(snapshot, freehand));
        let mut walls = PublicWalls::default();
        let terrain = TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.,
            maximum_y: 0.,
            sampling_fix: false,
        };
        walls.consume_with_paths(
            &mut inner,
            &terrain,
            None,
            &WallPublicationDirty::default(),
            &WallSpatialHash::default(),
        );
        assert!(walls.get(wall).unwrap().arch_segment_heights().is_some());
        let id = DecoratorId(42);
        let address = DecoratorStorageAddr::Wall(wall);
        let coord = WallSpaceCoord::new_unchecked(10., 3.);
        let position = coord.to_world_space(walls.get(wall).unwrap().simplified_curve_2d());
        let mut info = DecoratorInfo {
            rank: 3,
            ty: 56,
            u24: 0x12345678,
            u28: 0x99887766,
            u45: 5,
            ..Default::default()
        };
        info.set_destination(DecoratorDst::Wall(WallDecoratorDst {
            coord,
            platform: WallId(19),
            relative_y: 2.,
            position,
            corner_attachment: 0,
            bottom_as_door: true,
        }));
        info.set_extra(DecoratorExtraInfo::Window(WindowExtra(0x11223344)));
        let mut storage = DecoratorStorage::default();
        storage.get_or_insert_default(address).insert(id, info);
        storage.decorator_to_storage.insert(id, address);
        let (mut subtypes, mut meshes) = catalog();
        subtypes.insert(id.0, DecoratorSubtype(0));
        for rank in 1..=5 {
            let record = meshes.collision[&DecoratorLibKey::new(7, rank)].clone();
            meshes
                .collision
                .insert(DecoratorLibKey::new(0, rank), record);
        }
        let mut physics = DecoratorPhysicsColliders::default();
        let mut raycasting = RaycastWorld::new();
        let mesh = Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::ZERO));
        upsert_collider(
            &mut raycasting,
            &mut physics.first,
            id,
            RaycastGeoSource::decorator(id, address),
            mesh.clone(),
            position,
            glam::Quat::IDENTITY,
        );
        upsert_collider(
            &mut raycasting,
            &mut physics.second,
            id,
            RaycastGeoSource::decorator_interaction(id, address, 0),
            mesh.clone(),
            position,
            glam::Quat::IDENTITY,
        );
        raycasting.flush_colliders();
        let mut world = World::new();
        world.insert_resource(walls);
        world.insert_resource(storage);
        world.insert_resource(subtypes);
        world.insert_resource(meshes);
        world.insert_resource(physics);
        world.insert_resource(raycasting);
        world.init_resource::<CachedDecoratorTransforms>();
        let initial_pose = CachedDecoratorTransform {
            rotation: glam::Quat::IDENTITY,
            translation: position,
            scale: Vec3::ONE,
        };
        world
            .resource_mut::<CachedDecoratorTransforms>()
            .insert(id.0, initial_pose);
        let mut ids = DecoratorIdGen::default();
        ids.reserve(DecoratorId(999));
        world.insert_resource(ids);
        world.init_resource::<LiveDecoratorArchivist>();
        world.init_resource::<DeferredDecoratorEditHistory>();
        world.init_resource::<DeferredDecoratorOpEvents>();
        world.init_resource::<DecoratorBlueprints>();
        world.init_resource::<DecoratorBackup>();
        world.init_resource::<StairsState>();
        world.init_resource::<Output>();
        world.resource_mut::<StairsState>().add_node(9);
        world.resource_mut::<StairsState>().make_backup();
        world.insert_resource(Request {
            address,
            id,
            part: 0,
            commit: false,
        });
        world.run_system_once(archive).unwrap();
        assert_eq!(
            world.resource::<LiveDecoratorArchivist>().collision_a.len(),
            1
        );
        assert_eq!(
            world.resource::<LiveDecoratorArchivist>().collision_b.len(),
            1
        );
        world.resource_mut::<DecoratorBackup>().state = 1;
        world
            .run_system_once(create_or_restore_decorator_backup::<LiveColliderPayload>)
            .unwrap();
        world.run_system_once(apply).unwrap();
        let result = world.resource::<Output>().0.clone().unwrap();
        assert_eq!(
            result,
            WallDecoratorPartSplit {
                moving_id: DecoratorId(1000),
                remaining_ids: vec![id]
            }
        );
        let after = world.resource::<DecoratorStorage>();
        assert_eq!(after.decorator_info(id).unwrap().rank, 2);
        let moved = *after.decorator_info(result.moving_id).unwrap();
        assert_eq!(moved.rank, 1);
        assert_eq!(moved.state, 0);
        assert_eq!(moved.u24, info.u24);
        assert_eq!(moved.u28, info.u28);
        assert_eq!(moved.extra(), info.extra());
        assert_eq!(world.resource::<DecoratorIdGen>().peek(), DecoratorId(1000));
        let history = world.resource::<DeferredDecoratorEditHistory>();
        assert_eq!(
            history.records.iter().map(|r| r.kind).collect::<Vec<_>>(),
            [3, 4]
        );
        assert!(!history.pending);
        world
            .resource::<DeferredDecoratorOpEvents>()
            .with_events(|events| {
                assert_eq!(events.len(), 2);
                assert_eq!(&events[0][0x70..0x80], &anchor_event_data(address, true));
                assert_eq!(
                    &events[1][0x70..0x80],
                    &part_event_data(
                        result.moving_id,
                        wall_split_plan(
                            &info,
                            0,
                            DerivedDecoratorInfoView::new(
                                world.resource::<DerivedDecoratorSubtypes>(),
                                world.resource::<DecoratorMeshLibrary>()
                            ),
                            id
                        )
                        .selected_offset
                    )
                );
            });
        world.resource_mut::<DecoratorBackup>().state = 2;
        world
            .run_system_once(create_or_restore_decorator_backup::<LiveColliderPayload>)
            .unwrap();
        assert_eq!(
            world.resource::<DecoratorStorage>().decorator_info(id),
            Some(&info)
        );
        assert!(
            world
                .resource::<DecoratorStorage>()
                .decorator_info(DecoratorId(1000))
                .is_none()
        );
        assert!(
            world
                .resource::<DecoratorStorage>()
                .lookup_storage(DecoratorId(1000))
                .is_none()
        );
        assert_eq!(
            world
                .resource::<CachedDecoratorTransforms>()
                .try_transform(id.0),
            Some(initial_pose)
        );
        assert!(
            world
                .resource::<CachedDecoratorTransforms>()
                .try_transform(1000)
                .is_none()
        );
        assert!(
            world
                .resource::<DeferredDecoratorEditHistory>()
                .records
                .is_empty()
        );
        assert_eq!(
            world
                .resource::<DeferredDecoratorOpEvents>()
                .with_events(|v| v.len()),
            0
        );
        world.resource_mut::<Request>().commit = true;
        world.run_system_once(apply).unwrap();
        assert_eq!(
            world.resource::<Output>().0.as_ref().unwrap().moving_id,
            DecoratorId(1000)
        );
        assert_eq!(world.resource::<DecoratorIdGen>().peek(), DecoratorId(1001));
        assert_eq!(
            world
                .resource::<DecoratorStorage>()
                .decorator_info(DecoratorId(1000))
                .unwrap()
                .state,
            1
        );
        assert_eq!(
            world
                .resource::<LiveDecoratorArchivist>()
                .all_decorators()
                .count(),
            1,
            "重复预览/提交保留最初归档"
        );
        // 同一真实World继续比较原outer全部252组的可观察rank/ID/事件顺序，
        // 不用另写一份Rust计划模拟来冒充handler执行。
        let native = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m01-split-plan-native.tsv"
        ));
        for line in native.lines().filter(|s| !s.starts_with('#')) {
            let f: Vec<_> = line.split('\t').collect();
            let mut source = info;
            source.ty = f[0].parse().unwrap();
            source.rank = f[1].parse().unwrap();
            let mut destination = *info.destination().as_wall();
            destination.corner_attachment = f[3].parse().unwrap();
            source.set_destination(if f[4] == "1" {
                DecoratorDst::WallCompound(destination)
            } else {
                DecoratorDst::Wall(destination)
            });
            let mut storage = DecoratorStorage::default();
            storage.get_or_insert_default(address).insert(id, source);
            storage.decorator_to_storage.insert(id, address);
            world.insert_resource(storage);
            world.insert_resource(LiveDecoratorArchivist::default());
            world.insert_resource(DeferredDecoratorEditHistory::default());
            world.insert_resource(DeferredDecoratorOpEvents::default());
            world.insert_resource(CachedDecoratorTransforms::default());
            let mut ids = DecoratorIdGen::default();
            ids.reserve(DecoratorId(999));
            world.insert_resource(ids);
            *world.resource_mut::<Request>() = Request {
                address,
                id,
                part: f[2].parse().unwrap(),
                commit: false,
            };
            world.run_system_once(apply).unwrap();
            let output = world.resource::<Output>().0.as_ref().unwrap();
            assert_eq!(output.moving_id.0, f[6].parse::<u32>().unwrap(), "{line}");
            let expected_remaining: Vec<_> = f[7]
                .split(',')
                .filter(|s| !s.is_empty())
                .map(|s| DecoratorId(s.parse().unwrap()))
                .collect();
            assert_eq!(output.remaining_ids, expected_remaining, "{line}");
            assert_eq!(world.resource::<DecoratorIdGen>().peek(), DecoratorId(1000));
            let descriptors: Vec<_> = f[5].split(';').map(numbers).collect();
            world
                .resource::<DeferredDecoratorOpEvents>()
                .with_events(|events| {
                    assert_eq!(events.len(), descriptors.len(), "{line}");
                    for (index, (event, part)) in events.iter().zip(&descriptors).enumerate() {
                        let (id_at, rank_at) = if index == 0 && !expected_remaining.is_empty() {
                            (0x68, 0x61)
                        } else {
                            (0x40, 0x45)
                        };
                        assert_eq!(
                            u32::from_le_bytes(event[id_at..id_at + 4].try_into().unwrap()),
                            part[0],
                            "{line}"
                        );
                        assert_eq!(event[rank_at], part[4] as u8, "{line}");
                        assert_eq!(
                            world
                                .resource::<DecoratorStorage>()
                                .decorator_info(DecoratorId(part[0]))
                                .unwrap()
                                .rank,
                            part[4] as u8,
                            "{line}"
                        );
                    }
                });
            let history = world.resource::<DeferredDecoratorEditHistory>();
            assert_eq!(history.records.len(), descriptors.len());
            assert_eq!(
                history.records.iter().map(|r| r.kind).collect::<Vec<_>>(),
                (0..descriptors.len())
                    .map(|i| if i == 0 && !expected_remaining.is_empty() {
                        3
                    } else {
                        4
                    })
                    .collect::<Vec<_>>()
            );
        }
        drop(world);
        assert_eq!(
            Arc::strong_count(&mesh),
            1,
            "真实registry、两表归档及World释放全部网格引用"
        );
    }

    #[test]
    fn original_failure_order_keeps_missing_record_before_id_and_bad_source_after_id() {
        use bevy_ecs::{prelude::*, system::SystemState};
        use country_core::{
            resources::walls::{
                decorator::{DecoratorIdGen, DecoratorRotationXZ, TerrainDecoratorDst},
                decorator_collision::DecoratorPhysicsColliders,
            },
            systems::collision::world::RaycastWorld,
        };
        let mut world = World::new();
        world.insert_resource(RaycastWorld::new());
        world.init_resource::<DecoratorPhysicsColliders>();
        let mut params = SystemState::<DecoratorArchivalResources>::new(&mut world);
        let resources = params.get(&world);
        let mut ids = DecoratorIdGen::default();
        ids.reserve(DecoratorId(999));
        let mut storage = DecoratorStorage::default();
        let mut cache = CachedDecoratorTransforms::default();
        let mut archive = LiveDecoratorArchivist::default();
        let mut history = DeferredDecoratorEditHistory::default();
        let events = DeferredDecoratorOpEvents::default();
        let subtypes = DerivedDecoratorSubtypes::default();
        let meshes = DecoratorMeshLibrary::default();
        let walls = PublicWalls::default();
        let view = DerivedDecoratorInfoView::new(&subtypes, &meshes);
        let address = DecoratorStorageAddr::Terrain;
        let missing = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut transient = ids.transient(true);
            handle_moving_out_wall_decorator_part(
                0,
                &mut storage,
                address,
                DecoratorId(42),
                &walls,
                &mut cache,
                view,
                &mut transient,
                &mut archive,
                &mut history,
                &events,
                &resources,
                1,
                false,
            )
        }));
        assert!(missing.is_err());
        assert_eq!(ids.peek(), DecoratorId(1000));
        let mut info = DecoratorInfo {
            ty: 56,
            rank: 2,
            ..Default::default()
        };
        info.set_destination(DecoratorDst::Terrain(TerrainDecoratorDst {
            position_xz: Vec2::new(2., 3.),
            yaw: DecoratorRotationXZ(0.3),
        }));
        storage
            .get_or_insert_default(address)
            .insert(DecoratorId(42), info);
        storage
            .decorator_to_storage
            .insert(DecoratorId(42), address);
        let source = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut transient = ids.transient(true);
            handle_moving_out_wall_decorator_part(
                0,
                &mut storage,
                address,
                DecoratorId(42),
                &walls,
                &mut cache,
                view,
                &mut transient,
                &mut archive,
                &mut history,
                &events,
                &resources,
                1,
                false,
            )
        }));
        assert!(source.is_err());
        assert_eq!(
            ids.peek(),
            DecoratorId(1001),
            "原新ID先生成再验证source墙地址"
        );
        assert_eq!(storage.decorator_info(DecoratorId(42)), Some(&info));
        assert!(history.records.is_empty());
        assert_eq!(events.with_events(|events| events.len()), 0);
        assert_eq!(archive.all_storages().count(), 0);
        assert!(cache.try_transform(1000).is_none());
    }
}
