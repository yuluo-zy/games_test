//! `country_core::systems::render_loop::draw_passes` — frame raster passes.
//!
//! Mirrors the skeleton tree `draw_passes/<name>.rs`. Each pass composes
//! `RasterPass::new` + `CommonDrawParams::new` + `DrawListItem::to_draw_packet`
//! (see `contracts/draw-passes.md`).

pub mod common_draw_params;
pub mod custom_cursor_pass;
pub mod deferred_raster;
pub mod draw_lists;
pub mod draw_shaders;
pub mod feedback_report_bg;
pub mod final_post;
pub mod foliage_exclusion_mask;
pub mod logo_overlay;
pub mod main_pass;
pub mod material_id;
pub mod object_grass_push;
pub mod shadow_pass;
pub mod transparency_pass;
pub mod ui_2d_pass;
pub mod ui_pass;
pub mod wireframe_pass;
