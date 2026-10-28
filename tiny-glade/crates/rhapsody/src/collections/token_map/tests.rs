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
