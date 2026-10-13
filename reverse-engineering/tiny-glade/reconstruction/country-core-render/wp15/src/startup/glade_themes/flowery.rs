//! `startup/glade_themes/flowery.rs` — the Flowery glade theme (preplaced
//! flowers).
//!
//! Source: B skeleton `crates/country-core/src/startup/glade_themes/flowery.rs`.
//!
//! O evidence (Ghidra, B patched EXE; `startup_flowery` @0x141553120):
//! 1. loads a flower-bed mesh + shader → spawns `(DrawableMeshBundle,
//!    DontCastShadow)`;
//! 2. loads flower placement data; per placement row (stride 0x30) spawns
//!    `NaniBundle<PreplacedFlowerInstanceData>`:
//!      - transform rotation = `Quat(x=0, y=sin(a/2), z=0, w=cos(a/2))` where
//!        `a = placement.yaw` (row @+0x10),
//!      - translation = placement row @+0x00 (Vec3),
//!      - subset kind **0xe**,
//!      - name hash `0x13fa53c58fdddee8`;
//! 3. loads a flower albedo mesh → `(DrawableMeshBundle, AlbedoMap)`.

#![allow(dead_code, unused_imports)]

use crate::Gap;

/// `startup_flowery` @0x141553120 (O).
pub fn startup_flowery(_session: &Gap, _gpu: &Gap, _assets: &Gap, _commands: &mut Gap) {
    todo!("recover startup_flowery (flower-bed mesh + preplaced flower NaniBundle kind 0xe + albedo mesh)")
}
