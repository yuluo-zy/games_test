//! The production hand-over (reports/render-thread-2026-10-22/ARCHITECTURE.md):
//! everything declaration reads, collected from the modules that read it.
use crate::render_thread::ExtractSet;

pub fn production_extract() -> ExtractSet {
    use country_core::systems::render_loop::draw_passes as dp;
    let mut set = ExtractSet::default();
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
    set
}
