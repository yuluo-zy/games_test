//! Deterministic seed mixing for the game random-number generator.
pub fn init_rng_u64(seed: u64) -> u64 {
    seed.wrapping_mul(0x7598_39da_217f)
        .wrapping_add(0x0001_3c7c_a929_3765)
        ^ 0x4d59_5df4_d0f3_3173
}
pub fn init_rng(seed: u32) -> u64 {
    init_rng_u64(u64::from(seed))
}
/// Builds a generator from the mixed seed. Output sequences depend on the
/// pinned fastrand version, which must stay compatible with existing saves.
pub fn rng_from_seed_u64(seed: u64) -> fastrand::Rng {
    fastrand::Rng::with_seed(init_rng_u64(seed))
}

#[cfg(test)]
#[path = "oracle/rng_native_oracle.rs"]
mod native_oracle;
