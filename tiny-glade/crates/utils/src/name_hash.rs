//! Stable name hashes computed from UTF-8 bytes with unseeded XXH3.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NameHash(pub u64);
impl NameHash {
    pub fn compute(name: &str) -> Self {
        Self(xxhash_rust::xxh3::xxh3_64(name.as_bytes()))
    }
    pub fn debug_name(&self) -> &'static str {
        "(NameHash)"
    }
}
