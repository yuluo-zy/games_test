//! `upload_world_raster.rs` — W1 upload_cpu_raster ×6 (staging→texture).
//!
//! URI `/upload_cpu_raster.cs.hlsl` (r16f / rgba16f entry points). Six
//! monomorphizations (O, PDB): `PathRaster`, `Cobblestone1Raster`,
//! `Cobblestone2Raster`, `Cobblestone3Raster`, `WaterRaster`, `GardenRaster`.
//! Each: `utils::get_res_mut` + `on_uploaded_to_texture` (fmt name) + uniform +
//! write_texture + dispatch_2d (O).

use crate::typed_gap::Gap;

pub struct PathRaster(pub Gap);
pub struct Cobblestone1Raster(pub Gap);
pub struct Cobblestone2Raster(pub Gap);
pub struct Cobblestone3Raster(pub Gap);
pub struct WaterRaster(pub Gap);
pub struct GardenRaster(pub Gap);

/// Generic upload: write the CPU raster into a texture (W1, slot n).
fn upload_world_raster<T>(world: &mut Gap, _slot: u32, _data: &T, _texture: &Gap) {
    let _ = world;
    unimplemented!()
}
