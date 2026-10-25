//! `apply_screen_shadows.rs` — W42 apply_screen_shadows, W47 compute_shadow_mask_mean.
//!
//! URIs (O): `/apply_screen_shadows.cs.hlsl`, `/shadow_mask_mean.cs.hlsl`.
//! W42 (0x140a39e70): uniform + `bind_shadowmap` + read_texture + write_texture +
//! dispatch_2d; writes the shadow-mask histogram. W47 (0x140a3a6f0): dispatch_1d;
//! `ShadowMaskHistogramSsbo` → `ShadowMaskMeanSsbo` (consumed by final_post).

use crate::typed_gap::Gap;

/// Screen-shadow histogram (O name; layout U).
pub struct ShadowMaskHistogramSsbo(pub Gap);
/// Screen-shadow mean (O name; layout U).
pub struct ShadowMaskMeanSsbo(pub Gap);

pub fn apply_screen_shadows(world: &mut Gap, _frame: &crate::consume::core::FrameConstants) {
    let _ = world;
    unimplemented!()
}

pub fn compute_shadow_mask_mean(world: &mut Gap) {
    let _ = world;
    unimplemented!()
}
