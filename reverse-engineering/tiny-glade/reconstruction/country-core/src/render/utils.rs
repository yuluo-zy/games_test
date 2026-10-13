//! `render/utils.rs` — `calculate_hash` (referenced by wp1 `shader_uri` /
//! `draw_stream` as `crate::render::utils::calculate_hash`).

/// `utils::calculate_hash<T: Hash>` — hash a value to a u64.
/// Exact algorithm U (typed gap); std DefaultHasher is a behavior-equivalent
/// stand-in for the skeleton.
pub fn calculate_hash<T: core::hash::Hash>(t: &T) -> u64 {
    use core::hash::{BuildHasher, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    t.hash(&mut h);
    h.finish()
}
