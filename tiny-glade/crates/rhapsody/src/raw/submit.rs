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
    _memory: Option<super::allocator::CpuMemoryReservation>,
}
impl<A: smallvec::Array> SubmitScratch<A> {
    pub(crate) fn try_collect(
        count: usize,
        stats: &std::sync::Arc<super::allocator::AllocatorStats>,
        values: impl IntoIterator<Item = crate::Result<A::Item>>,
    ) -> crate::Result<Self> {
        let inline_size = smallvec::SmallVec::<A>::new().inline_size();
        let mut memory = if count > inline_size {
            let bytes = std::alloc::Layout::array::<A::Item>(count)
                .map_err(|_| crate::Error::invalid_usage("submission scratch size overflows"))?
                .size();
            Some(stats.reserve_cpu_bytes(bytes as u64, crate::device::BudgetCategory::Live)?)
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

    /// 仅在所有数组和 pNext 仍存活的回调期间提供真实 ash 结构。
    pub fn with_vk<R>(&self, call: impl FnOnce(&vk::SubmitInfo<'_>) -> R) -> R {
        let mut timeline = vk::TimelineSemaphoreSubmitInfo::default()
            .wait_semaphore_values(&self.wait_values)
            .signal_semaphore_values(&self.signal_values);
        let submit = vk::SubmitInfo::default()
            .wait_semaphores(&self.wait_semaphores)
            .wait_dst_stage_mask(&self.wait_stages)
            .command_buffers(&self.commands)
            .signal_semaphores(&self.signal_semaphores)
            .push_next(&mut timeline);
        call(&submit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::{
        QueueId,
        timeline::{TimelineApi, Timelines},
    };
    use ash::vk::Handle;

    #[test]
    fn adapter_nested_capacity_counts_every_spilled_array_and_checks_overflow() {
        assert_eq!(PreparedSubmit::heap_bytes_for_counts(8, 8, 8).unwrap(), 0);
        let waits = 9
            * (size_of::<vk::Semaphore>() + size_of::<u64>() + size_of::<vk::PipelineStageFlags>());
        let signals = 10 * (size_of::<vk::Semaphore>() + size_of::<u64>());
        let commands = 11 * size_of::<vk::CommandBuffer>();
        assert_eq!(
            PreparedSubmit::heap_bytes_for_counts(9, 10, 11).unwrap(),
            waits + signals + commands
        );
        assert!(PreparedSubmit::heap_bytes_for_counts(usize::MAX, 0, 0).is_err());
        // Public PreparedSubmit can be malformed before validation. Accounting
        // must still cover its individual vectors rather than assume equal lengths.
        let mut malformed = PreparedSubmit::default();
        malformed.wait_values.try_reserve_exact(9).unwrap();
        malformed.wait_values.extend([0; 9]);
        assert_eq!(
            malformed.copied_heap_bytes(0).unwrap(),
            9 * size_of::<u64>()
        );
        assert_eq!(malformed.heap_capacity_bytes(), 9 * size_of::<u64>());
    }

    #[test]
    fn submission_scratch_stays_inline_and_spills_with_its_actual_owner() {
        let stats = std::sync::Arc::new(super::super::allocator::AllocatorStats::default());
        stats.set_total_budget(1).unwrap();
        let inline = SubmitScratch::<[u64; 8]>::try_collect(8, &stats, (0..8).map(Ok)).unwrap();
        assert_eq!(&*inline, &[0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
        drop(inline);
        let consumed = std::cell::Cell::new(0);
        assert!(matches!(
            SubmitScratch::<[u64; 8]>::try_collect(
                9,
                &stats,
                (0..9).map(|value| {
                    consumed.set(consumed.get() + 1);
                    Ok(value)
                })
            ),
            Err(crate::Error::OutOfHostMemory)
        ));
        assert_eq!(
            consumed.get(),
            0,
            "reserve before collecting owners or guards"
        );
        stats.set_total_budget(4096).unwrap();
        let spill = SubmitScratch::<[u64; 8]>::try_collect(9, &stats, (0..9).map(Ok)).unwrap();
        assert!(stats.cpu_memory_usage().live >= 9 * 8);
        drop(spill);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }

    #[test]
    fn submission_scratch_error_drops_partial_owners_and_returns_all_credit() {
        let stats = std::sync::Arc::new(super::super::allocator::AllocatorStats::default());
        let owner = std::sync::Arc::new(1u32);
        let values = (0..10).map(|index| {
            if index == 4 {
                Err(crate::Error::DeviceLost)
            } else {
                Ok(owner.clone())
            }
        });
        assert!(matches!(
            SubmitScratch::<[std::sync::Arc<u32>; 8]>::try_collect(10, &stats, values),
            Err(crate::Error::DeviceLost)
        ));
        assert_eq!(std::sync::Arc::strong_count(&owner), 1);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }

    #[test]
    fn fallible_owned_copy_keeps_common_batches_inline_and_reserves_all_queue_waits() {
        let batch = PreparedSubmit {
            commands: [vk::CommandBuffer::from_raw(1)].into_iter().collect(),
            ..Default::default()
        };
        let mut copied = batch
            .try_clone_with_wait_capacity(crate::sync::MAX_QUEUES - 1)
            .unwrap();
        assert_eq!(
            batch
                .copied_heap_bytes(crate::sync::MAX_QUEUES - 1)
                .unwrap(),
            0
        );
        assert_eq!(copied.heap_capacity_bytes(), 0);
        let capacities = (
            copied.wait_semaphores.capacity(),
            copied.wait_values.capacity(),
            copied.wait_stages.capacity(),
        );
        for index in 1..crate::sync::MAX_QUEUES {
            copied
                .wait_semaphores
                .push(vk::Semaphore::from_raw(index as u64));
            copied.wait_values.push(index as u64);
            copied
                .wait_stages
                .push(vk::PipelineStageFlags::ALL_COMMANDS);
        }
        assert_eq!(
            (
                copied.wait_semaphores.capacity(),
                copied.wait_values.capacity(),
                copied.wait_stages.capacity()
            ),
            capacities
        );
        assert_eq!(copied.heap_capacity_bytes(), 0);
        assert!(batch.wait_semaphores.is_empty());
    }

    struct NoGpu;
    impl TimelineApi for NoGpu {
        fn counter_value(&self, _: vk::Semaphore) -> crate::Result<u64> {
            panic!("validation must not query GPU completion")
        }
        fn wait_all(&self, _: &[vk::Semaphore], _: &[u64], _: u64) -> crate::Result<bool> {
            panic!("validation must not wait")
        }
    }
    fn wait(semaphore: vk::Semaphore, value: u64) -> PreparedSubmit {
        PreparedSubmit {
            wait_semaphores: [semaphore].into_iter().collect(),
            wait_values: [value].into_iter().collect(),
            wait_stages: [vk::PipelineStageFlags::ALL_COMMANDS].into_iter().collect(),
            ..Default::default()
        }
    }
    #[test]
    fn every_owned_timeline_wait_requires_a_successful_prior_submit() {
        let semaphores = [vk::Semaphore::from_raw(10), vk::Semaphore::from_raw(20)];
        let timelines = Timelines::new(NoGpu, &semaphores).unwrap();
        for (index, &semaphore) in semaphores.iter().enumerate() {
            let queue = QueueId::new(index).unwrap();
            assert_eq!(
                wait(semaphore, 1).validate(&timelines).unwrap_err().kind(),
                crate::ErrorKind::InvalidUsage
            );
            assert_eq!(timelines.submitted(queue).unwrap(), 0);
            let failed = timelines.submit(queue, |_| Err::<(), _>(crate::Error::OutOfDeviceMemory));
            assert!(failed.is_err());
            assert!(wait(semaphore, 1).validate(&timelines).is_err());
            timelines.submit(queue, |_| Ok(())).unwrap();
            wait(semaphore, 1).validate(&timelines).unwrap();
            assert!(wait(semaphore, 2).validate(&timelines).is_err());
        }
        // User-owned timeline waits remain valid: their producer is outside
        // device completion bookkeeping and may signal from another submission.
        wait(vk::Semaphore::from_raw(30), 7)
            .validate(&timelines)
            .unwrap();
    }
    #[test]
    fn malformed_arrays_and_manual_device_timeline_signals_are_rejected() {
        let owned = vk::Semaphore::from_raw(10);
        let timelines = Timelines::new(NoGpu, &[owned]).unwrap();
        let mut malformed = wait(owned, 0);
        malformed.wait_values.clear();
        assert!(malformed.validate(&timelines).is_err());
        let manual_signal = PreparedSubmit {
            signal_semaphores: [owned].into_iter().collect(),
            signal_values: [1].into_iter().collect(),
            ..Default::default()
        };
        assert!(manual_signal.validate(&timelines).is_err());
        let null_wait = wait(vk::Semaphore::null(), 0);
        assert!(null_wait.validate(&timelines).is_err());
        let mut duplicate = wait(vk::Semaphore::from_raw(30), 0);
        duplicate.wait_semaphores.push(duplicate.wait_semaphores[0]);
        duplicate.wait_values.push(0);
        duplicate
            .wait_stages
            .push(vk::PipelineStageFlags::ALL_COMMANDS);
        assert!(duplicate.validate(&timelines).is_err());
    }
}
