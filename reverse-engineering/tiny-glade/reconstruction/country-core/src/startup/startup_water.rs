//! `startup/startup_water.rs` — water/surface plane startup.
//!
//! Source: B skeleton `crates/country-core/src/startup/startup_water.rs`.
//!
//! O evidence (Ghidra, B patched EXE; `startup_water` @0x1408f1a20):
//! Non-demo build spawns a sequence of water/surface `DrawableMeshBundle`
//! entities:
//!   - water surface `(DrawableMeshBundle, DontCastShadow,
//!     DesignatedRenderPass)` pass selector **0xf**,
//!   - a second surface pass selector **0xe**,
//!   - when `ActiveSession` glade-type byte @+0x58 == 4 (winter) an extra
//!     `(DrawableMeshBundle, DontCastShadow)` ice surface,
//!   - then (winter-dependent water mesh) + more plane entities.
//! Each spawn is preceded by `load_json_as_mesh` + `load_mesh_into_library` +
//! `load_shader_into_library`; winter branch also uses
//! `load_shader_into_library_replace_previous`.

#![allow(dead_code, unused_imports)]

use crate::Gap;

/// `startup_water` @0x1408f1a20 (O).
pub fn startup_water(_session: &Gap, _gpu: &Gap, _assets: &Gap, _commands: &mut Gap) {
    todo!("recover startup_water surface spawns (pass selectors 0xf/0xe + winter ice plane)")
}
