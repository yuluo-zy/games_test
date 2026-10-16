//! 兼容资源的延迟销毁适配；完成条件统一来自设备每队列时间线。
use super::device::VulkanDevice;
use crate::sync::{CompletedValues, SyncSet};
use ash::vk;
use std::sync::atomic::Ordering;
pub const AUTO_COLLECT_THRESHOLD: usize = 64;
pub enum DeferredMemory {
    /// 专用分配：整块 `VkDeviceMemory`。
    Dedicated {
        handle: vk::DeviceMemory,
        accounting_id: u64,
    },
    /// 子分配：归还 gpu-allocator 里的区间。
    #[cfg(feature = "suballocation")]
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GraveyardStats {
    pub pending: usize,
    pub submit_epoch: u64,
    pub completed_epoch: u64,
    pub idle_waits: u64,
}
/// 原 `FUN_1418c7ae0(queue+0x40, cb_seq, dev)`：按命令序号（`cb_seq`，命令项 `+0x08`）**严格顺序**退役。
///
/// 机器码/反编译（`producer-closure/*/0x1418c7ae0.c`）：
/// - 结构 `{cap, ptr, head, len}`（`VecDeque<u64>`，0 = 空洞）+ `next@+0x20`（已退役的最大序号）；
/// - `seq <= next` → panic（已退役或重复，消息 0x142d06fd8，长 0x29）；
/// - `seq == next + 1` → `next = seq`，发 profiler 事件 9（载荷 `0x1e, seq`），调用
///   `dispose_in_order(dev+0xa50, seq)`（0x1418c4100）取出该序号桶，再 `FUN_140bf92f0(dev, 桶)` 销毁；
///   然后弹出队首空洞，并循环：队首非 0 → 弹出、`next += 1`、断言 `next == 值`、同样退役；
/// - `seq > next + 1` → 暂存于环内下标 `seq - next - 1`（不足补 0，非 0 槽重复 → panic）。
///
/// 本类型只做序号保序（返回应按序退役的序号列表）；“桶销毁”由 `VulkanDevice::bury` 的 epoch 机制承担
/// （⚠️ 未改成原版按序号桶：原桶按 cb_seq 归属，重建按提交 epoch，语义等价于“序号 ≤ 完成线”）。
#[derive(Debug, Default, Clone)]
pub struct InOrderRetire {
    next: u64,
    slots: std::collections::VecDeque<u64>,
}

impl InOrderRetire {
    pub fn new() -> Self {
        Self::default()
    }
    /// 已退役的最大序号（原 `+0x20`）。
    pub fn next(&self) -> u64 {
        self.next
    }
    /// 暂存环长度（原 `+0x18`）。
    pub fn parked(&self) -> usize {
        self.slots.len()
    }
    /// 登记 `seq` 完成；返回此刻可退役的序号（严格递增、连续）。
    ///
    /// # Panics
    /// `seq <= next`、`seq` 重复暂存，或出队序号与期望不连续（原版同样 panic）。
    pub fn retire(&mut self, seq: u64) -> Vec<u64> {
        assert!(seq > self.next, "命令序号已退役或重复（0x1418c7ae0）");
        let offset = seq - self.next - 1;
        if offset != 0 {
            let offset = offset as usize;
            if offset > self.slots.len() {
                self.slots.resize(offset, 0);
            }
            if offset == self.slots.len() {
                self.slots.push_back(seq);
            } else {
                assert_eq!(self.slots[offset], 0, "暂存槽重复写入（0x1418c7ae0）");
                self.slots[offset] = seq;
            }
            return Vec::new();
        }
        self.next = seq;
        let mut out = vec![seq];
        // 弹出队首（下标 0 对应刚退役的 seq，恒为空洞）。
        self.slots.pop_front();
        while let Some(&front) = self.slots.front() {
            if front == 0 {
                break;
            }
            self.slots.pop_front();
            self.next += 1;
            assert_eq!(self.next, front, "出队序号不连续（0x1418c7ae0）");
            out.push(front);
        }
        out
    }
}

impl VulkanDevice {
    /// 旧资源尚未携带 last_use 时，保守等待当前所有队列已提交的工作。
    pub fn bury(&self, doomed: Doomed) {
        self.retire(self.timelines.submitted_set(), doomed);
    }
    pub fn retire(&self, after: SyncSet, doomed: Doomed) {
        let completed = match self.timelines.poll() {
            Ok(done) => done,
            Err(crate::Error::DeviceLost) => CompletedValues::all(),
            Err(_) => CompletedValues::default(),
        };
        let ready = self.retirement.lock().retire(after, doomed, &completed);
        if let Some(doomed) = ready {
            self.destroy_now(doomed);
        }
        if self.retirement.lock().len() >= AUTO_COLLECT_THRESHOLD {
            self.collect_garbage();
        }
    }
    pub fn collect_garbage(&self) -> usize {
        self.try_collect_garbage().unwrap_or(0)
    }
    pub fn try_collect_garbage(&self) -> crate::Result<usize> {
        let completed = match self.timelines.poll() {
            Err(crate::Error::DeviceLost) => CompletedValues::all(),
            other => other?,
        };
        let mut ready = Vec::new();
        self.retirement.lock().collect(&completed, &mut ready);
        let count = ready.len();
        for doomed in ready {
            self.destroy_now(doomed);
        }
        Ok(count)
    }
    pub fn wait_idle_and_collect(&self) -> Result<usize, vk::Result> {
        self.idle_waits.fetch_add(1, Ordering::Relaxed);
        self.timelines
            .with_all_queues(|| unsafe { self.device.device_wait_idle() }.map_err(Into::into))
            .map_err(crate::device::legacy_vk_error)?;
        self.try_collect_garbage()
            .map_err(crate::device::legacy_vk_error)
    }
    pub fn wait_submitted(&self) -> Result<(), vk::Result> {
        self.timelines
            .wait(&self.timelines.submitted_set(), u64::MAX)
            .map_err(crate::device::legacy_vk_error)
    }
    pub fn garbage_stats(&self) -> GraveyardStats {
        let submitted = self.timelines.submitted_set();
        let completed = self.timelines.poll().unwrap_or_default();
        GraveyardStats {
            pending: self.retirement.lock().len(),
            submit_epoch: submitted.points().map(|p| p.value).sum(),
            completed_epoch: submitted.points().map(|p| completed.get(p.queue)).sum(),
            idle_waits: self.idle_waits.load(Ordering::Relaxed),
        }
    }
    pub(crate) fn drain_graveyard_for_drop(&self) {
        let _ = unsafe { self.device.device_wait_idle() };
        let mut ready = Vec::new();
        self.retirement.lock().drain_all(&mut ready);
        for doomed in ready {
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
            #[cfg(feature = "suballocation")]
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
