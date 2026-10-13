//! uniform.rs — lazy uniform binding + uniform-buffer write helpers.
//! O/I/U tags per `contracts/uniform.md`. All three functions are inlined (0 standalone procs).
//!
//! 接入说明（assembly）：WP1-boundary `UniformBinder` stub replaced by the real
//! `render::pass_builder::uniform_binder::UniformBinder`. The real
//! `write_optional_uniform<T: UniformBytes>(BindingIndex, Option<T>)` signature differs from
//! the wp2-stub call shape (`(u64 hash, &T)`), so `bind_lazy_uniform` is an integration gap
//! (its body is a todo marker; the O call shape is preserved in the doc).

pub use crate::render::pass_builder::uniform_binder::UniformBinder;

/// I — `bind_lazy_uniform`: look up a binding by hash; when present, write the value lazily
/// via `write_optional_uniform` (O deps: HashMap::get, Option, write_optional_uniform).
///
/// INTEGRATION GAP: the real `UniformBinder::write_optional_uniform` keys by
/// `shader_pipeline::reflection::BindingIndex` + `Option<T>`; the wp2-recovered call shape
/// (`u64 hash, &T value`) does not match, so the body is a todo marker (never fabricated).
pub fn bind_lazy_uniform<T>(
    _binder: &mut UniformBinder,
    _hash: u64,
    _value: &T,
) {
    todo!("bind_lazy_uniform — real UniformBinder keys by BindingIndex (integration gap)")
}

/// I — `impl$23::write_uniform_to_buffer`: copy a value into a mapped uniform buffer at `offset`.
pub fn write_uniform_to_buffer<T>(buffer: &mut [u8], offset: usize, value: &T) {
    let es = core::mem::size_of::<T>();
    assert!(offset + es <= buffer.len(), "write_uniform_to_buffer: out of range");
    unsafe {
        core::ptr::copy_nonoverlapping(value as *const T as *const u8, buffer.as_mut_ptr().add(offset), es);
    }
}

/// I — `impl$24::write_uniform_to_buffer`: specialization forwarding to impl$23.
pub fn write_uniform_to_buffer_aligned<T>(buffer: &mut [u8], offset: usize, value: &T) {
    write_uniform_to_buffer(buffer, offset, value);
}
