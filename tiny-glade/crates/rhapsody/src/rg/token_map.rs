//! A hash map for `u64` resource tokens.
//!
//! Tokens come from one process-wide counter, so they are already unique and
//! well spread; a multiplicative mix is all the hashing they need. The default
//! SipHash and the tree it replaces were both measurable per-frame costs
//! (hundreds of lookups per frame in recording and binding).
use std::{
    collections::HashMap,
    hash::{BuildHasherDefault, Hasher},
};

#[derive(Default)]
pub struct TokenHasher(u64);
impl Hasher for TokenHasher {
    fn write(&mut self, bytes: &[u8]) {
        // Only u64 keys are used; keep the fallback correct, not fast.
        for &byte in bytes {
            self.0 = (self.0 ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    fn write_u64(&mut self, value: u64) {
        self.0 = value.wrapping_mul(0x9e37_79b9_7f4a_7c15);
    }
    fn finish(&self) -> u64 {
        // hashbrown takes the top bits for its control bytes and the low bits
        // for the bucket; fold so both see the mixed value.
        self.0 ^ (self.0 >> 32)
    }
}
pub type TokenMap<V> = HashMap<u64, V, BuildHasherDefault<TokenHasher>>;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn behaves_like_a_map_for_sequential_and_sparse_tokens() {
        let mut map: TokenMap<u64> = TokenMap::default();
        for token in (1..5000u64).chain([u64::MAX, 1 << 40, 1 << 63]) {
            assert!(map.insert(token, token.wrapping_mul(3)).is_none());
        }
        assert_eq!(map.len(), 5002);
        for token in (1..5000u64).chain([u64::MAX, 1 << 40, 1 << 63]) {
            assert_eq!(map.get(&token), Some(&token.wrapping_mul(3)));
        }
        assert_eq!(map.remove(&77), Some(231));
        assert!(map.get(&77).is_none());
    }
}
