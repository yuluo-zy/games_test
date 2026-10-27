use super::*;
use crate::native::submit;
use ash::vk;
use std::sync::Arc;
use ash::vk::Handle;

#[test]
fn submission_capture_is_inline_or_fallibly_budgeted_for_its_real_lifetime() {
    let stats = Arc::new(crate::accounting::AllocatorStats::default());
    stats.set_total_budget(1).unwrap();
    let batch = submit::PreparedSubmit {
        commands: [vk::CommandBuffer::from_raw(1)].into_iter().collect(),
        ..Default::default()
    };
    let capture = GpuSubmissionCapture::try_new(
        vk::Queue::from_raw(1),
        std::slice::from_ref(&batch),
        2,
        &stats,
    )
    .unwrap();
    assert!(!capture.batches.spilled());
    assert_eq!(capture.batches[0].heap_capacity_bytes(), 0);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
    drop(capture);
    let batches = [batch.clone(), batch];
    assert!(matches!(
        GpuSubmissionCapture::try_new(vk::Queue::from_raw(1), &batches, 2, &stats),
        Err(crate::Error::OutOfHostMemory)
    ));
    assert_eq!(stats.cpu_memory_usage().total(), 0);
    stats.set_total_budget(1 << 20).unwrap();
    let capture =
        GpuSubmissionCapture::try_new(vk::Queue::from_raw(1), &batches, 2, &stats).unwrap();
    assert!(capture.batches.spilled());
    assert_eq!(capture.batches.len(), 2);
    assert!(
        stats.cpu_memory_usage().cache
            >= (2 * std::mem::size_of::<submit::PreparedSubmit>()) as u64
    );
    drop(capture);
    assert_eq!(stats.cpu_memory_usage().total(), 0);
    assert_eq!(batches[0].commands[0], vk::CommandBuffer::from_raw(1));
}
