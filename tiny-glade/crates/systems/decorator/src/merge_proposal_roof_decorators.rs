//! B屋顶装饰合并：Intent专用碰撞141fe7280/141fe6f20，提议142000e60/141fe6a40。
//! 借用真实storage/derived/cached/Roof；候选由原碰撞链产生，不以两个外部盒替代。

use country_core::resources::{
    roofs::{geometry::surface::RoofSection, roof_shape::Roof},
    walls::{
        cached_decorator_transforms::{CachedDecoratorTransform, CachedDecoratorTransforms},
        decorator::{DecoratorId, DecoratorInfo, DecoratorRotationXZ},
        decorator_storage::Decorators,
        decorator_subtype::{DecoratorLibKey, DerivedDecoratorInfoView},
    },
};
use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};
use glam::{Quat, Vec2, Vec3};
use utils::geometry::{Circle2d, Rectangle2d, Rectangle3d};

/// 原112B Option载荷的完整已定义业务字段；None的原tag4由Rust Option表达。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RoofDecoratorMergeProposal {
    pub rotation: DecoratorRotationXZ,
    pub coord: Vec2,
    /// 原incoming完整72B，仅rank替换为合并等级；目的地由唯一Intent消费者写回。
    pub info: DecoratorInfo,
    /// 独立map key，不能由Info+2c未知业务word替代。
    pub target_id: DecoratorId,
    pub distance: f32,
}

/// 原offset<8保留（含7），否则6；不是min(6)。
#[inline]
pub fn decorator_family(ty: u8) -> u8 {
    let offset = ty.wrapping_sub(0x38);
    if offset < 8 { offset } else { 6 }
}
#[inline]
fn families_compatible(new_ty: u8, old_ty: u8) -> bool {
    let family = decorator_family(new_ty);
    family == decorator_family(old_ty) && (family != 6 || new_ty == old_ty)
}

fn projected_x_axis(rotation: Quat) -> Vec2 {
    let direction = rotation * Vec3::X;
    let reciprocal = 1.0 / (direction.z * direction.z + direction.x * direction.x).sqrt();
    Vec2::new(direction.x * reciprocal, direction.z * reciprocal)
}

/// 原141fe6f20，保留排除/目的变体/族门与三个required读取的顺序。
#[allow(clippy::too_many_arguments)]
pub fn roof_decorator_intersects(
    info: &DecoratorInfo,
    query: Rectangle3d,
    target_id: DecoratorId,
    target: &DecoratorInfo,
    derived: DerivedDecoratorInfoView<'_>,
    cached: &CachedDecoratorTransforms,
    excluded_id: DecoratorId,
) -> bool {
    if target_id == excluded_id {
        return false;
    }
    match target.dst_tag() {
        2 | 4 => {}
        3 => panic!("internal error: entered unreachable code"),
        _ => return false,
    }
    if !families_compatible(info.ty, target.ty) {
        return false;
    }
    let transform = cached.transform(target_id.0);
    let subtype = derived.subtype(target_id);
    let collision = derived
        .mesh_library()
        .collision_record(DecoratorLibKey::new(subtype.0, target.rank()));
    query.overlaps_rectangle3d(roof_collision_bounds(transform, collision.bounds))
}

fn roof_collision_bounds(transform: CachedDecoratorTransform, bounds: Rectangle3d) -> Rectangle3d {
    let height = bounds.top - bounds.bottom;
    // 原世界Y半高与旋转后Z偏置各自计算；不把二者合并成局部向量旋转。
    let center = Vec3::new(height * 0.0 * 0.5, height * 0.5, height * 0.0 * 0.5)
        + transform.translation
        - transform.rotation * Vec3::new(0.0, 0.0, f32::from_bits(0x3dca_c084));
    let dimensions = Vec3::new(bounds.footprint.size.x + 0.4, height + 0.0, 0.3);
    Rectangle3d::new(center, dimensions, projected_x_axis(transform.rotation))
}

/// 原141fe7280：真实查询盒→桶内独立ID排除/族/缓存碰撞→有序命中数组。
#[allow(clippy::too_many_arguments)]
pub fn intersect_roof_decorators(
    info: &DecoratorInfo,
    position: Vec3,
    rotation: Quat,
    width: f32,
    height: f32,
    decorators: &Decorators,
    derived: DerivedDecoratorInfoView<'_>,
    cached: &CachedDecoratorTransforms,
    excluded_id: DecoratorId,
) -> Vec<u32> {
    let query = Rectangle3d::new(
        position,
        Vec3::new(width, height, 0.1),
        projected_x_axis(rotation),
    );
    let mut hits = Vec::new();
    for (&id, target) in &decorators.decorators {
        if roof_decorator_intersects(
            info,
            query,
            DecoratorId(id),
            target,
            derived,
            cached,
            excluded_id,
        ) {
            hits.push(id);
        }
    }
    hits
}

pub(crate) fn section_shape(roof: &Roof, position: Vec3) -> ShapeParameters {
    ShapeParameters::from_shape(match roof.get_roof_shape_from_pos_ws_clamped(position) {
        RoofSection::Circle {
            frame,
            radius,
            word_0x18,
        } => Shape::Circle(Circle2d {
            center: Vec2::from_array(frame),
            radius,
            rotation: f32::from_bits(word_0x18),
        }),
        RoofSection::Rectangle { frame, dimensions } => Shape::Rectangle(Rectangle2d {
            axis: Vec2::new(frame[0], frame[1]),
            center: Vec2::new(frame[2], frame[3]),
            size: Vec2::from_array(dimensions),
        }),
    })
}

/// 原141fe6a40和主迭代内联同一提议算法；不存在的候选required失败，不跳过。
#[allow(clippy::too_many_arguments)]
fn propose(
    info: &DecoratorInfo,
    new_coord: Vec2,
    position: Vec3,
    roof: &Roof,
    decorators: &Decorators,
    derived: DerivedDecoratorInfoView<'_>,
    target_id: DecoratorId,
) -> Option<RoofDecoratorMergeProposal> {
    let target = decorators.get(target_id).expect("Decorator doesn't exist");
    // 原先查目标碰撞再筛同族；缺资源不可被族门吞掉。
    let subtype = derived.subtype(target_id);
    let width = derived
        .mesh_library()
        .collision_record(DecoratorLibKey::new(subtype.0, target.rank()))
        .bounds
        .footprint
        .size
        .x;
    if !families_compatible(info.ty, target.ty) {
        return None;
    }
    let rank_sum = u16::from(info.rank()) + u16::from(target.rank());
    if rank_sum > u16::from(info.decorator_type().max_rank()) {
        return None;
    }
    let mut merged = *info;
    merged.rank = rank_sum as u8;
    let merged_width = derived
        .mesh_library()
        .collision_record(DecoratorLibKey::new(subtype.0, merged.rank()))
        .bounds
        .footprint
        .size
        .x;
    let mut delta = (merged_width - width) * 0.5;
    let shape = section_shape(roof, position);
    if roof.observed()[8] & 1 != 0 {
        let rectangle = shape.as_rectangle();
        let edge = rectangle.get_closest_edge_id(Vec2::new(position.x, position.z));
        delta = (delta / rectangle.edge_length(edge)) * 0.25;
    } else {
        delta /= shape.perimeter();
    }
    let base = target.dst_as_roof();
    let coord = base.wall_space_coord();
    let proposed = Vec2::new(
        (if coord.u < new_coord.x { delta } else { -delta }) + coord.u,
        coord.y,
    );
    let dx = proposed.x - new_coord.x;
    let dy = coord.y - new_coord.y;
    Some(RoofDecoratorMergeProposal {
        rotation: DecoratorRotationXZ(f32::from_bits(base.dst_secondary as u32)),
        coord: proposed,
        info: merged,
        target_id,
        distance: (dy * dy + dx * dx).sqrt(),
    })
}

/// 原142000e60：按原命中次序选最近，首候选NaN原样保留；仅实际比较NaN时panic。
#[allow(clippy::too_many_arguments)]
pub fn find_best_roof_decorator_merge_position(
    info: &DecoratorInfo,
    new_coord: Vec2,
    position: Vec3,
    roof: &Roof,
    decorators: &Decorators,
    derived: DerivedDecoratorInfoView<'_>,
    intersections: &[u32],
) -> Option<RoofDecoratorMergeProposal> {
    let mut best: Option<RoofDecoratorMergeProposal> = None;
    for &id in intersections {
        if let Some(next) = propose(
            info,
            new_coord,
            position,
            roof,
            decorators,
            derived,
            DecoratorId(id),
        ) {
            best = Some(match best {
                None => next,
                Some(previous) => match next
                    .distance
                    .partial_cmp(&previous.distance)
                    .expect("called Option::unwrap() on a None value")
                {
                    std::cmp::Ordering::Less => next,
                    _ => previous,
                },
            });
        }
    }
    best
}

#[cfg(test)]
mod native_business_chain_tests {
    use super::*;
    use country_core::{
        resources::walls::{
            WallId,
            cached_decorator_transforms::CachedDecoratorTransform,
            decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
            decorator_subtype::{
                DecoratorCollision, DecoratorMeshLibrary, DecoratorSubtype,
                DerivedDecoratorSubtypes,
            },
        },
        systems::collision::world::RaycastMeshData,
    };
    use std::sync::Arc;
    fn decode(b: &[u8; 72]) -> DecoratorInfo {
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
    fn encode(info: &DecoratorInfo) -> [u8; 72] {
        let mut b = [0; 72];
        for (o, q) in [
            (0, info.dst_wall),
            (8, info.dst_secondary),
            (16, info.dst_payload),
            (24, info.u18),
            (48, info.u30),
        ] {
            b[o..o + 8].copy_from_slice(&q.to_le_bytes());
        }
        for (o, d) in [
            (32, info.u20),
            (36, info.u24),
            (40, info.u28),
            (44, info.decorator_id),
            (56, info.u38),
            (60, info.u3c),
            (64, info.u40),
        ] {
            b[o..o + 4].copy_from_slice(&d.to_le_bytes());
        }
        b[68..].copy_from_slice(&[info.rank, info.u45, info.ty, info.state]);
        b
    }
    fn mesh_library(missing: bool) -> DecoratorMeshLibrary {
        let mut mesh = DecoratorMeshLibrary::default();
        if !missing {
            let widths = [1060320051, 1069799178, 1075251773, 1078733046];
            for rank in 0..4 {
                let height = (1.8f64 + f64::from(rank) * 0.1) as f32;
                mesh.collision.insert(
                    DecoratorLibKey::new(8, rank),
                    DecoratorCollision {
                        triangles: Vec::new(),
                        mesh: Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::ZERO)),
                        bounds: Rectangle3d {
                            footprint: Rectangle2d {
                                axis: Vec2::X,
                                center: Vec2::ZERO,
                                size: Vec2::new(f32::from_bits(widths[rank as usize]), 0.),
                            },
                            bottom: -0.2,
                            top: height,
                        },
                    },
                );
            }
        }
        mesh
    }
    fn roof(rect: bool) -> Roof {
        let mut b = [0; 88];
        b[..8].copy_from_slice(&7u64.to_le_bytes());
        b[8..12].copy_from_slice(&u32::from(rect).to_le_bytes());
        let frame = if rect {
            [1f32, 0., 0., 0., 8., 6.]
        } else {
            [0., 0., 4., 0., 0., 0.]
        };
        for (i, v) in frame.into_iter().enumerate() {
            b[12 + i * 4..16 + i * 4].copy_from_slice(&v.to_le_bytes());
        }
        for (i, v) in [0f32, 0., 0.5, 0.6, 0.3, 0.2, 0.].into_iter().enumerate() {
            b[0x24 + i * 4..0x28 + i * 4].copy_from_slice(&v.to_le_bytes());
        }
        Roof::from_observed(b)
    }
    enum NativeResult {
        None,
        Panic,
        Some([u32; 3], [u8; 72], u32, u32),
    }
    #[test]
    fn roof_merge_complete_native_proposal_preserves_all_info_words_rank_direction_and_minimum() {
        type Case = (
            &'static [u8; 72],
            &'static [(u32, [u8; 72])],
            &'static [u32],
            [u32; 2],
            [u32; 3],
            bool,
            bool,
            NativeResult,
        );
        let cases: &[Case] = include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m01-roof-merge-native-fixtures.rs"
        ));
        let mut failures = Vec::new();
        for (i, (input, targets, ids, coord, position, rect, missing, expected)) in
            cases.iter().enumerate()
        {
            let mut bucket = Decorators::default();
            let mut subtypes = DerivedDecoratorSubtypes::default();
            for (id, info) in *targets {
                bucket.insert(DecoratorId(*id), decode(info));
                subtypes.insert(*id, DecoratorSubtype(8));
            }
            let mesh = mesh_library(*missing);
            let info = decode(input);
            let actual = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                find_best_roof_decorator_merge_position(
                    &info,
                    Vec2::from_array(coord.map(f32::from_bits)),
                    Vec3::from_array(position.map(f32::from_bits)),
                    &roof(*rect),
                    &bucket,
                    DerivedDecoratorInfoView::new(&subtypes, &mesh),
                    ids,
                )
            }));
            let valid = match (actual, expected) {
                (Err(_), NativeResult::Panic) | (Ok(None), NativeResult::None) => true,
                (Ok(Some(a)), NativeResult::Some(c, b, target, d)) => {
                    [
                        a.rotation.0.to_bits(),
                        a.coord.x.to_bits(),
                        a.coord.y.to_bits(),
                    ] == *c
                        && encode(&a.info) == *b
                        && a.target_id.0 == *target
                        && a.distance.to_bits() == *d
                }
                _ => false,
            };
            if !valid {
                failures.push(i);
            }
        }
        assert!(failures.is_empty(), "native cases {failures:?}");
    }
    #[test]
    fn roof_merge_complete_native_predicate_keeps_destination_exclusion_family_and_real_box() {
        type Case = (
            &'static [u8; 72],
            &'static [u8; 72],
            u32,
            [u32; 4],
            [u32; 3],
            [u32; 10],
            u32,
            u32,
            [u32; 8],
            Option<[u32; 8]>,
            Option<bool>,
        );
        let cases: &[Case] = include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m01-roof-predicate-native-fixtures-v2.rs"
        ));
        for (
            i,
            (
                input,
                target,
                exclude,
                quat,
                pos,
                transform,
                width,
                height,
                query,
                target_box,
                expected,
            ),
        ) in cases.iter().enumerate()
        {
            let position = Vec3::from_array(pos.map(f32::from_bits));
            let rotation = Quat::from_array(quat.map(f32::from_bits));
            let query_rect = Rectangle3d::new(
                position,
                Vec3::new(f32::from_bits(*width), f32::from_bits(*height), 0.1),
                projected_x_axis(rotation),
            );
            let r = query_rect;
            let actual_query = [
                r.footprint.axis.x,
                r.footprint.axis.y,
                r.footprint.center.x,
                r.footprint.center.y,
                r.footprint.size.x,
                r.footprint.size.y,
                r.bottom,
                r.top,
            ]
            .map(f32::to_bits);
            assert_eq!(actual_query, *query, "case {i} query");
            let mut subtypes = DerivedDecoratorSubtypes::default();
            subtypes.insert(9, DecoratorSubtype(8));
            let mesh = mesh_library(false);
            let mut cached = CachedDecoratorTransforms::default();
            cached.insert(
                9,
                CachedDecoratorTransform::from_components(&transform.map(f32::from_bits)),
            );
            if let Some(expected) = target_box {
                let collision =
                    mesh.collision_record(DecoratorLibKey::new(8, decode(target).rank()));
                let r = roof_collision_bounds(cached.transform(9), collision.bounds);
                assert_eq!(
                    [
                        r.footprint.axis.x,
                        r.footprint.axis.y,
                        r.footprint.center.x,
                        r.footprint.center.y,
                        r.footprint.size.x,
                        r.footprint.size.y,
                        r.bottom,
                        r.top
                    ]
                    .map(f32::to_bits),
                    *expected,
                    "case {i} target box"
                );
            }
            let actual = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                roof_decorator_intersects(
                    &decode(input),
                    query_rect,
                    DecoratorId(9),
                    &decode(target),
                    DerivedDecoratorInfoView::new(&subtypes, &mesh),
                    &cached,
                    DecoratorId(*exclude),
                )
            }));
            match (actual, expected) {
                (Ok(a), Some(b)) => assert_eq!(a, *b, "native {i}"),
                (Err(_), None) => {}
                _ => panic!("native {i} failure boundary"),
            }
        }
    }
    #[test]
    fn roof_merge_world_authority_hits_proposes_updates_cached_edit_and_removes_without_stale_candidates()
     {
        use bevy_ecs::world::World;
        let mut world = World::new();
        let address = DecoratorStorageAddr::Wall(WallId(7));
        let mut target = DecoratorInfo {
            rank: 1,
            ty: 0x38,
            state: 1,
            decorator_id: 0xaabbccdd,
            ..Default::default()
        };
        target.set_destination(
            country_core::resources::walls::decorator::DecoratorDst::Roof(
                country_core::resources::walls::decorator::RoofDecoratorDst {
                    coord: Vec2::new(0.25, 0.6),
                    yaw: DecoratorRotationXZ(0.7),
                },
            ),
        );
        let incoming = DecoratorInfo {
            decorator_id: 0x88776655,
            ..target
        };
        let mut storage = DecoratorStorage::default();
        storage.apply_decorator_place(address, DecoratorId(9), &target);
        world.insert_resource(storage);
        let mut subtypes = DerivedDecoratorSubtypes::default();
        subtypes.insert(9, DecoratorSubtype(8));
        world.insert_resource(subtypes);
        world.insert_resource(mesh_library(false));
        let mut cached = CachedDecoratorTransforms::default();
        cached.insert(
            9,
            CachedDecoratorTransform {
                rotation: Quat::IDENTITY,
                translation: Vec3::ZERO,
                scale: Vec3::ONE,
            },
        );
        world.insert_resource(cached);
        let roof_entity = world.spawn(roof(true)).id();
        let run = |w: &World| {
            let derived = DerivedDecoratorInfoView::new(
                w.resource::<DerivedDecoratorSubtypes>(),
                w.resource::<DecoratorMeshLibrary>(),
            );
            let bucket = w.resource::<DecoratorStorage>().get(address).unwrap();
            let hits = intersect_roof_decorators(
                &incoming,
                Vec3::Y,
                Quat::IDENTITY,
                1.7,
                2.2,
                bucket,
                derived,
                w.resource::<CachedDecoratorTransforms>(),
                DecoratorId(7),
            );
            let proposal = find_best_roof_decorator_merge_position(
                &incoming,
                Vec2::new(0.7, 0.6),
                Vec3::new(-4., 2., 0.),
                w.get::<Roof>(roof_entity).unwrap(),
                bucket,
                derived,
                &hits,
            );
            (hits, proposal)
        };
        let (hits, proposal) = run(&world);
        assert_eq!(hits, [9]);
        let proposal = proposal.unwrap();
        assert_eq!(proposal.target_id, DecoratorId(9));
        assert_eq!(proposal.info.rank(), 2);
        assert_eq!(proposal.info.decorator_id, incoming.decorator_id);
        assert_eq!(proposal.rotation, target.destination().as_roof().yaw);
        assert_eq!(run(&world).1, Some(proposal));
        world.resource_mut::<CachedDecoratorTransforms>().insert(
            9,
            CachedDecoratorTransform {
                rotation: Quat::IDENTITY,
                translation: Vec3::splat(10.),
                scale: Vec3::ONE,
            },
        );
        assert!(run(&world).0.is_empty());
        assert!(run(&world).1.is_none());
        world
            .resource_mut::<DecoratorStorage>()
            .apply_decorator_remove(address, DecoratorId(9));
        assert!(run(&world).0.is_empty());
        assert!(run(&world).1.is_none());
    }
}
