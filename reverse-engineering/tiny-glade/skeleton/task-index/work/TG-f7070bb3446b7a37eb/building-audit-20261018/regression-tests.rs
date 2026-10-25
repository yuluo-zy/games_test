

#[cfg(all(test, feature = "native-physics"))]
mod collision_query_regressions {
    use super::*;

    fn insert_cuboid(
        world: &mut RaycastWorld,
        slot_index: usize,
        center: glam::Vec3,
        half: glam::Vec3,
    ) -> SubworldColliderHandle {
        let slot = &mut world.slots[slot_index];
        let handle = slot.colliders.insert(
            ColliderBuilder::cuboid(half.x, half.y, half.z)
                .translation(Vector::new(center.x, center.y, center.z))
                .build(),
        );
        slot.pipeline.update(&slot.colliders);
        let (index, generation) = handle.into_raw_parts();
        SubworldColliderHandle { index, generation, slot: slot_index as u8 }
    }

    #[test]
    fn ray_queries_keep_nearest_subworld_hit() {
        let mut world = RaycastWorld::new();
        insert_cuboid(&mut world, 0, glam::Vec3::new(2.0, 0.0, 0.0), glam::Vec3::splat(0.5));
        insert_cuboid(&mut world, 1, glam::Vec3::new(6.0, 0.0, 0.0), glam::Vec3::splat(0.5));
        let hit = world.raycast(glam::Vec3::ZERO, glam::Vec3::X, 3);
        assert!(hit.is_hit());
        assert_eq!(hit.toi, 1.5);
        assert_eq!(world.raycast_simple(glam::Vec3::ZERO, glam::Vec3::X, 3).toi, 1.5);
    }

    #[test]
    fn legacy_parameterized_miss_preserves_integer_tag_bits() {
        let hit = RaycastWorld::new().legacy_raycast_w_params(glam::Vec3::ZERO, glam::Vec3::X, 1);
        assert!(hit.is_miss());
        assert_eq!(hit.toi.to_bits(), 2);
    }

    #[test]
    fn rectangle_overlap_keeps_center_and_orientation() {
        let mut world = RaycastWorld::new();
        insert_cuboid(&mut world, 0, glam::Vec3::new(10.0, 2.0, 22.0), glam::Vec3::splat(0.1));
        let center = glam::Vec3::new(10.0, 2.0, 20.0);
        let size = glam::Vec3::new(6.0, 2.0, 0.5);
        let rotated = utils::geometry::rectangle::Rectangle3d::new(center, size, glam::Vec2::Y);
        let unrotated = utils::geometry::rectangle::Rectangle3d::new(center, size, glam::Vec2::X);
        assert!(world.shape_overlaps_with(rotated, 1));
        assert!(!world.shape_overlaps_with(unrotated, 1));
    }

    #[test]
    fn overlap_handle_keeps_pose_and_callback_stop_rule() {
        let mut world = RaycastWorld::new();
        let center = glam::Vec3::new(10.0, 3.0, 20.0);
        let query = insert_cuboid(&mut world, 0, center, glam::Vec3::ONE);
        insert_cuboid(&mut world, 0, center, glam::Vec3::splat(0.2));
        let mut hits = 0;
        world.iter_shape_overlaps_handle(query, 1, |_, _| { hits += 1; false });
        assert_eq!(hits, 2, "false continues through all overlaps");
        hits = 0;
        world.iter_shape_overlaps_handle(query, 1, |_, _| { hits += 1; true });
        assert_eq!(hits, 1, "true stops at the first overlap");
    }
}
