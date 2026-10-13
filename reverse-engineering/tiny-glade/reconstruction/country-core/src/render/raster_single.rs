//! raster_single.rs — `raster_single_object` (single-color-target raster pass helper).
//! O/I/U tags per `contracts/raster_single.md`.
//!
//! 接入说明（assembly）：WP1-boundary `pass_builder_boundary` stub removed and replaced by
//! the real wp1 types (`draw_stream::RasterPass`, `draw_builder::DrawBuilder`,
//! `framebuffer_binding::ColorAttachment`). The real `RasterPass::new` needs
//! (`&[ColorAttachment]`, `&GraphicsDeviceResource`, `&World`) and `DrawBuilder::new` needs
//! (`&World`, `Handle<RasterShaderProgram>`, `u8`) — the wp2-stub call shapes
//! (`new_impl(extent)`, zero-arg `new()`, `bind_texture_to_draw_builder`) differ, so the
//! helper body is an integration gap (todo marker; O entry addresses preserved).

use crate::render::pass_builder::draw_builder::DrawBuilder;
use crate::render::pass_builder::draw_stream::RasterPass;
use crate::render::pass_builder::framebuffer_binding::ColorAttachment;

/// O — `raster_single_object<1, [ColorAttachment; 1], CLOSURE>` 0x1409cdec0 / 0x1409ce780 / 0x1409d00a0.
/// Builds a single-color-attachment raster pass over `extent`, binds the pass texture,
/// issues one `draw_no_vao`, and renders. The closure carries the pass-specific parameters.
/// The three monomorphizations (mei_pass / final_post / resolve_to_swapchain) share this shape.
///
/// INTEGRATION GAP: the wp2 skeleton calls the stub's `new_impl(extent)` /
/// `DrawBuilder::new()` / `bind_texture_to_draw_builder` shapes, which the real wp1 API does
/// not expose (device + world params, builder-mode capture). Body is a todo marker; the real
/// `RasterPass::new` / `draw_no_vao` / `render` path is the documented O shape.
pub fn raster_single_object<F>(
    _extent: (u32, u32),
    _color_attachment: ColorAttachment,
    _closure: F,
) where
    F: FnOnce(&mut DrawBuilder),
{
    todo!("raster_single_object — real wp1 RasterPass/DrawBuilder API differs (integration gap)")
}
