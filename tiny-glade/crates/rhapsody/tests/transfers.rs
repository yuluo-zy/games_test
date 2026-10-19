mod gpu_test_support;
use ash::vk;
use rhapsody::{
    device::{Device, DeviceConfig},
    raw::{
        buffer::{Buffer, BufferDesc},
        command_buffer::VulkanCommandPool,
        semaphore::TimelineSemaphore,
    },
    rg::{
        resource::AccessRange,
        vulkan::{AccessTable, VulkanGraphBackend, VulkanGraphSession},
    },
    transfer::{TransferConfig, TransferManager},
};
use std::sync::Arc;

fn device() -> Option<Arc<Device>> {
    gpu_test_support::initialize(
        "buffer transfer device",
        Device::new_headless(&DeviceConfig::headless()),
    )
}

#[test]
fn bounded_async_uploads_feed_the_graph_and_readbacks_keep_their_slots_leased() {
    let Some(device) = device() else {
        return;
    };
    let mut transfers = TransferManager::new(
        device.clone(),
        TransferConfig {
            slots: 2,
            chunk_bytes: 1024,
            max_bytes: 16384,
        },
    )
    .unwrap();
    let target = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_only(
                2048,
                vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST,
            ),
        )
        .unwrap(),
    );
    let expected = (0..768).map(|i| ((i * 17) % 251) as u8).collect::<Vec<_>>();
    let uploaded = transfers
        .upload_buffer(target.clone(), 128, &expected)
        .unwrap();
    let mut session = VulkanGraphSession::new(
        VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap(),
    );
    let mut graph = session.begin_frame().unwrap();
    let id = uploaded
        .import_buffer(&mut graph, &mut session.backend, "uploaded input")
        .unwrap();
    let mut pass = graph.add_pass("consume uploaded range").with_side_effect();
    pass.read_buffer(
        id,
        0x25,
        AccessRange::Buffer {
            offset: 128,
            size: expected.len() as u64,
        },
    )
    .unwrap();
    pass.render(|_| true);
    let frame = session.execute_frame(graph.build()).unwrap();
    let first = transfers
        .read_buffer(target.clone(), 128, expected.len())
        .unwrap();
    let second = transfers
        .read_buffer(target.clone(), 128, expected.len())
        .unwrap();
    assert!(matches!(
        transfers.read_buffer(target.clone(), 128, expected.len()),
        Err(rhapsody::Error::Timeout)
    ));
    first.wait(10_000_000_000).unwrap();
    let mut actual = vec![0; expected.len()];
    assert_eq!(first.try_read(&mut actual).unwrap(), Some(expected.len()));
    assert_eq!(actual, expected);
    assert_eq!(transfers.stats().unwrap().leased_readbacks, 2);
    let clone = first.clone();
    drop(first);
    assert!(matches!(
        transfers.read_buffer(target.clone(), 128, expected.len()),
        Err(rhapsody::Error::Timeout)
    ));
    drop(clone);
    let third = transfers.read_buffer(target, 128, expected.len()).unwrap();
    third.wait(10_000_000_000).unwrap();
    assert_eq!(third.try_read(&mut actual).unwrap(), Some(expected.len()));
    assert_eq!(actual, expected);
    second.wait(10_000_000_000).unwrap();
    session.wait_frame(frame.frame, 10_000_000_000).unwrap();
    drop((second, third));
    let stats = transfers.stats().unwrap();
    assert_eq!(stats.leased_readbacks, 0);
    assert_eq!(stats.capacity_stalls, 2);
    assert!(stats.committed_bytes <= 16384);
}

struct ReleaseWait(Arc<TimelineSemaphore>, bool);
impl ReleaseWait {
    fn signal(&mut self) {
        if !self.1 {
            unsafe {
                self.0
                    .device()
                    .raw_device()
                    .signal_semaphore(
                        &vk::SemaphoreSignalInfo::default()
                            .semaphore(self.0.raw_handle())
                            .value(1),
                    )
                    .unwrap();
            }
            self.1 = true;
        }
    }
}
impl Drop for ReleaseWait {
    fn drop(&mut self) {
        self.signal();
    }
}

#[test]
fn nonblocking_host_writes_reject_in_flight_use_and_recover_after_completion() {
    let Some(device) = device() else {
        return;
    };
    let target = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_cpu_to_gpu(
                64,
                vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::TRANSFER_SRC,
            ),
        )
        .unwrap(),
    );
    let mut transfers = TransferManager::new(
        device.clone(),
        TransferConfig {
            slots: 1,
            chunk_bytes: 64,
            max_bytes: 8192,
        },
    )
    .unwrap();
    let idle_waits = device.retirement_stats().idle_waits;
    let blocker = TimelineSemaphore::new(device.clone(), 0).unwrap();
    let mut release = ReleaseWait(blocker.clone(), false);
    let pool = VulkanCommandPool::new(device.clone()).unwrap();
    let mut command = pool.allocate().unwrap();
    command.retain_opaque(target.clone()).unwrap();
    command.begin().unwrap();
    unsafe {
        device.raw_device().cmd_fill_buffer(
            command.raw_handle(),
            target.raw_handle(),
            0,
            64,
            0x01010101,
        );
    }
    command.end().unwrap();
    let pending = command
        .submit_to(
            rhapsody::raw::device::DeviceQueue {
                handle: device.primary_queue(),
                family: device.queue_family_index(),
                index: 0,
            },
            &[(blocker, 1, vk::PipelineStageFlags::ALL_COMMANDS)],
            &[],
        )
        .unwrap();
    let rejected = target.write_bytes_unsynchronized(0, &[2; 64]);
    let readback = transfers.read_buffer(target.clone(), 0, 64).unwrap();
    let mut actual = [0; 64];
    assert_eq!(readback.try_read(&mut actual).unwrap(), None);
    release.signal();
    pending.wait(10_000_000_000).unwrap();
    readback.wait(10_000_000_000).unwrap();
    assert_eq!(readback.try_read(&mut actual).unwrap(), Some(64));
    assert_eq!(actual, [1; 64]);
    assert!(matches!(rejected, Err(rhapsody::Error::Timeout)));
    target.write_bytes_unsynchronized(0, &[2; 64]).unwrap();
    target.read_bytes(0, &mut actual).unwrap();
    assert_eq!(actual, [2; 64]);
    assert_eq!(device.retirement_stats().idle_waits, idle_waits);
}

#[test]
fn queued_dirty_uploads_merge_exact_bytes_cancel_old_epochs_and_prioritize_other_targets() {
    use rhapsody::transfer::{
        UploadCancellation, UploadKey, UploadOptions, UploadPriority, UploadStatus,
    };
    let Some(device) = device() else { return };
    let mut manager = TransferManager::new(
        device.clone(),
        TransferConfig {
            slots: 1,
            chunk_bytes: 256,
            max_bytes: 16384,
        },
    )
    .unwrap();
    let make = || {
        Arc::new(
            Buffer::new(
                device.clone(),
                &BufferDesc::new_gpu_only(
                    256,
                    vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST,
                ),
            )
            .unwrap(),
        )
    };
    let a = make();
    let b = make();
    let idle = device.retirement_stats().idle_waits;
    manager
        .upload_buffer(a.clone(), 0, &[7; 256])
        .unwrap()
        .wait(5_000_000_000)
        .unwrap();
    let opts = |epoch, priority| UploadOptions {
        key: UploadKey(11),
        epoch,
        priority,
    };
    let obsolete = manager
        .enqueue_upload(a.clone(), 0, &[1; 8], opts(1, UploadPriority::Normal))
        .unwrap();
    let first = manager
        .enqueue_upload(a.clone(), 0, &[2; 8], opts(2, UploadPriority::Normal))
        .unwrap();
    let overlap = manager
        .enqueue_upload(a.clone(), 4, &[3; 8], opts(2, UploadPriority::Normal))
        .unwrap();
    let separate = manager
        .enqueue_upload(a.clone(), 20, &[4; 4], opts(2, UploadPriority::Normal))
        .unwrap();
    let high = manager
        .enqueue_upload(b.clone(), 0, &[9; 4], opts(1, UploadPriority::High))
        .unwrap();
    assert!(matches!(
        obsolete.status(),
        UploadStatus::Cancelled(UploadCancellation::Superseded)
    ));
    assert!(
        manager
            .enqueue_upload(a.clone(), 0, &[5; 4], opts(1, UploadPriority::High))
            .is_err()
    );
    let flush = manager.flush_uploads(1).unwrap();
    assert_eq!(
        (
            flush.submitted_jobs,
            flush.submitted_bytes,
            flush.remaining_jobs
        ),
        (1, 4, 1)
    );
    assert!(first.try_submitted().unwrap().is_none());
    let high = high.try_submitted().unwrap().unwrap();
    high.wait(5_000_000_000).unwrap();
    assert_eq!(manager.flush_uploads(1).unwrap().submitted_jobs, 1);
    let first = first.try_submitted().unwrap().unwrap();
    assert_eq!(overlap.try_submitted().unwrap().unwrap().point, first.point);
    assert_eq!(
        separate.try_submitted().unwrap().unwrap().point,
        first.point
    );
    assert_eq!(first.offset, 0);
    assert_eq!(first.bytes, 8);
    first.wait(5_000_000_000).unwrap();
    let read = manager.read_buffer(a, 0, 256).unwrap();
    read.wait(5_000_000_000).unwrap();
    let mut output = [0; 256];
    read.try_read(&mut output).unwrap().unwrap();
    let mut expected = [7; 256];
    expected[..8].fill(2);
    expected[4..12].fill(3);
    expected[20..24].fill(4);
    assert_eq!(
        output, expected,
        "untouched gaps must retain their previous GPU contents"
    );
    let stats = manager.upload_queue_stats();
    assert_eq!(stats.submitted_jobs, 2);
    assert_eq!(stats.merged_requests, 2);
    assert_eq!(stats.cancelled_jobs, 1);
    assert_eq!(stats.queued_bytes, 0);
    assert!(stats.known_cpu_bytes <= 32 << 20);
    assert_eq!(device.retirement_stats().idle_waits, idle);
}

#[test]
fn queued_upload_full_staging_defers_without_publishing_then_recovers_after_real_completion() {
    use rhapsody::transfer::{UploadKey, UploadOptions, UploadPriority, UploadStatus};
    let Some(device) = device() else { return };
    let mut manager = TransferManager::new(
        device.clone(),
        TransferConfig {
            slots: 1,
            chunk_bytes: 256,
            max_bytes: 16384,
        },
    )
    .unwrap();
    let target = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_only(
                256,
                vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST,
            ),
        )
        .unwrap(),
    );
    let semaphore = TimelineSemaphore::new(device.clone(), 0).unwrap();
    let mut release = ReleaseWait(semaphore.clone(), false);
    let pool = VulkanCommandPool::new(device.clone()).unwrap();
    let mut blocked = pool.allocate().unwrap();
    blocked.retain_opaque(target.clone()).unwrap();
    blocked.begin().unwrap();
    blocked.end().unwrap();
    let blocked = blocked
        .submit_to(
            rhapsody::raw::device::DeviceQueue {
                handle: device.primary_queue(),
                family: device.queue_family_index(),
                index: 0,
            },
            &[(semaphore, 1, vk::PipelineStageFlags::ALL_COMMANDS)],
            &[],
        )
        .unwrap();
    let initial = manager.upload_buffer(target.clone(), 0, &[1; 256]).unwrap();
    let queued = manager
        .enqueue_upload(
            target.clone(),
            0,
            &[2; 256],
            UploadOptions {
                key: UploadKey(7),
                epoch: 1,
                priority: UploadPriority::High,
            },
        )
        .unwrap();
    let flush = manager.flush_uploads(1).unwrap();
    assert!(flush.capacity_deferred);
    assert_eq!(flush.submitted_jobs, 0);
    assert!(matches!(queued.status(), UploadStatus::Pending));
    release.signal();
    blocked.wait(5_000_000_000).unwrap();
    initial.wait(5_000_000_000).unwrap();
    assert_eq!(manager.flush_uploads(1).unwrap().submitted_jobs, 1);
    let ticket = queued.try_submitted().unwrap().unwrap();
    ticket.wait(5_000_000_000).unwrap();
    assert!(
        !manager.cancel_upload(&queued).unwrap(),
        "submitted work cannot be falsely cancelled"
    );
    let read = manager.read_buffer(target, 0, 256).unwrap();
    read.wait(5_000_000_000).unwrap();
    let mut output = [0; 256];
    read.try_read(&mut output).unwrap().unwrap();
    assert_eq!(output, [2; 256]);
}

#[test]
fn lost_device_fails_all_accepted_upload_tickets_without_any_completion_point() {
    use rhapsody::transfer::{UploadKey, UploadOptions, UploadPriority, UploadStatus};
    let Some(device) = device() else { return };
    let mut manager = TransferManager::new(
        device.clone(),
        TransferConfig {
            slots: 1,
            chunk_bytes: 256,
            max_bytes: 16384,
        },
    )
    .unwrap();
    let target = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_only(256, vk::BufferUsageFlags::TRANSFER_DST),
        )
        .unwrap(),
    );
    let first = manager
        .enqueue_upload(
            target.clone(),
            0,
            &[1; 4],
            UploadOptions {
                key: UploadKey(1),
                epoch: 1,
                priority: UploadPriority::Normal,
            },
        )
        .unwrap();
    let second = manager
        .enqueue_upload(
            target.clone(),
            4,
            &[2; 4],
            UploadOptions {
                key: UploadKey(2),
                epoch: 1,
                priority: UploadPriority::High,
            },
        )
        .unwrap();
    device.timelines().mark_device_lost(); // Idle native device; explicit protocol-failure injection.
    assert!(matches!(
        manager.flush_uploads(4),
        Err(rhapsody::Error::DeviceLost)
    ));
    for ticket in [first, second] {
        assert!(
            matches!(ticket.status(), UploadStatus::Failed(error) if matches!(error.error(), rhapsody::Error::DeviceLost))
        );
        assert!(matches!(
            ticket.try_submitted(),
            Err(rhapsody::Error::DeviceLost)
        ));
    }
    assert_eq!(manager.upload_queue_stats().submitted_jobs, 0);
    assert!(matches!(
        manager.enqueue_upload(
            target,
            0,
            &[3; 4],
            UploadOptions {
                key: UploadKey(3),
                epoch: 1,
                priority: UploadPriority::High,
            }
        ),
        Err(rhapsody::Error::DeviceLost)
    ));
}

#[test]
fn readback_envelopes_reuse_capacity_and_escaping_tickets_keep_their_actual_cpu_budget() {
    let Some(device) = device() else { return };
    let mut manager = TransferManager::new(
        device.clone(),
        TransferConfig {
            slots: 1,
            chunk_bytes: 256,
            max_bytes: 16384,
        },
    )
    .unwrap();
    let target = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_only(
                256,
                vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST,
            ),
        )
        .unwrap(),
    );
    manager
        .upload_buffer(target.clone(), 0, &[0x31; 256])
        .unwrap()
        .wait(5_000_000_000)
        .unwrap();
    let mut baseline = None;
    for _ in 0..64 {
        let read = manager.read_buffer(target.clone(), 0, 256).unwrap();
        read.wait(5_000_000_000).unwrap();
        let mut output = [0; 256];
        read.try_read(&mut output).unwrap().unwrap();
        assert_eq!(output, [0x31; 256]);
        drop(read);
        device.try_collect_all_completed().unwrap();
        let bytes = device.allocator_stats().cpu_memory_usage().total();
        if let Some(expected) = baseline {
            assert_eq!(bytes, expected, "fixed readback capacity must not grow");
        } else {
            baseline = Some(bytes);
        }
    }
    let escaped = manager.read_buffer(target, 0, 256).unwrap();
    escaped.wait(5_000_000_000).unwrap();
    drop(manager);
    device.try_collect_all_completed().unwrap();
    let retained = device.allocator_stats().cpu_memory_usage().total();
    let mut output = [0; 256];
    escaped.try_read(&mut output).unwrap().unwrap();
    assert_eq!(output, [0x31; 256]);
    drop(escaped);
    device.try_collect_all_completed().unwrap();
    assert!(
        device.allocator_stats().cpu_memory_usage().total() < retained,
        "escaped lease CPU credit must follow its last actual owner"
    );
}

#[test]
fn overwriting_a_range_read_by_earlier_gpu_work_is_ordered_by_tracked_state() {
    let Some(device) = device() else {
        return;
    };
    let mut manager = TransferManager::new(
        device.clone(),
        TransferConfig {
            slots: 2,
            chunk_bytes: 256,
            max_bytes: 4096,
        },
    )
    .unwrap();
    let target = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_only(
                256,
                vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST,
            ),
        )
        .unwrap(),
    );
    let mut previous = None;
    for round in 0..4u8 {
        let bytes = [round + 1; 64];
        manager.upload_buffer(target.clone(), 32, &bytes).unwrap();
        // The readback reads the range; the next upload must order after it.
        let readback = manager.read_buffer(target.clone(), 32, 64).unwrap();
        readback.wait(10_000_000_000).unwrap();
        let mut output = [0; 64];
        assert_eq!(readback.try_read(&mut output).unwrap(), Some(64));
        assert_eq!(output, bytes);
        assert_eq!(readback.len(), 64);
        previous = Some(readback);
    }
    drop(previous);
    assert_eq!(manager.stats().unwrap().pending_uploads, 0);
}
