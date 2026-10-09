//! Click-only broad phase + exact CPU triangle picking of the displayed mesh.
use crate::renderer::DisplayedBuilding;
use bevy::prelude::*;
use garden_domain::BlockId;

#[derive(Debug, Clone, Copy)]
pub struct PartHit {
    pub distance: f32,
    pub block: BlockId,
}

pub fn hit_distance(
    ray: Ray3d,
    building: &DisplayedBuilding,
    transform: &Transform,
) -> Option<f32> {
    hit_part(ray, building, transform).map(|hit| hit.distance)
}

/// Semantic block identity comes from the actually displayed chunk, not an AABB
/// or a target layout that is still waiting for upload/transition.
pub fn hit_part(
    ray: Ray3d,
    building: &DisplayedBuilding,
    transform: &Transform,
) -> Option<PartHit> {
    let inverse = transform.compute_affine().inverse();
    let origin = inverse.transform_point3(ray.origin);
    let direction = inverse.transform_vector3(*ray.direction);
    if !origin.is_finite()
        || !direction.is_finite()
        || !box_hit(origin, direction, building.min, building.max)
    {
        return None;
    }
    building
        .mesh
        .batches
        .iter()
        .flat_map(|batch| {
            batch.data.indices.chunks_exact(3).filter_map(|t| {
                let p = &batch.data.positions;
                triangle_hit(
                    origin - Vec3::from_array(batch.offset),
                    direction,
                    Vec3::from_array(p[t[0] as usize]),
                    Vec3::from_array(p[t[1] as usize]),
                    Vec3::from_array(p[t[2] as usize]),
                )
                .map(|distance| PartHit {
                    distance,
                    block: batch.id.part.block,
                })
            })
        })
        .min_by(|a, b| {
            a.distance
                .total_cmp(&b.distance)
                .then(a.block.cmp(&b.block))
        })
}
fn box_hit(origin: Vec3, direction: Vec3, min: Vec3, max: Vec3) -> bool {
    let mut near = 0.0f32;
    let mut far = f32::INFINITY;
    for axis in 0..3 {
        if direction[axis].abs() < 1e-8 {
            if origin[axis] < min[axis] || origin[axis] > max[axis] {
                return false;
            }
        } else {
            let a = (min[axis] - origin[axis]) / direction[axis];
            let b = (max[axis] - origin[axis]) / direction[axis];
            near = near.max(a.min(b));
            far = far.min(a.max(b));
            if near > far {
                return false;
            }
        }
    }
    true
}
fn triangle_hit(origin: Vec3, direction: Vec3, a: Vec3, b: Vec3, c: Vec3) -> Option<f32> {
    let edge1 = b - a;
    let edge2 = c - a;
    let h = direction.cross(edge2);
    let det = edge1.dot(h);
    if det.abs() < 1e-8 {
        return None;
    }
    let s = origin - a;
    let u = s.dot(h) / det;
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let q = s.cross(edge1);
    let v = direction.dot(q) / det;
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    let distance = edge2.dot(q) / det;
    (distance >= 0.0 && distance.is_finite()).then_some(distance)
}

#[cfg(test)]
mod tests {
    use super::*;
    use garden_domain::{Building, BuildingId, sample_building};
    use garden_generation::{
        compile,
        mesh::{GeometryProfile, compile_mesh},
    };
    use std::sync::Arc;
    #[test]
    fn exact_picking_handles_translation_rotation_and_misses() {
        let layout = compile(
            &Building::try_new(sample_building(BuildingId::new(1).unwrap(), 6.0, 3.0)).unwrap(),
        );
        let mesh = compile_mesh(&layout, GeometryProfile::default()).unwrap();
        let building = DisplayedBuilding {
            ticket: garden_application::JobTicket {
                session: 1,
                building: layout.building,
                object_revision: 1,
                rules_revision: 1,
                request_serial: 1,
            },
            target: Arc::new(layout),
            mesh: Arc::new(garden_generation::incremental::PreparedBuilding::whole(
                mesh,
            )),
            min: Vec3::new(-1.0, -1.0, -1.0),
            max: Vec3::new(7.0, 7.0, 7.0),
        };
        let transform = Transform::from_xyz(10.0, 0.0, 5.0)
            .with_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2));
        let ray = Ray3d::new(Vec3::new(13.0, 12.0, 2.0), Dir3::NEG_Y);
        let distance = hit_distance(ray, &building, &transform).unwrap();
        assert!((5.0..10.0).contains(&distance));
        assert!(
            hit_distance(
                Ray3d::new(Vec3::new(50.0, 12.0, 50.0), Dir3::NEG_Y),
                &building,
                &transform
            )
            .is_none()
        );
    }
    #[test]
    fn slab_and_triangle_reject_parallel_and_behind_hits() {
        assert!(!box_hit(
            Vec3::new(2.0, 0.0, 0.0),
            Vec3::Y,
            Vec3::ZERO,
            Vec3::ONE
        ));
        assert!(box_hit(Vec3::splat(0.5), Vec3::Y, Vec3::ZERO, Vec3::ONE));
        assert!(
            triangle_hit(
                Vec3::new(0.2, 0.2, 1.0),
                Vec3::Z,
                Vec3::ZERO,
                Vec3::X,
                Vec3::Y
            )
            .is_none()
        );
        assert!(
            triangle_hit(
                Vec3::new(0.2, 0.2, 1.0),
                Vec3::NEG_Z,
                Vec3::ZERO,
                Vec3::X,
                Vec3::Y
            )
            .is_some()
        );
    }
}
