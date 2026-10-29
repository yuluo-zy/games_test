//! 统一内存预算：按用途分类记账，并在水位触发淘汰/背压决策。
//!
//! # 设计
//!
//! [`MemoryBudgetController`] 把设备内存分成四类：
//! - [`BudgetCategory::Live`]：当前存活的资源；
//! - [`BudgetCategory::InFlight`]：GPU 正在使用、尚未退役的资源；
//! - [`BudgetCategory::Cache`]：可淘汰的缓存（管线、编译计划、结果缓存等）；
//! - [`BudgetCategory::Retiring`]：已提交回收、等待完成点后销毁的资源。
//!
//! 正确性核心：`in_flight` 与 `retiring` 是**已提交的事实**，只能 [`MemoryBudgetController::record`]
//! 记账、不可因超限而拒绝；`live`/`cache` 是**可拒绝的分配**，用 [`MemoryBudgetController::reserve`]
//! 在超限时返回错误，绝不隐藏 OOM。
//!
//! 水位带滞回：总占用达到 `high_water` 进入压力模式，回落到 `low_water` 才退出，避免在
//! 边界反复抖动。压力下优先 [`BudgetAction::EvictCache`]（淘汰空闲缓存），没有缓存可淘汰
//! 才 [`BudgetAction::Backpressure`]（延后低优先级分配/上传）。这些是策略建议，不擅自替
//! 上层做淘汰或降级。

use thiserror::Error;

/// 预算记账的用途分类。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BudgetCategory {
    Live,
    InFlight,
    Cache,
    Retiring,
}

/// 各分类的字节占用快照。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BudgetUsage {
    pub live: u64,
    pub in_flight: u64,
    pub cache: u64,
    pub retiring: u64,
}

impl BudgetUsage {
    pub fn total(self) -> u64 {
        self.live
            .saturating_add(self.in_flight)
            .saturating_add(self.cache)
            .saturating_add(self.retiring)
    }
}

/// 越限错误：请求无法在预算内满足。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
#[error("内存预算超限：需要 {requested} 字节，当前 {total} 字节，上限 {limit} 字节")]
pub struct BudgetExceeded {
    pub requested: u64,
    pub total: u64,
    pub limit: u64,
}

/// 压力下每帧建议采取的动作。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BudgetAction {
    /// 无需动作。
    None,
    /// 优先淘汰空闲缓存以回收内存。
    EvictCache,
    /// 没有缓存可淘汰，触发背压（延后低优先级工作）。
    Backpressure,
}

/// 统一内存预算控制器。
#[derive(Clone, Copy, Debug)]
pub struct MemoryBudgetController {
    limit: u64,
    high_water: u64,
    low_water: u64,
    usage: BudgetUsage,
    under_pressure: bool,
}
impl Default for MemoryBudgetController {
    fn default() -> Self {
        Self::new(0)
    }
}

impl MemoryBudgetController {
    /// 用默认水位（高 80%、低 60%）创建。`limit == 0` 表示不设上限。
    pub fn new(limit: u64) -> Self {
        let high_water = (limit as u128 * 80 / 100) as u64;
        let low_water = (limit as u128 * 60 / 100) as u64;
        Self {
            limit,
            high_water,
            low_water,
            usage: BudgetUsage::default(),
            under_pressure: false,
        }
    }

    /// 显式指定水位。`low_water <= high_water <= limit` 是内部不变量。
    pub fn new_with_watermarks(limit: u64, high_water: u64, low_water: u64) -> Self {
        debug_assert!(low_water <= high_water && high_water <= limit);
        Self {
            limit,
            high_water,
            low_water,
            usage: BudgetUsage::default(),
            under_pressure: false,
        }
    }

    /// 为可拒绝的分配（live/cache）预留内存；越限返回错误并回滚，不改变账本。
    pub fn reserve(&mut self, category: BudgetCategory, bytes: u64) -> Result<(), BudgetExceeded> {
        debug_assert!(matches!(
            category,
            BudgetCategory::Live | BudgetCategory::Cache
        ));
        let total = self
            .usage
            .live
            .checked_add(self.usage.in_flight)
            .and_then(|n| n.checked_add(self.usage.cache))
            .and_then(|n| n.checked_add(self.usage.retiring))
            .and_then(|n| n.checked_add(bytes));
        if total.is_none_or(|n| self.limit != 0 && n > self.limit) {
            return Err(BudgetExceeded {
                requested: bytes,
                total: self.usage.total(),
                limit: self.limit,
            });
        }
        self.add(category, bytes);
        self.refresh_pressure();
        Ok(())
    }

    pub fn set_limit(&mut self, limit: u64) -> Result<(), BudgetExceeded> {
        if limit != 0 && self.total() > limit {
            return Err(BudgetExceeded {
                requested: 0,
                total: self.total(),
                limit,
            });
        }
        self.limit = limit;
        self.high_water = (u128::from(limit) * 80 / 100) as u64;
        self.low_water = (u128::from(limit) * 60 / 100) as u64;
        self.refresh_pressure();
        Ok(())
    }

    /// 记录已提交、不可回滚的占用（in-flight/retiring），永不失败。
    pub fn record(&mut self, category: BudgetCategory, bytes: u64) {
        debug_assert!(matches!(
            category,
            BudgetCategory::InFlight | BudgetCategory::Retiring
        ));
        self.add(category, bytes);
        self.refresh_pressure();
    }

    /// Record an already-owned cache charge that may exceed a lowered limit.
    #[doc(hidden)]
    pub fn record_cache(&mut self, bytes: u64) {
        self.add(BudgetCategory::Cache, bytes);
        self.refresh_pressure();
    }

    /// 释放一块内存。
    pub fn release(&mut self, category: BudgetCategory, bytes: u64) {
        self.sub(category, bytes);
        self.refresh_pressure();
    }
    #[doc(hidden)]
    pub fn reclassify(&mut self, from: BudgetCategory, to: BudgetCategory, bytes: u64) {
        self.sub(from, bytes);
        self.add(to, bytes);
        self.refresh_pressure();
    }

    /// 淘汰最多 `bytes` 的缓存，返回实际淘汰量（不超过当前 cache 占用）。
    pub fn evict_cache(&mut self, bytes: u64) -> u64 {
        let evicted = bytes.min(self.usage.cache);
        self.sub(BudgetCategory::Cache, evicted);
        self.refresh_pressure();
        evicted
    }

    pub fn usage(&self) -> BudgetUsage {
        self.usage
    }

    pub fn total(&self) -> u64 {
        self.usage.total()
    }

    pub fn available(&self) -> u64 {
        if self.limit == 0 { u64::MAX } else { self.limit.saturating_sub(self.total()) }
    }

    pub fn limit(&self) -> u64 {
        self.limit
    }

    pub fn is_under_pressure(&self) -> bool {
        self.under_pressure
    }

    /// 每帧根据压力给出动作建议。非压力期为 [`BudgetAction::None`]。
    pub fn tick(&self) -> BudgetAction {
        if !self.under_pressure {
            return BudgetAction::None;
        }
        if self.usage.cache > 0 {
            BudgetAction::EvictCache
        } else {
            BudgetAction::Backpressure
        }
    }

    fn add(&mut self, category: BudgetCategory, bytes: u64) {
        match category {
            BudgetCategory::Live => self.usage.live = self.usage.live.saturating_add(bytes),
            BudgetCategory::InFlight => {
                self.usage.in_flight = self.usage.in_flight.saturating_add(bytes)
            }
            BudgetCategory::Cache => self.usage.cache = self.usage.cache.saturating_add(bytes),
            BudgetCategory::Retiring => {
                self.usage.retiring = self.usage.retiring.saturating_add(bytes)
            }
        }
    }

    fn sub(&mut self, category: BudgetCategory, bytes: u64) {
        match category {
            BudgetCategory::Live => self.usage.live = self.usage.live.saturating_sub(bytes),
            BudgetCategory::InFlight => {
                self.usage.in_flight = self.usage.in_flight.saturating_sub(bytes)
            }
            BudgetCategory::Cache => self.usage.cache = self.usage.cache.saturating_sub(bytes),
            BudgetCategory::Retiring => {
                self.usage.retiring = self.usage.retiring.saturating_sub(bytes)
            }
        }
    }

    fn refresh_pressure(&mut self) {
        if self.limit == 0 {
            // 未设上限时没有水位，也永远不会进入压力模式。
            self.under_pressure = false;
            return;
        }
        if self.total() >= self.high_water {
            self.under_pressure = true;
        } else if self.total() <= self.low_water {
            self.under_pressure = false;
        }
    }
}

/// A cache whose idle entries can be released under memory pressure.
///
/// Implementations release only entries that have no CPU owner and whose
/// every GPU use is reached in `completed`, least recently used first, until
/// about `target_bytes` are released or nothing idle remains. They are called
/// from inside allocations and must never block: a cache that is busy (its
/// lock is held, e.g. by the allocation that triggered eviction) returns 0.
pub trait Evictable: Send + Sync {
    /// Release idle entries; returns the bytes released (native memory or
    /// accounted host memory).
    fn evict_idle(&self, completed: &crate::sync::CompletedValues, target_bytes: u64) -> u64;
}

/// Order in which registered caches are asked to release entries. Cheaper
/// to rebuild comes first.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum EvictionPriority {
    /// Staging and transient pools, recreated on demand.
    Transient,
    #[default]
    Normal,
    /// Expensive to rebuild (pipelines, persistent results).
    Expensive,
}

struct EvictionEntry {
    priority: EvictionPriority,
    sequence: u64,
    cache: std::sync::Weak<dyn Evictable>,
}

/// Device-level registry of [`Evictable`] caches. Holds weak references only:
/// a dropped cache unregisters itself.
#[derive(Default)]
pub struct EvictionRegistry {
    entries: parking_lot::Mutex<Vec<EvictionEntry>>,
    sequence: std::sync::atomic::AtomicU64,
    evicted_bytes: std::sync::atomic::AtomicU64,
    passes: std::sync::atomic::AtomicU64,
}
impl EvictionRegistry {
    pub fn register(&self, cache: std::sync::Weak<dyn Evictable>, priority: EvictionPriority) {
        let sequence = self
            .sequence
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let mut entries = self.entries.lock();
        entries.retain(|entry| entry.cache.strong_count() != 0);
        if entries.try_reserve(1).is_err() {
            log::warn!("eviction registry is out of host memory; cache not registered");
            return;
        }
        let at = entries
            .partition_point(|entry| (entry.priority, entry.sequence) <= (priority, sequence));
        entries.insert(
            at,
            EvictionEntry {
                priority,
                sequence,
                cache,
            },
        );
    }
    /// Live registered caches.
    pub fn len(&self) -> usize {
        self.entries
            .lock()
            .iter()
            .filter(|entry| entry.cache.strong_count() != 0)
            .count()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Ask caches in priority/registration order to release idle entries
    /// until `target_bytes` are released. Caches run without the registry
    /// lock held. Returns the bytes released.
    pub fn evict(&self, completed: &crate::sync::CompletedValues, target_bytes: u64) -> u64 {
        let caches: smallvec::SmallVec<[std::sync::Arc<dyn Evictable>; 8]> = {
            let mut entries = self.entries.lock();
            entries.retain(|entry| entry.cache.strong_count() != 0);
            entries
                .iter()
                .filter_map(|entry| entry.cache.upgrade())
                .collect()
        };
        if caches.is_empty() {
            return 0;
        }
        self.passes
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let target = target_bytes.max(1);
        let mut released = 0u64;
        for cache in caches {
            released = released.saturating_add(cache.evict_idle(completed, target - released));
            if released >= target {
                break;
            }
        }
        self.evicted_bytes
            .fetch_add(released, std::sync::atomic::Ordering::Relaxed);
        released
    }
    /// `(eviction passes, bytes released)` since creation.
    pub fn stats(&self) -> (u64, u64) {
        (
            self.passes.load(std::sync::atomic::Ordering::Relaxed),
            self.evicted_bytes
                .load(std::sync::atomic::Ordering::Relaxed),
        )
    }
}

#[cfg(test)]
mod tests;
