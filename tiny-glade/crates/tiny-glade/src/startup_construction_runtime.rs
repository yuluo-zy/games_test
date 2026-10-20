//! Original building and forest runtime assembly on the gameplay App's owners.
//! Existing real producers only. No draw/edit UI, platform TODO, legacy main
//! construct_walls, empty path sampler, or duplicate AssetMaintenance is mounted.
use bevy_app::{App, Update};
use bevy_ecs::prelude::*;
use crate::run_game::GameSystemSet;
use country_core::{resources as res, systems as sys};

pub(super) fn install(app: &mut App) {
    // Critter systems/resources (control_critters, butterflies/fireflies GPU upload,
    // CritterControl) belong to CritterPlugin14067d440 (run_game::add_original_plugins).
    // Owning empty constructors run before actual load; init preserves existing
    // authoritative content and never resets a loaded scene to make it start.
    sys::wall::registration::register_wall_cpu_resources(app.world_mut());
    crate::wall_constructor_plugin::build(app);
    app.init_resource::<res::walls::wooden_gates::Gates>();
    app.init_resource::<res::walls::arches::ArchSegments>();
    app.init_resource::<res::walls::arches::BuildingHadArchesLastFrame>();
    app.init_resource::<res::walls::public_walls::WallEntities>();
    app.init_resource::<res::walls::public_walls::WallPlasters>();
    app.init_resource::<res::walls::WallHighlight>();
    app.init_resource::<res::walls::wall_holes::WallHoles>();
    app.init_resource::<res::walls::wall_holes::PrevWallHoles>();
    app.init_resource::<sys::wall::construct_wall_bricks::WallBrickBuildState>();
    app.init_resource::<res::rt::RtWorld>();
    app.init_resource::<sys::ui::signifiers::signifier_create::UiSignifierStream>();
    app.add_event::<res::walls::public_walls::OnWallChanged>();
    app.add_event::<res::walls::public_walls::OnWallDeleted>();
    app.add_event::<sys::wall::construct_wall_bricks::RecalculateWallBricks>();
    app.add_event::<sys::wall::construct_wall_bricks::WallBricksChanged>();
    app.add_event::<sys::wall::calc_wall_bend::WallBendChangedEvent>();
    app.add_event::<res::rt::UpdateRtWorldEvent>();
    app.add_event::<system_wall_constructor::construct_gates::systems::ConstructGateCmd>();
    app.add_event::<res::roofs::commands::WallGableChanged>();
    app.add_event::<res::roofs::commands::WallRoofPresenceChanged>();
    app.add_event::<res::roofs::commands::RoofIsInvalid>();

    // The ForestPlugin140650c50 chain and InteractableForestPlacements, and the
    // timberframe ConstructTimberframeCmd event, are mounted once by
    // run_game::add_original_plugins.

    install_open_adapter_inputs(app);

    // OPEN (not closed): wall_roof_hashmap and process_invalid_roof_notifications are NOT
    // members of the original add_roof_systems@14062fcb0 (RoofSystems); their original
    // registration point is unlocated, so they stay here exactly as before.
    // The root patch extends the EXISTING19 publication tuple in place. The
    // second copy of publication/spatial systems must never be installed here.
    app.add_systems(Update, (
        system_roof::create_roof::wall_roof_hashmap,
        system_roof::delete_roof::process_invalid_roof_notifications,
        system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs,
        system_wall_constructor::construct_arches::construct_arches,
        system_wall_constructor::construct_lintels::construct_lintels,
    ).chain_ignore_deferred().in_set(GameSystemSet(22)));
    app.add_systems(Update,
        system_wall_constructor::gpu_output::flush_constructor_output.in_set(GameSystemSet(44)));
}

/// OPEN (NOT CLOSED) inputs of the plugin-registered systems. Every item below is a
/// Rust-side seam/adapter of a recovered crate with no recovered original creator
/// (the original system reads a different native owner or an unrecovered event).
/// They are created in their idle (empty/zero) state ONLY so that the Bevy system
/// parameter validation of the original-registered systems passes; no producer is
/// wired, so the dependent behaviour stays unclosed until the owning domain replaces
/// the seam with the real source. None of this is claimed as original behaviour.
fn install_open_adapter_inputs(app: &mut App) {
    // Original creator is known and exact: add_wall_shape_ux@140945f50 inserts it
    // all-zero (u32@+0 = 0, u64@+8 = 0), read by timberframe's two construct systems.
    // The remaining half of add_wall_shape_ux (8 shape systems + 4 resources) belongs to
    // the wall owner's pass and is intentionally not mounted here.
    app.insert_resource(
        sys::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache::default());
    // OPEN: terrain tool adapters (country-core terrain/*): "I" seams whose native
    // owners (CursorTerrainRaycast/Rapier raycast/SSBO readback/UI context) are not yet
    // bridged; no host writer exists, so hover/focus/selection stay idle.
    use sys::terrain::{terrain_hover_idx as hover, ui_draw_terrain as draw,
        ui_edit_terrain as edit, ui_terrain_focus_mode as focus, deferred_terrain_history as hist};
    app.init_resource::<hover::MouseTerrainHoverReadback>();
    app.init_resource::<hover::TerrainWaterSample>();
    app.init_resource::<draw::TerrainHoverInput>();
    app.init_resource::<draw::TerrainDrawState>();
    app.init_resource::<edit::TerrainEditUiState>();
    app.init_resource::<edit::TerrainWidgetResponseTable>();
    app.init_resource::<edit::StairsPlacementBucket>();
    app.init_resource::<edit::TerrainEditHistoryPending>();
    app.init_resource::<focus::TerrainFocusRaycast>();
    app.init_resource::<focus::TerrainFocusHover>();
    app.init_resource::<focus::TerrainFocusSelection>();
    app.init_resource::<hist::PendingTerrainHistory>();
    // ShowPinnedDecoClutter is an Event (56 B) in the original (registered by ClutterPlugin as its
    // 8th register_event); decoclutter_show_pins reads it through an EventReader.
    app.add_event::<sys::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter>();
    {
        use sys::ui_clutter::decoclutter_show_pins as pins;
        app.init_resource::<pins::PinnedDecoClutter>();
        app.init_resource::<pins::PinnedDecoGeometryTable>();
        app.init_resource::<pins::PinnedDecoClutterBuffers>();
        app.init_resource::<pins::PinAnimationTable>();
        app.init_resource::<pins::PinHitDedup>();
        app.add_event::<pins::PinVisibilityToggle>();
    }
    // OPEN: HintResetState is an onboarding-crate adapter (reset_hints state).
    app.init_resource::<system_onboarding::reset_hints::HintResetState>();
    // OPEN: SheepInputStream is a crate seam; the original gate is a field of an
    // unidentified native resource (+0x40). Empty stream => the system returns early.
    app.init_resource::<system_critters::sheep::sheep_input_record_replay::SheepInputStream>();
    // OPEN: onboarding seam events (unrecovered original trigger sources) and the
    // ToolbarUiIsInFocus event (toolbar-owned in the original, per the onboarding report).
    app.add_event::<system_onboarding::hint_deselect_tool::DeselectReset>();
    app.add_event::<system_onboarding::reset_hints::ResetHintsCommand>();
    app.add_event::<system_onboarding::hint_orchestrator::ToolbarUiIsInFocus>();
    // Terrain-tool adapter events with no recovered original registrar.
    app.add_event::<edit::TerrainWindowFocus>();
    app.add_event::<focus::SignifierDrawTopology>();
    app.add_event::<focus::TerrainFocusTrigger>();
}
