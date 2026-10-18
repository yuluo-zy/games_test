
    #[test]
    fn sphere_options_keep_limit_target_distance_and_nearest_subworld() {
        use rapier3d::parry::query::ShapeCastOptions;
        let mut world = RaycastWorld::new();
        insert_cuboid(&mut world, 0, glam::Vec3::new(2.0, 0.0, 0.0), glam::Vec3::splat(0.5));
        insert_cuboid(&mut world, 1, glam::Vec3::new(6.0, 0.0, 0.0), glam::Vec3::splat(0.5));
        let mut options = ShapeCastOptions {
            max_time_of_impact: 10.0,
            target_distance: 0.0,
            stop_at_penetration: false,
            compute_impact_geometry_on_penetration: false,
        };
        let hit = world.spherecast_with_options(glam::Vec3::ZERO, glam::Vec3::X, 0.5, options, 3);
        assert!(hit.is_hit());
        assert!((hit.toi - 1.0).abs() < 1e-4);
        options.max_time_of_impact = 0.25;
        let miss = world.spherecast_with_options(glam::Vec3::ZERO, glam::Vec3::X, 0.5, options, 3);
        assert!(miss.is_miss());
        assert_eq!(miss.toi, 0.25);
        options.max_time_of_impact = 10.0;
        options.target_distance = 0.5;
        let expanded = world.spherecast_with_options(glam::Vec3::ZERO, glam::Vec3::X, 0.5, options, 3);
        assert!(expanded.is_hit());
        assert!((expanded.toi - 0.5).abs() < 1e-4);
    }

    #[test]
    fn sphere_terrain_uses_sweep_limit_and_preserves_existing_entry() {
        use rapier3d::parry::query::ShapeCastOptions;
        let world = Raycasting { world: RaycastWorld::new() };
        let terrain = TerrainHeightsData::<TerrainHeightSsbo>::new();
        // Outside the terrain grid, the existing terrain owner supplies its y=0 plane.
        let origin = glam::Vec3::new(100.0, 3.0, 0.0);
        let direction = glam::Vec3::NEG_Y;
        let mut options = ShapeCastOptions::with_max_time_of_impact(3.0);
        let hit = world.spherecast_with_options(&terrain, origin, direction, 0.5, options, 0x20);
        assert!(hit.is_hit() && hit.is_terrain());
        assert_eq!(hit.toi, 2.5);
        options.max_time_of_impact = 2.0;
        assert!(world.spherecast_with_options(&terrain, origin, direction, 0.5, options, 0x20).is_miss());
        assert!(world.spherecast(&terrain, origin, direction, 0.5, 0x20).is_hit());
    }

    #[test]
    fn sphere_terrain_projection_matches_original_b_wrapper_instructions() {
        // B 0x1408cc390 executed with resolved dependency query boundaries.
        // Each row: origin[3], direction[3], radius, terrain point[3], native TOI.
        let cases: &[[u32; 11]] = &[
            [0x40800000, 0x40400000, 0x40000000, 0x00000000, 0xbf800000, 0x00000000, 0x3f000000, 0x40800000, 0x00000000, 0x40000000, 0x40200000],
            [0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f000000, 0x00000000, 0x00000000, 0x00000000, 0x602d78ec],
            [0x3f800000, 0x00000000, 0x00000000, 0x0da24260, 0x00000000, 0x00000000, 0x3f000000, 0x00000000, 0x00000000, 0x00000000, 0x602d78ec],
            [0x3f800000, 0x00000000, 0x00000000, 0x1e3ce508, 0x00000000, 0x00000000, 0x3f000000, 0x00000000, 0x00000000, 0x00000000, 0x602d78ec],
            [0x3f800000, 0x00000000, 0x00000000, 0x2edbe6ff, 0x00000000, 0x00000000, 0x3f000000, 0x00000000, 0x00000000, 0x00000000, 0x4f9502f9],
            [0x3f800000, 0x00000000, 0x00000000, 0x40000000, 0x00000000, 0x00000000, 0x3f000000, 0x00000000, 0x00000000, 0x00000000, 0x3e800000],
            [0x3f800000, 0x00000000, 0x00000000, 0x7fc00000, 0x00000000, 0x00000000, 0x3f000000, 0x00000000, 0x00000000, 0x00000000, 0x602d78ec],
            [0x3f800000, 0x00000000, 0x00000000, 0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x3f800000],
            [0x3f800000, 0x00000000, 0x00000000, 0x3f800000, 0x00000000, 0x00000000, 0x3f000000, 0x00000000, 0x00000000, 0x00000000, 0x3f000000],
            [0x3f800000, 0x00000000, 0x00000000, 0x3f800000, 0x00000000, 0x00000000, 0x3f800000, 0x00000000, 0x00000000, 0x00000000, 0x00000000],
            [0x3f800000, 0x00000000, 0x00000000, 0x3f800000, 0x00000000, 0x00000000, 0x40000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000],
            [0x3f800000, 0x00000000, 0x00000000, 0x3f800000, 0x00000000, 0x00000000, 0x7fc00000, 0x00000000, 0x00000000, 0x00000000, 0x00000000],
        ];
        for row in cases {
            let origin = glam::Vec3::new(f32::from_bits(row[0]), f32::from_bits(row[1]), f32::from_bits(row[2]));
            let direction = glam::Vec3::new(f32::from_bits(row[3]), f32::from_bits(row[4]), f32::from_bits(row[5]));
            let point = glam::Vec3::new(f32::from_bits(row[7]), f32::from_bits(row[8]), f32::from_bits(row[9]));
            let actual = terrain_spherecast_time_of_impact(origin, direction, f32::from_bits(row[6]), point);
            assert_eq!(actual.to_bits(), row[10], "original sphere terrain row {row:?}");
        }
    }
