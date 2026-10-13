//! 设备级延迟销毁场：Drop 路径 O(1) 入场，不再每次 `device_wait_idle`。
//!
//! 安全性依据：所有真实 `vkQueueSubmit` 都在 `queue_lock` 内调用 `note_submit` 递增 epoch；
//! 入场项记录入场时的 epoch。只有当 `completed_epoch >= epoch`（即入场前发出的所有提交
//! 已由一次整设备 idle 确认完成）才销毁。没有新提交时完全不需要等待；有提交时把 N 次
//! 全设备停顿摊成一次。显存以 [`DeferredMemory`] 随句柄一起延后释放，且不持有设备 Arc，
//! 避免设备 ↔ 场的引用环。这是候选优化，不是原作 `dispose` 桶（桶按序号时间线排出）的镜像。
use super::device::VulkanDevice;
use ash::vk;
use parking_lot::Mutex;
use std::sync::{
    atomic::{AtomicU64, Ordering},
};

/// 累积到该数量后，入场路径会尝试（非阻塞地）顺带做一次回收。
pub const AUTO_COLLECT_THRESHOLD: usize = 64;

/// 脱离 `MemoryBlock` 的显存所有权：不含设备 Arc，释放时由设备完成。
pub enum DeferredMemory {
    /// 专用分配：整块 `VkDeviceMemory`。
    Dedicated {
        handle: vk::DeviceMemory,
        accounting_id: u64,
    },
    /// 子分配：归还 gpu-allocator 里的区间。
    Sub {
        allocation: gpu_allocator::vulkan::Allocation,
        accounting_id: u64,
    },
}

/// 等待销毁的 Vulkan 对象。
pub enum Doomed {
    Buffer(vk::Buffer, DeferredMemory),
    Image(vk::Image, DeferredMemory),
    ImageView(vk::ImageView),
    Sampler(vk::Sampler),
    RenderPass(vk::RenderPass),
    Framebuffer(vk::Framebuffer),
    Semaphore(vk::Semaphore),
    PipelineLayout(vk::PipelineLayout),
    Pipeline(vk::Pipeline),
    DescriptorSetLayout(vk::DescriptorSetLayout),
    DescriptorPool(vk::DescriptorPool),
    CommandPool(vk::CommandPool),
    QueryPool(vk::QueryPool),
}

#[derive(Default)]
pub struct Graveyard {
    entries: Mutex<Vec<(u64, Doomed)>>,
    submit_epoch: AtomicU64,
    completed_epoch: AtomicU64,
    idle_waits: AtomicU64,
}

/// 回收统计，供性能追踪使用。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GraveyardStats {
    pub pending: usize,
    pub submit_epoch: u64,
    pub completed_epoch: u64,
    /// 经本模块发起的整设备 idle 等待次数（性能追踪用，越少越好）。
    pub idle_waits: u64,
}

impl Graveyard {
    /// 调用者必须持有 `queue_lock`，保证与回收中的 idle 等待互斥。
    pub fn note_submit(&self) {
        self.submit_epoch.fetch_add(1, Ordering::AcqRel);
    }
    pub fn stats(&self) -> GraveyardStats {
        GraveyardStats {
            pending: self.entries.lock().len(),
            submit_epoch: self.submit_epoch.load(Ordering::Acquire),
            completed_epoch: self.completed_epoch.load(Ordering::Acquire),
            idle_waits: self.idle_waits.load(Ordering::Relaxed),
        }
    }
    /// 当前 epoch 之前的所有提交均已由 idle 确认完成。
    fn is_quiescent(&self) -> bool {
        self.completed_epoch.load(Ordering::Acquire) >= self.submit_epoch.load(Ordering::Acquire)
    }
    fn push(&self, doomed: Doomed) -> usize {
        let epoch = self.submit_epoch.load(Ordering::Acquire);
        let mut entries = self.entries.lock();
        entries.push((epoch, doomed));
        entries.len()
    }
    fn take_all(&self) -> Vec<(u64, Doomed)> {
        std::mem::take(&mut *self.entries.lock())
    }
    fn put_back(&self, mut rest: Vec<(u64, Doomed)>) {
        if rest.is_empty() {
            return;
        }
        let mut entries = self.entries.lock();
        rest.append(&mut entries);
        *entries = rest;
    }
}

impl VulkanDevice {
    /// 提交前（持 `queue_lock`）调用；见模块说明。
    pub fn note_submit(&self) {
        self.graveyard.note_submit();
    }
    /// 把对象送入延迟销毁场。达到阈值时尝试非阻塞回收（队列锁被占用则放弃，留待下次）。
    pub fn bury(&self, doomed: Doomed) {
        // 入场前发出的提交都已确认完成：无需等待，就地销毁（保持“无在飞 GPU 工作时立即释放”）。
        if self.graveyard.is_quiescent() {
            self.destroy_now(doomed);
            return;
        }
        if self.graveyard.push(doomed) >= AUTO_COLLECT_THRESHOLD {
            if let Some(guard) = self.queue_lock.try_lock() {
                self.collect_locked();
                drop(guard);
            }
        }
    }
    /// 阻塞回收：必要时做一次整设备 idle，然后销毁所有已确认完成的项，返回销毁数量。
    pub fn collect_garbage(&self) -> usize {
        let _guard = self.queue_lock.lock();
        self.collect_locked()
    }
    /// 整设备 idle 并回收：同时把完成线推进到当前提交上界，使之后的释放可就地进行。
    pub fn wait_idle_and_collect(&self) -> Result<usize, vk::Result> {
        let _guard = self.queue_lock.lock();
        let target = self.graveyard.submit_epoch.load(Ordering::Acquire);
        self.graveyard.idle_waits.fetch_add(1, Ordering::Relaxed);
        unsafe { self.device.device_wait_idle() }?;
        self.graveyard
            .completed_epoch
            .fetch_max(target, Ordering::AcqRel);
        Ok(self.collect_locked())
    }
    /// 宿主访问前的同步：仅当存在未确认完成的提交时才整设备 idle，并推进完成线。
    /// 调用者必须已持有 `queue_lock`。
    pub fn idle_if_in_flight_locked(&self) -> Result<(), vk::Result> {
        if self.graveyard.is_quiescent() {
            return Ok(());
        }
        let target = self.graveyard.submit_epoch.load(Ordering::Acquire);
        self.graveyard.idle_waits.fetch_add(1, Ordering::Relaxed);
        unsafe { self.device.device_wait_idle() }?;
        self.graveyard
            .completed_epoch
            .fetch_max(target, Ordering::AcqRel);
        Ok(())
    }
    pub fn garbage_stats(&self) -> GraveyardStats {
        self.graveyard.stats()
    }
    /// 调用时必须已持有 `queue_lock`。
    fn collect_locked(&self) -> usize {
        let entries = self.graveyard.take_all();
        if entries.is_empty() {
            return 0;
        }
        let newest = entries.iter().map(|(epoch, _)| *epoch).max().unwrap_or(0);
        if self.graveyard.completed_epoch.load(Ordering::Acquire) < newest {
            // 持锁期间不会有新的提交；此刻的 submit_epoch 即 idle 后确认完成的上界。
            let target = self.graveyard.submit_epoch.load(Ordering::Acquire);
            self.graveyard.idle_waits.fetch_add(1, Ordering::Relaxed);
            let _ = unsafe { self.device.device_wait_idle() };
            self.graveyard
                .completed_epoch
                .fetch_max(target, Ordering::AcqRel);
        }
        let completed = self.graveyard.completed_epoch.load(Ordering::Acquire);
        let (ready, rest): (Vec<_>, Vec<_>) =
            entries.into_iter().partition(|(epoch, _)| *epoch <= completed);
        self.graveyard.put_back(rest);
        let count = ready.len();
        for (_, doomed) in ready {
            self.destroy_now(doomed);
        }
        count
    }
    /// 设备销毁前无条件排空（调用者已保证无其他引用）。
    pub(crate) fn drain_graveyard_for_drop(&self) {
        let entries = self.graveyard.take_all();
        if entries.is_empty() {
            return;
        }
        let _ = unsafe { self.device.device_wait_idle() };
        for (_, doomed) in entries {
            self.destroy_now(doomed);
        }
    }
    fn destroy_now(&self, doomed: Doomed) {
        unsafe {
            match doomed {
                Doomed::Buffer(handle, memory) => {
                    self.device.destroy_buffer(handle, None);
                    self.free_deferred(memory);
                }
                Doomed::Image(handle, memory) => {
                    self.device.destroy_image(handle, None);
                    self.free_deferred(memory);
                }
                Doomed::ImageView(handle) => self.device.destroy_image_view(handle, None),
                Doomed::Sampler(handle) => self.device.destroy_sampler(handle, None),
                Doomed::RenderPass(handle) => self.device.destroy_render_pass(handle, None),
                Doomed::Framebuffer(handle) => self.device.destroy_framebuffer(handle, None),
                Doomed::Semaphore(handle) => self.device.destroy_semaphore(handle, None),
                Doomed::PipelineLayout(handle) => self.device.destroy_pipeline_layout(handle, None),
                Doomed::Pipeline(handle) => self.device.destroy_pipeline(handle, None),
                Doomed::DescriptorSetLayout(handle) => {
                    self.device.destroy_descriptor_set_layout(handle, None)
                }
                Doomed::DescriptorPool(handle) => self.device.destroy_descriptor_pool(handle, None),
                Doomed::CommandPool(handle) => self.device.destroy_command_pool(handle, None),
                Doomed::QueryPool(handle) => self.device.destroy_query_pool(handle, None),
            }
        }
    }
    unsafe fn free_deferred(&self, memory: DeferredMemory) {
        let accounting_id = match memory {
            DeferredMemory::Dedicated {
                handle,
                accounting_id,
            } => {
                unsafe { self.device.free_memory(handle, None) };
                accounting_id
            }
            DeferredMemory::Sub {
                allocation,
                accounting_id,
            } => {
                self.free_suballocation(allocation);
                accounting_id
            }
        };
        self.allocator_stats
            .release(accounting_id)
            .expect("内存登记与释放必须一一对应");
    }
}
