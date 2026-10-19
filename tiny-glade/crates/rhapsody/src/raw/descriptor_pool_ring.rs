//! Growable descriptor allocation for non-bindless per-frame/per-material sets.
//!
//! A single `VkDescriptorPool` has a fixed `maxSets`. [`DescriptorPoolRing`]
//! keeps a bounded list of equally sized pools: allocation uses the current
//! pool and moves to an idle or new pool once it is exhausted (either by the
//! ring's own `maxSets` accounting or a driver `OUT_OF_POOL_MEMORY` /
//! `FRAGMENTED_POOL`). Transient rings reset whole pools once every set
//! allocated from them is dropped and its GPU uses have completed; persistent
//! rings free sets individually and reuse any pool with spare capacity.
use super::{LayoutPlan, VulkanDescriptorPool, VulkanDescriptorSet, VulkanDescriptorSetLayout};
use crate::{Error, Result, device::Device};
use parking_lot::Mutex;
use std::sync::Arc;

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
}
struct RingState {
    pools: Vec<RingPool>,
    current: usize,
    clock: u64,
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
            state: Mutex::new(RingState {
                pools: Vec::new(),
                current: 0,
                clock: 0,
                stats: DescriptorPoolRingStats::default(),
            }),
        })
    }

    /// Let the device release this ring's idle pools under memory pressure.
    pub fn register_eviction(self: &Arc<Self>) {
        let evictable: Arc<dyn crate::device::Evictable> = self.clone();
        self.device.register_evictable(
            Arc::downgrade(&evictable),
            crate::device::EvictionPriority::Normal,
        );
    }

    pub fn stats(&self) -> DescriptorPoolRingStats {
        let state = self.state.lock();
        DescriptorPoolRingStats {
            pools: state.pools.len(),
            ..state.stats
        }
    }

    /// Allocate one set, growing the ring when every usable pool is exhausted.
    pub fn allocate(&self, layout: Arc<VulkanDescriptorSetLayout>) -> Result<VulkanDescriptorSet> {
        if !Arc::ptr_eq(&layout.device, &self.device) || layout.plan != self.plan {
            return Err(Error::invalid_usage(
                "descriptor layout does not match the pool ring plan",
            ));
        }
        let mut state = self.state.lock();
        if !self.config.transient && state.pools.iter().any(|entry| entry.full) {
            // Dropped persistent sets are freed through device retirement;
            // reclaim completed ones so the driver sees their space again.
            self.device.try_collect_garbage()?;
            for entry in &mut state.pools {
                entry.full = false;
            }
        }
        let count = state.pools.len();
        // Visit every existing pool once, starting from the current one.
        for step in 0..count {
            let index = (state.current + step) % count;
            let RingState { pools, stats, .. } = &mut *state;
            if !self.make_usable(&mut pools[index], stats)? {
                continue;
            }
            match self.try_allocate(&mut state.pools[index], &layout)? {
                Some(set) => {
                    state.current = index;
                    state.clock += 1;
                    state.pools[index].used = state.clock;
                    return Ok(set);
                }
                None => continue,
            }
        }
        if count >= self.config.max_pools {
            return Err(Error::exhausted(
                "descriptor pool ring reached its pool limit",
            ));
        }
        let pool = if self.config.transient {
            VulkanDescriptorPool::new_transient(
                self.device.clone(),
                &self.plan,
                self.config.sets_per_pool,
            )?
        } else {
            VulkanDescriptorPool::new(self.device.clone(), &self.plan, self.config.sets_per_pool)?
        };
        state.clock += 1;
        let used = state.clock;
        state.pools.push(RingPool {
            pool,
            allocated: 0,
            full: false,
            used,
        });
        state.stats.created += 1;
        state.current = count;
        let entry = &mut state.pools[count];
        self.try_allocate(entry, &layout)?.ok_or_else(|| {
            Error::exhausted("a new descriptor pool cannot hold one set of its plan")
        })
    }

    /// Reset every idle exhausted transient pool now (normally done lazily).
    pub fn recycle(&self) -> Result<usize> {
        let mut state = self.state.lock();
        let RingState { pools, stats, .. } = &mut *state;
        let mut recycled = 0;
        for entry in pools.iter_mut() {
            if entry.allocated != 0 && self.reset_if_idle(entry, stats)? {
                recycled += 1;
            }
        }
        Ok(recycled)
    }

    /// Returns whether `entry` can be tried for an allocation.
    fn make_usable(
        &self,
        entry: &mut RingPool,
        stats: &mut DescriptorPoolRingStats,
    ) -> Result<bool> {
        if self.config.transient {
            if entry.full || entry.allocated >= self.config.sets_per_pool {
                return self.reset_if_idle(entry, stats);
            }
            return Ok(true);
        }
        Ok(!entry.full && entry.pool.live_set_count() < self.config.sets_per_pool as usize)
    }

    fn reset_if_idle(
        &self,
        entry: &mut RingPool,
        stats: &mut DescriptorPoolRingStats,
    ) -> Result<bool> {
        if entry.pool.live_set_count() != 0 {
            return Ok(false);
        }
        match entry.pool.reset() {
            Ok(()) => {
                entry.allocated = 0;
                entry.full = false;
                stats.resets += 1;
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
impl crate::device::Evictable for DescriptorPoolRing {
    fn evict_idle(&self, completed: &crate::sync::CompletedValues, target_bytes: u64) -> u64 {
        let Some(mut state) = self.state.try_lock() else {
            return 0;
        };
        let per_pool = u64::from(self.config.sets_per_pool) * DESCRIPTOR_SET_BYTES_ESTIMATE;
        let mut released = 0;
        while released < target_bytes {
            let victim = state
                .pools
                .iter()
                .enumerate()
                .filter(|(_, entry)| {
                    entry.pool.live_set_count() == 0 && entry.pool.last_use.is_reached(completed)
                })
                .min_by_key(|(_, entry)| entry.used)
                .map(|(index, _)| index);
            let Some(index) = victim else {
                break;
            };
            // Dropping retires the pool after its (already reached) last use.
            state.pools.remove(index);
            if state.current >= index && state.current > 0 {
                state.current -= 1;
            }
            released += per_pool;
        }
        if state.current >= state.pools.len() {
            state.current = 0;
        }
        released
    }
}
