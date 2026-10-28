use super::*;

#[test]
fn diagnostics_distinguish_key_misses_capacity_and_explicit_invalidation() {
    let mut entries = BoundedCache::new(1, 100);
    entries.insert(1, 1, 8);
    entries.insert(2, 2, 8);
    assert_eq!(
        entries
            .stats()
            .diagnostics
            .invalidations(CacheReason::CapacityEntries),
        1
    );
    assert!(entries.get(&3).is_none());
    entries.classify_miss(CacheReason::Descriptor);
    assert_eq!(entries.stats().diagnostics.misses(CacheReason::Cold), 0);
    assert_eq!(
        entries.stats().diagnostics.misses(CacheReason::Descriptor),
        1
    );
    entries.clear();
    assert_eq!(
        entries
            .stats()
            .diagnostics
            .invalidations(CacheReason::ExplicitClear),
        1
    );
    entries.record_bypass(CacheReason::Disabled);
    assert_eq!(entries.stats().diagnostics.misses(CacheReason::Disabled), 1);
    let mut bytes = BoundedCache::new(8, 10);
    bytes.insert(1, 1, 6);
    bytes.insert(2, 2, 6);
    assert_eq!(
        bytes
            .stats()
            .diagnostics
            .invalidations(CacheReason::CapacityBytes),
        1
    );
    bytes.remove_with_reason(&2, CacheReason::DependencyVersion);
    assert_eq!(
        bytes
            .stats()
            .diagnostics
            .invalidations(CacheReason::DependencyVersion),
        1
    );
}

#[test]
fn eligible_eviction_skips_held_values_and_observes_lru_order() {
    let mut cache = BoundedCache::new(3, 3);
    cache.insert(1, (false, 10), 1);
    cache.insert(2, (true, 20), 1);
    cache.insert(3, (true, 30), 1);
    assert_eq!(cache.lru_key_where(|value| value.0), Some(2));
    assert_eq!(cache.get(&2), Some(&(true, 20)));
    assert_eq!(cache.lru_key_where(|value| value.0), Some(3));
    assert_eq!(
        cache.iter_lru().map(|(key, _)| *key).collect::<Vec<_>>(),
        vec![1, 3, 2]
    );
    assert_eq!(cache.lru_key_where(|_| false), None);
    assert_eq!(cache.len(), 3);
}

#[test]
fn insert_get_remove_round_trip() {
    let mut cache = BoundedCache::new(0, 0);
    assert!(matches!(cache.insert("a", 1u32, 8), Inserted::Stored(None)));
    assert_eq!(cache.get(&"a"), Some(&1));
    assert!(cache.contains(&"a"));
    assert_eq!(cache.len(), 1);
    assert_eq!(cache.bytes(), 8);
    assert_eq!(cache.remove(&"a"), Some(1));
    assert!(!cache.contains(&"a"));
}

#[test]
fn insert_replacement_returns_the_old_value() {
    let mut cache = BoundedCache::new(0, 0);
    cache.insert("k", 1u32, 4);
    assert!(matches!(cache.insert("k", 2, 4), Inserted::Stored(Some(1))));
    assert_eq!(cache.get(&"k"), Some(&2));
    assert_eq!(cache.len(), 1);
}

#[test]
fn entry_budget_evicts_least_recently_used() {
    let mut cache = BoundedCache::new(2, 0);
    cache.insert("a", 1u32, 1);
    cache.insert("b", 2, 1);
    cache.insert("c", 3, 1);
    assert!(!cache.contains(&"a"), "最久未使用的 a 被逐出");
    assert!(cache.contains(&"b"));
    assert!(cache.contains(&"c"));
    assert_eq!(cache.len(), 2);

    // 访问 b 把它提升为最近使用，再插入 d 时应逐出 c。
    assert_eq!(cache.get(&"b"), Some(&2));
    cache.insert("d", 4, 1);
    assert!(cache.contains(&"b"));
    assert!(!cache.contains(&"c"));
}

#[test]
fn byte_budget_evicts_until_within_limit() {
    let mut cache = BoundedCache::new(0, 100);
    cache.insert("a", 1u32, 40);
    cache.insert("b", 2, 40);
    cache.insert("c", 3, 40);
    // 40*3=120 > 100：逐出 a 后 80 <= 100。
    assert!(!cache.contains(&"a"));
    assert_eq!(cache.bytes(), 80);
    assert_eq!(cache.stats().evictions, 1);
}

#[test]
fn oversized_entry_is_rejected_without_caching() {
    let mut cache = BoundedCache::new(0, 100);
    assert!(matches!(
        cache.insert("big", 42u32, 200),
        Inserted::Rejected(42)
    ));
    assert!(!cache.contains(&"big"));
    assert_eq!(cache.stats().rejected_oversized, 1);
}

/// 哈希冲突也不得误命中：完整键比较才是命中的最终判定。
#[test]
fn full_key_equality_confirms_hits_even_on_hash_collision() {
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Colliding(u32);
    impl Hash for Colliding {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            // 所有键哈希到同一个桶，强制碰撞。
            state.write_u32(0);
        }
    }

    let mut cache = BoundedCache::new(0, 0);
    cache.insert(Colliding(1), "one", 1);
    cache.insert(Colliding(2), "two", 1);
    assert_eq!(cache.get(&Colliding(1)), Some(&"one"));
    assert_eq!(cache.get(&Colliding(2)), Some(&"two"));
    assert_eq!(cache.get(&Colliding(3)), None);
}

#[test]
fn stats_accumulate_across_operations() {
    let mut cache = BoundedCache::new(1, 0);
    cache.insert("a", 1u32, 1);
    cache.get(&"a");
    cache.get(&"missing");
    cache.insert("b", 2, 1);
    let stats = cache.stats();
    assert_eq!(stats.hits, 1);
    assert_eq!(stats.misses, 1);
    assert_eq!(stats.insertions, 2);
    assert_eq!(stats.evictions, 1);
}

#[test]
fn clear_empties_entries_but_keeps_statistics() {
    let mut cache = BoundedCache::new(0, 0);
    cache.insert("a", 1u32, 1);
    cache.clear();
    assert!(cache.is_empty());
    assert_eq!(cache.bytes(), 0);
    assert_eq!(cache.stats().insertions, 1);
}

#[test]
fn repeated_hits_keep_lru_metadata_bounded() {
    let mut cache = BoundedCache::new(2, 128);
    cache.insert("first", 1u32, 4);
    cache.insert("second", 2, 4);
    for _ in 0..10_000 {
        assert_eq!(cache.get(&"first"), Some(&1));
    }
    assert_eq!(cache.iter_lru().count(), 2);
    cache.insert("third", 3, 4);
    assert!(!cache.contains(&"second"));
    assert_eq!(cache.get(&"first"), Some(&1));
}

#[test]
fn borrowed_lookup_preserves_lru_and_errors_do_not_publish_hits() {
    let mut cache = BoundedCache::new(2, 128);
    cache.insert(String::from("first"), 1, 4);
    cache.insert(String::from("second"), 2, 4);
    let error = cache.try_get_matching(|_| Err::<bool, _>("invalid frame"));
    assert_eq!(error, Err("invalid frame"));
    assert_eq!(cache.stats().hits, 0);
    assert_eq!(cache.stats().misses, 0);
    for _ in 0..1000 {
        assert_eq!(
            cache.try_get_matching(|key| Ok::<_, ()>(key == "first")),
            Ok(Some(&1))
        );
    }
    assert_eq!(cache.iter_lru().count(), 2);
    cache.insert(String::from("third"), 3, 4);
    assert!(!cache.contains(&String::from("second")));
    assert_eq!(
        cache.try_get_matching(|key| Ok::<_, ()>(key == "missing")),
        Ok(None)
    );
    assert_eq!(cache.stats().hits, 1000);
    assert_eq!(cache.stats().misses, 1);
}
