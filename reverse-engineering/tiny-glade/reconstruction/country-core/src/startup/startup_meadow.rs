//! `startup/startup_meadow.rs` — meadow foliage startup (`init_meadow`).
//!
//! Source: B skeleton `crates/country-core/src/startup/startup_meadow.rs`.
//! O evidence (Ghidra, B patched EXE; `init_meadow` @0x1408639f0):
//! 1. `load_shader_into_library`;
//! 2. reads a `Vec<PlacementConfigEntry>` (serde_json) from a config path;
//! 3. per entry (stride 9 qwords = 72 B): joins the mesh path, loads placement
//!    data (`AssetPlacementDataLibrary::load<PathBuf>`), `load_json_as_mesh`,
//!    `load_mesh_into_library`, `AssetShaderLibrary::get_raster_handle_by_name`,
//!    and spawns `(DrawableMeshBundle, InstancedDrawFromPlacementData)`.

#![allow(dead_code, unused_imports)]

use serde::Deserialize;

use crate::Gap;

/// `PlacementConfigEntry` (O; 72 B meadow config row; fields U — typed gap).
#[derive(Clone, Debug, Default, Deserialize)]
pub struct PlacementConfigEntry {
    pub _gap: Gap,
}

/// `init_meadow` @0x1408639f0 (O).
pub fn init_meadow(_gpu: &Gap, _assets: &Gap, _commands: &mut Gap) {
    todo!("recover init_meadow (PlacementConfigEntry rows -> InstancedDrawFromPlacementData)")
}
