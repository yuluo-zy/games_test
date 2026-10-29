//! B141277bb0/1174B and B141278050/499B: grab correction and terrain-backed ray.
//! The original grab result is Option<Vec2>, not the previous guessed record.
use country_core::resources::terrain_height::{TerrainHeightSsbo, TerrainHeightsData};
use country_core::resources::walls::decorator::DecoratorInfo;
use country_core::resources::walls::decorator_storage::DecoratorStorageAddr;
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::walls::WallId;
use country_core::systems::collision::world::{RaycastWorld, RaycastWorldResult};
use country_core::utils::components::transform::Transform;
use glam::{Mat4, Vec2, Vec3};
use utils::{geometry::ray::Ray, wall_space::WallSpaceCoord};

pub type GrabOffsetSs = Vec2;
/// Canonical logical physics result; no claim of the original moved-enum ABI.
pub type RaycastIntersection = RaycastWorldResult;

/// Mandatory host accessors are the real projection, ray query and trigger
/// volume lookup. None from premade_wall_anchor means the actual volume could
/// not locate those UVs, not that its producer is missing. The surrounding
/// ui_move_decorator system remains unrecovered.
#[allow(clippy::too_many_arguments)]
pub fn try_get_grab_offset_ss_if_valid(
    info: &DecoratorInfo,
    source: &Transform,
    storage: DecoratorStorageAddr,
    walls: &PublicWalls,
    cursor_ss: Vec2,
    project: impl Fn(Vec3) -> Vec2,
    raycast_offset: impl FnOnce(Vec2) -> RaycastIntersection,
    premade_wall_anchor: impl FnOnce(WallId, WallSpaceCoord) -> Option<Vec3>,
) -> Option<GrabOffsetSs> {
    let tag = info.dst_tag();
    let index = if tag.wrapping_sub(2) < 3 { tag - 1 } else { 0 };
    let anchor = if index == 0 {
        let wall_id = storage.as_wall();
        let wall = walls.get(wall_id).unwrap();
        if wall.shape().tag() < 2 {
            let coordinate = info.wall_space_coord();
            match premade_wall_anchor(wall_id, coordinate) {
                Some(position) => position,
                None => {
                    coordinate.to_world_space(wall.simplified_curve_2d())
                        + source.rotation * Vec3::new(0.0, 0.0, 0.315)
                }
            }
        } else {
            source.translation
        }
    } else {
        source.translation
    };
    let delta = project(anchor) - cursor_ss;
    let hit = raycast_offset(delta);
    let projected_hit = project(hit.position);
    let ty = info.decorator_type().0;
    let family = ty.wrapping_sub(0x38);
    let valid = if family >= 8 || family == 6 || ty == b'?' {
        true
    } else if index < 2 {
        hit.handle[0] as u8 & 0xfd == 0 && hit.handle[1] == storage.as_wall().0
    } else if index == 2 {
        hit.handle[0] as u8 == 0xb
    } else {
        hit.handle[0] as u8 == 0xd
            && (hit.handle[0] >> 32) as u32 == info.dst_wall as u32
            && hit.handle[1] as u32 == (info.dst_wall >> 32) as u32
    };
    // Preserve this exact f32 sequence, including cancellation and NaN policy.
    valid.then(|| (cursor_ss + delta - projected_hit) + delta)
}

/// Actual unprojection and terrain-aware physics. The independent terrain
/// query always runs; a physical hit retains priority without distance sorting.
#[allow(clippy::too_many_arguments)]
pub fn raycast_with_ss_offset(
    cursor_ss: Vec2,
    offset: Vec2,
    viewport: Vec2,
    inverse_projection: Mat4,
    inverse_view: Mat4,
    decorator_type: u8,
    world: &RaycastWorld,
    terrain: &TerrainHeightsData<TerrainHeightSsbo>,
) -> RaycastIntersection {
    let ray = Ray::from_mouse_ss(
        cursor_ss + offset,
        viewport,
        inverse_projection,
        inverse_view,
    );
    select_offset_raycast(
        ray,
        decorator_type,
        |ray, mask| world.raycast_w_terrain(terrain, ray.origin, ray.direction, mask),
        |ray| {
            terrain
                .intersect_ray(
                    utils::geometry::Ray {
                        origin: ray.origin,
                        direction: ray.direction,
                    },
                    f32::MAX,
                )
                .map(|hit| hit.position)
        },
    )
}

pub(super) fn select_offset_raycast(
    ray: Ray,
    decorator_type: u8,
    raycast: impl FnOnce(&Ray, u32) -> RaycastWorldResult,
    terrain_intersect: impl FnOnce(&Ray) -> Option<Vec3>,
) -> RaycastIntersection {
    let mask = if decorator_type == b'?' { 0x23 } else { 0x30ab };
    let physical_hit = raycast(&ray, mask);
    let position = match terrain_intersect(&ray) {
        Some(position) => position,
        None => {
            // B has no epsilon, forward-t or finite guard. A horizontal ray
            // retains IEEE infinities/NaNs instead of becoming the ray origin.
            let t = (0.0 - ray.origin.y) / ray.direction.y;
            ray.origin + ray.direction * t
        }
    };
    if physical_hit.is_miss() {
        RaycastWorldResult {
            handle: [0xb, 0, 0],
            position,
            normal: Vec3::Y,
            toi: 0.0,
            discriminant: 0,
            extra: [0.0; 2],
        }
    } else {
        physical_hit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn physical_hit_has_priority_and_terrain_query_still_runs() {
        let calls = Cell::new(0);
        let ray = Ray {
            origin: Vec3::new(1.0, 2.0, 3.0),
            direction: -Vec3::Y,
        };
        let hit = RaycastWorldResult {
            discriminant: 0,
            position: Vec3::splat(17.0),
            ..RaycastWorldResult::miss()
        };
        for ty in [b'?', b'8', b'C'] {
            let result = select_offset_raycast(
                ray,
                ty,
                |_, mask| {
                    assert_eq!(mask, if ty == b'?' { 0x23 } else { 0x30ab });
                    hit
                },
                |_| {
                    calls.set(calls.get() + 1);
                    Some(Vec3::ZERO)
                },
            );
            assert_eq!(result, hit);
        }
        assert_eq!(calls.get(), 3);
    }

    #[test]
    fn miss_uses_terrain_then_unclamped_plane_and_ieee_horizontal_policy() {
        let ray = Ray {
            origin: Vec3::new(1.0, 2.0, 3.0),
            direction: Vec3::new(2.0, -1.0, 4.0),
        };
        let result = select_offset_raycast(ray, b'8', |_, _| RaycastWorldResult::miss(), |_| None);
        assert_eq!(result.position, Vec3::new(5.0, 0.0, 11.0));
        assert!(result.is_terrain());
        assert_eq!(result.normal, Vec3::Y);
        let terrain = select_offset_raycast(
            ray,
            b'8',
            |_, _| RaycastWorldResult::miss(),
            |_| Some(Vec3::splat(9.0)),
        );
        assert_eq!(terrain.position, Vec3::splat(9.0));
        let horizontal = Ray {
            direction: Vec3::X,
            ..ray
        };
        let result = select_offset_raycast(
            horizontal,
            b'8',
            |_, _| RaycastWorldResult::miss(),
            |_| None,
        );
        assert!(result.position.x.is_infinite());
        assert!(result.position.y.is_nan());
        assert!(result.position.z.is_nan());
    }
}
