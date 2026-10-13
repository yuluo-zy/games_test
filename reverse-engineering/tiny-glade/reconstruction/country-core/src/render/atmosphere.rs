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
    /// `lerp(&self, other, t) -> SunSkyColors`（O 0x140a97650，87 B）：12 个 f32 逐分量
    /// `other * t + self * (1 - t)`（`mulps`×2 + `addps`；`1-t` 先算）。**不是** glam 的
    /// `a + (b-a)*t`，二者在舍入上不同（t=1 时后者可能得不到恰好 `other`）。
    pub fn lerp(&self, other: &SunSkyColors, t: f32) -> SunSkyColors {
        let w = 1.0 - t;
        SunSkyColors { a: other.a * t + self.a * w, b: other.b * t + self.b * w, c: other.c * t + self.c * w }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lerp_endpoints_are_exact() {
        let a = SunSkyColors { a: Vec4::new(0.1, 0.2, 0.3, 0.4), b: Vec4::splat(5.0), c: Vec4::splat(-2.0) };
        let b = SunSkyColors { a: Vec4::splat(1.0), b: Vec4::splat(9.0), c: Vec4::splat(0.7) };
        assert_eq!(a.lerp(&b, 0.0), a);
        assert_eq!(a.lerp(&b, 1.0), b);
        assert_eq!(a.lerp(&b, 0.5).b, Vec4::splat(7.0));
    }
}
