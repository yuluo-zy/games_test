//! `country_core::systems::render_loop` — frame-pass composition.
//!
//! Skeleton root: `crates/country-core/src/systems/render_loop/`. Bundles:
//! `mod.rs` (RenderContext + buffer-clear helpers), `compute_passes/`,
//! `draw_passes/`. Frame wiring (W1–W54 + branches) is consumed from
//! `contracts/frame-wiring.md` / `evidence/render-passes/cpu-frame/sequence.md`.
//!
//! Helper functions (`mod.rs`, addresses resolved from public-symbols.json):
//! - `clear_buffer`          — clear a CPU-mapped buffer (ssbo_to_rg_handle)
//! - `clear_rg_buffer`       — clear an RG buffer via PassBuilder::render (O,
//!                             0x1409ddbb0 is `render_loop::clear_rg_buffer`)
//! - `clear_rg_color_image`  — clear an RG color image via PassBuilder::render
//! - `rg_buffer_copy_from_slice<u32>` — write a slice into an RG buffer through
//!                             DynamicConstants::push_slice + `impl$2::write`
//! - `finish`                — GpuOwned::mark_unused / drop

pub mod compute_passes;
pub mod draw_passes;

use crate::typed_gap::Gap;

/// Per-frame render context handed to the render_loop systems (O name; layout U).
pub struct RenderContext(pub Gap);

/// Clear a CPU-side storage buffer (O: UnsafeWorldCell::get_resource +
/// `pass_resource_access::ssbo_to_rg_handle`).
pub fn clear_buffer(_world: &Gap, _handle: &Gap) {
    // O: read the buffer's RG handle, then PassBuilder render with a clear closure.
    unimplemented!()
}

/// Clear an RG buffer through `RenderGraphRecorder::add_pass` +
/// `impl$2::write` + `PassBuilder::render` (O, module 521).
pub fn clear_rg_buffer(_recorder: &mut Gap, _buffer: &Gap, _clear_value: u32) {
    unimplemented!()
}

/// Clear an RG color image (same shape as clear_rg_buffer).
pub fn clear_rg_color_image(_recorder: &mut Gap, _image: &Gap, _clear: Gap) {
    unimplemented!()
}

/// Copy `data` into an RG buffer (O: bytemuck::cast_slice + DynamicConstants::push_slice).
pub fn rg_buffer_copy_from_slice<T: Copy + Sized>(_recorder: &mut Gap, _buffer: &Gap, _data: &[T]) {
    unimplemented!()
}

/// Finish / drop helpers (O: GpuOwned::mark_unused + drop_in_place).
pub fn finish<T>(_value: T) {
    unimplemented!()
}
