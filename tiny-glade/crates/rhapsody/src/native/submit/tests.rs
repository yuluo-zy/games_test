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
    let stats = std::sync::Arc::new(crate::accounting::AllocatorStats::default());
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
    let stats = std::sync::Arc::new(crate::accounting::AllocatorStats::default());
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
