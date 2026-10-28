//! 有界缓存原语：条目数与字节预算受限，哈希只定位候选、完整键相等才确认命中。
//!
//! # 设计
//!
//! [`BoundedCache`] 是图编译计划、子图计划与结果缓存的公共存储，直接落实
//! `ARCHITECTURE.md` 的「内存预算、记账与回收」一节中的两条缓存硬约束：
//! - 缓存同时受**条目数**与**字节预算**约束，任一超限就逐出最久未使用（LRU）条目；
//! - 查找用 `HashMap`（哈希只决定桶），命中的最终判定是 `K: Eq` 的完整比较。即使两个
//!   不同键哈希到同一桶，也不会误命中；调用方绝不能把 64 位哈希值本身当键用。
//!
//! A bounded reusable node arena maintains LRU links. Hits update links in O(1),
//! without key cloning, allocation or an accumulating access log.

use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

/// Fixed diagnostics, recorded without retaining caller names or growing logs.
/// When several key fields change, owners report the first differing field in
/// their documented comparison order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[repr(usize)]
pub enum CacheReason {
    Cold,
    Disabled,
    UnknownDependencies,
    DependencyVersion,
    Parameters,
    ShaderImplementation,
    ShaderInterface,
    Descriptor,
    Sharing,
    DeviceGeneration,
    QueuePolicy,
    SubgraphInterface,
    GraphStructure,
    AccessSemantics,
    CompilerVersion,
    MetadataLimit,
    ExplicitClear,
    ExplicitRemoval,
    CapacityEntries,
    CapacityBytes,
    CapacityChanged,
    Pressure,
    AbortedFrame,
}
impl CacheReason {
    pub const ALL: [Self; 23] = [
        Self::Cold,
        Self::Disabled,
        Self::UnknownDependencies,
        Self::DependencyVersion,
        Self::Parameters,
        Self::ShaderImplementation,
        Self::ShaderInterface,
        Self::Descriptor,
        Self::Sharing,
        Self::DeviceGeneration,
        Self::QueuePolicy,
        Self::SubgraphInterface,
        Self::GraphStructure,
        Self::AccessSemantics,
        Self::CompilerVersion,
        Self::MetadataLimit,
        Self::ExplicitClear,
        Self::ExplicitRemoval,
        Self::CapacityEntries,
        Self::CapacityBytes,
        Self::CapacityChanged,
        Self::Pressure,
        Self::AbortedFrame,
    ];
}
const CACHE_REASON_COUNT: usize = CacheReason::AbortedFrame as usize + 1;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CacheDiagnostics {
    misses: [u64; CACHE_REASON_COUNT],
    invalidations: [u64; CACHE_REASON_COUNT],
}
impl CacheDiagnostics {
    pub fn misses(self, reason: CacheReason) -> u64 {
        self.misses[reason as usize]
    }
    pub fn invalidations(self, reason: CacheReason) -> u64 {
        self.invalidations[reason as usize]
    }
    pub(crate) fn miss(&mut self, reason: CacheReason) {
        self.misses[reason as usize] += 1;
    }
    pub(crate) fn invalidate(&mut self, reason: CacheReason, count: usize) {
        self.invalidations[reason as usize] += count as u64;
    }
    pub(crate) fn merge_invalidations(&mut self, other: Self) {
        for (target, count) in self.invalidations.iter_mut().zip(other.invalidations) {
            *target += count;
        }
    }
}

/// 缓存运行统计（累计）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CacheStats {
    pub diagnostics: CacheDiagnostics,
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

struct Entry<K, V> {
    key: K,
    value: V,
    bytes: usize,
    previous: Option<usize>,
    next: Option<usize>,
}

/// 有界 LRU 缓存。
pub struct BoundedCache<K, V> {
    map: HashMap<K, usize>,
    entries: Vec<Option<Entry<K, V>>>,
    free: Vec<usize>,
    oldest: Option<usize>,
    newest: Option<usize>,
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
            entries: Vec::new(), free: Vec::new(), oldest: None, newest: None,
            max_entries,
            max_bytes,
            total_bytes: 0,
            stats: CacheStats::default(),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        let Some(&index) = self.map.get(key) else {
            self.stats.misses += 1;
            self.stats.diagnostics.miss(CacheReason::Cold);
            return None;
        };
        self.stats.hits += 1;
        self.touch(index);
        self.entries[index].as_ref().map(|entry| &entry.value)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let Some(&index) = self.map.get(key) else {
            self.stats.misses += 1;
            self.stats.diagnostics.miss(CacheReason::Cold);
            return None;
        };
        self.stats.hits += 1;
        self.touch(index);
        self.entries[index].as_mut().map(|entry| &mut entry.value)
    }

    /// Compare a borrowed input with complete keys without constructing an owned
    /// key. The bounded LRU is scanned newest first; a hit moves the existing key
    /// and does not clone its owned strings or vectors. A failed validation does
    /// not publish a hit or change recency.
    pub fn try_get_matching<E>(&mut self, mut matches: impl FnMut(&K) -> Result<bool, E>) -> Result<Option<&V>, E> {
        let mut cursor = self.newest;
        while let Some(index) = cursor {
            let entry = self.entries[index].as_ref().expect("live cache entry");
            cursor = entry.previous;
            if matches(&entry.key)? {
                self.touch(index);
                self.stats.hits += 1;
                return Ok(self.entries[index].as_ref().map(|entry| &entry.value));
            }
        }
        self.stats.misses += 1;
        self.stats.diagnostics.miss(CacheReason::Cold);
        Ok(None)
    }

    /// Refine an enabled lookup's generic miss after the owner has constructed
    /// and compared its complete key. This is never called on the hit path.
    pub(crate) fn classify_miss(&mut self, reason: CacheReason) {
        if reason != CacheReason::Cold {
            let cold = &mut self.stats.diagnostics.misses[CacheReason::Cold as usize];
            debug_assert!(*cold != 0);
            *cold = cold.saturating_sub(1);
            self.stats.diagnostics.miss(reason);
        }
    }
    pub(crate) fn record_bypass(&mut self, reason: CacheReason) {
        self.stats.diagnostics.miss(reason);
    }

    /// Read-only traversal for bounded dependency invalidation and accounting.
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.entries.iter().flatten().map(|entry| (&entry.key, &entry.value))
    }

    /// Preserve recency when an owner replaces cache backing during admission.
    pub(crate) fn iter_lru(&self) -> impl Iterator<Item = (&K, &V)> {
        std::iter::successors(self.oldest, |&index| self.entries[index].as_ref().unwrap().next)
            .map(|index| { let entry = self.entries[index].as_ref().unwrap(); (&entry.key, &entry.value) })
    }

    /// Return the oldest entry that may be evicted without changing recency.
    /// A cache owner uses this to keep externally held or in-flight values live.
    pub(crate) fn lru_key_where(&self, mut ready: impl FnMut(&V) -> bool) -> Option<K> {
        self.iter_lru().find_map(|(key, value)| ready(value).then(|| key.clone()))
    }

    /// Reserve optional-cache publication storage before any GPU submission.
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> {
        self.map.try_reserve(additional)?;
        self.entries.try_reserve(additional)?;
        self.free.try_reserve(additional)
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
        self.remove_with_reason(key, CacheReason::ExplicitRemoval)
    }
    pub(crate) fn remove_with_reason(&mut self, key: &K, reason: CacheReason) -> Option<V> {
        let index = *self.map.get(key)?;
        let entry = self.remove_index(index);
        self.stats.diagnostics.invalidate(reason, 1);
        Some(entry.value)
    }

    pub fn contains(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    pub fn clear(&mut self) {
        self.clear_with_reason(CacheReason::ExplicitClear);
    }
    pub(crate) fn clear_with_reason(&mut self, reason: CacheReason) {
        self.stats.diagnostics.invalidate(reason, self.map.len());
        self.map.clear(); self.entries.clear(); self.free.clear();
        self.oldest = None; self.newest = None; self.total_bytes = 0;
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
        if let Some(&index) = self.map.get(&key) {
            let entry = self.entries[index].as_mut().unwrap();
            self.total_bytes = self.total_bytes - entry.bytes + bytes;
            entry.bytes = bytes;
            let old = std::mem::replace(&mut entry.value, value);
            self.touch(index);
            return Some(old);
        }
        let index = self.free.pop().unwrap_or_else(|| { self.entries.push(None); self.entries.len() - 1 });
        self.map.insert(key.clone(), index);
        self.entries[index] = Some(Entry { key, value, bytes, previous: self.newest, next: None });
        if let Some(previous) = self.newest { self.entries[previous].as_mut().unwrap().next = Some(index); }
        else { self.oldest = Some(index); }
        self.newest = Some(index);
        self.total_bytes += bytes;
        None
    }

    fn touch(&mut self, index: usize) {
        if self.newest == Some(index) { return; }
        self.unlink(index);
        let entry = self.entries[index].as_mut().unwrap();
        entry.previous = self.newest; entry.next = None;
        if let Some(previous) = self.newest { self.entries[previous].as_mut().unwrap().next = Some(index); }
        else { self.oldest = Some(index); }
        self.newest = Some(index);
    }
    fn unlink(&mut self, index: usize) {
        let entry = self.entries[index].as_ref().unwrap();
        let (previous, next) = (entry.previous, entry.next);
        if let Some(previous) = previous { self.entries[previous].as_mut().unwrap().next = next; }
        else { self.oldest = next; }
        if let Some(next) = next { self.entries[next].as_mut().unwrap().previous = previous; }
        else { self.newest = previous; }
    }
    fn remove_index(&mut self, index: usize) -> Entry<K, V> {
        self.unlink(index);
        let entry = self.entries[index].take().unwrap();
        self.map.remove(&entry.key); self.free.push(index); self.total_bytes -= entry.bytes;
        entry
    }

    fn evict_to_limits(&mut self) {
        loop {
            let over_entries = self.max_entries != 0 && self.map.len() > self.max_entries;
            let over_bytes = self.max_bytes != 0 && self.total_bytes > self.max_bytes;
            if !over_entries && !over_bytes { return; }
            let Some(index) = self.oldest else { return; };
            let entry = self.remove_index(index);
            self.stats.evictions += 1; self.stats.evicted_bytes += entry.bytes;
            self.stats.diagnostics.invalidate(if over_entries { CacheReason::CapacityEntries } else { CacheReason::CapacityBytes }, 1);
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
mod tests;
