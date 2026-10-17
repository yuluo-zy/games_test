//! Exercise the public thin submission adapter with real native input objects.
mod gpu_test_support;
use ash::vk;
use rhapsody::{
    device::{Device, DeviceConfig, Feature, Signal, SubmissionApi, SubmitBatch, Wait},
    sync::{QueueId, SyncPoint},
};
use std::sync::Arc;

struct BinarySignals {
    device: Arc<Device>,
    handles: Vec<vk::Semaphore>,
}
impl BinarySignals {
    fn new(device: Arc<Device>, count: usize) -> Self {
        let mut owner = Self {
            device,
            handles: Vec::new(),
        };
        for _ in 0..count {
            owner.handles.push(unsafe {
                owner
                    .device
                    .device
                    .create_semaphore(&vk::SemaphoreCreateInfo::default(), None)
                    .unwrap()
            });
        }
        owner
    }
}
impl Drop for BinarySignals {
    fn drop(&mut self) {
        // This test never submits commands. Wait before destruction even when
        // an assertion unwinds after a successful binary signal operation.
        let _ = self.device.wait_idle();
        for semaphore in &self.handles {
            unsafe {
                self.device.device.destroy_semaphore(*semaphore, None);
            }
        }
    }
}
fn device() -> Option<Arc<Device>> {
    gpu_test_support::initialize(
        "public submission adapter",
        Device::new_headless(&DeviceConfig::headless()),
    )
}

#[test]
fn outer_and_nested_spills_fail_before_submission_and_release_their_cpu_budget() {
    let Some(device) = device() else { return };
    let queue = QueueId::new(0).unwrap();
    let signals = BinarySignals::new(device.clone(), 9);
    let signal_info: Vec<_> = signals
        .handles
        .iter()
        .copied()
        .map(Signal::Binary)
        .collect();
    let before = device.allocator_stats.cpu_memory_usage();
    let submitted = device.timelines.submitted(queue).unwrap();
    device
        .allocator_stats
        .set_total_budget(device.allocator_stats.budget_usage().total() + 1)
        .unwrap();
    let batches = std::array::from_fn::<_, 2, _>(|_| SubmitBatch {
        waits: &[],
        commands: &[],
        signals: &[],
    });
    assert!(matches!(
        unsafe { device.submit(queue, &batches) },
        Err(rhapsody::Error::OutOfHostMemory)
    ));
    assert_eq!(device.timelines.submitted(queue).unwrap(), submitted);
    assert_eq!(device.allocator_stats.cpu_memory_usage(), before);
    let batch = SubmitBatch {
        waits: &[],
        commands: &[],
        signals: &signal_info,
    };
    assert!(matches!(
        unsafe { device.submit(queue, &[batch]) },
        Err(rhapsody::Error::OutOfHostMemory)
    ));
    assert_eq!(device.timelines.submitted(queue).unwrap(), submitted);
    assert_eq!(device.allocator_stats.cpu_memory_usage(), before);
    device.allocator_stats.set_total_budget(u64::MAX).unwrap();
    let batch = SubmitBatch {
        waits: &[],
        commands: &[],
        signals: &signal_info,
    };
    let point = unsafe { device.submit(queue, &[batch]) }.unwrap();
    device.wait(&point.into(), 10_000_000_000).unwrap();
    assert_eq!(point.value, submitted + 1);
    assert_eq!(device.allocator_stats.cpu_memory_usage(), before);
}

#[test]
fn rejected_waits_and_duplicate_signals_do_not_publish_a_completion_point() {
    let Some(device) = device() else { return };
    let queue = QueueId::new(0).unwrap();
    let semaphore = BinarySignals::new(device.clone(), 1);
    let signals = [Signal::Binary(semaphore.handles[0]); 2];
    let before = device.timelines.submitted(queue).unwrap();
    let batch = SubmitBatch {
        waits: &[],
        commands: &[],
        signals: &signals,
    };
    assert!(unsafe { device.submit(queue, &[batch]) }.is_err());
    let waits = [Wait::Timeline {
        point: SyncPoint {
            queue,
            value: before + 1,
        },
        stage: vk::PipelineStageFlags::ALL_COMMANDS,
    }];
    let batch = SubmitBatch {
        waits: &waits,
        commands: &[],
        signals: &[],
    };
    assert!(unsafe { device.submit(queue, &[batch]) }.is_err());
    assert_eq!(device.timelines.submitted(queue).unwrap(), before);
    let point = unsafe { device.submit(queue, &[]) }.unwrap();
    device.wait(&point.into(), 10_000_000_000).unwrap();
    assert_eq!(point.value, before + 1);
}

#[test]
fn known_loss_is_rejected_before_adapter_scratch_admission() {
    let Some(device) = device() else { return };
    let queue = QueueId::new(0).unwrap();
    let before = device.allocator_stats.cpu_memory_usage();
    device
        .allocator_stats
        .set_total_budget(device.allocator_stats.budget_usage().total() + 1)
        .unwrap();
    device.timelines.mark_device_lost();
    let batches = std::array::from_fn::<_, 2, _>(|_| SubmitBatch {
        waits: &[],
        commands: &[],
        signals: &[],
    });
    assert!(matches!(
        unsafe { device.submit(queue, &batches) },
        Err(rhapsody::Error::DeviceLost)
    ));
    assert_eq!(device.allocator_stats.cpu_memory_usage(), before);
}

#[test]
fn legacy_submission_preserves_supported_caps_and_copies_with_real_timeline_waits() {
    use rhapsody::raw::{
        buffer::{Buffer, BufferDesc},
        command_buffer::VulkanCommandPool,
    };
    let Some(device) = gpu_test_support::initialize(
        "legacy timeline submission",
        Device::new_headless(&DeviceConfig {
            submission_api: SubmissionApi::Legacy,
            ..DeviceConfig::headless()
        }),
    ) else {
        return;
    };
    let Some(device) = gpu_test_support::initialize(
        "supported-but-disabled synchronization2 comparison",
        if device.caps().is_supported(Feature::Synchronization2) {
            Ok(device)
        } else {
            Err(rhapsody::Error::unsupported(
                "comparison requires a synchronization2-capable device",
            ))
        },
    ) else {
        return;
    };
    assert!(device.caps().is_supported(Feature::Synchronization2));
    assert!(!device.caps().is_enabled(Feature::Synchronization2));
    for feature in [
        Feature::TimelineSemaphore,
        Feature::DynamicRendering,
        Feature::DescriptorIndexing,
    ] {
        assert!(device.caps().is_enabled(feature));
    }
    let desc = BufferDesc::new_gpu_to_cpu(
        32,
        vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST,
    );
    let source = Buffer::new(device.clone(), &desc).unwrap();
    let destination = Buffer::new(device.clone(), &desc).unwrap();
    let expected: [u8; 32] = std::array::from_fn(|index| index as u8 ^ 0x5a);
    source.write_bytes(0, &expected).unwrap();
    destination.write_bytes(0, &[0; 32]).unwrap();
    let pool = VulkanCommandPool::new(device.clone()).unwrap();
    let mut command = pool.allocate().unwrap();
    command.begin().unwrap();
    unsafe {
        device.device.cmd_pipeline_barrier(
            command.handle,
            vk::PipelineStageFlags::HOST,
            vk::PipelineStageFlags::TRANSFER,
            vk::DependencyFlags::empty(),
            &[],
            &[
                (source.handle, vk::AccessFlags::TRANSFER_READ),
                (destination.handle, vk::AccessFlags::TRANSFER_WRITE),
            ]
            .map(|(buffer, access)| {
                vk::BufferMemoryBarrier::default()
                    .src_access_mask(vk::AccessFlags::HOST_WRITE)
                    .dst_access_mask(access)
                    .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                    .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                    .buffer(buffer)
                    .offset(0)
                    .size(32)
            }),
            &[],
        );
        device.device.cmd_copy_buffer(
            command.handle,
            source.handle,
            destination.handle,
            &[vk::BufferCopy {
                src_offset: 0,
                dst_offset: 0,
                size: 32,
            }],
        );
        device.device.cmd_pipeline_barrier(
            command.handle,
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::HOST,
            vk::DependencyFlags::empty(),
            &[],
            &[vk::BufferMemoryBarrier::default()
                .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                .dst_access_mask(vk::AccessFlags::HOST_READ)
                .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .buffer(destination.handle)
                .offset(0)
                .size(32)],
            &[],
        );
    }
    command.end().unwrap();
    let queue = QueueId::new(0).unwrap();
    let initialized = unsafe { device.submit(queue, &[]) }.unwrap();
    // This public unsafe entry requires the caller to retain these exact
    // commands/resources; all owners remain in scope through the final wait.
    let point = unsafe {
        device.submit(
            queue,
            &[
                SubmitBatch {
                    waits: &[],
                    commands: &[command.handle],
                    signals: &[],
                },
                SubmitBatch {
                    waits: &[Wait::Timeline {
                        point: initialized,
                        stage: vk::PipelineStageFlags::ALL_COMMANDS,
                    }],
                    commands: &[],
                    signals: &[],
                },
            ],
        )
    }
    .unwrap();
    let waited = unsafe {
        device.submit(
            queue,
            &[SubmitBatch {
                waits: &[Wait::Timeline {
                    point,
                    stage: vk::PipelineStageFlags::ALL_COMMANDS,
                }],
                commands: &[],
                signals: &[],
            }],
        )
    }
    .unwrap();
    device.wait(&waited.into(), 10_000_000_000).unwrap();
    let mut actual = [0; 32];
    destination.read_bytes(0, &mut actual).unwrap();
    assert_eq!(actual, expected);
    assert!(waited.value > point.value);
    eprintln!(
        "legacy native copy passed: supported synchronization2=true, enabled=false, real timeline {} -> {}",
        point.value, waited.value
    );
}
