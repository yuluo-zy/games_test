//! `foliage_exclusion_mask.rs` — W7 rasterize foliage exclusion mask (0x1409cb7b0).
//!
//! `RasterPass::new` + `DrawPacket::new` + `set_cull_mode` + `bind_terrain_heightmap`
//! (O). Foliage-exclusion family.

pub fn rasterize_foliage_exclusion_mask(world: &mut crate::typed_gap::Gap, _foliage: &crate::typed_gap::Gap) {
    let _ = world;
    unimplemented!()
}
