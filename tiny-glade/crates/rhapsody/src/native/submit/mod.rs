//! Owned Vulkan submission arrays. Completion is managed by device::submission
//! and sync::timeline; this module does not allocate completion values.
use ash::vk;

/// Common submissions retain Copy Vulkan arrays inline; larger input remains
/// owned and uses the existing fallible pre-submit reservations.
pub type SubmitArray<T> = smallvec::SmallVec<[T; 8]>;

/// Submission-local owners and guards stay inline for common graphs. A spill
/// is reserved fallibly before collecting values (and before taking any gates).
/// The allocation is freed before its CPU budget credit is returned.
pub(crate) struct SubmitScratch<A: smallvec::Array> {
    items: smallvec::SmallVec<A>,
    _memory: Option<crate::accounting::CpuMemoryReservation>,
}
impl<A: smallvec::Array> SubmitScratch<A> {
    pub(crate) fn try_collect(
        count: usize,
        stats: &std::sync::Arc<crate::accounting::AllocatorStats>,
        values: impl IntoIterator<Item = crate::Result<A::Item>>,
    ) -> crate::Result<Self> {
        let inline_size = smallvec::SmallVec::<A>::new().inline_size();
        let mut memory = if count > inline_size {
            let bytes = std::alloc::Layout::array::<A::Item>(count)
                .map_err(|_| crate::Error::invalid_usage("submission scratch size overflows"))?
                .size();
            Some(stats.reserve_cpu_bytes(bytes as u64, crate::accounting::BudgetCategory::Live)?)
        } else {
            None
        };
        let mut items = smallvec::SmallVec::<A>::new();
        items
            .try_reserve_exact(count)
            .map_err(|_| crate::Error::OutOfHostMemory)?;
        if let Some(memory) = &mut memory {
            memory.resize((items.capacity() * std::mem::size_of::<A::Item>()) as u64)?;
        }
        for value in values {
            if items.len() == count {
                return Err(crate::Error::invalid_usage(
                    "submission scratch count changed",
                ));
            }
            items.push(value?);
        }
        Ok(Self {
            items,
            _memory: memory,
        })
    }
}
impl<A: smallvec::Array> std::ops::Deref for SubmitScratch<A> {
    type Target = [A::Item];
    fn deref(&self) -> &Self::Target {
        &self.items
    }
}
impl<A: smallvec::Array> std::ops::DerefMut for SubmitScratch<A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

#[derive(Clone, Debug, Default)]
pub struct PreparedSubmit {
    pub wait_semaphores: SubmitArray<vk::Semaphore>,
    pub wait_stages: SubmitArray<vk::PipelineStageFlags>,
    pub wait_values: SubmitArray<u64>,
    pub signal_semaphores: SubmitArray<vk::Semaphore>,
    pub signal_values: SubmitArray<u64>,
    pub commands: SubmitArray<vk::CommandBuffer>,
}

fn array_heap_bytes<T>(count: usize) -> crate::Result<usize> {
    if count <= 8 {
        return Ok(0);
    }
    Ok(std::alloc::Layout::array::<T>(count)
        .map_err(|_| crate::Error::invalid_usage("submission array size overflows"))?
        .size())
}
fn sum_heap_bytes(values: impl IntoIterator<Item = usize>) -> crate::Result<usize> {
    values.into_iter().try_fold(0usize, |sum, value| {
        sum.checked_add(value)
            .ok_or_else(|| crate::Error::invalid_usage("submission array storage overflows"))
    })
}

impl PreparedSubmit {
    /// Capacity used by a fallible owned copy. Extra waits are reserved before
    /// resource gates are acquired, so dependency insertion cannot allocate.
    pub(crate) fn copied_heap_bytes(&self, extra_waits: usize) -> crate::Result<usize> {
        fn bytes<T>(len: usize, extra: usize) -> crate::Result<usize> {
            let count = len
                .checked_add(extra)
                .ok_or_else(|| crate::Error::invalid_usage("submission array length overflows"))?;
            array_heap_bytes::<T>(count)
        }
        sum_heap_bytes([
            bytes::<vk::Semaphore>(self.wait_semaphores.len(), extra_waits)?,
            bytes::<vk::PipelineStageFlags>(self.wait_stages.len(), extra_waits)?,
            bytes::<u64>(self.wait_values.len(), extra_waits)?,
            bytes::<vk::Semaphore>(self.signal_semaphores.len(), 0)?,
            bytes::<u64>(self.signal_values.len(), 0)?,
            bytes::<vk::CommandBuffer>(self.commands.len(), 0)?,
        ])
    }
    /// The six nested arrays are independent allocations once their inline
    /// capacity is exceeded. Public and owned submission adapters reserve this
    /// storage before building any of them, then observe their real capacities.
    pub(crate) fn heap_bytes_for_counts(
        waits: usize,
        signals: usize,
        commands: usize,
    ) -> crate::Result<usize> {
        sum_heap_bytes([
            array_heap_bytes::<vk::Semaphore>(waits)?,
            array_heap_bytes::<vk::PipelineStageFlags>(waits)?,
            array_heap_bytes::<u64>(waits)?,
            array_heap_bytes::<vk::Semaphore>(signals)?,
            array_heap_bytes::<u64>(signals)?,
            array_heap_bytes::<vk::CommandBuffer>(commands)?,
        ])
    }
    pub(crate) fn try_clone_with_wait_capacity(&self, extra_waits: usize) -> crate::Result<Self> {
        fn copy<T: Copy>(values: &[T], extra: usize) -> crate::Result<SubmitArray<T>> {
            let count = values
                .len()
                .checked_add(extra)
                .ok_or_else(|| crate::Error::invalid_usage("submission array length overflows"))?;
            let mut copied = SubmitArray::new();
            copied
                .try_reserve_exact(count)
                .map_err(|_| crate::Error::OutOfHostMemory)?;
            copied.extend_from_slice(values);
            Ok(copied)
        }
        Ok(Self {
            wait_semaphores: copy(&self.wait_semaphores, extra_waits)?,
            wait_stages: copy(&self.wait_stages, extra_waits)?,
            wait_values: copy(&self.wait_values, extra_waits)?,
            signal_semaphores: copy(&self.signal_semaphores, 0)?,
            signal_values: copy(&self.signal_values, 0)?,
            commands: copy(&self.commands, 0)?,
        })
    }
    pub(crate) fn heap_capacity_bytes(&self) -> usize {
        fn bytes<T>(values: &SubmitArray<T>) -> usize {
            if values.spilled() {
                values.capacity() * std::mem::size_of::<T>()
            } else {
                0
            }
        }
        bytes(&self.wait_semaphores)
            + bytes(&self.wait_stages)
            + bytes(&self.wait_values)
            + bytes(&self.signal_semaphores)
            + bytes(&self.signal_values)
            + bytes(&self.commands)
    }
    /// Validate all owned submission entry points before taking a queue lock.
    /// Device-owned timeline waits can only depend on already published submits.
    pub(crate) fn validate<A: crate::sync::timeline::TimelineApi>(
        &self,
        timelines: &crate::sync::timeline::Timelines<A>,
    ) -> crate::Result<()> {
        if self.wait_semaphores.len() != self.wait_values.len()
            || self.wait_semaphores.len() != self.wait_stages.len()
            || self.signal_semaphores.len() != self.signal_values.len()
            || self.wait_stages.iter().any(|stage| stage.is_empty())
        {
            return Err(crate::Error::invalid_usage(
                "提交的信号量/值/阶段数量不匹配",
            ));
        }
        for (index, &semaphore) in self.wait_semaphores.iter().enumerate() {
            if semaphore == vk::Semaphore::null()
                || self.wait_semaphores[..index].contains(&semaphore)
            {
                return Err(crate::Error::invalid_usage("提交等待信号量为空或重复"));
            }
        }
        for (index, &semaphore) in self.signal_semaphores.iter().enumerate() {
            if semaphore == vk::Semaphore::null()
                || self.signal_semaphores[..index].contains(&semaphore)
            {
                return Err(crate::Error::invalid_usage("提交信号量为空或重复"));
            }
        }
        for index in 0..timelines.queue_count() {
            let queue = crate::sync::QueueId::new(index).expect("队列数在构造时已验证");
            let semaphore = timelines.semaphore(queue)?;
            if self.signal_semaphores.contains(&semaphore) {
                return Err(crate::Error::invalid_usage(
                    "设备时间线只允许由统一提交入口signal",
                ));
            }
            for (&wait, &value) in self.wait_semaphores.iter().zip(&self.wait_values) {
                if wait == semaphore && value > timelines.submitted(queue)? {
                    return Err(crate::Error::invalid_usage("等待的设备时间线值尚未提交"));
                }
            }
        }
        Ok(())
    }


}

#[cfg(test)]
mod tests;
