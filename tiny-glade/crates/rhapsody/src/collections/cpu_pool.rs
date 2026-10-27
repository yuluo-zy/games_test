//! Bounded CPU storage with exclusive leases. Native/GPU lifetime tracking does
//! not belong here: payloads own their exact CPU reservations and reset rules.
use crate::{Error, Result};
use crate::accounting::{AllocatorStats, BudgetAction, BudgetCategory, CpuMemoryReservation};
use parking_lot::Mutex;
use std::sync::Arc;

pub(crate) trait CpuPoolValue {
    type Statistics: Copy + Default;
    fn capacity_bytes(&self) -> usize;
    fn available(&self) -> bool {
        true
    }
    fn reset(&mut self);
    fn record_return(&mut self, _: &mut Self::Statistics) {}
    fn reclassify(&mut self, category: BudgetCategory);
}

#[derive(Clone, Copy)]
pub(crate) struct CpuPoolLimits {
    pub items: usize,
    pub total_bytes: usize,
    pub cached_bytes: usize,
    pub full: fn() -> Error,
}

#[derive(Clone, Copy, Default)]
pub(crate) struct CpuPoolStats<S> {
    pub checked_out: usize,
    pub cached: usize,
    pub pinned: usize,
    pub live_bytes: usize,
    pub cached_bytes: usize,
    pub metadata_bytes: usize,
    pub peak_total_bytes: usize,
    pub peak_checked_out: usize,
    pub creations: u64,
    pub capacity_growths: u64,
    pub capacity_stalls: u64,
    pub payload: S,
}
impl<S> CpuPoolStats<S> {
    fn total_bytes(&self) -> usize {
        self.live_bytes
            .saturating_add(self.cached_bytes)
            .saturating_add(self.metadata_bytes)
    }
}
struct Entry<T> {
    value: T,
    cached: bool,
}
struct State<T: CpuPoolValue> {
    idle: Vec<Entry<T>>,
    stats: CpuPoolStats<T::Statistics>,
}
struct Shared<T: CpuPoolValue> {
    state: Mutex<State<T>>,
    limits: CpuPoolLimits,
    budget: Arc<AllocatorStats>,
    _metadata: CpuMemoryReservation,
}
pub(crate) struct CpuPool<T: CpuPoolValue> {
    shared: Arc<Shared<T>>,
}
impl<T: CpuPoolValue> Clone for CpuPool<T> {
    fn clone(&self) -> Self {
        Self {
            shared: self.shared.clone(),
        }
    }
}
impl<T: CpuPoolValue> CpuPool<T> {
    pub(crate) fn new(limits: CpuPoolLimits, budget: Arc<AllocatorStats>) -> Result<Self> {
        if !(1..=256).contains(&limits.items) || limits.cached_bytes > limits.total_bytes {
            return Err(Error::invalid_usage("invalid bounded CPU pool limits"));
        }
        let mut idle = Vec::new();
        idle.try_reserve_exact(limits.items)
            .map_err(|_| Error::OutOfHostMemory)?;
        let metadata_bytes = idle
            .capacity()
            .saturating_mul(std::mem::size_of::<Entry<T>>())
            .saturating_add(std::mem::size_of::<Shared<T>>())
            .saturating_add(2 * std::mem::size_of::<usize>())
            .saturating_add(std::mem::align_of::<Shared<T>>());
        if metadata_bytes >= limits.total_bytes {
            return Err(Error::invalid_usage(
                "CPU pool limit cannot hold its metadata",
            ));
        }
        let metadata = budget.reserve_cpu_bytes(metadata_bytes as u64, BudgetCategory::Live)?;
        Ok(Self {
            shared: Arc::new(Shared {
                state: Mutex::new(State {
                    idle,
                    stats: CpuPoolStats {
                        metadata_bytes,
                        peak_total_bytes: metadata_bytes,
                        ..Default::default()
                    },
                }),
                limits,
                budget,
                _metadata: metadata,
            }),
        })
    }
    /// A snapshot has no accounting side effects. External immutable owners
    /// may have released their last clone since the last pool maintenance.
    pub(crate) fn stats(&self) -> CpuPoolStats<T::Statistics> {
        let state = self.shared.state.lock();
        let mut stats = state.stats;
        if stats.pinned != 0 {
            for entry in &state.idle {
                if !entry.cached && entry.value.available() {
                    let bytes = entry.value.capacity_bytes();
                    stats.pinned -= 1;
                    stats.cached += 1;
                    stats.live_bytes -= bytes;
                    stats.cached_bytes += bytes;
                }
            }
        }
        stats
    }
    pub(crate) fn checkout(&self, create: impl FnOnce() -> Result<T>) -> Result<CpuPoolLease<T>> {
        let mut state = self.shared.state.lock();
        state.refresh();
        let value = if state.stats.cached != 0 {
            let index = state
                .idle
                .iter()
                .rposition(|entry| entry.cached)
                .expect("cached CPU storage");
            let mut value = state.idle.swap_remove(index).value;
            let bytes = value.capacity_bytes();
            state.stats.cached -= 1;
            state.stats.cached_bytes -= bytes;
            state.stats.live_bytes += bytes;
            value.reclassify(BudgetCategory::Live);
            value
        } else {
            if state.stats.checked_out + state.idle.len() >= self.shared.limits.items {
                state.stats.capacity_stalls += 1;
                return Err((self.shared.limits.full)());
            }
            let value = create()?;
            let bytes = value.capacity_bytes();
            if state.stats.total_bytes().saturating_add(bytes) > self.shared.limits.total_bytes {
                state.stats.capacity_stalls += 1;
                return Err((self.shared.limits.full)());
            }
            state.stats.live_bytes += bytes;
            state.stats.creations += 1;
            value
        };
        state.stats.checked_out += 1;
        state.stats.peak_checked_out = state.stats.peak_checked_out.max(state.stats.checked_out);
        state.stats.peak_total_bytes = state.stats.peak_total_bytes.max(state.stats.total_bytes());
        Ok(CpuPoolLease {
            value: Some(value),
            shared: self.shared.clone(),
            reusable: true,
        })
    }
    pub(crate) fn trim_idle(&self, target: usize) -> usize {
        let mut state = self.shared.state.lock();
        state.refresh();
        let mut freed = 0;
        let mut index = state.idle.len();
        while state.stats.cached_bytes > target && index != 0 {
            index -= 1;
            if state.idle[index].cached {
                let value = state.idle.swap_remove(index).value;
                let bytes = value.capacity_bytes();
                state.stats.cached -= 1;
                state.stats.cached_bytes -= bytes;
                freed += bytes;
                drop(value);
            }
        }
        freed
    }
}
impl<T: CpuPoolValue> State<T> {
    fn refresh(&mut self) {
        if self.stats.pinned == 0 {
            return;
        }
        for entry in &mut self.idle {
            if !entry.cached && entry.value.available() {
                entry.value.reset();
                entry.value.record_return(&mut self.stats.payload);
                entry.value.reclassify(BudgetCategory::Cache);
                let bytes = entry.value.capacity_bytes();
                entry.cached = true;
                self.stats.pinned -= 1;
                self.stats.cached += 1;
                self.stats.live_bytes -= bytes;
                self.stats.cached_bytes += bytes;
            }
        }
    }
}
pub(crate) struct CpuPoolLease<T: CpuPoolValue> {
    value: Option<T>,
    shared: Arc<Shared<T>>,
    reusable: bool,
}
impl<T: CpuPoolValue> CpuPoolLease<T> {
    pub(crate) fn value(&self) -> &T {
        self.value.as_ref().expect("active CPU storage lease")
    }
    pub(crate) fn value_mut(&mut self) -> &mut T {
        self.value.as_mut().expect("active CPU storage lease")
    }
    /// Admission and accounting surround capacity changes, never individual
    /// element operations. `peak_extra` includes any replacement block which
    /// coexists with the old allocation; the payload enforces its exact budget.
    pub(crate) fn update<R>(
        &mut self,
        peak_extra: usize,
        change: impl FnOnce(&mut T, usize) -> Result<R>,
    ) -> Result<R> {
        let mut state = self.shared.state.lock();
        let available = self
            .shared
            .limits
            .total_bytes
            .saturating_sub(state.stats.total_bytes());
        if peak_extra > available {
            state.stats.capacity_stalls += 1;
            return Err((self.shared.limits.full)());
        }
        let value = self.value.as_mut().expect("active CPU storage lease");
        let previous = value.capacity_bytes();
        let result = change(value, available);
        let actual = value.capacity_bytes();
        state.stats.live_bytes = state.stats.live_bytes - previous + actual;
        state.stats.capacity_growths += u64::from(actual > previous);
        state.stats.peak_total_bytes = state.stats.peak_total_bytes.max(state.stats.total_bytes());
        if state.stats.total_bytes() > self.shared.limits.total_bytes {
            self.reusable = false;
            state.stats.capacity_stalls += 1;
            return Err((self.shared.limits.full)());
        }
        result
    }
    pub(crate) fn discard_on_return(&mut self) {
        self.reusable = false;
    }
}
impl<T: CpuPoolValue> Drop for CpuPoolLease<T> {
    fn drop(&mut self) {
        let Some(value) = self.value.take() else {
            return;
        };
        let available = value.available();
        let mut returned = Returned {
            value: Some(value),
            shared: &self.shared,
            available,
            reusable: false,
        };
        // Dropping list elements may run arbitrary resource destructors. Do
        // that before taking the recycler lock, including reentrant returns.
        // The guard also credits the checkout when an element destructor
        // unwinds; partially cleared storage is then discarded, never cached.
        if available {
            returned
                .value
                .as_mut()
                .expect("returned CPU storage")
                .reset();
        }
        returned.reusable = self.reusable;
    }
}
struct Returned<'a, T: CpuPoolValue> {
    value: Option<T>,
    shared: &'a Shared<T>,
    available: bool,
    reusable: bool,
}
impl<T: CpuPoolValue> Drop for Returned<'_, T> {
    fn drop(&mut self) {
        let Some(mut value) = self.value.take() else {
            return;
        };
        let mut state = self.shared.state.lock();
        state.stats.checked_out -= 1;
        let bytes = value.capacity_bytes();
        if !self.available {
            // Published immutable clones own this storage. They must stay in
            // the live limit until exclusivity returns, even under pressure.
            state.stats.pinned += 1;
            state.idle.push(Entry {
                value,
                cached: false,
            });
            return;
        }
        value.record_return(&mut state.stats.payload);
        if self.reusable
            && state.stats.cached_bytes.saturating_add(bytes) <= self.shared.limits.cached_bytes
            && !matches!(
                self.shared.budget.budget_action(),
                BudgetAction::EvictCache | BudgetAction::Backpressure
            )
        {
            value.reclassify(BudgetCategory::Cache);
            state.stats.live_bytes -= bytes;
            state.stats.cached += 1;
            state.stats.cached_bytes += bytes;
            state.idle.push(Entry {
                value,
                cached: true,
            });
        } else {
            // Payload elements were reset before taking this lock. Release
            // their backing before another checkout can spend its local credit.
            drop(value);
            state.stats.live_bytes -= bytes;
        }
    }
}
