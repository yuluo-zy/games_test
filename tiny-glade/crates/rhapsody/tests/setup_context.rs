//! Reusable setup context: pooled command recycling, declared accesses and
//! opt-in waits.
mod gpu_test_support;
use ash::vk;
use rhapsody::{
    device::{Device, DeviceConfig},
    raw::{
        buffer::{Buffer, BufferDesc},
        setup::{SETUP_WAIT_TIMEOUT_NS, SetupAccess, with_setup_resources},
    },
};
use std::sync::Arc;

fn device() -> Option<Arc<Device>> {
    gpu_test_support::initialize(
        "setup context",
        Device::new_headless(&DeviceConfig {
            validation: true,
            ..DeviceConfig::headless()
        }),
    )
}

fn copy(
    device: &Arc<Device>,
    source: &Buffer,
    destination: &Buffer,
    offset: u64,
    size: u64,
) -> rhapsody::raw::setup::SetupSubmission<()> {
    with_setup_resources(
        device,
        &[
            SetupAccess::transfer_read(source, 0, size),
            SetupAccess::transfer_write(destination, offset, size),
        ],
        |command| unsafe {
            device.raw_device().cmd_copy_buffer(
                command,
                source.raw_handle(),
                destination.raw_handle(),
                &[vk::BufferCopy {
                    src_offset: 0,
                    dst_offset: offset,
                    size,
                }],
            );
        },
    )
    .unwrap()
}

#[test]
fn held_context_reuses_one_pool_and_recycles_commands_without_waiting() {
    let Some(device) = device() else {
        return;
    };
    let context = device.setup_context().unwrap();
    let destination = Buffer::new(
        device.clone(),
        &BufferDesc::new_gpu_to_cpu(
            32 * 16,
            vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::TRANSFER_SRC,
        ),
    )
    .unwrap();
    let mut points = Vec::new();
    for index in 0..32u8 {
        let source = Buffer::new(
            device.clone(),
            &BufferDesc::new_cpu_to_gpu(16, vk::BufferUsageFlags::TRANSFER_SRC),
        )
        .unwrap();
        source.write_bytes(0, &[index; 16]).unwrap();
        // The staging buffer drops right away; its storage retires after the
        // copy's own point because the declared access tracked it.
        points.push(copy(&device, &source, &destination, u64::from(index) * 16, 16).point);
    }
    // Successive copies to disjoint ranges are ordered by tracked state; the
    // host read waits for exactly the overlapping GPU writes.
    let mut bytes = vec![0u8; 32 * 16];
    destination.read_bytes(0, &mut bytes).unwrap();
    for (index, chunk) in bytes.chunks(16).enumerate() {
        assert!(chunk.iter().all(|&b| b == index as u8), "range {index}");
    }
    let stats = context.stats();
    assert_eq!(stats.pools_created, 1, "one reusable pool");
    assert_eq!(stats.submissions, 32);
    assert!(stats.commands_allocated <= 17, "{stats:?}");
    // Further work after completion recycles instead of allocating.
    device.wait_idle().unwrap();
    let before = context.stats();
    let source = Buffer::new(
        device.clone(),
        &BufferDesc::new_cpu_to_gpu(16, vk::BufferUsageFlags::TRANSFER_SRC),
    )
    .unwrap();
    source.write_bytes(0, &[0xAB; 16]).unwrap();
    copy(&device, &source, &destination, 0, 16)
        .wait(SETUP_WAIT_TIMEOUT_NS)
        .unwrap();
    let after = context.stats();
    assert_eq!(after.commands_allocated, before.commands_allocated);
    assert_eq!(after.commands_recycled, before.commands_recycled + 1);
    assert!(points.windows(2).all(|pair| pair[0].value < pair[1].value));
}

#[test]
fn nested_setup_calls_use_separate_pools_and_the_device_keeps_them() {
    let Some(device) = device() else {
        return;
    };
    let context = device.setup_context().unwrap();
    let buffer = Buffer::new(
        device.clone(),
        &BufferDesc::new_gpu_only(64, vk::BufferUsageFlags::TRANSFER_DST),
    )
    .unwrap();
    let inner = context
        .record(
            &[SetupAccess::transfer_write(&buffer, 0, 32)],
            |command| unsafe {
                device.raw_device().cmd_fill_buffer(
                    command.raw_handle(),
                    buffer.raw_handle(),
                    0,
                    32,
                    1,
                );
                // A nested call cannot deadlock on the outer pool.
                let nested =
                    context.record(&[SetupAccess::transfer_write(&buffer, 32, 32)], |nested| {
                        device.raw_device().cmd_fill_buffer(
                            nested.raw_handle(),
                            buffer.raw_handle(),
                            32,
                            32,
                            2,
                        );
                        Ok(())
                    })?;
                Ok(nested.point)
            },
        )
        .unwrap();
    inner.wait(SETUP_WAIT_TIMEOUT_NS).unwrap();
    assert_eq!(context.stats().pools_created, 2);
    // The pools belong to the device, not to the handle: dropping every
    // handle keeps them and later calls reuse them.
    drop(context);
    let fresh = device.setup_context().unwrap();
    assert_eq!(fresh.stats().pools_created, 2, "the device kept its pools");
    device.wait_idle().unwrap();
    fill(&device, &buffer, 0, 32, 3)
        .wait(SETUP_WAIT_TIMEOUT_NS)
        .unwrap();
    assert_eq!(fresh.stats().pools_created, 2, "reused, not recreated");
}

fn fill(
    device: &Arc<Device>,
    buffer: &Buffer,
    offset: u64,
    size: u64,
    value: u32,
) -> rhapsody::raw::setup::SetupSubmission<()> {
    with_setup_resources(
        device,
        &[SetupAccess::transfer_write(buffer, offset, size)],
        |command| unsafe {
            device
                .raw_device()
                .cmd_fill_buffer(command, buffer.raw_handle(), offset, size, value);
        },
    )
    .unwrap()
}

fn target(device: &Arc<Device>) -> Buffer {
    Buffer::new(
        device.clone(),
        &BufferDesc::new_gpu_only(1 << 20, vk::BufferUsageFlags::TRANSFER_DST),
    )
    .unwrap()
}

#[test]
fn consecutive_setup_calls_reuse_one_pool_without_any_caller_holding_the_context() {
    let Some(device) = device() else {
        return;
    };
    let buffer = target(&device);
    // No `SetupContext` or `Arc` to it is ever kept by the caller.
    for round in 0..12u32 {
        fill(&device, &buffer, 0, 4096, round)
            .wait(SETUP_WAIT_TIMEOUT_NS)
            .unwrap();
        let stats = device.setup_context().unwrap().stats();
        assert_eq!(
            stats.pools_created, 1,
            "round {round}: no new VkCommandPool"
        );
        assert_eq!(stats.submissions, u64::from(round) + 1);
    }
    let stats = device.setup_context().unwrap().stats();
    assert_eq!(stats.commands_allocated, 1, "{stats:?}");
    assert_eq!(stats.commands_recycled, 11, "{stats:?}");
    // The parked pools hold no device reference: no Arc cycle.
    assert_eq!(Arc::strong_count(&device), 1 + 1, "buffer owns one");
    drop(buffer);
    assert_eq!(Arc::strong_count(&device), 1);
}

#[test]
fn dropping_the_device_with_an_in_flight_setup_submission_is_clean() {
    let Some(device) = device() else {
        return;
    };
    let weak = Arc::downgrade(&device);
    {
        let buffer = target(&device);
        // Not waited: the command may still be queued when the device goes.
        for round in 0..4u32 {
            drop(fill(&device, &buffer, 0, 1 << 20, round));
        }
        assert!(device.setup_context().unwrap().stats().pools_created >= 1);
    }
    drop(device);
    assert!(
        weak.upgrade().is_none(),
        "parked setup pools must not keep the device alive"
    );
}
