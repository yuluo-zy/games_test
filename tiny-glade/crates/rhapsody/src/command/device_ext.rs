//! Command-recording conveniences on [`Device`]: one-off command buffers and barrier batches.
use crate::device::Device;
use ash::vk;

impl Device {
    /// Allocate a primary command buffer from a new pool for `family` and name
    /// it when debug utils are enabled.
    pub fn create_command_buffer(
        &self,
        family: u32,
        name: &str,
    ) -> crate::Result<crate::command::command_buffer::VulkanCommandBuffer> {
        let command =
            crate::command::command_buffer::VulkanCommandPool::new_for_family(self.shared(), family)?
                .allocate()?;
        if let Some(utils) = self.debug_utils_option() {
            let _ = unsafe { utils.set_object_name(command.handle, name) };
        }
        Ok(command)
    }
    /// Record one barrier batch through the enabled barrier API
    /// (synchronization2; empty stage masks
    /// keep their empty-scope meaning on both).
    ///
    /// # Safety
    /// `command` must be recording on this device and every barrier resource
    /// must stay alive through the command's completion.
    pub unsafe fn issue_barriers(
        &self,
        command: vk::CommandBuffer,
        scope: &crate::command::barrier::SynchronizationScope,
    ) -> crate::error::Result<()> {
        unsafe { crate::command::barrier::BarrierDispatch::new(self).issue(self, command, scope) }
    }
    /// Record the batch, then clear its barriers for reuse.
    ///
    /// # Safety
    /// As [`Self::issue_barriers`].
    pub unsafe fn issue_and_reset_pipeline_barrier(
        &self,
        command: vk::CommandBuffer,
        scope: &mut crate::command::barrier::SynchronizationScope,
    ) -> crate::error::Result<()> {
        let issued = unsafe { self.issue_barriers(command, scope) };
        scope.memory.clear();
        scope.buffers.clear();
        scope.images.clear();
        issued
    }
    /// Full execution and memory dependency (all commands to all commands).
    ///
    /// # Safety
    /// `command` must be recording on this device.
    pub unsafe fn issue_full_pipeline_barrier(
        &self,
        command: vk::CommandBuffer,
    ) -> crate::error::Result<()> {
        let mut scope = crate::command::barrier::SynchronizationScope::new(
            vk::PipelineStageFlags::ALL_COMMANDS,
            vk::PipelineStageFlags::ALL_COMMANDS,
        );
        scope.memory.push(
            vk::MemoryBarrier::default()
                .src_access_mask(vk::AccessFlags::MEMORY_WRITE)
                .dst_access_mask(vk::AccessFlags::MEMORY_READ | vk::AccessFlags::MEMORY_WRITE),
        );
        unsafe { self.issue_barriers(command, &scope) }
    }
    /// Method form of `super::binding` pipeline binding.
    ///
    /// # Safety
    /// `command_buffer` 必须处于录制状态且属于本设备。
    pub unsafe fn bind_pipeline(
        &self,
        command_buffer: vk::CommandBuffer,
        point: vk::PipelineBindPoint,
        pipeline: vk::Pipeline,
    ) {
        unsafe { crate::command::binding::bind_pipeline(&self.shared(), command_buffer, point, pipeline) }
    }
}
