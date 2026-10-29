//! 原B1407cb1a0：装饰/屋顶缓存差分→脏墙稀疏瓦片重建。
//! 保持B既有cottage_dormers归属；实例仅进入唯一RoofTileSsbo，不发布空蓝图。

use super::util_assemble_gothic_dormer::assemble_gothic_dormer_tiles;
use crate::blueprint::DecoratorBlueprints;
use bevy_ecs::prelude::*;
use country_core::{
    asset_libraries::ssbo_library::AssetSsboLibrary,
    resources::{
        roof_tile_ssbo::RoofTileSsbo,
        roofs::{dormer_tiles::DormerTiles, roof_shape::Roof},
        walls::{
            WallId,
            cached_decorator_transforms::{CachedDecoratorTransform, CachedDecoratorTransforms},
            decorator::DecoratorId,
            decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
            decorator_subtype::{DecoratorLibKey, DerivedDecoratorInfoResources},
            public_walls::PublicWalls,
        },
    },
    systems::ui::signifiers::UiSignifierStream,
};
use glam::{Affine3A, Vec2, Vec3};
use std::collections::HashMap;

pub const DORMER_ROOF_SUBTYPE_MASK: u32 = 0x0080_0186;
pub const DORMER_ROOF_SUBTYPE_LIMIT: u8 = 0x18;
pub fn is_dormer_roof_subtype(subtype: u8) -> bool {
    subtype < DORMER_ROOF_SUBTYPE_LIMIT && ((DORMER_ROOF_SUBTYPE_MASK >> subtype) & 1) != 0
}

/// 原48B缓存只比较位置、floor、color和屋顶参数；不私自加入scale/rotation。
#[derive(Clone, Debug)]
struct DormerCache {
    position: Vec3,
    floor: f32,
    color: u32,
    parameters: [u8; 28],
}
impl DormerCache {
    fn new(transform: CachedDecoratorTransform, roof: &Roof) -> Self {
        Self {
            position: transform.translation,
            floor: roof.floor_y(),
            color: u32::from_le_bytes(roof.observed()[0x50..0x54].try_into().unwrap()),
            parameters: roof.observed()[0x24..0x40].try_into().unwrap(),
        }
    }
}
impl PartialEq for DormerCache {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
            && self.floor == other.floor
            && self.color == other.color
            && (0..6).all(|i| {
                let a = f32::from_le_bytes(self.parameters[i * 4..i * 4 + 4].try_into().unwrap());
                let b = f32::from_le_bytes(other.parameters[i * 4..i * 4 + 4].try_into().unwrap());
                a == b
            })
            && self.parameters[24..26] == other.parameters[24..26]
    }
}

#[derive(Default)]
pub struct CacheDormerPositions(HashMap<DecoratorId, (WallId, DormerCache)>);

fn warn_tile_capacity(gothic: bool) {
    use std::sync::atomic::{AtomicBool, Ordering};
    static COTTAGE: AtomicBool = AtomicBool::new(false);
    static GOTHIC: AtomicBool = AtomicBool::new(false);
    let flag = if gothic { &GOTHIC } else { &COTTAGE };
    if !flag.swap(true, Ordering::Relaxed) {
        log::warn!(target:"system_decorator::decorator_visual::cottage_dormers", "Out of roof tile instances");
    }
}

/// 原10组参数。现安全SSBO API通过ResMut表达真实写依赖，不仿写原内部容器。
pub fn generate_dormer_roofs(
    derived: DerivedDecoratorInfoResources,
    storage: Res<DecoratorStorage>,
    blueprints: Res<DecoratorBlueprints>,
    cached: Res<CachedDecoratorTransforms>,
    walls: Res<PublicWalls>,
    roofs: Query<&Roof>,
    mut ssbos: ResMut<AssetSsboLibrary>,
    _signifiers: Res<UiSignifierStream>,
    mut tiles: ResMut<DormerTiles>,
    mut previous: Local<CacheDormerPositions>,
) {
    let view = derived.as_view();
    let mut current = HashMap::with_capacity(previous.0.len());
    for (wall_id, decorators) in storage.iter_walls() {
        let Some(entity) = walls.roof_entity(wall_id) else {
            continue;
        };
        let Ok(roof) = roofs.get(entity) else {
            continue;
        };
        for (&raw_id, info) in &decorators.decorators {
            let id = DecoratorId(raw_id);
            let Some(subtype) = view.try_get_subtype(id) else {
                continue;
            };
            if !is_dormer_roof_subtype(subtype.0) && !(subtype.0 == 0 && info.rank() == 3) {
                continue;
            }
            current.insert(
                id,
                (wall_id, DormerCache::new(cached.transform(raw_id), roof)),
            );
        }
    }
    let mut dirty = Vec::new();
    for (&id, (wall, state)) in &current {
        match previous.0.remove(&id) {
            Some((old_wall, old)) if old_wall == *wall && old == *state => {}
            Some((old_wall, _)) => {
                dirty.push(old_wall);
                dirty.push(*wall);
            }
            None => dirty.push(*wall),
        }
    }
    dirty.extend(previous.0.drain().map(|(_, (wall, _))| wall));
    previous.0 = current;
    dirty.sort_unstable_by_key(|wall| wall.0);
    dirty.dedup();
    // 原查询即使dirty为空也解析required typed buffer；缺资源不默默吞掉。
    if dirty.is_empty() {
        ssbos
            .get::<RoofTileSsbo>()
            .expect("RoofTileSsbo asset must be registered");
        return;
    }
    let buffer = ssbos
        .get_mut::<RoofTileSsbo>()
        .expect("RoofTileSsbo asset must be registered");
    for wall_id in dirty {
        let Some(entity) = walls.roof_entity(wall_id) else {
            continue;
        };
        let Ok(roof) = roofs.get(entity) else {
            continue;
        };
        let allocation = tiles
            .allocations
            .entry(wall_id)
            .or_insert_with(|| buffer.new_allocation());
        let mut writer = buffer.writer(allocation);
        writer.clear();
        let Some(decorators) = storage.get(DecoratorStorageAddr::Wall(wall_id)) else {
            continue;
        };
        for (&raw_id, info) in &decorators.decorators {
            let id = DecoratorId(raw_id);
            let subtype = view.subtype(id).0;
            // 原1407cd2d5..cd343缺blueprint跳过当前装饰，不创建空entry。
            if !is_dormer_roof_subtype(subtype) && !(subtype == 0 && info.rank() == 3) {
                continue;
            }
            let Some(_blueprint) = blueprints.get(raw_id) else { continue; };
            let mut transform = cached.transform(raw_id);
            let mut emit = |record| {
                if writer.push(&record).is_err() {
                    warn_tile_capacity(matches!(subtype, 7 | 8 | 23));
                }
            };
            if subtype == 0 && info.rank() == 3 {
                let rank = usize::from(info.rank() - 1);
                transform.translation.y += 0.8;
                let mut bay_roof = roof.clone();
                let b = bay_roof.observed_mut();
                b[0x2c..0x30].copy_from_slice(&0.5f32.to_le_bytes());
                b[0x30..0x34].copy_from_slice(&0.2f32.to_le_bytes());
                b[0x4c..0x50].copy_from_slice(&(transform.translation.y + 0.2).to_le_bytes());
                let padding = if b[8] == 0 { 0.15 } else { 0.35 };
                assemble_dormer_roof_tiles(
                    &transform,
                    [0.7, 1.4, 2.1000001][rank] + 0.2,
                    0.0,
                    info.rank(),
                    &bay_roof,
                    padding,
                    false,
                    &mut emit,
                );
                continue;
            }
            if roof.is_flat() {
                continue;
            }
            match subtype {
                1 | 2 => {
                    assert!(
                        (1..=3).contains(&info.rank()),
                        "internal error: entered unreachable code"
                    );
                    let rank = usize::from(info.rank() - 1);
                    let (width, height, padding) = if subtype == 1 {
                        let wall = walls
                            .get(wall_id)
                            .expect("called Option::unwrap() on a None value");
                        let max_y = wall.max_y();
                        let collision = view
                            .mesh_library()
                            .collision_record(DecoratorLibKey::new(1, info.rank()));
                        let height = (collision.bounds.top - collision.bounds.bottom) * 0.5
                            - (max_y - transform.translation.y);
                        transform.translation.y = max_y;
                        (
                            collision.bounds.footprint.size.x + 0.2,
                            height,
                            [0.1, 0.05, 0.3][rank],
                        )
                    } else {
                        ([0.8, 1.5, 2.2][rank] + 0.2, 1.0, 0.1)
                    };
                    assemble_dormer_roof_tiles(
                        &transform,
                        width,
                        height,
                        info.rank(),
                        roof,
                        padding,
                        true,
                        &mut emit,
                    );
                }
                8 => {
                    let profile =
                        crate::decorator_visual::gothic_dormer::gothic_dormer_profile_curve(
                            info.rank(),
                        );
                    let affine = Affine3A::from_scale_rotation_translation(
                        transform.scale,
                        transform.rotation,
                        transform.translation,
                    );
                    let profiles = [1.0, -1.0].map(|side| {
                        utils::curve::Curve3::try_new_from_points(
                            profile
                                .iter()
                                .map(|p| affine.transform_point3(Vec3::new(p.x * side, p.y, 0.0)))
                                .collect(),
                            false,
                        )
                        .unwrap()
                    });
                    assemble_gothic_dormer_tiles(&profiles, roof, &transform, &mut emit);
                }
                7 | 23 => {
                    let profile = if subtype == 23 {
                        vec![Vec2::new(-0.0, 1.27), Vec2::new(0.6, 0.0)]
                    } else {
                        country_core::resources::gothic_window_profile::gothic_half_dormer_profile_curve(info.rank())
                    };
                    let coord = info.dst_as_wall().wall_space_coord();
                    let wall = walls
                        .get(wall_id)
                        .expect("Missing public wall for gothic dormer");
                    let half_base = if subtype == 23 {
                        0.0
                    } else {
                        match info.rank() {
                            1 => 0.95128,
                            2 => 1.49408,
                            3 => 1.9242719,
                            _ => panic!("internal error: entered unreachable code"),
                        }
                    };
                    let bones = crate::decorator_visual::gothic_wall_window::get_gothic_hat_bones(
                        wall, info, coord.u, coord.y, half_base,
                    );
                    let profiles = crate::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws(info.rank(),subtype,&profile,&bones,wall);
                    assemble_gothic_dormer_tiles(&profiles, roof, &transform, &mut emit);
                }
                _ => unreachable!("native subtype gate"),
            }
        }
    }
}

/// 原14128db20：保持圆周跨零、曲面各行截面和真实侧板；调用者持有唯一Sparse writer。
#[allow(clippy::too_many_arguments)]
pub fn assemble_dormer_roof_tiles(
    transform: &country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransform,
    width: f32,
    height: f32,
    rank: u8,
    roof: &country_core::resources::roofs::roof_shape::Roof,
    padding: f32,
    side_boards: bool,
    mut emit: impl FnMut(country_core::resources::roof_tile_ssbo::RoofTileSsbo),
) {
    use super::{
        collision_geo::roof_section,
        util_assemble_gothic_dormer::{length, position_rng, tile_record},
    };
    use glam::{Mat3, Quat, Vec2, Vec3};
    let roof_height = roof.height();
    assert!(
        (1..=3).contains(&rank),
        "internal error: entered unreachable code"
    );
    let position = transform.translation;
    let location = Vec2::new(position.x, position.z);
    let front_base_y = height + position.y;
    let top_y = roof_height * [0.1, 0.15, 0.2][usize::from(rank - 1)] + front_base_y;
    let bottom = roof_section(roof, position);
    let top = roof_section(roof, Vec3::new(position.x, top_y, position.z));
    let bottom_point = bottom.get_pos_at_u(bottom.get_closest_u_from_pos(location));
    let top_point = top.get_pos_at_u(top.get_closest_u_from_pos(location));
    let a = Vec3::new(bottom_point.x, front_base_y, bottom_point.y);
    let b = Vec3::new(top_point.x, top_y, top_point.y);
    let distance = length(a - b);
    let rng = position_rng(position);
    let splits = utils::random_splits(
        ((distance / 0.4).ceil() as usize).max(2),
        f32::from_bits(0x3da3_d70b) / distance,
        &rng,
    );
    let half_width_u = width / bottom.perimeter() * 0.5;
    let roof_id = roof.wall_id().0 as u32;
    let quarter = Quat::from_xyzw(0.70710677, 0.0, 0.0, 0.70710677);
    let wrap = |u: f32| {
        let r = u % 1.0;
        if r < 0.0 { r + 1.0 } else { r }
    };
    let across = |a: f32, b: f32, t: f32| {
        let b = if a <= b { b } else { b + 1.0 };
        wrap(b * t + a * (1.0 - t))
    };
    let normalize = |v: Vec3| v * (1.0 / length(v));
    for (row, pair) in splits.windows(2).enumerate() {
        // 原先在两真实anchor间插值Y，再inverse_lerp；不可直接以pair作为t。
        let row_t = |u: f32| {
            ((top_y * u + front_base_y * (1.0 - u)) - front_base_y) / (top_y - front_base_y)
        };
        let t0 = row_t(pair[0]);
        let t1 = row_t(pair[1]);
        let section_y0 = position.y * (1.0 - t0) + top_y * t0;
        let section_y1 = position.y * (1.0 - t1) + top_y * t1;
        let low = roof_section(roof, Vec3::new(position.x, section_y0, position.z))
            .expand((1.0 - t0) * padding + t0 * 0.0);
        let high = roof_section(roof, Vec3::new(position.x, section_y1, position.z))
            .expand((1.0 - t1) * padding + t1 * 0.0);
        let low_u = low.get_closest_u_from_pos(location);
        let high_u = high.get_closest_u_from_pos(location);
        let row_width = low.perimeter() / bottom.perimeter() * width;
        let columns =
            utils::random_splits(((row_width / 0.3) as usize).max(2), 0.06 / row_width, &rng);
        let y0 = top_y * t0 + front_base_y * (1.0 - t0);
        let y1 = top_y * t1 + front_base_y * (1.0 - t1);
        let low_start = wrap(low_u - half_width_u);
        let low_end = wrap(low_u + half_width_u);
        let high_start = wrap(high_u - half_width_u);
        let high_end = wrap(high_u + half_width_u);
        let point =
            |shape: &country_core::systems::wall::valid_enclosure::ShapeParameters, u, y| {
                let p = shape.get_pos_at_u(u);
                Vec3::new(p.x, y, p.y)
            };
        for (column, pair) in columns.windows(2).enumerate() {
            let a = point(&low, across(low_start, low_end, pair[0]), y0);
            let b = point(&low, across(low_start, low_end, pair[1]), y0);
            let c = point(&high, across(high_start, high_end, pair[0]), y1);
            let d = point(&high, across(high_start, high_end, pair[1]), y1);
            let lower_center = a * 0.5 + b * 0.5;
            if row == 0 {
                let left = low.get_pos_at_u(wrap(low_u - half_width_u));
                let right = low.get_pos_at_u(wrap(low_u + half_width_u));
                let center = low.get_pos_at_u(low_u);
                let delta = left * 0.5 + right * 0.5 - center;
                let depth = ((delta.y * delta.y + delta.x * delta.x).sqrt() + 0.1).max(0.3);
                let upper_center = Vec3::new(c.x, y0, c.z) * 0.5 + Vec3::new(d.x, y0, d.z) * 0.5;
                let direction = normalize(upper_center - lower_center);
                let right = normalize(direction.cross(Vec3::Y));
                let rotation = Quat::from_mat3(&Mat3::from_cols(right, direction, Vec3::Y));
                emit(tile_record(
                    direction * depth * 0.5 + lower_center,
                    Vec3::new(length(b - a), depth, 0.1),
                    rotation,
                    roof_id,
                    &rng,
                ));
            }
            let width_direction = normalize(b - a);
            let slope = normalize((c * 0.5 + d * 0.5) - lower_center);
            let normal = width_direction.cross(slope);
            let basis = Quat::from_mat3(&Mat3::from_cols(width_direction, slope, normal));
            let pitch = rng.f32();
            let angle = (pitch * -10.0 - ((1.0 - pitch) + (1.0 - pitch)))
                * f32::from_bits(0x3c8e_fa35)
                * 0.5;
            let (sin, cos) = angle.sin_cos();
            let rotation = basis * Quat::from_xyzw(sin, 0.0, 0.0, cos);
            emit(tile_record(
                (a + b + c + d) * 0.25,
                Vec3::new(length(b - a), length(c - a) + 0.1, 0.1),
                rotation,
                roof_id,
                &rng,
            ));
            for (edge_a, edge_b, last) in [(a, c, false), (b, d, true)] {
                if (!last && column != 0) || (last && column != columns.len() - 2) {
                    continue;
                }
                let edge = edge_a - edge_b;
                let edge_length = length(edge);
                let rotation = utils::rotation_from_normal(edge * (1.0 / edge_length)) * quarter;
                emit(tile_record(
                    edge_a * 0.5 + edge_b * 0.5,
                    Vec3::new(0.15, edge_length + 0.16, 0.2),
                    rotation,
                    roof_id,
                    &rng,
                ));
                if side_boards {
                    emit(side_board(
                        edge_a, edge_b, section_y0, section_y1, last, roof_id, &rng,
                    ));
                }
            }
        }
    }
}

/// 原1412905d0：侧板的独立竖直面、偏移、半精度裁剪平面和随机种子。
#[allow(clippy::too_many_arguments)]
fn side_board(
    a: glam::Vec3,
    b: glam::Vec3,
    low_y: f32,
    high_y: f32,
    last: bool,
    roof_id: u32,
    rng: &fastrand::Rng,
) -> country_core::resources::roof_tile_ssbo::RoofTileSsbo {
    use super::util_assemble_gothic_dormer::{length, tile_record, to_f16_bits};
    use glam::{Mat3, Quat, Vec3};
    let floor = low_y.min(high_y) - 0.1;
    let horizontal = Vec3::new(b.x - a.x, 0.0, b.z - a.z);
    let span = length(horizontal);
    let direction = horizontal * (1.0 / span);
    let normal = direction.cross(Vec3::Y);
    let center = Vec3::new(
        (a.x + b.x) * 0.5,
        b.y * 0.5 + floor * 0.5,
        (a.z + b.z) * 0.5,
    );
    let base = Quat::from_mat3(&Mat3::from_cols(direction, Vec3::Y, normal));
    let rotation = if last {
        base
    } else {
        base * Quat::from_rotation_y(std::f32::consts::PI)
    };
    let position = if last {
        center - normal * 0.08
    } else {
        normal * 0.08 + center
    };
    let mut tile = tile_record(
        position,
        Vec3::new(span, b.y - floor, 0.1),
        rotation,
        roof_id,
        rng,
    );
    let clipping_normal = (b - a).cross(normal);
    let distance = b.z * clipping_normal.z + (b.y * clipping_normal.y + b.x * clipping_normal.x);
    let plane = [
        clipping_normal.x,
        clipping_normal.y,
        clipping_normal.z,
        -distance,
    ]
    .map(to_f16_bits);
    tile.clipping_plane = [
        u32::from(plane[0]) | (u32::from(plane[1]) << 16),
        u32::from(plane[2]) | (u32::from(plane[3]) << 16),
    ];
    tile
}

#[cfg(test)]
mod complete_cottage_tile_native_tests {
    use super::*;
    use country_core::resources::{
        roofs::roof_shape::Roof, walls::cached_decorator_transforms::CachedDecoratorTransform,
    };
    use glam::{Quat, Vec3};
    fn words(t: country_core::resources::roof_tile_ssbo::RoofTileSsbo) -> [u32; 16] {
        [
            t.base_position.x.to_bits(),
            t.base_position.y.to_bits(),
            t.base_position.z.to_bits(),
            t.current_height.to_bits(),
            t.clipping_plane[0],
            t.clipping_plane[1],
            t.scale_t[0],
            t.scale_t[1],
            t.rotation_quat[0],
            t.rotation_quat[1],
            t.roof_id,
            t.velocity_rotation,
            t.animation_t.to_bits(),
            t.seed as u32,
            t.is_circle as u32,
            t.is_main_roof_body as u32,
        ]
    }
    #[test]
    fn cottage_original_tiles_compare_every_record_of_complete_native_algorithms() {
        let cases: &[(u8, bool, bool, &[[u32; 16]])] = include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m01-cottage-tile-native-fixtures.rs"
        ));
        for &(rank, rectangle, extra, expected) in cases {
            let mut b = [0u8; 88];
            b[..8].copy_from_slice(&7u64.to_le_bytes());
            b[8..12].copy_from_slice(&u32::from(rectangle).to_le_bytes());
            let frame = if rectangle {
                [1., 0., 0., 0., 8., 6.]
            } else {
                [0., 0., 4., 0., 0., 0.]
            };
            for (j, v) in frame.into_iter().enumerate() {
                b[12 + j * 4..16 + j * 4].copy_from_slice(&f32::to_le_bytes(v));
            }
            for (o, v) in [(0x2c, 0.5f32), (0x30, 0.6)] {
                b[o..o + 4].copy_from_slice(&v.to_le_bytes());
            }
            let roof = Roof::from_observed(b);
            let transform = CachedDecoratorTransform {
                rotation: Quat::IDENTITY,
                translation: Vec3::new(-4., 1., 0.),
                scale: Vec3::ONE,
            };
            let mut actual = Vec::new();
            assemble_dormer_roof_tiles(
                &transform,
                [0.8, 1.5, 2.2][usize::from(rank - 1)],
                1.,
                rank,
                &roof,
                0.1,
                extra,
                |record| actual.push(words(record)),
            );
            assert_eq!(
                actual.len(),
                expected.len(),
                "rank {rank} rectangle {rectangle} extra {extra}"
            );
            let mismatches: Vec<_> = actual
                .iter()
                .zip(expected)
                .enumerate()
                .flat_map(|(i, (a, b))| {
                    (0..16).filter_map(move |field| {
                        (a[field] != b[field]).then_some((i, field, a[field], b[field]))
                    })
                })
                .collect();
            assert!(
                mismatches.is_empty(),
                "rank {rank} rectangle {rectangle} extra {extra}: {mismatches:?}"
            );
        }
    }
}

#[cfg(test)]
mod complete_dormer_roof_main_world_tests {
    use super::*;
    use bevy_ecs::system::RunSystemOnce;
    use country_core::{
        asset_libraries::shader_library::{AssetShaderLibrary, SharedAssetShaderLibrary},
        render::{sparse_instance_buffer::SparseInstanceBuffer,test_device::TestDevice},
        resources::walls::{decorator_collision::DecoratorPhysicsColliders,wall_holes::WallHoles,public_walls::OnWallDeleted},
        systems::{collision::{WindowAutoClutterCandidates,world::RaycastWorld},wall::wall_state::path_preparation::PathRasters512},
    };
    use crate::{blueprint::PrevDecoratorBlueprints,decorator_visual_state::DecoratorVisualState,
        decorator_visual::deco_differential_state::{WallAttachedDecoTracker,ChimneyAssemblyParams,DoorStairsAssemblyParams}};
    use std::sync::Arc;
    fn count(world:&World)->usize {
        let tiles=world.resource::<DormerTiles>();
        let Some(allocation)=tiles.allocations.get(&WallId(42)) else { return 0; };
        let buffer=world.resource::<AssetSsboLibrary>().get::<RoofTileSsbo>().unwrap();
        allocation.handle.0.as_ref().unwrap().chunk_ids.lock().iter().map(|&chunk|usize::from(buffer.byte_map[chunk as usize])).sum()
    }
    #[test]
    fn actual_window_and_arrow_producers_reach_all_half_and_bay_roof_branches_then_unload() {
        for subtype in [0,1,7,23] {
            for rank in 1..=3 {
                let mut world=crate::decorator_visual::cottage_wall_window::original_window_helper_tests::producer_world(subtype,rank);
                // 公用生成器夹具仅填桶；真实删除还要求唯一的反向存储索引。
                world.resource_mut::<DecoratorStorage>().decorator_to_storage.insert(DecoratorId(7),DecoratorStorageAddr::Wall(WallId(42)));
                if subtype == 23 {
                    let mut storage=world.resource_mut::<DecoratorStorage>();
                    let info=storage.get_mut(DecoratorStorageAddr::Wall(WallId(42))).unwrap().decorators.get_mut(&7).unwrap();
                    info.ty=crate::decorator_visual::arrow_slit::ARROW_SLIT_TYPE;
                    info.set_extra(country_core::resources::walls::decorator::DecoratorExtraInfo::Window(country_core::resources::walls::decorator::WindowExtra(77)));
                }
                let paths=PathRasters512::from_encoded(std::array::from_fn(|_|vec![0u8;512*512].into()),false).unwrap();
                world.resource_mut::<PublicWalls>().get_mut(WallId(42)).unwrap().prepare_paths(&paths).unwrap();
                let mut bytes=[0u8;88];bytes[..8].copy_from_slice(&42u64.to_le_bytes());
                bytes[8..0x24].copy_from_slice(world.resource::<PublicWalls>().get(WallId(42)).unwrap().shape().bytes());
                for (offset,value) in [(0x2c,0.5f32),(0x30,0.6),(0x34,0.5)] { bytes[offset..offset+4].copy_from_slice(&value.to_le_bytes()); }
                let roof=world.spawn(Roof::from_observed(bytes)).id();world.resource_mut::<PublicWalls>().associate_roof(WallId(42),roof);
                let ssbos=AssetSsboLibrary::new();ssbos.register::<RoofTileSsbo>(SparseInstanceBuffer::<RoofTileSsbo>::new_storage_buffer(&TestDevice(1),None));
                world.insert_resource(ssbos);world.init_resource::<DormerTiles>();
                match subtype {
                    0|1=>world.run_system_once(crate::decorator_visual::cottage_wall_window::generate_cottage_wall_windows).unwrap(),
                    7=>world.run_system_once(crate::decorator_visual::gothic_wall_window::generate_gothic_wall_windows).unwrap(),
                    23=>world.run_system_once(crate::decorator_visual::arrow_slit::generate_arrow_slits).unwrap(),
                    _=>unreachable!(),
                }
                world.init_resource::<PrevDecoratorBlueprints>();world.init_resource::<DecoratorVisualState>();world.init_resource::<WallHoles>();
                world.insert_resource(RaycastWorld::new());world.init_resource::<DecoratorPhysicsColliders>();world.init_resource::<WindowAutoClutterCandidates>();
                world.init_resource::<WallAttachedDecoTracker<ChimneyAssemblyParams>>();world.init_resource::<WallAttachedDecoTracker<DoorStairsAssemblyParams>>();
                world.insert_resource(SharedAssetShaderLibrary(Arc::new(AssetShaderLibrary::new())));world.init_resource::<Events<OnWallDeleted>>();
                world.run_system_once(crate::blueprint::instantiate_blueprint_visual_state).unwrap();
                let mut schedule=Schedule::default();schedule.add_systems(generate_dormer_roofs);schedule.run(&mut world);
                let expected_nonempty=!(subtype==0 && rank<3 || subtype==23 && rank==1);
                assert_eq!(count(&world)>0,expected_nonempty,"subtype {subtype} rank {rank}");
                if expected_nonempty {
                    assert!(!world.resource::<DecoratorVisualState>().persistent_entities_of(7).unwrap().is_empty());
                    world.run_system_once(crate::blueprint::copy_blueprints_to_prev).unwrap();
                    world.resource_mut::<DecoratorStorage>().apply_decorator_remove(DecoratorStorageAddr::Wall(WallId(42)),DecoratorId(7));
                    world.run_system_once(crate::blueprint::clear_blueprints).unwrap();
                    world.run_system_once(crate::blueprint::instantiate_blueprint_visual_state).unwrap();schedule.run(&mut world);
                    assert_eq!(count(&world),0);assert!(world.resource::<DecoratorVisualState>().persistent_entities_of(7).is_none());
                }
            }
        }
    }
}
