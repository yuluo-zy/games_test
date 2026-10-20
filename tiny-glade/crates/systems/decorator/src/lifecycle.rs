//! Original `decorators_on_glade_unload` (140347090, 9451 bytes), scoped to
//! owners represented by the current source. This is not the full native
//! unload entry: StairColliderRegistry and several owner payloads still need
//! recovery. Do not mount this as a complete replacement for that entry.
//!
//! Each aggregate replacement below is individually proved by the native body.
//! StructuralGraph and Affordance instead use their original clear operations.
//! Entity destruction and same-buffer waste collection belong to their existing
//! separate consumers; this helper must not impersonate either consumer.

use bevy_ecs::{prelude::*, system::SystemParam};
use country_core::resources::{
    roofs::dormer_tiles::DormerTiles,
    stairs::{
        stairs_state::StairsState, stairs_top_down::StairsTopDownMesh,
        structural_graph::StairsStructuralGraph, visual::StairsVisualState,
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
    doors: ResMut<'w, WallAttachedDecoTracker<DoorStairsAssemblyParams>>,
    stairs_visual: ResMut<'w, StairsVisualState>,
    structural_graph: ResMut<'w, StairsStructuralGraph>,
    commands: Commands<'w, 's>,
}

/// Restore represented owners in the original mutation/drop order. This helper
/// is intentionally not registered by itself: the missing native registry reset
/// belongs between StairsState and DoorTracker. Audio, interaction, columns,
/// stair visual and graph schemas also retain documented recovery gaps.
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
    // Native StairColliderRegistry replacement is not yet representable here.
    *owners.doors = WallAttachedDecoTracker::default();
    *owners.stairs_visual = StairsVisualState::default();
    owners.structural_graph.clear();
    // Unlike Default, clear writes state=6/payload=0 and preserves ty.
    owners.affordance.clear();
    // Native queues this command last. Resource removal stays deferred and does
    // not claim retirement of the current logical ManagedHandle's buffer.
    owners.commands.remove_resource::<StairsTopDownMesh>();
}
