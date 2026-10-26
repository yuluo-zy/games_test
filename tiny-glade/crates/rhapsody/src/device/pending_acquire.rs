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

/// Physical backing is released before returning its real capacity credit.
#[derive(Default)]
struct AcquireStorage {
    entries: Vec<PendingAcquire>,
    _memory: Option<crate::device::CpuStorageReservation>,
}
impl AcquireStorage {
    fn new(budget: &Arc<crate::device::AllocatorStats>, capacity: usize) -> Result<Self> {
        if capacity == 0 { return Ok(Self::default()); }
        let mut memory = budget.reserve_cpu_storage(
            crate::raw::allocator::cpu_vec_bytes::<PendingAcquire>(capacity)?)?;
        let mut entries = Vec::new();
        entries.try_reserve_exact(capacity).map_err(|_| Error::OutOfHostMemory)?;
        memory.resize(crate::raw::allocator::cpu_vec_bytes::<PendingAcquire>(entries.capacity())?)?;
        Ok(Self { entries, _memory: Some(memory) })
    }
}
#[derive(Default)]
struct AcquireState {
    storage: AcquireStorage,
    producer_slots: usize,
    checked_out_slots: usize,
}
impl AcquireState {
    fn required(&self, extra: usize) -> Result<usize> {
        self.storage.entries.len().checked_add(self.producer_slots)
            .and_then(|value| value.checked_add(self.checked_out_slots))
            .and_then(|value| value.checked_add(extra))
            .ok_or_else(|| Error::invalid_usage("pending acquire slot capacity overflow"))
    }
    fn assert_slots(&self) {
        debug_assert!(self.required(0).is_ok_and(|needed| needed <= self.storage.entries.capacity()));
    }
}
struct AcquireQueue {
    state: parking_lot::Mutex<AcquireState>,
    count: AtomicUsize,
    budget: Arc<crate::device::AllocatorStats>,
    // No strong Device edge. All real storage/metadata precedes its credit.
    _memory: crate::device::CpuStorageReservation,
}
impl AcquireQueue {
    fn new(budget: Arc<crate::device::AllocatorStats>) -> Result<Arc<Self>> {
        let memory = budget.reserve_cpu_storage(
            crate::raw::allocator::shared_cpu_bytes::<Self>()?)?;
        Ok(Arc::new(Self { state: Default::default(), count: AtomicUsize::new(0),
            budget, _memory: memory }))
    }
    fn reserve(self: &Arc<Self>, count: usize) -> Result<PendingAcquireReservation> {
        self.trim_empty_if_pressure();
        let mut state = self.state.lock();
        let needed = state.required(count)?;
        // Allocate an independent replacement while the OLD backing/credit
        // remains alive. Thus both sides of a growth peak are actually billed.
        let old = if needed > state.storage.entries.capacity() {
            let target = needed.max(state.storage.entries.capacity().saturating_mul(2));
            let mut next = AcquireStorage::new(&self.budget, target)?;
            next.entries.append(&mut state.storage.entries);
            Some(std::mem::replace(&mut state.storage, next))
        } else { None };
        state.producer_slots += count;
        state.assert_slots();
        drop(state);
        drop(old); // Empty physical storage and its credit drop outside the lock.
        Ok(PendingAcquireReservation { queue: Some(self.clone()), remaining: count })
    }
    fn trim_empty_if_pressure(&self) -> u64 {
        if self.budget.budget_action() == crate::device::BudgetAction::None { return 0; }
        let old = {
            let mut state = self.state.lock();
            // No scan, wait or complete-drain: checked-out and producer slots
            // remain owners even when the visible pending list is empty.
            if state.required(0).is_ok_and(|needed| needed == 0) { Some(std::mem::take(&mut state.storage)) }
            else { None }
        };
        let released_cpu_bytes = old.as_ref().and_then(|storage| storage._memory.as_ref())
            .map_or(0, crate::device::CpuStorageReservation::bytes);
        drop(old); // Physical backing then CPU credit, outside the queue guard.
        released_cpu_bytes
    }
    fn drain(self: &Arc<Self>, family: u32) -> Result<(DrainedAcquires, DrainedAcquires)> {
            let mut state = self.state.lock();
            let capacity = state.storage.entries.len();
            // Both physical scratch owners are admitted before removing any
            // queued entry. Failure leaves the original queue untouched.
            let mut candidates = DrainedAcquires::with_capacity(self.clone(), capacity)?;
            let drained = DrainedAcquires::with_capacity(self.clone(), capacity)?;
            let mut index = 0;
            while index < state.storage.entries.len() {
                if state.storage.entries[index].transfer.destination_family == family {
                    let entry = state.storage.entries.swap_remove(index);
                    state.checked_out_slots += 1;
                    candidates.take_from_queue(entry);
                } else { index += 1; }
            }
            self.count.store(state.storage.entries.len(), Ordering::Release);
            state.assert_slots();
        Ok((candidates, drained))
    }
    fn publish(&self, reservation: &mut PendingAcquireReservation, acquire: PendingAcquire) -> Result<()> {
        if reservation.queue.as_ref().is_none_or(|queue| !std::ptr::eq(self, Arc::as_ptr(queue))) {
            return Err(Error::invalid_usage("pending acquire reservation belongs to another queue"));
        }
        if reservation.remaining == 0 {
            return Err(Error::invalid_usage("pending acquire reservation exhausted"));
        }
        let mut state = self.state.lock();
        state.assert_slots();
        assert!(state.storage.entries.len() < state.storage.entries.capacity());
        state.producer_slots -= 1;
        reservation.remaining -= 1;
        state.storage.entries.push(acquire); // Admitted before native submit: cannot grow.
        self.count.store(state.storage.entries.len(), Ordering::Release);
        state.assert_slots();
        Ok(())
    }
}

/// Exclusive pre-native-submit slots. Dropping an unused prefix cancels only
/// its own reservation. It cannot consume another producer's free capacity.
#[derive(Default)]
pub(crate) struct PendingAcquireReservation {
    queue: Option<Arc<AcquireQueue>>,
    remaining: usize,
}
impl Drop for PendingAcquireReservation {
    fn drop(&mut self) {
        if let Some(queue) = self.queue.take() {
            if self.remaining != 0 {
                let mut state = queue.state.lock();
                state.producer_slots -= self.remaining;
                self.remaining = 0;
                state.assert_slots();
            }
            queue.trim_empty_if_pressure();
            // Queue/entries/credit final destruction is after the guard above.
        }
    }
}

/// Empty/default queues allocate nothing. The first nonempty pre-submit
/// admission creates shared, budgeted state; leases keep only that state alive.
#[derive(Default)]
pub struct PendingAcquires {
    queue: std::sync::OnceLock<Arc<AcquireQueue>>,
}
impl PendingAcquires {
    /// O(1) maintenance of an already-created queue. The result is released
    /// CPU backing credit, not GPU bytes or a native retirement entry count.
    pub(crate) fn trim_empty_if_pressure(&self) -> u64 {
        self.queue.get().map_or(0, |queue| queue.trim_empty_if_pressure())
    }
    fn shared(&self, budget: &Arc<crate::device::AllocatorStats>) -> Result<Arc<AcquireQueue>> {
        if let Some(queue) = self.queue.get() {
            if !Arc::ptr_eq(&queue.budget, budget) {
                return Err(Error::invalid_usage("pending acquire queue belongs to another CPU budget"));
            }
            return Ok(queue.clone());
        }
        let candidate = match AcquireQueue::new(budget.clone()) {
            Ok(candidate) => candidate,
            Err(error) => return match self.queue.get() {
                Some(queue) if Arc::ptr_eq(&queue.budget, budget) => Ok(queue.clone()),
                Some(_) => Err(Error::invalid_usage("pending acquire queue belongs to another CPU budget")),
                None => Err(error),
            },
        };
        // Losing concurrent initialization drops its unused billed owner;
        // publication never changes the established device/queue identity.
        let _ = self.queue.set(candidate);
        let queue = self.queue.get().expect("set or existing acquire queue");
        if !Arc::ptr_eq(&queue.budget, budget) {
            return Err(Error::invalid_usage("pending acquire queue belongs to another CPU budget"));
        }
        Ok(queue.clone())
    }
    fn is_empty(&self) -> bool {
        self.queue.get().is_none_or(|queue| queue.count.load(Ordering::Acquire) == 0)
    }
}

/// Drained entries retain exclusive return slots until the actual native
/// command submits. Drop/cancellation returns them to their ORIGINAL queue
/// without allocating; the lease is deliberately not Clone.
#[derive(Default)]
pub struct DrainedAcquires {
    storage: AcquireStorage,
    queue: Option<Arc<AcquireQueue>>,
    slots: usize,
}
impl std::ops::Deref for DrainedAcquires {
    type Target = [PendingAcquire];
    fn deref(&self) -> &Self::Target { &self.storage.entries }
}
impl<'a> IntoIterator for &'a DrainedAcquires {
    type Item = &'a PendingAcquire;
    type IntoIter = std::slice::Iter<'a, PendingAcquire>;
    fn into_iter(self) -> Self::IntoIter { self.storage.entries.iter() }
}
impl std::fmt::Debug for DrainedAcquires {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(&self.storage.entries).finish()
    }
}
impl DrainedAcquires {
    fn with_capacity(queue: Arc<AcquireQueue>, capacity: usize) -> Result<Self> {
        let storage = AcquireStorage::new(&queue.budget, capacity)?;
        Ok(Self { storage, queue: Some(queue), slots: 0 })
    }
    fn take_from_queue(&mut self, acquire: PendingAcquire) {
        assert!(self.storage.entries.len() < self.storage.entries.capacity());
        self.storage.entries.push(acquire);
        self.slots += 1;
    }
    fn move_to(&mut self, index: usize, other: &mut Self) {
        assert!(Arc::ptr_eq(self.queue.as_ref().unwrap(), other.queue.as_ref().unwrap()));
        let item = self.storage.entries.swap_remove(index);
        self.slots -= 1;
        other.take_from_queue(item);
    }
    fn discard(&mut self, index: usize) {
        let item = self.storage.entries.swap_remove(index);
        self.slots -= 1;
        {
            let mut state = self.queue.as_ref().unwrap().state.lock();
            state.checked_out_slots -= 1;
            state.assert_slots();
        }
        drop(item); // Never release an acquire/Weak owner while holding the queue lock.
        self.queue.as_ref().unwrap().trim_empty_if_pressure();
    }
    /// Only call after the command's real successful submission proof.
    pub(crate) fn submitted(&mut self) {
        self.storage.entries.clear(); // Destruction is outside the queue lock.
        if let Some(queue) = self.queue.take() {
            let mut state = queue.state.lock();
            state.checked_out_slots -= self.slots;
            self.slots = 0;
            state.assert_slots();
            drop(state);
            queue.trim_empty_if_pressure();
        }
    }
}
impl Drop for DrainedAcquires {
    fn drop(&mut self) {
        if self.slots == 0 {
            debug_assert!(self.storage.entries.is_empty());
            return; // In particular: scratch-admission failure under the queue lock.
        }
        if let Some(queue) = self.queue.take() {
            assert_eq!(self.slots, self.storage.entries.len());
            let mut state = queue.state.lock();
            state.assert_slots();
            state.checked_out_slots -= self.slots;
            self.slots = 0;
            // Other producers cannot spend checked-out return capacity.
            assert!(state.storage.entries.capacity() - state.storage.entries.len() >= self.storage.entries.len());
            state.storage.entries.append(&mut self.storage.entries);
            queue.count.store(state.storage.entries.len(), Ordering::Release);
            state.assert_slots();
            drop(state);
        }
    }
}

impl Device {
    /// Register a released ownership range whose acquire on
    /// `transfer.destination_family` should be coalesced into the next
    /// submit on that family. The release must already be submitted
    /// (`release_point`) and recorded on the resource's native state.
    pub(crate) fn queue_pending_acquire(&self, reservation: &mut PendingAcquireReservation, acquire: PendingAcquire) -> Result<()> {
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
        let queue = self.queues.pending_acquires.queue.get()
            .ok_or_else(|| Error::invalid_usage("pending acquire reservation missing"))?;
        queue.publish(reservation, acquire)
    }
    /// Reserve room for `additional` registrations before submitting their
    /// releases, so that a host allocation failure cannot strand a range
    /// that was already released.
    pub(crate) fn reserve_pending_acquires(&self, additional: usize) -> Result<PendingAcquireReservation> {
        if additional == 0 { return Ok(PendingAcquireReservation::default()); }
        self.queues.pending_acquires.shared(&self.memory.stats)?.reserve(additional)
    }
    /// Pending acquires for queues of `family` (checked-out leases excluded).
    pub(crate) fn pending_acquire_count(&self, family: u32) -> usize {
        if self.queues.pending_acquires.is_empty() { return 0; }
        let queue = self.queues.pending_acquires.queue.get().expect("nonempty acquire queue");
        queue.state.lock().storage.entries.iter()
            .filter(|entry| entry.transfer.destination_family == family).count()
    }
    /// Cancellation always returns the non-Clone lease to its original queue.
    /// Its return slots have remained reserved, even across concurrent producers.
    pub(crate) fn requeue_pending_acquires(&self, drained: DrainedAcquires) {
        drop(drained);
    }
    /// Record every pending acquire for `command`'s queue family into one
    /// barrier batch of the recording `command`, with the matching state
    /// patches (their release points become waits at the acquire's
    /// destination stages when the command is submitted). Returns the
    /// recorded acquires; re-queue them if the command is never submitted.
    pub(crate) fn record_pending_acquires(
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
        let (drained, scope) = self.record_pending_acquire_scope(command, keep)?;
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

    // Pure recording projection shared by the native entry point and the
    // synthetic-family protocol tests. Draining, gates, reservations and patch
    // order are unchanged; only the caller decides whether to issue the scope.
    fn record_pending_acquire_scope(
        &self,
        command: &mut VulkanCommandBuffer,
        keep: impl Fn(&Arc<SyncCell>) -> bool,
    ) -> Result<(DrainedAcquires, crate::raw::barrier::SynchronizationScope)> {
        let Some(queue) = self.queues.pending_acquires.queue.get() else {
            return Ok((DrainedAcquires::default(), crate::raw::barrier::SynchronizationScope::new(
                vk::PipelineStageFlags::empty(), vk::PipelineStageFlags::empty())));
        };
        if queue.count.load(Ordering::Acquire) == 0 {
            return Ok((DrainedAcquires::default(), crate::raw::barrier::SynchronizationScope::new(
                vk::PipelineStageFlags::empty(), vk::PipelineStageFlags::empty())));
        }
        if !command.is_recording() {
            return Err(Error::invalid_usage("pending acquires need a recording command"));
        }
        let family = command.pool.family;
        let (mut candidates, mut drained) = queue.drain(family)?;
        // Resource gates and command metadata can allocate or wait. The shared
        // registration list is no longer locked while any of that work runs.
        let mut scope = crate::raw::barrier::SynchronizationScope::new(
            vk::PipelineStageFlags::empty(),
            vk::PipelineStageFlags::empty(),
        );
        let mut index = 0;
        let mut outcome = Ok(());
        while index < candidates.len() {
            let entry = &candidates[index];
            let Some(target) = entry.target.upgrade() else {
                candidates.discard(index);
                continue;
            };
            if keep(target.cell()) {
                index += 1;
                continue;
            }
            let revision = match released_revision(target.cell(), entry) {
                Ok(Some(revision)) => revision,
                Ok(None) => {
                    candidates.discard(index);
                    continue;
                }
                Err(error) => {
                    outcome = Err(error);
                    break;
                }
            };
            if let Err(error) = record_acquire(command, &target, entry, revision, &mut scope) {
                outcome = Err(error);
                break;
            }
            candidates.move_to(index, &mut drained);
        }
        if let Err(error) = outcome {
            // Each lease owns its original return slots. Neither cancellation
            // can allocate or spend another producer's reservation.
            drop(candidates);
            drop(drained);
            return Err(error);
        }
        if !candidates.is_empty() {
            self.requeue_pending_acquires(candidates);
        }
        Ok((drained, scope))
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
    use crate::resource::range_state::{AccessScope, Frontier, NativeState, RangeCell};
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
        let pool = crate::raw::command_buffer::VulkanCommandPool::new_for_family(
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
        let pool = crate::raw::command_buffer::VulkanCommandPool::new_for_family(device.clone(), device.queues.primary_family)
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
}

#[cfg(test)]
mod reservation_storage_tests {
    use super::*;
    use crate::resource::{OwnershipRange, range_state::AccessScope};
    use crate::sync::QueueId;

    // Metadata-only rows: no fake native handle or Vulkan queue-family work.
    fn entry(pair: u64, destination_family: u32) -> PendingAcquire {
        let scope = AccessScope { stages: vk::PipelineStageFlags::TRANSFER,
            access: vk::AccessFlags::TRANSFER_WRITE, write: true };
        PendingAcquire { target: AcquireTarget::Buffer(Weak::new()),
            transfer: RangeTransfer { pair, source_family: 0, destination_family,
                range: OwnershipRange::Buffer { offset: 0, size: 4 },
                source: scope, destination: scope,
                old_layout: vk::ImageLayout::UNDEFINED, new_layout: vk::ImageLayout::UNDEFINED },
            release_point: SyncPoint { queue: QueueId::new(0).unwrap(), value: 1 } }
    }
    fn pairs(queue: &AcquireQueue) -> Vec<u64> {
        let mut pairs = queue.state.lock().storage.entries.iter().map(|entry| entry.transfer.pair).collect::<Vec<_>>();
        pairs.sort_unstable(); pairs
    }

    #[test]
    fn concurrent_producers_keep_exclusive_pre_submit_slots_under_full_host_budget() {
        let stats = Arc::new(crate::device::AllocatorStats::default());
        let queue = AcquireQueue::new(stats.clone()).unwrap();
        let admitted = std::sync::Barrier::new(3);
        let publish = std::sync::Barrier::new(3);
        std::thread::scope(|scope| {
            for producer in 0..2 {
                let queue = &queue;
                let admitted = &admitted;
                let publish = &publish;
                scope.spawn(move || {
                    let mut ticket = queue.reserve(4).unwrap();
                    admitted.wait(); publish.wait();
                    for item in 0..4 { queue.publish(&mut ticket, entry(producer * 4 + item, 1)).unwrap(); }
                });
            }
            admitted.wait();
            {
                let state = queue.state.lock();
                assert_eq!(state.producer_slots, 8);
                assert_eq!(state.storage.entries.len(), 0);
                assert!(state.storage.entries.capacity() >= 8);
            }
            let usage = stats.cpu_memory_usage().total();
            stats.set_host_budget(usage).unwrap();
            publish.wait();
        });
        assert_eq!(pairs(&queue), (0..8).collect::<Vec<_>>());
        let state = queue.state.lock();
        assert_eq!((state.producer_slots, state.checked_out_slots), (0, 0));
        state.assert_slots(); drop(state);
        stats.set_host_budget(0).unwrap(); drop(queue);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }

    #[test]
    fn partial_filter_drain_cancel_after_another_producer_preserves_every_return_slot() {
        let stats = Arc::new(crate::device::AllocatorStats::default());
        let queue = AcquireQueue::new(stats.clone()).unwrap();
        let mut ticket = queue.reserve(6).unwrap();
        for pair in 0..6 { queue.publish(&mut ticket, entry(pair, if pair < 3 { 1 } else { 2 })).unwrap(); }
        drop(ticket);
        let (mut candidates, mut recorded) = queue.drain(1).unwrap();
        assert_eq!((candidates.len(), recorded.len()), (3, 0));
        assert_eq!(queue.state.lock().checked_out_slots, 3);
        let recorded_pair = candidates[0].transfer.pair;
        candidates.move_to(0, &mut recorded);
        let discarded = candidates[0].transfer.pair;
        candidates.discard(0); // Dead/stale owner needs no later acquire.
        drop(candidates); // Filtered/skipped row returns to its original queue.
        assert_eq!(queue.state.lock().checked_out_slots, 1);
        let mut concurrent = queue.reserve(2).unwrap();
        let before_cancel_capacity = queue.state.lock().storage.entries.capacity();
        let usage = stats.cpu_memory_usage().total();
        stats.set_host_budget(usage).unwrap();
        queue.publish(&mut concurrent, entry(6, 1)).unwrap();
        queue.publish(&mut concurrent, entry(7, 2)).unwrap();
        drop(concurrent);
        drop(recorded); // Cannot allocate or lose its prepaid return slot.
        let expected = (0..8).filter(|&pair| pair != discarded).collect::<Vec<_>>();
        assert_eq!(pairs(&queue), expected);
        assert!(pairs(&queue).contains(&recorded_pair));
        let state = queue.state.lock();
        assert_eq!(state.storage.entries.capacity(), before_cancel_capacity);
        assert_eq!((state.producer_slots, state.checked_out_slots), (0, 0));
        drop(state);
        stats.set_host_budget(0).unwrap(); drop(queue);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }

    #[test]
    fn failed_growth_keeps_old_storage_owners_and_exact_credit_then_recovers() {
        let stats = Arc::new(crate::device::AllocatorStats::default());
        let queue = AcquireQueue::new(stats.clone()).unwrap();
        let mut first = queue.reserve(4).unwrap();
        for pair in 0..4 { queue.publish(&mut first, entry(pair, 1)).unwrap(); }
        drop(first);
        let old_capacity = queue.state.lock().storage.entries.capacity();
        let old_usage = stats.cpu_memory_usage().total();
        stats.set_host_budget(old_usage).unwrap();
        assert!(matches!(queue.reserve(old_capacity + 1), Err(Error::OutOfHostMemory)));
        assert_eq!(pairs(&queue), vec![0, 1, 2, 3]);
        assert_eq!(queue.state.lock().storage.entries.capacity(), old_capacity);
        assert_eq!(queue.state.lock().producer_slots, 0);
        assert_eq!(stats.cpu_memory_usage().total(), old_usage);
        stats.set_host_budget(0).unwrap();
        let next = queue.reserve(old_capacity + 1).unwrap();
        assert!(queue.state.lock().storage.entries.capacity() >= old_capacity + 5);
        drop(next); drop(queue);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }

    #[test]
    fn second_drain_scratch_refusal_rolls_back_before_removal_without_drop_reentry_deadlock() {
        let stats = Arc::new(crate::device::AllocatorStats::default());
        let queue = AcquireQueue::new(stats.clone()).unwrap();
        let mut ticket = queue.reserve(4).unwrap();
        for pair in 0..4 { queue.publish(&mut ticket, entry(pair, 1)).unwrap(); }
        drop(ticket);
        let original = stats.cpu_memory_usage().total();
        let one_scratch = crate::raw::allocator::cpu_vec_bytes::<PendingAcquire>(4).unwrap();
        stats.set_host_budget(original + one_scratch).unwrap();
        assert!(matches!(queue.drain(1), Err(Error::OutOfHostMemory)));
        assert_eq!(pairs(&queue), vec![0, 1, 2, 3]);
        assert_eq!(queue.state.lock().checked_out_slots, 0);
        assert_eq!(stats.cpu_memory_usage().total(), original);
        stats.set_host_budget(0).unwrap();
        let (mut candidates, recorded) = queue.drain(1).unwrap();
        assert_eq!(candidates.len(), 4);
        candidates.submitted(); // Storage protocol commit; no GPU claim.
        drop(recorded); drop(candidates); drop(queue);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }

    #[test]
    fn wrong_queue_or_post_submit_registration_error_keeps_published_prefix_and_cancels_unused_credit() {
        let stats = Arc::new(crate::device::AllocatorStats::default());
        let first = AcquireQueue::new(stats.clone()).unwrap();
        let second = AcquireQueue::new(stats.clone()).unwrap();
        let mut ticket = first.reserve(2).unwrap();
        assert!(second.publish(&mut ticket, entry(99, 1)).is_err());
        assert!(pairs(&second).is_empty());
        assert_eq!(ticket.remaining, 2);
        first.publish(&mut ticket, entry(7, 1)).unwrap();
        // Model later fallible registration validation after a real submit;
        // the slot owner preserves the accepted prefix and unused credit.
        let registration_result: Result<()> = Err(Error::invalid_usage("injected later validation failure"));
        assert!(registration_result.is_err());
        drop(ticket);
        assert_eq!(pairs(&first), vec![7]);
        assert_eq!(first.state.lock().producer_slots, 0);
        let mut exhausted = first.reserve(1).unwrap();
        first.publish(&mut exhausted, entry(8, 1)).unwrap();
        assert!(first.publish(&mut exhausted, entry(9, 1)).is_err());
        assert_eq!(pairs(&first), vec![7, 8]);
        drop(exhausted); drop(first); drop(second);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }

    #[test]
    fn pressure_keeps_checked_out_return_capacity_until_cancel_requeues_it() {
        let stats = Arc::new(crate::device::AllocatorStats::default());
        let queue = AcquireQueue::new(stats.clone()).unwrap();
        let mut ticket = queue.reserve(4).unwrap();
        for pair in 0..4 { queue.publish(&mut ticket, entry(pair, 1)).unwrap(); }
        drop(ticket);
        let (candidates, recorded) = queue.drain(1).unwrap();
        let capacity = queue.state.lock().storage.entries.capacity();
        assert!(queue.state.lock().storage.entries.is_empty());
        assert_eq!(queue.state.lock().checked_out_slots, 4);
        stats.set_host_budget(stats.cpu_memory_usage().total()).unwrap();
        queue.trim_empty_if_pressure();
        assert_eq!(queue.state.lock().storage.entries.capacity(), capacity);
        drop(candidates);
        assert_eq!(pairs(&queue), vec![0, 1, 2, 3]);
        assert_eq!(queue.state.lock().checked_out_slots, 0);
        assert_eq!(queue.state.lock().storage.entries.capacity(), capacity);
        stats.set_host_budget(0).unwrap(); drop(recorded); drop(queue);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }

    #[test]
    fn pressure_trims_only_empty_unowned_capacity_and_recovers_under_the_same_host_limit() {
        let stats = Arc::new(crate::device::AllocatorStats::default());
        let queue = AcquireQueue::new(stats.clone()).unwrap();
        let ticket = queue.reserve(8).unwrap();
        let capacity = queue.state.lock().storage.entries.capacity();
        let backing = crate::raw::allocator::cpu_vec_bytes::<PendingAcquire>(capacity).unwrap();
        let before = stats.cpu_memory_usage().total();
        stats.set_host_budget(before).unwrap();
        queue.trim_empty_if_pressure();
        assert_eq!(queue.state.lock().storage.entries.capacity(), capacity,
            "an invisible producer reservation still owns its slots");
        drop(ticket);
        assert_eq!(queue.state.lock().storage.entries.capacity(), 0);
        assert_eq!(stats.cpu_memory_usage().total(), before - backing);
        let recovered = queue.reserve(4).unwrap();
        assert!(queue.state.lock().storage.entries.capacity() >= 4);
        assert_eq!(queue.state.lock().producer_slots, 4);
        assert!(stats.cpu_memory_usage().total() <= before,
            "recover using released actual backing credit without lifting the host limit");
        stats.set_host_budget(0).unwrap(); drop(recovered); drop(queue);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }

    #[test]
    fn existing_device_maintenance_reclaims_idle_queue_after_later_external_cpu_pressure() {
        let device = match Device::new_headless(&crate::device::DeviceConfig::headless()) {
            Ok(device) => device,
            Err(error) => {
                assert!(!std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|v| v == "1"), "{error}");
                return;
            }
        };
        let stats = &device.memory.stats;
        assert_eq!(stats.budget_action(), crate::device::BudgetAction::None);
        let ticket = device.reserve_pending_acquires(8).unwrap();
        drop(ticket); // No pressure yet: backing remains cached while empty.
        let queue = device.queues.pending_acquires.queue.get().unwrap();
        let backing = queue.state.lock().storage._memory.as_ref().unwrap().bytes();
        assert!(backing > 0);
        let idle_usage = stats.cpu_memory_usage().total();
        let external_size = usize::try_from(idle_usage).unwrap().checked_add(1024 * 1024).unwrap();
        let host_limit = idle_usage.checked_add(external_size as u64).unwrap();
        stats.set_host_budget(host_limit).unwrap();
        assert_eq!(stats.budget_action(), crate::device::BudgetAction::None);
        let external = crate::resource::CpuBytes::new(stats, external_size).unwrap();
        assert_ne!(stats.budget_action(), crate::device::BudgetAction::None);
        let pressured_usage = stats.cpu_memory_usage().total();
        let retirement = device.retirement_stats();
        assert_eq!(retirement.pending, 0);
        assert!(queue.state.lock().storage.entries.capacity() >= 8);
        // Existing maintenance reaches this empty queue without another
        // producer/release. Its native-retirement result is still exactly zero.
        assert_eq!(device.try_collect_garbage().unwrap(), 0);
        assert_eq!(queue.state.lock().storage.entries.capacity(), 0);
        assert_eq!(stats.cpu_memory_usage().total(), pressured_usage - backing);
        assert_eq!(stats.host_budget_limit(), host_limit);
        assert_eq!(device.retirement_stats().idle_waits, retirement.idle_waits);
        assert_eq!(device.retirement_stats().submitted, retirement.submitted);
        assert_eq!(device.retirement_stats().pending, 0);
        let recovered = crate::resource::CpuBytes::new(stats, usize::try_from(backing).unwrap()).unwrap();
        assert!(stats.cpu_memory_usage().total() <= host_limit);
        drop(recovered);
        drop(external);
        assert_eq!(stats.cpu_memory_usage().total(), idle_usage - backing);
        stats.set_host_budget(0).unwrap();
    }

    #[test]
    fn lazy_queue_admission_failure_and_cross_budget_access_do_not_publish_foreign_storage() {
        let stats = Arc::new(crate::device::AllocatorStats::default());
        let wrapper = PendingAcquires::default();
        assert!(wrapper.is_empty());
        stats.set_host_budget(1).unwrap();
        assert!(matches!(wrapper.shared(&stats), Err(Error::OutOfHostMemory)));
        assert!(wrapper.queue.get().is_none());
        assert_eq!(stats.cpu_memory_usage().total(), 0);
        stats.set_host_budget(0).unwrap();
        let queue = wrapper.shared(&stats).unwrap();
        let other = Arc::new(crate::device::AllocatorStats::default());
        assert!(wrapper.shared(&other).is_err());
        assert_eq!(other.cpu_memory_usage().total(), 0);
        drop(queue); drop(wrapper);
        assert_eq!(stats.cpu_memory_usage().total(), 0);
    }
}
