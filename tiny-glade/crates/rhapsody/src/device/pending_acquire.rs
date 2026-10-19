//! Coalesced queue-family ownership acquires.
//!
//! A cross-family upload releases each written range on the transfer queue.
//! Instead of one acquire submit per job on the destination queue, producers
//! register the released range with [`Device::queue_pending_acquire`]. The
//! next render-graph batch on a queue of the destination family records every
//! pending acquire in one barrier batch at the start of its first command and
//! waits for each release point at exactly the acquire's destination stages,
//! so N jobs cost zero extra submits when a graph submit follows. Without a
//! graph submit, [`Device::record_pending_acquires`] records them into one
//! caller-owned command (at most one extra submit per flush).
use crate::raw::{
    buffer::Buffer, command_buffer::VulkanCommandBuffer, device::Device, texture::Texture,
};
use crate::resource::{
    SyncCell,
    range_state::{PatchAction, RangeOwner, RangeTransfer, StatePatch},
};
use crate::sync::SyncPoint;
use crate::{Error, Result};
use ash::vk;
use std::sync::{
    Arc, Weak,
    atomic::{AtomicUsize, Ordering},
};

/// The resource whose released range awaits its acquire. Held weakly: a
/// resource dropped before its acquire needs none (its destruction already
/// waits for the release through its last use).
#[derive(Clone, Debug)]
pub enum AcquireTarget {
    Buffer(Weak<Buffer>),
    Image(Weak<Texture>),
}
impl AcquireTarget {
    pub fn buffer(buffer: &Arc<Buffer>) -> Self {
        Self::Buffer(Arc::downgrade(buffer))
    }
    pub fn image(image: &Arc<Texture>) -> Self {
        Self::Image(Arc::downgrade(image))
    }
    fn upgrade(&self) -> Option<LiveTarget> {
        match self {
            Self::Buffer(buffer) => buffer.upgrade().map(LiveTarget::Buffer),
            Self::Image(image) => image.upgrade().map(LiveTarget::Image),
        }
    }
}
enum LiveTarget {
    Buffer(Arc<Buffer>),
    Image(Arc<Texture>),
}
impl LiveTarget {
    fn cell(&self) -> &Arc<SyncCell> {
        match self {
            Self::Buffer(buffer) => &buffer.last_use,
            Self::Image(image) => &image.last_use,
        }
    }
}

/// One released range: `transfer` exactly as recorded by its release and the
/// real submitted point of that release.
#[derive(Clone, Debug)]
pub struct PendingAcquire {
    pub target: AcquireTarget,
    pub transfer: RangeTransfer,
    pub release_point: SyncPoint,
}

/// Device-owned queue of pending acquires. The count makes the per-batch
/// check a single relaxed load when nothing is pending.
#[derive(Default)]
pub struct PendingAcquires {
    entries: parking_lot::Mutex<Vec<PendingAcquire>>,
    count: AtomicUsize,
}
impl PendingAcquires {
    fn push(&self, acquire: PendingAcquire) -> Result<()> {
        let mut entries = self.entries.lock();
        entries.try_reserve(1).map_err(|_| Error::OutOfHostMemory)?;
        entries.push(acquire);
        self.count.store(entries.len(), Ordering::Release);
        Ok(())
    }
    fn is_empty(&self) -> bool {
        self.count.load(Ordering::Acquire) == 0
    }
}

/// Acquires recorded into one command. Re-queue them with
/// [`Device::requeue_pending_acquires`] if that command is never submitted.
pub type DrainedAcquires = smallvec::SmallVec<[PendingAcquire; 4]>;

impl Device {
    /// Register a released ownership range whose acquire on
    /// `transfer.destination_family` should be coalesced into the next
    /// submit on that family. The release must already be submitted
    /// (`release_point`) and recorded on the resource's native state.
    pub fn queue_pending_acquire(&self, acquire: PendingAcquire) -> Result<()> {
        let transfer = acquire.transfer;
        if transfer.source_family == transfer.destination_family
            || acquire.release_point.value == 0
            || acquire.release_point.value
                > self
                    .queues
                    .timelines
                    .submitted(acquire.release_point.queue)?
        {
            return Err(Error::invalid_usage(
                "pending acquire needs a cross-family transfer with a submitted release",
            ));
        }
        if transfer.destination_family != self.queues.primary_family
            && !self
                .queues
                .additional
                .iter()
                .any(|queue| queue.family == transfer.destination_family)
        {
            return Err(Error::invalid_usage(
                "pending acquire destination family has no device queue",
            ));
        }
        self.queues.pending_acquires.push(acquire)
    }
    /// Reserve room for `additional` registrations before submitting their
    /// releases, so that a host allocation failure cannot strand a range
    /// that was already released.
    pub(crate) fn reserve_pending_acquires(&self, additional: usize) -> Result<()> {
        self.queues
            .pending_acquires
            .entries
            .lock()
            .try_reserve(additional)
            .map_err(|_| Error::OutOfHostMemory)
    }
    /// Pending acquires for queues of `family`.
    pub fn pending_acquire_count(&self, family: u32) -> usize {
        if self.queues.pending_acquires.is_empty() {
            return 0;
        }
        self.queues
            .pending_acquires
            .entries
            .lock()
            .iter()
            .filter(|entry| entry.transfer.destination_family == family)
            .count()
    }
    /// Put back acquires drained into a command that was never submitted.
    pub fn requeue_pending_acquires(&self, drained: DrainedAcquires) {
        let mut entries = self.queues.pending_acquires.entries.lock();
        if entries.try_reserve(drained.len()).is_ok() {
            entries.extend(drained);
        }
        // Without host memory the releases stay recorded on the resources;
        // a render graph using them still recovers through its own acquire.
        self.queues
            .pending_acquires
            .count
            .store(entries.len(), Ordering::Release);
    }
    /// Record every pending acquire for `command`'s queue family into one
    /// barrier batch of the recording `command`, with the matching state
    /// patches (their release points become waits at the acquire's
    /// destination stages when the command is submitted). Returns the
    /// recorded acquires; re-queue them if the command is never submitted.
    pub fn record_pending_acquires(
        &self,
        command: &mut VulkanCommandBuffer,
    ) -> Result<DrainedAcquires> {
        self.record_pending_acquires_except(command, |_| false)
    }
    /// As [`Self::record_pending_acquires`], leaving pending the acquires of
    /// resources for which `keep` returns true (e.g. used by the graph frame
    /// being recorded, whose planner acquires them itself).
    pub(crate) fn record_pending_acquires_except(
        &self,
        command: &mut VulkanCommandBuffer,
        keep: impl Fn(&Arc<SyncCell>) -> bool,
    ) -> Result<DrainedAcquires> {
        let mut drained = DrainedAcquires::new();
        if self.queues.pending_acquires.is_empty() {
            return Ok(drained);
        }
        if !command.is_recording() {
            return Err(Error::invalid_usage(
                "pending acquires need a recording command",
            ));
        }
        let family = command.pool.family;
        let mut entries = self.queues.pending_acquires.entries.lock();
        let mut scope = crate::raw::barrier::SynchronizationScope::new(
            vk::PipelineStageFlags::empty(),
            vk::PipelineStageFlags::empty(),
        );
        let mut index = 0;
        let mut outcome = Ok(());
        while index < entries.len() {
            let entry = &entries[index];
            if entry.transfer.destination_family != family {
                index += 1;
                continue;
            }
            let Some(target) = entry.target.upgrade() else {
                entries.swap_remove(index);
                continue;
            };
            if keep(target.cell()) {
                index += 1;
                continue;
            }
            let revision = match released_revision(target.cell(), entry) {
                Ok(Some(revision)) => revision,
                // Already acquired (by a graph planner) or superseded.
                Ok(None) => {
                    entries.swap_remove(index);
                    continue;
                }
                Err(error) => {
                    outcome = Err(error);
                    break;
                }
            };
            if let Err(error) = drained
                .try_reserve(1)
                .map_err(|_| Error::OutOfHostMemory)
                .and_then(|()| record_acquire(command, &target, entry, revision, &mut scope))
            {
                outcome = Err(error);
                break;
            }
            drained.push(entries.swap_remove(index));
        }
        if let Err(error) = outcome {
            // The command's recorded patches are discarded with it; keep
            // every acquire pending for a later command.
            entries.extend(drained);
            self.queues
                .pending_acquires
                .count
                .store(entries.len(), Ordering::Release);
            return Err(error);
        }
        self.queues
            .pending_acquires
            .count
            .store(entries.len(), Ordering::Release);
        drop(entries);
        if !drained.is_empty() {
            // SAFETY: `command` is recording on this device; the barrier
            // covers exactly the ranges whose Acquire patches were recorded.
            let issued = unsafe { self.issue_barriers(command.handle, &scope) };
            if let Err(error) = issued {
                self.requeue_pending_acquires(drained);
                return Err(error);
            }
        }
        Ok(drained)
    }
}

/// The revision of the still-released range, or `None` once the resource no
/// longer holds exactly this release.
fn released_revision(cell: &Arc<SyncCell>, entry: &PendingAcquire) -> Result<Option<u64>> {
    let expected = RangeOwner::Released {
        transfer: entry.transfer,
        point: entry.release_point,
    };
    Ok(cell
        .with_existing_range_state(|table| {
            let mut revision = None;
            for current in table.cells() {
                if crate::resource::range_state::intersection(current.range, entry.transfer.range)
                    .is_none()
                {
                    continue;
                }
                if current.state.owner != expected
                    || revision.is_some_and(|value| value != current.state.revision)
                {
                    return Ok(None);
                }
                revision = Some(current.state.revision);
            }
            Ok(revision)
        })?
        .flatten())
}

fn record_acquire(
    command: &mut VulkanCommandBuffer,
    target: &LiveTarget,
    entry: &PendingAcquire,
    revision: u64,
    scope: &mut crate::raw::barrier::SynchronizationScope,
) -> Result<()> {
    let transfer = entry.transfer;
    command.record_state_patch(
        target.cell().clone(),
        StatePatch {
            range: transfer.range,
            expected_revision: revision,
            action: PatchAction::Acquire {
                transfer,
                release_point: entry.release_point,
            },
        },
    )?;
    // Execution-only source at the destination stages: the acquire chains
    // with the release wait, which blocks exactly those stages.
    scope.source_stages |= transfer.destination.stages;
    scope.destination_stages |= transfer.destination.stages;
    use crate::raw::barrier::{BarrierTarget, OwnershipHalf, RangeBarrier};
    // SAFETY: the acquire touches exactly the patched range.
    let target = match target {
        LiveTarget::Buffer(buffer) => {
            unsafe { command.retain_recorded(buffer.clone())? };
            BarrierTarget::Buffer(buffer.handle)
        }
        LiveTarget::Image(image) => {
            unsafe { command.retain_recorded(image.clone())? };
            BarrierTarget::Image(image.handle)
        }
    };
    scope.push(
        target,
        &RangeBarrier::ownership(&transfer, OwnershipHalf::Acquire),
    )
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::raw::buffer::BufferDesc;
    use crate::resource::OwnershipRange;
    use crate::resource::range_state::{AccessScope, NativeState};
    use crate::sync::QueueId;

    fn device() -> Option<Arc<Device>> {
        crate::device::Device::new_headless(&crate::device::DeviceConfig::headless()).ok()
    }
    /// A buffer whose whole range was released from (simulated) family 1 to
    /// family 0 by a submitted release.
    fn released_buffer(device: &Arc<Device>, release: SyncPoint) -> (Arc<Buffer>, RangeTransfer) {
        released_buffer_with(device, release, vk::BufferUsageFlags::TRANSFER_DST)
    }
    /// A 256-byte buffer of `usage` whose whole range was released from
    /// (simulated) family 1 to family 0 by the submitted `release`.
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
        let transfer = RangeTransfer {
            pair: 1,
            source_family: 1,
            destination_family: 0,
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
                        family: 1,
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
    pub(crate) fn owner(buffer: &Buffer) -> RangeOwner {
        buffer
            .last_use
            .with_existing_range_state(|table| Ok(table.cells()[0].state.owner))
            .unwrap()
            .unwrap()
    }

    #[test]
    fn graph_batch_coalesces_every_pending_acquire_without_an_extra_submit() {
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
                .queue_pending_acquire(PendingAcquire {
                    target: AcquireTarget::buffer(buffer),
                    transfer: *transfer,
                    release_point: release,
                })
                .unwrap();
        }
        assert_eq!(device.pending_acquire_count(0), 3);
        assert_eq!(device.pending_acquire_count(1), 0);
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
        assert_eq!(receipt.stats.coalesced_acquires, 3);
        assert_eq!(
            session.backend.submissions.len(),
            receipt.stats.batches as usize,
            "acquires ride in the graph batch: no separate acquire submit"
        );
        assert_eq!(device.pending_acquire_count(0), 0);
        session.wait_submits_done(10_000_000_000).unwrap();
        for (buffer, _) in &buffers {
            assert_eq!(owner(buffer), RangeOwner::Owned(0));
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
    fn explicit_flush_records_one_batch_and_unsubmitted_acquires_can_be_requeued() {
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
                .queue_pending_acquire(PendingAcquire {
                    target: AcquireTarget::buffer(buffer),
                    transfer,
                    release_point: release,
                })
                .unwrap();
        }
        // A dropped resource needs no acquire.
        let (dropped, _) = released_buffer(&device, release);
        device
            .queue_pending_acquire(PendingAcquire {
                target: AcquireTarget::buffer(&dropped),
                transfer,
                release_point: release,
            })
            .unwrap();
        drop(dropped);
        let pool = crate::raw::command_buffer::VulkanCommandPool::new_for_family(device.clone(), 0)
            .unwrap();
        let mut command = pool.allocate().unwrap();
        command.begin().unwrap();
        let drained = device.record_pending_acquires(&mut command).unwrap();
        assert_eq!(drained.len(), 2);
        assert_eq!(device.pending_acquire_count(0), 0);
        // The command is abandoned: its acquires go back to the queue.
        drop(command);
        device.requeue_pending_acquires(drained);
        assert_eq!(device.pending_acquire_count(0), 2);
        let mut command = pool.allocate().unwrap();
        command.begin().unwrap();
        assert_eq!(
            device.record_pending_acquires(&mut command).unwrap().len(),
            2
        );
        command.end().unwrap();
        command.submit().unwrap().wait(10_000_000_000).unwrap();
        assert_eq!(owner(&first), RangeOwner::Owned(0));
        assert_eq!(owner(&second), RangeOwner::Owned(0));
        // Invalid registrations are rejected up front.
        let unsubmitted = SyncPoint {
            queue,
            value: release.value + 100,
        };
        assert!(
            device
                .queue_pending_acquire(PendingAcquire {
                    target: AcquireTarget::buffer(&first),
                    transfer,
                    release_point: unsubmitted,
                })
                .is_err()
        );
    }
}
