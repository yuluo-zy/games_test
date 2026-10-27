//! Vulkan command pools and buffers with precise resource ownership and retirement.
use ash::vk;
use std::sync::{Arc, atomic::Ordering};
mod pool;
mod retention;
mod recorded_state;
mod submission;
pub use pool::VulkanCommandPool;
use retention::MetadataFailure;
use recorded_state::{RecordedOpaqueUse, RecordedStatePatch};
pub use submission::PendingCommand;
pub(crate) use submission::{prepare_submission_states, validate_submission_commands};

/// 命令缓冲录制时的图形状态缓存；`record_default` 提供初始默认值。
/// 候选只保留最少的深度/剔除/混合默认，具体默认策略由管线声明决定。
#[derive(Clone, Copy, Debug)]
pub struct CbGraphicsState {
    pub depth_test: bool,
    pub depth_write: bool,
    pub cull_mode: vk::CullModeFlags,
}
impl CbGraphicsState {
    pub fn record_default() -> Self {
        Self {
            depth_test: true,
            depth_write: true,
            cull_mode: vk::CullModeFlags::BACK,
        }
    }
}

pub struct VulkanCommandBuffer {
    retirement: Option<crate::device::RetireTicket>,
    owners_retirement: Option<crate::device::RetireTicket>,
    pub(crate) handle: vk::CommandBuffer,
    pub pool: Arc<VulkanCommandPool>,
    recording: bool,
    /// The native buffer was handed to a cache ([`Self::detach`]); dropping
    /// this wrapper must not retire or free it.
    detached: bool,
    owners: crate::collections::temp_list::TempList<crate::tracking::ResourceOwner>,
    uses: crate::tracking::tracked::SubmissionUseCells,
    /// Pointer membership for the objects retained in `owners`.
    owner_set: crate::collections::token_map::TokenMap<()>,
    range_uses: crate::collections::temp_list::TempList<RecordedOpaqueUse>,
    state_patches: crate::collections::temp_list::TempList<RecordedStatePatch>,
    range_failure: Option<MetadataFailure>,
    last_use: crate::sync::SyncSet,
}

impl VulkanCommandBuffer {
    /// Interop escape hatch: the native handle. It stays owned by `self`; never destroy it or use it after `self` is dropped.
    pub fn raw_handle(&self) -> vk::CommandBuffer {
        self.handle
    }
}

impl VulkanCommandBuffer {
    fn clear_after_reset(&mut self) {
        self.recording = false;
        self.owners.clear();
        self.owner_set.clear();
        self.range_uses.clear();
        self.state_patches.clear();
        self.range_failure = None;
        self.uses.reset(&self.pool.last_use);
        self.last_use = crate::sync::SyncSet::EMPTY;
    }
    pub fn is_recording(&self) -> bool {
        self.recording
    }
    pub(crate) fn has_submissions(&self) -> bool {
        !self.last_use.is_empty()
    }
    /// 安全公开入口先确认设备完成，再复用原命令缓冲及释放其帧内拥有者。
    pub fn reset(&mut self) -> crate::Result<()> {
        if self.pool.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        // A rejected metadata operation has issued no queue work. Vulkan permits
        // resetting this recording buffer; preserve normal begin/end discipline
        // while allowing its admitted storage to recover after pressure clears.
        if self.recording && self.range_failure.is_none() {
            return Err(crate::Error::invalid_usage("命令状态或队列族无效"));
        }
        self.pool
            .device
            .queues
            .timelines
            .wait(&self.last_use, u64::MAX)?;
        {
            let mut sync = self.pool.allocation.lock.lock();
            if self.recording {
                self.pool.allocation.end_recording(&mut sync);
                self.recording = false;
            }
            crate::device::observe_vulkan(&self.pool.device, unsafe {
                self.pool
                    .device
                    .core
                    .device
                    .reset_command_buffer(self.handle, vk::CommandBufferResetFlags::empty())
            })?;
        }
        self.clear_after_reset();
        Ok(())
    }
    /// 与 `reset` 相同但不做整设备 idle，用于帧循环复用。
    ///
    /// # Safety
    /// 调用者必须已等待覆盖此命令缓冲全部提交的时间线/fence（与 `release_completed_owners` 同一契约）。
    pub unsafe fn reset_after_completion(&mut self) -> crate::Result<()> {
        if self.pool.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        {
            let mut sync = self.pool.allocation.lock.lock();
            if self.recording {
                self.pool.allocation.end_recording(&mut sync);
                self.recording = false;
            }
            crate::device::observe_vulkan(&self.pool.device, unsafe {
                self.pool
                    .device
                    .core
                    .device
                    .reset_command_buffer(self.handle, vk::CommandBufferResetFlags::empty())
            })?;
        }
        self.clear_after_reset();
        Ok(())
    }
    pub fn is_executable(&self) -> bool {
        self.uses.is_sealed()
    }
    pub fn begin(&mut self) -> crate::Result<()> {
        if self.pool.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        if self.recording || self.uses.is_sealed() {
            return Err(crate::Error::invalid_usage("命令状态或队列族无效"));
        }
        let info = vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        let mut sync = self.pool.allocation.lock.lock();
        crate::device::observe_vulkan(&self.pool.device, unsafe {
            self.pool
                .device
                .core
                .device
                .begin_command_buffer(self.handle, &info)
        })?;
        sync.recording += 1;
        self.recording = true;
        Ok(())
    }
    pub fn end(&mut self) -> crate::Result<()> {
        if self.pool.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        if !self.recording {
            return Err(crate::Error::invalid_usage("命令状态或队列族无效"));
        }
        if let Some(error) = self.range_failure {
            return Err(error.error());
        }
        {
            let mut sync = self.pool.allocation.lock.lock();
            let ended = crate::device::observe_vulkan(&self.pool.device, unsafe {
                self.pool.device.core.device.end_command_buffer(self.handle)
            });
            // A failed end leaves the buffer invalid; either way it no longer
            // records, so pending frees may proceed.
            self.pool.allocation.end_recording(&mut sync);
            if let Err(error) = ended {
                self.recording = false;
                return Err(error);
            }
        }
        self.uses.seal();
        self.recording = false;
        Ok(())
    }
    /// # Safety
    /// command 必须在合法录制状态（GRAPHICS 绑定点）。
    pub unsafe fn set_stencil_reference(&self, value: u8) {
        unsafe {
            self.pool.device.core.device.cmd_set_stencil_reference(
                self.handle,
                vk::StencilFaceFlags::FRONT_AND_BACK,
                u32::from(value),
            )
        }
    }

}
impl VulkanCommandBuffer {
    /// Hand the native buffer to a cache: this wrapper (and the resources it
    /// retained, which retire through their own use points) is dropped, the
    /// native command buffer is neither freed nor retired and stays owned by
    /// the pool's allocation. Returns the handle with the completion set of
    /// every submission of this command; re-wrap it with
    /// [`VulkanCommandPool::adopt`] after that set completed.
    pub(crate) fn detach(
        mut self,
    ) -> (
        vk::CommandBuffer,
        crate::sync::SyncSet,
        crate::device::RetireTicket,
    ) {
        self.detached = true;
        (
            self.handle,
            self.last_use,
            self.retirement
                .take()
                .expect("detached command retirement node"),
        )
    }
}
impl Drop for VulkanCommandBuffer {
    fn drop(&mut self) {
        {
            let mut sync = self.pool.allocation.lock.lock();
            if self.recording {
                self.pool.allocation.end_recording(&mut sync);
            }
            self.pool.live_commands.fetch_sub(1, Ordering::Relaxed);
            if !self.detached {
                self.pool
                    .allocation
                    .live_owned_commands
                    .fetch_sub(1, Ordering::Relaxed);
            }
        }
        // Retained objects live until the command's completion, not until this
        // wrapper is dropped.
        // Objects that track completion through their own use cells retire
        // themselves; only the others move into the graveyard.
        if !self.last_use.is_empty()
            && self
                .owners
                .as_slice()
                .iter()
                .any(|owner| owner.held_until_completion())
        {
            self.owners.retain(|owner| owner.held_until_completion());
            let owners = std::mem::replace(
                &mut self.owners,
                crate::collections::temp_list::TempList::new(1, self.pool.device.memory.stats.clone())
                    .expect("nonzero limit"),
            );
            self.pool.device.retire_reserved(
                self.last_use,
                crate::device::graveyard::Doomed::Owners(owners),
                self.owners_retirement
                    .take()
                    .expect("command owners retirement node"),
            );
        }
        if self.detached {
            return;
        }
        self.pool.device.retire_reserved(
            self.last_use,
            crate::device::graveyard::Doomed::CommandBuffer(
                self.handle,
                self.pool.allocation.clone(),
                false,
            ),
            self.retirement.take().expect("command retirement node"),
        );
    }
}

#[cfg(test)]
mod native_state_submission_tests;

#[cfg(test)]
mod allocation_owner_tests;

#[cfg(test)]
mod retirement_failure_tests;
