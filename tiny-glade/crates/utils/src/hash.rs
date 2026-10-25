//! Generic value → u64 hash via std zero-key SipHash-1-3 (`DefaultHasher`).
//!
//! This is the canonical `calculate_hash` used across the workspace for
//! process-internal keys and identities.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Hash `value` to a `u64` with a zero-key SipHash-1-3 (std `DefaultHasher`).
///
/// `T: ?Sized` mirrors the original `&T` hashing call sites: `&str`, `&[u8]`
/// and slices hash identically to their unsized targets.
pub fn calculate_hash<T: Hash + ?Sized>(value: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}
