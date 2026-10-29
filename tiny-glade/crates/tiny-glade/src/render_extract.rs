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
        Some(Box::new(move |mut game: bevy_ecs::change_detection::Mut<'_, R>| {
            if game.consumption() != state {
                game.adopt_consumption(state);
            }
        }))
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
        Some(Box::new(move |mut game: bevy_ecs::change_detection::Mut<'_, TerrainEditingData>| {
            if game.rasterized != rasterized {
                game.0.rasterized = rasterized;
            }
        }))
    });
    // Terrain readback: requests are atomic increments through `&self`, so
    // subtract exactly what the frame consumed instead of overwriting.
    let consumed = std::sync::Arc::new(parking_lot::Mutex::new(None));
    let at_extract = consumed.clone();
    set.extract.add(move |game, render, _| {
        let session = crate::gpu_readback::TerrainReadbackSession::capture(game);
        render.insert_resource(session);
        *at_extract.lock() = game.get_resource_change_ticks::<ScheduledTerrainReadback>().map(|ticks| {
            let pending = render.get_resource::<ScheduledTerrainReadback>()
                .map_or(0, |readback| readback.pending.load(std::sync::atomic::Ordering::Acquire));
            (game.id(), ticks.added, pending, session)
        });
    });
    set.write_back.add(move |render, at, out| {
        let Some(readback) = render.get_resource::<ScheduledTerrainReadback>() else {
            return;
        };
        let Some((source, added, before, session)) = *consumed.lock() else { return; };
        if !render.get_resource_change_ticks::<ScheduledTerrainReadback>().is_some_and(|ticks|
            ticks.changed.is_newer_than(at.render, render.read_change_tick())) { return; }
        let taken = before.saturating_sub(readback.pending.load(std::sync::atomic::Ordering::Acquire));
        let (issued, tick) = (readback.copy_issued, readback.changed_tick);
        if taken == 0 && !issued {
            return;
        }
        out.push(Box::new(move |game: &mut bevy_ecs::world::World| {
            if game.id() != source { return; }
            if crate::gpu_readback::TerrainReadbackSession::capture(game) != session { return; }
            let Some(ticks) = game.get_resource_change_ticks::<ScheduledTerrainReadback>() else { return; };
            if ticks.added != added { return; }
            if issued { game.insert_resource(country_core::render::gpu_readback::TerrainCopyEpoch(session)); }
            if let Some(mut readback) = game.get_resource_mut::<ScheduledTerrainReadback>() {
                // Requests can arrive through shared references during render.
                // Subtract only this frame's consumption without losing increments.
                let _ = readback.pending.fetch_update(std::sync::atomic::Ordering::AcqRel,
                    std::sync::atomic::Ordering::Acquire, |pending| Some(pending.saturating_sub(taken)));
                // CPU延迟系统每帧取走copy_issued，也会改变changed tick。
                // 同一实例的新提交确认必须OR合并；会话/World/added共同隔离Reload。
                readback.copy_issued |= issued;
                readback.changed_tick = tick;
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

    fn pending_path() -> country_core::resources::world_raster_defs::PathRaster {
        use country_core::resources::{world_raster::DirtyTilePublisher, world_raster_defs::PathRaster};
        let mut raster = PathRaster::new(DirtyTilePublisher::unique());
        raster.dirty_mut().set_cell(1, 1);
        raster.mark_staging_uploaded();
        raster
    }

    #[test]
    fn consumption_feedback_marks_changed_only_for_actual_consumption() {
        use country_core::resources::{terrain_editing::TerrainEditingData, world_raster_defs::PathRaster};
        let mut game = World::new();
        let mut render = World::new();
        game.insert_resource(pending_path());
        game.insert_resource(TerrainEditingData(Default::default()));
        let mut exchange = production_exchange();
        exchange.extract.run(&mut game, &mut render);
        let before_path = game.get_resource_change_ticks::<PathRaster>().unwrap().changed;
        let before_terrain = game.get_resource_change_ticks::<TerrainEditingData>().unwrap().changed;
        for _ in 0..4 {
            game.increment_change_tick();
            let mut out = Vec::new();
            exchange.write_back.run_submitted(&mut render, &mut out);
            for apply in out { apply(&mut game); }
            assert_eq!(game.get_resource_change_ticks::<PathRaster>().unwrap().changed, before_path);
            assert_eq!(game.get_resource_change_ticks::<TerrainEditingData>().unwrap().changed, before_terrain);
        }
        render.resource_mut::<PathRaster>().consume_reupload();
        render.resource_mut::<TerrainEditingData>().0.rasterized = true;
        game.increment_change_tick();
        let mut out = Vec::new();
        exchange.write_back.run_submitted(&mut render, &mut out);
        for apply in out { apply(&mut game); }
        assert_ne!(game.get_resource_change_ticks::<PathRaster>().unwrap().changed, before_path);
        assert_ne!(game.get_resource_change_ticks::<TerrainEditingData>().unwrap().changed, before_terrain);
        assert!(!game.resource::<PathRaster>().consumption().1);
        assert!(!game.resource::<PathRaster>().dirty().contains(1, 1));
        assert!(game.resource::<TerrainEditingData>().rasterized);
    }

    #[test]
    fn raster_feedback_preserves_failed_submission_retry() {
        use country_core::resources::world_raster_defs::PathRaster;
        let mut game = World::new();
        let mut render = World::new();
        game.insert_resource(pending_path());
        let mut exchange = production_exchange();
        exchange.extract.run(&mut game, &mut render);
        render.resource_mut::<PathRaster>().consume_reupload();
        let mut out = Vec::new();
        exchange.write_back.run(&mut render, &mut out);
        assert!(out.is_empty());
        exchange.write_back.abort_unsubmitted(&mut render);
        assert!(game.resource::<PathRaster>().consumption().1);
        assert!(game.resource::<PathRaster>().dirty().contains(1, 1));
        exchange.extract.run(&mut game, &mut render);
        assert!(render.resource::<PathRaster>().consumption().1);
        assert!(render.resource::<PathRaster>().dirty().contains(1, 1));
        render.resource_mut::<PathRaster>().consume_reupload();
        exchange.write_back.run_submitted(&mut render, &mut out);
        for apply in out { apply(&mut game); }
        assert!(!game.resource::<PathRaster>().consumption().1);
        assert!(!game.resource::<PathRaster>().dirty().contains(1, 1));
    }

    #[test]
    fn raster_feedback_cannot_overwrite_late_edits_or_replacement_owners() {
        use country_core::resources::world_raster_defs::PathRaster;
        for replacement in 0..3 {
            let mut game = World::new();
            let mut render = World::new();
            game.insert_resource(pending_path());
            let mut exchange = production_exchange();
            exchange.extract.run(&mut game, &mut render);
            render.resource_mut::<PathRaster>().consume_reupload();
            let mut out = Vec::new();
            exchange.write_back.run_submitted(&mut render, &mut out);
            game.increment_change_tick();
            match replacement {
                0 => game.resource_mut::<PathRaster>().dirty_mut().set_cell(2, 2),
                1 => {
                    game.remove_resource::<PathRaster>();
                    game.insert_resource(pending_path());
                }
                _ => { game = World::new(); game.insert_resource(pending_path()); }
            }
            for apply in out { apply(&mut game); }
            assert!(game.resource::<PathRaster>().consumption().1);
            assert!(game.resource::<PathRaster>().dirty().contains(1, 1));
            if replacement == 0 { assert!(game.resource::<PathRaster>().dirty().contains(2, 2)); }
        }
    }

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
    fn terrain_copy_ack_survives_the_live_cpu_delay_system_running_before_feedback() {
        use country_core::{
            asset_libraries::ssbo_library::AssetSsboLibrary,
            render::gpu_readback::GpuReadbacks,
            render::ssbo::{MappedStorageBuffer, rhapsody_boundary::Buffer},
            resources::terrain_height::{TerrainHeightSsbo, TerrainHeightsData, VisualTerrainHeightSsbo},
            systems::terrain::delayed_terrain_changed_msg::{delayed_terrain_changed_msg, OnTerrainChange},
        };
        use bevy_ecs::prelude::*;

        let mut game = World::new();
        let mut render = World::new();
        game.init_resource::<ScheduledTerrainReadback>();
        game.insert_resource(AssetSsboLibrary::new());
        // 回归用真实类型化注册表和完成副本；空mapping刻意禁止从GPU在途页读取。
        game.resource::<AssetSsboLibrary>().register::<TerrainHeightSsbo>(MappedStorageBuffer::from_mapping(
            &mut [], Buffer { id: 81, generation: 1, byte_size: 0x80000, ..Default::default() },
        ));
        game.resource::<AssetSsboLibrary>().register::<VisualTerrainHeightSsbo>(MappedStorageBuffer::from_mapping(
            &mut [], Buffer { id: 82, generation: 1, byte_size: 0x80000, ..Default::default() },
        ));
        game.insert_resource(TerrainHeightsData::<TerrainHeightSsbo>::new());
        game.insert_resource(TerrainHeightsData::<VisualTerrainHeightSsbo>::new());
        game.init_resource::<Events<OnTerrainChange>>();
        game.init_resource::<GpuReadbacks>();
        game.resource::<ScheduledTerrainReadback>().on_terrain_modified();
        let mut exchange = production_exchange();
        exchange.extract.run(&mut game, &mut render);
        render.resource_mut::<ScheduledTerrainReadback>().on_gpu_copy_issued(29);
        let mut out = Vec::new();
        exchange.write_back.run_submitted(&mut render, &mut out);

        // 真实游戏线程在渲染反馈到达前继续执行。该系统每帧取走copy_issued，
        // 会改变资源tick；这不应把同一资源实例尚未到达的提交确认作废。
        game.increment_change_tick();
        let mut cpu = Schedule::default();
        cpu.add_systems(delayed_terrain_changed_msg);
        cpu.run(&mut game);
        game.resource::<ScheduledTerrainReadback>().on_terrain_modified();
        for apply in out { apply(&mut game); }
        let readback = game.resource::<ScheduledTerrainReadback>();
        assert_eq!(readback.pending.load(Ordering::Acquire), 1);
        assert!(readback.copy_issued, "running the CPU delay system lost the submitted GPU copy acknowledgment");
        assert_eq!(readback.changed_tick, 29);
        {
            let mut readbacks = game.resource_mut::<GpuReadbacks>();
            readbacks.note_submitted(31);
            readbacks.publish_slice(&vec![TerrainHeightSsbo(0x000a_0009); 0x20000], 31);
            readbacks.publish_slice(&vec![VisualTerrainHeightSsbo(0x000e_000d); 0x20000], 31);
        }
        for _ in 0..2 {
            cpu.run(&mut game);
            assert!(game.resource::<Events<OnTerrainChange>>().is_empty());
        }
        cpu.run(&mut game);
        assert_eq!(game.resource::<Events<OnTerrainChange>>().len(), 1);
        assert_eq!(&game.resource::<TerrainHeightsData<TerrainHeightSsbo>>().heights()[..2], &[9, 10]);
        assert_eq!(&game.resource::<TerrainHeightsData<VisualTerrainHeightSsbo>>().heights()[..2], &[13, 14]);
    }

    #[test]
    fn terrain_feedback_rejects_previous_session_without_replacing_its_request_owner() {
        use country_core::resources::session::{ActiveSession, SessionId, SessionMeta};
        use country_core::systems::render_loop::compute_passes::read_terrain_data::ScheduledTerrainReadback;
        use std::sync::atomic::Ordering;
        let mut game = World::new();
        let mut render = World::new();
        game.insert_resource(ActiveSession::new(SessionId::new(), SessionMeta::default()));
        game.init_resource::<ScheduledTerrainReadback>();
        let added = game.get_resource_change_ticks::<ScheduledTerrainReadback>().unwrap().added;
        game.resource::<ScheduledTerrainReadback>().on_terrain_modified();
        let mut exchange = production_exchange();
        exchange.extract.run(&mut game, &mut render);
        render.resource_mut::<ScheduledTerrainReadback>().on_gpu_copy_issued(19);
        let mut old = Vec::new();
        exchange.write_back.run_submitted(&mut render, &mut old);
        game.insert_resource(ActiveSession::new(SessionId::new(), SessionMeta::default()));
        game.resource::<ScheduledTerrainReadback>().on_terrain_modified();
        assert_eq!(game.get_resource_change_ticks::<ScheduledTerrainReadback>().unwrap().added, added);
        for apply in old { apply(&mut game); }
        assert_eq!(game.resource::<ScheduledTerrainReadback>().pending.load(Ordering::Acquire), 2);
        assert!(!game.resource::<ScheduledTerrainReadback>().copy_issued);
        // 新会话的有效确认必须继续消费，而不是一概拒绝反馈。
        exchange.extract.run(&mut game, &mut render);
        assert_eq!(crate::gpu_readback::terrain_session(&render).session, Some(game.resource::<ActiveSession>().id));
        render.resource_mut::<ScheduledTerrainReadback>().on_gpu_copy_issued(20);
        let mut current = Vec::new();
        exchange.write_back.run_submitted(&mut render, &mut current);
        for apply in current { apply(&mut game); }
        assert_eq!(game.resource::<ScheduledTerrainReadback>().pending.load(Ordering::Acquire), 0);
        assert!(game.resource::<ScheduledTerrainReadback>().copy_issued);
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
