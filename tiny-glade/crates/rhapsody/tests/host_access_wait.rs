//! Host reads/writes wait for GPU work without holding the buffer's host or
//! submission locks, and give up with `Timeout` after their bound.
mod gpu_test_support;
use ash::vk;
use rhapsody::device::{Device, DeviceConfig};
use rhapsody::command::command_buffer::VulkanCommandPool;
use rhapsody::resource::buffer::{Buffer, BufferDesc};
use rhapsody::device::semaphore::TimelineSemaphore;
use std::sync::Arc;

fn signal(gate: &TimelineSemaphore) {
    unsafe {
        gate.device()
            .raw_device()
            .signal_semaphore(
                &vk::SemaphoreSignalInfo::default()
                    .semaphore(gate.raw_handle())
                    .value(1),
            )
            .unwrap();
    }
}

#[test]
fn bounded_host_wait_times_out_and_completes_after_the_gpu_finishes() {
    let Some(device) = gpu_test_support::initialize(
        "host access wait",
        Device::new_headless(&DeviceConfig::headless()),
    ) else {
        return;
    };
    let buffer = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_to_cpu(256, vk::BufferUsageFlags::TRANSFER_DST),
        )
        .unwrap(),
    );
    let gate = TimelineSemaphore::new(device.clone(), 0).unwrap();
    let pool = VulkanCommandPool::new(device.clone()).unwrap();
    let mut command = pool.allocate().unwrap();
    command.retain_opaque(buffer.clone()).unwrap();
    command.begin().unwrap();
    unsafe {
        device.raw_device().cmd_fill_buffer(
            command.raw_handle(),
            buffer.raw_handle(),
            0,
            256,
            0x0101_0101,
        );
    }
    command.end().unwrap();
    let pending = command
        .submit_to(
            rhapsody::device::DeviceQueue {
                handle: device.primary_queue(),
                family: device.queue_family_index(),
                index: 0,
            },
            &[(gate.clone(), 1, vk::PipelineStageFlags::ALL_COMMANDS)],
            &[],
        )
        .unwrap();
    let point = pending.point;
    let mut bytes = [0u8; 4];
    let result = buffer.read_bytes_timeout(0, &mut bytes, 1_000_000);
    if !matches!(result, Err(rhapsody::Error::Timeout)) {
        signal(&gate);
        panic!("blocked GPU work must time out the bounded wait: {result:?}");
    }
    // A concurrent bounded wait does not keep the buffer's host lock, so a
    // second host access on another thread also returns promptly.
    let other = buffer.clone();
    let worker =
        std::thread::spawn(move || other.write_bytes_timeout(0, &[9; 4], 50_000_000).err());
    let concurrent = buffer.write_bytes_timeout(8, &[1; 4], 50_000_000);
    let worker = worker.join().unwrap();
    signal(&gate);
    assert!(matches!(concurrent, Err(rhapsody::Error::Timeout)));
    assert!(matches!(worker, Some(rhapsody::Error::Timeout)));
    device.wait(&point.into(), 10_000_000_000).unwrap();
    drop(pending);
    buffer.read_bytes(0, &mut bytes).unwrap();
    assert_eq!(bytes, [1; 4]);
    buffer
        .write_segments(&[(0, &[5, 6][..]), (100, &[7][..]), (50, &[][..])])
        .unwrap();
    let mut readback = [0u8; 101];
    buffer.read_bytes(0, &mut readback).unwrap();
    assert_eq!((readback[0], readback[1], readback[100]), (5, 6, 7));
    assert!(buffer.write_segments(&[(255, &[1, 2][..])]).is_err());
}
