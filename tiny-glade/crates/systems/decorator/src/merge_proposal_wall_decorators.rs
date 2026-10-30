//! B墙装饰合并：142001d40与142001880，完整原提案与严格最小距离选择。
//! 原源码路径已与B骨架核对；提案保留全72B新Info，只替换rank。
//! 沿墙移动使用合并前后的宽度差，不能用两半宽之和近似。

use crate::collision_detection::{
    WallIntersectCandidate, existing_collision_metrics, wall_decorator_intersects_in_space,
};
use country_core::resources::walls::WallId;
use country_core::resources::walls::decorator::{DecoratorId, DecoratorInfo, WallDecoratorDst};
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::{
    DecoratorLibKey, DecoratorMeshLibrary, DerivedDecoratorInfoView, DerivedDecoratorSubtypes,
};
use country_core::resources::walls::public_wall_state::PublicWallState;
use utils::comparative_space::ComparativeSpace;
use utils::wall_space::{WallSpace, WallSpaceCoord};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallMergePosition {
    pub wall: WallId,
    pub decorator_id: u32,
    pub target_id: u32,
    pub coord: WallSpaceCoord,
    pub rank_sum: u8,
    pub family: u8,
    pub ty: u8,
    /// 2是原Option niche，拒绝时其它载荷不参与消费。
    pub status: u8,
    pub distance: f32,
    pub info: DecoratorInfo,
    pub destination: WallDecoratorDst,
    pub compound: bool,
}
impl WallMergePosition {
    pub fn is_blocked(&self) -> bool {
        self.status == 2
    }
}

pub struct WallMergeEnv<'a> {
    pub mesh_lib: &'a DecoratorMeshLibrary,
    pub derived_subtypes: &'a DerivedDecoratorSubtypes,
    pub wall_space: &'a WallSpace,
    /// 原独立map key；Info.word2c不是装饰ID。
    pub current_id: DecoratorId,
}

pub fn wall_space_of(state: &PublicWallState) -> WallSpace {
    WallSpace {
        is_loop: state.shape().tag() != 2,
        length: state.wall_space_length(),
    }
}

#[inline]
pub fn decorator_family(ty: u8) -> u8 {
    let family = ty.wrapping_sub(0x38);
    if family < 8 { family } else { 6 }
}
#[inline]
fn families_compatible(a: u8, b: u8) -> bool {
    let family = decorator_family(a);
    family == decorator_family(b) && (family != 6 || a == b)
}
fn blocked(
    wall: WallId,
    current_id: DecoratorId,
    target_id: u32,
    coord: WallSpaceCoord,
    info: &DecoratorInfo,
) -> WallMergePosition {
    WallMergePosition {
        wall,
        decorator_id: current_id.0,
        target_id,
        coord,
        rank_sum: 0,
        family: decorator_family(info.ty),
        ty: info.ty,
        status: 2,
        distance: 0.0,
        info: *info,
        destination: WallDecoratorDst {
            coord,
            platform: WallId(0),
            relative_y: 0.0,
            position: glam::Vec3::ZERO,
            corner_attachment: 0,
            bottom_as_door: false,
        },
        compound: false,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn propose_wall_decorator_merge_position_inner(
    info: &DecoratorInfo,
    storage: &DecoratorStorage,
    env: &WallMergeEnv<'_>,
    wall: WallId,
    target_id: u32,
    candidate: WallIntersectCandidate,
    excluded: &[u32],
) -> WallMergePosition {
    let rejected = || blocked(wall, env.current_id, target_id, candidate.coord, info);
    // B142001ddc..1eff：目标及其subtype/mesh读取先于类型与rank门。
    let bucket = storage
        .wall_decorators(wall)
        .expect("target decorator missing");
    let existing = bucket
        .get(DecoratorId(target_id))
        .expect("target decorator missing");
    let subtype = env.derived_subtypes.get(target_id).0;
    let old = existing_collision_metrics(target_id, existing, env.derived_subtypes, env.mesh_lib);
    let mut destination = *existing.destination().as_wall();
    if !families_compatible(info.ty, existing.ty) {
        return rejected();
    }
    let rank = usize::from(existing.rank) + usize::from(info.rank);
    if rank > usize::from(info.decorator_type().max_rank()) {
        return rejected();
    }
    // 原第二次lookup以目标subtype和合并rank取真实碰撞，不能复用待放置宽度。
    let merged = env
        .mesh_lib
        .collision_record(DecoratorLibKey::new(subtype, rank as u8));
    let width = merged.bounds.footprint.size.x;
    let height = merged.bounds.top - merged.bounds.bottom;
    let old_coord = destination.coord;
    if (old_coord.y - candidate.coord.y).abs() < old.vertical_extent * 0.5 {
        // 原WallSpace::distance结果不消费，保留其准确纯计算边界。
        let _ = crate::collision_detection::wall_intersection_distance(
            env.wall_space,
            old_coord.u,
            candidate.coord.u,
        );
    }
    if destination.corner_attachment == 0 {
        if info.ty != 0x3a {
            // 原权重1/0仍参与乘加；NaN及非有限输入不得改写成符号近似。
            let a = 0.0 * old_coord.y + 1.0 * old_coord.u;
            let b = 0.0 * candidate.coord.y + 1.0 * candidate.coord.u;
            let difference = env.wall_space.subtract(a, b);
            let delta = if difference < 0.0 {
                width - old.along_width
            } else {
                old.along_width - width
            };
            let Some(u) = env.wall_space.project(old_coord.u + delta * 0.5) else {
                return rejected();
            };
            destination.coord.u = u;
            // 原仅移动的正常窗分支对gothic door subtype9进行垂直居中。
            let offset = if subtype == 9 {
                (height - old.vertical_extent) * 0.5
            } else {
                0.0
            };
            destination.coord.y += offset;
        }
        destination.corner_attachment = 0;
    }
    let proposed = WallIntersectCandidate {
        coord: destination.coord,
        along_width: width,
        vertical_extent: height,
    };
    let derived = DerivedDecoratorInfoView::new(env.derived_subtypes, env.mesh_lib);
    // 两个独立ID与额外slice共同排除，避免分配临时排除表。
    if bucket.decorators.iter().any(|(&id, other)| {
        id != env.current_id.0
            && id != target_id
            && wall_decorator_intersects_in_space(
                proposed,
                id,
                other,
                derived,
                env.wall_space,
                excluded,
            )
    }) {
        return rejected();
    }
    let dx = destination.coord.u - candidate.coord.u;
    let dy = destination.coord.y - candidate.coord.y;
    let mut merged_info = *info;
    merged_info.rank = rank as u8;
    WallMergePosition {
        wall,
        decorator_id: env.current_id.0,
        target_id,
        coord: destination.coord,
        rank_sum: rank as u8,
        family: decorator_family(info.ty),
        ty: info.ty,
        status: info.state,
        distance: (dy * dy + dx * dx).sqrt(),
        info: merged_info,
        destination,
        compound: existing.dst_tag() == 1,
    }
}

pub fn find_best_wall_decorator_merge_position_in(
    wall: WallId,
    targets: &[u32],
    info: &DecoratorInfo,
    storage: &DecoratorStorage,
    env: &WallMergeEnv<'_>,
    candidate: WallIntersectCandidate,
    excluded: &[u32],
) -> Option<WallMergePosition> {
    let mut best: Option<WallMergePosition> = None;
    for &target in targets {
        let next = propose_wall_decorator_merge_position_inner(
            info, storage, env, wall, target, candidate, excluded,
        );
        if next.is_blocked() {
            continue;
        }
        best = Some(match best {
            None => next,
            Some(previous) => {
                // 原min_by unwrap严格拒绝NaN；相同距离保留桶遍历的第一个目标。
                match next.distance.partial_cmp(&previous.distance).unwrap() {
                    std::cmp::Ordering::Less => next,
                    _ => previous,
                }
            }
        });
    }
    best
}

#[cfg(test)]
mod original_wall_merge_tests {
    use super::*;
    use country_core::resources::walls::decorator_storage::DecoratorStorageAddr;
    use country_core::resources::walls::decorator_subtype::{DecoratorCollision, DecoratorSubtype};
    use country_core::systems::collision::world::RaycastMeshData;
    use glam::{Vec2, Vec3};
    use std::sync::Arc;
    use utils::geometry::{Rectangle2d, Rectangle3d};
    fn info(b: [u8; 72]) -> DecoratorInfo {
        let u32at = |o: usize| u32::from_le_bytes(b[o..o + 4].try_into().unwrap());
        let u64at = |o: usize| u64::from_le_bytes(b[o..o + 8].try_into().unwrap());
        DecoratorInfo {
            dst_wall: u64at(0),
            dst_secondary: u64at(8),
            dst_payload: u64at(16),
            u18: u64at(24),
            u20: u32at(32),
            u24: u32at(36),
            u28: u32at(40),
            decorator_id: u32at(44),
            u30: u64at(48),
            u38: u32at(56),
            u3c: u32at(60),
            u40: u32at(64),
            rank: b[68],
            u45: b[69],
            ty: b[70],
            state: b[71],
        }
    }
    fn meshes() -> DecoratorMeshLibrary {
        let mut meshes = DecoratorMeshLibrary::default();
        for subtype in [0, 9] {
            for (rank, (width, top)) in [
                (1060320051u32, 1072064102u32),
                (1069799178, 1072902963),
                (1075251773, 1073741824),
                (1078733046, 1074161254),
            ]
            .into_iter()
            .enumerate()
            {
                meshes.collision.insert(
                    DecoratorLibKey::new(subtype, rank as u8),
                    DecoratorCollision {
                        triangles: vec![],
                        mesh: Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::ZERO)),
                        bounds: Rectangle3d {
                            footprint: Rectangle2d {
                                axis: Vec2::X,
                                center: Vec2::ZERO,
                                size: Vec2::new(f32::from_bits(width), 0.0),
                            },
                            bottom: -0.2,
                            top: f32::from_bits(top),
                        },
                    },
                );
            }
        }
        meshes
    }
    #[test]
    fn full_original_wall_merge_352_proposals_preserve_geometry_full_info_and_failure_order() {
        type Case = (
            [u8; 72],
            [u8; 72],
            [u32; 2],
            bool,
            u32,
            u8,
            Option<([u32; 2], u64, [u8; 3], [u8; 72], u32)>,
        );
        let cases: &[Case] = include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m01-wall-merge-native-vectors.rs"
        ));
        let meshes = meshes();
        for (index, &(incoming, target, coord, is_loop, length, subtype, expected)) in
            cases.iter().enumerate()
        {
            let mut storage = DecoratorStorage::default();
            let mut subtypes = DerivedDecoratorSubtypes::default();
            storage
                .get_or_insert_default(DecoratorStorageAddr::Wall(WallId(7)))
                .insert(DecoratorId(9), info(target));
            subtypes.insert(9, DecoratorSubtype(subtype));
            let space = WallSpace {
                is_loop,
                length: f32::from_bits(length),
            };
            let env = WallMergeEnv {
                mesh_lib: &meshes,
                derived_subtypes: &subtypes,
                wall_space: &space,
                current_id: DecoratorId(177),
            };
            let candidate = WallIntersectCandidate {
                coord: WallSpaceCoord::new_unchecked(
                    f32::from_bits(coord[0]),
                    f32::from_bits(coord[1]),
                ),
                along_width: 111.0,
                vertical_extent: 222.0,
            };
            let got = propose_wall_decorator_merge_position_inner(
                &info(incoming),
                &storage,
                &env,
                WallId(7),
                9,
                candidate,
                &[],
            );
            match expected {
                None => assert!(got.is_blocked(), "原case{index}"),
                Some((coordinate, platform, attachment, bytes, distance)) => {
                    assert!(!got.is_blocked(), "原case{index}");
                    assert_eq!(
                        [got.coord.u.to_bits(), got.coord.y.to_bits()],
                        coordinate,
                        "原case{index}"
                    );
                    assert_eq!(got.destination.platform.0, platform);
                    assert_eq!(
                        [
                            got.destination.corner_attachment,
                            u8::from(got.destination.bottom_as_door),
                            u8::from(got.compound)
                        ],
                        attachment
                    );
                    assert_eq!(
                        *crate::move_decorators_following_anchors::observed_from_info(&got.info)
                            .bytes(),
                        bytes,
                        "原完整Info case{index}"
                    );
                    assert_eq!(got.distance.to_bits(), distance, "原distance case{index}");
                }
            }
        }
    }
    #[test]
    fn original_intent_wall_predicates_160_cases_include_circular_seam_and_id_exclusion() {
        let cases: &[(bool, u8, bool, u32, u32, bool)] = include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m01-wall-intent-predicate-native-vectors.rs"
        ));
        let meshes = meshes();
        let mut subtypes = DerivedDecoratorSubtypes::default();
        subtypes.insert(9, DecoratorSubtype(0));
        for &(is_loop, ty, excluded, dx, dy, expected) in cases {
            let mut bucket =
                country_core::resources::walls::decorator_storage::Decorators::default();
            let target = DecoratorInfo {
                dst_wall: u64::from(0.1f32.to_bits()) | (u64::from(1.0f32.to_bits()) << 32),
                rank: 1,
                ty,
                ..Default::default()
            };
            bucket
                .decorators
                .insert(if excluded { 177 } else { 9 }, target);
            let candidate = WallIntersectCandidate {
                coord: WallSpaceCoord::new_unchecked(
                    0.1 + f32::from_bits(dx),
                    1.0 + f32::from_bits(dy),
                ),
                along_width: 1.7,
                vertical_extent: 2.2,
            };
            let got = crate::collision_detection::intersect_wall_decorators_for_intent(
                &bucket,
                DecoratorId(177),
                candidate,
                &WallSpace {
                    is_loop,
                    length: 10.0,
                },
                DerivedDecoratorInfoView::new(&subtypes, &meshes),
            );
            assert_eq!(
                !got.is_empty(),
                expected,
                "loop{is_loop} ty{ty} dx{dx:x} dy{dy:x} excluded{excluded}"
            );
        }
    }
    #[test]
    fn complete_original_minimum_24_cases_keep_ties_order_duplicates_and_nan_unwrap() {
        type Case = (&'static [u32], u32, Result<Option<(u32, u32)>, ()>);
        let cases: &[Case] = include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m01-wall-merge-minimum-native-vectors.rs"
        ));
        let meshes = meshes();
        let mut subtypes = DerivedDecoratorSubtypes::default();
        let mut storage = DecoratorStorage::default();
        for (id, u) in [(9, 2f32), (10, 8f32)] {
            subtypes.insert(id, DecoratorSubtype(0));
            let mut target = DecoratorInfo {
                ty: 0x38,
                rank: 1,
                state: 1,
                ..Default::default()
            };
            target.dst_wall = u64::from(u.to_bits()) | (u64::from(0.8f32.to_bits()) << 32);
            storage.apply_decorator_place(
                DecoratorStorageAddr::Wall(WallId(7)),
                DecoratorId(id),
                &target,
            );
        }
        let source = DecoratorInfo {
            ty: 0x38,
            rank: 1,
            state: 1,
            ..Default::default()
        };
        let space = WallSpace {
            is_loop: true,
            length: 10.,
        };
        let env = WallMergeEnv {
            mesh_lib: &meshes,
            derived_subtypes: &subtypes,
            wall_space: &space,
            current_id: DecoratorId(177),
        };
        for &(ids, u, expected) in cases {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                find_best_wall_decorator_merge_position_in(
                    WallId(7),
                    ids,
                    &source,
                    &storage,
                    &env,
                    WallIntersectCandidate {
                        coord: WallSpaceCoord::new_unchecked(f32::from_bits(u), 0.8),
                        along_width: 9.,
                        vertical_extent: 9.,
                    },
                    &[],
                )
            }));
            match (result, expected) {
                (Err(_), Err(())) => {}
                (Ok(None), Ok(None)) => {}
                (Ok(Some(got)), Ok(Some((id, distance)))) => {
                    assert_eq!(got.target_id, id);
                    assert_eq!(got.distance.to_bits(), distance);
                }
                _ => panic!("原最小提案选择{ids:?} u{u:x}"),
            }
        }
    }
}
