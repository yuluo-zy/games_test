//! Float-word oracle from original B 1407dec90, independently executed with
//! shape-constructor boundary hooks. See phase2-decorator-lifecycle native oracle.
use country_core::systems::collision::world::RaycastMeshData;
use glam::{Vec2, Vec3};
use system_decorator::decorator_visual::stairs_assemble::{
    gen_stairs_structural_collider, structural_stair_collider_geometry,
    upsert_structural_stair_segment,
};

fn cases() -> [(Vec3, Vec3, Vec2, Vec2, f32, f32); 2] {
    [
        (
            Vec3::new(1.25, 2.5, -3.0),
            Vec3::new(4.0, -1.25, 7.5),
            Vec2::new(0.6, 0.8),
            Vec2::new(-0.8, 0.6),
            1.2,
            0.7,
        ),
        (
            Vec3::new(-2.0, 0.0, 3.0),
            Vec3::new(-2.0, 0.0, 3.0),
            Vec2::X,
            Vec2::X,
            0.0,
            0.0,
        ),
    ]
}

#[test]
fn actual_mesh_production_populates_distinct_segment_keys_and_canonical_disposal_preserves_other_segment()
 {
    use country_core::{
        resources::stairs::{StairGraphEdge, stairs_collider::StairColliderRegistry},
        systems::collision::world::{RaycastGeoSource, RaycastWorld, remove_collider},
    };
    let mut world = RaycastWorld::new();
    let mut registry = StairColliderRegistry::default();
    let edge = StairGraphEdge::new(5, 7);
    let (start, end, start_dir, end_dir, start_width, end_width) = cases()[0];
    for index in 0..2 {
        upsert_structural_stair_segment(
            &mut world,
            &mut registry,
            (edge, index),
            RaycastGeoSource::Slot0,
            start,
            end,
            start_dir,
            end_dir,
            start_width,
            end_width,
        );
    }
    assert_eq!(registry.structural_colliders.len(), 2);
    let first = registry.structural_colliders[&(edge, 0)];
    let second = registry.structural_colliders[&(edge, 1)];
    assert_ne!(first.collider, second.collider);
    assert!(
        world
            .get_rapier_collider(first.slot, first.collider)
            .is_some()
    );
    assert!(
        world
            .get_rapier_collider(second.slot, second.collider)
            .is_some()
    );
    remove_collider(&mut world, &mut registry.structural_colliders, &(edge, 0)).unwrap();
    assert!(
        world
            .get_rapier_collider(first.slot, first.collider)
            .is_none()
    );
    assert!(
        world
            .get_rapier_collider(second.slot, second.collider)
            .is_some()
    );
    assert_eq!(registry.structural_colliders.len(), 1);
    remove_collider(&mut world, &mut registry.structural_colliders, &(edge, 1)).unwrap();
    assert!(registry.structural_colliders.is_empty());
}

#[test]
fn dirty_edge_removal_disposes_all_three_native_lanes_without_touching_another_edge() {
    use country_core::{
        resources::stairs::{StairGraphEdge, stairs_collider::StairColliderRegistry},
        systems::collision::world::{RaycastGeoSource, RaycastWorld, upsert_collider},
    };
    let mut world = RaycastWorld::new();
    let mut registry = StairColliderRegistry::default();
    let edge = StairGraphEdge::new(5, 7);
    let other = StairGraphEdge::new(6, 8);
    for map in &mut registry.edge_colliders {
        for key in [edge, other] {
            upsert_collider(
                &mut world,
                map,
                key,
                RaycastGeoSource::Slot0,
                std::sync::Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::ZERO)),
                Vec3::ZERO,
                glam::Quat::IDENTITY,
            );
        }
    }
    let old: Vec<_> = registry.edge_colliders.iter().map(|m| m[&edge]).collect();
    let removed = registry.remove_edge_colliders(&mut world, edge);
    assert!(removed.iter().all(Option::is_some));
    for (index, map) in registry.edge_colliders.iter().enumerate() {
        assert!(!map.contains_key(&edge));
        assert!(
            world
                .get_rapier_collider(old[index].slot, old[index].collider)
                .is_none()
        );
        let kept = map[&other];
        assert!(
            world
                .get_rapier_collider(kept.slot, kept.collider)
                .is_some()
        );
    }
    assert_eq!(registry.remove_edge_colliders(&mut world, edge), [None; 3]);
    registry.remove_edge_colliders(&mut world, other);
}

#[test]
fn all_eight_vertices_and_fallback_words_match_original_simd_execution() {
    let bytes = include_bytes!("fixtures/native_structural_geometry.bin");
    assert_eq!(bytes.len(), 2 * 28 * 4);
    for (index, (start, end, start_dir, end_dir, start_width, end_width)) in
        cases().into_iter().enumerate()
    {
        let (points, center) = structural_stair_collider_geometry(
            start,
            end,
            start_dir,
            end_dir,
            start_width,
            end_width,
        );
        let got: Vec<u32> = points
            .into_iter()
            .flat_map(|p| p.to_array().map(f32::to_bits))
            .chain(center.to_array().map(f32::to_bits))
            .chain([0x3727_c5ac])
            .collect();
        let expected: Vec<u32> = bytes[index * 112..(index + 1) * 112]
            .chunks_exact(4)
            .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
            .collect();
        assert_eq!(got, expected, "original geometry words, case {index}");
    }
}

#[test]
fn actual_convex_keeps_fractional_negative_coordinates_and_degenerate_case_uses_native_sphere() {
    let (start, end, start_dir, end_dir, start_width, end_width) = cases()[0];
    let mesh =
        gen_stairs_structural_collider(start, end, start_dir, end_dir, start_width, end_width);
    let aabb = mesh.shape_ref().unwrap().compute_local_aabb();
    assert!(aabb.mins.x < 1. && aabb.mins.z < -3.);
    assert!(aabb.maxs.x < 5. && aabb.maxs.z < 8.);
    let (start, end, start_dir, end_dir, start_width, end_width) = cases()[1];
    let (points, center) =
        structural_stair_collider_geometry(start, end, start_dir, end_dir, start_width, end_width);
    assert!(RaycastMeshData::new_convex(&points).is_none());
    let fallback =
        gen_stairs_structural_collider(start, end, start_dir, end_dir, start_width, end_width);
    assert_eq!(fallback.local_transform().translation, center);
    let aabb = fallback.shape_ref().unwrap().compute_local_aabb();
    assert_eq!(aabb.maxs.x.to_bits(), 0x3727_c5ac);
    assert_eq!(aabb.mins.x.to_bits(), 0xb727_c5ac);
}
