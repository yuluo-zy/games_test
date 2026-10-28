//! Read-only statistics and counters.
use std::sync::Arc;
use super::VulkanGraphBackend;

impl VulkanGraphBackend {
    pub fn managed_projection_stats(&self) -> crate::rg::pass_bindings::ManagedProjectionStats {
        self.caches.projection().pool.stats()
    }

    pub fn transient_memory_plan(&self) -> &crate::rg::transient_memory::PhysicalMemoryPlan {
        self.active_alias
            .as_ref()
            .map_or(&self.prepared_transients.plan, |active| &*active.plan)
    }

    /// Alias-set members handed back while their GPU work may still run;
    /// they rejoin their set (and the set becomes reusable) once idle.
    pub fn transient_alias_retiring(&self) -> usize {
        self.caches.alias().retiring_count()
    }

    pub fn transient_backing_statistics(
        &self,
    ) -> crate::device::transient_backing::TransientBackingStatistics {
        self.transient_counters.snapshot()
    }

    pub fn transient_backing_counters(
        &self,
    ) -> Arc<crate::device::transient_backing::TransientBackingCounters> {
        self.transient_counters.clone()
    }

    pub fn cpu_arena_pool(&self) -> &crate::command::CpuArenaPool {
        &self.cpu_arenas
    }

    pub fn cpu_arena_statistics(&self) -> crate::command::CpuArenaStats {
        self.cpu_arenas.stats()
    }

    pub fn owner_storage_statistics(&self) -> crate::collections::temp_list_pool::TempListPoolStats {
        self.owner_pool.statistics()
    }

    pub fn frame_storage_statistics(&self) -> crate::rg::frame_storage::FrameStorageStats {
        self.frame_storage.stats()
    }

    pub fn command_pool_count(&self) -> usize {
        self.frame_pools.len()
    }

    pub fn command_pool_reset_count(&self) -> u64 {
        self.frame_pools
            .values()
            .map(|entry| entry.pool.reset_count())
            .sum()
    }

    pub fn poll_gpu_timings(&mut self) -> crate::Result<Option<crate::rg::profiling::GpuTimingStats>> {
        self.gpu_timing
            .as_mut()
            .map(|ring| {
                ring.poll()?;
                Ok(ring.stats())
            })
            .transpose()
    }

    pub fn gpu_timings(&self) -> impl Iterator<Item = &crate::rg::profiling::GpuCommandTiming> {
        self.gpu_timing.iter().flat_map(|ring| ring.samples())
    }

    pub fn drain_gpu_timings(
        &mut self,
    ) -> impl Iterator<Item = crate::rg::profiling::GpuCommandTiming> + '_ {
        self.gpu_timing
            .iter_mut()
            .flat_map(|ring| ring.drain_samples())
    }
}
