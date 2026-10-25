//! `startup/glade_themes/olden.rs` — the Olden glade theme (preplaced stone
//! pillars/walls).
//!
//! Source: B skeleton `crates/country-core/src/startup/glade_themes/olden.rs`.
//!
//! O evidence (Ghidra, B patched EXE; `startup_olden` @0x1415539d0):
//! 1. iterates **6 placement configs** (name strings of lengths
//!    `{0x1e,0x11,0x24,0xf,0x22,0x22}`; loop stops at a `0x03` marker byte);
//!    each row spawns `system_wall_constructor::place_pillars::PillarPlacementProposal`
//!    (stride 0x1c = `{ Rectangle2d, pillar_kind: u8 }`) per placement entry
//!    (stride 0x30, yaw `a = row@+0x10`, quat `(0, sin(a/2), 0, cos(a/2))`).
//! 2. loads an olden wall/arch mesh → spawns
//!    `(DrawableMeshBundle, AlbedoMap, AlphaMap, NormalMap)` with three name
//!    hashes `{0x23ae4b5c6ce0d14d, 0x691e5a80737217f2, 0xfbf3a82f352c1401}`;
//! 3. loads a second mesh → `(DrawableMeshBundle, AlbedoMap)` with hash
//!    `0x2bb813728bcb5dd4`.

#![allow(dead_code, unused_imports)]

use crate::Gap;

/// O name-hash constants for the olden theme drawables.
pub const OLDEN_ALBEDO_ALPHA_NORMAL_HASHES: [u64; 3] =
    [0x23ae4b5c6ce0d14d, 0x691e5a80737217f2, 0xfbf3a82f352c1401];
pub const OLDEN_ALBEDO2_HASH: u64 = 0x2bb813728bcb5dd4;

/// `startup_olden` @0x1415539d0 (O).
pub fn startup_olden(_session: &Gap, _gpu: &Gap, _assets: &Gap, _commands: &mut Gap) {
    todo!("recover startup_olden (6 pillar configs + albedo/alpha/normal drawables)")
}
