//! `prefix_scan.rs` — inclusive prefix scan over u32 (1M limit), reusable primitive.
//!
//! `div_ceil` + `Ord::max` + `next_multiple_of` + dispatch_1d +
//! `RenderGraphRecorder::create` (O). Reused by grass (O, grass.rs bundle).

pub fn inclusive_prefix_scan_u32_1m(world: &mut crate::typed_gap::Gap, _src: &crate::typed_gap::Gap, _count: u32) {
    let _ = world;
    unimplemented!()
}
