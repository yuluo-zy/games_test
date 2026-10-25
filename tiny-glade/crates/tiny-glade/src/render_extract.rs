//! The production hand-over (reports/render-thread-2026-10-22/ARCHITECTURE.md):
//! everything declaration reads, collected from the modules that read it.
use crate::render_thread::RenderExchange;

pub fn production_exchange() -> RenderExchange {
    use country_core::systems::render_loop::draw_passes as dp;
    let mut set = RenderExchange::default();
    let m = &mut set;
    // country-core declaration modules.
    dp::draw_lists::render_extract(m);
    dp::draw_callbacks_a::render_extract(m);
    dp::draw_callbacks_b::render_extract(m);
    dp::common_draw_params::render_extract(m);
    dp::custom_cursor_pass::render_extract(m);
    // Host declaration modules.
    crate::production_renderer::render_extract(m);
    crate::render_bridge::render_extract(m);
    crate::frame_setup::render_extract(m);
    crate::loading_graph::render_extract(m);
    crate::raster_host::render_extract(m);
    crate::menu_background_host::render_extract(m);
    crate::world_postprocess::session_preview_host::render_extract(m);
    crate::mei_host::render_extract(m);
    crate::screenshot::shared_texture_capture::render_extract(m);
    crate::world_graph::render_extract(m);
    crate::world_postprocess::render_extract(m);
    crate::world_geometry::render_extract(m);
    crate::world_vegetation::render_extract(m);
    crate::world_walls::render_extract(m);
    crate::world_terrain::render_extract(m);
    crate::world_lighting::render_extract(m);
    crate::scene_compute::render_extract(m);
    crate::scene_raster::render_extract(m);
    crate::screenshot::shared_texture_capture::render_exchange(&mut set);
    write_backs(&mut set);
    set
}

/// Declaration's writes the game world reads (ARCHITECTURE.md step 3).
fn write_backs(set: &mut RenderExchange) {
    use country_core::{
        render::extract::{RenderExtract, WriteBack},
        resources::{terrain_editing::TerrainEditingData, world_raster_defs as rasters},
        systems::render_loop::{
            compute_passes::read_terrain_data::ScheduledTerrainReadback,
            lut_gallery::LutGalleryState,
            painterly::PainterlyState,
            session_selection::ThumbnailTextures,
            vermeer::{VermeerDebugControl, VermeerState},
        },
    };
    // Whole values: reveal/painterly progress, gallery
    // and thumbnail publication, the saved loading background.
    resolution_feedback(set);
    set.write_back::<VermeerState>();
    set.write_back::<VermeerDebugControl>();
    set.write_back::<PainterlyState>();
    set.write_back::<LutGalleryState>();
    set.write_back::<ThumbnailTextures>();
    set.write_back::<country_core::resources::shader_refactoring_state::ShaderRefactoringState>();
    set.write_back::<crate::loading_graph::SavedLoadingBackgroundTexture>();
    set.forward_events::<country_core::systems::save::load_game::StartupRevealFinished>();
    // Rasters: only the upload/consumer flags travel back; a raster the game
    // repainted since the extract keeps its own (the next frame consumes it).
    fn raster<R: bevy_ecs::resource::Resource + std::ops::Deref<Target = country_core::resources::world_raster::WorldRaster<512>> + std::ops::DerefMut>(
        render: &R,
    ) -> Option<WriteBack<R>> {
        let state = render.consumption();
        Some(Box::new(move |game: &mut R| game.adopt_consumption(state)))
    }
    set.invalidate_unsubmitted::<rasters::PathRaster>();
    set.invalidate_unsubmitted::<rasters::Cobblestone1Raster>();
    set.invalidate_unsubmitted::<rasters::Cobblestone2Raster>();
    set.invalidate_unsubmitted::<rasters::Cobblestone3Raster>();
    set.invalidate_unsubmitted::<rasters::WaterRaster>();
    set.invalidate_unsubmitted::<rasters::GardenRaster>();
    set.invalidate_unsubmitted::<TerrainEditingData>();
    set.invalidate_unsubmitted::<ScheduledTerrainReadback>();
    set.after_submission(|set| {
    set.write_back_with::<rasters::PathRaster>(true, raster);
    set.write_back_with::<rasters::Cobblestone1Raster>(true, raster);
    set.write_back_with::<rasters::Cobblestone2Raster>(true, raster);
    set.write_back_with::<rasters::Cobblestone3Raster>(true, raster);
    set.write_back_with::<rasters::WaterRaster>(true, raster);
    set.write_back_with::<rasters::GardenRaster>(true, raster);
    set.write_back_with::<TerrainEditingData>(true, |render| {
        let rasterized = render.rasterized;
        Some(Box::new(move |game: &mut TerrainEditingData| game.0.rasterized = rasterized))
    });
    // Terrain readback: requests are atomic increments through `&self`, so
    // subtract exactly what the frame consumed instead of overwriting.
    let consumed = std::sync::Arc::new(parking_lot::Mutex::new(None));
    let at_extract = consumed.clone();
    set.extract.add(move |game, render, _| {
        *at_extract.lock() = game.get_resource_change_ticks::<ScheduledTerrainReadback>().map(|ticks| {
            let pending = render.get_resource::<ScheduledTerrainReadback>()
                .map_or(0, |readback| readback.pending.load(std::sync::atomic::Ordering::Acquire));
            (game.id(), ticks.added, ticks.changed, pending)
        });
    });
    set.write_back.add(move |render, at, out| {
        let Some(readback) = render.get_resource::<ScheduledTerrainReadback>() else {
            return;
        };
        let Some((source, added, changed, before)) = *consumed.lock() else { return; };
        if !render.get_resource_change_ticks::<ScheduledTerrainReadback>().is_some_and(|ticks|
            ticks.changed.is_newer_than(at.render, render.read_change_tick())) { return; }
        let taken = before.saturating_sub(readback.pending.load(std::sync::atomic::Ordering::Acquire));
        let (issued, tick) = (readback.copy_issued, readback.changed_tick);
        if taken == 0 && !issued {
            return;
        }
        out.push(Box::new(move |game: &mut bevy_ecs::world::World| {
            if game.id() != source { return; }
            let Some(ticks) = game.get_resource_change_ticks::<ScheduledTerrainReadback>() else { return; };
            if ticks.added != added { return; }
            let merge_flags = ticks.changed == changed;
            if let Some(mut readback) = game.get_resource_mut::<ScheduledTerrainReadback>() {
                // Requests can arrive through shared references during render.
                // Subtract only this frame's consumption without losing increments.
                let _ = readback.pending.fetch_update(std::sync::atomic::Ordering::AcqRel,
                    std::sync::atomic::Ordering::Acquire, |pending| Some(pending.saturating_sub(taken)));
                if merge_flags {
                    readback.copy_issued |= issued;
                    readback.changed_tick = tick;
                }
            }
        }));
    });
    });
}

/// Adaptive feedback is a field merge: resize and unrelated settings may run
/// while rendering. An OOM latch must survive those updates, but cannot cross
/// an App/resource replacement or supersede a newer requested scale.
fn resolution_feedback(set: &mut RenderExchange) {
    use country_core::{game_config::GameConfig, render::{
        adaptive_render_state::AdaptiveRenderState, render_config::FrameRenderConfig}};
    let source = std::sync::Arc::new(parking_lot::Mutex::new(None));
    let capture = source.clone();
    set.extract.add(move |game, _, _| {
        *capture.lock() = (|| {
            Some((game.id(), game.get_resource_change_ticks::<FrameRenderConfig>()?.added,
                game.get_resource_change_ticks::<AdaptiveRenderState>()?.added,
                game.get_resource::<GameConfig>()?.video.world_resolution_scale.map(f32::to_bits),
                *game.get_resource::<AdaptiveRenderState>()?,
                game.get_resource::<FrameRenderConfig>()?.world_resolution_scale.to_bits()))
        })();
    });
    set.write_back.add(move |render, _, out| {
        let Some((id, frame_added, state_added, requested, before, before_scale)) = *source.lock() else { return; };
        let (Some(state), Some(frame)) = (render.get_resource::<AdaptiveRenderState>().copied(), render.get_resource::<FrameRenderConfig>()) else { return; };
        let scale = frame.world_resolution_scale;
        if state == before && scale.to_bits() == before_scale { return; }
        out.push(Box::new(move |game: &mut bevy_ecs::world::World| {
            if game.id() != id
                || game.get_resource_change_ticks::<FrameRenderConfig>().map(|t| t.added) != Some(frame_added)
                || game.get_resource_change_ticks::<AdaptiveRenderState>().map(|t| t.added) != Some(state_added)
                || game.get_resource::<GameConfig>().map(|c| c.video.world_resolution_scale.map(f32::to_bits)) != Some(requested)
                || game.get_resource::<AdaptiveRenderState>().map(|s| s.enabled) != Some(state.enabled) { return; }
            *game.resource_mut::<AdaptiveRenderState>() = state;
            game.resource_mut::<FrameRenderConfig>().world_resolution_scale = scale;
            crate::frame_resolution::refresh(game);
        }));
    });
}


#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::world::World;

    #[test]
    fn adaptive_feedback_merges_resize_and_preserves_user_target() {
        use country_core::{game_config::GameConfig, resources::window::WindowSize,
            render::{adaptive_render_state::AdaptiveRenderState, render_config::FrameRenderConfig}};
        for newer_request in [false, true] {
            let mut game = World::new();
            let config = GameConfig::default();
            let window = WindowSize::new(640, 360);
            game.insert_resource(FrameRenderConfig::at_startup(window, &config.video));
            game.insert_resource(config);
            game.insert_resource(window);
            game.insert_resource(AdaptiveRenderState::at_startup(false));
            let mut render = World::new();
            let mut exchange = production_exchange();
            exchange.extract.run(&mut game, &mut render);
            render.resource_mut::<FrameRenderConfig>().world_resolution_scale = 0.5;
            render.resource_mut::<AdaptiveRenderState>().mark_out_of_memory();
            let mut feedback = Vec::new();
            exchange.write_back.run(&mut render, &mut feedback);
            game.insert_resource(WindowSize::new(800, 600));
            game.resource_mut::<FrameRenderConfig>().flag_20 = true;
            if newer_request { game.resource_mut::<GameConfig>().video.world_resolution_scale = Some(0.75); }
            for apply in feedback { apply(&mut game); }
            let frame = game.resource::<FrameRenderConfig>();
            assert!(frame.flag_20);
            if newer_request {
                assert_eq!(game.resource::<AdaptiveRenderState>().native_tag(), 0);
                assert_eq!(frame.world_resolution_scale, 1.0);
            } else {
                assert_eq!(frame.extent_8, [400, 300]);
                assert_eq!(frame.world_resolution_scale, 0.5);
                assert_eq!(game.resource::<AdaptiveRenderState>().native_tag(), 2);
                assert_eq!(game.resource::<GameConfig>().video.world_resolution_scale, None);
            }
        }
    }

    #[test]
    fn sheep_material_reads_canonical_owners_after_production_extraction() {
        use country_core::{
            resources::critters::sheep::{Sheep, SheepMoveState, SheepWalkRecord},
            systems::render_loop::draw_passes::draw_callbacks_a::{PrevTransformView, read_env2},
        };
        let mut game = World::new();
        let entity = game.spawn((
            Sheep {
                behavior_state: 0, goal_timer: 0.0, pet_phase: 0, pet_progress: 0.0,
                pet_target: 0, move_state: SheepMoveState::Idle { timeout_s: 0.0 },
                amplitude: 1.5, control_flag: 1,
            },
            SheepWalkRecord { frame_index: 7, ..Default::default() },
            PrevTransformView { rotation: glam::Quat::IDENTITY,
                translation: glam::Vec3::ZERO, scale: glam::Vec3::ONE },
        )).id();
        let expected = read_env2(&game, entity);
        let mut render = World::new();
        let mut exchange = production_exchange();
        exchange.extract.run(&mut game, &mut render);
        assert_eq!(read_env2(&render, entity), expected);
        game.component_mut::<SheepWalkRecord>(entity).unwrap().frame_index = 11;
        assert_eq!(read_env2(&render, entity), expected);
        exchange.extract.run(&mut game, &mut render);
        assert_eq!(read_env2(&render, entity).sheep.q28, 11);
        game.entity_mut(entity).remove::<SheepWalkRecord>();
        exchange.extract.run(&mut game, &mut render);
        assert!(render.get::<SheepWalkRecord>(entity).is_none());
    }
    use country_core::systems::render_loop::compute_passes::read_terrain_data::ScheduledTerrainReadback;
    use std::sync::atomic::Ordering;

    #[test]
    fn terrain_requests_survive_failed_submission_and_merge_later_requests() {
        let mut game = World::new();
        let mut render = World::new();
        game.init_resource::<ScheduledTerrainReadback>();
        let mut exchange = production_exchange();
        exchange.extract.run(&mut game, &mut render);
        game.resource::<ScheduledTerrainReadback>().on_terrain_modified();
        exchange.extract.run(&mut game, &mut render);
        assert_eq!(render.resource::<ScheduledTerrainReadback>().pending.load(Ordering::Acquire), 1);
        render.resource_mut::<ScheduledTerrainReadback>().on_gpu_copy_issued(17);
        let mut out = Vec::new();
        exchange.write_back.run(&mut render, &mut out);
        assert!(out.is_empty(), "declaration is not a GPU consumption acknowledgment");
        exchange.write_back.abort_unsubmitted(&mut render);
        exchange.extract.run(&mut game, &mut render);
        assert_eq!(render.resource::<ScheduledTerrainReadback>().pending.load(Ordering::Acquire), 1);
        render.resource_mut::<ScheduledTerrainReadback>().on_gpu_copy_issued(18);
        game.resource::<ScheduledTerrainReadback>().on_terrain_modified();
        exchange.write_back.run_submitted(&mut render, &mut out);
        assert!(!out.is_empty());
        for apply in out { apply(&mut game); }
        let readback = game.resource::<ScheduledTerrainReadback>();
        assert_eq!(readback.pending.load(Ordering::Acquire), 1);
        assert!(readback.copy_issued);
        assert_eq!(readback.changed_tick, 18);
    }

    #[test]
    fn terrain_feedback_cannot_cross_application_or_resource_replacement() {
        for replace in [false, true] {
            let mut game = World::new();
            let mut render = World::new();
            game.init_resource::<ScheduledTerrainReadback>();
            game.resource::<ScheduledTerrainReadback>().on_terrain_modified();
            let mut exchange = production_exchange();
            exchange.extract.run(&mut game, &mut render);
            render.resource_mut::<ScheduledTerrainReadback>().on_gpu_copy_issued(19);
            let mut out = Vec::new();
            exchange.write_back.run_submitted(&mut render, &mut out);
            if replace { game.remove_resource::<ScheduledTerrainReadback>(); }
            else { game = World::new(); }
            game.init_resource::<ScheduledTerrainReadback>();
            game.resource::<ScheduledTerrainReadback>().on_terrain_modified();
            for apply in out { apply(&mut game); }
            let readback = game.resource::<ScheduledTerrainReadback>();
            assert_eq!(readback.pending.load(Ordering::Acquire), 1);
            assert!(!readback.copy_issued);
        }
    }
}

use country_core::render::extract::WorldComponentWrite as _;
