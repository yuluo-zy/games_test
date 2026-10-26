//! [`Memory`]: allocation accounting, the optional suballocator, driver
//! budgets and the eviction registry of one device.
//!
//! Locks owned here (suballocator shards, accounting tables, eviction list)
//! are leaves: no other device lock is taken while one is held. Eviction
//! callbacks run after the registry lock is released.
use super::Device;
use ash::vk;
use std::sync::Arc;

use crate::raw::allocator::AllocatorStats;

pub(crate) struct Memory {
    /// Shared with every allocation for accounting and budgets.
    pub(crate) stats: Arc<AllocatorStats>,
    /// gpu-allocator suballocation; `None` means one dedicated allocation per resource.
    #[cfg(feature = "suballocation")]
    pub(crate) suballocator: Option<Arc<crate::raw::suballoc::SubAllocator>>,
    /// Whether `VK_EXT_memory_budget` was enabled at creation.
    pub(crate) budget_extension: bool,
    /// Raw driver telemetry is distinct from allocator policy limits and is
    /// available even when no optional PhysicalDevice wrapper was attached.
    driver_budget: parking_lot::RwLock<Option<crate::raw::physical_device::MemoryBudgetSnapshot>>,
    /// When the driver budget was last queried by `refresh_memory_budget_if_due`.
    pub(crate) budget_refreshed_at: parking_lot::Mutex<Option<std::time::Instant>>,
    /// Caches asked to release idle entries under memory pressure.
    pub(crate) evictables: crate::device::EvictionRegistry,
}

impl Memory {
    /// Prepare allocation accounting with the configured budgets; no Vulkan call.
    pub(crate) fn new_stats(
        runtime: &crate::device::init::RuntimeConfig,
    ) -> crate::Result<Arc<AllocatorStats>> {
        let stats = Arc::new(AllocatorStats::default());
        stats.set_device_budget(runtime.memory_budget_bytes)?;
        stats.set_host_budget(runtime.host_memory_budget_bytes)?;
        Ok(stats)
    }

    /// Create the allocation backend selected by the runtime policy.
    pub(crate) fn create(
        core: &super::native::DeviceCore,
        stats: Arc<AllocatorStats>,
        runtime: &crate::device::init::RuntimeConfig,
    ) -> crate::Result<Self> {
        #[cfg(feature = "suballocation")]
        let suballocator = match runtime.policy.allocation {
            crate::device::AllocationStrategy::Dedicated => None,
            crate::device::AllocationStrategy::Suballocated => {
                Some(Arc::new(crate::raw::suballoc::SubAllocator::new(
                    &core.instance.instance,
                    &core.device,
                    core.physical,
                    core.capabilities.buffer_device_address,
                    stats.clone(),
                )?))
            }
        };
        #[cfg(not(feature = "suballocation"))]
        let _ = runtime;
        stats.set_max_native_objects(u64::from(
            core.properties.limits.max_memory_allocation_count,
        ));
        Ok(Self {
            stats,
            #[cfg(feature = "suballocation")]
            suballocator,
            budget_extension: core.has_extension("VK_EXT_memory_budget"),
            budget_refreshed_at: parking_lot::Mutex::new(None),
            driver_budget: parking_lot::RwLock::new(None),
            evictables: Default::default(),
        })
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        // Native memory blocks must be freed before the VkDevice.
        #[cfg(feature = "suballocation")]
        if let Some(sub) = self.suballocator.take() {
            sub.shutdown();
        }
    }
}
/// Known application VkDeviceMemory only. Driver-internal pipeline/pool memory
/// and swapchain image allocations are not visible here.
#[derive(Clone, Copy, Debug, Default, serde::Serialize)]
pub struct GpuMemoryUsage {
    pub resource_range_bytes: u64,
    pub suballocated_used_bytes: u64,
    pub suballocator_reserved_bytes: u64,
    pub suballocator_unused_bytes: u64,
    /// Observed native reservation peak, including an allocation rolled back
    /// when the public allocator exposes its actual native delta only afterward.
    pub suballocator_peak_reserved_bytes: u64,
    pub dedicated_bytes: u64,
    pub application_vk_memory_bytes: u64,
    pub native_memory_blocks: usize,
}
impl Device {
    pub fn trim_unused_suballocations(&self) -> u64 {
        #[cfg(feature = "suballocation")]
        return self
            .memory
            .suballocator
            .as_ref()
            .map_or(0, |sub| sub.trim_unused());
        #[cfg(not(feature = "suballocation"))]
        0
    }
    /// Take at a frame boundary without concurrent allocation/free for a
    /// coherent combined snapshot. Released suballocation ranges can leave
    /// whole native blocks reserved; this is separate from range accounting.
    pub fn memory_usage_snapshot(&self) -> GpuMemoryUsage {
        #[cfg(feature = "suballocation")]
        let (used, reserved) = self.memory.suballocator.as_ref().map_or((0, 0), |sub| {
            let (used, reserved, _, _) = sub.memory_usage();
            (used, reserved)
        });
        #[cfg(not(feature = "suballocation"))]
        let (used, reserved) = (0u64, 0u64);
        let resource_range_bytes = self.memory.stats.totals().total_allocated_bytes;
        let dedicated_bytes = resource_range_bytes.saturating_sub(used);
        GpuMemoryUsage {
            resource_range_bytes,
            suballocated_used_bytes: used,
            suballocator_reserved_bytes: reserved,
            suballocator_unused_bytes: reserved.saturating_sub(used),
            suballocator_peak_reserved_bytes: {
                #[cfg(feature = "suballocation")]
                {
                    self.memory
                        .suballocator
                        .as_ref()
                        .map_or(0, |sub| sub.physical_peak_bytes())
                }
                #[cfg(not(feature = "suballocation"))]
                {
                    0
                }
            },
            dedicated_bytes,
            application_vk_memory_bytes: dedicated_bytes.saturating_add(reserved),
            // Dedicated objects plus suballocator blocks.
            native_memory_blocks: usize::try_from(self.memory.stats.native_object_count())
                .unwrap_or(usize::MAX),
        }
    }
}

impl Device {
    /// Query `VK_EXT_memory_budget` and apply the per-heap budgets to the
    /// allocator (driver budget minus memory this device did not allocate).
    /// Call once per frame or maintenance tick. Returns `Ok(None)` when the
    /// extension is not enabled.
    pub fn refresh_memory_budget(
        &self,
    ) -> crate::Result<Option<vk::PhysicalDeviceMemoryBudgetPropertiesEXT<'static>>> {
        if !self.memory.budget_extension {
            return Ok(None);
        }
        let mut budget = vk::PhysicalDeviceMemoryBudgetPropertiesEXT::default();
        let mut properties = vk::PhysicalDeviceMemoryProperties2::default().push_next(&mut budget);
        unsafe {
            self.core
                .instance
                .instance
                .get_physical_device_memory_properties2(self.core.physical, &mut properties);
        }
        let heaps =
            (self.core.memory_properties.memory_heap_count as usize).min(vk::MAX_MEMORY_HEAPS);
        let mut snapshot = [(0u64, 0u64); vk::MAX_MEMORY_HEAPS];
        for (index, entry) in snapshot.iter_mut().enumerate().take(heaps) {
            *entry = (budget.heap_budget[index], budget.heap_usage[index]);
        }
        self.memory
            .stats
            .apply_driver_heap_budgets(&snapshot[..heaps]);
        let budget = vk::PhysicalDeviceMemoryBudgetPropertiesEXT {
            heap_budget: budget.heap_budget,
            heap_usage: budget.heap_usage,
            ..Default::default()
        };
        *self.memory.driver_budget.write() = Some(crate::raw::physical_device::MemoryBudgetSnapshot::from_vk(&budget));
        self.core.physical_info.publish_memory_budget(&budget);
        Ok(Some(budget))
    }
    /// Minimum time between driver budget queries on the frame path.
    pub const MEMORY_BUDGET_MIN_INTERVAL: std::time::Duration =
        std::time::Duration::from_millis(250);
    /// Like [`Self::refresh_memory_budget`], but queries the driver only when the
    /// last query is older than [`Self::MEMORY_BUDGET_MIN_INTERVAL`] or the
    /// allocator is already under budget pressure. Returns `Ok(None)` when it
    /// skipped the query or the extension is not enabled. Teardown and
    /// callers that need a fresh value keep using the forced version.
    pub fn refresh_memory_budget_if_due(
        &self,
    ) -> crate::Result<Option<vk::PhysicalDeviceMemoryBudgetPropertiesEXT<'static>>> {
        if !self.memory.budget_extension {
            return Ok(None);
        }
        let pressure = self.memory.stats.budget_action() != crate::device::BudgetAction::None;
        let queried_at = {
            let mut last = self.memory.budget_refreshed_at.lock();
            let now = std::time::Instant::now();
            if !pressure
                && last.is_some_and(|at| now.duration_since(at) < Self::MEMORY_BUDGET_MIN_INTERVAL)
            {
                return Ok(None);
            }
            *last = Some(now);
            now
        };
        let result = self.refresh_memory_budget();
        if result.is_err() {
            let mut last = self.memory.budget_refreshed_at.lock();
            if *last == Some(queried_at) { *last = None; }
        }
        result
    }
    /// Last actual driver query, without a Vulkan call. None means unavailable
    /// telemetry, never a zero-byte budget or the allocator's configured cap.
    pub fn driver_memory_budget_snapshot(&self) -> Option<crate::raw::physical_device::MemoryBudgetSnapshot> {
        *self.memory.driver_budget.read()
    }
    /// Whether `VK_EXT_memory_budget` was enabled at creation.
    pub fn memory_budget_enabled(&self) -> bool {
        self.memory.budget_extension
    }
    /// Allocation accounting and budgets shared by every resource of this device.
    pub fn allocator_stats(&self) -> &Arc<AllocatorStats> {
        &self.memory.stats
    }
    /// Run `body` with the device allocator statistics.
    pub fn with_global_allocator<R>(&self, body: impl FnOnce(&AllocatorStats) -> R) -> R {
        body(&self.memory.stats)
    }
    pub fn suballocation_enabled(&self) -> bool {
        #[cfg(feature = "suballocation")]
        return self.memory.suballocator.is_some();
        #[cfg(not(feature = "suballocation"))]
        false
    }
    /// The suballocator, when the allocation policy selected one.
    #[cfg(feature = "suballocation")]
    pub fn suballocator(&self) -> Option<&Arc<crate::raw::suballoc::SubAllocator>> {
        self.memory.suballocator.as_ref()
    }
    /// Caches registered for eviction under memory pressure.
    pub fn evictables(&self) -> &crate::device::EvictionRegistry {
        &self.memory.evictables
    }
    /// Register a cache whose idle entries the allocator may release under
    /// memory pressure. The device keeps only the weak reference.
    pub fn register_evictable(
        &self,
        cache: std::sync::Weak<dyn crate::device::Evictable>,
        priority: crate::device::EvictionPriority,
    ) {
        self.memory.evictables.register(cache, priority);
    }
    /// Ask registered caches to release idle (CPU-unreferenced, GPU-complete)
    /// entries until about `target_bytes` are freed, then destroy what became
    /// collectable. Returns the bytes the caches reported released.
    pub fn evict_idle_caches(&self, target_bytes: u64) -> u64 {
        let mut reclaim = crate::raw::allocator::AllocationReclaim::new(self);
        self.evict_idle_caches_reclaim(target_bytes, &mut reclaim)
    }
    pub(crate) fn evict_idle_caches_reclaim(
        &self, target_bytes: u64, reclaim: &mut crate::raw::allocator::AllocationReclaim,
    ) -> u64 {
        let completed = reclaim.completed.get_or_insert_with(|| match self.poll() {
            Ok(completed) => Some(completed),
            Err(crate::Error::DeviceLost) => Some(crate::sync::CompletedValues::all()),
            Err(error) => { log::error!("idle cache eviction completion query failed: {error}"); None }
        });
        let Some(completed) = completed.as_ref() else { return 0 };
        let released = self.memory.evictables.evict(completed, target_bytes);
        // Caches can drop their last physical owner without reporting its
        // native bytes. Collect with the SAME finite allowance even when
        // reported bytes are zero, including when no cache is registered.
        self.collect_pressure_snapshot(completed, &mut reclaim.remaining);
        let _ = self.trim_unused_suballocations();
        released
    }
}


#[cfg(test)]
mod pressure_reclaim_tests {
    use super::*;
    #[test]
    fn one_allocation_shares_its_quota_and_snapshot_even_when_eviction_reports_zero() {
        let budget = crate::device::ReclamationBudget { max_scans: 3, max_destroys: 2 };
        let device = match Device::new_headless(&crate::device::DeviceConfig {
            reclamation: crate::device::ReclamationConfig { normal: budget, pressure: budget },
            ..crate::device::DeviceConfig::headless()
        }) {
            Ok(device) => device,
            Err(error) => {
                assert!(!std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|v| v == "1"), "{error}");
                return;
            }
        };
        for _ in 0..9 {
            let owners = crate::raw::temp_list::TempList::new(1, device.allocator_stats().clone()).unwrap();
            assert!(device.lifetime.retirement.try_enqueue(crate::sync::SyncSet::EMPTY,
                crate::raw::graveyard::Doomed::Owners(owners)).is_ok());
        }
        let mut reclaim = crate::raw::allocator::AllocationReclaim::new(&device);
        for _ in 0..4 {
            assert_eq!(device.evict_idle_caches_reclaim(u64::MAX, &mut reclaim), 0);
        }
        assert!(reclaim.completed.as_ref().unwrap().is_some());
        assert_eq!(reclaim.remaining.max_destroys, 0);
        assert_eq!(device.retirement_stats().pending, 7, "one attempt never gets a fresh quota");
        while device.retirement_stats().pending > 0 {
            device.evict_idle_caches(u64::MAX);
        }
        assert_eq!(device.retirement_stats().idle_waits, 0);
    }
}
