//! Command pools and their buffer recycling.
use ash::vk;
use crate::device::Device;
use crate::device::pools::CommandPoolAllocation;
use crate::device::pools::PoolSync;
use parking_lot::Mutex;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use super::VulkanCommandBuffer;
use super::recorded_state::OwnershipValidationState;

/// 实际命令池与命令缓冲持有关系；默认采用可单独重置的主命令缓冲。
pub struct VulkanCommandPool {
    // Rust drops fields in declaration order. Destroy the raw pool before the
    // last device owner; copies in Retirement are drained by Device::drop.
    pub(super) allocation: Arc<CommandPoolAllocation>,
    pub(crate) device: Arc<Device>,
    pub(crate) handle: vk::CommandPool,
    pub family: u32,
    pub(super) last_use: Arc<crate::tracking::SyncCell>,
    pub(super) live_commands: AtomicUsize,
    resets: AtomicU64,
    max_resources: usize,
    cpu_validation: Mutex<crate::collections::temp_list::TempList<OwnershipValidationState>>,
    pub(super) cpu_state_patches: Mutex<crate::collections::temp_list::TempList<crate::tracking::range_state::StatePatch>>,
    pub(super) cpu_state_buckets: Mutex<crate::collections::temp_list::TempList<(usize, u8, usize, usize)>>,
    _memory: crate::accounting::CpuMemoryReservation,
}

impl VulkanCommandPool {
    /// The device that owns this object.
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }
    /// Interop escape hatch: the native handle. It stays owned by `self`; never destroy it or use it after `self` is dropped.
    pub fn raw_handle(&self) -> vk::CommandPool {
        self.handle
    }
}

impl VulkanCommandPool {
    pub fn new(device: Arc<Device>) -> crate::Result<Arc<Self>> {
        let family = device.queues.primary_family;
        Self::new_for_family(device, family)
    }
    pub fn new_for_family(device: Arc<Device>, family: u32) -> crate::Result<Arc<Self>> {
        Self::new_for_family_with_cpu_limit(device, family, 65_536)
    }
    pub fn new_for_family_with_cpu_limit(
        device: Arc<Device>,
        family: u32,
        max_validation_items: usize,
    ) -> crate::Result<Arc<Self>> {
        Self::build(device, family, max_validation_items, None)
    }
    /// A recording wrapper over an existing native pool (kept by the device's
    /// setup cache without owning the device). The pool is destroyed when the
    /// last `allocation` owner drops, never by this wrapper.
    pub(crate) fn attach(
        device: Arc<Device>,
        family: u32,
        allocation: Arc<CommandPoolAllocation>,
    ) -> crate::Result<Arc<Self>> {
        Self::build(device, family, 65_536, Some(allocation))
    }
    /// The shared native pool, for caches that outlive this wrapper.
    pub(crate) fn allocation(&self) -> Arc<CommandPoolAllocation> {
        self.allocation.clone()
    }
    fn build(
        device: Arc<Device>,
        family: u32,
        max_validation_items: usize,
        existing: Option<Arc<CommandPoolAllocation>>,
    ) -> crate::Result<Arc<Self>> {
        if device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        let cpu_validation = Mutex::new(crate::collections::temp_list::TempList::new(
            max_validation_items,
            device.memory.stats.clone(),
        )?);
        let cpu_state_patches = Mutex::new(crate::collections::temp_list::TempList::new(
            max_validation_items,
            device.memory.stats.clone(),
        )?);
        let cpu_state_buckets = Mutex::new(crate::collections::temp_list::TempList::new(
            usize::MAX,
            device.memory.stats.clone(),
        )?);
        if family != device.queues.primary_family
            && !device.queues.additional.iter().any(|q| q.family == family)
        {
            return Err(crate::Error::invalid_usage("命令状态或队列族无效"));
        }
        let memory =
            crate::accounting::reserve_shared_cpu_metadata::<Self>(&device.memory.stats, 0)?;
        let allocation = match existing {
            Some(allocation) => allocation,
            None => {
                let sync_storage =
                    crate::tracking::SyncCell::reserve_arc_storage(&device.memory.stats)?;
                let info = vk::CommandPoolCreateInfo::default()
                    .queue_family_index(family)
                    .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);
                let allocation_memory = crate::accounting::reserve_shared_cpu_metadata::<
                    CommandPoolAllocation,
                >(&device.memory.stats, 0)?;
                let handle = crate::device::observe_vulkan(&device, unsafe {
                    device.core.device.create_command_pool(&info, None)
                })?;
                Arc::new(CommandPoolAllocation {
                    device: device.core.device.clone(),
                    handle,
                    lock: Mutex::new(PoolSync::default()),
                    last_use: crate::tracking::SyncCell::for_device(device.generation())
                        .into_reserved_arc(sync_storage),
                    live_owned_commands: AtomicUsize::new(0),
                    pending_retirement_frees: AtomicUsize::new(0),
                    _memory: allocation_memory,
                })
            }
        };
        Ok(Arc::new(Self {
            max_resources: max_validation_items,
            cpu_validation,
            cpu_state_patches,
            cpu_state_buckets,
            last_use: allocation.last_use.clone(),
            device,
            handle: allocation.handle,
            family,
            allocation,
            live_commands: AtomicUsize::new(0),
            resets: AtomicU64::new(0),
            _memory: memory,
        }))
    }
    /// Wrap a native command buffer previously detached from this pool's
    /// allocation ([`VulkanCommandBuffer::detach`]). The buffer must have
    /// been allocated from this pool and completed. Ownership transfers even
    /// on failure: the reserved node frees or queues the native handle.
    pub(crate) fn adopt(
        self: &Arc<Self>,
        handle: vk::CommandBuffer,
        retirement: crate::device::RetireTicket,
    ) -> crate::Result<VulkanCommandBuffer> {
        let owners_retirement = match self.device.reserve_retirement() {
            Ok(ticket) => ticket,
            Err(error) => {
                self.retire_unwrapped(handle, retirement);
                return Err(error);
            }
        };
        self.wrap(handle, retirement, owners_retirement)
    }
    fn retire_unwrapped(&self, handle: vk::CommandBuffer, retirement: crate::device::RetireTicket) {
        self.allocation.release_detached();
        self.device.retire_reserved(
            crate::sync::SyncSet::EMPTY,
            crate::device::graveyard::Doomed::CommandBuffer(handle, self.allocation.clone(), false),
            retirement,
        );
    }
    pub fn allocate(self: &Arc<Self>) -> crate::Result<VulkanCommandBuffer> {
        if self.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        // Both payloads can outlive this command independently at Drop.
        let retirement = self.device.reserve_retirement()?;
        let owners_retirement = self.device.reserve_retirement()?;
        let _guard = self.allocation.lock.lock();
        let info = vk::CommandBufferAllocateInfo::default()
            .command_pool(self.handle)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);
        let handle = crate::device::observe_vulkan(&self.device, unsafe {
            self.device.core.device.allocate_command_buffers(&info)
        })?[0];
        self.allocation
            .live_owned_commands
            .fetch_add(1, Ordering::Relaxed);
        drop(_guard);
        self.wrap(handle, retirement, owners_retirement)
    }

    fn wrap(
        self: &Arc<Self>,
        handle: vk::CommandBuffer,
        retirement: crate::device::RetireTicket,
        owners_retirement: crate::device::RetireTicket,
    ) -> crate::Result<VulkanCommandBuffer> {
        let prepared = (|| {
            if self.device.is_device_lost() {
                return Err(crate::Error::DeviceLost);
            }
            let stats = &self.device.memory.stats;
            let range_uses = crate::collections::temp_list::TempList::new(self.max_resources, stats.clone())?;
            let state_patches = crate::collections::temp_list::TempList::new(self.max_resources, stats.clone())?;
            let owners = crate::collections::temp_list::TempList::new(self.max_resources, stats.clone())?;
            let uses = crate::tracking::tracked::SubmissionUseCells::new(
                self.max_resources,
                stats.clone(),
                &self.last_use,
            )?;
            Ok((range_uses, state_patches, owners, uses))
        })();
        let (range_uses, state_patches, owners, uses) = match prepared {
            Ok(prepared) => prepared,
            Err(error) => {
                self.retire_unwrapped(handle, retirement);
                return Err(error);
            }
        };
        self.live_commands.fetch_add(1, Ordering::Relaxed);
        Ok(VulkanCommandBuffer {
            retirement: Some(retirement),
            owners_retirement: Some(owners_retirement),
            handle,
            pool: self.clone(),
            recording: false,
            detached: false,
            owners,
            owner_set: Default::default(),
            uses,
            range_uses,
            state_patches,
            range_failure: None,
            last_use: crate::sync::SyncSet::EMPTY,
        })
    }
    /// 回收一条命令缓冲：等待设备完成、reset 并清空其拥有者，供下一帧复用。
    pub fn recycle(&self, command: &mut VulkanCommandBuffer) -> crate::Result<()> {
        command.reset()
    }

    /// Reset the entire pool once all of its live CPU commands are exclusively
    /// borrowed and every submission using this pool has actually completed.
    /// Failure preserves command states, owners and last-use receipts.
    pub fn reset_completed(&self, commands: &mut [&mut VulkanCommandBuffer]) -> crate::Result<()> {
        if self.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        let _reservation = self.last_use.lock_submission();
        let mut guard = self.allocation.lock.lock();
        if !self
            .device
            .queues
            .timelines
            .is_reached(&self.last_use.snapshot())?
        {
            return Err(vk::Result::NOT_READY.into());
        }
        if commands.len() != self.allocation.live_owned_commands.load(Ordering::Relaxed)
            || commands
                .iter()
                .any(|command| !std::ptr::eq(self, command.pool.as_ref()))
            || commands.iter().enumerate().any(|(index, command)| {
                commands[..index]
                    .iter()
                    .any(|previous| previous.handle == command.handle)
            })
        {
            return Err(crate::Error::invalid_usage(
                "whole-pool reset requires all live commands from this pool",
            ));
        }
        crate::device::observe_vulkan(&self.device, unsafe {
            self.device
                .core
                .device
                .reset_command_pool(self.handle, vk::CommandPoolResetFlags::empty())
        })?;
        self.resets.fetch_add(1, Ordering::Relaxed);
        for command in commands.iter_mut() {
            if command.recording {
                self.allocation.end_recording(&mut guard);
                command.recording = false;
            }
        }
        drop(guard);
        drop(_reservation);
        for command in commands {
            command.clear_after_reset();
        }
        Ok(())
    }

    pub fn cpu_scratch_capacity_bytes(&self) -> usize {
        self.cpu_validation.lock().capacity_bytes()
            + self.cpu_state_patches.lock().capacity_bytes()
            + self.cpu_state_buckets.lock().capacity_bytes()
    }
    pub fn cpu_scratch_growth_count(&self) -> u64 {
        self.cpu_validation.lock().growth_count()
            + self.cpu_state_patches.lock().growth_count()
            + self.cpu_state_buckets.lock().growth_count()
    }
    /// CPU-only validation storage can be trimmed while GPU commands are in
    /// flight; all temporary resource-state references have already been dropped.
    pub fn trim_cpu_scratch(&self) {
        self.cpu_validation.lock().trim_idle();
        self.cpu_state_patches.lock().trim_idle();
        self.cpu_state_buckets.lock().trim_idle();
    }

    pub fn reset_count(&self) -> u64 {
        self.resets.load(Ordering::Relaxed)
    }
    /// Command buffers whose free waits for the pool's current recording to end.
    pub fn pending_free_count(&self) -> usize {
        let sync = self.allocation.lock.lock();
        if sync.recording != 0 {
            self.allocation
                .pending_retirement_frees
                .load(Ordering::Relaxed)
        } else {
            0
        }
    }
    pub fn live_command_count(&self) -> usize {
        self.live_commands.load(Ordering::Relaxed)
    }
}
