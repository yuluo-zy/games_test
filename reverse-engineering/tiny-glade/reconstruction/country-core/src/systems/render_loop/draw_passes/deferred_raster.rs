//! `deferred_raster.rs` — W43 deferred light raster (0x1409d20b0).
//!
//! URIs (O): `/deferred/{solid, grass, plant, canopy}_light_sun_sky.raster.hlsl`.
//! `Duration::as_secs_f32` (time uniform) + `RasterPass::new` +
//! `AssetSsboLibrary::get` + `InstanceBuffer::instance_count` + `DrawPacket::new` +
//! `stencil_ref` + `set_blend_mode/depth_write/depth_test/stencil_test/stencil_compare`
//! (O). draw_no_vao + bind_color_library + bind_ray_tracing (O, sequence.md).
//! Reads `LanternSsbo` / `SpatialResolveOffsetsSsbo`, `ssgi_tex`/`screen_shadows_tex`/
//! `rdi_reservoir_tex`/`light_indirection_grid`/`grid_light_indices`.

use crate::consume::core::{LanternSsbo, SpatialResolveOffsetsSsbo};
use crate::typed_gap::Gap;

/// Deferred-raster resource bundle (O name; layout U).
pub struct DeferredRasterResources(pub Gap);

/// W43 — shade deferred G-buffer with sun/sky light (O).
pub fn deferred_raster(
    world: &mut Gap,
    _frame: &crate::consume::core::FrameConstants,
    _view: &Gap,
    _lanterns: &LanternSsbo,
    _spatial_offsets: &SpatialResolveOffsetsSsbo,
    _sublists: &Gap,
    _resources: &DeferredRasterResources,
) {
    let _ = world;
    unimplemented!()
}
