//! `SunSkyColors` — sun/sky color palette (WP4-owned).
//!
//! Skeleton bundle: `skeleton/B/crates/country-core/src/render/atmosphere.rs`
//! (source_sha256 `8ba1e09e…`, 3 procs). Decompile `A_0x140a97650.c`.
//!
//! `SunSkyColors` is 48 bytes = 3 × Vec4 (O: `lerp` reads/writes [0..11] f32).
//! Named Vec4 slots (I): sun tint day/dusk, sky base/high/low/horizon/clouds —
//! exact mapping is a typed gap.

use glam::Vec4;
use serde::{Deserialize, Serialize};

/// Sun + sky color palette (48 bytes / 3 × Vec4, O).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SunSkyColors {
    /// Vec4 slot 0 (O).
    pub a: Vec4,
    /// Vec4 slot 1 (O).
    pub b: Vec4,
    /// Vec4 slot 2 (O).
    pub c: Vec4,
}

impl SunSkyColors {
    /// `lerp(&self, other, t) -> SunSkyColors` (O: 0x140a97650, 87B):
    /// per-component `self * (1 - t) + other * t`.
    pub fn lerp(&self, other: &SunSkyColors, t: f32) -> SunSkyColors {
        SunSkyColors {
            a: self.a.lerp(other.a, t),
            b: self.b.lerp(other.b, t),
            c: self.c.lerp(other.c, t),
        }
    }
}
