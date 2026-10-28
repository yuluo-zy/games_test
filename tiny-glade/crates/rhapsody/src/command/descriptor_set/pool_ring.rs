//! Growable descriptor allocation for non-bindless per-frame/per-material sets.
//!
//! A single `VkDescriptorPool` has a fixed `maxSets`. [`DescriptorPoolRing`]
//! keeps a bounded list of equally sized pools: allocation uses the current
//! pool and moves to an idle or new pool once it is exhausted (either by the
//! ring's own `maxSets` accounting or a driver `OUT_OF_POOL_MEMORY` /
//! `FRAGMENTED_POOL`). Transient rings reset whole pools once every set
//! allocated from them is dropped and its GPU uses have completed; persistent
//! rings free sets individually and reuse any pool with spare capacity.
use crate::command::descriptor_set::{LayoutPlan, VulkanDescriptorPool, VulkanDescriptorSet, VulkanDescriptorSetLayout};
use crate::{Error, Result, device::Device};
use parking_lot::Mutex;
use std::sync::{Arc, atomic::{AtomicU64, Ordering}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DescriptorPoolRingConfig {
    /// `maxSets` (and per-type descriptor multiplier) of each pool.
    pub sets_per_pool: u32,
    /// Upper bound on pools; allocation beyond it returns `Exhausted`.
    pub max_pools: usize,
    /// Reset whole pools (per-frame sets) instead of freeing sets individually.
    pub transient: bool,
}
impl Default for DescriptorPoolRingConfig {
    fn default() -> Self {
        Self {
            sets_per_pool: 64,
            max_pools: 64,
            transient: true,
        }
    }
}

struct RingPool {
    pool: Arc<VulkanDescriptorPool>,
    /// Sets allocated since creation/last reset (transient) — a transient pool
    /// never reuses the space of dropped sets before a whole-pool reset.
    allocated: u32,
    /// The driver reported pool exhaustion; skip until reset (transient) or
    /// until retired sets are reclaimed (persistent).
    full: bool,
    /// Allocation clock of the last set taken from this pool (LRU eviction).
    used: u64,
}

struct PoolCreation<'a> {
    state: &'a Mutex<RingState>,
    committed: bool,
}
impl Drop for PoolCreation<'_> {
    fn drop(&mut self) {
        if !self.committed { self.state.lock().creating -= 1; }
    }
}

/// Driver memory of one descriptor set, for eviction progress accounting.
/// Descriptor pools live in driver memory outside the device-memory ledger.
const DESCRIPTOR_SET_BYTES_ESTIMATE: u64 = 256;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DescriptorPoolRingStats {
    pub pools: usize,
    pub resets: u64,
    pub created: u64,
}

pub struct DescriptorPoolRing {
    device: Arc<Device>,
    plan: LayoutPlan,
    config: DescriptorPoolRingConfig,
    state: Mutex<RingState>,
    clock: AtomicU64,
    resets: AtomicU64,
}
struct RingState {
    pools: Vec<Arc<Mutex<RingPool>>>,
    creating: usize,
    current: usize,
    stats: DescriptorPoolRingStats,
}

impl DescriptorPoolRing {
    pub fn new(
        device: Arc<Device>,
        plan: &LayoutPlan,
        config: DescriptorPoolRingConfig,
    ) -> Result<Self> {
        if config.sets_per_pool == 0 || config.max_pools == 0 {
            return Err(Error::invalid_usage(
                "descriptor pool ring needs nonzero pool size and pool count",
            ));
        }
        Ok(Self {
            device,
            plan: plan.clone(),
            config,
            clock: AtomicU64::new(0),
            resets: AtomicU64::new(0),
            state: Mutex::new(RingState {
                pools: Vec::new(),
                current: 0,
                creating: 0,
                stats: DescriptorPoolRingStats::default(),
            }),
        })
    }

    /// Let the device release this ring's idle pools under memory pressure.
    pub fn register_eviction(self: &Arc<Self>) {
        let evictable: Arc<dyn crate::accounting::Evictable> = self.clone();
        self.device.register_evictable(
            Arc::downgrade(&evictable),
            crate::accounting::EvictionPriority::Normal,
        );
    }

    pub fn stats(&self) -> DescriptorPoolRingStats {
        let state = self.state.lock();
        DescriptorPoolRingStats {
            pools: state.pools.len(),
            resets: self.resets.load(Ordering::Relaxed),
            ..state.stats
        }
    }

    /// Allocate one set, growing the ring when every usable pool is exhausted.
    pub fn allocate(&self, layout: &Arc<VulkanDescriptorSetLayout>) -> Result<VulkanDescriptorSet> {
        if !Arc::ptr_eq(&layout.device, &self.device) || layout.plan != self.plan {
            return Err(Error::invalid_usage(
                "descriptor layout does not match the pool ring plan",
            ));
        }
        let (count, start) = { let state = self.state.lock(); (state.pools.len(), state.current) };
        // Reclamation can enter other device caches. No ring or pool lock is
        // held while it runs; each native pool is leased independently below.
        let mut reclaimed = false;
        for step in 0..count {
            let index = (start + step) % count;
            let entry = self.state.lock().pools.get(index).cloned();
            let Some(entry) = entry else { continue; };
            let Some(mut leased) = entry.try_lock() else { continue; };
            if !self.config.transient && leased.full {
                if !reclaimed {
                    drop(leased);
                    self.device.try_collect_garbage()?;
                    reclaimed = true;
                    let Some(retry) = entry.try_lock() else { continue; };
                    leased = retry;
                }
                leased.full = false;
            }
            if !self.make_usable(&mut leased)? { continue; }
            if let Some(set) = self.try_allocate(&mut leased, layout)? {
                leased.used = self.clock.fetch_add(1, Ordering::Relaxed);
                drop(leased);
                self.state.lock().current = index;
                return Ok(set);
            }
        }
        {
            let mut state = self.state.lock();
            if state.pools.len() + state.creating >= self.config.max_pools {
                return Err(Error::exhausted("descriptor pool ring reached its pool limit"));
            }
            let reservations = state.creating + 1;
            state.pools.try_reserve(reservations).map_err(|_| Error::OutOfHostMemory)?;
            state.creating += 1;
        }
        let mut creating = PoolCreation { state: &self.state, committed: false };
        let pool = if self.config.transient {
            VulkanDescriptorPool::new_transient(self.device.clone(), &self.plan, self.config.sets_per_pool)?
        } else {
            VulkanDescriptorPool::new(self.device.clone(), &self.plan, self.config.sets_per_pool)?
        };
        let mut entry = RingPool {
            pool, allocated: 0, full: false,
            used: self.clock.fetch_add(1, Ordering::Relaxed),
        };
        let set = self.try_allocate(&mut entry, layout)?.ok_or_else(|| {
            Error::exhausted("a new descriptor pool cannot hold one set of its plan")
        })?;
        let entry = Arc::new(Mutex::new(entry));
        let mut state = self.state.lock();
        state.creating -= 1;
        creating.committed = true;
        state.current = state.pools.len();
        state.pools.push(entry);
        state.stats.created += 1;
        Ok(set)
    }

    /// Reset idle exhausted transient pools independently of the ring metadata.
    pub fn recycle(&self) -> Result<usize> {
        let count = self.state.lock().pools.len();
        let mut recycled = 0;
        for index in 0..count {
            let entry = self.state.lock().pools.get(index).cloned();
            let Some(entry) = entry else { continue; };
            let Some(mut entry) = entry.try_lock() else { continue; };
            if (entry.allocated != 0 || entry.full) && self.reset_if_idle(&mut entry)? {
                recycled += 1;
            }
        }
        Ok(recycled)
    }

    /// Returns whether `entry` can be tried for an allocation.
    fn make_usable(
        &self,
        entry: &mut RingPool,
    ) -> Result<bool> {
        if self.config.transient {
            if entry.full || entry.allocated >= self.config.sets_per_pool {
                return self.reset_if_idle(entry);
            }
            return Ok(true);
        }
        Ok(!entry.full && entry.pool.live_set_count() < self.config.sets_per_pool as usize)
    }

    fn reset_if_idle(
        &self,
        entry: &mut RingPool,
    ) -> Result<bool> {
        if entry.pool.live_set_count() != 0 {
            return Ok(false);
        }
        match entry.pool.reset() {
            Ok(()) => {
                entry.allocated = 0;
                entry.full = false;
                self.resets.fetch_add(1, Ordering::Relaxed);
                Ok(true)
            }
            // GPU work still references sets from this pool.
            Err(Error::Vulkan(ash::vk::Result::NOT_READY)) => Ok(false),
            Err(error) => Err(error),
        }
    }

    fn try_allocate(
        &self,
        entry: &mut RingPool,
        layout: &Arc<VulkanDescriptorSetLayout>,
    ) -> Result<Option<VulkanDescriptorSet>> {
        match entry.pool.allocate(layout.clone()) {
            Ok(set) => {
                entry.allocated += 1;
                Ok(Some(set))
            }
            Err(Error::Exhausted(_)) => {
                entry.full = true;
                Ok(None)
            }
            Err(error) => Err(error),
        }
    }
}

/// Idle pools (no live set, every GPU use completed) are destroyed in LRU
/// order; the ring recreates pools on demand.
impl crate::accounting::Evictable for DescriptorPoolRing {
    fn evict_idle(&self, completed: &crate::sync::CompletedValues, target_bytes: u64) -> u64 {
        let per_pool = u64::from(self.config.sets_per_pool) * DESCRIPTOR_SET_BYTES_ESTIMATE;
        let mut released = 0;
        while released < target_bytes {
            let removed = {
                let Some(mut state) = self.state.try_lock() else { break; };
                let victim = state.pools.iter().enumerate().filter_map(|(index, entry)| {
                    // A checked-out Arc pins this entry until its allocation
                    // finishes; eviction never waits for its native pool lock.
                    if Arc::strong_count(entry) != 1 { return None; }
                    let entry = entry.try_lock()?;
                    (entry.pool.live_set_count() == 0 && entry.pool.last_use.is_reached(completed))
                        .then_some((index, entry.used))
                }).min_by_key(|(_, used)| *used).map(|(index, _)| index);
                let Some(index) = victim else { break; };
                let removed = state.pools.remove(index);
                if state.current >= index && state.current > 0 { state.current -= 1; }
                Some(removed)
            };
            // Native retirement may enter the device budget controller.
            drop(removed);
            released += per_pool;
        }
        released
    }
}
