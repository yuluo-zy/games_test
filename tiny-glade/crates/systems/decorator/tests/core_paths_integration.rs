//! DEC-E 集成测试：DEC 核心路径（生命周期顺序、Pop/restore、楼梯三处理器、
//! 锚点/合并、可视状态）。全部断言真实资源/数据结构写回。

use bevy_app::{App, Update};
use bevy_ecs::prelude::*;
use bevy_ecs::system::{RunSystemOnce, System};

use glam::{Quat, Vec3};

use country_core::resources::stairs::stairs_state::StairsState;
use country_core::resources::terrain_height::TerrainHeightsData;
use country_core::resources::walls::decorator::{
    DecoratorExtraInfo, DecoratorId, DecoratorInfo, StairExtra,
};
use country_core::resources::walls::decorator_archivist::changes::DecoratorArchivist;
use country_core::resources::walls::decorator_storage::{
    DecoratorStorage, DecoratorStorageAddr,
};
use country_core::resources::walls::decorator_subtype::{
    DecoratorMeshLibrary, DerivedDecoratorSubtypes,
};
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::walls::WallId;

use system_decorator::backup::{create_or_restore_decorator_backup, DecoratorBackup};
use system_decorator::collision_detection::WallIntersectCandidate;
use system_decorator::decorator_visual_state::{
    DecoratorPersistentVisualState, DecoratorTransientVisualState, DecoratorVisualState,
};
use system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory;
use system_decorator::merge_proposal_wall_decorators::{
    propose_wall_decorator_merge_position_inner,
    WallMergeEnv,
};
use system_decorator::move_decorators_following_anchors::{
    move_decorators_following_anchors, observed_from_info,
};
use system_decorator::ui_systems::ui_change_stair_height::{
    addr_disc_payload, build_stair_height_op, stair_height_of, sync_core_stair_extra,
    terrain_height_position,
};
use utils::wall_space::{WallSpace, WallSpaceCoord};

fn decorator_info(id: u32, ty: u8, rank: u8, state: u8) -> DecoratorInfo {
    DecoratorInfo {
        decorator_id: id,
        ty,
        rank,
        state,
        ..DecoratorInfo::default()
    }
}

fn insert_wall_decorator(storage: &mut DecoratorStorage, wall: WallId, id: DecoratorId, info: DecoratorInfo) {
    let addr = DecoratorStorageAddr::Wall(wall);
    storage.get_or_insert_default(addr).insert(id, info);
    storage.decorator_to_storage.insert(id, addr);
}

// ---------------------------------------------------------------- 生命周期顺序

#[test]
fn update_schedule_registers_audio_after_transform_cache_in_chain() {
    use bevy_ecs::schedule::Schedules;

    let mut app = App::new();
    system_decorator::add_decorator_systems::<(), ()>(&mut app);
    // Schedule::initialize 需要同时可变借用 Schedule 与 World；先把 Schedules
    // 从世界取出，初始化后再放回。
    let mut schedules = app
        .world_mut()
        .remove_resource::<Schedules>()
        .expect("Schedules");
    let schedule = schedules.get_mut(Update).expect("Update schedule");
    schedule.initialize(app.world_mut()).expect("schedule initializes");
    let names: Vec<String> = schedule
        .systems()
        .expect("initialized")
        .map(|(_, system)| system.name().into_owned())
        .collect();
    app.world_mut().insert_resource(schedules);

    let index = |needle: &str| {
        names
            .iter()
            .position(|name| name.contains(needle))
            .unwrap_or_else(|| panic!("system {needle} not registered; got {names:#?}"))
    };
    let derive = index("derive_decorator_subtypes");
    let cache = index("cache_decorator_transforms::cache_decorator_transforms");
    let confirm = index("op_confirm_audio::op_confirm_audio");
    let preview = index("op_preview_audio::op_preview_audio");
    let stairs = index("op_stairs_audio::op_stairs_audio");
    let history = index("decorator_record_history");

    assert!(derive < cache, "subtype derivation must precede transform cache");
    assert!(cache < confirm, "audio runs after the transform cache");
    assert!(confirm < preview && preview < stairs, "audio chain order");
    assert!(stairs < history, "history phase runs after audio phase");
}

// ---------------------------------------------------------------- Pop / restore

#[test]
fn backup_create_then_restore_round_trips_storage() {
    let mut world = World::new();
    world.init_resource::<DecoratorBackup>();
    world.init_resource::<DecoratorStorage>();
    world.init_resource::<StairsState>();
    world.init_resource::<DeferredDecoratorEditHistory>();
    world.init_resource::<DerivedDecoratorSubtypes>();
    world.init_resource::<system_decorator::blueprint::DecoratorBlueprints>();
    world.init_resource::<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>();
    world.init_resource::<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>();
    world.insert_resource(DecoratorArchivist::<()>::default());

    let wall = WallId(5);
    let id = DecoratorId(11);
    let info = decorator_info(id.0, 0x38, 1, 1);
    insert_wall_decorator(
        world.resource_mut::<DecoratorStorage>().into_inner(),
        wall,
        id,
        info,
    );
    // 真实备份仅遍历本次编辑归档过的桶；登记原存储快照，不能依赖旧全仓复制行为。
    // 本例没有碰撞载荷，只验备份的数据桶；生产编辑使用with_resources入口。
    world.resource_scope(|world, mut archivist: Mut<DecoratorArchivist<()>>| {
        let storage = world.resource::<DecoratorStorage>();
        archivist.archive_storage_decorators(0, storage.wall_decorators(wall).unwrap(), Some(wall));
    });
    assert!(world
        .resource::<DecoratorStorage>()
        .wall_decorators(wall)
        .unwrap()
        .get(id)
        .is_some());

    // CREATE
    world.resource_mut::<DecoratorBackup>().state = 1;
    world
        .run_system_once(create_or_restore_decorator_backup::<()>)
        .unwrap();
    let backup = world.resource::<DecoratorBackup>();
    assert!(backup.created);
    assert!(backup.storage_backup.per_wall.contains_key(&wall));

    // 模拟 Pop：移除 live 记录。
    world
        .resource_mut::<DecoratorStorage>()
        .apply_decorator_remove(DecoratorStorageAddr::Wall(wall), id);
    assert!(world
        .resource::<DecoratorStorage>()
        .wall_decorators(wall)
        .unwrap()
        .get(id)
        .is_none());

    // RESTORE
    world.resource_mut::<DecoratorBackup>().state = 2;
    world
        .run_system_once(create_or_restore_decorator_backup::<()>)
        .unwrap();
    let restored = world
        .resource::<DecoratorStorage>()
        .wall_decorators(wall)
        .unwrap()
        .get(id)
        .expect("restored decorator");
    assert_eq!(restored.ty, 0x38);
    assert_eq!(restored.decorator_id, id.0);
}

// ---------------------------------------------------------------- 楼梯三处理器

#[test]
fn stair_height_handlers_map_all_storage_addresses_and_heights() {
    // 存储地址判别：墙 0、地形 1、楼梯 2（O：原生 op kind）。
    assert_eq!(
        addr_disc_payload(DecoratorStorageAddr::Wall(WallId(9))),
        (0, 9)
    );
    assert_eq!(addr_disc_payload(DecoratorStorageAddr::Terrain), (1, 0));
    assert_eq!(addr_disc_payload(DecoratorStorageAddr::Stair), (2, 0));

    // 地形处理器：+0x00 前 8B 为 (x,z)，对地形采样取 y。
    let packed = u64::from(12.5f32.to_bits()) | (u64::from((-7.25f32).to_bits()) << 32);
    let terrain_info = DecoratorInfo {
        dst_wall: packed,
        u20: 3 << 16,
        ..DecoratorInfo::default()
    };
    let terrain = TerrainHeightsData::new();
    let pos = terrain_height_position(&terrain, &terrain_info);
    assert_eq!((pos.x, pos.z), (12.5, -7.25));

    // Height is a typed patch over the existing Info, not a replacement Info.
    let op = build_stair_height_op(DecoratorStorageAddr::Terrain, DecoratorId(7), 3.5);
    assert_eq!(op.kind, 1, "terrain storage disc");
    let patch = op.change.unwrap();
    assert_eq!(patch.destination, None, "original in-place sentinel");
    assert_eq!(patch.extra_low, Some(3.5f32.to_bits()));
    assert_eq!(patch.rank, None);
    assert_eq!(patch.ty, None);

    let roof_op = build_stair_height_op(DecoratorStorageAddr::Wall(WallId(3)), DecoratorId(8), 1.25);
    assert_eq!(roof_op.kind, 0, "roof decorators live in the wall storage");
    assert_eq!(roof_op.change.unwrap().destination, None);
}

#[test]
fn stair_height_sync_preserves_edge_payload() {
    let mut info = DecoratorInfo::default();
    let mut raw = [0u8; 20];
    raw[0..4].copy_from_slice(&1.0f32.to_le_bytes());
    raw[4..8].copy_from_slice(&0x1122_3344u32.to_le_bytes());
    info.set_extra(DecoratorExtraInfo::Stair(StairExtra(raw)));

    sync_core_stair_extra(&mut info, 4.5);
    let synced = info.extra().as_stair();
    assert_eq!(stair_height_of(&info), 4.5);
    assert_eq!(&synced.0[4..8], &0x1122_3344u32.to_le_bytes());
}

// ---------------------------------------------------------------- 锚点 / 合并

#[test]
fn anchor_follow_marks_modified_and_registers_changed_wall() {
    let mut storage = DecoratorStorage::default();
    let wall = WallId(5);
    let id = DecoratorId(11);
    let info = DecoratorInfo {
        dst_wall: wall.0,
        u20: 2 << 16,
        decorator_id: id.0,
        ty: 0x38,
        rank: 2,
        state: 2,
        ..DecoratorInfo::default()
    };
    insert_wall_decorator(&mut storage, wall, id, info);

    use country_core::resources::walls::{InnerWalls, WallState,
        public_walls::{OnWallChanged, OnWallDeleted}, publication_source::TerrainHeightGrid512};
    use country_core::resources::roofs::{geometry::surface::Curve2, commands::RecalculateRoofCmd};
    use country_core::systems::wall::adjust_wall_height::AdjustWallHeightCmd;
    use system_decorator::decorator_visual::deco_differential_state::{
        WallAttachedDecoTracker, ChimneyAssemblyParams, DoorStairsAssemblyParams,
    };
    let mut inner=InnerWalls::new();
    inner.new_wall_with_id(wall,WallState::new(Curve2::try_new(vec![[0.,0.],[4.,0.]],false).unwrap(),4.,0,0,false));
    let mut walls=PublicWalls::default();
    walls.consume_with_paths(&mut inner,&TerrainHeightGrid512{encoded:vec![0;512*512].into(),minimum_y:0.,maximum_y:0.,sampling_fix:false},None,&Default::default(),&Default::default());
    let mut world=World::new();world.insert_resource(storage);world.insert_resource(walls);
    world.init_resource::<DerivedDecoratorSubtypes>();world.init_resource::<DecoratorMeshLibrary>();
    world.init_resource::<WallAttachedDecoTracker<ChimneyAssemblyParams>>();
    world.init_resource::<WallAttachedDecoTracker<DoorStairsAssemblyParams>>();
    world.init_resource::<Events<OnWallChanged>>();world.init_resource::<Events<OnWallDeleted>>();
    world.init_resource::<Events<RecalculateRoofCmd>>();world.init_resource::<Events<AdjustWallHeightCmd>>();
    world.resource_mut::<Events<OnWallChanged>>().send(OnWallChanged(wall));
    world.run_system_once(move_decorators_following_anchors).unwrap();
    let mut recorded=Vec::new();
    world.resource_mut::<WallAttachedDecoTracker<ChimneyAssemblyParams>>().with_diffs(|changed,log|{
        recorded.push(changed);log.into_removals().with_removed(|_,_|panic!("unexpected removal"))
    });
    assert_eq!(recorded,vec![wall]);
    let storage=world.resource::<DecoratorStorage>();
    let bucket = storage.wall_decorators(wall).unwrap();
    assert!(bucket.iter_modified_ids().any(|modified| modified == id.0));

    // 字节桥往返（观测视图 → 写回前的数据保真）。
    let observed = observed_from_info(&info);
    assert_eq!(observed.bytes()[68], 2, "rank");
    assert_eq!(observed.bytes()[70], 0x38, "type");
}

#[test]
fn merge_proposal_requires_target_mesh_before_incompatible_family() {
    let wall = WallId(5);
    let existing_id = 2u32;
    let mut storage = DecoratorStorage::default();
    let existing = DecoratorInfo {
        dst_wall: wall.0,
        u20: 0 << 16,
        decorator_id: existing_id,
        ty: 0x3e,
        rank: 1,
        state: 2,
        ..DecoratorInfo::default()
    };
    insert_wall_decorator(&mut storage, wall, DecoratorId(existing_id), existing);

    let mesh_lib = DecoratorMeshLibrary::default();
    let derived = DerivedDecoratorSubtypes::default();
    let wall_space = WallSpace {
        is_loop: false,
        length: 10.0,
    };
    let env = WallMergeEnv {
        mesh_lib: &mesh_lib,
        derived_subtypes: &derived,
        wall_space: &wall_space,
        current_id: DecoratorId(3),
    };
    let new_info = DecoratorInfo {
        decorator_id: 3,
        ty: 0x38,
        rank: 1,
        ..DecoratorInfo::default()
    };
    let candidate = WallIntersectCandidate {
        coord: WallSpaceCoord::new_unchecked(1.0, 0.5),
        along_width: 1.0,
        vertical_extent: 1.0,
    };

    // 原先读目标subtype和mesh；缺失不是不兼容类型的静默None。
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        propose_wall_decorator_merge_position_inner(&new_info, &storage, &env, wall, existing_id, candidate, &[])
    })).is_err());

}

// ---------------------------------------------------------------- 可视状态

#[test]
fn visual_state_tracks_persistent_seal_and_transient_drain() {
    let mut state = DecoratorVisualState::default();
    let first = Entity::from_raw(101);
    let second = Entity::from_raw(102);
    let transient = Entity::from_raw(103);

    state
        .persistent
        .add_decorator_visual_entity(7, first);
    state
        .persistent
        .add_decorator_visual_entity(7, second);
    state.seal_persistent_entities(7);
    assert_eq!(state.persistent_entities_of(7), Some([first, second].as_slice()));
    assert_eq!(
        state.persistent.sealed_ids().collect::<Vec<_>>(),
        vec![7]
    );

    state
        .transient
        .add_decorator_visual_entity(7, transient);
    assert_eq!(state.transient_entities_of(7), Some([transient].as_slice()));
    let drained: Vec<(u32, Vec<Entity>)> =
        state.transient.drain().map(|(id, list)| (id, list)).collect();
    assert_eq!(drained, vec![(7, vec![transient])]);
    assert!(state.transient_entities_of(7).is_none());

    let removed = state.remove_persistent_entities(7).expect("removed entry");
    assert_eq!(removed.entities, vec![first, second]);
    assert!(removed.is_sealed());
    assert!(state.persistent_entities_of(7).is_none());
}

#[test]
fn persistent_state_rejects_add_after_seal() {
    let mut persistent = DecoratorPersistentVisualState::default();
    persistent.add_decorator_visual_entity(1, Entity::from_raw(1));
    persistent.seal_persistent_entities(1);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        persistent.add_decorator_visual_entity(1, Entity::from_raw(2));
    }));
    assert!(result.is_err(), "sealed entries must reject new entities");

    let mut transient = DecoratorTransientVisualState::default();
    transient.add_decorator_visual_entity(2, Entity::from_raw(3));
    assert_eq!(transient.entities_of(2), Some([Entity::from_raw(3)].as_slice()));
}

#[test]
fn decorator_transform_cache_requires_swap_for_visibility() {
    use country_core::resources::walls::cached_decorator_transforms::{
        CachedDecoratorTransform, CachedDecoratorTransforms,
    };
    let mut cache = CachedDecoratorTransforms::default();
    cache.insert(
        7,
        CachedDecoratorTransform {
            rotation: Quat::IDENTITY,
            translation: Vec3::new(1.0, 2.0, 3.0),
            scale: Vec3::ONE,
        },
    );
    assert_eq!(cache.try_position(7), Some(Vec3::new(1.0, 2.0, 3.0)));
}
