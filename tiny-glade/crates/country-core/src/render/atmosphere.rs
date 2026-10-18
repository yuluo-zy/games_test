//! `SunSkyColors` — sun/sky color palette (WP4-owned).
//!
//! Skeleton bundle: `skeleton/B/crates/country-core/src/render/atmosphere.rs`
//! (source_sha256 `8ba1e09e…`, 3 procs). Decompile `A_0x140a97650.c`.
//!
//! `SunSkyColors` is 48 bytes = 3 × Vec4 (O: `lerp` reads/writes [0..11] f32).
//! Named Vec4 slots (I): sun tint day/dusk, sky base/high/low/horizon/clouds —
//! exact mapping is a typed gap.

use bevy_ecs::resource::Resource;
use glam::{Vec3, Vec4};
use serde::{Deserialize, Serialize};

/// Sun + sky color palette (48 bytes / 3 × Vec4, O).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(from = "SunSkyColorsData", into = "SunSkyColorsData")]
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
        SunSkyColors {
            a: other.a * t + self.a * w,
            b: other.b * t + self.b * w,
            c: other.c * t + self.c * w,
        }
    }
}

// DTO restores the original four Vec3 JSON fields while retaining the current
// twelve-float arithmetic container; this is a serialization projection only.
#[derive(Serialize, Deserialize)]
struct SunSkyColorsData {
    sun_tint_color: Vec3,
    moon_tint_color: Vec3,
    sky_low_color: Vec3,
    sky_high_color: Vec3,
}
impl From<SunSkyColorsData> for SunSkyColors {
    fn from(v: SunSkyColorsData) -> Self {
        Self {
            a: Vec4::new(
                v.sun_tint_color.x,
                v.sun_tint_color.y,
                v.sun_tint_color.z,
                v.moon_tint_color.x,
            ),
            b: Vec4::new(
                v.moon_tint_color.y,
                v.moon_tint_color.z,
                v.sky_low_color.x,
                v.sky_low_color.y,
            ),
            c: Vec4::new(
                v.sky_low_color.z,
                v.sky_high_color.x,
                v.sky_high_color.y,
                v.sky_high_color.z,
            ),
        }
    }
}
impl From<SunSkyColors> for SunSkyColorsData {
    fn from(v: SunSkyColors) -> Self {
        Self {
            sun_tint_color: v.a.truncate(),
            moon_tint_color: Vec3::new(v.a.w, v.b.x, v.b.y),
            sky_low_color: Vec3::new(v.b.z, v.b.w, v.c.x),
            sky_high_color: Vec3::new(v.c.y, v.c.z, v.c.w),
        }
    }
}

#[repr(C)]
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct DerivedAtmosphere {
    pub sun_tint: Vec3,          // +00
    pub fake_light_color: Vec3,  // +0c
    pub fake_light_dir: Vec3,    // +18
    pub fake_light2_color: Vec3, // +24
    pub fake_light2_dir: Vec3,   // +30
    pub sky_low_color: Vec3,     // +3c
    pub sky_high_color: Vec3,    // +48
    pub fog_thickness: f32,      // +54
    pub fog_tint: Vec3,          // +58
    pub dir_to_sun: Vec3,        // +64
    pub dir_to_moon: Vec3,       // +70
}

// Default is original startup only: insert_resources @140940f83..fb9 zeroes
// 0x7c bytes. Runtime consumer must run the original TOD producer first.
const _: () = assert!(core::mem::size_of::<DerivedAtmosphere>() == 0x7c);
const _: () = assert!(core::mem::offset_of!(DerivedAtmosphere, fog_thickness) == 0x54);
const _: () = assert!(core::mem::offset_of!(DerivedAtmosphere, dir_to_sun) == 0x64);
const _: () = assert!(core::mem::offset_of!(DerivedAtmosphere, dir_to_moon) == 0x70);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lerp_endpoints_are_exact() {
        let a = SunSkyColors {
            a: Vec4::new(0.1, 0.2, 0.3, 0.4),
            b: Vec4::splat(5.0),
            c: Vec4::splat(-2.0),
        };
        let b = SunSkyColors {
            a: Vec4::splat(1.0),
            b: Vec4::splat(9.0),
            c: Vec4::splat(0.7),
        };
        assert_eq!(a.lerp(&b, 0.0), a);
        assert_eq!(a.lerp(&b, 1.0), b);
        assert_eq!(a.lerp(&b, 0.5).b, Vec4::splat(7.0));
    }
}
