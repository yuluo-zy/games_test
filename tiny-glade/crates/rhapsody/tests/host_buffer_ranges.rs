//! Real pending GPU work distinguishes disjoint host access from overlap and
//! unknown-use fallback. No fake completion values or driver faults are used.
mod gpu_test_support;
use ash::vk;
use rhapsody::{
    Error,
    device::{Device, DeviceConfig, SubmissionApi},
    raw::{
        buffer::{Buffer, BufferDesc},
        command_buffer::{PendingCommand, VulkanCommandPool},
        device::DeviceQueue,
        instance::{InstanceBuilder, ValidationMessenger},
        semaphore::TimelineSemaphore,
    },
    sync::QueueId,
};
use std::sync::Arc;
const TIMEOUT: u64 = 10_000_000_000;
struct SignalOnDrop {
    semaphore: Arc<TimelineSemaphore>,
    armed: bool,
}
impl SignalOnDrop {
    fn release(&mut self) {
        if self.armed {
            let result = unsafe {
                self.semaphore.device().raw_device().signal_semaphore(
                    &vk::SemaphoreSignalInfo::default()
                        .semaphore(self.semaphore.raw_handle())
                        .value(1),
                )
            };
            self.armed = false;
            if result == Err(vk::Result::ERROR_DEVICE_LOST) {
                self.semaphore.device().timelines().mark_device_lost();
            }
            result.unwrap();
        }
    }
}
impl Drop for SignalOnDrop {
    fn drop(&mut self) {
        // Always unblock real work before assertion unwinding can drop Device.
        if self.armed {
            let result = unsafe {
                self.semaphore.device().raw_device().signal_semaphore(
                    &vk::SemaphoreSignalInfo::default()
                        .semaphore(self.semaphore.raw_handle())
                        .value(1),
                )
            };
            if result == Err(vk::Result::ERROR_DEVICE_LOST) {
                self.semaphore.device().timelines().mark_device_lost();
            }
        }
    }
}
fn fixture(api: SubmissionApi) -> Option<(Arc<Device>, ValidationMessenger)> {
    gpu_test_support::initialize(
        "host buffer interval ledger",
        (|| -> rhapsody::Result<_> {
            let mut instance = InstanceBuilder::default();
            instance.debug_markers();
            instance
                .layers
                .push(std::ffi::CString::new("VK_LAYER_KHRONOS_validation").unwrap());
            let device = Device::new(
                instance.build()?,
                None,
                &DeviceConfig {
                    submission_api: api,
                    validation: true,
                    ..DeviceConfig::headless()
                },
            )?;
            let validation = ValidationMessenger::new(device.instance())?;
            rhapsody::Result::Ok((device, validation))
        })(),
    )
}
fn blocked_fill(
    device: &Arc<Device>,
    buffer: &Arc<Buffer>,
    span: u64,
    unknown: Option<bool>,
) -> (PendingCommand, SignalOnDrop) {
    let pool = VulkanCommandPool::new(device.clone()).unwrap();
    let gate = TimelineSemaphore::new(device.clone(), 0).unwrap();
    let release = SignalOnDrop {
        semaphore: gate.clone(),
        armed: true,
    };
    let mut command = pool.allocate().unwrap();
    if unknown == Some(true) {
        command.retain_opaque(buffer.clone()).unwrap();
    }
    unsafe {
        command
            .retain_opaque_buffer_range(buffer.clone(), 0, span)
            .unwrap();
    }
    if unknown == Some(false) {
        command.retain_opaque(buffer.clone()).unwrap();
    }
    command.begin().unwrap();
    unsafe {
        device.raw_device().cmd_fill_buffer(
            command.raw_handle(),
            buffer.raw_handle(),
            0,
            span,
            0x44332211,
        );
        device.raw_device().cmd_pipeline_barrier(
            command.raw_handle(),
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::HOST,
            vk::DependencyFlags::empty(),
            &[],
            &[vk::BufferMemoryBarrier::default()
                .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                .dst_access_mask(vk::AccessFlags::HOST_READ)
                .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .buffer(buffer.raw_handle())
                .offset(0)
                .size(span)],
            &[],
        );
    }
    command.end().unwrap();
    let pending = command
        .submit_to(
            DeviceQueue {
                handle: device.primary_queue(),
                family: device.queue_family_index(),
                index: 0,
            },
            &[(gate, 1, vk::PipelineStageFlags::ALL_COMMANDS)],
            &[],
        )
        .unwrap();
    (pending, release)
}
fn buffer(device: &Arc<Device>) -> (Arc<Buffer>, u64) {
    let span = device
        .limits()
        .non_coherent_atom_size
        .max(256)
        .next_multiple_of(4);
    let buffer = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_to_cpu(
                span * 2,
                vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::TRANSFER_SRC,
            ),
        )
        .unwrap(),
    );
    buffer.write_bytes(0, &vec![0; span as usize * 2]).unwrap();
    (buffer, span)
}
fn assert_validation_empty(validation: &ValidationMessenger) {
    assert!(
        validation.collector.errors().is_empty(),
        "{:?}",
        validation.collector.errors()
    );
    assert!(
        validation.collector.warnings().is_empty(),
        "{:?}",
        validation.collector.warnings()
    );
}
#[test]
fn a_disjoint_host_write_succeeds_while_real_gpu_work_is_blocked_in_both_submit_apis() {
    for api in [SubmissionApi::Auto, SubmissionApi::Legacy] {
        let Some((device, validation)) = fixture(api) else {
            return;
        };
        let (buffer, span) = buffer(&device);
        let (pending, mut release) = blocked_fill(&device, &buffer, span, None);
        assert!(!pending.is_complete().unwrap());
        buffer
            .write_bytes_unsynchronized(span, &vec![0x5a; span as usize])
            .unwrap();
        assert!(matches!(
            buffer.write_bytes_unsynchronized(0, &[0; 4]),
            Err(Error::Timeout)
        ));
        release.release();
        pending.wait(TIMEOUT).unwrap();
        let mut actual = vec![0; span as usize * 2];
        buffer.read_bytes(0, &mut actual).unwrap();
        assert!(
            actual[..span as usize]
                .as_chunks::<4>()
                .0
                .iter()
                .all(|word| word == &[0x11, 0x22, 0x33, 0x44])
        );
        assert!(actual[span as usize..].iter().all(|byte| *byte == 0x5a));
        drop(pending);
        drop(release);
        drop(buffer);
        device.try_collect_all_completed().unwrap();
        assert_validation_empty(&validation);
    }
}
#[test]
fn generic_reference_cannot_be_overridden_by_an_earlier_or_later_precise_use() {
    let Some((device, validation)) = fixture(SubmissionApi::Auto) else {
        return;
    };
    for unknown_first in [true, false] {
        let (buffer, span) = buffer(&device);
        let (pending, mut release) = blocked_fill(&device, &buffer, span, Some(unknown_first));
        assert!(!pending.is_complete().unwrap());
        assert!(matches!(
            buffer.write_bytes_unsynchronized(span, &[0; 4]),
            Err(Error::Timeout)
        ));
        release.release();
        pending.wait(TIMEOUT).unwrap();
        buffer.write_bytes_unsynchronized(span, &[0x5a; 4]).unwrap();
    }
    device.try_collect_all_completed().unwrap();
    assert_validation_empty(&validation);
}
#[test]
fn metadata_admission_failure_stays_on_the_command_and_publishes_no_completion_point() {
    let Some((device, validation)) = fixture(SubmissionApi::Auto) else {
        return;
    };
    let (buffer, span) = buffer(&device);
    let pool = VulkanCommandPool::new(device.clone()).unwrap();
    let queue = QueueId::new(0).unwrap();
    for generic in [false, true] {
        let mut command = pool.allocate().unwrap();
        command.begin().unwrap();
        let submitted = device.timelines().submitted(queue).unwrap();
        let before = device.allocator_stats().cpu_memory_usage();
        device
            .allocator_stats()
            .set_host_budget(device.allocator_stats().cpu_memory_usage().total() + 1)
            .unwrap();
        if generic {
            assert!(matches!(
                command.retain_opaque(buffer.clone()),
                Err(Error::OutOfHostMemory)
            ));
        } else {
            assert!(matches!(
                unsafe { command.retain_opaque_buffer_range(buffer.clone(), 0, span) },
                Err(Error::OutOfHostMemory)
            ));
        }
        device.allocator_stats().set_host_budget(u64::MAX).unwrap();
        assert!(matches!(command.end(), Err(Error::OutOfHostMemory)));
        assert!(matches!(command.end(), Err(Error::OutOfHostMemory)));
        assert_eq!(device.timelines().submitted(queue).unwrap(), submitted);
        assert_eq!(device.allocator_stats().cpu_memory_usage(), before);
        drop(command);
        buffer.write_bytes_unsynchronized(0, &[1; 4]).unwrap();
    }
    device.try_collect_all_completed().unwrap();
    assert_validation_empty(&validation);
}
