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
use crate::device::Device;
use crate::command::command_buffer::VulkanCommandBuffer;
use crate::resource::buffer::Buffer;
use crate::resource::texture::Texture;
use crate::tracking::SyncCell;
use crate::tracking::range_state::{PatchAction, RangeOwner, RangeTransfer, StatePatch};
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
    _memory: Option<crate::accounting::CpuStorageReservation>,
}
impl AcquireStorage {
    fn new(budget: &Arc<crate::accounting::AllocatorStats>, capacity: usize) -> Result<Self> {
        if capacity == 0 { return Ok(Self::default()); }
        let mut memory = budget.reserve_cpu_storage(
            crate::accounting::cpu_vec_bytes::<PendingAcquire>(capacity)?)?;
        let mut entries = Vec::new();
        entries.try_reserve_exact(capacity).map_err(|_| Error::OutOfHostMemory)?;
        memory.resize(crate::accounting::cpu_vec_bytes::<PendingAcquire>(entries.capacity())?)?;
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
    budget: Arc<crate::accounting::AllocatorStats>,
    // No strong Device edge. All real storage/metadata precedes its credit.
    _memory: crate::accounting::CpuStorageReservation,
}
impl AcquireQueue {
    fn new(budget: Arc<crate::accounting::AllocatorStats>) -> Result<Arc<Self>> {
        let memory = budget.reserve_cpu_storage(
            crate::accounting::shared_cpu_bytes::<Self>()?)?;
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
        if self.budget.budget_action() == crate::accounting::BudgetAction::None { return 0; }
        let old = {
            let mut state = self.state.lock();
            // No scan, wait or complete-drain: checked-out and producer slots
            // remain owners even when the visible pending list is empty.
            if state.required(0).is_ok_and(|needed| needed == 0) { Some(std::mem::take(&mut state.storage)) }
            else { None }
        };
        let released_cpu_bytes = old.as_ref().and_then(|storage| storage._memory.as_ref())
            .map_or(0, crate::accounting::CpuStorageReservation::bytes);
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
    fn shared(&self, budget: &Arc<crate::accounting::AllocatorStats>) -> Result<Arc<AcquireQueue>> {
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

impl crate::device::extensions::DeviceExtension for PendingAcquires {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn trim_under_pressure(&self) {
        self.trim_empty_if_pressure();
    }
}
impl Device {
    /// The device's queue of released ranges awaiting their ownership acquire.
    pub(crate) fn pending_acquires(&self) -> &PendingAcquires {
        self.extensions.get_or_init(
            crate::device::extensions::ExtensionSlot::PendingAcquires,
            PendingAcquires::default,
        )
    }

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
        let queue = self.pending_acquires().queue.get()
            .ok_or_else(|| Error::invalid_usage("pending acquire reservation missing"))?;
        queue.publish(reservation, acquire)
    }
    /// Reserve room for `additional` registrations before submitting their
    /// releases, so that a host allocation failure cannot strand a range
    /// that was already released.
    pub(crate) fn reserve_pending_acquires(&self, additional: usize) -> Result<PendingAcquireReservation> {
        if additional == 0 { return Ok(PendingAcquireReservation::default()); }
        self.pending_acquires().shared(&self.memory.stats)?.reserve(additional)
    }
    /// Pending acquires for queues of `family` (checked-out leases excluded).
    pub(crate) fn pending_acquire_count(&self, family: u32) -> usize {
        if self.pending_acquires().is_empty() { return 0; }
        let queue = self.pending_acquires().queue.get().expect("nonempty acquire queue");
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
    ) -> Result<(DrainedAcquires, crate::command::barrier::SynchronizationScope)> {
        let Some(queue) = self.pending_acquires().queue.get() else {
            return Ok((DrainedAcquires::default(), crate::command::barrier::SynchronizationScope::new(
                vk::PipelineStageFlags::empty(), vk::PipelineStageFlags::empty())));
        };
        if queue.count.load(Ordering::Acquire) == 0 {
            return Ok((DrainedAcquires::default(), crate::command::barrier::SynchronizationScope::new(
                vk::PipelineStageFlags::empty(), vk::PipelineStageFlags::empty())));
        }
        if !command.is_recording() {
            return Err(Error::invalid_usage("pending acquires need a recording command"));
        }
        let family = command.pool.family;
        let (mut candidates, mut drained) = queue.drain(family)?;
        // Resource gates and command metadata can allocate or wait. The shared
        // registration list is no longer locked while any of that work runs.
        let mut scope = crate::command::barrier::SynchronizationScope::new(
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
                if crate::tracking::range_state::intersection(current.range, entry.transfer.range)
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
    scope: &mut crate::command::barrier::SynchronizationScope,
) -> Result<()> {
    let transfer = entry.transfer;
    command.record_state_patch(
        &target.cell().clone(),
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
    use crate::command::barrier::{BarrierTarget, OwnershipHalf, RangeBarrier};
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
pub(crate) mod tests;

#[cfg(test)]
mod reservation_storage_tests;
