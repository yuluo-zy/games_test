//! Compact float packing helpers shared across the workspace.
//! This is the canonical `pack_2x16f_uint` (original B `0x140c90df0`): two
//! `f32`s rounded to `f16` and packed into a single `u32` (`lo | hi << 16`).
/// Pack two `f32`s into `u32` as two `f16` halves (low half first).
pub fn pack_2x16f_uint(a: f32, b: f32) -> u32 {
    let lo = half::f16::from_f32(a).to_bits() as u32;
    let hi = half::f16::from_f32(b).to_bits() as u32;
    (hi << 16) | lo
}
