//! 设备级延迟销毁场：Drop 路径 O(1) 入场，从不为销毁而整设备 idle。
//!
//! 与原作 `dispose` 桶（按序号时间线排出）同构：每次主队列提交在 `queue_lock` 内取得单调递增的
//! epoch，并在该次提交末尾让设备持有的“退役时间线信号量”signal 到这个 epoch；入场项记录入场时的
//! epoch，只有时间线计数 `>= epoch`（入场前发出的所有主队列提交已由 GPU 完成）才销毁，查询用
//! `vkGetSemaphoreCounterValue`，不阻塞。非主队列的提交不在这条时间线上，保守地要求一次整设备
//! idle 才能确认；为免入场无界增长，入场累积到 [`AUTO_COLLECT_THRESHOLD`] 且能非阻塞拿到
//! `queue_lock` 时，若仍有未确认的非主队列提交，就地做一次整设备 idle 并回收（唯一的入场侧 idle）。
//! 显存以 [`DeferredMemory`] 随句柄一起延后释放，且不持有设备 Arc，避免引用环。
//!
//! ⚠️ 本模块是宿主/候选层策略（原作按 `cb_seq` 桶退役，见 [`InOrderRetire`]），非原版机器码行为。
use super::device::VulkanDevice;
use ash::vk;
use parking_lot::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

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

#[derive(Default)]
pub struct Graveyard {
    entries: Mutex<Vec<(u64, Doomed)>>,
    submit_epoch: AtomicU64,
    /// 已确认完成的 epoch 上界（时间线计数或整设备 idle 推进）。
    completed_epoch: AtomicU64,
    /// 最近一次非主队列提交的 epoch；未被 idle 确认之前，时间线计数不能证明完成。
    foreign_epoch: AtomicU64,
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

impl Graveyard {
    /// 调用者必须持有 `queue_lock`；返回本次提交的 epoch。
    fn next_epoch(&self, primary: bool) -> u64 {
        let epoch = self.submit_epoch.fetch_add(1, Ordering::AcqRel) + 1;
        if !primary {
            self.foreign_epoch.store(epoch, Ordering::Release);
        }
        epoch
    }
    pub fn stats(&self) -> GraveyardStats {
        GraveyardStats {
            pending: self.entries.lock().len(),
            submit_epoch: self.submit_epoch.load(Ordering::Acquire),
            completed_epoch: self.completed_epoch.load(Ordering::Acquire),
            idle_waits: self.idle_waits.load(Ordering::Relaxed),
        }
    }
    fn is_quiescent(&self) -> bool {
        self.completed_epoch.load(Ordering::Acquire) >= self.submit_epoch.load(Ordering::Acquire)
    }
    fn foreign_unconfirmed(&self) -> bool {
        self.foreign_epoch.load(Ordering::Acquire) > self.completed_epoch.load(Ordering::Acquire)
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
    /// 提交前（持 `queue_lock`）调用。返回需要附在这次提交末尾的退役信号 `(信号量, 值)`；
    /// 非主队列提交不在时间线上，返回 `None`，其完成只能由整设备 idle 确认。
    pub fn note_submit_on(&self, queue: vk::Queue) -> Option<(vk::Semaphore, u64)> {
        let primary = queue == self.queue;
        let epoch = self.graveyard.next_epoch(primary);
        primary.then_some((self.retire_timeline, epoch))
    }
    /// 提交失败时由宿主补发退役信号，保证时间线不会停在这个 epoch 之前。调用者必须持有 `queue_lock`。
    ///
    /// 失败的提交不会 signal，但更早的 epoch（`< value`）可能仍在 GPU 上：直接主机 signal 到 `value`
    /// 会让完成线越过在途工作（`destroy_now` 释放仍在用的资源），且新值小于挂起的 signal 违反
    /// VUID-VkSemaphoreSignalInfo-value-03258/03260。故先整设备 idle（`queue_lock` 已挡住新提交），
    /// 确认全部已发提交完成后再补发；idle 失败（如设备丢失）则不补发、不推进完成线。
    pub fn note_submit_failed(&self, signal: Option<(vk::Semaphore, u64)>) {
        let Some((semaphore, value)) = signal else {
            return;
        };
        if self.idle_and_confirm_locked().is_err() {
            return;
        }
        let info = vk::SemaphoreSignalInfo::default().semaphore(semaphore).value(value);
        let _ = unsafe { self.device.signal_semaphore(&info) };
    }
    /// 整设备 idle，把完成线推进到当前提交上界（含非主队列提交）。调用者必须持有 `queue_lock`。
    fn idle_and_confirm_locked(&self) -> Result<(), vk::Result> {
        let target = self.graveyard.submit_epoch.load(Ordering::Acquire);
        self.graveyard.idle_waits.fetch_add(1, Ordering::Relaxed);
        unsafe { self.device.device_wait_idle() }?;
        self.graveyard.completed_epoch.fetch_max(target, Ordering::AcqRel);
        Ok(())
    }
    /// 用时间线计数推进完成线（非阻塞）；有未确认的非主队列提交时不推进。
    fn refresh_completed(&self) {
        if self.graveyard.foreign_unconfirmed() {
            return;
        }
        if let Ok(value) = unsafe { self.device.get_semaphore_counter_value(self.retire_timeline) } {
            self.graveyard.completed_epoch.fetch_max(value, Ordering::AcqRel);
        }
    }
    /// 把对象送入延迟销毁场。入场前的提交都已完成则就地销毁；达到阈值时尝试非阻塞回收。
    ///
    /// 有未确认的非主队列提交时时间线无法推进完成线，阈值处改为整设备 idle 再回收，否则永不回收。
    pub fn bury(&self, doomed: Doomed) {
        self.refresh_completed();
        if self.graveyard.is_quiescent() {
            self.destroy_now(doomed);
            return;
        }
        if self.graveyard.push(doomed) >= AUTO_COLLECT_THRESHOLD
            && let Some(guard) = self.queue_lock.try_lock()
        {
            if self.graveyard.foreign_unconfirmed() {
                // idle 失败（设备丢失）时完成线不动，只回收已确认部分。
                let _ = self.idle_and_confirm_locked();
            }
            self.collect_locked();
            drop(guard);
        }
    }
    /// 非阻塞回收：销毁所有已确认完成的项，返回销毁数量。
    pub fn collect_garbage(&self) -> usize {
        let _guard = self.queue_lock.lock();
        self.collect_locked()
    }
    /// 整设备 idle 并回收：把完成线推进到当前提交上界。
    pub fn wait_idle_and_collect(&self) -> Result<usize, vk::Result> {
        let _guard = self.queue_lock.lock();
        self.idle_and_confirm_locked()?;
        Ok(self.collect_locked())
    }
    /// 宿主访问前的同步：等待时间线到当前 epoch（只等已提交的工作，不整设备停顿）；
    /// 有非主队列提交时退回整设备 idle。调用者必须已持有 `queue_lock`。
    pub fn idle_if_in_flight_locked(&self) -> Result<(), vk::Result> {
        self.refresh_completed();
        if self.graveyard.is_quiescent() {
            return Ok(());
        }
        if self.graveyard.foreign_unconfirmed() {
            return self.idle_and_confirm_locked();
        }
        let target = self.graveyard.submit_epoch.load(Ordering::Acquire);
        let semaphores = [self.retire_timeline];
        let values = [target];
        let info = vk::SemaphoreWaitInfo::default().semaphores(&semaphores).values(&values);
        unsafe { self.device.wait_semaphores(&info, u64::MAX) }?;
        self.graveyard.completed_epoch.fetch_max(target, Ordering::AcqRel);
        Ok(())
    }
    pub fn garbage_stats(&self) -> GraveyardStats {
        self.graveyard.stats()
    }
    /// 调用时必须已持有 `queue_lock`。不等待 GPU：只销毁完成线已覆盖的项。
    fn collect_locked(&self) -> usize {
        self.refresh_completed();
        let entries = self.graveyard.take_all();
        if entries.is_empty() {
            return 0;
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
