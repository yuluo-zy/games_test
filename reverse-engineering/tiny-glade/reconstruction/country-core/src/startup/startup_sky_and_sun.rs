//! `startup/startup_sky_and_sun.rs` — sky+sun dome startup.
//!
//! Source: B skeleton `crates/country-core/src/startup/startup_sky_and_sun.rs`.
//! O evidence (Ghidra, B patched EXE; @0x140864430): `load_json_as_mesh` +
//! `load_mesh_into_library` + `load_shader_into_library` + spawn a
//! `DrawableMeshBundle` sky dome entity.

#![allow(dead_code, unused_imports)]

use crate::Gap;

pub fn startup_sky_and_sun(_gpu: &Gap, _assets: &Gap, _commands: &mut Gap) {
    todo!("recover startup_sky_and_sun (sky dome DrawableMeshBundle)")
}
