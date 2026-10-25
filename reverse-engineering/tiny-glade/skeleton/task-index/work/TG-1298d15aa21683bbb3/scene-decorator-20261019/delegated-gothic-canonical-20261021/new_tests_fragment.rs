    #[test]
    fn brick_payloads_enter_canonical_components_with_all_words_and_float_bits_preserved() {
        use bevy_ecs::world::{CommandQueue, World};
        use country_core::components::{BrickColor as CoreBrick, GothicWindowBrickDeformation as CoreDef};
        assert_eq!(core::any::TypeId::of::<pending_core::BrickColor>(), core::any::TypeId::of::<CoreBrick>());
        assert_eq!(core::any::TypeId::of::<pending_core::GothicWindowBrickDeformation>(), core::any::TypeId::of::<CoreDef>());
        let brick = [0xdead_beef, 42, 0x8000_0000, 0x7fc0_0123, 0xff80_0000, 0xaabb_cc01_u32];
        let deformation = [0x8000_0000, 0x7fc0_0123, 0xff80_0000, 0x3f80_0000, 0x4000_0000,
            0x4040_0000, 0x4080_0000, 0x40a0_0000, 0x40c0_0000, 0x40e0_0000,
            0x4100_0000, 0x4110_0000, 0x4120_0000, 0x4130_0000, 0x4140_0000_u32];
        let mut world = World::new(); let mut queue = CommandQueue::default();
        let mut state = DecoratorTransientVisualState::default();
        let mut builder = WindowVisualEntityBuilder::new(Vec3::ONE, Quat::IDENTITY, 71);
        builder.mesh_name = "decorators/window_gothic_2x1_hat";
        builder.nani_name = "nani_gothic_window_bricks";
        builder.brick_color = Some(brick.map(u32::to_le_bytes).concat().try_into().unwrap());
        builder.gothic_window_brick_deformation = Some(deformation.map(u32::to_le_bytes).concat().try_into().unwrap());
        let entity = builder.build(&mut Commands::new(&mut queue, &world), 71,
            &AssetMeshLibrary::new(), &AssetShaderLibrary::new(), &mut state);
        assert!(world.get::<CoreBrick>(entity).is_none());
        queue.apply(&mut world);
        assert_eq!(world.get::<CoreBrick>(entity).unwrap().words(), brick);
        assert_eq!(world.get::<CoreDef>(entity).unwrap().words(), deformation);
        assert!(world.get::<InstanceDataMarker<GothicWindowBricksInstanceData>>(entity).is_some());
        let count = world.query::<(&CoreBrick, &CoreDef)>().iter(&world).count();
        assert_eq!(count, 1);
    }

    #[test]
    fn original_brick_niche_two_and_absent_deformation_are_not_inserted_by_builder() {
        use bevy_ecs::world::{CommandQueue, World};
        use country_core::components::{BrickColor as CoreBrick, GothicWindowBrickDeformation as CoreDef};
        for payload in [None, Some([0, 0, 0, 0, 0, 2_u32])] {
            let mut world = World::new(); let mut queue = CommandQueue::default();
            let mut state = DecoratorTransientVisualState::default();
            let mut builder = WindowVisualEntityBuilder::new(Vec3::ZERO, Quat::IDENTITY, 73);
            builder.nani_name = "nani_solid_vertex_color_bricks";
            builder.brick_color = payload.map(|words| words.map(u32::to_le_bytes).concat().try_into().unwrap());
            let entity = builder.build(&mut Commands::new(&mut queue, &world), 73,
                &AssetMeshLibrary::new(), &AssetShaderLibrary::new(), &mut state);
            queue.apply(&mut world);
            assert!(world.get::<CoreBrick>(entity).is_none());
            assert!(world.get::<CoreDef>(entity).is_none());
        }
    }

    #[test]
    fn real_builder_canonical_bricks_reach_the_same_world_owned_nani_rows() {
        use bevy_ecs::{schedule::{ApplyDeferred, IntoScheduleConfigs, Schedule}, world::{CommandQueue, World}};
        use country_core::{render::{mesh::Mesh,
            nani::{gpu_world::{GpuWorld, GpuWorldSubset, InstanceDataBuffer, InstanceKey, OwnedGpuWorldSubset},
                systems::{nani_maintain_instances_ecs, nani_update_solid_vertex_color_bricks_ecs,
                    nani_update_gothic_window_bricks_ecs}}}, resources::render::mesh_atlas_library::MeshAtlasLibrary};
        let device = country_core::render::test_device::TestDevice(73);
        let mut gpu = GpuWorld::new(&device);
        gpu.assign_subset(GpuWorldSubsetKind::SolidVertexColorBricks, GpuWorldSubset::Owned(
            OwnedGpuWorldSubset::new::<SolidVertexColorBricksInstanceData>(&device, GpuWorldSubsetKind::SolidVertexColorBricks)));
        gpu.assign_subset(GpuWorldSubsetKind::GothicWindowBricks, GpuWorldSubset::Owned(
            OwnedGpuWorldSubset::new::<GothicWindowBricksInstanceData>(&device, GpuWorldSubsetKind::GothicWindowBricks)));
        let mut atlas = MeshAtlasLibrary::default();
        let mut mesh = Mesh::new("同World砖窗三角形夹具");
        mesh.set_attribute(country_core::render::mesh::ATTR_POSITION, vec![Vec3::X, Vec3::Y, Vec3::Z]);
        mesh.set_indices(vec![0, 1, 2]);
        atlas.add(GpuWorldSubsetKind::SolidVertexColorBricks, "decorators/window_gothic_1x1_hat", &mesh);
        atlas.add(GpuWorldSubsetKind::GothicWindowBricks, "decorators/window_gothic_2x1_hat", &mesh);
        let mut world = World::new(); world.insert_resource(gpu); world.insert_resource(atlas);
        let brick = [0xdead_beef, 42, 1.0_f32.to_bits(), 2.0_f32.to_bits(), 0x3fbf_3e03, 0xaabb_cc01_u32];
        let deformation = [1.0_f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
        let mut queue = CommandQueue::default(); let mut state = DecoratorTransientVisualState::default();
        let mut entities = Vec::new();
        for gothic in [false, true] {
            let mut builder = WindowVisualEntityBuilder::new(Vec3::new(3.0, 4.0, 5.0), Quat::IDENTITY, 79);
            builder.mesh_name = if gothic { "decorators/window_gothic_2x1_hat" } else { "decorators/window_gothic_1x1_hat" };
            builder.nani_name = if gothic { "nani_gothic_window_bricks" } else { "nani_solid_vertex_color_bricks" };
            builder.brick_color = Some(brick.map(u32::to_le_bytes).concat().try_into().unwrap());
            if gothic { builder.gothic_window_brick_deformation = Some(deformation.map(f32::to_le_bytes).concat().try_into().unwrap()); }
            entities.push(builder.build(&mut Commands::new(&mut queue, &world), 79,
                &AssetMeshLibrary::new(), &AssetShaderLibrary::new(), &mut state));
        }
        queue.apply(&mut world);
        let mut schedule = Schedule::default();
        schedule.add_systems((nani_maintain_instances_ecs, ApplyDeferred,
            (nani_update_solid_vertex_color_bricks_ecs, nani_update_gothic_window_bricks_ecs)).chain_ignore_deferred());
        schedule.run(&mut world);
        let solid_key = *world.get::<InstanceKey>(entities[0]).unwrap();
        let gothic_key = *world.get::<InstanceKey>(entities[1]).unwrap();
        let gpu = world.resource::<GpuWorld>();
        let solid = gpu.owned_subset_mut(GpuWorldSubsetKind::SolidVertexColorBricks);
        let raw = solid.data().as_any().downcast_ref::<InstanceDataBuffer<SolidVertexColorBricksInstanceData>>()
            .unwrap().shadow.data[solid_key.index as usize].raw;
        assert_eq!(&raw[12..], &[0x4000_3c00, 0xdead_beef, 0x8000_002a, 0x3fbf_3e03]);
        assert_eq!(solid.instance_count(), 1);
        drop(solid);
        let gothic = gpu.owned_subset_mut(GpuWorldSubsetKind::GothicWindowBricks);
        let raw = gothic.data().as_any().downcast_ref::<InstanceDataBuffer<GothicWindowBricksInstanceData>>()
            .unwrap().shadow.data[gothic_key.index as usize].raw;
        assert_eq!(&raw[12..16], &[0xdead_beef, 42, 0x4000_3c00, 1]);
        assert_eq!(&raw[16..19], &deformation[6..9].iter().map(|v| v.to_bits()).collect::<Vec<_>>());
        assert_eq!(raw[19], 0x4000_3c00);
        assert_eq!(&raw[20..23], &deformation[9..12].iter().map(|v| v.to_bits()).collect::<Vec<_>>());
        assert_eq!(raw[23], 0x4400_4200);
        assert_eq!(&raw[24..27], &deformation[12..15].iter().map(|v| v.to_bits()).collect::<Vec<_>>());
        assert_eq!(raw[27], 0x4600_4500);
        assert_eq!(gothic.instance_count(), 1);
        assert_eq!(gpu.total_instance_count, 2);
    }
