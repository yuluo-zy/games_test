//! B原屋顶装饰分裂：真实屋顶曲面、指示符、分裂操作、归档和历史共享同一身份链。
use crate::{
    apply_decorator_op::{
        DecoratorChangePatch, DecoratorOp, apply_decorator_op, apply_place_with_resources,
    },
    deferred_decorator_history::DeferredDecoratorEditHistory,
};
use bevy_ecs::prelude::Query;
use country_core::resources::{
    roofs::{Roof, RoofTriggerVolume},
    walls::{
        cached_decorator_transforms::CachedDecoratorTransform,
        decorator::{
            DecoratorDst, DecoratorId, DecoratorInfo, DecoratorRotationXZ,
            DeferredDecoratorOpEvents, RoofDecoratorDst, TransientDecoratorIdGen,
        },
        decorator_archivist::{DecoratorArchivalResources, LiveDecoratorArchivist},
        decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
        decorator_subtype::{DecoratorLibKey, DecoratorSubtype, DerivedDecoratorInfoView},
        public_walls::PublicWalls,
    },
};
use country_core::systems::{
    collision::world::RaycastWorldResult,
    ui::signifiers::{
        signifier_create::UiSignifierStream,
        signifier_factory::{SignifierEntityBuilder, SignifierFactory},
    },
};
use glam::{Quat, Vec2, Vec3};
use std::sync::Arc;

/// 原结果Vec<ID>、新ID、完整目的地；不把40B目的地误当作变换缓存。
#[derive(Debug, PartialEq)]
pub struct RoofSplitResult {
    pub remaining_ids: Vec<DecoratorId>,
    pub moving_id: DecoratorId,
    pub destination: DecoratorDst,
}

const WIDTHS: [f32; 3] = [f32::from_bits(0x3f4ccccd), 1.5, f32::from_bits(0x400ccccd)];
fn width(rank: u8) -> f32 {
    let index = rank.wrapping_sub(1);
    assert!(index < 3, "unsupported roof decorator rank");
    WIDTHS[index as usize]
}

/// 原14127c7f0，仅回绕一次；不能用rem_euclid改变大偏移和端点1的行为。
fn shifted_roof_destination(
    roof: &Roof,
    position: Vec3,
    base: RoofDecoratorDst,
    distance: f32,
) -> DecoratorDst {
    let delta = roof_distance_to_u(roof, position, distance);
    roof_destination_with_delta(base, delta)
}

fn roof_distance_to_u(roof: &Roof, position: Vec3, distance: f32) -> f32 {
    let shape = crate::merge_proposal_roof_decorators::section_shape(roof, position);
    if roof.observed()[8] & 1 == 0 {
        distance / shape.perimeter()
    } else {
        let rectangle = shape.as_rectangle();
        let edge = rectangle.get_closest_edge_id(Vec2::new(position.x, position.z));
        (distance / rectangle.edge_length(edge)) * 0.25
    }
}

fn roof_destination_with_delta(base: RoofDecoratorDst, delta: f32) -> DecoratorDst {
    let mut u = delta + base.coord.x;
    if u > 1.0 {
        u += -1.0;
    } else if u < 0.0 {
        u += 1.0;
    }
    DecoratorDst::Roof(RoofDecoratorDst {
        coord: Vec2::new(u, base.coord.y),
        yaw: DecoratorRotationXZ(0.0),
    })
}

/// 原三个闭包都创建真实temp_sphere；中间不可选、普通部件和当前候选有独立scale。
fn sphere(name: String, position: Vec3, scale: f32, stream: &mut UiSignifierStream) {
    let factory = SignifierFactory::from_closure(
        &name,
        Arc::new(move |meshes, shaders, commands| {
            vec![
                SignifierEntityBuilder::from_mesh_name("temp_sphere")
                    .animation_duration(f32::from_bits(0x3dcccccd))
                    .scale(scale)
                    .position(position)
                    .build(meshes, shaders, commands),
            ]
        }),
    );
    stream.write_factory(&factory);
}

/// 原GLAM旋转的完整多项式；不删零乘项，不为非单位四元数补归一化。
fn rotate(q: Quat, v: Vec3) -> Vec3 {
    let dot = (q.z * v.z + q.y * v.y) + q.x * v.x;
    let dot = dot + dot;
    let squared = (q.z * q.z + q.y * q.y) + q.x * q.x;
    let ww = q.w * q.w - squared;
    let tw = q.w + q.w;
    Vec3::new(
        ((q.y * v.z - q.z * v.y) * tw + dot * q.x) + ww * v.x,
        ((q.z * v.x - q.x * v.z) * tw + dot * q.y) + ww * v.y,
        ((q.x * v.y - q.y * v.x) * tw + dot * q.z) + ww * v.z,
    )
}

/// 原14127a790：查询两组件均成功后才显示；返回仅对原subtype2/8生效。
#[allow(clippy::too_many_arguments)]
pub fn try_display_roof_decorator_parts_and_get_hovered_part_id(
    hit: &RaycastWorldResult,
    info: &DecoratorInfo,
    transform: &CachedDecoratorTransform,
    id: DecoratorId,
    subtype: DecoratorSubtype,
    address: DecoratorStorageAddr,
    stream: &mut UiSignifierStream,
    walls: &PublicWalls,
    roofs: &Query<&Roof>,
    volumes: &Query<&RoofTriggerVolume>,
) -> Option<usize> {
    let entity = walls.roof_entity(address.as_wall())?;
    let roof = roofs.get(entity).ok()?;
    let volume = volumes.get(entity).ok()?;
    Some(display_roof_decorator_parts_and_get_hovered_part_id(
        hit, info, transform, id, stream, roof, volume,
    ))
    .filter(|_| matches!(subtype.0, 2 | 8))
}

/// 原内联display主体，默认索引0、距离上限100，三联中心部件禁止成为候选。
fn display_roof_decorator_parts_and_get_hovered_part_id(
    hit: &RaycastWorldResult,
    info: &DecoratorInfo,
    transform: &CachedDecoratorTransform,
    id: DecoratorId,
    stream: &mut UiSignifierStream,
    roof: &Roof,
    volume: &RoofTriggerVolume,
) -> usize {
    let normal = rotate(transform.rotation, Vec3::Z);
    let d = hit.position - transform.translation;
    let distance = (normal.z * d.z + d.x * normal.x) + normal.y * d.y;
    let projected = hit.position - normal * distance;
    let rank = info.rank;
    let mut best = 0;
    let mut best_position = transform.translation;
    let mut best_distance = 100.0;
    for part in 0..usize::from(rank) {
        let offset = (part as f32 / (usize::from(rank).wrapping_sub(1) as f32) + (-0.5))
            * (width(rank) * (1.0 - 1.0 / f32::from(rank)));
        // 原先计算截面/边长，之后才检查dst_as_roof；rank0完全不读取该变体。
        let delta = roof_distance_to_u(roof, transform.translation, offset);
        let base = *info.dst_as_roof().destination().as_roof();
        let destination = roof_destination_with_delta(base, delta);
        let roof_destination = destination.as_roof();
        let position = volume
            .get_position_from_mesh_uv(roof_destination.coord, roof.floor_y())
            .unwrap_or_else(|| {
                transform.translation
                    + rotate(
                        transform.rotation,
                        Vec3::new(offset, offset * 0.0, offset * 0.0),
                    )
            });
        let marker = position + Vec3::new(0.0, 0.5, 0.0);
        if rank == 3 && part == 1 {
            sphere(
                format!("outline_split_{part}_{id:?}"),
                marker,
                f32::from_bits(0x3d4ccccd),
                stream,
            );
        } else {
            let d = position - projected;
            let distance = ((d.z * d.z + d.y * d.y) + d.x * d.x).sqrt();
            if distance < best_distance {
                best = part;
                best_position = marker;
                best_distance = distance;
            }
            sphere(format!("outline_split_{part}_{id:?}"), marker, 0.2, stream);
        }
    }
    sphere(
        format!("outline_split_{best_position:?}_{id:?}"),
        best_position,
        0.35,
        stream,
    );
    best
}

#[derive(Clone, Copy, Debug)]
struct FracturedPart {
    rank: u8,
    offset_x: f32,
    offset_y: f32,
}

fn fractured_parts(
    rank: u8,
    part: usize,
    ty: u8,
    full_width: f32,
    height: f32,
) -> [Option<FracturedPart>; 2] {
    let horizontal = if ty == b':' { 0.0 } else { 1.0 };
    let vertical = if ty == b':' { 1.0 } else { 0.0 };
    [
        (part, -1.0),
        (usize::from(rank).wrapping_sub(part).wrapping_sub(1), 1.0),
    ]
    .map(|(count, sign)| {
        if count == 0 {
            return None;
        }
        let count = count as u8;
        Some(FracturedPart {
            rank: count,
            offset_x: sign * (full_width - width(count)) * 0.5 * horizontal,
            offset_y: sign * (height + (-1.0)) * 0.5 * vertical,
        })
    })
}

/// 原14127bab0：先修改首个残留组，后续组分配瞬时ID，最后创建被拖出的原子件。
/// 历史、归档和事件仍走原共享操作入口；本函数不另写变换缓存或结束主交互。
#[allow(clippy::too_many_arguments)]
pub fn handle_moving_out_roof_decorator_part(
    part: usize,
    storage: &mut DecoratorStorage,
    address: DecoratorStorageAddr,
    id: DecoratorId,
    transform: &CachedDecoratorTransform,
    derived: DerivedDecoratorInfoView<'_>,
    ids: &mut TransientDecoratorIdGen<'_>,
    archivist: &mut LiveDecoratorArchivist,
    mut history: Option<&mut DeferredDecoratorEditHistory>,
    events: &DeferredDecoratorOpEvents,
    archival: &DecoratorArchivalResources<'_>,
    state: u8,
    roof: &Roof,
    demo: bool,
) -> RoofSplitResult {
    let info = *storage
        .get(address)
        .and_then(|bucket| bucket.get(id))
        .expect("Decorator doesn't exist");
    let base = *info.dst_as_roof().destination().as_roof();
    let (full_width, height) = match derived.subtype(id).0 {
        2 => (width(info.rank), 1.0),
        8 => {
            let bounds = derived
                .mesh_library()
                .collision_record(DecoratorLibKey::new(8, info.rank))
                .bounds;
            (bounds.footprint.size.x, bounds.top - bounds.bottom)
        }
        _ => unreachable!("Decorators that cannot be split must not reach roof split"),
    };
    let pieces = fractured_parts(info.rank, part, info.ty, full_width, height);
    assert_ne!(info.rank, 0, "called Option::unwrap() on a None value");
    let mut remaining_ids = Vec::new();
    for piece in pieces.into_iter().flatten() {
        // 原FracturedPart保留两个方向的偏移，但roof闭包只消费X偏移，不改变V。
        let _vertical_offset = piece.offset_y;
        let destination =
            shifted_roof_destination(roof, transform.translation, base, piece.offset_x);
        if remaining_ids.is_empty() {
            remaining_ids.push(id);
            let patch = DecoratorChangePatch {
                id,
                destination: Some((address, destination)),
                rank: Some(piece.rank),
                ty: None,
                word_2c_delta: None, seed_demo_mode: false,
                extra_low: None,
                extra_high: None,
                extra_byte: None,
                event_position: Vec3::ZERO,
                event_position_flag: 2,
            };
            let op = DecoratorOp::from_change(address, patch);
            archivist.archive_current_decorators_at_with_resources(
                address,
                storage,
                archival,
                "move preview src",
            );
            apply_decorator_op(
                &op,
                state,
                address,
                storage,
                Some(archivist),
                history.as_deref_mut(),
                events,
            );
        } else {
            let new_id = DecoratorId(ids.generate());
            remaining_ids.push(new_id);
            let mut new_info = info;
            new_info.rank = piece.rank;
            new_info.set_destination(destination);
            // 原place的placement标志1与rank独立；demo由共享真实owner传入。
            let op = DecoratorOp::place_at(new_id, new_info, Vec3::ZERO, 2, true, demo);
            apply_place_with_resources(
                &op,
                state,
                address,
                storage,
                archivist,
                history.as_deref_mut(),
                archival,
                events,
            );
        }
    }
    let moving_id = DecoratorId(ids.generate());
    let horizontal = if info.ty == b':' { 0.0 } else { 1.0 };
    let extent = horizontal * ((full_width + (-f32::from_bits(0x3f4ccccd))) * 0.5 + 0.2);
    let ratio = part as f32 / (usize::from(info.rank).wrapping_sub(1) as f32);
    let distance = ((ratio + ratio) + (-1.0)) * extent;
    let destination = shifted_roof_destination(roof, transform.translation, base, distance);
    let mut new_info = info;
    new_info.rank = 1;
    new_info.set_destination(destination);
    let op = DecoratorOp::place_at(moving_id, new_info, Vec3::ZERO, 2, true, demo);
    apply_place_with_resources(
        &op, state, address, storage, archivist, history, archival, events,
    );
    RoofSplitResult {
        remaining_ids,
        moving_id,
        destination,
    }
}

#[cfg(test)]
mod native_tests {
    use super::*;
    use bevy_ecs::{prelude::*, system::SystemState, world::CommandQueue};
    use country_core::resources::walls::WallId;
    use country_core::{
        asset_libraries::{
            mesh_library::{AssetMeshLibrary, MeshAsset},
            shader_library::AssetShaderLibrary,
        },
        render::{mesh::Mesh, shader::RasterShaderProgram, test_device::TestDevice},
        resources::walls::{
            decorator::DecoratorIdGen,
            decorator_collision::DecoratorPhysicsColliders,
            decorator_subtype::{
                DecoratorCollision, DecoratorMeshLibrary, DerivedDecoratorSubtypes,
            },
        },
        systems::{
            collision::world::{RaycastMeshData, RaycastWorld},
            ui::signifiers::{SignifierAnimation, UiSignifierEntry},
        },
        utils::components::transform::Transform,
    };
    use utils::geometry::{Rectangle2d, Rectangle3d};
    fn nums(s: &str) -> Vec<u32> {
        if s.is_empty() {
            Vec::new()
        } else {
            s.split(',').map(|v| v.parse().unwrap()).collect()
        }
    }
    fn hex<const N: usize>(s: &str) -> [u8; N] {
        assert_eq!(s.len(), N * 2);
        std::array::from_fn(|i| u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).unwrap())
    }
    fn info(b: [u8; 72]) -> DecoratorInfo {
        let q = |o| u64::from_le_bytes(b[o..o + 8].try_into().unwrap());
        let d = |o| u32::from_le_bytes(b[o..o + 4].try_into().unwrap());
        DecoratorInfo {
            dst_wall: q(0),
            dst_secondary: q(8),
            dst_payload: q(16),
            u18: q(24),
            u20: d(32),
            u24: d(36),
            u28: d(40),
            decorator_id: d(44),
            u30: q(48),
            u38: d(56),
            u3c: d(60),
            u40: d(64),
            rank: b[68],
            u45: b[69],
            ty: b[70],
            state: b[71],
        }
    }
    fn pose(s: &str) -> CachedDecoratorTransform {
        CachedDecoratorTransform::from_components(
            &nums(s)
                .into_iter()
                .map(f32::from_bits)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }
    fn coordinates(destination: DecoratorDst) -> [u32; 3] {
        let v = destination.as_roof();
        [v.coord.x.to_bits(), v.coord.y.to_bits(), v.yaw.0.to_bits()]
    }
    fn catalog(
        subtype: u8,
        rank: u8,
        bounds: &[u32],
    ) -> (DerivedDecoratorSubtypes, DecoratorMeshLibrary) {
        let mut subtypes = DerivedDecoratorSubtypes::default();
        subtypes.insert(41, DecoratorSubtype(subtype));
        let mut lib = DecoratorMeshLibrary::default();
        lib.collision.insert(
            DecoratorLibKey::new(8, rank),
            DecoratorCollision {
                triangles: Vec::new(),
                mesh: Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::ZERO)),
                bounds: Rectangle3d {
                    footprint: Rectangle2d {
                        axis: Vec2::X,
                        center: Vec2::ZERO,
                        size: Vec2::new(f32::from_bits(bounds[0]), f32::from_bits(bounds[1])),
                    },
                    bottom: f32::from_bits(bounds[2]),
                    top: f32::from_bits(bounds[3]),
                },
            },
        );
        (subtypes, lib)
    }

    // 仅隔离已留首次严格失败的x86/ARM canonical NaN符号边界；所有其它字仍逐bit校验。
    fn exact_words(actual: &[u32], expected: &[u32], label: &str) -> usize {
        assert_eq!(actual.len(), expected.len());
        let mut signs = 0;
        for (&a, &b) in actual.iter().zip(expected) {
            if a ^ b == 0x8000_0000 && a & 0x7fff_ffff == 0x7fc0_0000 {
                signs += 1;
            } else {
                assert_eq!(a, b, "{label}");
            }
        }
        signs
    }

    #[test]
    fn original_roof_split_finite_bits_nan_sign_boundary_and_real_rollback() {
        let rows = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m01-roof-split-native.tsv"
        ));
        let mut success = 0;
        let mut panics = 0;
        let mut nan_signs = 0;
        for (index, line) in rows.lines().filter(|s| !s.starts_with('#')).enumerate() {
            let c: Vec<_> = line.split('\t').collect();
            let original = info(hex(c[0]));
            let roof = Roof::from_observed(hex(c[1]));
            let part = c[2].parse().unwrap();
            let subtype = c[3].parse().unwrap();
            let pose = pose(c[4]);
            let bounds = nums(c[5]);
            let (subtypes, library) = catalog(subtype, original.rank, &bounds);
            let derived = DerivedDecoratorInfoView::new(&subtypes, &library);
            let address = DecoratorStorageAddr::Wall(WallId(7));
            let mut storage = DecoratorStorage::default();
            storage.apply_decorator_place(address, DecoratorId(41), &original);
            let mut world = World::new();
            world.insert_resource(storage);
            world.insert_resource(LiveDecoratorArchivist::default());
            world.insert_resource(RaycastWorld::new());
            world.insert_resource(DecoratorPhysicsColliders::default());
            world.insert_resource(DeferredDecoratorEditHistory::default());
            let events = DeferredDecoratorOpEvents::default();
            let mut ids = DecoratorIdGen::default();
            ids.reserve(DecoratorId(99));
            let mut params = SystemState::<(
                ResMut<DecoratorStorage>,
                ResMut<LiveDecoratorArchivist>,
                DecoratorArchivalResources,
                ResMut<DeferredDecoratorEditHistory>,
            )>::new(&mut world);
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let (mut storage, mut archivist, archival, mut history) =
                    params.get_mut(&mut world);
                let mut transient = ids.transient(false);
                handle_moving_out_roof_decorator_part(
                    part,
                    &mut storage,
                    address,
                    DecoratorId(41),
                    &pose,
                    derived,
                    &mut transient,
                    &mut archivist,
                    Some(&mut history),
                    &events,
                    &archival,
                    1,
                    &roof,
                    true,
                )
            }));
            if c[6] == "panic" {
                assert!(result.is_err(), "native case {index} must fail");
                panics += 1;
                continue;
            }
            let result = result.unwrap_or_else(|_| panic!("native case {index} should succeed"));
            assert_eq!(
                result.remaining_ids.iter().map(|x| x.0).collect::<Vec<_>>(),
                nums(c[7]),
                "native ids {index}"
            );
            assert_eq!(
                result.moving_id.0,
                c[8].parse::<u32>().unwrap(),
                "native new id {index}"
            );
            nan_signs += exact_words(
                &coordinates(result.destination),
                &nums(c[9]),
                &format!("native destination {index}"),
            );
            let ops: Vec<_> = c[10].split(';').map(nums).collect();
            let mut output = Vec::new();
            events.unleash(&mut output);
            assert_eq!(output.len(), ops.len());
            for (event, op) in output.iter().zip(&ops) {
                let id = DecoratorId(op[1]);
                let value = world
                    .resource::<DecoratorStorage>()
                    .decorator_info(id)
                    .unwrap();
                assert_eq!(
                    value.rank,
                    u8::try_from(op[2]).unwrap(),
                    "native rank {index}"
                );
                nan_signs += exact_words(
                    &coordinates(value.destination()),
                    &op[3..6],
                    &format!("native stored destination {index}"),
                );
                assert_eq!(value.ty, original.ty);
                assert_eq!(value.u28, original.u28);
                assert_eq!(value.u30, original.u30);
                assert_eq!(value.u38, original.u38);
                assert_eq!(value.u3c, original.u3c);
                assert_eq!(value.u40, original.u40);
                assert_eq!(event[0x7c], op[6] as u8, "native position tag {index}");
                assert_eq!(&event[0x7d..0x80], &[0; 3]);
            }
            assert_eq!(
                world
                    .resource::<DeferredDecoratorEditHistory>()
                    .records
                    .len(),
                ops.len()
            );
            assert_eq!(ids.peek(), DecoratorId(100), "预览不能消耗实际ID基数");
            {
                let (mut storage, mut archivist, _, _) = params.get_mut(&mut world);
                archivist.extract_and_restore_to_storage(&mut storage);
                assert_eq!(storage.get(address).unwrap().decorators.len(), 1);
                assert_eq!(*storage.decorator_info(DecoratorId(41)).unwrap(), original);
                assert!(storage.decorator_info(result.moving_id).is_none());
            }
            success += 1;
        }
        eprintln!("roof split exact finite words; canonical NaN sign differences={nan_signs}");
        assert_eq!((success, panics), (128, 136));
        assert_eq!(nan_signs, 32);
    }

    #[test]
    fn original_roof_picker_finite_bits_nan_sign_boundary_and_real_factories() {
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
            "/../../../reports/module-recovery-2026-10-23/m01-roof-split-display-native.tsv"
        ));
        let mut success = 0;
        let mut panics = 0;
        let mut nan_signs = 0;
        for (index, line) in rows.lines().filter(|s| !s.starts_with('#')).enumerate() {
            let c: Vec<_> = line.split('\t').collect();
            let info = info(hex(c[0]));
            let roof = Roof::from_observed(hex(c[1]));
            let pose = pose(c[2]);
            let p = nums(c[3]);
            let hit = RaycastWorldResult {
                handle: [2, 7, 0],
                position: Vec3::from_array(std::array::from_fn(|i| f32::from_bits(p[i]))),
                normal: Vec3::Y,
                toi: 0.0,
                discriminant: 1,
                extra: [0.0; 2],
            };
            let subtype = DecoratorSubtype(c[4].parse().unwrap());
            let mut volume = RoofTriggerVolume::default();
            volume.set_triangles(vec![[
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(4.0, 1.0, 0.0),
                Vec3::new(0.0, 1.0, 4.0),
            ]]);
            volume.set_uvs(vec![[Vec2::ZERO, Vec2::X, Vec2::Y]]);
            let mut world = World::new();
            let entity = world.spawn((roof, volume)).id();
            let mut walls = PublicWalls::default();
            walls.associate_roof(WallId(7), entity);
            let mut params =
                SystemState::<(Query<&Roof>, Query<&RoofTriggerVolume>)>::new(&mut world);
            let (roofs, volumes) = params.get(&world);
            let mut stream = UiSignifierStream::default();
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                try_display_roof_decorator_parts_and_get_hovered_part_id(
                    &hit,
                    &info,
                    &pose,
                    DecoratorId(41),
                    subtype,
                    DecoratorStorageAddr::Wall(WallId(7)),
                    &mut stream,
                    &walls,
                    &roofs,
                    &volumes,
                )
            }));
            if c[5] == "panic" {
                assert!(result.is_err(), "native picker {index} must fail");
                panics += 1;
                continue;
            }
            let selected =
                result.unwrap_or_else(|_| panic!("native picker {index} should succeed"));
            let expected = c[6].parse::<isize>().unwrap();
            assert_eq!(
                selected,
                if expected < 0 {
                    None
                } else {
                    Some(expected as usize)
                },
                "native picker {index}"
            );
            drop(roofs);
            drop(volumes);
            let entries = stream.take_entries();
            let expected: Vec<_> = c[7].split(';').collect();
            assert_eq!(entries.len(), expected.len(), "native factories {index}");
            let mut queue = CommandQueue::default();
            for (entry, expected) in entries.into_iter().zip(expected) {
                let UiSignifierEntry::Factory(factory) = entry else {
                    panic!("必须是真实Factory")
                };
                let (position, marker) = expected.split_once('|').unwrap();
                let bits = nums(position);
                let entities =
                    factory.wake(&meshes, &shaders, &mut Commands::new(&mut queue, &world));
                queue.apply(&mut world);
                assert_eq!(entities.len(), 1);
                let entity = entities[0];
                nan_signs += exact_words(
                    &world
                        .get::<Transform>(entity)
                        .unwrap()
                        .translation
                        .to_array()
                        .map(f32::to_bits),
                    &bits,
                    &format!("native marker position {index}"),
                );
                let scale: f32 = match marker {
                    "0x142c23df8" => 0.05,
                    "0x142c23e28" => 0.2,
                    "0x142c23d60" => 0.35,
                    _ => panic!("unknown original closure"),
                };
                let animation = world.get::<SignifierAnimation>(entity).unwrap();
                assert_eq!(animation.value.to_bits(), scale.to_bits());
                assert_eq!(animation.duration.to_bits(), 0.1_f32.to_bits());
                assert!(factory.name().unwrap().starts_with("outline_split_"));
                world.despawn(entity);
            }
            success += 1;
        }
        eprintln!("roof picker exact finite words; canonical NaN sign differences={nan_signs}");
        assert_eq!((success, panics), (384, 96));
        assert_eq!(nan_signs, 288);
    }
    #[test]
    fn actual_world_preview_reselect_and_cancel_restore_all_original_owners() {
        use crate::{
            backup::{DecoratorBackup, create_or_restore_decorator_backup},
            blueprint::DecoratorBlueprints,
        };
        use bevy_ecs::system::RunSystemOnce;
        use country_core::resources::{
            stairs::stairs_state::StairsState,
            walls::{
                cached_decorator_transforms::CachedDecoratorTransforms,
                decorator_archivist::LiveColliderPayload,
                decorator_subtype::DerivedDecoratorInfoResources,
            },
        };
        let line = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m01-roof-split-native.tsv"
        ))
        .lines()
        .filter(|line| !line.starts_with('#'))
        .find(|line| {
            let c: Vec<_> = line.split('\t').collect();
            let b = hex::<72>(c[0]);
            c[2] == "1" && c[3] == "2" && b[68] == 3 && b[70] == 0x38
        })
        .unwrap();
        let c: Vec<_> = line.split('\t').collect();
        let original = info(hex(c[0]));
        let roof = Roof::from_observed(hex(c[1]));
        let transform = pose(c[4]);
        let address = DecoratorStorageAddr::Wall(WallId(7));
        let mut storage = DecoratorStorage::default();
        storage.apply_decorator_place(address, DecoratorId(41), &original);
        let (subtypes, library) = catalog(2, 3, &nums(c[5]));
        let mut cached = CachedDecoratorTransforms::default();
        cached.insert(41, transform);
        let mut world = World::new();
        world.insert_resource(storage);
        world.insert_resource(LiveDecoratorArchivist::default());
        world.insert_resource(RaycastWorld::new());
        world.insert_resource(DecoratorPhysicsColliders::default());
        world.insert_resource(DeferredDecoratorEditHistory::default());
        world.insert_resource(DeferredDecoratorOpEvents::default());
        world.insert_resource(subtypes);
        world.insert_resource(library);
        world.insert_resource(cached);
        world.insert_resource(DecoratorBlueprints::default());
        world.insert_resource(DecoratorBackup::default());
        world.insert_resource(StairsState::default());
        let mut params = SystemState::<(
            ResMut<DecoratorStorage>,
            ResMut<LiveDecoratorArchivist>,
            DecoratorArchivalResources,
            ResMut<DeferredDecoratorEditHistory>,
            Res<DeferredDecoratorOpEvents>,
            DerivedDecoratorInfoResources,
        )>::new(&mut world);
        {
            let (storage, mut archivist, archival, _, _, _) = params.get_mut(&mut world);
            archivist.archive_current_decorators_at_with_resources(
                address,
                &storage,
                &archival,
                "move preview src",
            );
        }
        world.resource_mut::<DecoratorBackup>().state = 1;
        world
            .run_system_once(create_or_restore_decorator_backup::<LiveColliderPayload>)
            .unwrap();
        let mut ids = DecoratorIdGen::default();
        ids.reserve(DecoratorId(99));
        let first = {
            let (mut storage, mut archivist, archival, mut history, events, derived) =
                params.get_mut(&mut world);
            let mut transient = ids.transient(false);
            handle_moving_out_roof_decorator_part(
                1,
                &mut storage,
                address,
                DecoratorId(41),
                &transform,
                derived.as_view(),
                &mut transient,
                &mut archivist,
                Some(&mut history),
                &events,
                &archival,
                1,
                &roof,
                false,
            )
        };
        assert_eq!(first.remaining_ids, vec![DecoratorId(41), DecoratorId(100)]);
        assert_eq!(first.moving_id, DecoratorId(101));
        assert_eq!(
            world
                .resource::<DecoratorStorage>()
                .get(address)
                .unwrap()
                .decorators
                .len(),
            3
        );
        assert_eq!(
            world
                .resource::<DeferredDecoratorEditHistory>()
                .records
                .len(),
            3
        );
        world.resource_mut::<DecoratorBackup>().state = 2;
        world
            .run_system_once(create_or_restore_decorator_backup::<LiveColliderPayload>)
            .unwrap();
        assert_eq!(
            *world
                .resource::<DecoratorStorage>()
                .decorator_info(DecoratorId(41))
                .unwrap(),
            original
        );
        assert!(
            world
                .resource::<DecoratorStorage>()
                .decorator_info(DecoratorId(100))
                .is_none()
        );
        assert!(
            world
                .resource::<DecoratorStorage>()
                .decorator_info(DecoratorId(101))
                .is_none()
        );
        assert_eq!(
            world.resource::<CachedDecoratorTransforms>().transform(41),
            transform
        );
        assert!(
            world
                .resource::<DeferredDecoratorEditHistory>()
                .records
                .is_empty()
        );
        let mut events = Vec::new();
        world
            .resource::<DeferredDecoratorOpEvents>()
            .unleash(&mut events);
        assert!(events.is_empty());
        let second = {
            let (mut storage, mut archivist, archival, mut history, events, derived) =
                params.get_mut(&mut world);
            let mut transient = ids.transient(false);
            handle_moving_out_roof_decorator_part(
                0,
                &mut storage,
                address,
                DecoratorId(41),
                &transform,
                derived.as_view(),
                &mut transient,
                &mut archivist,
                Some(&mut history),
                &events,
                &archival,
                1,
                &roof,
                false,
            )
        };
        assert_eq!(second.remaining_ids, vec![DecoratorId(41)]);
        assert_eq!(second.moving_id, DecoratorId(100));
        assert_eq!(
            world
                .resource::<DecoratorStorage>()
                .decorator_info(DecoratorId(41))
                .unwrap()
                .rank,
            2
        );
        world.resource_mut::<DecoratorBackup>().state = 2;
        world
            .resource_mut::<DecoratorBackup>()
            .consume_after_processing();
        world
            .run_system_once(create_or_restore_decorator_backup::<LiveColliderPayload>)
            .unwrap();
        assert_eq!(
            *world
                .resource::<DecoratorStorage>()
                .decorator_info(DecoratorId(41))
                .unwrap(),
            original
        );
        assert_eq!(
            world
                .resource::<DecoratorStorage>()
                .get(address)
                .unwrap()
                .decorators
                .len(),
            1
        );
        assert!(!world.resource::<DecoratorBackup>().created);
        assert!(
            world
                .resource::<DeferredDecoratorEditHistory>()
                .records
                .is_empty()
        );
        assert_eq!(ids.peek(), DecoratorId(100));
    }

    #[test]
    fn actual_query_identity_missing_components_and_wrong_address_keep_original_gates() {
        let mut world = World::new();
        let roof = Roof::from_observed([0; 88]);
        let mut volume = RoofTriggerVolume::default();
        volume.set_triangles(vec![[Vec3::ZERO, Vec3::X, Vec3::Z]]);
        volume.set_uvs(vec![[Vec2::ZERO, Vec2::X, Vec2::Y]]);
        let missing_volume = world.spawn(roof.clone()).id();
        let missing_roof = world.spawn(volume.clone()).id();
        let dead = world.spawn((roof.clone(), volume.clone())).id();
        world.despawn(dead);
        let new = world.spawn((roof, volume)).id();
        assert_eq!(dead.index(), new.index());
        assert_ne!(dead, new);
        let mut info = DecoratorInfo {
            rank: 2,
            ty: 0x38,
            ..Default::default()
        };
        info.set_destination(DecoratorDst::Roof(RoofDecoratorDst {
            coord: Vec2::splat(0.2),
            yaw: DecoratorRotationXZ(0.0),
        }));
        let transform = CachedDecoratorTransform {
            rotation: Quat::IDENTITY,
            translation: Vec3::ZERO,
            scale: Vec3::ONE,
        };
        let hit = RaycastWorldResult::miss();
        let mut params = SystemState::<(Query<&Roof>, Query<&RoofTriggerVolume>)>::new(&mut world);
        let (roofs, volumes) = params.get(&world);
        let mut walls = PublicWalls::default();
        for entity in [None, Some(missing_volume), Some(missing_roof), Some(dead)] {
            if let Some(entity) = entity {
                walls.associate_roof(WallId(7), entity);
            } else {
                walls.remove_roof(WallId(7));
            }
            let mut stream = UiSignifierStream::default();
            assert_eq!(
                try_display_roof_decorator_parts_and_get_hovered_part_id(
                    &hit,
                    &info,
                    &transform,
                    DecoratorId(41),
                    DecoratorSubtype(2),
                    DecoratorStorageAddr::Wall(WallId(7)),
                    &mut stream,
                    &walls,
                    &roofs,
                    &volumes
                ),
                None
            );
            assert!(stream.take_entries().is_empty());
        }
        let mut stream = UiSignifierStream::default();
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                try_display_roof_decorator_parts_and_get_hovered_part_id(
                    &hit,
                    &info,
                    &transform,
                    DecoratorId(41),
                    DecoratorSubtype(2),
                    DecoratorStorageAddr::Terrain,
                    &mut stream,
                    &walls,
                    &roofs,
                    &volumes,
                )
            }))
            .is_err()
        );
        // 原rank0不进入部件循环，不能把dst_as_roof验证提前到循环外。
        walls.associate_roof(WallId(7), new);
        let mut zero = info;
        zero.rank = 0;
        zero.set_destination(DecoratorDst::Terrain(
            country_core::resources::walls::decorator::TerrainDecoratorDst {
                position_xz: Vec2::ZERO,
                yaw: DecoratorRotationXZ(0.0),
            },
        ));
        let mut stream = UiSignifierStream::default();
        assert_eq!(
            try_display_roof_decorator_parts_and_get_hovered_part_id(
                &hit,
                &zero,
                &transform,
                DecoratorId(41),
                DecoratorSubtype(2),
                DecoratorStorageAddr::Wall(WallId(7)),
                &mut stream,
                &walls,
                &roofs,
                &volumes,
            ),
            Some(0)
        );
        assert_eq!(stream.take_entries().len(), 1);
    }
    #[test]
    fn required_storage_and_derived_assets_fail_before_ids_history_or_archival() {
        let address = DecoratorStorageAddr::Wall(WallId(7));
        for missing in 0..3 {
            let mut original = DecoratorInfo {
                rank: 3,
                ty: 0x38,
                ..Default::default()
            };
            original.set_destination(DecoratorDst::Roof(RoofDecoratorDst {
                coord: Vec2::new(0.25, 0.6),
                yaw: DecoratorRotationXZ(0.7),
            }));
            let mut storage = DecoratorStorage::default();
            if missing != 0 {
                storage.apply_decorator_place(address, DecoratorId(41), &original);
            }
            let (mut subtypes, mut library) = catalog(
                if missing == 2 { 8 } else { 2 },
                3,
                &[
                    3.1_f32.to_bits(),
                    0,
                    (-0.2_f32).to_bits(),
                    2.3_f32.to_bits(),
                ],
            );
            if missing == 1 {
                subtypes.clear();
            }
            if missing == 2 {
                library.collision.clear();
            }
            let derived = DerivedDecoratorInfoView::new(&subtypes, &library);
            let mut world = World::new();
            world.insert_resource(storage);
            world.insert_resource(LiveDecoratorArchivist::default());
            world.insert_resource(RaycastWorld::new());
            world.insert_resource(DecoratorPhysicsColliders::default());
            world.insert_resource(DeferredDecoratorEditHistory::default());
            let events = DeferredDecoratorOpEvents::default();
            let mut ids = DecoratorIdGen::default();
            ids.reserve(DecoratorId(99));
            let mut params = SystemState::<(
                ResMut<DecoratorStorage>,
                ResMut<LiveDecoratorArchivist>,
                DecoratorArchivalResources,
                ResMut<DeferredDecoratorEditHistory>,
            )>::new(&mut world);
            let transform = CachedDecoratorTransform {
                rotation: Quat::IDENTITY,
                translation: Vec3::Y,
                scale: Vec3::ONE,
            };
            let roof = Roof::from_observed([0; 88]);
            assert!(
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let (mut storage, mut archivist, archival, mut history) =
                        params.get_mut(&mut world);
                    let mut transient = ids.transient(false);
                    handle_moving_out_roof_decorator_part(
                        1,
                        &mut storage,
                        address,
                        DecoratorId(41),
                        &transform,
                        derived,
                        &mut transient,
                        &mut archivist,
                        Some(&mut history),
                        &events,
                        &archival,
                        1,
                        &roof,
                        false,
                    )
                }))
                .is_err()
            );
            assert_eq!(ids.peek(), DecoratorId(100));
            assert!(
                world
                    .resource::<DeferredDecoratorEditHistory>()
                    .records
                    .is_empty()
            );
            assert_eq!(
                world
                    .resource::<LiveDecoratorArchivist>()
                    .all_storages()
                    .count(),
                0
            );
            let mut emitted = Vec::new();
            events.unleash(&mut emitted);
            assert!(emitted.is_empty());
            assert_eq!(
                world
                    .resource::<DecoratorStorage>()
                    .decorator_info(DecoratorId(41))
                    .copied(),
                if missing == 0 { None } else { Some(original) }
            );
        }
    }
}
