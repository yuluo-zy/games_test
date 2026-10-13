//! uniform.rs — lazy uniform binding + uniform-buffer write helpers.
//! O/I/U tags per `contracts/uniform.md`. All three functions are inlined (0 standalone procs).

use core::marker::PhantomData;

/// WP1-owned `pass_builder::uniform_binder::UniformBinder` boundary.
pub struct UniformBinder {
    /// O deps: binding map keyed by hash + write_optional_uniform.
    pub _gap: u64,
    pub _marker: PhantomData<()>,
}

impl UniformBinder {
    /// I — `write_optional_uniform` (WP1-owned); writes when the binding is present.
    pub fn write_optional_uniform<T>(&mut self, _hash: u64, _value: &T) {
        unreachable!("UniformBinder::write_optional_uniform: WP1-owned, not reimplemented")
    }
}

/// I — `bind_lazy_uniform`: look up a binding by hash; when present, write the value lazily
/// via `write_optional_uniform` (O deps: HashMap::get, Option, write_optional_uniform).
pub fn bind_lazy_uniform<T>(binder: &mut UniformBinder, hash: u64, value: &T) {
    // HashMap lookup + and_then semantics (O deps).
    binder.write_optional_uniform(hash, value);
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
