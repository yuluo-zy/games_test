//! Bounded production constant pages: CPU leases and actual submitted GPU use.
mod gpu_test_support;
use ash::vk;
use rhapsody::{
    device::{Device, DeviceConfig},
    raw::{
        buffer::{Buffer, BufferDesc},
        command_buffer::VulkanCommandPool,
        dynamic_constants::{
            DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES, DynamicConstants, DynamicConstantsConfig,
        },
        semaphore::TimelineSemaphore,
    },
};
use std::sync::Arc;

fn config() -> DynamicConstantsConfig {
    DynamicConstantsConfig {
        max_pages: 1,
        max_backing_bytes: DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES,
        max_idle_pages: 1,
    }
}
struct Unblock(Arc<TimelineSemaphore>, bool);
impl Unblock {
    fn signal(&mut self) {
        if !self.1 {
            // SAFETY: this test is the sole host signaler and submits value 1 only once.
            unsafe {
                self.0.device.device.signal_semaphore(
                    &vk::SemaphoreSignalInfo::default()
                        .semaphore(self.0.handle)
                        .value(1),
                )
            }
            .unwrap();
            self.1 = true;
        }
    }
}
impl Drop for Unblock {
    fn drop(&mut self) {
        self.signal();
    }
}
#[test]
fn unsubmitted_constant_leases_bound_capacity_and_reuse_one_real_page() {
    let Some(device) = gpu_test_support::initialize(
        "dynamic constant CPU lease",
        Device::new_headless(&DeviceConfig {
            validation: true,
            ..DeviceConfig::headless()
        }),
    ) else {
        return;
    };
    let mut constants = DynamicConstants::with_config(device, config()).unwrap();
    let chunk = constants
        .allocate(DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES)
        .unwrap();
    let page = chunk.buffer.handle;
    let held = chunk.buffer.clone();
    drop(chunk);
    assert!(matches!(
        constants.allocate(1),
        Err(rhapsody::Error::Timeout)
    ));
    assert_eq!(constants.trim_idle(0).unwrap(), 0);
    drop(held);
    for index in 0_u32..1000 {
        let chunk = constants
            .allocate(DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES)
            .unwrap();
        assert_eq!(chunk.buffer.handle, page);
        assert_eq!(chunk.offset, 0);
        chunk.write_bytes(&index.to_ne_bytes()).unwrap();
    }
    assert_eq!(constants.stats().page_creations, 1);
    assert_eq!(constants.stats().page_reuses, 1000);
    assert_eq!(constants.trim_idle(0).unwrap(), 1);
    assert_eq!(constants.stats().backing_bytes, 0);
}
#[test]
fn dropped_cpu_owners_cannot_reset_a_page_until_actual_gpu_copy_completes() {
    let Some(device) = gpu_test_support::initialize(
        "dynamic constant real GPU completion",
        Device::new_headless(&DeviceConfig {
            validation: true,
            ..DeviceConfig::headless()
        }),
    ) else {
        return;
    };
    let mut constants = DynamicConstants::with_config(device.clone(), config()).unwrap();
    let pool = VulkanCommandPool::new(device.clone()).unwrap();
    let blocker = TimelineSemaphore::new(device.clone(), 0).unwrap();
    let mut unblock = Unblock(blocker.clone(), false);
    let chunk = constants
        .allocate(DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES)
        .unwrap();
    let source_handle = chunk.buffer.handle;
    let payload: Vec<u8> = (0..64).map(|index| index ^ 0x5a).collect();
    chunk.write_bytes(&payload).unwrap();
    let output = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_to_cpu(64, vk::BufferUsageFlags::TRANSFER_DST),
        )
        .unwrap(),
    );
    let mut command = pool.allocate().unwrap();
    command.retain_opaque(chunk.buffer.clone()).unwrap();
    command.retain_opaque(output.clone()).unwrap();
    command.begin().unwrap();
    // SAFETY: both owned buffers are alive, same-device, correctly sized and recorded.
    unsafe {
        device.device.cmd_copy_buffer(
            command.handle,
            source_handle,
            output.handle,
            &[vk::BufferCopy {
                src_offset: 0,
                dst_offset: 0,
                size: 64,
            }],
        );
        device.device.cmd_pipeline_barrier(
            command.handle,
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::HOST,
            vk::DependencyFlags::empty(),
            &[],
            &[vk::BufferMemoryBarrier::default()
                .buffer(output.handle)
                .offset(0)
                .size(64)
                .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                .dst_access_mask(vk::AccessFlags::HOST_READ)],
            &[],
        );
    }
    command.end().unwrap();
    let pending = command
        .submit_to(
            rhapsody::raw::device::DeviceQueue {
                handle: device.queue,
                family: device.queue_family,
                index: 0,
            },
            &[(blocker, 1, vk::PipelineStageFlags::ALL_COMMANDS)],
            &[],
        )
        .unwrap();
    let point = pending.point;
    drop(pending);
    drop(chunk);
    assert!(matches!(
        constants.allocate(1),
        Err(rhapsody::Error::Timeout)
    ));
    assert_eq!(constants.stats().referenced_pages, 0);
    assert_eq!(constants.stats().in_flight_pages, 1);
    assert_eq!(constants.trim_idle(0).unwrap(), 0);
    unblock.signal();
    device
        .timelines
        .wait(&point.into(), 10_000_000_000)
        .unwrap();
    let mut actual = [0; 64];
    output.read_bytes(0, &mut actual).unwrap();
    assert_eq!(actual.as_slice(), payload);
    let coherent = held_properties(&device);
    if coherent {
        assert_eq!(
            device.deferred_host_flush_count(),
            0,
            "coherent constants pages are never flushed"
        );
    } else {
        assert_eq!(device.deferred_host_flush_count(), 1);
    }
    let reused = constants
        .allocate(DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES)
        .unwrap();
    assert_eq!(reused.buffer.handle, source_handle);
    assert_eq!(reused.offset, 0);
    reused.write_bytes(&[0xff; 64]).unwrap();
    drop(reused);
    assert_eq!(constants.stats().page_creations, 1);
    assert_eq!(constants.stats().page_reuses, 1);
    assert_eq!(constants.trim_idle(0).unwrap(), 1);
    device.try_collect_all_completed().unwrap();
}

/// Whether the memory backing constants pages is host coherent on this device.
fn held_properties(device: &Arc<Device>) -> bool {
    let page = Buffer::new(
        device.clone(),
        &BufferDesc::new_cpu_to_gpu(64, vk::BufferUsageFlags::UNIFORM_BUFFER),
    )
    .unwrap();
    page.memory_properties()
        .contains(vk::MemoryPropertyFlags::HOST_COHERENT)
}
