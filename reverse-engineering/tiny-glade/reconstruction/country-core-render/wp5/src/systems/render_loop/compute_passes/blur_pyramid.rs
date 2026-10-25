//! `blur_pyramid.rs` — reusable blur-pyramid primitive (not a standalone frame pass).
//!
//! `blur_pyramid` (dispatch_2d, `overflowing_sub`/`max` mip walk), `rev_blur_pyramid`
//! (dispatch_3d + read_texture_view), `bidirectional_blur_pyramid` (combination).
//! Consumed by GI/nani/post families (exact reuse points U).

use crate::typed_gap::Gap;

pub fn blur_pyramid(world: &mut crate::typed_gap::Gap, _src: &Gap, _levels: u32) {
    let _ = world;
    unimplemented!()
}

pub fn rev_blur_pyramid(world: &mut crate::typed_gap::Gap, _src: &Gap, _levels: u32) {
    let _ = world;
    unimplemented!()
}

pub fn bidirectional_blur_pyramid(world: &mut crate::typed_gap::Gap, _src: &Gap, _levels: u32) {
    // O: rev_blur_pyramid + blur_pyramid.
    let _ = world;
    unimplemented!()
}
