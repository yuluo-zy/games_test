//! `reset_indirect_args.rs` — reset indirect args buffers.
//!
//! `ssbo_to_rg_handle` + `RenderGraphRecorder::add_pass` + `Mutex::lock` +
//! `DynamicConstants::push_slice` + `impl$2::write` (`GpuOwned::track_unwrap`),
//! `Option::unwrap` (O). `/clearing/` family (runtime-synthesized URI, U).

pub fn reset_indirect_args(world: &mut crate::typed_gap::Gap, _args: &[crate::typed_gap::Gap]) {
    let _ = world;
    unimplemented!()
}
