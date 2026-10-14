//! `utils::name_hash::NameHash`（本地 `utils` crate，O 0x140c9cd80，46 B）。
//!
//! 证据（O）：`NameHash::compute(&str)` 是 `xxhash_rust::xxh3::xxh3_64`（默认 secret、seed 0，调用 `xxh3_64_internal`
//! 的 `xxh3_64_long_default` 回调），结果即 u64。`PartialEq` 是 `impl$2::eq`（0x140c9cd70）：比较 u64。
//! 原版依赖 `xxhash-rust 0.8.10`（`skeleton/B/Cargo.lock`），这里直接复用，不重写。

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct NameHash(pub u64);

/// O：原版 Debug 是 `debug_struct_field1_finish("NameHash", "hash", &u64)`，即 `NameHash { hash: N }`
/// （`AtlassedMeshName` 的 panic 文案里可见）。
impl core::fmt::Debug for NameHash {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("NameHash").field("hash", &self.0).finish()
    }
}

impl NameHash {
    pub fn compute(name: &str) -> Self {
        NameHash(xxhash_rust::xxh3::xxh3_64(name.as_bytes()))
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
