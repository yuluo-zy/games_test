//! `compute_path_masks.rs` — W10 compute_path_masks (0x1409014e0).
//!
//! URIs (O): `/paths/{combine_path_exclusions, directional_contract,
//! path_mask_visual}.cs.hlsl` (+ `/gauss_blur_mask.cs.hlsl` blur helpers, I).
//! `on_texture_processed` + `blur_exclusion_mask`/`blur_path_mask` + uniform +
//! read_texture + write_texture + dispatch_2d; `bind_terrain_heightmap` (O).

pub fn compute_path_masks(world: &mut crate::typed_gap::Gap, _paths: &crate::typed_gap::Gap) {
    let _ = world;
    unimplemented!()
}

fn blur_exclusion_mask(world: &mut crate::typed_gap::Gap, _mask: &crate::typed_gap::Gap) {
    let _ = world;
    unimplemented!()
}

fn blur_path_mask(world: &mut crate::typed_gap::Gap, _mask: &crate::typed_gap::Gap) {
    let _ = world;
    unimplemented!()
}
