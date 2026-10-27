//! Reusable CPU scratch with real capacity accounting. Values are dropped at the
//! CPU scope boundary; the storage remains cached until pressure trims it. No
//! GPU completion controls these temporary values or the backing Vec allocation.
use crate::{Error, Result};
use crate::accounting::{AllocatorStats, BudgetCategory, CpuMemoryReservation};
use std::sync::Arc;

pub struct TempList<T> {
    // Free the real allocation before crediting its reservation.
    items: Vec<T>,
    memory: Option<CpuMemoryReservation>,
    budget: Arc<AllocatorStats>,
    max_items: usize,
    growths: u64,
}
impl<T> TempList<T> {
    pub fn new(max_items: usize, budget: Arc<AllocatorStats>) -> Result<Self> {
        if max_items == 0 {
            return Err(Error::invalid_usage("temporary list limit must be nonzero"));
        }
        Ok(Self {
            items: Vec::new(),
            memory: None,
            budget,
            max_items,
            growths: 0,
        })
    }
    /// Reserve before entering a queue call or other non-rollback operation.
    /// Growth allocates a new block first so OOM leaves the old list untouched.
    pub fn reserve(&mut self, needed: usize) -> Result<()> {
        self.reserve_capped(needed, usize::MAX)
    }
    pub(crate) fn reserve_capped(&mut self, needed: usize, max_new_bytes: usize) -> Result<()> {
        if needed > self.max_items {
            return Err(Error::exhausted("temporary list capacity exceeded"));
        }
        if let Some(memory) = &mut self.memory {
            memory.reclassify(BudgetCategory::Live);
        }
        if needed <= self.items.capacity() {
            return Ok(());
        }
        let capacity = needed
            .max(self.items.capacity().saturating_mul(2))
            .min(self.max_items);
        let bytes = std::alloc::Layout::array::<T>(capacity)
            .map_err(|_| Error::invalid_usage("temporary list layout overflows"))?
            .size();
        if bytes > max_new_bytes {
            return Err(Error::exhausted("temporary list capacity exceeded"));
        }
        let mut memory = self
            .budget
            .reserve_cpu_bytes(bytes as u64, BudgetCategory::Live)?;
        let mut items = Vec::new();
        items
            .try_reserve_exact(capacity)
            .map_err(|_| Error::OutOfHostMemory)?;
        let actual_bytes = items
            .capacity()
            .checked_mul(std::mem::size_of::<T>())
            .ok_or_else(|| Error::invalid_usage("temporary list capacity overflows"))?;
        if actual_bytes > max_new_bytes {
            return Err(Error::exhausted("temporary list capacity exceeded"));
        }
        memory.resize(actual_bytes as u64)?;
        items.append(&mut self.items);
        let old_items = std::mem::replace(&mut self.items, items);
        let old_memory = self.memory.replace(memory);
        drop(old_items);
        drop(old_memory);
        self.growths += 1;
        Ok(())
    }
    pub fn add(&mut self, item: T) -> Result<&T> {
        self.reserve(
            self.items
                .len()
                .checked_add(1)
                .ok_or_else(|| Error::invalid_usage("temporary list length overflows"))?,
        )?;
        self.items.push(item);
        Ok(self.items.last().expect("pushed one item"))
    }
    pub(crate) fn reclassify(&mut self, category: BudgetCategory) {
        if let Some(memory) = &mut self.memory { memory.reclassify(category); }
    }
    pub fn clear(&mut self) {
        self.items.clear();
        if let Some(memory) = &mut self.memory {
            memory.reclassify(BudgetCategory::Cache);
        }
    }
    pub(crate) fn capacity(&self) -> usize {
        self.items.capacity()
    }
    pub(crate) fn drain(&mut self) -> std::vec::Drain<'_, T> {
        self.items.drain(..)
    }
    pub(crate) fn retain(&mut self, keep: impl FnMut(&T) -> bool) {
        self.items.retain(keep);
        if self.items.is_empty()
            && let Some(memory) = &mut self.memory
        {
            memory.reclassify(BudgetCategory::Cache);
        }
    }
    pub fn as_slice(&self) -> &[T] {
        &self.items
    }
    pub(crate) fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.items.get_mut(index)
    }
    /// Capacity was admitted before an irreversible native operation. This
    /// insertion cannot grow storage or fail after that operation succeeds.
    pub(crate) fn add_reserved(&mut self, item: T) {
        assert!(self.items.len() < self.items.capacity() && self.items.len() < self.max_items);
        self.items.push(item);
    }
    /// Move the last item to its position in a list otherwise sorted by `key`
    /// (one binary search and memmove instead of a full re-sort).
    pub(crate) fn sift_last_by_key<K: Ord>(&mut self, mut key: impl FnMut(&T) -> K) {
        let Some(last) = self.items.last() else {
            return;
        };
        let wanted = key(last);
        let len = self.items.len();
        let at = self.items[..len - 1].partition_point(|item| key(item) < wanted);
        self.items[at..].rotate_right(1);
    }
    pub(crate) fn sort_dedup_by_key<K: Ord>(&mut self, mut key: impl FnMut(&T) -> K) {
        self.items.sort_unstable_by_key(&mut key);
        self.items.dedup_by(|a, b| key(a) == key(b));
    }
    /// A fallible builder over the same accounted capacity. Every push goes
    /// through add/reserve; success, error and panic all drop temporary values.
    pub(crate) fn with_entries<R>(&mut self, run: impl FnOnce(&mut Self) -> R) -> R {
        self.clear();
        struct Active<'a, T>(&'a mut TempList<T>);
        impl<T> Drop for Active<'_, T> {
            fn drop(&mut self) {
                self.0.clear();
            }
        }
        let active = Active(self);
        run(active.0)
    }
    pub fn capacity_bytes(&self) -> usize {
        self.items
            .capacity()
            .saturating_mul(std::mem::size_of::<T>())
    }
    /// Take cached storage without holding its mutex during resource gates,
    /// budget admission or driver work. Concurrent callers own separate lists.
    pub(crate) fn checkout(cache: &parking_lot::Mutex<Self>) -> CachedTempList<'_, T> {
        let mut cached = cache.lock();
        let empty = Self {
            items: Vec::new(), memory: None, budget: cached.budget.clone(),
            max_items: cached.max_items, growths: 0,
        };
        let list = std::mem::replace(&mut *cached, empty);
        CachedTempList { cache, list }
    }
    pub fn growth_count(&self) -> u64 {
        self.growths
    }
    pub fn trim_idle(&mut self) -> bool {
        if !self.items.is_empty() || self.items.capacity() == 0 {
            return false;
        }
        let old = std::mem::take(&mut self.items);
        drop(old);
        self.memory.take();
        true
    }
    /// The only Vec mutation escape is crate-private. The caller must reserve
    /// the maximum number of pushes first; scope exit (including panic) clears
    /// all CPU values while retaining the accounted backing for the next call.
    pub(crate) fn with_reserved<R>(
        &mut self,
        needed: usize,
        run: impl FnOnce(&mut Vec<T>) -> R,
    ) -> Result<R> {
        self.clear();
        if let Err(error) = self.reserve(needed) {
            self.clear();
            return Err(error);
        }
        struct Active<'a, T>(&'a mut TempList<T>);
        impl<T> Drop for Active<'_, T> {
            fn drop(&mut self) {
                self.0.clear();
            }
        }
        let active = Active(self);
        Ok(run(&mut active.0.items))
    }
}
pub(crate) struct CachedTempList<'a, T> {
    cache: &'a parking_lot::Mutex<TempList<T>>,
    list: TempList<T>,
}
impl<T> std::ops::Deref for CachedTempList<'_, T> {
    type Target = TempList<T>;
    fn deref(&self) -> &Self::Target { &self.list }
}
impl<T> std::ops::DerefMut for CachedTempList<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.list }
}
impl<T> Drop for CachedTempList<'_, T> {
    fn drop(&mut self) {
        self.list.clear();
        let mut cached = self.cache.lock();
        let growths = cached.growths + self.list.growths;
        if self.list.capacity() > cached.capacity() {
            std::mem::swap(&mut *cached, &mut self.list);
        }
        cached.growths = growths;
    }
}
#[cfg(test)]
mod tests;
