//! Persistently mapped staging slots of a transfer manager. Idle slots can be
//! released under device memory pressure and are recreated on demand.
use crate::{Error, Result, device::Device, sync::{CompletedValues, QueueId, SyncPoint, SyncSet}};
use crate::resource::buffer::{Buffer, BufferDesc};
use crate::accounting::Evictable;
use ash::vk;
use std::sync::Arc;

pub(super) struct UploadSlot {
    buffer: Option<Arc<Buffer>>,
    pub last_use: SyncSet,
    pub leased: bool,
    used: u64,
}
/// An upload slot remains exclusive while packing/recording outside the
/// staging lock. The irreversible submit point is published even on error.
pub(super) struct UploadLease {
    pub staging: Arc<StagingLock>,
    pub index: Option<usize>,
    pub submitted: Option<SyncPoint>,
}
impl Drop for UploadLease {
    fn drop(&mut self) {
        if let Some(index) = self.index {
            let mut staging = self.staging.lock();
            let slot = &mut staging.uploads[index];
            if let Some(point) = self.submitted {
                slot.last_use = SyncSet::from_point(point);
            }
            slot.leased = false;
        }
    }
}
pub(super) struct ReadbackSlot {
    buffer: Option<Arc<Buffer>>,
    pub last_use: SyncSet,
    lease: Option<Arc<ReadbackLease>>,
    used: u64,
}
pub(super) struct ReadbackLease {
    pub buffer: Arc<Buffer>,
    pub point: SyncPoint,
    pub bytes: usize,
    _memory: crate::accounting::CpuMemoryReservation,
}
impl ReadbackSlot {
    /// Not referenced by a ticket.
    fn unleased(&self) -> bool {
        self.lease
            .as_ref()
            .is_none_or(|lease| Arc::strong_count(lease) == 1 && Arc::weak_count(lease) == 0)
    }
    pub fn leased(&self) -> bool {
        !self.unleased()
    }
}

pub(super) struct Staging {
    device: Arc<Device>,
    /// Both families when a transfer queue of another family shares staging.
    families: Option<[u32; 2]>,
    chunk_bytes: u64,
    max_bytes: u64,
    lease_queue: QueueId,
    pub uploads: Vec<UploadSlot>,
    pub readbacks: Vec<ReadbackSlot>,
    pub committed_bytes: u64,
    clock: u64,
    pub evicted_slots: u64,
    _slots_memory: crate::accounting::CpuMemoryReservation,
}

impl Staging {
    pub fn new(
        device: Arc<Device>,
        families: Option<[u32; 2]>,
        slots: u32,
        chunk_bytes: u64,
        max_bytes: u64,
        lease_queue: QueueId,
    ) -> Result<Self> {
        let mut slots_memory = device.memory.stats.reserve_cpu_bytes(
            u64::from(slots) * (size_of::<UploadSlot>() + size_of::<ReadbackSlot>()) as u64,
            crate::accounting::BudgetCategory::Live,
        )?;
        let mut uploads = Vec::new();
        let mut readbacks = Vec::new();
        uploads
            .try_reserve_exact(slots as usize)
            .map_err(|_| Error::OutOfHostMemory)?;
        readbacks
            .try_reserve_exact(slots as usize)
            .map_err(|_| Error::OutOfHostMemory)?;
        slots_memory.resize(
            (uploads.capacity() * size_of::<UploadSlot>()
                + readbacks.capacity() * size_of::<ReadbackSlot>()) as u64,
        )?;
        let mut staging = Self {
            device,
            families,
            chunk_bytes,
            max_bytes,
            lease_queue,
            uploads,
            readbacks,
            committed_bytes: 0,
            clock: 0,
            evicted_slots: 0,
            _slots_memory: slots_memory,
        };
        // Create every slot up front: the configuration is validated against
        // real memory requirements, and steady state never allocates.
        for _ in 0..slots {
            staging.uploads.push(UploadSlot {
                buffer: None,
                last_use: SyncSet::EMPTY,
                leased: false,
                used: 0,
            });
            staging.readbacks.push(ReadbackSlot {
                buffer: None,
                last_use: SyncSet::EMPTY,
                lease: None,
                used: 0,
            });
        }
        for index in 0..slots as usize {
            staging.upload_buffer(index)?;
            staging.ensure_readback(index)?;
        }
        Ok(staging)
    }

    fn create(&self, desc: &BufferDesc) -> Result<Arc<Buffer>> {
        let mut buffer = match self.families {
            Some(families) => Buffer::new_concurrent(self.device.clone(), desc, &families)?,
            None => Buffer::new_dedicated(self.device.clone(), desc)?,
        };
        // Map once for the slot lifetime; every later host copy reuses it.
        // SAFETY: the buffer is new, so no GPU or host access can overlap.
        unsafe {
            buffer.mapped_slice_mut(0, desc.size)?;
        }
        Ok(Arc::new(buffer))
    }

    fn charge(&mut self, buffer: &Buffer) -> Result<()> {
        let total = self
            .committed_bytes
            .checked_add(buffer.allocation_bytes())
            .filter(|total| *total <= self.max_bytes)
            .ok_or(Error::OutOfDeviceMemory)?;
        self.committed_bytes = total;
        Ok(())
    }

    fn tick(&mut self) -> u64 {
        self.clock += 1;
        self.clock
    }

    /// The buffer of upload slot `index`, recreated if it was evicted.
    pub fn upload_buffer(&mut self, index: usize) -> Result<Arc<Buffer>> {
        let used = self.tick();
        if let Some(buffer) = self.uploads[index].buffer.clone() {
            self.uploads[index].used = used;
            return Ok(buffer);
        }
        let buffer = self.create(&BufferDesc::new_cpu_to_gpu(
            self.chunk_bytes,
            vk::BufferUsageFlags::TRANSFER_SRC,
        ))?;
        self.charge(&buffer)?;
        let slot = &mut self.uploads[index];
        slot.buffer = Some(buffer.clone());
        slot.used = used;
        Ok(buffer)
    }

    /// Make readback slot `index` usable, recreating an evicted buffer/lease.
    pub fn ensure_readback(&mut self, index: usize) -> Result<()> {
        let used = self.tick();
        self.readbacks[index].used = used;
        if self.readbacks[index].buffer.is_some() {
            return Ok(());
        }
        let buffer = self.create(&BufferDesc::new_gpu_to_cpu(
            self.chunk_bytes,
            vk::BufferUsageFlags::TRANSFER_DST,
        ))?;
        let memory = self.device.memory.stats.reserve_cpu_bytes(
            crate::accounting::shared_cpu_bytes::<ReadbackLease>()?,
            crate::accounting::BudgetCategory::Live,
        )?;
        let lease = Arc::new(ReadbackLease {
            buffer: buffer.clone(),
            point: SyncPoint {
                queue: self.lease_queue,
                value: 0,
            },
            bytes: 0,
            _memory: memory,
        });
        self.charge(&buffer)?;
        let slot = &mut self.readbacks[index];
        slot.buffer = Some(buffer);
        slot.lease = Some(lease);
        Ok(())
    }

    /// The first upload slot whose last use completed, preferring resident
    /// buffers over evicted ones.
    pub fn free_upload(&self, completed: &CompletedValues) -> Option<usize> {
        let free = |slot: &UploadSlot| !slot.leased && slot.last_use.is_reached(completed);
        self.uploads
            .iter()
            .position(|slot| slot.buffer.is_some() && free(slot))
            .or_else(|| self.uploads.iter().position(free))
    }

    /// The first unleased, completed readback slot (resident ones first).
    pub fn free_readback(&self, completed: &CompletedValues) -> Option<usize> {
        let free = |slot: &ReadbackSlot| slot.unleased() && slot.last_use.is_reached(completed);
        self.readbacks
            .iter()
            .position(|slot| slot.buffer.is_some() && free(slot))
            .or_else(|| self.readbacks.iter().position(free))
    }

    pub fn readback_buffer(&self, index: usize) -> Arc<Buffer> {
        self.readbacks[index]
            .buffer
            .clone()
            .expect("readback slot was made resident")
    }

    /// Publish a successful readback; the lease is private until now.
    pub fn publish_readback(
        &mut self,
        index: usize,
        point: SyncPoint,
        bytes: usize,
    ) -> Arc<ReadbackLease> {
        let slot = &mut self.readbacks[index];
        let lease = slot
            .lease
            .as_mut()
            .expect("readback slot was made resident");
        let private = Arc::get_mut(lease).expect("lease is private until publication");
        private.point = point;
        private.bytes = bytes;
        slot.last_use = SyncSet::from_point(point);
        lease.clone()
    }

    /// Release idle slots (no ticket, last use completed), least recently
    /// used first, until `target` bytes are freed. Returns the bytes freed.
    fn evict(&mut self, completed: &CompletedValues, target: u64) -> u64 {
        let mut candidates: smallvec::SmallVec<[(u64, bool, usize); 16]> =
            smallvec::SmallVec::new();
        for (index, slot) in self.uploads.iter().enumerate() {
            if !slot.leased && slot.buffer.is_some() && slot.last_use.is_reached(completed) {
                candidates.push((slot.used, true, index));
            }
        }
        for (index, slot) in self.readbacks.iter().enumerate() {
            if slot.buffer.is_some() && slot.unleased() && slot.last_use.is_reached(completed) {
                candidates.push((slot.used, false, index));
            }
        }
        candidates.sort_unstable();
        let mut released = 0u64;
        for (_, upload, index) in candidates {
            if released >= target {
                break;
            }
            let buffer = if upload {
                self.uploads[index].buffer.take()
            } else {
                self.readbacks[index].lease = None;
                self.readbacks[index].buffer.take()
            };
            if let Some(buffer) = buffer {
                let bytes = buffer.allocation_bytes();
                self.committed_bytes -= bytes;
                released += bytes;
                self.evicted_slots += 1;
            }
        }
        released
    }
}

/// `Mutex<Staging>` as a type local to this crate. `Evictable` is defined in `prism-gfx`, and the orphan
/// rule does not allow implementing it for `parking_lot::Mutex<Staging>` from here. `Deref` keeps every
/// `.lock()` call site unchanged.
pub(super) struct StagingLock(parking_lot::Mutex<Staging>);
impl StagingLock {
    pub(super) fn new(staging: Staging) -> Self {
        Self(parking_lot::Mutex::new(staging))
    }
}
impl std::ops::Deref for StagingLock {
    type Target = parking_lot::Mutex<Staging>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Registered with the device budget controller. Never blocks: a manager in
/// the middle of an operation (holding the lock) is skipped.
impl Evictable for StagingLock {
    fn evict_idle(&self, completed: &CompletedValues, target_bytes: u64) -> u64 {
        match self.0.try_lock() {
            Some(mut staging) => staging.evict(completed, target_bytes),
            None => 0,
        }
    }
}
