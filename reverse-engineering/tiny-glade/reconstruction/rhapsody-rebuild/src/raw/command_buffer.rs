// 候选实现迁移来源：reconstruction/render/work-packages/queue-callers-r7/src/lib.rs.
// 这是行为重建，不是原作者源码。
//! 原 ensure_active_cb 调用方入口为 0x140bebc90；保留 CPU 契约，真实命令池适配另见本模块。

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandSnapshot(pub [u8; 88]);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecordingQueue {
    Primary,
}

pub trait CommandBackend {
    fn create(&mut self, pool_family_index: u32) -> CommandSnapshot;
    fn begin(&mut self, command: &mut CommandSnapshot, queue: RecordingQueue);
}

/// 设备 +0xad4 的池队列族与录制队列标签为独立输入；已有命令向量保持不变。
pub fn ensure_active_cb(
    commands: &mut Vec<CommandSnapshot>,
    pool_family_index: u32,
    backend: &mut impl CommandBackend,
) {
    if commands.is_empty() {
        let mut command = backend.create(pool_family_index);
        backend.begin(&mut command, RecordingQueue::Primary);
        commands.push(command);
    }
}

#[derive(Default)]
pub struct FixtureBackend {
    pub fill: u8,
    pub sequence: u64,
    pub events: Vec<String>,
}

impl CommandBackend for FixtureBackend {
    fn create(&mut self, family: u32) -> CommandSnapshot {
        self.events.push(format!("create:{family}"));
        CommandSnapshot([self.fill; 88])
    }

    fn begin(&mut self, command: &mut CommandSnapshot, queue: RecordingQueue) {
        assert_eq!(queue, RecordingQueue::Primary);
        self.events.push("begin:0".into());
        command.0[..8].copy_from_slice(&1u64.to_le_bytes());
        command.0[8..16].copy_from_slice(&self.sequence.to_le_bytes());
        command.0[16..24].fill(0);
        command.0[86] = 0;
    }
}

use super::device::VulkanDevice;
use ash::vk;
use parking_lot::Mutex;
use std::sync::{Arc};

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
        self.sequence.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }
    pub fn set(&mut self, value: u64) {
        self.sequence.store(value, std::sync::atomic::Ordering::Relaxed);
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
        let signal_handles = signals.iter().map(|(s, _)| s.handle).collect::<Vec<_>>();
        let signal_values = signals.iter().map(|(_, value)| *value).collect::<Vec<_>>();
        let fence = unsafe {
            device
                .device
                .create_fence(&vk::FenceCreateInfo::default(), None)?
        };
        let buffers = [self.handle];
        let mut timeline = vk::TimelineSemaphoreSubmitInfo::default()
            .wait_semaphore_values(&wait_values)
            .signal_semaphore_values(&signal_values);
        let submits = [vk::SubmitInfo::default()
            .command_buffers(&buffers)
            .wait_semaphores(&wait_handles)
            .wait_dst_stage_mask(&wait_stages)
            .signal_semaphores(&signal_handles)
            .push_next(&mut timeline)];
        let result = {
            let _guard = device.queue_lock.lock();
            device.note_submit();
            unsafe { device.device.queue_submit(queue.handle, &submits, fence) }
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
    pub fn wait(&self, timeout: u64) -> Result<(), vk::Result> {
        unsafe {
            self.command
                .as_ref()
                .unwrap()
                .pool
                .device
                .device
                .wait_for_fences(&[self.fence], true, timeout)
        }
    }
    pub fn is_complete(&self) -> Result<bool, vk::Result> {
        unsafe {
            self.command
                .as_ref()
                .unwrap()
                .pool
                .device
                .device
                .get_fence_status(self.fence)
        }
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
