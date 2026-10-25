//! `logo_overlay.rs` — logo overlay (session start).
//!
//! `render_logo_overlay`: `RasterPass::new` + `DrawPacket::new` + read_texture +
//! `set_depth_test_enable` + `set_blend_mode` (O). `LogoOverlayPlugin` + `build`/`startup`
//! (load shader into library).

use crate::typed_gap::Gap;

pub struct LogoOverlayPlugin(pub Gap);

pub fn render_logo_overlay(world: &mut Gap, _logo: &Gap) {
    let _ = world;
    unimplemented!()
}
