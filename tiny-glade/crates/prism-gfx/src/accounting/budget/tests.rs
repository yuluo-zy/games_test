use super::*;

#[test]
fn reserve_within_limit_succeeds_and_over_limit_fails() {
    let mut budget = MemoryBudgetController::new(100);
    assert!(budget.reserve(BudgetCategory::Live, 60).is_ok());
    assert!(budget.reserve(BudgetCategory::Cache, 30).is_ok());
    let error = budget.reserve(BudgetCategory::Live, 20).unwrap_err();
    assert_eq!(error.requested, 20);
    assert_eq!(budget.total(), 90, "失败的 reserve 必须回滚");
}

#[test]
fn record_is_infallible_even_over_limit() {
    let mut budget = MemoryBudgetController::new(100);
    budget.record(BudgetCategory::InFlight, 150);
    assert_eq!(budget.total(), 150);
    assert!(budget.is_under_pressure());
}

#[test]
fn pressure_has_hysteresis_between_watermarks() {
    let mut budget = MemoryBudgetController::new_with_watermarks(100, 80, 60);
    budget.reserve(BudgetCategory::Live, 85).unwrap();
    assert!(budget.is_under_pressure());
    // 回落到 60..80 之间：滞回保持压力态，不反复抖动。
    budget.release(BudgetCategory::Live, 15);
    assert_eq!(budget.total(), 70);
    assert!(budget.is_under_pressure());
    // 回落到低水位以下才退出。
    budget.release(BudgetCategory::Live, 15);
    assert_eq!(budget.total(), 55);
    assert!(!budget.is_under_pressure());
}

#[test]
fn tick_prefers_evict_cache_then_backpressure() {
    let mut budget = MemoryBudgetController::new_with_watermarks(100, 80, 60);
    budget.reserve(BudgetCategory::Live, 90).unwrap();
    budget.reserve(BudgetCategory::Cache, 10).unwrap();
    assert_eq!(budget.tick(), BudgetAction::EvictCache);

    budget.evict_cache(10);
    assert_eq!(budget.tick(), BudgetAction::Backpressure);
    budget.release(BudgetCategory::Live, 90);
    assert_eq!(budget.tick(), BudgetAction::None);
}

#[test]
fn evict_cache_is_capped_at_current_cache_bytes() {
    let mut budget = MemoryBudgetController::new(1000);
    budget.reserve(BudgetCategory::Cache, 100).unwrap();
    assert_eq!(budget.evict_cache(1000), 100);
    assert_eq!(budget.usage().cache, 0);
}

#[test]
fn zero_limit_means_unbounded() {
    let mut budget = MemoryBudgetController::new(0);
    assert!(budget.reserve(BudgetCategory::Live, u64::MAX / 2).is_ok());
    assert!(!budget.is_under_pressure());
}

#[test]
fn reclassifying_existing_memory_preserves_pressure_hysteresis() {
    let mut budget = MemoryBudgetController::new_with_watermarks(100, 80, 60);
    budget.reserve(BudgetCategory::Live, 90).unwrap();
    budget.release(BudgetCategory::Live, 20);
    assert!(budget.is_under_pressure());
    budget.reclassify(BudgetCategory::Live, BudgetCategory::Cache, 70);
    assert_eq!(budget.total(), 70);
    assert!(budget.is_under_pressure());
}

struct FakeCache {
    /// `(bytes, completion value, cpu referenced)` in LRU order.
    entries: parking_lot::Mutex<Vec<(u64, u64, bool)>>,
    order: std::sync::Arc<parking_lot::Mutex<Vec<&'static str>>>,
    name: &'static str,
}
impl Evictable for FakeCache {
    fn evict_idle(&self, completed: &crate::sync::CompletedValues, target: u64) -> u64 {
        let Some(mut entries) = self.entries.try_lock() else {
            return 0;
        };
        self.order.lock().push(self.name);
        let queue = crate::sync::QueueId::new(0).unwrap();
        let mut released = 0;
        entries.retain(|&(bytes, value, referenced)| {
            if released >= target || referenced || completed.get(queue) < value {
                return true;
            }
            released += bytes;
            false
        });
        released
    }
}

#[test]
fn eviction_releases_only_idle_entries_in_priority_order_and_skips_busy_caches() {
    let registry = EvictionRegistry::default();
    let order = std::sync::Arc::new(parking_lot::Mutex::new(Vec::new()));
    let cache = |name, entries| {
        std::sync::Arc::new(FakeCache {
            entries: parking_lot::Mutex::new(entries),
            order: order.clone(),
            name,
        })
    };
    // In flight (value 9 not reached) and CPU-referenced entries survive.
    let pipelines = cache("pipelines", vec![(100, 0, false)]);
    let staging = cache(
        "staging",
        vec![
            (10, 1, false),
            (20, 9, false),
            (30, 1, true),
            (40, 2, false),
        ],
    );
    let busy = cache("busy", vec![(1000, 0, false)]);
    let pipelines_dyn: std::sync::Arc<dyn Evictable> = pipelines.clone();
    let staging_dyn: std::sync::Arc<dyn Evictable> = staging.clone();
    let busy_dyn: std::sync::Arc<dyn Evictable> = busy.clone();
    registry.register(
        std::sync::Arc::downgrade(&pipelines_dyn),
        EvictionPriority::Expensive,
    );
    registry.register(
        std::sync::Arc::downgrade(&busy_dyn),
        EvictionPriority::Normal,
    );
    registry.register(
        std::sync::Arc::downgrade(&staging_dyn),
        EvictionPriority::Transient,
    );
    let mut completed = crate::sync::CompletedValues::default();
    completed.raise(crate::sync::QueueId::new(0).unwrap(), 2);
    let guard = busy.entries.lock();
    assert_eq!(registry.evict(&completed, 50), 50);
    assert_eq!(
        *order.lock(),
        ["staging"],
        "target met before costlier caches"
    );
    assert_eq!(
        staging.entries.lock().as_slice(),
        [(20, 9, false), (30, 1, true)]
    );
    assert_eq!(registry.evict(&completed, 1000), 100);
    assert_eq!(*order.lock(), ["staging", "staging", "pipelines"]);
    drop(guard);
    assert_eq!(registry.evict(&completed, 1000), 1000);
    drop((pipelines_dyn, pipelines));
    assert_eq!(registry.len(), 2, "dropped caches unregister");
    assert_eq!(registry.stats().1, 1150);
}
