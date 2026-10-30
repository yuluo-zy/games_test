//! Original building and forest runtime assembly on the gameplay App's owners.
//! Existing real producers only. No draw/edit UI, platform TODO, legacy main
//! construct_walls, empty path sampler, or duplicate AssetMaintenance is mounted.
use bevy_app::{App, Update};
use bevy_ecs::prelude::*;
use crate::run_game::GameSystemSet;
use country_core::{resources as res, systems as sys};

pub(super) fn install_resources(app: &mut App) {
    // Critter systems/resources (control_critters, butterflies/fireflies GPU upload,
    // CritterControl) belong to CritterPlugin14067d440 (run_game::add_original_plugins).
    // Owning empty constructors run before actual load; init preserves existing
    // authoritative content and never resets a loaded scene to make it start.
    sys::wall::registration::register_wall_cpu_resources(app.world_mut());
    crate::wall_constructor_plugin::build(app);
    app.init_resource::<res::walls::wooden_gates::Gates>();
    app.init_resource::<res::walls::arches::ArchSegments>();
    app.init_resource::<res::walls::arches::LintelSegments>();
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

}

pub(super) fn install(app: &mut App) {
    install_resources(app);

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
    // 与原add_wall_shape_ux共用唯一缓存；初始化不能覆盖已加载的权威内容。
    app.init_resource::<sys::wall::shapes::calc_shape_height::TimberframeConstructionParamsCache>();
    // 地形聚焦已恢复真实Cursor/Rapier/UiState与Local，不再初始化三份无生产者镜像。
    // 下列兼容资源仍由各原owner逐项审阅；初始化本身不证明工具链已验收。
    use sys::terrain::{ui_draw_terrain as draw, ui_edit_terrain as edit,
        deferred_terrain_history as hist};
    app.init_resource::<draw::TerrainDrawState>();
    app.init_resource::<edit::TerrainEditUiState>();
    app.init_resource::<edit::TerrainWidgetResponseTable>();
    app.init_resource::<edit::StairsPlacementBucket>();
    app.init_resource::<edit::TerrainEditHistoryPending>();
    app.init_resource::<hist::PendingTerrainHistory>();
    // ShowPinnedDecoClutter is an Event (56 B) in the original (registered by ClutterPlugin as its
    // 8th register_event); decoclutter_show_pins reads it through an EventReader.
    app.add_event::<sys::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter>();
    // OPEN: HintResetState is an onboarding-crate adapter (reset_hints state).
    app.init_resource::<system_onboarding::reset_hints::HintResetState>();
    // OPEN: SheepInputStream is a crate seam; the original gate is a field of an
    // unidentified native resource (+0x40). Empty stream => the system returns early.
    app.init_resource::<system_critters::sheep::sheep_input_record_replay::SheepInputStream>();
    // OPEN: onboarding seam events (unrecovered original trigger sources) and the
    // ToolbarUiIsInFocus event (toolbar-owned in the original, per the onboarding report).
    app.add_event::<country_core::systems::ui_edit_focus::ToolSelectionCancelled>();
    app.add_event::<system_onboarding::reset_hints::ResetHintsCommand>();
    app.add_event::<system_onboarding::hint_orchestrator::ToolbarUiIsInFocus>();
    // Terrain-tool adapter events with no recovered original registrar.
    app.add_event::<edit::TerrainWindowFocus>();
}

#[cfg(test)]
mod segmentation_chain_tests {
    use crate::app_systems::WallSystems;
    use bevy_app::{App, Update};
    use bevy_ecs::schedule::IntoScheduleConfigs;
    use country_core::systems::GameSystemSet;

    /// 原版 0x140493120 把 `chain_ignore_deferred(chain1, chain2)` 放进 Update/set 19，
    /// chain2 尾部三项（0x140335d53/5f/68）依次为 sample_wall_path_intersections →
    /// calculate_path_wall_segmentation → init_wall_height_and_arch_segment_height；
    /// set 22（startup_construction_runtime）里 split_wall_path_segment_into_platform_stairs
    /// 读取前者产出的原始掩码，必须排在其后。这里按宿主的 set 链构图并读出拓扑序。
    #[test]
    fn segmentation_chain_is_scheduled_in_original_order() {
        let mut app = App::new();
        for stage in 0..44 {
            app.configure_sets(Update, GameSystemSet(stage).before(GameSystemSet(stage + 1)));
        }
        app.add_wall_systems();
        super::install(&mut app);
        let order: Vec<String> = app.world_mut().schedule_scope(Update, |world, schedule| {
            schedule.initialize(world).expect("Update schedule must build");
            schedule
                .systems()
                .expect("initialized")
                .map(|(_, system)| system.name().to_string())
                .collect()
        });
        let pos = |needle: &str| {
            let hits: Vec<usize> = order
                .iter()
                .enumerate()
                .filter(|(_, name)| name.contains(needle))
                .map(|(i, _)| i)
                .collect();
            assert_eq!(hits.len(), 1, "{needle} must be registered exactly once: {hits:?}");
            hits[0]
        };
        let sample = pos("sample_wall_path_intersections::sample_wall_path_intersections");
        let calc = pos("calculate_wall_path_segmentation::calculate_wall_path_segmentation");
        let init = pos("init_wall_height_and_arch_segment_height::init_wall_height_and_arch_segment_height");
        let gate_clear = pos("gpu_clear_wall_gate_pieces");
        let split = pos("split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs");
        let arches = pos("construct_arches::construct_arches");
        let lintels = pos("construct_lintels::construct_lintels");
        let adjust = pos("adjust_wall_height_impl::adjust_wall_height_impl");
        assert!(adjust < gate_clear && gate_clear < sample, "set 19 chain2 head order");
        assert!(sample < calc && calc < init, "set 19 chain2 tail order");
        assert!(init < split && split < arches && arches < lintels, "set 19 before set 22 chain");
    }
}
