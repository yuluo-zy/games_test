//! Actual ECS cache -> blueprint generator -> fullvisual -> collider/Nani
//! components. Explicit CPU metadata and descriptor boundary; no GPU verdict.
use bevy_ecs::prelude::*;
use country_core::{
    asset_libraries::{
        mesh_library::AssetMeshLibrary,
        shader_library::{AssetShaderLibrary, SharedAssetShaderLibrary},
        texture_library::{
            AssetTextureLibrary, DescriptorImageUpdate, ImageView, TextureLibraryDevice,
            TextureUpdate,
        },
    },
    components::drawable::NaniInstanceDesc,
    render::load_texture::boundary::TextureDevice,
    resources::{
        clutter_meshes::{ClutterMesh, ClutterMeshes},
        stairs::{StairsState, stairs_assembly_params::PlayermadeStairsAssemblyRequests},
        terrain_height::TerrainHeightsData,
        walls::{
            cached_decorator_transforms::CachedDecoratorTransforms,
            decorator::{
                DecoratorDst, DecoratorId, DecoratorInfo, DecoratorRotationXZ, TerrainDecoratorDst,
            },
            decorator_collision::DecoratorPhysicsColliders,
            decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
            public_walls::{OnWallDeleted, PublicWalls},
            wall_holes::WallHoles,
        },
    },
    rhapsody_contract::DescriptorSet,
    rhapsody_types::{BackendError, Texture, TextureDesc},
    systems::{
        collision::{WindowAutoClutterCandidates, world::RaycastWorld},
        ui::signifiers::UiSignifierStream,
    },
};
use glam::{Vec2, Vec3};
use std::sync::Arc;
use system_decorator::{
    blueprint::{
        DecoratorBlueprints, PrevDecoratorBlueprints, clear_blueprints, copy_blueprints_to_prev,
        instantiate_blueprints,
    },
    cache_decorator_transforms::cache_decorator_transforms,
    decorator_visual::{
        deco_differential_state::{
            ChimneyAssemblyParams, DoorStairsAssemblyParams, WallAttachedDecoTracker,
        },
        decorator_clutter::generate_decorator_clutter,
    },
    decorator_visual_state::DecoratorVisualState,
};

struct DescriptorBoundary;
impl country_core::render::ssbo::rhapsody_boundary::GraphicsDevice for DescriptorBoundary {
    fn update_buffer_bytes(
        &self,
        _: &country_core::render::ssbo::rhapsody_boundary::Buffer,
        _: &[u8],
        _: &[std::ops::Range<usize>],
        _: usize,
    ) -> Result<(), BackendError> {
        panic!("CPU row test does not publish to a GPU")
    }
    fn create_buffer(
        &self,
        desc: &country_core::render::ssbo::rhapsody_boundary::BufferDesc,
        _: &str,
        _: Option<&[u8]>,
    ) -> Result<country_core::render::ssbo::rhapsody_boundary::Buffer, BackendError> {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        Ok(country_core::render::ssbo::rhapsody_boundary::Buffer {
            id: NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            generation: 1,
            byte_size: desc.size as u64,
            _gap10: 0,
            _gap20: 0,
        })
    }
    fn mapped_slice_mut(
        &self,
        buffer: &country_core::render::ssbo::rhapsody_boundary::Buffer,
    ) -> Option<&mut [u8]> {
        let words =
            Box::leak(vec![0u128; (buffer.byte_size as usize).div_ceil(16)].into_boxed_slice());
        Some(unsafe {
            std::slice::from_raw_parts_mut(words.as_mut_ptr().cast(), buffer.byte_size as usize)
        })
    }
}
impl TextureDevice for DescriptorBoundary {
    fn create_texture(&self, _: &str, _: &TextureDesc, _: &[u8]) -> Result<Texture, BackendError> {
        panic!("GPU texture upload is outside this CPU ECS case")
    }
}
impl TextureLibraryDevice for DescriptorBoundary {
    fn create_persistent_descriptor_set(&self, _: &str) -> Result<DescriptorSet, BackendError> {
        Ok(DescriptorSet::default())
    }
    fn update_persistent_descriptor_set(
        &self,
        _: &DescriptorSet,
        _: &[DescriptorImageUpdate],
    ) -> Result<(), BackendError> {
        panic!("no GPU publication")
    }
    fn get_vk_image_view(&self, _: &Texture, _: u32) -> Result<ImageView, BackendError> {
        panic!("no GPU image")
    }
    fn update_texture(&self, _: &Texture, _: &TextureUpdate) -> Result<(), BackendError> {
        panic!("no GPU texture")
    }
}

fn fixture() -> (World, Schedule) {
    let mut world = World::new();
    world.init_resource::<DecoratorStorage>();
    world.init_resource::<CachedDecoratorTransforms>();
    world.init_resource::<PublicWalls>();
    world.insert_resource(TerrainHeightsData::<
        country_core::resources::terrain_height::TerrainHeightSsbo,
    >::new());
    world.init_resource::<PlayermadeStairsAssemblyRequests>();
    world.init_resource::<StairsState>();
    let bounds = utils::geometry::rectangle::Rectangle3d::new(
        Vec3::new(0.2, 1.0, -0.3),
        Vec3::new(2.0, 2.0, 3.0),
        Vec2::X,
    );
    let mesh = ClutterMesh {
        bounds,
        mesh_name: "cpu_terrain_clutter",
        mesh: None,
        name_hash: utils::name_hash::NameHash::compute("cpu_terrain_clutter"),
        radius_xz: 2.0,
        native_group: 0,
        solid_texture: None,
        native_flag_46: false,
        native_flag_47: false,
    };
    world.insert_resource(ClutterMeshes::from_parts(
        std::array::from_fn(|_| mesh.clone()),
        vec![mesh],
    ));
    world.init_resource::<DecoratorBlueprints>();
    world.init_resource::<PrevDecoratorBlueprints>();
    world.init_resource::<DecoratorVisualState>();
    world.init_resource::<WallHoles>();
    world.insert_resource(RaycastWorld::new());
    world.init_resource::<DecoratorPhysicsColliders>();
    world.init_resource::<WindowAutoClutterCandidates>();
    world.init_resource::<WallAttachedDecoTracker<ChimneyAssemblyParams>>();
    world.init_resource::<WallAttachedDecoTracker<DoorStairsAssemblyParams>>();
    world.insert_resource(AssetMeshLibrary::new());
    world.insert_resource(SharedAssetShaderLibrary(
        Arc::new(AssetShaderLibrary::new()),
    ));
    world.insert_resource(AssetTextureLibrary::new(&DescriptorBoundary));
    world.init_resource::<UiSignifierStream>();
    world.init_resource::<Events<OnWallDeleted>>();
    let mut info = DecoratorInfo {
        decorator_id: 91,
        ..Default::default()
    };
    info.set_destination(DecoratorDst::Terrain(TerrainDecoratorDst {
        position_xz: Vec2::new(7.0, -4.0),
        yaw: DecoratorRotationXZ(0.61),
    }));
    world
        .resource_mut::<DecoratorStorage>()
        .get_or_insert_default(DecoratorStorageAddr::Terrain)
        .insert(DecoratorId(91), info);
    let mut schedule = Schedule::default();
    schedule.add_systems(
        (
            cache_decorator_transforms,
            clear_blueprints,
            generate_decorator_clutter,
            instantiate_blueprints,
            copy_blueprints_to_prev,
        )
            .chain_ignore_deferred(),
    );
    (world, schedule)
}

fn entity(world: &World) -> Entity {
    world
        .resource::<DecoratorVisualState>()
        .persistent_entities_of(91)
        .unwrap()[0]
}

// The original validate_blueprints entry performs this metadata cleanup after
// preview/collision generation and before the full visual group. That complete
// validator is not mounted by this CPU fixture: invoke the actual owner method
// between frames, rather than duplicating its internals or claiming host stage
// coverage. No history validation or gameplay rollback is asserted here.
fn drain(world: &mut World) {
    world
        .resource_mut::<DecoratorStorage>()
        .clear_modification_tracking();
}

#[test]
fn terrain_real_cache_blueprint_colliders_and_nani_update_retire_together() {
    let (mut world, mut schedule) = fixture();
    schedule.run(&mut world);
    let first = entity(&world);
    let pose = world.resource::<CachedDecoratorTransforms>().transform(91);
    assert_eq!(pose.translation, Vec3::new(7.0, -2.5, -4.0));
    assert!(
        pose.rotation
            .abs_diff_eq(DecoratorRotationXZ(0.61).to_quat(), 0.0)
    );
    assert!(world.get::<NaniInstanceDesc>(first).is_some());
    assert_eq!(
        world
            .get::<country_core::utils::components::transform::Transform>(first)
            .unwrap()
            .translation,
        pose.translation
    );
    let maps = world.resource::<DecoratorPhysicsColliders>();
    assert_eq!((maps.first.len(), maps.second.len()), (1, 1));
    let old = world.resource::<DecoratorBlueprints>().map[&91].clone();
    drain(&mut world);
    schedule.run(&mut world);
    assert_eq!(entity(&world), first);
    assert!(Arc::ptr_eq(
        &old,
        &world.resource::<DecoratorBlueprints>().map[&91]
    ));
    assert!(
        world
            .resource::<DecoratorStorage>()
            .get(DecoratorStorageAddr::Terrain)
            .unwrap()
            .modified_ids
            .is_empty()
    );
    // A height-field edit, without editing the decorator record, must make the
    // transform cache mark the decorator dirty and cause a real rebuild.
    world
        .resource_mut::<TerrainHeightsData>()
        .heights_mut()
        .fill(u16::MAX);
    schedule.run(&mut world);
    let second = entity(&world);
    assert_ne!(first, second);
    assert!(!world.entities().contains(first));
    assert_eq!(
        world.resource::<CachedDecoratorTransforms>().position(91),
        Vec3::new(7.0, 24.0, -4.0)
    );
    assert_eq!(
        world
            .get::<country_core::utils::components::transform::Transform>(second)
            .unwrap()
            .translation
            .y,
        24.0
    );
    assert!(world.get::<NaniInstanceDesc>(second).is_some());
    assert!(!Arc::ptr_eq(
        &old,
        &world.resource::<DecoratorBlueprints>().map[&91]
    ));
    drain(&mut world);
    {
        let mut storage = world.resource_mut::<DecoratorStorage>();
        let bucket = storage.get_mut(DecoratorStorageAddr::Terrain).unwrap();
        bucket.decorators.remove(&91);
        bucket.removed_ids.insert(91);
    }
    schedule.run(&mut world);
    assert!(!world.entities().contains(second));
    assert!(
        !world
            .resource::<DecoratorBlueprints>()
            .map
            .contains_key(&91)
    );
    assert!(
        !world
            .resource::<PrevDecoratorBlueprints>()
            .map
            .contains_key(&91)
    );
    assert!(
        world
            .resource::<CachedDecoratorTransforms>()
            .try_transform(91)
            .is_none()
    );
    assert!(
        world
            .resource::<DecoratorVisualState>()
            .persistent_entities_of(91)
            .is_none()
    );
    let maps = world.resource::<DecoratorPhysicsColliders>();
    assert!(maps.first.is_empty() && maps.second.is_empty());
}

#[test]
fn flat_roof_cache_resolves_actual_uv_floor_and_preserves_previous_on_uv_miss() {
    use country_core::resources::{
        roofs::{Roof, roof_trigger_volume::RoofTriggerVolume},
        walls::{WallId, decorator::RoofDecoratorDst},
    };
    let (mut world, _) = fixture();
    let mut bytes = [0u8; 88];
    bytes[8..12].copy_from_slice(&1u32.to_le_bytes()); // rectangular enclosure
    bytes[12..16].copy_from_slice(&1f32.to_le_bytes()); // actual +X frame
    bytes[0x4c..0x50].copy_from_slice(&2f32.to_le_bytes());
    let mut volume = RoofTriggerVolume::default();
    volume.set_triangles(vec![[Vec3::ZERO, Vec3::X, Vec3::Z]]);
    volume.set_uvs(vec![[Vec2::ZERO, Vec2::X, Vec2::Y]]);
    let roof = world.spawn((Roof::from_observed(bytes), volume)).id();
    let wall = WallId(72);
    world
        .resource_mut::<PublicWalls>()
        .associate_roof(wall, roof);
    let mut info = DecoratorInfo {
        decorator_id: 92,
        ..Default::default()
    };
    info.set_destination(DecoratorDst::Roof(RoofDecoratorDst {
        coord: Vec2::splat(0.25),
        yaw: DecoratorRotationXZ(0.4),
    }));
    world
        .resource_mut::<DecoratorStorage>()
        .get_or_insert_default(DecoratorStorageAddr::Wall(wall))
        .insert(DecoratorId(92), info);
    let mut schedule = Schedule::default();
    schedule.add_systems(cache_decorator_transforms);
    schedule.run(&mut world);
    let before = world.resource::<CachedDecoratorTransforms>().transform(92);
    assert_eq!(before.translation, Vec3::new(0.25, 2.0, 0.25));
    assert!(
        before
            .rotation
            .abs_diff_eq(DecoratorRotationXZ(0.4).to_quat(), 0.0)
    );
    world
        .resource_mut::<DecoratorStorage>()
        .get_mut(DecoratorStorageAddr::Wall(wall))
        .unwrap()
        .get_mut(DecoratorId(92))
        .unwrap()
        .set_destination(DecoratorDst::Roof(RoofDecoratorDst {
            coord: Vec2::splat(10.0),
            yaw: DecoratorRotationXZ(f32::NAN),
        }));
    schedule.run(&mut world);
    assert_eq!(
        world.resource::<CachedDecoratorTransforms>().transform(92),
        before
    );
    // A new record with no previous pose takes the original explicit sentinel.
    info.decorator_id = 93;
    world
        .resource_mut::<DecoratorStorage>()
        .get_or_insert_default(DecoratorStorageAddr::Wall(WallId(73)))
        .insert(DecoratorId(93), info);
    schedule.run(&mut world);
    assert_eq!(
        world.resource::<CachedDecoratorTransforms>().position(93),
        Vec3::splat(-100.0)
    );
    // UV failure must reuse the valid old quaternion even when the pending yaw
    // is invalid. With valid UVs, that same yaw reaches native finite guards.
    world
        .resource_mut::<DecoratorStorage>()
        .get_mut(DecoratorStorageAddr::Wall(wall))
        .unwrap()
        .get_mut(DecoratorId(92))
        .unwrap()
        .set_destination(DecoratorDst::Roof(RoofDecoratorDst {
            coord: Vec2::splat(0.25),
            yaw: DecoratorRotationXZ(f32::NAN),
        }));
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| schedule.run(&mut world)))
            .is_err()
    );
}

#[test]
fn actual_cache_matches_original_uv_fallback_golden_for_arbitrary_old_transforms() {
    use country_core::resources::walls::{
        WallId, cached_decorator_transforms::CachedDecoratorTransform, decorator::RoofDecoratorDst,
    };
    let (mut world, _) = fixture();
    world.resource_mut::<DecoratorStorage>().terrain = None;
    let mut info = DecoratorInfo {
        decorator_id: 92,
        ..Default::default()
    };
    info.set_destination(DecoratorDst::Roof(RoofDecoratorDst {
        coord: Vec2::splat(3.0),
        yaw: DecoratorRotationXZ(f32::NAN),
    }));
    world
        .resource_mut::<DecoratorStorage>()
        .get_or_insert_default(DecoratorStorageAddr::Wall(WallId(72)))
        .insert(DecoratorId(92), info);
    let mut schedule = Schedule::default();
    schedule.add_systems(cache_decorator_transforms);
    for (line, row) in include_str!("fixtures/roof_uv_fallback_native.txt")
        .lines()
        .enumerate()
    {
        let v: Vec<u32> = row.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let mut current = CachedDecoratorTransforms::default();
        if v[0] != 0 {
            current.insert(
                92,
                CachedDecoratorTransform::from_components(&std::array::from_fn(|i| {
                    f32::from_bits(v[1 + i])
                })),
            );
        }
        world.insert_resource(current);
        schedule.run(&mut world);
        assert_eq!(
            world
                .resource::<CachedDecoratorTransforms>()
                .transform(92)
                .to_components()
                .map(f32::to_bits),
            v[11..21],
            "original native case {line}"
        );
    }
}

#[test]
fn generated_terrain_visual_feeds_actual_nani_rows_and_removes_active_instance() {
    use bevy_ecs::schedule::ApplyDeferred;
    use country_core::{
        render::{
            mesh::{ATTR_POSITION, Mesh},
            nani::{
                gpu_world::{
                    GpuWorld, GpuWorldSubset, GpuWorldSubsetKind, InstanceDataBuffer, InstanceKey,
                    OwnedGpuWorldSubset,
                },
                instance_data::SolidVertexColorInstanceData,
                systems::{nani_maintain_instances_ecs, nani_update_solid_vertex_color_ecs},
            },
        },
        resources::render::mesh_atlas_library::MeshAtlasLibrary,
    };
    let (mut world, mut visual) = fixture();
    let kind = GpuWorldSubsetKind::SolidVertexColor;
    let mut gpu = GpuWorld::new(&DescriptorBoundary);
    gpu.assign_subset(
        kind,
        GpuWorldSubset::Owned(OwnedGpuWorldSubset::new::<SolidVertexColorInstanceData>(
            &DescriptorBoundary,
            kind, 1)),
    );
    world.insert_resource(gpu);
    let mut mesh = Mesh::new("actual CPU fixture triangle");
    mesh.set_attribute(ATTR_POSITION, vec![Vec3::ZERO, Vec3::X, Vec3::Z]);
    mesh.set_indices(vec![0, 1, 2]);
    let mut atlas = MeshAtlasLibrary::default();
    atlas.add(kind, "cpu_terrain_clutter", &mesh);
    world.insert_resource(atlas);
    let mut nani = Schedule::default();
    nani.add_systems(
        (
            nani_maintain_instances_ecs,
            ApplyDeferred,
            nani_update_solid_vertex_color_ecs,
        )
            .chain_ignore_deferred(),
    );
    for y in [-2.5, 24.0] {
        if y == 24.0 {
            drain(&mut world);
            world
                .resource_mut::<TerrainHeightsData>()
                .heights_mut()
                .fill(u16::MAX);
        }
        visual.run(&mut world);
        nani.run(&mut world);
        let e = entity(&world);
        let key = *world.get::<InstanceKey>(e).unwrap();
        let gpu = world.resource::<GpuWorld>();
        let subset = gpu.owned_subset_mut(kind);
        assert_eq!(subset.instance_count(), 1);
        assert_eq!(
            subset.positions()[key.index as usize].position,
            Vec3::new(7.0, y, -4.0)
        );
        let row = subset
            .data()
            .as_any()
            .downcast_ref::<InstanceDataBuffer<SolidVertexColorInstanceData>>()
            .unwrap()
            .shadow
            .data[key.index as usize]
            .raw;
        assert_eq!(
            &row[9..12],
            &[7f32.to_bits(), y.to_bits(), (-4f32).to_bits()]
        );
    }
    drain(&mut world);
    {
        let mut storage = world.resource_mut::<DecoratorStorage>();
        let bucket = storage.get_mut(DecoratorStorageAddr::Terrain).unwrap();
        bucket.decorators.remove(&91);
        bucket.removed_ids.insert(91);
    }
    visual.run(&mut world);
    nani.run(&mut world);
    assert_eq!(
        world
            .resource::<GpuWorld>()
            .owned_subset_mut(kind)
            .instance_count(),
        0
    );
}

#[test]
fn clutter_original_insert_keeps_zero_state_through_preview_commit_and_removal() {
    let (mut world, mut schedule) = fixture();
    let mut previous_entity = None;
    let mut previous_blueprint = None;
    // 原 clutter producer 明确传零；验证正式→预览→正式真实记录，不统一套用 Info.state。
    for state in [0, 1, 0] {
        {
            let mut storage = world.resource_mut::<DecoratorStorage>();
            let bucket = storage.get_mut(DecoratorStorageAddr::Terrain).unwrap();
            bucket.get_mut(DecoratorId(91)).unwrap().state = state;
            bucket.modified_ids.insert(91);
        }
        schedule.run(&mut world);
        assert_eq!(
            world
                .resource::<DecoratorStorage>()
                .get(DecoratorStorageAddr::Terrain)
                .unwrap()
                .get(DecoratorId(91))
                .unwrap()
                .state,
            state,
        );
        let current_entity = entity(&world);
        let blueprint = world.resource::<DecoratorBlueprints>().map[&91].clone();
        assert_eq!(blueprint.u138, 0, "原 Info.state={state} 不能覆盖显式零参数");
        assert_eq!(blueprint.interaction_source_flags, 0);
        assert_eq!(blueprint.storage_addr, Some(DecoratorStorageAddr::Terrain));
        assert!(!blueprint.visual_entities.is_empty());
        assert!(world.get::<NaniInstanceDesc>(current_entity).is_some());
        let colliders = world.resource::<DecoratorPhysicsColliders>();
        assert_eq!((colliders.first.len(), colliders.second.len()), (1, 1));
        if let Some(old_entity) = previous_entity {
            assert_ne!(old_entity, current_entity);
            assert!(!world.entities().contains(old_entity));
        }
        if let Some(old) = &previous_blueprint {
            assert!(!Arc::ptr_eq(old, &blueprint));
        }
        previous_entity = Some(current_entity);
        previous_blueprint = Some(blueprint);
        drain(&mut world);
    }
    {
        let mut storage = world.resource_mut::<DecoratorStorage>();
        let bucket = storage.get_mut(DecoratorStorageAddr::Terrain).unwrap();
        bucket.decorators.remove(&91);
        bucket.removed_ids.insert(91);
    }
    schedule.run(&mut world);
    assert!(!world.entities().contains(previous_entity.unwrap()));
    assert!(!world.resource::<DecoratorBlueprints>().map.contains_key(&91));
    assert!(!world.resource::<PrevDecoratorBlueprints>().map.contains_key(&91));
    let colliders = world.resource::<DecoratorPhysicsColliders>();
    assert!(colliders.first.is_empty() && colliders.second.is_empty());
}
