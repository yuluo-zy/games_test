//! Original `decorators_on_glade_unload` (140347090, 9451 bytes), scoped to
//! owners represented by the current source. This is not the full native
//! 卸载入口仍有完整载荷模型前提；本次把唯一StairColliderRegistry补回原顺序。
//! 不能把当前可表示owner的重置当作原9451B入口已完整恢复并提前挂载。
//!
//! Each aggregate replacement below is individually proved by the native body.
//! StructuralGraph and Affordance instead use their original clear operations.
//! Entity destruction and same-buffer waste collection belong to their existing
//! separate consumers; this helper must not impersonate either consumer.

use bevy_ecs::{prelude::*, system::SystemParam};
use country_core::resources::{
    roofs::dormer_tiles::DormerTiles,
    stairs::{
        stairs_collider::StairColliderRegistry, stairs_state::StairsState,
        stairs_top_down::StairsTopDownMesh, structural_graph::StairsStructuralGraph,
        visual::StairsVisualState,
    },
    walls::{
        decorator::DecoratorIdGen, decorator_archivist::LiveDecoratorArchivist,
        decorator_collision::DecoratorPhysicsColliders, decorator_storage::DecoratorStorage,
        wall_stone_columns::StoneColumnCollisionStorage,
    },
};

use crate::{
    audio::DecoratorAudioEvents,
    blueprint::{DecoratorBlueprints, PrevDecoratorBlueprints},
    decorator_visual::deco_differential_state::{
        ChimneyAssemblyParams, DoorStairsAssemblyParams, WallAttachedDecoTracker,
    },
    decorator_visual_state::DecoratorVisualState,
    deferred_decorator_history::DeferredDecoratorEditHistory,
    ui_systems::{
        ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher,
        ui_place_decorator::PlaceDecoratorInteractionState,
    },
    window_lights::GlobalWindowLightsControl,
};

/// All current owners are mandatory. Missing resources must fail system
/// initialization rather than silently skipping cleanup and declaring success.
#[derive(SystemParam)]
pub struct ExistingDecoratorUnloadOwners<'w, 's> {
    storage: ResMut<'w, DecoratorStorage>,
    blueprints: ResMut<'w, DecoratorBlueprints>,
    previous_blueprints: ResMut<'w, PrevDecoratorBlueprints>,
    archivist: ResMut<'w, LiveDecoratorArchivist>,
    deferred_history: ResMut<'w, DeferredDecoratorEditHistory>,
    visual: ResMut<'w, DecoratorVisualState>,
    id_gen: ResMut<'w, DecoratorIdGen>,
    interaction: ResMut<'w, PlaceDecoratorInteractionState>,
    audio: ResMut<'w, DecoratorAudioEvents>,
    window_lights: ResMut<'w, GlobalWindowLightsControl>,
    affordance: ResMut<'w, DecoratorAffordanceDispatcher>,
    physics: ResMut<'w, DecoratorPhysicsColliders>,
    columns: ResMut<'w, StoneColumnCollisionStorage>,
    dormer_tiles: ResMut<'w, DormerTiles>,
    chimneys: ResMut<'w, WallAttachedDecoTracker<ChimneyAssemblyParams>>,
    stairs: ResMut<'w, StairsState>,
    // 原secondary tuple首项：三条edge registry与一条(edge,sample) registry。
    stair_colliders: ResMut<'w, StairColliderRegistry>,
    doors: ResMut<'w, WallAttachedDecoTracker<DoorStairsAssemblyParams>>,
    stairs_visual: ResMut<'w, StairsVisualState>,
    structural_graph: ResMut<'w, StairsStructuralGraph>,
    commands: Commands<'w, 's>,
}

/// 按原写入与Drop顺序重置当前可表示的owner，不自行注册为完整卸载入口。
/// 原四表楼梯注册器位于StairsState和DoorTracker之间；其余载荷/宿主/真GPU
/// 生命周期前提见本次交付合同，不能用局部模型重置冒充完整原业务。
pub fn reset_scoped_existing_decorator_owners_on_glade_unload(
    mut owners: ExistingDecoratorUnloadOwners,
) {
    // RHS construction precedes dropping each old aggregate. Do not substitute
    // map.clear(), mem::take(), or a transaction that reorders owner destruction.
    *owners.storage = DecoratorStorage::default();
    *owners.blueprints = DecoratorBlueprints::default();
    *owners.previous_blueprints = PrevDecoratorBlueprints::default();
    *owners.archivist = LiveDecoratorArchivist::default();
    *owners.deferred_history = DeferredDecoratorEditHistory::default();
    *owners.visual = DecoratorVisualState::default();
    *owners.id_gen = DecoratorIdGen::default();
    *owners.interaction = PlaceDecoratorInteractionState::default();
    *owners.audio = DecoratorAudioEvents::default();
    *owners.window_lights = GlobalWindowLightsControl::default();
    *owners.physics = DecoratorPhysicsColliders::default();
    *owners.columns = StoneColumnCollisionStorage::default();
    *owners.dormer_tiles = DormerTiles::default();
    *owners.chimneys = WallAttachedDecoTracker::default();
    *owners.stairs = StairsState::default();
    *owners.stair_colliders = StairColliderRegistry::default();
    *owners.doors = WallAttachedDecoTracker::default();
    *owners.stairs_visual = StairsVisualState::default();
    owners.structural_graph.clear();
    // Unlike Default, clear writes state=6/payload=0 and preserves ty.
    owners.affordance.clear();
    // Native queues this command last. Resource removal stays deferred and does
    // not claim retirement of the current logical ManagedHandle's buffer.
    owners.commands.remove_resource::<StairsTopDownMesh>();
}

#[cfg(test)]
mod registry_reset_tests {
    use super::*;
    use bevy_ecs::system::SystemState;
    use country_core::rhapsody_contract::ManagedHandle;
    use std::sync::Arc;

    fn world() -> World {
        let mut world = World::new();
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
        world.insert_resource(StairsTopDownMesh {
            buffer: ManagedHandle(73),
            vertex_count: 6,
        });
        world
    }

    #[test]
    fn four_real_registry_owners_release_capacity_and_top_down_removal_stays_deferred() {
        let mut world = world();
        let entity = world.spawn_empty().id();
        let shared = Arc::new(crate::blueprint::DecoratorBlueprint::new());
        world
            .resource_mut::<DecoratorBlueprints>()
            .map
            .insert(7, shared.clone());
        world
            .resource_mut::<PrevDecoratorBlueprints>()
            .map
            .insert(7, shared.clone());
        let mut registry = world.resource_mut::<StairColliderRegistry>();
        for lane in &mut registry.edge_colliders {
            lane.reserve(64);
            assert!(lane.capacity() >= 64);
        }
        registry.structural_colliders.reserve(64);
        assert!(registry.structural_colliders.capacity() >= 64);
        world.resource_mut::<GlobalWindowLightsControl>().lights_on = true;
        let mut state = SystemState::<ExistingDecoratorUnloadOwners>::new(&mut world);
        reset_scoped_existing_decorator_owners_on_glade_unload(state.get_mut(&mut world));
        let registry = world.resource::<StairColliderRegistry>();
        for lane in &registry.edge_colliders {
            assert_eq!(lane.capacity(), 0);
        }
        assert_eq!(registry.structural_colliders.capacity(), 0);
        assert_eq!(Arc::strong_count(&shared), 1);
        assert!(!world.resource::<GlobalWindowLightsControl>().lights_on);
        assert!(world.get_entity(entity).is_ok());
        assert!(world.contains_resource::<StairsTopDownMesh>());
        state.apply(&mut world);
        assert!(!world.contains_resource::<StairsTopDownMesh>());
        assert!(world.get_entity(entity).is_ok());
    }

    #[test]
    fn missing_required_stair_registry_does_not_execute_a_partial_reset() {
        let mut world = world();
        world.remove_resource::<StairColliderRegistry>();
        world.resource_mut::<GlobalWindowLightsControl>().lights_on = true;
        let mut state = SystemState::<ExistingDecoratorUnloadOwners>::new(&mut world);
        let failure = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            reset_scoped_existing_decorator_owners_on_glade_unload(state.get_mut(&mut world));
        }));
        assert!(failure.is_err());
        assert!(world.resource::<GlobalWindowLightsControl>().lights_on);
        state.apply(&mut world);
        assert!(world.contains_resource::<StairsTopDownMesh>());
    }
}
