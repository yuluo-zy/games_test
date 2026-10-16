//! 有界缓存原语：条目数与字节预算受限，哈希只定位候选、完整键相等才确认命中。
//!
//! # 设计
//!
//! [`BoundedCache`] 是图编译计划、子图计划与结果缓存的公共存储，直接落实
//! `ARCHITECTURE.md` §4.4 / §3.11 的两条缓存硬约束：
//! - 缓存同时受**条目数**与**字节预算**约束，任一超限就逐出最久未使用（LRU）条目；
//! - 查找用 `HashMap`（哈希只决定桶），命中的最终判定是 `K: Eq` 的完整比较。即使两个
//!   不同键哈希到同一桶，也不会误命中；调用方绝不能把 64 位哈希值本身当键用。
//!
//! LRU 用“代际戳 + 双端队列”实现：每次访问把键连同新戳压入队尾，逐出时从队头弹出、
//! 跳过戳已过期的陈旧记录，摊销 O(1)、无需每命中 O(n) 移动。统计（命中/未命中/插入/
//! 逐出/拒绝超大项）只增不减，供缓存 A/B 与失效原因观测。

use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::hash::Hash;

/// 缓存运行统计（累计）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub insertions: u64,
    pub evictions: u64,
    pub evicted_bytes: usize,
    /// 因单条字节数超过整库预算而拒绝缓存、原样退回的次数。
    pub rejected_oversized: u64,
}

/// `insert` 的结果：明确区分“已存入”与“超预算被拒绝”。
pub enum Inserted<V> {
    /// 已存入；内层是替换掉同键旧值（无则 `None`）。
    Stored(Option<V>),
    /// 单条字节数超过整库预算，未缓存，原值退回。
    Rejected(V),
}

struct Entry<V> {
    value: V,
    bytes: usize,
    /// 最近一次访问的代际戳，与 `order` 里的戳配对以跳过陈旧记录。
    stamp: u64,
}

/// 有界 LRU 缓存。
pub struct BoundedCache<K, V> {
    map: HashMap<K, Entry<V>>,
    /// (key, stamp) 的访问顺序队列；陈旧记录在逐出时惰性清理。
    order: VecDeque<(K, u64)>,
    next_stamp: u64,
    max_entries: usize,
    max_bytes: usize,
    total_bytes: usize,
    stats: CacheStats,
}

impl<K: Clone + Eq + Hash, V> BoundedCache<K, V> {
    /// 新建缓存。`max_entries`/`max_bytes` 为 0 表示该维度不限。
    pub fn new(max_entries: usize, max_bytes: usize) -> Self {
        Self {
            map: HashMap::new(),
            order: VecDeque::new(),
            next_stamp: 0,
            max_entries,
            max_bytes,
            total_bytes: 0,
            stats: CacheStats::default(),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if !self.map.contains_key(key) {
            self.stats.misses += 1;
            return None;
        }
        self.stats.hits += 1;
        self.touch(key);
        self.map.get(key).map(|entry| &entry.value)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if !self.map.contains_key(key) {
            self.stats.misses += 1;
            return None;
        }
        self.stats.hits += 1;
        self.touch(key);
        self.map.get_mut(key).map(|entry| &mut entry.value)
    }

    /// 插入或替换，随后按预算逐出 LRU 条目。单条字节数超过整库字节预算时拒绝并退回。
    pub fn insert(&mut self, key: K, value: V, bytes: usize) -> Inserted<V> {
        if self.max_bytes != 0 && bytes > self.max_bytes {
            self.stats.rejected_oversized += 1;
            return Inserted::Rejected(value);
        }
        let old = self.replace(key, value, bytes);
        self.evict_to_limits();
        self.stats.insertions += 1;
        Inserted::Stored(old)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let entry = self.map.remove(key)?;
        self.total_bytes -= entry.bytes;
        Some(entry.value)
    }

    pub fn contains(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    pub fn clear(&mut self) {
        self.map.clear();
        self.order.clear();
        self.total_bytes = 0;
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn bytes(&self) -> usize {
        self.total_bytes
    }

    pub fn max_entries(&self) -> usize {
        self.max_entries
    }

    pub fn max_bytes(&self) -> usize {
        self.max_bytes
    }

    pub fn stats(&self) -> CacheStats {
        self.stats
    }

    fn replace(&mut self, key: K, value: V, bytes: usize) -> Option<V> {
        self.next_stamp = self.next_stamp.wrapping_add(1);
        let stamp = self.next_stamp;
        if let Some(old) = self.map.remove(&key) {
            self.total_bytes -= old.bytes;
            self.map.insert(
                key.clone(),
                Entry {
                    value,
                    bytes,
                    stamp,
                },
            );
            self.total_bytes += bytes;
            self.order.push_back((key, stamp));
            Some(old.value)
        } else {
            self.map.insert(
                key.clone(),
                Entry {
                    value,
                    bytes,
                    stamp,
                },
            );
            self.total_bytes += bytes;
            self.order.push_back((key, stamp));
            None
        }
    }

    fn touch(&mut self, key: &K) {
        self.next_stamp = self.next_stamp.wrapping_add(1);
        let stamp = self.next_stamp;
        if let Some(entry) = self.map.get_mut(key) {
            entry.stamp = stamp;
        }
        self.order.push_back((key.clone(), stamp));
    }

    fn evict_to_limits(&mut self) {
        loop {
            let over_entries = self.max_entries != 0 && self.map.len() > self.max_entries;
            let over_bytes = self.max_bytes != 0 && self.total_bytes > self.max_bytes;
            if !over_entries && !over_bytes {
                return;
            }
            let Some((key, stamp)) = self.order.pop_front() else {
                return;
            };
            let is_current = matches!(self.map.get(&key), Some(entry) if entry.stamp == stamp);
            if !is_current {
                continue;
            }
            let entry = self.map.remove(&key).expect("刚校验过条目仍存在");
            self.total_bytes -= entry.bytes;
            self.stats.evictions += 1;
            self.stats.evicted_bytes += entry.bytes;
        }
    }
}

impl<K: Clone + Eq + Hash, V> fmt::Debug for BoundedCache<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundedCache")
            .field("entries", &self.map.len())
            .field("bytes", &self.total_bytes)
            .field("max_entries", &self.max_entries)
            .field("max_bytes", &self.max_bytes)
            .field("stats", &self.stats)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
