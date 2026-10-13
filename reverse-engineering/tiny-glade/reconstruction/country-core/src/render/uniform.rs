//! uniform.rs — lazy uniform binding + uniform-buffer write helpers.
//! O/I/U tags per `contracts/uniform.md`. All three functions are inlined (0 standalone procs).
//!
//! 接入说明（assembly）：WP1-boundary `UniformBinder` stub replaced by the real
//! `render::pass_builder::uniform_binder::UniformBinder`. The real
//! `write_optional_uniform<T: UniformBytes>(BindingIndex, Option<T>)` signature differs from
//! the wp2-stub call shape (`(u64 hash, &T)`), so `bind_lazy_uniform` is an integration gap
//! (now implemented against the recovered signature).

pub use crate::render::pass_builder::uniform_binder::{
    UniformBinder, UniformBytes, UniformReflection,
};

/// I — `bind_lazy_uniform`：按名字哈希查绑定，命中才惰性求值并写入
/// （O 依赖：HashMap::get、Option、`write_optional_uniform`）。返回是否写入。
/// 签名跟随已恢复的 `UniformBinder::write_optional_uniform`（反射表 + u64 键 + 惰性闭包）。
pub fn bind_lazy_uniform<T: UniformBytes>(
    binder: &mut UniformBinder,
    reflection: &UniformReflection,
    hash: u64,
    value: impl FnOnce() -> T,
) -> bool {
    binder.write_optional_uniform(reflection, hash, value)
}

/// I — `impl$23::write_uniform_to_buffer`: copy a value into a mapped uniform buffer at `offset`.
pub fn write_uniform_to_buffer<T>(buffer: &mut [u8], offset: usize, value: &T) {
    let es = core::mem::size_of::<T>();
    assert!(
        offset + es <= buffer.len(),
        "write_uniform_to_buffer: out of range"
    );
    unsafe {
        core::ptr::copy_nonoverlapping(
            value as *const T as *const u8,
            buffer.as_mut_ptr().add(offset),
            es,
        );
    }
}

/// I — `impl$24::write_uniform_to_buffer`: specialization forwarding to impl$23.
pub fn write_uniform_to_buffer_aligned<T>(buffer: &mut [u8], offset: usize, value: &T) {
    write_uniform_to_buffer(buffer, offset, value);
}
