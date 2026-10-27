//! Allocation totals and allocator reports.
use super::AllocationTotals;
use super::AllocatorReport;
use super::AllocatorStats;

impl AllocatorStats {
    pub fn report(&self) -> AllocatorReport {
        self.state.lock().report()
    }

    /// Current totals without copying every allocation record.
    pub fn totals(&self) -> AllocationTotals {
        self.state.lock().totals
    }

    pub fn peak_allocator_report(&self) -> Option<AllocatorReport> {
        let mut state = self.state.lock();
        state.materialize_peak();
        state.peak_report.clone()
    }

    /// Cumulative owned allocation ranges. A suballocator can place many ranges
    /// in one VkDeviceMemory; native block counts are reported separately.
    pub fn total_registered(&self) -> u64 {
        self.state.lock().next_id
    }

    pub fn peak_totals(&self) -> AllocationTotals {
        self.state.lock().peak
    }
}
