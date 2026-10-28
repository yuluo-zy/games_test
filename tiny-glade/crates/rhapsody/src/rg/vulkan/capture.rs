//! Bounded capture of recent submissions for diagnostics.
use ash::vk;
use crate::native::submit;
use std::sync::Arc;

/// `submissions` 捕获的上限：只保留最近的提交，避免不经 `begin_frame` 的长时间运行无界增长。
pub const SUBMISSION_CAPTURE_LIMIT: usize = 64;

pub struct GpuSubmissionCapture {
    pub queue: vk::Queue,
    pub batches: smallvec::SmallVec<[submit::PreparedSubmit; 1]>,
    pub result: i32,
    // Owned arrays are destroyed before their budget reservation is released.
    _memory: Option<crate::accounting::CpuMemoryReservation>,
}

impl GpuSubmissionCapture {
    pub(super) fn refresh_budget(
        &mut self,
        stats: &Arc<crate::accounting::AllocatorStats>,
    ) -> crate::Result<()> {
        let bytes = self
            .batches
            .iter()
            .map(submit::PreparedSubmit::heap_capacity_bytes)
            .sum::<usize>()
            + if self.batches.spilled() {
                self.batches.capacity() * std::mem::size_of::<submit::PreparedSubmit>()
            } else {
                0
            };
        if let Some(memory) = &mut self._memory {
            memory.resize(bytes as u64)?;
        } else if bytes != 0 {
            self._memory =
                Some(stats.reserve_cpu_bytes(bytes as u64, crate::accounting::BudgetCategory::Cache)?);
        }
        Ok(())
    }
    pub(super) fn try_new(
        queue: vk::Queue,
        prepared: &[submit::PreparedSubmit],
        extra_waits: usize,
        stats: &Arc<crate::accounting::AllocatorStats>,
    ) -> crate::Result<Self> {
        let outer_bytes = if prepared.len() > 1 {
            std::alloc::Layout::array::<submit::PreparedSubmit>(prepared.len())
                .map_err(|_| crate::Error::invalid_usage("submission capture size overflows"))?
                .size()
        } else {
            0
        };
        let bytes = prepared
            .iter()
            .enumerate()
            .try_fold(outer_bytes, |sum, (index, batch)| {
                sum.checked_add(batch.copied_heap_bytes(if index == 0 {
                    extra_waits
                } else {
                    0
                })?)
                .ok_or_else(|| crate::Error::invalid_usage("submission capture storage overflows"))
            })?;
        let mut memory = if bytes == 0 {
            None
        } else {
            Some(stats.reserve_cpu_bytes(bytes as u64, crate::accounting::BudgetCategory::Cache)?)
        };
        let mut batches = smallvec::SmallVec::<[submit::PreparedSubmit; 1]>::new();
        batches
            .try_reserve_exact(prepared.len())
            .map_err(|_| crate::Error::OutOfHostMemory)?;
        for (index, batch) in prepared.iter().enumerate() {
            batches.push(batch.try_clone_with_wait_capacity(if index == 0 {
                extra_waits
            } else {
                0
            })?);
        }
        if let Some(memory) = &mut memory {
            let actual = batches
                .iter()
                .map(submit::PreparedSubmit::heap_capacity_bytes)
                .sum::<usize>()
                + if batches.spilled() {
                    batches.capacity() * std::mem::size_of::<submit::PreparedSubmit>()
                } else {
                    0
                };
            memory.resize(actual as u64)?;
        }
        Ok(Self {
            queue,
            batches,
            result: 0,
            _memory: memory,
        })
    }
}
