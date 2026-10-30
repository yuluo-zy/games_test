//! 原B1407c0910/5224B：屋顶烟囱的烟雾、共享碰撞与40B砖装配输出。
//! 原参数六组；所有资源取自同一个World，不使用旧scalar chimney兼容槽。
//! 常量与输出逐写面见本轮m01-hints-chimney-roof-1407c0910-full.asm。

use bevy_ecs::prelude::*;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources;
use country_core::resources::walls::public_walls::PublicWalls;

use crate::blueprint::DecoratorBlueprints;
use crate::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams;
use crate::utils::visual_entity_builder::WindowVisualEntityBuilder;
use country_core::systems::ui::signifiers::UiSignifierStream;
use glam::Vec3;

/// 屋顶烟囱子类型（O：0x1407c0910 行 501）。
pub const ROOF_CHIMNEY_SUBTYPE: u8 = 0x15;
/// 原1407c1233的XMM8来自3.0；旧版误把另一常量0.1当作该操作数。
pub const CHIMNEY_HEIGHT_EXTRA: f32 = 3.0;
/// 立方体截面/高（O：0x3f59999a = 0.85）。
pub const CHIMNEY_CUBOID_SIZE: f32 = 0.85;
/// 视觉 nani 名（O：0x142c21224，len 10）。
pub const CHIMNEY_NANI_SMOKE: &str = "nani_smoke";

/// 屋顶烟囱几何参数（O 公式）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RoofChimney {
    /// 立方体尺寸（两轴 0.85，高度轴另计）。
    pub cuboid: [f32; 3],
    /// 烟囱总高 = extra.f + 3.0。
    pub height: f32,
    /// 烟雾字节（`ChimneyExtra::smoke`，O：+4 字段）。
    pub smoke: u8,
}

/// 由 `ChimneyExtra` 计算屋顶烟囱几何（O）。
pub fn roof_chimney_geometry(extra_height: f32, smoke: u8) -> RoofChimney {
    RoofChimney {
        cuboid: [
            CHIMNEY_CUBOID_SIZE,
            extra_height + CHIMNEY_HEIGHT_EXTRA,
            CHIMNEY_CUBOID_SIZE,
        ],
        height: extra_height + CHIMNEY_HEIGHT_EXTRA,
        smoke,
    }
}

/// 0x1407c0910（O 结构）：生成屋顶烟囱装饰视觉。
pub fn generate_roof_chimneys(
    derived: DerivedDecoratorInfoResources,
    decorator_storage: Res<DecoratorStorage>,
    cached_transforms: Res<CachedDecoratorTransforms>,
    walls: Res<PublicWalls>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    _signifiers: Res<UiSignifierStream>,
) {
    let derived = derived.as_view();
    use country_core::resources::walls::decorator_storage::DecoratorStorageAddr;
    // 原140a1e5b0先墙桶，再两个inline桶；旧iter()兼容接口只枚举墙，不能复用。
    let buckets = decorator_storage.per_wall.iter()
        .map(|(&wall, bucket)| (DecoratorStorageAddr::Wall(wall), bucket))
        .chain(decorator_storage.terrain.iter().map(|bucket| (DecoratorStorageAddr::Terrain, bucket)))
        .chain(decorator_storage.stairs.iter().map(|bucket| (DecoratorStorageAddr::Stair, bucket)));
    for (addr, decorators) in buckets {
        for &id in &decorators.modified_ids {
            let info = decorators.get(country_core::resources::walls::decorator::DecoratorId(id))
                .expect("modified roof chimney is missing from its storage");
            // 原112a先required读子类型；完整原body没有额外目的地tag前门。
            if derived
                .subtype(country_core::resources::walls::decorator::DecoratorId(id))
                .0
                != ROOF_CHIMNEY_SUBTYPE
            {
                continue;
            }
            let transform = cached_transforms.transform(id);
            let blueprint = blueprints.insert_for(
                id,
                addr,
                info.state(),
            );
            // 原11a7..11e8先写outline，再检查真实Wall owner与extra。
            blueprint.set_cached_transform(transform);
            let wall = walls.get(addr.as_wall()).expect("roof chimney requires its public wall");
            let extra = info.extra().as_chimney();
            let chimney = roof_chimney_geometry(extra.f, extra.smoke);
            let mut smoke = WindowVisualEntityBuilder::new(
                Vec3::new(transform.translation.x, chimney.height + transform.translation.y,
                    transform.translation.z), transform.rotation, id);
            smoke.nani_name = CHIMNEY_NANI_SMOKE;
            smoke.scale = transform.scale;
            smoke.smoke = Some((0, extra.smoke));
            smoke.semi_transparent = info.state() != 0;
            blueprint.visual_entities.push(smoke);
            let second_base_y = cached_transforms.transform(id).translation.y;
            let wall_max_y = wall.max_y();
            // 原局部cuboid中心为零，世界中心沿Y抬高半个实际烟囱高度。
            let collider = std::sync::Arc::new(
                country_core::systems::collision::world::RaycastMeshData::new_cuboid_at(
                    glam::Vec3::from_array(chimney.cuboid),
                    glam::Vec3::ZERO,
                ),
            );
            let position = glam::Vec3::new(
                transform.translation.x,
                transform.translation.y + chimney.height * 0.5,
                transform.translation.z,
            );

            blueprint.set_positioned_colliders(
                std::sync::Arc::clone(&collider),
                collider,
                &position,
                &transform.rotation,
            );
            // 原16c5/1770保留中心加减半高的舍入，不能直接用原baseY替代。
            let half = chimney.height * 0.5;
            let base = position.y - half;
            let axis = transform.rotation * Vec3::X;
            blueprint.chimney_assembly = Some(Box::new(ChimneyAssemblyParams { words: [
                2,
                (second_base_y - wall_max_y).to_bits(),
                position.x.to_bits(), position.z.to_bits(), base.to_bits(),
                (half + position.y).to_bits(), axis.x.to_bits(), axis.z.to_bits(),
                info.u28, (base + 3.0).to_bits(),
            ] }));
        }
    }
}

/// 判别（O：dst=屋顶 且 subtype=0x15）。
pub fn is_roof_chimney(dst_tag: u8, subtype: u8) -> bool {
    dst_tag == 2 && subtype == ROOF_CHIMNEY_SUBTYPE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geometry_matches_machine_constants() {
        let c = roof_chimney_geometry(1.4, 3);
        assert_eq!(c.height.to_bits(), 4.4f32.to_bits());
        assert_eq!(c.cuboid, [0.85, 4.4, 0.85]);
        assert_eq!(c.smoke, 3);
    }

    #[test]
    fn predicate_matches_decomp() {
        assert!(is_roof_chimney(2, 0x15));
        assert!(!is_roof_chimney(2, 0x13));
        assert!(!is_roof_chimney(0, 0x15));
    }
}


#[cfg(test)]
pub(crate) mod original_fixture {
    include!(concat!(env!("CARGO_MANIFEST_DIR"),
        "/../../../reports/module-recovery-2026-10-23/m01-hints-chimney-original-cases.rs"));
}

#[cfg(test)]
mod canonical_roof_collider_regressions {
    use super::*;
    use crate::blueprint::BlueprintMeshSlot;
    use bevy_ecs::system::RunSystemOnce;
    use country_core::resources::walls::{
        WallId, InnerWalls, WallState, publication_source::TerrainHeightGrid512,
        cached_decorator_transforms::{CachedDecoratorTransform, CachedDecoratorTransforms},
        decorator::{ChimneyExtra, DecoratorExtraInfo, DecoratorId, DecoratorInfo},
        decorator_storage::{DecoratorStorage, Decorators},
        decorator_subtype::{DecoratorMeshLibrary, DecoratorSubtype, DerivedDecoratorSubtypes},
    };
    use glam::{Quat, Vec3};
    use std::sync::Arc;

    fn roof_world() -> World {
        let mut world = World::new();
        let mut info = DecoratorInfo {
            decorator_id: 9,
            u20: 2 << 16,
            rank: 1,
            state: 1,
            ..Default::default()
        };
        info.set_extra(DecoratorExtraInfo::Chimney(ChimneyExtra {
            f: 1.4,
            smoke: 3,
        }));
        let mut bucket = Decorators::default();
        bucket.insert(DecoratorId(9), info);
        let mut storage = DecoratorStorage::default();
        storage.per_wall.insert(WallId(2), bucket);
        world.insert_resource(storage);
        let transform = CachedDecoratorTransform {
            rotation: Quat::from_rotation_y(0.4),
            translation: Vec3::new(4., 7., -3.),
            scale: Vec3::ONE,
        };
        let mut cached = CachedDecoratorTransforms::default();
        cached.insert(9, transform);
        world.insert_resource(cached);
        let mut subtypes = DerivedDecoratorSubtypes::default();
        subtypes.insert(9, DecoratorSubtype(21));
        world.insert_resource(subtypes);
        // 动态烟囱不读取loader的球体记录，空库仍必须生成真实cuboid。
        world.insert_resource(DecoratorMeshLibrary::default());
        let mut inner = InnerWalls::new();
        inner.new_wall_with_id(WallId(2), WallState::new(
            country_core::resources::roofs::geometry::surface::Curve2::try_new(
                vec![[0.0, 0.0], [8.0, 0.0]], false).unwrap(), 8.0, 0, 0, false));
        let mut public = PublicWalls::default();
        public.consume_with_paths(&mut inner, &TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(), minimum_y: 0.0, maximum_y: 0.0,
            sampling_fix: false,
        }, None, &Default::default(), &Default::default());
        let paths = country_core::systems::wall::wall_state::path_preparation::PathRasters512::from_encoded(
            std::array::from_fn(|_| Arc::from(vec![0u8; 512 * 512])), false).unwrap();
        public.get_mut(WallId(2)).unwrap().prepare_paths(&paths).unwrap();
        world.insert_resource(public);
        world.init_resource::<UiSignifierStream>();
        world.insert_resource(DecoratorBlueprints::default());
        world
    }

    #[test]
    fn roof_chimney_uses_native_y_height_zero_local_center_and_shared_arc() {
        let mut world = roof_world();
        let transform = world.resource::<CachedDecoratorTransforms>().transform(9);
        world.run_system_once(generate_roof_chimneys).unwrap();
        let blueprints = world.resource::<DecoratorBlueprints>();
        let blueprint = &blueprints.map[&9];
        assert_eq!(blueprint.storage_addr, Some(country_core::resources::walls::decorator_storage::DecoratorStorageAddr::Wall(WallId(2))));
        assert_eq!(blueprint.u138, 1);
        assert_eq!(blueprint.interaction_source_flags, 0);
        let BlueprintMeshSlot::Mesh(collision) = &blueprint.collision_mesh else {
            panic!("实际烟囱collision")
        };
        let BlueprintMeshSlot::Mesh(interaction) = &blueprint.interaction_mesh else {
            panic!("实际烟囱interaction")
        };
        assert!(Arc::ptr_eq(&collision.mesh, &interaction.mesh));
        assert_eq!(
            collision
                .mesh
                .shape_ref()
                .unwrap()
                .as_cuboid()
                .unwrap()
                .half_extents
                .as_slice(),
            &[0.425, 2.2, 0.425]
        );
        assert_eq!(collision.mesh.local_transform().translation, Vec3::ZERO);
        assert_eq!(collision.position, Vec3::new(4., 9.2, -3.));
        assert_eq!(collision.rotation, transform.rotation);
        assert_eq!(interaction.position, collision.position);
        assert_eq!(interaction.rotation, collision.rotation);
        assert!(blueprint.colliders.is_empty());
        assert_eq!(blueprint.transform, Some(transform));
        let smoke = &blueprint.visual_entities[0];
        assert_eq!(smoke.nani_name, "nani_smoke");
        assert_eq!(smoke.translation, Vec3::new(4., 11.4, -3.));
        assert_eq!(smoke.smoke, Some((0, 3)));
        assert_eq!(blueprint.chimney_assembly.as_ref().unwrap().words[0], 2);
        assert!(blueprint.chimney.is_none());
    }

    #[test]
    fn complete_roof_outputs_match_original_builder_and_assembly_instruction_ranges() {
        let mut world = roof_world();
        for (case, (input, expected_smoke, expected_center, expected_params)) in
            super::original_fixture::ROOF_CASES.iter().enumerate() {
            let transform = CachedDecoratorTransform {
                rotation: Quat::from_array(std::array::from_fn(|i| f32::from_bits(input[i]))),
                translation: Vec3::from_array(std::array::from_fn(|i| f32::from_bits(input[i + 4]))),
                scale: Vec3::from_array(std::array::from_fn(|i| f32::from_bits(input[i + 7]))),
            };
            world.resource_mut::<CachedDecoratorTransforms>().insert(9, transform);
            {
                let mut storage = world.resource_mut::<DecoratorStorage>();
                let bucket = storage.per_wall.get_mut(&WallId(2)).unwrap();
                let mut info = *bucket.get(DecoratorId(9)).unwrap();
                info.set_extra(DecoratorExtraInfo::Chimney(ChimneyExtra {
                    f: f32::from_bits(input[10]), smoke: input[11] as u8,
                }));
                info.state = input[12] as u8;
                info.u28 = input[13];
                bucket.insert(DecoratorId(9), info);
            }
            world.run_system_once(generate_roof_chimneys).unwrap();
            let blueprints = world.resource::<DecoratorBlueprints>();
            let blueprint = blueprints.get(9).unwrap();
            let smoke = &blueprint.visual_entities[0];
            let actual_smoke: Vec<_> = smoke.rotation.to_array().into_iter()
                .chain(smoke.translation.to_array()).chain(smoke.scale.to_array())
                .map(f32::to_bits).collect();
            assert_eq!(actual_smoke, expected_smoke, "roof smoke case {case}");
            assert_eq!(smoke.smoke, Some((0, input[11] as u8)));
            assert_eq!(smoke.semi_transparent, input[12] != 0);
            assert_eq!(blueprint.chimney_assembly.as_ref().unwrap().words,
                *expected_params, "roof assembly case {case}");
            let BlueprintMeshSlot::Mesh(collider) = &blueprint.collision_mesh else { panic!() };
            assert_eq!(collider.position.to_array().map(f32::to_bits), *expected_center,
                "roof collider case {case}");
        }
    }

    #[test]
    fn real_world_generators_fullvisual_bricks_smoke_upload_and_removal_share_owners() {
        use bevy_ecs::schedule::{ApplyDeferred, IntoScheduleConfigs, Schedule};
        use country_core::{
            asset_libraries::{ssbo_library::AssetSsboLibrary, texture_library::AssetTextureLibrary,
                shader_library::{AssetShaderLibrary, SharedAssetShaderLibrary}, mesh_library::MeshAsset},
            components::Smoke,
            geometry::instanced_wall::{BrickTransformSsbo, InstancedWall},
            render::{mesh::{Mesh, ATTR_POSITION}, sparse_instance_buffer::SparseInstanceBuffer,
                test_device::TestDevice, ssbo::rhapsody_boundary::{GraphicsDevice, Buffer, BufferDesc, BackendError},
                nani::{gpu_world::{GpuWorld, GpuWorldSubset, GpuWorldSubsetKind, OwnedGpuWorldSubset, InstanceDataBuffer, InstanceKey},
                    instance_data::{SmokeInstanceData, SolidVertexColorInstanceData},
                    systems::{nani_maintain_instances_ecs, nani_update_smoke_ecs,
                        nani_update_solid_vertex_color_ecs, nani_flush_instance_updates}}},
            resources::{render::mesh_atlas_library::MeshAtlasLibrary,
                walls::{decorator_collision::DecoratorPhysicsColliders, decorator_storage::DecoratorStorageAddr,
                    decorator_subtype::{DecoratorCollision, DecoratorLibKey, DecoratorCollisionOutline, DecoratorOutline},
                    public_walls::{WallEntities, OnWallDeleted}, wall_holes::WallHoles},
                terrain_height::{TerrainHeightsData, TerrainHeightSsbo}, time::GameTime, time_of_day::TimeOfDay},
            systems::{audio::PlayDecoratorSoundCmd, collision::{WindowAutoClutterCandidates,
                world::{RaycastMeshData, RaycastWorld}}},
        };
        use crate::{blueprint::{clear_blueprints, instantiate_blueprints, copy_blueprints_to_prev,
                PrevDecoratorBlueprints}, decorator_visual_state::DecoratorVisualState,
            decorator_visual::{chimney_wall::generate_wall_chimneys,
                chimney_assemble_bricks::{chimney_assemble_bricks, ChimneyAssemblyParams},
                deco_differential_state::{WallAttachedDecoTracker, DoorStairsAssemblyParams}},
            chimney_smoke_anim_thickness::{chimney_smoke_anim_thickness, ChimneySmokeThickness}};

        // 仅校验CPU真实owner、写入范围与回收；TestDevice无本机GPU提交能力。
        struct CheckedDevice { backing: TestDevice, writes: std::cell::Cell<usize> }
        impl GraphicsDevice for CheckedDevice {
            fn create_buffer(&self, d: &BufferDesc, n: &str, b: Option<&[u8]>) -> Result<Buffer, BackendError> {
                assert!(b.is_none_or(|bytes| bytes.len() <= d.size));
                self.backing.create_buffer(d, n, b)
            }
            fn mapped_slice_mut(&self, b: &Buffer) -> Option<&mut [u8]> { self.backing.mapped_slice_mut(b) }
            fn update_buffer_bytes(&self, b: &Buffer, data: &[u8], ranges: &[std::ops::Range<usize>], offset: usize) -> Result<(), BackendError> {
                for range in ranges {
                    assert!(range.start <= range.end && range.end <= data.len());
                    assert!(offset.checked_add(range.end).unwrap() as u64 <= b.byte_size);
                }
                self.writes.set(self.writes.get() + 1);
                Ok(())
            }
        }
        for state in [0, 1] {
        let device = CheckedDevice { backing: TestDevice(91), writes: Default::default() };
        let mut world = roof_world();
        world.resource_mut::<DecoratorStorage>().per_wall.get_mut(&WallId(2)).unwrap()
            .get_mut(DecoratorId(9)).unwrap().state = state;
        let mut gpu = GpuWorld::new(&device);
        gpu.assign_subset(GpuWorldSubsetKind::Smoke, GpuWorldSubset::Owned(
            OwnedGpuWorldSubset::new::<SmokeInstanceData>(&device, GpuWorldSubsetKind::Smoke, 8)));
        gpu.assign_subset(GpuWorldSubsetKind::SolidVertexColor, GpuWorldSubset::Owned(
            OwnedGpuWorldSubset::new::<SolidVertexColorInstanceData>(&device, GpuWorldSubsetKind::SolidVertexColor, 8)));
        let mut triangle = Mesh::new("CPU原输入三角形，不替代生产资产");
        triangle.set_attribute(ATTR_POSITION, vec![Vec3::ZERO, Vec3::X, Vec3::Y]);
        triangle.set_indices(vec![0, 1, 2]);
        let mut atlas = MeshAtlasLibrary::default();
        atlas.add(GpuWorldSubsetKind::Smoke, "smoke", &triangle);
        atlas.add(GpuWorldSubsetKind::SolidVertexColor, "decorators/vent_pipe", &triangle);
        world.insert_resource(atlas);
        world.insert_resource(gpu);
        let assets = country_core::asset_libraries::mesh_library::AssetMeshLibrary::new();
        assets.add(MeshAsset::new(triangle, Some("decorators/vent_pipe_collision".into())));
        world.insert_resource(assets);
        world.insert_resource(SharedAssetShaderLibrary(Arc::new(AssetShaderLibrary::new())));
        world.insert_resource(AssetTextureLibrary::new(&device.backing));
        world.init_resource::<PrevDecoratorBlueprints>();
        world.init_resource::<DecoratorVisualState>();
        world.init_resource::<WallHoles>();
        world.init_resource::<WindowAutoClutterCandidates>();
        world.insert_resource(RaycastWorld::new());
        world.init_resource::<DecoratorPhysicsColliders>();
        world.init_resource::<WallAttachedDecoTracker<ChimneyAssemblyParams>>();
        world.init_resource::<WallAttachedDecoTracker<DoorStairsAssemblyParams>>();
        world.init_resource::<Events<OnWallDeleted>>();
        world.init_resource::<Events<PlayDecoratorSoundCmd>>();
        world.init_resource::<ChimneySmokeThickness>();
        world.init_resource::<TimeOfDay>();
        world.resource_mut::<TimeOfDay>().time_of_day = 0.1;
        world.init_resource::<GameTime>();
        world.resource_mut::<GameTime>().delta = 0.125;
        world.insert_resource(TerrainHeightsData::<TerrainHeightSsbo>::new());
        let sparse = SparseInstanceBuffer::<BrickTransformSsbo>::try_new(&device).unwrap();
        let owner = world.spawn(InstancedWall::new_awaiting_production(2, &sparse)).id();
        let library = AssetSsboLibrary::new(); library.register::<BrickTransformSsbo>(sparse);
        world.insert_resource(library);
        let mut entities = WallEntities::default(); entities.0.insert(WallId(2), owner);
        world.insert_resource(entities);
        for (id, subtype, u) in [(41, 19, 2.0f32), (42, 20, 7.0f32)] {
            let mut info = DecoratorInfo { decorator_id: id, dst_wall:
                u64::from(u.to_bits()) | (u64::from(2.0f32.to_bits()) << 32), rank: 1, state,
                ..Default::default() };
            info.set_extra(DecoratorExtraInfo::Chimney(ChimneyExtra { f: 1.0, smoke: 1 }));
            world.resource_mut::<DecoratorStorage>().per_wall.get_mut(&WallId(2)).unwrap().insert(DecoratorId(id), info);
            world.resource_mut::<DecoratorStorage>().decorator_to_storage.insert(
                DecoratorId(id), country_core::resources::walls::decorator_storage::DecoratorStorageAddr::Wall(WallId(2)));
            world.resource_mut::<DerivedDecoratorSubtypes>().insert(id, DecoratorSubtype(subtype));
            world.resource_mut::<CachedDecoratorTransforms>().insert(id, CachedDecoratorTransform {
                rotation: Quat::IDENTITY, translation: Vec3::new(u, 2.0, 0.0), scale: Vec3::ONE });
            let collision = DecoratorCollision { triangles: vec![],
                mesh: Arc::new(RaycastMeshData::new_cuboid_at(Vec3::ONE, Vec3::ZERO)),
                bounds: utils::geometry::Rectangle3d::new(Vec3::ZERO, Vec3::ONE, glam::Vec2::X) };
            world.resource_mut::<DecoratorMeshLibrary>().insert(DecoratorLibKey::new(subtype, 1),
                collision.clone(), collision, DecoratorOutline::None, DecoratorCollisionOutline::None);
        }
        let mut schedule = Schedule::default();
        schedule.add_systems((clear_blueprints, (generate_roof_chimneys, generate_wall_chimneys),
            instantiate_blueprints, chimney_assemble_bricks, copy_blueprints_to_prev,
            ApplyDeferred, chimney_smoke_anim_thickness, nani_maintain_instances_ecs,
            ApplyDeferred, (nani_update_smoke_ecs, nani_update_solid_vertex_color_ecs)).chain_ignore_deferred());
        schedule.run(&mut world);
        let smoke_entities: Vec<_> = world.query::<(Entity, &Smoke)>().iter(&world)
            .map(|(entity, _)| entity).collect();
        assert_eq!(smoke_entities.len(), 3);
        for &entity in &smoke_entities {
            let smoke = world.get::<Smoke>(entity).unwrap();
            let key = world.get::<InstanceKey>(entity).unwrap();
            let gpu = world.resource::<GpuWorld>();
            let subset = gpu.owned_subset_mut(GpuWorldSubsetKind::Smoke);
            let data = subset.data().as_any().downcast_ref::<InstanceDataBuffer<SmokeInstanceData>>().unwrap();
            assert_eq!(&data.shadow.data[key.index as usize].raw[12..],
                &[u32::from(smoke.kind), smoke.key as u32, smoke.thickness.to_bits(), 0]);
        }
        let allocation = world.get::<InstancedWall<BrickTransformSsbo>>(owner).unwrap().chimney_allocation().unwrap();
        assert!(!world.resource::<AssetSsboLibrary>().get::<BrickTransformSsbo>().unwrap()
            .allocation_to_draw_multi_instanced(allocation).is_empty());
        nani_flush_instance_updates(&mut world.resource_mut::<GpuWorld>(), &device);
        assert!(device.writes.get() > 0);
        // 原state写+138屏蔽双collider，独立flags位+139保持零；两种业务状态都走真实链。
        let expected_colliders = if state == 0 { 3 } else { 0 };
        assert_eq!(world.resource::<DecoratorPhysicsColliders>().first.len(), expected_colliders);
        assert_eq!(world.resource::<DecoratorPhysicsColliders>().second.len(), expected_colliders);

        // 使用真实removed追踪→clear→fullvisual→tracker removal，不直接伪造diff。
        // 底层Decorators::remove仅删记录；原编辑入口还清modified、标removed并注销d2s。
        // 必须经过完整入口，不能借iter_modified的filter_map吞掉损坏追踪。
        world.resource_mut::<DecoratorStorage>().decorator_to_storage.insert(
            DecoratorId(9), country_core::resources::walls::decorator_storage::DecoratorStorageAddr::Wall(WallId(2)));
        for id in [9, 41, 42] { world.resource_mut::<DecoratorStorage>()
            .apply_decorator_remove(country_core::resources::walls::decorator_storage::DecoratorStorageAddr::Wall(WallId(2)), DecoratorId(id)); }
        schedule.run(&mut world);
        assert!(smoke_entities.iter().all(|&entity| world.get_entity(entity).is_err()));
        assert!(world.resource::<ChimneySmokeThickness>().thickness.is_empty());
        assert!(world.resource::<DecoratorPhysicsColliders>().first.is_empty());
        assert!(world.resource::<DecoratorPhysicsColliders>().second.is_empty());
        assert!(world.resource::<GpuWorld>().owned_subset_mut(GpuWorldSubsetKind::Smoke).instance_count() == 0);
        let allocation = world.get::<InstancedWall<BrickTransformSsbo>>(owner).unwrap().chimney_allocation().unwrap();
        assert!(world.resource::<AssetSsboLibrary>().get::<BrickTransformSsbo>().unwrap()
            .allocation_to_draw_multi_instanced(allocation).is_empty());
        assert!(world.resource::<Events<PlayDecoratorSoundCmd>>().iter_current_update_events()
            .any(|sound| sound.kind == 0x13));
        // 同World实际实体退场后还会有allocator回收队列；销毁World释放余下owners。
        world.despawn(owner);
        drop(schedule);
        drop(world);
        }
    }

    #[test]
    fn non_wall_roof_branch_preserves_outline_before_original_address_failure() {
        use country_core::resources::walls::decorator_storage::DecoratorStorageAddr;
        use std::panic::{catch_unwind, AssertUnwindSafe};
        for addr in [DecoratorStorageAddr::Terrain, DecoratorStorageAddr::Stair] {
            let mut world = roof_world();
            let mut storage = world.resource_mut::<DecoratorStorage>();
            let bucket = storage.per_wall.remove(&WallId(2)).unwrap();
            match addr {
                DecoratorStorageAddr::Terrain => storage.terrain = Some(bucket),
                _ => storage.stairs = Some(bucket),
            }
            drop(storage);
            assert!(catch_unwind(AssertUnwindSafe(||
                world.run_system_once(generate_roof_chimneys).unwrap())).is_err());
            let blueprint = world.resource::<DecoratorBlueprints>().get(9).unwrap();
            assert_eq!(blueprint.storage_addr, Some(addr));
            assert!(blueprint.transform.is_some());
            assert!(blueprint.visual_entities.is_empty());
            assert!(blueprint.chimney_assembly.is_none());
            assert!(matches!(blueprint.collision_mesh, BlueprintMeshSlot::Unspecified));
        }
    }

    #[test]
    fn missing_modified_record_panics_before_subtype_or_blueprint_side_effects() {
        use std::panic::{catch_unwind, AssertUnwindSafe};
        let mut world = roof_world();
        let mut storage = world.resource_mut::<DecoratorStorage>();
        let bucket = storage.per_wall.get_mut(&WallId(2)).unwrap();
        bucket.modified_ids.clear(); bucket.modified_ids.insert(999);
        drop(storage);
        let error = catch_unwind(AssertUnwindSafe(||
            world.run_system_once(generate_roof_chimneys).unwrap())).unwrap_err();
        let message = error.downcast_ref::<&str>().copied()
            .or_else(|| error.downcast_ref::<String>().map(String::as_str)).unwrap_or("");
        assert!(message.contains("modified roof chimney is missing"));
        assert!(world.resource::<DecoratorBlueprints>().map.is_empty());
    }
}
