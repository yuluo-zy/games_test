use super::*;
use crate::resource::buffer::BufferDesc;
use crate::tracking::OwnershipRange;
use crate::tracking::range_state::{Frontier, NativeState, RangeCell};
use crate::native::AccessScope;
use crate::sync::QueueId;

fn device() -> Option<Arc<Device>> {
    match crate::device::Device::new_headless(&crate::device::DeviceConfig::headless()) {
        Ok(device) => Some(device),
        Err(error) => {
            assert!(!std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|v| v == "1"), "{error}");
            None
        }
    }
}
/// CPU-only protocol fixture. Its symbolic source family is deliberately
/// different from the actual graphics family; it is NEVER issued to Vulkan.
fn released_buffer(device: &Arc<Device>, release: SyncPoint) -> (Arc<Buffer>, RangeTransfer) {
    released_buffer_with(device, release, vk::BufferUsageFlags::TRANSFER_DST)
}
/// The native buffer is real, but the release exists only in CPU metadata.
/// The empty submitted point tests dependency bookkeeping, not native QFOT.
pub(crate) fn released_buffer_with(
    device: &Arc<Device>,
    release: SyncPoint,
    usage: vk::BufferUsageFlags,
) -> (Arc<Buffer>, RangeTransfer) {
    let buffer =
        Arc::new(Buffer::new(device.clone(), &BufferDesc::new_gpu_only(256, usage)).unwrap());
    let range = OwnershipRange::Buffer {
        offset: 0,
        size: 256,
    };
    let write = AccessScope {
        stages: vk::PipelineStageFlags::TRANSFER,
        access: vk::AccessFlags::TRANSFER_WRITE,
        write: true,
    };
    let destination_family = device.queues.primary_family;
    let source_family = destination_family.checked_add(1).unwrap();
    let transfer = RangeTransfer {
        pair: 1,
        source_family,
        destination_family,
        range,
        source: write,
        destination: AccessScope {
            stages: vk::PipelineStageFlags::VERTEX_SHADER,
            access: vk::AccessFlags::SHADER_READ,
            write: false,
        },
        old_layout: vk::ImageLayout::UNDEFINED,
        new_layout: vk::ImageLayout::UNDEFINED,
    };
    let cell = buffer.last_use.clone();
    let stats = &device.memory.stats;
    cell.with_range_state(NativeState::default(), stats, |_| Ok(()))
        .unwrap();
    let _gate = cell.lock_submission();
    cell.prepare_range_state(
        &[
            StatePatch {
                range,
                expected_revision: 0,
                action: PatchAction::Access {
                    scope: write,
                    layout: vk::ImageLayout::UNDEFINED,
                    queue: release.queue,
                    family: source_family,
                },
            },
            StatePatch {
                range,
                expected_revision: 1,
                action: PatchAction::Release(transfer),
            },
        ],
        release.queue,
        1,
        stats,
    )
    .unwrap();
    cell.publish_range_state(release);
    cell.insert_overall(release);
    drop(_gate);
    (buffer, transfer)
}
/// Exercise the production CPU recorder without dispatching its synthetic
/// family indices to the driver. Scope contents and the command's virtual
/// state must describe exactly the same acquire for every retained range.
pub(crate) fn record_synthetic_acquire_scope(
    device: &Arc<Device>,
    command: &mut VulkanCommandBuffer,
) -> DrainedAcquires {
    let (drained, scope) = device.record_pending_acquire_scope(command, |_| false).unwrap();
    assert!(scope.memory.is_empty());
    assert!(scope.images.is_empty());
    assert_eq!(scope.buffers.len(), drained.len());
    for entry in &drained {
        let AcquireTarget::Buffer(weak) = &entry.target else { panic!("buffer fixture") };
        let buffer = weak.upgrade().unwrap();
        let barrier = scope.buffers.iter().find(|barrier| barrier.buffer == buffer.handle).unwrap();
        let OwnershipRange::Buffer { offset, size } = entry.transfer.range else { panic!("buffer range") };
        assert_eq!((barrier.offset, barrier.size), (offset, size));
        assert_eq!(barrier.src_queue_family_index, entry.transfer.source_family);
        assert_eq!(barrier.dst_queue_family_index, entry.transfer.destination_family);
        assert!(barrier.src_access_mask.is_empty());
        assert_eq!(barrier.dst_access_mask, entry.transfer.destination.access);
        assert!(scope.destination_stages.contains(entry.transfer.destination.stages));
        command.with_recorded_state(&buffer.last_use, NativeState::default(), entry.release_point.queue, |table| {
            // cells() is committed state even inside a preview. Only
            // prepared_cells() includes this command's unsubmitted patches.
            let prepared = table.prepared_cells().collect::<Vec<_>>();
            assert_eq!(table.domain(), entry.transfer.range,
                "whole-buffer fixture: entry={entry:?}, committed={:?}, prepared={prepared:?}", table.cells());
            assert_eq!(table.cells().len(), 1,
                "committed whole range: entry={entry:?}, committed={:?}, prepared={prepared:?}", table.cells());
            let committed = table.cells()[0];
            assert_eq!(committed.range, entry.transfer.range);
            assert_eq!(committed.state.owner, RangeOwner::Released {
                transfer: entry.transfer, point: entry.release_point,
            }, "recording must leave committed ownership released: committed={:?}, prepared={prepared:?}", table.cells());
            let mut writer = Frontier::EMPTY;
            writer.add(entry.release_point.queue, entry.transfer.destination);
            let expected = RangeCell {
                range: entry.transfer.range,
                state: NativeState {
                    owner: RangeOwner::Owned(command.pool.family),
                    layout: entry.transfer.new_layout,
                    layout_known: true,
                    writer,
                    readers: Frontier::EMPTY,
                    revision: committed.state.revision.checked_add(1).unwrap(),
                },
            };
            assert_eq!(prepared.as_slice(), &[expected],
                "exact acquire preview: entry={entry:?}, committed={:?}, prepared={prepared:?}", table.cells());
            assert_eq!(table.prepared_dependencies(), crate::sync::SyncSet::from_point(entry.release_point));
            assert_eq!(table.prepared_staged_dependencies().stages(entry.release_point.queue),
                entry.transfer.destination.stages);
            Ok(())
        }).unwrap();
        buffer.last_use.with_existing_range_state(|table| {
            assert_eq!(table.prepared_cells().count(), 0,
                "a preview must discard every unpublished scratch cell");
            Ok(())
        }).unwrap();
        assert_eq!(owner(&buffer), RangeOwner::Released {
            transfer: entry.transfer, point: entry.release_point,
        }, "recording must not publish the acquire before submission");
    }
    // Intentionally drop the CPU scope: it contains fictitious family
    // indices and is not a valid native release/acquire pair.
    drained
}

/// Commit only the recorded CPU patches in one otherwise empty, legal native
/// command. No synthetic ownership barrier ever reaches the Vulkan driver.
/// This is CPU protocol coverage, not real cross-family GPU coverage.
pub(crate) fn complete_synthetic_acquire_metadata(device: &Arc<Device>) -> usize {
    let queue = QueueId::new(0).unwrap();
    let before = device.queues.timelines.submitted(queue).unwrap();
    let pool = crate::command::command_buffer::VulkanCommandPool::new_for_family(
        device.clone(), device.queues.primary_family,
    ).unwrap();
    let mut command = pool.allocate().unwrap();
    command.begin().unwrap();
    let mut drained = record_synthetic_acquire_scope(device, &mut command);
    let count = drained.len();
    command.end().unwrap();
    command.submit().unwrap().wait(10_000_000_000).unwrap();
    assert_eq!(device.queues.timelines.submitted(queue).unwrap(), before + 1,
        "one CPU acquire batch must not create a submit per resource");
    for entry in &drained {
        let AcquireTarget::Buffer(weak) = &entry.target else { panic!("buffer fixture") };
        assert_eq!(owner(&weak.upgrade().unwrap()), RangeOwner::Owned(device.queues.primary_family));
    }
    drained.submitted();
    count
}

pub(crate) fn owner(buffer: &Buffer) -> RangeOwner {
    buffer
        .last_use
        .with_existing_range_state(|table| Ok(table.cells()[0].state.owner))
        .unwrap()
        .unwrap()
}

#[test]
fn synthetic_acquire_scope_then_native_graph_have_separate_submission_evidence() {
    use crate::rg::vulkan::{
        AccessTable, ResourceSpec, VulkanGraphBackend, VulkanGraphSession,
    };
    let Some(device) = device() else {
        eprintln!("SKIP: no Vulkan device");
        return;
    };
    let queue = QueueId::new(0).unwrap();
    // SAFETY: an empty batch only signals the device timeline.
    let release = unsafe { device.submit(queue, &[]) }.unwrap();
    let buffers = (0..3)
        .map(|_| released_buffer(&device, release))
        .collect::<Vec<_>>();
    for (buffer, transfer) in &buffers {
        device
            .queue_pending_acquire(&mut device.reserve_pending_acquires(1).unwrap(), PendingAcquire {
                target: AcquireTarget::buffer(buffer),
                transfer: *transfer,
                release_point: release,
            })
            .unwrap();
    }
    let family = device.queues.primary_family;
    assert_eq!(device.pending_acquire_count(family), 3);
    assert_eq!(device.pending_acquire_count(family + 1), 0);
    assert_eq!(complete_synthetic_acquire_metadata(&device), 3);
    let before_graph = device.queues.timelines.submitted(queue).unwrap();
    let backend = VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap();
    let mut session = VulkanGraphSession::new(backend);
    let mut graph = session.begin_frame().unwrap();
    let scratch = session
        .backend
        .declare(
            &mut graph,
            "scratch",
            ResourceSpec::Buffer(BufferDesc::new_gpu_only(
                64,
                vk::BufferUsageFlags::TRANSFER_DST,
            )),
        )
        .unwrap();
    let mut pass = graph.add_pass("fill").with_side_effect();
    pass.add_resource_access(scratch, 0x34, 0).unwrap();
    pass.render(|_| true);
    let receipt = session.execute_frame(graph.build()).unwrap();
    assert_eq!(receipt.stats.coalesced_acquires, 0,
        "synthetic acquires were inspected as CPU scopes, never native graph barriers");
    assert_eq!(device.queues.timelines.submitted(queue).unwrap() - before_graph,
        u64::from(receipt.stats.batches));
    assert_eq!(
        session.backend.submissions.len(),
        receipt.stats.batches as usize,
        "native same-family graph records only its own batches"
    );
    assert_eq!(device.pending_acquire_count(device.queues.primary_family), 0);
    session.wait_submits_done(10_000_000_000).unwrap();
    for (buffer, _) in &buffers {
        assert_eq!(owner(buffer), RangeOwner::Owned(device.queues.primary_family));
    }
    // Nothing left: the next frame records no acquire.
    let mut graph = session.begin_frame().unwrap();
    let scratch = session
        .backend
        .declare(
            &mut graph,
            "scratch",
            ResourceSpec::Buffer(BufferDesc::new_gpu_only(
                64,
                vk::BufferUsageFlags::TRANSFER_DST,
            )),
        )
        .unwrap();
    let mut pass = graph.add_pass("fill").with_side_effect();
    pass.add_resource_access(scratch, 0x34, 0).unwrap();
    pass.render(|_| true);
    let receipt = session.execute_frame(graph.build()).unwrap();
    assert_eq!(receipt.stats.coalesced_acquires, 0);
    session.wait_submits_done(10_000_000_000).unwrap();
}

#[test]
fn synthetic_cpu_scope_records_one_batch_and_unsubmitted_acquires_can_be_requeued() {
    let Some(device) = device() else {
        eprintln!("SKIP: no Vulkan device");
        return;
    };
    let queue = QueueId::new(0).unwrap();
    // SAFETY: an empty batch only signals the device timeline.
    let release = unsafe { device.submit(queue, &[]) }.unwrap();
    let (first, transfer) = released_buffer(&device, release);
    let (second, _) = released_buffer(&device, release);
    for buffer in [&first, &second] {
        device
            .queue_pending_acquire(&mut device.reserve_pending_acquires(1).unwrap(), PendingAcquire {
                target: AcquireTarget::buffer(buffer),
                transfer,
                release_point: release,
            })
            .unwrap();
    }
    // A dropped resource needs no acquire.
    let (dropped, _) = released_buffer(&device, release);
    device
        .queue_pending_acquire(&mut device.reserve_pending_acquires(1).unwrap(), PendingAcquire {
            target: AcquireTarget::buffer(&dropped),
            transfer,
            release_point: release,
        })
        .unwrap();
    drop(dropped);
    let pool = crate::command::command_buffer::VulkanCommandPool::new_for_family(device.clone(), device.queues.primary_family)
        .unwrap();
    let mut command = pool.allocate().unwrap();
    command.begin().unwrap();
    let drained = record_synthetic_acquire_scope(&device, &mut command);
    assert_eq!(drained.len(), 2);
    assert_eq!(device.pending_acquire_count(device.queues.primary_family), 0);
    // The command is abandoned: its acquires go back to the queue.
    drop(command);
    device.requeue_pending_acquires(drained);
    assert_eq!(device.pending_acquire_count(device.queues.primary_family), 2);
    let mut command = pool.allocate().unwrap();
    command.begin().unwrap();
    let mut drained = record_synthetic_acquire_scope(&device, &mut command);
    assert_eq!(drained.len(), 2);
    command.end().unwrap();
    command.submit().unwrap().wait(10_000_000_000).unwrap();
    drained.submitted();
    assert_eq!(owner(&first), RangeOwner::Owned(device.queues.primary_family));
    assert_eq!(owner(&second), RangeOwner::Owned(device.queues.primary_family));
    // Invalid registrations are rejected up front.
    let unsubmitted = SyncPoint {
        queue,
        value: release.value + 100,
    };
    assert!(
        device
            .queue_pending_acquire(&mut device.reserve_pending_acquires(1).unwrap(), PendingAcquire {
                target: AcquireTarget::buffer(&first),
                transfer,
                release_point: unsubmitted,
            })
            .is_err()
    );
}
