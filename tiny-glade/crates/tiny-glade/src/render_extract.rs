//! The production hand-over (reports/render-thread-2026-10-22/ARCHITECTURE.md):
//! everything declaration reads, collected from the modules that read it.
use crate::render_thread::RenderExchange;
use bevy_ecs::change_detection::DetectChangesMut;

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
        game_config::GameConfig,
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
    // Whole values: OOM resolution scale, reveal/painterly progress, gallery
    // and thumbnail publication, the saved loading background.
    set.write_back::<GameConfig>();
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
    let consumed = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
    let at_extract = consumed.clone();
    set.extract.add(move |_, render, _| {
        let pending = render
            .get_resource::<ScheduledTerrainReadback>()
            .map_or(0, |readback| readback.pending.load(std::sync::atomic::Ordering::Acquire));
        at_extract.store(pending, std::sync::atomic::Ordering::Relaxed);
    });
    set.write_back.add(move |render, _, out| {
        let Some(readback) = render.get_resource::<ScheduledTerrainReadback>() else {
            return;
        };
        let before = consumed.load(std::sync::atomic::Ordering::Relaxed);
        let taken = before.saturating_sub(readback.pending.load(std::sync::atomic::Ordering::Acquire));
        let (issued, tick) = (readback.copy_issued, readback.changed_tick);
        if taken == 0 && !issued {
            return;
        }
        out.push(Box::new(move |game: &mut bevy_ecs::world::World| {
            if let Some(mut readback) = game.get_resource_mut::<ScheduledTerrainReadback>() {
                let readback = readback.bypass_change_detection();
                readback.pending.fetch_sub(taken.min(readback.pending.load(std::sync::atomic::Ordering::Acquire)), std::sync::atomic::Ordering::AcqRel);
                readback.copy_issued |= issued;
                readback.changed_tick = tick;
            }
        }));
    });
}
