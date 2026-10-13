//! `rasterize_buildings.rs` — W11 rasterize_buildings (0x140a32770).
//!
//! URIs (O): `/rasterize_buildings.cs.hlsl` + `/rasterize_buildings_animate.cs.hlsl`.
//! `HashMap::iter` + `AssetSsboLibrary::get` + `InstanceBuffer::gpu_write` +
//! uniform + write_texture + dispatch_2d; `RenderGraphRecorder::create` (O).
//! `GroundedBuildingsSsbo` (O name; layout U).

use crate::typed_gap::Gap;

pub struct GroundedBuildingsSsbo(pub Gap);

pub fn rasterize_buildings(world: &mut Gap, _buildings: &Gap) {
    let _ = world;
    unimplemented!()
}
