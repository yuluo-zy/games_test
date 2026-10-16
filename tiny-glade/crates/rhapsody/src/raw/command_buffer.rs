//! Vulkan 命令池、命令缓冲与设备级命令缓冲复用池。
use super::device::VulkanDevice;
use ash::vk;
use parking_lot::Mutex;
use std::sync::Arc;

/// 实际命令池与命令缓冲持有关系；默认采用可单独重置的主命令缓冲。
pub struct VulkanCommandPool {
    pub device: Arc<VulkanDevice>,
    pub handle: vk::CommandPool,
    pub family: u32,
    lock: Mutex<()>,
}
impl VulkanCommandPool {
    pub fn new(device: Arc<VulkanDevice>) -> Result<Arc<Self>, vk::Result> {
        let family = device.queue_family;
        Self::new_for_family(device, family)
    }
    pub fn new_for_family(device: Arc<VulkanDevice>, family: u32) -> Result<Arc<Self>, vk::Result> {
        if family != device.queue_family
            && !device.additional_queues.iter().any(|q| q.family == family)
        {
            return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
        }
        let info = vk::CommandPoolCreateInfo::default()
            .queue_family_index(family)
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);
        let handle = unsafe { device.device.create_command_pool(&info, None)? };
        Ok(Arc::new(Self {
            device,
            handle,
            family,
            lock: Mutex::new(()),
        }))
    }
    pub fn allocate(self: &Arc<Self>) -> Result<VulkanCommandBuffer, vk::Result> {
        let _guard = self.lock.lock();
        let info = vk::CommandBufferAllocateInfo::default()
            .command_pool(self.handle)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);
        let handle = unsafe { self.device.device.allocate_command_buffers(&info)? }[0];
        Ok(VulkanCommandBuffer {
            handle,
            pool: self.clone(),
            recording: false,
            executable: false,
            owners: vec![],
        })
    }
    /// 回收一条命令缓冲：等待设备完成、reset 并清空其拥有者，供下一帧复用。
    pub fn recycle(&self, command: &mut VulkanCommandBuffer) -> Result<(), vk::Result> {
        command.reset()
    }
}

/// 命令缓冲序号跟踪器：CPU 侧单调递增，供 managed 退役按"当前序号"入桶。
#[derive(Debug, Default)]
pub struct CommandBufferSeqTracker {
    sequence: std::sync::atomic::AtomicU64,
}
impl CommandBufferSeqTracker {
    pub fn new() -> Self {
        Self::default()
    }
    /// 原入口：当前（即将分配的）序号。
    pub fn current(&self) -> u64 {
        self.sequence.load(std::sync::atomic::Ordering::Relaxed)
    }
    /// 原入口：分配新序号（返回分配到的值）。
    pub fn track(&mut self) -> u64 {
        self.sequence
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }
    pub fn set(&mut self, value: u64) {
        self.sequence
            .store(value, std::sync::atomic::Ordering::Relaxed);
    }
}

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
impl Drop for VulkanCommandPool {
    fn drop(&mut self) {
        self.device
            .bury(super::graveyard::Doomed::CommandPool(self.handle));
    }
}

pub struct VulkanCommandBuffer {
    pub handle: vk::CommandBuffer,
    pub pool: Arc<VulkanCommandPool>,
    recording: bool,
    executable: bool,
    owners: Vec<Box<dyn Send + Sync>>,
}
impl VulkanCommandBuffer {
    pub fn is_recording(&self) -> bool {
        self.recording
    }
    /// 安全公开入口先确认设备完成，再复用原命令缓冲及释放其帧内拥有者。
    pub fn reset(&mut self) -> Result<(), vk::Result> {
        if self.recording {
            return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
        }
        self.pool.device.wait_idle()?;
        {
            let _guard = self.pool.lock.lock();
            unsafe {
                self.pool
                    .device
                    .device
                    .reset_command_buffer(self.handle, vk::CommandBufferResetFlags::empty())?;
            }
        }
        self.recording = false;
        self.executable = false;
        self.owners.clear();
        Ok(())
    }
    /// 与 `reset` 相同但不做整设备 idle，用于帧循环复用。
    ///
    /// # Safety
    /// 调用者必须已等待覆盖此命令缓冲全部提交的时间线/fence（与 `release_completed_owners` 同一契约）。
    pub unsafe fn reset_after_completion(&mut self) -> Result<(), vk::Result> {
        if self.recording {
            return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
        }
        {
            let _guard = self.pool.lock.lock();
            unsafe {
                self.pool
                    .device
                    .device
                    .reset_command_buffer(self.handle, vk::CommandBufferResetFlags::empty())?;
            }
        }
        self.recording = false;
        self.executable = false;
        self.owners.clear();
        Ok(())
    }
    pub fn is_executable(&self) -> bool {
        self.executable
    }
    pub fn begin(&mut self) -> Result<(), vk::Result> {
        if self.recording || self.executable {
            return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
        }
        let info = vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        unsafe {
            self.pool
                .device
                .device
                .begin_command_buffer(self.handle, &info)?
        };
        self.recording = true;
        Ok(())
    }
    pub fn end(&mut self) -> Result<(), vk::Result> {
        if !self.recording {
            return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
        }
        unsafe { self.pool.device.device.end_command_buffer(self.handle)? };
        self.recording = false;
        self.executable = true;
        Ok(())
    }
    /// # Safety
    /// command 必须在合法录制状态（GRAPHICS 绑定点）。
    pub unsafe fn set_stencil_reference(&self, value: u8) {
        unsafe {
            self.pool.device.device.cmd_set_stencil_reference(
                self.handle,
                vk::StencilFaceFlags::FRONT_AND_BACK,
                u32::from(value),
            )
        }
    }
    pub fn keep_alive<T: Send + Sync + 'static>(&mut self, owner: Arc<T>) {
        self.owners.push(Box::new(owner))
    }
    pub fn keep_resource_owner(&mut self, owner: crate::raw::binding::ResourceOwner) {
        self.owners.push(Box::new(owner));
    }
    /// # Safety
    /// 调用者必须已等待覆盖此 CB 的提交凭据，不能仅依据 CPU 录制结束。
    pub unsafe fn release_completed_owners(&mut self) {
        self.owners.clear();
    }
    pub fn submit(self) -> Result<PendingCommand, vk::Result> {
        let queue = super::device::DeviceQueue {
            handle: self.pool.device.queue,
            family: self.pool.device.queue_family,
            index: 0,
        };
        self.submit_to(queue, &[], &[])
    }
    /// 显式生产时间线 wait/signal 数组，并持有信号量到真实 fence 完成。
    ///
    /// 与原 `queue_submit`（0x140c04e20）的差异（wave2 读通，详见
    /// `rhapsody-alignment/reports/wave2-queue-submit.md`）：原版 `fence = VK_NULL_HANDLE`，完成靠**每队列时间线**
    /// （`Queue+0x88`，每条命令 signal 一个递增值）；提交依次持 `dev+0x800` 读锁与队列自身字节锁（`Queue+0x70`）；
    /// 提交后 `FUN_1418c0a70(dev+0x648)` 处理延迟操作并对本队列 `reclaim_command_buffer_resources`。
    /// 本函数仍是 fence + 设备级退役时间线（`retire_timeline`）的自设计路径，纯 CPU 对齐版见
    /// [`super::submit::SubmissionEngine::queue_submit`]。
    pub fn submit_to(
        self,
        queue: super::device::DeviceQueue,
        waits: &[(
            Arc<super::semaphore::TimelineSemaphore>,
            u64,
            vk::PipelineStageFlags,
        )],
        signals: &[(Arc<super::semaphore::TimelineSemaphore>, u64)],
    ) -> Result<PendingCommand, vk::Result> {
        if !self.executable {
            return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
        }
        let device = &self.pool.device;
        let known = (queue.handle == device.queue
            && queue.family == device.queue_family
            && queue.index == 0)
            || device.additional_queues.iter().any(|q| {
                q.handle == queue.handle && q.family == queue.family && q.index == queue.index
            });
        if !known
            || queue.family != self.pool.family
            || waits
                .iter()
                .any(|(s, _, stage)| !Arc::ptr_eq(&s.device, device) || stage.is_empty())
            || signals.iter().any(|(s, _)| !Arc::ptr_eq(&s.device, device))
        {
            return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
        }
        let wait_handles = waits.iter().map(|(s, _, _)| s.handle).collect::<Vec<_>>();
        let wait_values = waits.iter().map(|(_, value, _)| *value).collect::<Vec<_>>();
        let wait_stages = waits.iter().map(|(_, _, stage)| *stage).collect::<Vec<_>>();
        let mut signal_handles = signals.iter().map(|(s, _)| s.handle).collect::<Vec<_>>();
        let mut signal_values = signals.iter().map(|(_, value)| *value).collect::<Vec<_>>();
        let fence = unsafe {
            device
                .device
                .create_fence(&vk::FenceCreateInfo::default(), None)?
        };
        let buffers = [self.handle];
        let result = {
            let _guard = device.queue_lock.lock();
            // 退役信号必须与提交同在 queue_lock 内取 epoch，保证时间线值按提交顺序单调。
            let retire = device.note_submit_on(queue.handle);
            if let Some((semaphore, value)) = retire {
                signal_handles.push(semaphore);
                signal_values.push(value);
            }
            let mut timeline = vk::TimelineSemaphoreSubmitInfo::default()
                .wait_semaphore_values(&wait_values)
                .signal_semaphore_values(&signal_values);
            let submits = [vk::SubmitInfo::default()
                .command_buffers(&buffers)
                .wait_semaphores(&wait_handles)
                .wait_dst_stage_mask(&wait_stages)
                .signal_semaphores(&signal_handles)
                .push_next(&mut timeline)];
            let result = unsafe { device.device.queue_submit(queue.handle, &submits, fence) };
            if result.is_err() {
                device.note_submit_failed(retire);
            }
            result
        };
        if let Err(error) = result {
            unsafe { device.device.destroy_fence(fence, None) };
            return Err(error);
        }
        Ok(PendingCommand {
            command: Some(self),
            fence,
            _semaphores: waits
                .iter()
                .map(|(s, _, _)| s.clone())
                .chain(signals.iter().map(|(s, _)| s.clone()))
                .collect(),
        })
    }
}
impl Drop for VulkanCommandBuffer {
    fn drop(&mut self) {
        let _guard = self.pool.lock.lock();
        unsafe {
            self.pool
                .device
                .device
                .free_command_buffers(self.pool.handle, &[self.handle])
        }
    }
}

/// 未完成提交持有命令缓冲及其所有资源。释放前等待真实 fence。
pub struct PendingCommand {
    command: Option<VulkanCommandBuffer>,
    pub fence: vk::Fence,
    _semaphores: Vec<Arc<super::semaphore::TimelineSemaphore>>,
}
impl PendingCommand {
    fn device(&self) -> &ash::Device {
        // `command` 只在构造时置 Some，之后从不取走（Option 仅为 Drop 顺序服务）。
        &self.command.as_ref().expect("PendingCommand 始终持有命令缓冲").pool.device.device
    }
    pub fn wait(&self, timeout: u64) -> Result<(), vk::Result> {
        unsafe { self.device().wait_for_fences(&[self.fence], true, timeout) }
    }
    pub fn is_complete(&self) -> Result<bool, vk::Result> {
        unsafe { self.device().get_fence_status(self.fence) }
    }
}
impl Drop for PendingCommand {
    fn drop(&mut self) {
        if let Some(command) = &self.command {
            let device = &command.pool.device;
            unsafe {
                let _ = device.device.wait_for_fences(&[self.fence], true, u64::MAX);
                device.device.destroy_fence(self.fence, None)
            }
        }
    }
}

/// 设备级命令缓冲池（原 `CommandBufferPool`，`GraphicsDevice+0x9b0` 锁 / `+0x9c0` 数组 / `+0x9c8` 长度）。
///
/// 机器码事实（0x140bed4c0 `CommandBufferPool::new`、0x140bef4a0 `create_command_buffer`、
/// 0x140bf0cc0 回收尾段）：
/// - 数组元素 0x18B，即一个 `Vec<CommandBuffer>`（`{cap@0, ptr@8, len@0x10}`），按**队列族索引**寻址，
///   初值 `{0, 8, 0}` = 空 `Vec`（`8` 是 `align_of::<CommandBuffer>()` 的悬垂指针，U#10 闭合）；
///   数组长度 = 物理设备队列族数量（与 `*(dev+0x658)+0x38` 的 0x1c 步长族属性数组同长）。
/// - 取：持 `+0x9b0` 自旋锁，越界索引返回 `InvalidUsage`；`Vec` 非空则弹出末尾（LIFO，`len-1`，元素 0x58B），
///   为空才新建（`CommandBuffer::new` = 每个命令缓冲一个独立 `vkCreateCommandPool`(flags=RESET) + 1 个主级
///   `vkAllocateCommandBuffers`）。
/// - 还：回收（`reclaim_command_buffer_resources`）完成项后持同一把锁把 0x58B 压回 `pool[cb.family]`。
///
/// 原版 `CommandBuffer` 自带状态机（0x58B，`begin` 内 reset），本重建的 `VulkanCommandBuffer` 在入池前
/// 先 reset，使池内元素总是可直接 `begin`。
pub struct CommandBufferPool {
    lists: Mutex<Vec<Vec<VulkanCommandBuffer>>>,
}
impl CommandBufferPool {
    /// 0x140bed4c0：`n` 个空 `Vec`。
    pub fn new(family_count: usize) -> Self {
        Self {
            lists: Mutex::new((0..family_count).map(|_| Vec::new()).collect()),
        }
    }
    /// 数组长度（`dev+0x9c8`）。
    pub fn family_count(&self) -> usize {
        self.lists.lock().len()
    }
    /// 某族空闲链长度。
    pub fn free_count(&self, family: u32) -> usize {
        self.lists.lock().get(family as usize).map_or(0, Vec::len)
    }
    /// 取：非空则 LIFO 弹出；族索引越界返回 `InvalidUsage`。
    pub fn take(&self, family: u32) -> crate::Result<Option<VulkanCommandBuffer>> {
        let mut lists = self.lists.lock();
        let len = lists.len();
        let list = lists.get_mut(family as usize).ok_or_else(|| family_out_of_range(family, len))?;
        Ok(list.pop())
    }
    /// 还：压回命令缓冲所属族的空闲链。
    ///
    /// # Safety
    /// 覆盖该命令缓冲全部提交的时间线必须已完成（与 `reset_after_completion` 同一契约）。
    pub unsafe fn recycle_completed(&self, mut command: VulkanCommandBuffer) -> crate::Result<()> {
        unsafe { command.reset_after_completion()? };
        let family = command.pool.family;
        let mut lists = self.lists.lock();
        let len = lists.len();
        lists
            .get_mut(family as usize)
            .ok_or_else(|| family_out_of_range(family, len))?
            .push(command);
        Ok(())
    }
}

fn family_out_of_range(family: u32, len: usize) -> crate::Error {
    crate::Error::invalid_usage(format!("队列族索引 {family} 越界（共 {len} 个族）"))
}
