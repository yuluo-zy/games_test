//! `startup/init_color_library.rs` — color-library SSBO + config watch.
//!
//! Source: B skeleton `crates/country-core/src/startup/init_color_library.rs`.
//! O evidence (Ghidra, B patched EXE; `init_color_library` @0x14082c000):
//! - builds the default `ColorLibrarySsbo`, registers it in
//!   `AssetSsboLibrary::add_new_with_data<ColorLibrarySsbo>`,
//!   `Commands::insert_resource<ColorLibrarySsbo>`,
//! - watches `assets/colors.json` via `ConfigWatch::watch`.
//! - `reload_color_library` (O system: `ActiveSession + GraphicsDeviceResource +
//!   ColorLibrarySsbo + AssetSsboLibrary`).

#![allow(dead_code, unused_imports)]

use crate::Gap;

pub fn init_color_library(_session: &Gap, _gpu: &Gap, _color_library: &mut Gap, _assets: &Gap) {
    todo!("recover init_color_library (ColorLibrarySsbo + assets/colors.json watch)")
}

pub fn reload_color_library(
    _session: &Gap,
    _gpu: &Gap,
    _color_library: &mut Gap,
    _assets: &Gap,
) {
    todo!("recover reload_color_library")
}
