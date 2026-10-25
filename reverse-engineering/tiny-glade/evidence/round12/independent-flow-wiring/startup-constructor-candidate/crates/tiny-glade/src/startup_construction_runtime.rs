//! Evidence-only startup assembly candidate; root owns production installation.
//! Existing real producers only. No draw/edit UI, platform TODO, legacy main
//! construct_walls, empty path sampler, or duplicate AssetMaintenance is mounted.
use bevy_app::{App, Update};
use bevy_ecs::prelude::*;
use crate::run_game::GameSystemSet;
use country_core::{resources as res, systems as sys};

pub(super) fn install(app: &mut App) {
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
    app.init_resource::<sys::render_loop::draw_passes::draw_callbacks_a::RoofHighlight>();
    app.add_event::<res::walls::public_walls::OnWallChanged>();
    app.add_event::<res::walls::public_walls::OnWallDeleted>();
    app.add_event::<sys::wall::construct_wall_bricks::RecalculateWallBricks>();
    app.add_event::<sys::wall::construct_wall_bricks::WallBricksChanged>();
    app.add_event::<sys::wall::calc_wall_bend::WallBendChangedEvent>();
    app.add_event::<res::rt::UpdateRtWorldEvent>();
    app.add_event::<system_wall_constructor::construct_gates::systems::ConstructGateCmd>();
    app.add_event::<res::roofs::commands::HighlightRoofCmd>();
    app.add_event::<res::roofs::commands::RoofSound>();
    app.add_event::<res::roofs::commands::CreateRoofCmd>();
    app.add_event::<res::roofs::commands::SwitchRoofTypeCmd>();
    app.add_event::<res::roofs::commands::RoofGeometryChanged>();
    app.add_event::<res::roofs::commands::WallGableChanged>();
    app.add_event::<res::roofs::commands::WallRoofPresenceChanged>();
    app.add_event::<res::roofs::commands::RoofIsInvalid>();

    // The root patch extends the EXISTING19 publication tuple in place. The
    // second copy of publication/spatial systems must never be installed here.
    app.add_systems(Update, (
        system_roof::create_roof::wall_roof_hashmap,
        system_roof::delete_roof::process_invalid_roof_notifications,
        system_wall_constructor::segmentation::split_wall_path_segment_into_platform_stairs::split_wall_path_segment_into_platform_stairs,
        system_wall_constructor::construct_arches::construct_arches,
        system_wall_constructor::construct_lintels::construct_lintels,
    ).chain_ignore_deferred().in_set(GameSystemSet(22)));
    app.add_systems(Update, (
        system_wall_constructor::construct_gates::systems::update_gates_hashmap,
        system_wall_constructor::construct_gates::systems::construct_gates,
        system_wall_constructor::construct_fences::construct_fences,
        // Current core owner finalizes actual holes before its full producer.
        // This explicit adapter edge is not a recovered separate native item.
        sys::wall::construct_wall_bricks::finalize_wall_holes,
        sys::wall::construct_wall_bricks::construct_wall_bricks,
    ).chain_ignore_deferred().in_set(GameSystemSet(28)));
    app.add_systems(Update,
        system_wall_constructor::gpu_output::flush_constructor_output.in_set(GameSystemSet(44)));
}
