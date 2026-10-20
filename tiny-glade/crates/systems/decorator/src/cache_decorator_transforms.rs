//! B cache_decorator_transforms @1408153d0. Current-map ownership and the
//! wall/corner, non-stair roof, and terrain clutter branches are recovered.
//! Adjusted stair producer schemas remain incomplete and are not fabricated.
//! Native support oracle: progress-20261020/phase2-decorator-visual.

#[path = "cache_roof_orientation.rs"]
mod roof_orientation;
#[path = "cache_terrain_support.rs"]
mod terrain_support;

use bevy_ecs::prelude::*;

use country_core::resources::clutter_meshes::{clutter_mesh_ty_data, ClutterMeshTy, ClutterMeshes};
use country_core::resources::roofs::roof_shape::Roof;
use country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume;
use country_core::resources::stairs::{
    stairs_assembly_params::PlayermadeStairsAssemblyRequests, stairs_state::StairsState,
};
use country_core::resources::terrain_height::TerrainHeightsData;
use country_core::resources::walls::cached_decorator_transforms::{
    CachedDecoratorTransform, CachedDecoratorTransformStorage, CachedDecoratorTransforms,
};
use country_core::resources::walls::decorator::DecoratorRotationXZ;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_storage::DecoratorStorageAddr;
use country_core::resources::walls::public_wall_state::PublicWallState;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::walls::WallId;
use glam::{Vec2, Vec3};

/// The nine original SystemParams, including private previous-frame scratch.
pub fn cache_decorator_transforms(
    mut storage: ResMut<DecoratorStorage>,
    mut cache: ResMut<CachedDecoratorTransforms>,
    mut previous: Local<CachedDecoratorTransformStorage>,
    walls: Res<PublicWalls>,
    terrain: Res<TerrainHeightsData>,
    roof_trigger_volumes: Query<(&Roof, &RoofTriggerVolume)>,
    _stairs_requests: Res<PlayermadeStairsAssemblyRequests>,
    meshes: Res<ClutterMeshes>,
    _stairs: Res<StairsState>,
) {
    cache.swap_buffers_and_clear(&mut previous);
    for (wall, bucket) in storage.iter_walls() {
        let wall_state = walls.get(wall);
        for (id, info) in bucket.iter_maybe_mut() {
            let transform = compute_decorator_transform(
                info,
                Some(wall),
                wall_state,
                &walls,
                &roof_trigger_volumes,
                &terrain,
                &meshes,
                previous.get(id),
            );
            let Some(transform) = transform else { continue };
            cache.insert(id, transform);
        }
    }
    // O：原生迭代 `DecoratorStorage` 全存储（墙 + 地形 + 楼梯）；旧实现只
    // 遍历墙桶，地形/楼梯装饰永远不会进入缓存（已修）。
    if let Some(bucket) = storage.get_mut(DecoratorStorageAddr::Terrain) {
        let mut changed = Vec::new();
        for (id, info) in bucket.iter_maybe_mut() {
            assert_eq!(
                info.dst_tag(),
                3,
                "terrain storage contains a non-terrain decorator"
            );
            let Some(transform) = compute_decorator_transform(
                info,
                None,
                None,
                &walls,
                &roof_trigger_volumes,
                &terrain,
                &meshes,
                previous.get(id),
            ) else {
                continue;
            };
            let transform = if let Some(old) = previous.get(id).filter(|old| {
                let to_transform =
                    |t: &CachedDecoratorTransform| utils::components::transform::Transform {
                        r: t.rotation,
                        t: t.translation,
                        s: t.scale,
                    };
                to_transform(old).abs_diff_eq(&to_transform(&transform), f32::from_bits(0x3a83126f))
            }) {
                *old
            } else {
                changed.push(id);
                transform
            };
            cache.insert(id, transform);
        }
        bucket.modified_ids.extend(changed);
    }
    for bucket in storage.iter_stair_decorators() {
        for (id, info) in bucket.iter_maybe_mut() {
            let Some(transform) = compute_decorator_transform(
                info,
                None,
                None,
                &walls,
                &roof_trigger_volumes,
                &terrain,
                &meshes,
                previous.get(id),
            ) else {
                continue;
            };
            cache.insert(id, transform);
        }
    }
}

/// Reuse canonical geometry helpers; no fallback world pose for missing owners.
#[allow(clippy::too_many_arguments)]
fn compute_decorator_transform(
    info: &country_core::resources::walls::decorator::DecoratorInfo,
    wall: Option<WallId>,
    wall_state: Option<&PublicWallState>,
    walls: &PublicWalls,
    roof_trigger_volumes: &Query<(&Roof, &RoofTriggerVolume)>,
    terrain: &TerrainHeightsData,
    meshes: &ClutterMeshes,
    previous_transform: Option<&CachedDecoratorTransform>,
) -> Option<CachedDecoratorTransform> {
    match info.dst_tag() {
        // B140815875..894 normalizes every raw tag outside 2..=4 to the
        // wall branch. Keep the original parity below, even at the raw boundary.
        tag if tag.wrapping_sub(2) >= 3 => {
            let wall_state = wall_state.expect("wall decorator refers to a missing public wall");
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
            Some(CachedDecoratorTransform {
                rotation: utils::xz_rotation_bad(normal),
                translation,
                scale: Vec3::ONE,
            })
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
                return Some(fallback());
            };
            let (roof, volume) = roof_trigger_volumes
                .get(entity)
                .expect("roof association does not resolve to Roof and RoofTriggerVolume");
            let coord = info.wall_space_coord();
            let Some(translation) =
                volume.get_position_from_mesh_uv(Vec2::new(coord.u, coord.y), roof.floor_y())
            else {
                return Some(fallback());
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
                    // Original adjusted outgoing stair orientation is owned by
                    // the still-missing assembly producer, not the public graph.
                    return None;
                }
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
            Some(CachedDecoratorTransform {
                rotation,
                translation,
                scale: Vec3::ONE,
            })
        }
        3 => {
            // Type63 needs AdjustedStairEdge (outgoing graph orientation), whose
            // actual producer is not yet present in the canonical stair owner.
            if info.decorator_type().0 == 63 {
                return None;
            }
            let (x, z) = decorator_terrain_xz(info);
            let rotation = DecoratorRotationXZ(f32::from_bits(info.dst_secondary as u32)).to_quat();
            let mut translation = Vec3::new(x, terrain.sample_at_world_xz(x, z), z);
            if info.decorator_type().0.wrapping_sub(0x38) >= 8 || info.decorator_type().0 == 62 {
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
            Some(CachedDecoratorTransform {
                rotation,
                translation,
                scale: Vec3::ONE,
            })
        }
        // 楼梯目的（tag 4）：O 跳表有专用处理器；其位置公式（楼梯拓扑
        // 采样）尚未闭合，显式返回 None（U），不以零变换冒充。
        _ => None,
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
    use country_core::resources::walls::decorator::DecoratorInfo;

    #[test]
    fn terrain_xz_reads_packed_dst_wall() {
        let info = DecoratorInfo {
            dst_wall: u64::from(1.5f32.to_bits()) | (u64::from((-2.25f32).to_bits()) << 32),
            ..DecoratorInfo::default()
        };
        assert_eq!(decorator_terrain_xz(&info), (1.5, -2.25));
    }
}
