//! `systems/ui/signifiers/animation.rs` — signifier animation system.
//!
//! Source: B skeleton `crates/country-core/src/systems/ui/signifiers/animation.rs`.
//! O evidence (Ghidra, B patched EXE):
//! - `signifier_animation` @0x140a69c30 (2 callers) — advances
//!   `SignifierAnimation::t` toward the target and animates the signifier
//!   entity (scale/alpha by easing; `WidgetVolumeShape::get_origin` for the
//!   volume-origin-parented ones).
//! - `signifier_animation_t` @0x140a69340 (2 callers) — the per-signifier
//!   animation-time tick.

#![allow(dead_code, unused_imports)]

use crate::Gap;

pub fn signifier_animation(_commands: &mut Gap) {
    todo!("recover signifier_animation")
}

pub fn signifier_animation_t(_animation: &mut Gap, _delta: f32) {
    todo!("recover signifier_animation_t")
}
