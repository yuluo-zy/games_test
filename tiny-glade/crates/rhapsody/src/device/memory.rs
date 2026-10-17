use super::Device;
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
            .suballocator
            .as_ref()
            .map_or(0, |sub| sub.trim_unused());
        #[cfg(not(feature = "suballocation"))]
        0
    }
    pub fn memory_block_count(&self) -> usize {
        self.memory_usage_snapshot().native_memory_blocks
    }
    /// Take at a frame boundary without concurrent allocation/free for a
    /// coherent combined snapshot. Released suballocation ranges can leave
    /// whole native blocks reserved; this is separate from range accounting.
    pub fn memory_usage_snapshot(&self) -> GpuMemoryUsage {
        #[cfg(feature = "suballocation")]
        let (used, reserved, allocations, blocks) = self
            .suballocator
            .as_ref()
            .map_or((0, 0, 0, 0), |sub| sub.memory_usage());
        #[cfg(not(feature = "suballocation"))]
        let (used, reserved, allocations, blocks) = (0u64, 0u64, 0usize, 0usize);
        let resources = self.allocator_stats.report();
        let resource_range_bytes = resources.totals.total_allocated_bytes;
        let dedicated_bytes = resource_range_bytes.saturating_sub(used);
        GpuMemoryUsage {
            resource_range_bytes,
            suballocated_used_bytes: used,
            suballocator_reserved_bytes: reserved,
            suballocator_unused_bytes: reserved.saturating_sub(used),
            suballocator_peak_reserved_bytes: {
                #[cfg(feature = "suballocation")]
                {
                    self.suballocator
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
            native_memory_blocks: resources
                .allocations
                .len()
                .saturating_sub(allocations)
                .saturating_add(blocks),
        }
    }
}
