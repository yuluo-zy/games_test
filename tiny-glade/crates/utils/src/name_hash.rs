//! Stable name hashes computed from UTF-8 bytes with unseeded XXH3.
//!
//! This is the single canonical `NameHash` for the workspace. `country_core`
//! re-exports it under `crate::name_hash` to keep existing call sites stable.

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct NameHash(pub u64);

/// 原版 Debug 是 `debug_struct_field1_finish("NameHash", "hash", &u64)`，即
/// `NameHash { hash: N }`。
impl core::fmt::Debug for NameHash {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("NameHash").field("hash", &self.0).finish()
    }
}

impl NameHash {
    pub fn compute(name: &str) -> Self {
        Self(xxhash_rust::xxh3::xxh3_64(name.as_bytes()))
    }
    pub fn debug_name(&self) -> &'static str {
        "(NameHash)"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_is_struct_style() {
        assert_eq!(format!("{:?}", NameHash(7)), "NameHash { hash: 7 }");
    }
}
