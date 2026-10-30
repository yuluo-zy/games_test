//! B `crates/tiny-glade/src/decorator_plugin.rs`（装饰物插件注册 / glade 卸载）。
//!
//! 恢复依据（Ghidra shared 2026-10-17）：
//! - `impl$0::build` `0x140342c10`（10,365 B，113 KB 反编译）
//! - `decorators_on_glade_unload` `0x140347090`（PDB 9,451 B 含清理 funclet；Ghidra body 5,395 B）
//!
//! `impl$0::build` 是 `DecoratorPlugin` 的 Bevy 注册面：事件/资源插入 + 多组
//! `add_systems`（`Update`/`PostUpdate`），反编译体 113 KB，包含大量
//! `system_decorator::*` 系统函数指针与 `GameSystemSet` 排序。逐注册项判等
//! 需要：
//! 1. 从反编译 CALL 的泛型签名还原每组 `(系统函数, schedule, set, run_if)`;
//! 2. 核对 `system-decorator` 中各系统函数签名是否一致（D1–D4 恢复批次仍在
//!    推进，`add_decorator_systems` 只接了部分系统）。
//!
//! 当前不做：用 `add_decorator_systems` 整体转发会改变原注册顺序与集合，
//! 属「callee 转发错」，故登记未实现而不是近似接线。
//!
//! `decorators_on_glade_unload` 是卸载时的真实业务系统（装饰物/碰撞/可视化
//! owner 释放）；本轮按原 20 资源合同恢复，实体销毁由通用卸载系统完成。

/// `tiny_glade::decorator_plugin::impl$0::build` `0x140342c10`。
///
/// 原 113KB 反编译是逐组 `(系统, schedule, set, run_if)` 注册表；本轮先接通
/// crate 内已恢复的三组注册入口，并给出契约 §1 的阶段顺序（由
/// `system_decorator::add_decorator_systems` 内的 `.chain()/.after()` 保证）：
/// 装载/派生 → 变换缓存 → 可视化清空/音频/窗灯 → 历史/备份/归档恢复 →
/// 外部命令 → 回放；cull 族在历史之后注册。
///
/// 仍未逐项还原原注册表的 schedule/set/run_if 与缺失生成族（见 DEC DELIVERY）。
pub fn build(app: &mut bevy_app::App) {
    use country_core::resources::history::MaybePendingSnapshotId;
    // 与原 cull 族同一 `DecoratorArchivist<C>` 载荷（tiny-glade 启用
    // native-physics → LiveDecoratorArchivist 即此二元组）。
    type CollisionPayload = country_core::resources::walls::decorator_archivist::LiveColliderPayload;

    system_decorator::add_decorator_events_and_resources(app.world_mut());
    // 装饰归档 owner（原 DecoratorPlugin @140343132/13d 安装；幂等）。
    app.world_mut()
        .init_resource::<country_core::resources::walls::decorator_archivist::LiveDecoratorArchivist>();
    system_decorator::add_decorator_systems::<MaybePendingSnapshotId, CollisionPayload>(app);
    system_decorator::register_cull_oob_systems::<CollisionPayload>(app);
    system_decorator::ui_systems::register_ui_systems::<CollisionPayload>(app);
}

/// 原 B `decorators_on_glade_unload`：`0x140347090`，PDB 9451 B（含清理 funclet）。
///
/// 两组参数保持原 16 + 4 个资源及 Commands 的所有权边界。原函数以新默认值
/// 替换旧 owner，必须真正释放蓝图 Arc、归档碰撞网格及稀疏实例分配，不能只清长度。
/// 实体由同一 OnExit 阶段的 despawn_glade_entities 销毁；碰撞世界由其原卸载
/// owner 清空，本函数不接收 RaycastWorld，不能把注册表析构改造成逐体删除。
pub fn decorators_on_glade_unload(
    mut first: (
        bevy_ecs::prelude::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,
        bevy_ecs::prelude::ResMut<system_decorator::blueprint::DecoratorBlueprints>,
        bevy_ecs::prelude::ResMut<system_decorator::blueprint::PrevDecoratorBlueprints>,
        bevy_ecs::prelude::ResMut<country_core::resources::walls::decorator_archivist::LiveDecoratorArchivist>,
        bevy_ecs::prelude::ResMut<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>,
        bevy_ecs::prelude::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,
        bevy_ecs::prelude::ResMut<country_core::resources::walls::decorator::DecoratorIdGen>,
        bevy_ecs::prelude::ResMut<system_decorator::ui_systems::ui_place_decorator::PlaceDecoratorInteractionState>,
        bevy_ecs::prelude::ResMut<system_decorator::audio::DecoratorAudioEvents>,
        bevy_ecs::prelude::ResMut<country_core::resources::window_lights::GlobalWindowLightsControl>,
        bevy_ecs::prelude::ResMut<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,
        bevy_ecs::prelude::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,
        bevy_ecs::prelude::ResMut<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>,
        bevy_ecs::prelude::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,
        bevy_ecs::prelude::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams>>,
        bevy_ecs::prelude::ResMut<country_core::resources::stairs::stairs_state::StairsState>,
    ),
    mut second: (
        bevy_ecs::prelude::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,
        bevy_ecs::prelude::ResMut<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams>>,
        bevy_ecs::prelude::ResMut<country_core::resources::stairs::visual::StairsVisualState>,
        bevy_ecs::prelude::ResMut<country_core::resources::stairs::structural_graph::StairsStructuralGraph>,
        bevy_ecs::prelude::Commands,
    ),
) {
    // 原 14034764x..14034848x 按此顺序构造新 owner 并析构旧值。
    *first.0 = Default::default();
    *first.1 = Default::default();
    *first.2 = Default::default();
    *first.3 = Default::default();
    *first.4 = Default::default();
    *first.5 = Default::default();
    *first.6 = Default::default();
    *first.7 = Default::default();
    *first.8 = Default::default();
    *first.9 = Default::default();
    *first.11 = Default::default();
    *first.12 = Default::default();
    *first.13 = Default::default();
    *first.14 = Default::default();
    *first.15 = Default::default();
    *second.0 = Default::default();
    *second.1 = Default::default();
    *second.2 = Default::default();
    // 14034849e 保留结构图容器容量；1403484b9 写派发判别 {6, 0}。
    second.3.clear();
    first.10.clear();
    // 1403484e0：仅排入命令，缺失资源合法；执行时才移除可选网格 owner。
    second.4.remove_resource::<country_core::resources::stairs::stairs_top_down::StairsTopDownMesh>();
}

/// Genuine startup owners, independent of incomplete edit/UI system adapters.
/// No updater, replay, or UI leaf is mounted by this resource constructor.
pub fn build_startup_resources(app: &mut bevy_app::App) {
    system_decorator::add_decorator_events_and_resources(app.world_mut());
    system_decorator::ui_systems::register_ui_resources(app);
    app.init_resource::<country_core::resources::walls::decorator_archivist::LiveDecoratorArchivist>();
    // 原DecoratorPlugin/楼梯owner的初始空容器；场景载入与退出复用唯一资源。
    app.init_resource::<country_core::resources::roofs::dormer_tiles::DormerTiles>();
    app.init_resource::<country_core::resources::stairs::stairs_collider::StairColliderRegistry>();
    app.init_resource::<country_core::resources::stairs::visual::StairsVisualState>();
    app.init_resource::<country_core::resources::stairs::structural_graph::StairsStructuralGraph>();
    app.add_event::<system_decorator::handle_external_decorator_cmds::ExternalDecoratorCmd>();
    // 与验证writer和两个独立音频reader共享单位事件及真实App保留周期。
    app.add_event::<system_decorator::audio::op_preview_audio::MutePreviewDecoratorOpSounds>();
    app.init_resource::<system_decorator::blueprint::PrevDecoratorBlueprints>();
    app.init_resource::<country_core::systems::collision::WindowAutoClutterCandidates>();
}

/// Recovered subset of original Update/GameSystemSet25. This installs the
/// actual terrain clutter pipeline; the other native generator/assembly members
/// remain separate gaps. The resource condition waits for the real asset owner.
/// The main host must not install this subset before the real stair-attached
/// cache producer is recovered: the generic clutter generator also accepts dst4.
pub fn build_recovered_visual_systems(app: &mut bevy_app::App) {
    use bevy_ecs::schedule::IntoScheduleConfigs;
    use bevy_ecs::schedule::common_conditions::resource_exists;
    use country_core::{systems::GameSystemSet, resources::clutter_meshes::ClutterMeshes};
    use system_decorator::{blueprint::{clear_blueprints, instantiate_blueprint_colliders,
        instantiate_blueprints, copy_blueprints_to_prev},
        cache_decorator_transforms::cache_decorator_transforms,
        decorator_visual::decorator_clutter::generate_decorator_clutter};
    // Original parent 1403451af/1b4/253: Update set25. Constructor
    // 1400d52f0 writes cache, clear, generators, collider-only in this order;
    // its later 1403d6ce0 chain_ignore_deferred installs fullvisual then copy.
    app.add_systems(bevy_app::Update, (
        cache_decorator_transforms,
        clear_blueprints,
        system_decorator::decorator_visual::maintain_visual_entities::clear_out_visual_entities,
        generate_decorator_clutter,
        instantiate_blueprint_colliders,
        instantiate_blueprints,
        copy_blueprints_to_prev,
    ).chain_ignore_deferred()
        .in_set(GameSystemSet(25))
        .after(country_core::resources::walls::decorator::unleash_decorator_op_events)
        .after(system_decorator::ui_systems::ui_decorator_affordance_dispatch::ui_dispatch_decorator_affordance)
        .run_if(resource_exists::<ClutterMeshes>));
}

#[cfg(test)]
mod decorator_unload_tests {
    use super::decorators_on_glade_unload;
    use bevy_ecs::{prelude::*, system::IntoSystem};
    use country_core::{
        render::{sparse_instance_buffer::SparseInstanceBuffer, ssbo::rhapsody_boundary::{BackendError, Buffer, BufferDesc, GraphicsDevice}},
        resources::{
            roof_tile_ssbo::RoofTileSsbo, roofs::dormer_tiles::DormerTiles,
            stairs::{stairs_collider::StairColliderRegistry, stairs_state::{StairGraphEdge, StairsState}, stairs_top_down::StairsTopDownMesh, structural_graph::{StairColliderHandle, StairsStructuralGraph}, visual::StairsVisualState},
            walls::{WallId, decorator::{DecoratorId, DecoratorIdGen}, decorator_archivist::LiveDecoratorArchivist, decorator_collision::DecoratorPhysicsColliders, decorator_storage::{DecoratorStorage, DecoratorStorageAddr}, wall_stone_columns::StoneColumnCollisionStorage},
            window_lights::{GlobalWindowLightsControl, LightInfo},
        },
        systems::collision::world::{RaycastGeoSource, RaycastMeshData, RaycastWorld, upsert_collider},
    };
    use system_decorator::{
        audio::DecoratorAudioEvents, blueprint::{DecoratorBlueprint, DecoratorBlueprints, PrevDecoratorBlueprints},
        decorator_visual::{deco_differential_state::WallAttachedDecoTracker, chimney_assemble_bricks::ChimneyAssemblyParams, door_stairs_assemble_bricks::DoorStairsAssemblyParams},
        decorator_visual_state::DecoratorVisualState, deferred_decorator_history::{DeferredDecoratorEditHistory, DeferredDecoratorRecord},
        ui_systems::{ui_place_decorator::PlaceDecoratorInteractionState, ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher},
    };
    use std::sync::Arc;

    // 只验证真实稀疏分配与回收队列；本 CPU 边界没有提交或验证任何 GPU 工作。
    struct CpuBuffer;
    impl GraphicsDevice for CpuBuffer {
        fn create_buffer(&self, desc: &BufferDesc, _: &str, _: Option<&[u8]>) -> Result<Buffer, BackendError> {
            Ok(Buffer { id: 91, byte_size: desc.size as u64, ..Default::default() })
        }
        fn update_buffer_bytes(&self, buffer: &Buffer, bytes: &[u8], ranges: &[std::ops::Range<usize>], offset: usize) -> Result<(), BackendError> {
            for range in ranges {
                assert!(range.start <= range.end && range.end <= bytes.len());
                assert!(offset.checked_add(range.end).is_some_and(|end| end as u64 <= buffer.byte_size));
            }
            Ok(())
        }
    }

    fn init(world: &mut World) {
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<DecoratorBlueprints>();
        world.init_resource::<PrevDecoratorBlueprints>();
        world.init_resource::<LiveDecoratorArchivist>();
        world.init_resource::<DeferredDecoratorEditHistory>();
        world.init_resource::<DecoratorVisualState>();
        world.init_resource::<DecoratorIdGen>();
        world.init_resource::<PlaceDecoratorInteractionState>();
        world.init_resource::<DecoratorAudioEvents>();
        world.init_resource::<GlobalWindowLightsControl>();
        world.init_resource::<DecoratorAffordanceDispatcher>();
        world.init_resource::<DecoratorPhysicsColliders>();
        world.init_resource::<StoneColumnCollisionStorage>();
        world.init_resource::<DormerTiles>();
        world.init_resource::<WallAttachedDecoTracker<ChimneyAssemblyParams>>();
        world.init_resource::<StairsState>();
        world.init_resource::<StairColliderRegistry>();
        world.init_resource::<WallAttachedDecoTracker<DoorStairsAssemblyParams>>();
        world.init_resource::<StairsVisualState>();
        world.init_resource::<StairsStructuralGraph>();
    }

    #[test]
    fn unload_drops_original_owners_then_defers_top_down_removal_and_global_despawn() {
        let mut world = World::new();
        init(&mut world);
        let wall = WallId(0x1_0000_0007);
        let id = DecoratorId(41);
        let edge = StairGraphEdge::new(41, 42);
        let entity = world.spawn_empty().id();
        let kept = world.spawn(country_core::components::KeepOnLevelUnload).id();
        {
            let mut storage = world.resource_mut::<DecoratorStorage>();
            storage.get_or_insert_default(DecoratorStorageAddr::Wall(wall));
            storage.get_or_insert_default(DecoratorStorageAddr::Terrain);
            storage.get_or_insert_default(DecoratorStorageAddr::Stair);
            storage.decorator_to_storage.insert(id, DecoratorStorageAddr::Wall(wall));
            storage.removed.entry(DecoratorStorageAddr::Terrain).or_default().insert(id.0);
            storage.immutable = true;
        }
        let blueprint = Arc::new(DecoratorBlueprint::new());
        let weak_blueprint = Arc::downgrade(&blueprint);
        world.resource_mut::<DecoratorBlueprints>().map.insert(id.0, blueprint.clone());
        world.resource_mut::<PrevDecoratorBlueprints>().map.insert(id.0, blueprint);
        let mut physics = RaycastWorld::new();
        let mesh = Arc::new(RaycastMeshData::new_cuboid_at(glam::Vec3::splat(2.0), glam::Vec3::ZERO));
        let weak_mesh = Arc::downgrade(&mesh);
        {
            let mut maps = world.resource_mut::<DecoratorPhysicsColliders>();
            upsert_collider(&mut physics, &mut maps.first, id,
                RaycastGeoSource::decorator(id, DecoratorStorageAddr::Wall(wall)),
                mesh.clone(), glam::Vec3::ZERO, glam::Quat::IDENTITY);
            let collider = maps.first[&id];
            maps.second.insert(id, collider);
            let mut stairs = world.resource_mut::<StairColliderRegistry>();
            for map in &mut stairs.edge_colliders { map.insert(edge, collider); }
            stairs.structural_colliders.insert((edge, 2), collider);
        }
        {
            let mut archivist = world.resource_mut::<LiveDecoratorArchivist>();
            let transform = country_core::resources::walls::decorator_archivist::ArchivedColliderTransform {
                translation: glam::Vec3::ZERO, rotation: glam::Quat::IDENTITY,
                shape: mesh.shape.as_ref().unwrap().0.clone(),
            };
            archivist.collision_a.insert(id.0, (transform, country_core::systems::collision::world::RaycastMesh::new(mesh, RaycastGeoSource::decorator(id, DecoratorStorageAddr::Wall(wall)))));
            archivist.culling.allow_any = true;
            archivist.culling.allowed_ids.insert(id.0);
        }
        physics.flush_colliders();
        world.insert_resource(physics);
        {
            let mut visual = world.resource_mut::<DecoratorVisualState>();
            visual.persistent.add_decorator_visual_entity(id.0, entity);
            visual.transient.add_decorator_visual_entity(id.0, entity);
        }
        world.resource_mut::<DecoratorIdGen>().reserve(id);
        *world.resource_mut::<PlaceDecoratorInteractionState>() = PlaceDecoratorInteractionState {
            height_constraint: Some(7.5), height_offset: 3.25, last_height: 12.0,
            native_flag_10: true, fixed_height: true, progress: 2,
        };
        world.resource_mut::<DecoratorAudioEvents>().set_state(3, 7);
        world.resource_mut::<GlobalWindowLightsControl>().lights.insert(8, LightInfo::new(true));
        world.resource_mut::<GlobalWindowLightsControl>().lights_on = true;
        world.resource_mut::<DecoratorAffordanceDispatcher>().state = 3;
        world.resource_mut::<StoneColumnCollisionStorage>().clear_and_get_mut(wall);
        {
            let mut deferred = world.resource_mut::<DeferredDecoratorEditHistory>();
            deferred.records.push(DeferredDecoratorRecord { kind: 4, f1: 41, f2: 0, f3: 0 });
            deferred.pending = true;
        }
        world.resource_mut::<WallAttachedDecoTracker<ChimneyAssemblyParams>>().add(DecoratorStorageAddr::Wall(wall), id, &ChimneyAssemblyParams { words: [0; 10] });
        world.resource_mut::<WallAttachedDecoTracker<DoorStairsAssemblyParams>>().add(DecoratorStorageAddr::Wall(wall), id, &DoorStairsAssemblyParams { words: [0; 8] });
        world.resource_mut::<StairsState>().add_edge(41, 42);
        world.resource_mut::<StairsState>().make_backup();
        world.resource_mut::<StairsVisualState>().reset_edge(edge);
        world.resource_mut::<StairsStructuralGraph>().upsert_span((edge, 2), StairColliderHandle::new(1, 2, 3), 2.5);
        let capacity = world.resource::<StairsStructuralGraph>().collider_to_span().capacity();
        let mut tiles = SparseInstanceBuffer::<RoofTileSsbo>::try_new(&CpuBuffer).unwrap();
        let allocation = tiles.new_allocation();
        tiles.writer(&allocation).push(&RoofTileSsbo::default()).unwrap();
        assert!(!tiles.to_draw_multi_instanced().is_empty());
        world.resource_mut::<DormerTiles>().allocations.insert(wall, allocation);
        world.insert_resource(StairsTopDownMesh { buffer: country_core::rhapsody_contract::ManagedHandle(72), vertex_count: 3 });

        let mut system = IntoSystem::into_system(decorators_on_glade_unload);
        system.initialize(&mut world);
        system.run_without_applying_deferred((), &mut world);
        assert!(world.contains_resource::<StairsTopDownMesh>(), "原 Commands 移除必须延迟");
        assert!(world.get_entity(entity).is_ok(), "原函数不负责实体销毁");
        assert!(weak_blueprint.upgrade().is_none(), "两张 Arc 蓝图表必须同时释放");
        let storage = world.resource::<DecoratorStorage>();
        assert!(storage.per_wall.is_empty() && storage.terrain.is_none() && storage.stairs.is_none() && storage.removed.is_empty());
        assert!(!storage.immutable && storage.decorator_to_storage.get(id).is_none());
        assert!(world.resource::<LiveDecoratorArchivist>().collision_a.is_empty());
        assert!(!world.resource::<LiveDecoratorArchivist>().culling.allow_any);
        assert!(world.resource::<LiveDecoratorArchivist>().culling.allowed_ids.is_empty());
        assert!(world.resource::<DeferredDecoratorEditHistory>().records.is_empty());
        assert!(!world.resource::<DeferredDecoratorEditHistory>().pending);
        assert!(world.resource::<DecoratorVisualState>().persistent.entries.is_empty());
        assert!(world.resource::<DecoratorVisualState>().transient.entities.is_empty());
        assert_eq!(world.resource_mut::<DecoratorIdGen>().generate(), DecoratorId(0));
        assert_eq!(*world.resource::<PlaceDecoratorInteractionState>(), PlaceDecoratorInteractionState::default());
        assert_eq!(world.resource::<DecoratorAudioEvents>().state(), [0; 4]);
        assert!(world.resource::<GlobalWindowLightsControl>().lights.is_empty());
        assert!(!world.resource::<GlobalWindowLightsControl>().lights_on);
        assert_eq!(world.resource::<DecoratorAffordanceDispatcher>().state, 6);
        assert_eq!(world.resource::<DecoratorAffordanceDispatcher>().payload, 0);
        assert!(world.resource::<DecoratorPhysicsColliders>().first.is_empty() && world.resource::<DecoratorPhysicsColliders>().second.is_empty());
        assert!(world.resource::<StoneColumnCollisionStorage>().cleared.is_empty());
        assert!(world.resource::<DormerTiles>().allocations.is_empty());
        world.resource_mut::<WallAttachedDecoTracker<ChimneyAssemblyParams>>().with_diffs(|_, _| panic!("卸载不得保留烟囱差分"));
        world.resource_mut::<WallAttachedDecoTracker<DoorStairsAssemblyParams>>().with_diffs(|_, _| panic!("卸载不得保留门梯差分"));
        assert_eq!(world.resource::<StairsState>().graph().node_count(), 0);
        assert!(!world.resource::<StairsState>().is_backed_up());
        assert!(world.resource::<StairColliderRegistry>().edge_colliders.iter().all(|map| map.is_empty()));
        assert!(world.resource::<StairColliderRegistry>().structural_colliders.is_empty());
        assert!(world.resource::<StairsVisualState>().edges().is_empty());
        assert!(!world.resource::<StairsVisualState>().is_dirty());
        let graph = world.resource::<StairsStructuralGraph>();
        assert_eq!(graph.graph().node_count(), 0);
        assert!(graph.collider_to_span().is_empty());
        assert_eq!(graph.collider_to_span().capacity(), capacity, "原图 clear 保留容量");
        assert!(!tiles.to_draw_multi_instanced().is_empty(), "原 sparse 退役要等资源宿主回收");
        tiles.take_out_waste();
        assert!(tiles.to_draw_multi_instanced().is_empty(), "卸载必须退役真实 dormer 分配");
        assert!(weak_mesh.upgrade().is_some(), "活碰撞世界在其原卸载前仍持有网格");
        system.apply_deferred(&mut world);
        assert!(!world.contains_resource::<StairsTopDownMesh>());
        // 原来相邻的通用卸载 owner：仅销毁没有 KeepOnLevelUnload 的实体。
        use bevy_ecs::system::RunSystemOnce;
        world.run_system_once(country_core::startup::clearing::despawn_glade_entities).unwrap();
        assert!(world.get_entity(entity).is_err());
        assert!(world.get_entity(kept).is_ok());
        world.resource_mut::<RaycastWorld>().on_level_reload();
        assert!(weak_mesh.upgrade().is_none(), "全局碰撞重载释放最后一个网格 Arc");
        // 缺失 top-down 的第二次卸载也合法，不要求伪造可选资源。
        system.run_without_applying_deferred((), &mut world);
        system.apply_deferred(&mut world);
        assert!(world.get_entity(kept).is_ok());
    }
}
