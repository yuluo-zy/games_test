//! 最小 Vulkan 专用分配适配。原 gpu-allocator 0.26 本地分叉来源待恢复，
//! 此处只调用分配原语，不重写该依赖的次分配/池策略。
use super::{
    device::Device,
    error::{Error, Result},
};
use ash::vk;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, sync::Arc};
pub fn choose_memory_type(
    memory: &vk::PhysicalDeviceMemoryProperties,
    bits: u32,
    required: vk::MemoryPropertyFlags,
    preferred: vk::MemoryPropertyFlags,
) -> Option<u32> {
    let candidates = (0..memory.memory_type_count).filter(|&i| {
        bits & (1 << i) != 0
            && memory.memory_types[i as usize]
                .property_flags
                .contains(required)
    });
    candidates
        .clone()
        .find(|&i| {
            memory.memory_types[i as usize]
                .property_flags
                .contains(preferred)
        })
        .or_else(|| candidates.into_iter().next())
}
pub struct MemoryBlock {
    pub device: Arc<Device>,
    pub handle: vk::DeviceMemory,
    /// 在 `handle` 所指内存对象内的偏移；专用分配恒为 0，子分配时绑定资源必须带上它。
    pub offset: u64,
    pub size: u64,
    pub properties: vk::MemoryPropertyFlags,
    pub(crate) accounting_id: u64,
    pub(crate) shared: Option<Arc<super::transient_backing::SharedBacking>>,
    /// 子分配时持有 gpu-allocator 的分配；专用分配为 None。
    #[cfg(feature = "suballocation")]
    pub(crate) sub: Option<super::suballoc::SubAllocation>,
}
impl MemoryBlock {
    /// 图像（最优平铺）的分配入口；设备启用子分配时走 gpu-allocator。
    pub fn allocate(
        device: Arc<Device>,
        requirements: vk::MemoryRequirements,
        required: vk::MemoryPropertyFlags,
        preferred: vk::MemoryPropertyFlags,
    ) -> Result<Self> {
        let flags = vk::MemoryAllocateFlags::empty();
        #[cfg(feature = "suballocation")]
        if device.suballocation_enabled() {
            return Self::allocate_sub(device, requirements, required, preferred, flags, false);
        }
        Self::allocate_dedicated(device, requirements, required, preferred, flags)
    }

    /// 缓冲（线性）的分配入口；设备启用子分配时走 gpu-allocator。
    pub fn allocate_with_flags(
        device: Arc<Device>,
        requirements: vk::MemoryRequirements,
        required: vk::MemoryPropertyFlags,
        preferred: vk::MemoryPropertyFlags,
        flags: vk::MemoryAllocateFlags,
    ) -> Result<Self> {
        #[cfg(feature = "suballocation")]
        if device.suballocation_enabled() {
            return Self::allocate_sub(device, requirements, required, preferred, flags, true);
        }
        Self::allocate_dedicated(device, requirements, required, preferred, flags)
    }

    /// 每次一块专用 `VkDeviceMemory`，不经子分配器（`DirectMemoryBackend` 与专用模式使用）。
    pub fn allocate_dedicated(
        device: Arc<Device>,
        requirements: vk::MemoryRequirements,
        required: vk::MemoryPropertyFlags,
        preferred: vk::MemoryPropertyFlags,
        flags: vk::MemoryAllocateFlags,
    ) -> Result<Self> {
        Self::allocate_for_resource(
            device,
            requirements,
            required,
            preferred,
            flags,
            vk::Buffer::null(),
            vk::Image::null(),
        )
    }

    /// Required dedicated allocations name the one resource they may bind.
    pub(crate) fn allocate_for_resource(
        device: Arc<Device>,
        requirements: vk::MemoryRequirements,
        required: vk::MemoryPropertyFlags,
        preferred: vk::MemoryPropertyFlags,
        flags: vk::MemoryAllocateFlags,
        buffer: vk::Buffer,
        image: vk::Image,
    ) -> Result<Self> {
        if device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        let index = choose_memory_type(
            &device.memory,
            requirements.memory_type_bits,
            required,
            preferred,
        )
        .ok_or(Error::unsupported("没有匹配的内存类型"))?;
        let properties = device.memory.memory_types[index as usize].property_flags;
        let mut allocation_flags = vk::MemoryAllocateFlagsInfo::default().flags(flags);
        let mut info = vk::MemoryAllocateInfo::default()
            .allocation_size(requirements.size)
            .memory_type_index(index);
        if !flags.is_empty() {
            info = info.push_next(&mut allocation_flags);
        }
        let mut dedicated = vk::MemoryDedicatedAllocateInfo::default()
            .buffer(buffer)
            .image(image);
        if buffer != vk::Buffer::null() || image != vk::Image::null() {
            info = info.push_next(&mut dedicated);
        }
        if requirements.size == 0
            || (!flags.is_empty()
                && flags.contains(vk::MemoryAllocateFlags::DEVICE_ADDRESS)
                && !device.capabilities.buffer_device_address)
        {
            return Err(Error::invalid_usage("内存大小或设备地址分配能力无效"));
        }
        let heap = device.memory.memory_types[index as usize].heap_index;
        let handle = super::error::observe_vulkan(&device, unsafe {
            device.device.allocate_memory(&info, None)
        })?;
        let accounting_id = match device.allocator_stats.register(
            requirements.size,
            index,
            heap,
            properties,
            String::new(),
        ) {
            Ok(id) => id,
            Err(error) => {
                unsafe {
                    device.device.free_memory(handle, None);
                }
                return Err(error);
            }
        };
        Ok(Self {
            device,
            handle,
            offset: 0,
            size: requirements.size,
            properties,
            accounting_id,
            shared: None,
            #[cfg(feature = "suballocation")]
            sub: None,
        })
    }
}
impl MemoryBlock {
    /// 脱离设备 Arc，把显存所有权转交延迟销毁场；之后由设备在确认 GPU 不再使用后释放。
    pub(crate) fn into_deferred(self) -> super::graveyard::DeferredMemory {
        if self.shared.is_some() {
            let mut memory = std::mem::ManuallyDrop::new(self);
            let backing = memory.shared.take().expect("checked shared allocation");
            backing.retire_member();
            drop(unsafe { std::ptr::read(&memory.device) });
            return super::graveyard::DeferredMemory::Shared(backing);
        }
        self.device
            .allocator_stats
            .mark_retiring(self.accounting_id);
        #[cfg(feature = "suballocation")]
        let (memory, sub) = {
            let mut block = self;
            let sub = block.sub.take();
            (block, sub)
        };
        #[cfg(not(feature = "suballocation"))]
        let memory = self;
        let me = std::mem::ManuallyDrop::new(memory);
        // SAFETY: `me` 不会再被 Drop，Arc 只在此处读出并释放一次；调用者仍持有另一个设备 Arc。
        drop(unsafe { std::ptr::read(&me.device) });
        #[cfg(feature = "suballocation")]
        return match sub {
            Some(allocation) => super::graveyard::DeferredMemory::Sub { allocation },
            None => super::graveyard::DeferredMemory::Dedicated {
                handle: me.handle,
                accounting_id: me.accounting_id,
            },
        };
        #[cfg(not(feature = "suballocation"))]
        super::graveyard::DeferredMemory::Dedicated {
            handle: me.handle,
            accounting_id: me.accounting_id,
        }
    }
    pub fn accounting_id(&self) -> u64 {
        self.accounting_id
    }
    pub(crate) fn from_shared(
        device: Arc<Device>,
        backing: Arc<super::transient_backing::SharedBacking>,
    ) -> Self {
        backing.add_member();
        Self {
            device,
            handle: backing.handle,
            offset: backing.offset,
            size: backing.size,
            properties: backing.properties,
            accounting_id: backing.accounting_id,
            shared: Some(backing),
            #[cfg(feature = "suballocation")]
            sub: None,
        }
    }
    pub(crate) fn shared_use(&self) -> Option<Arc<crate::resource::SyncCell>> {
        self.shared.as_ref().map(|backing| backing.last_use.clone())
    }
    pub(crate) fn is_shared(&self) -> bool {
        self.shared.is_some()
    }
    pub fn is_suballocated(&self) -> bool {
        #[cfg(feature = "suballocation")]
        return self.sub.is_some()
            || self
                .shared
                .as_ref()
                .is_some_and(|backing| backing.is_suballocated());
        #[cfg(not(feature = "suballocation"))]
        false
    }
    /// 子分配且宿主可见时，gpu-allocator 已持久映射整块并给出本分配起点的指针（已含偏移）。
    /// 返回 None 表示需要调用者自己 `vkMapMemory`（专用分配）。
    pub fn host_ptr(&self) -> Option<*mut u8> {
        #[cfg(feature = "suballocation")]
        return self
            .sub
            .as_ref()
            .and_then(|a| a.mapped_ptr())
            .map(|p| p.as_ptr().cast::<u8>())
            .or_else(|| self.shared.as_ref().and_then(|backing| backing.host_ptr()));
        #[cfg(not(feature = "suballocation"))]
        None
    }
    /// 刷新/失效范围 `(偏移, 大小)`：专用分配覆盖整个内存对象；子分配只覆盖本分配自己的范围
    /// （创建时已按非一致性粒度对齐，不会波及邻居）。
    pub fn host_range(&self) -> (u64, u64) {
        #[cfg(feature = "suballocation")]
        if self
            .shared
            .as_ref()
            .is_some_and(|backing| backing.is_suballocated())
        {
            return (self.offset, self.size);
        }
        #[cfg(feature = "suballocation")]
        return match &self.sub {
            Some(allocation) => (allocation.offset(), allocation.size()),
            None => (0, vk::WHOLE_SIZE),
        };
        #[cfg(not(feature = "suballocation"))]
        (0, vk::WHOLE_SIZE)
    }
    pub(crate) fn host_subrange(
        &self,
        buffer_size: u64,
        offset: u64,
        size: u64,
        atom: u64,
    ) -> Result<HostMemoryRange> {
        let dedicated = !self.is_shared();
        #[cfg(feature = "suballocation")]
        let dedicated = dedicated && self.sub.is_none();
        mapped_host_subrange(
            self.offset,
            self.size,
            buffer_size,
            offset,
            size,
            atom,
            dedicated,
        )
    }
}

pub(crate) struct HostMemoryRange {
    pub memory_offset: u64,
    pub memory_size: u64,
    pub buffer_offset: u64,
    pub buffer_size: u64,
}
fn mapped_host_subrange(
    base: u64,
    allocation_size: u64,
    buffer_size: u64,
    offset: u64,
    size: u64,
    atom: u64,
    dedicated: bool,
) -> Result<HostMemoryRange> {
    let invalid = || Error::invalid_usage("mapped buffer interval exceeds its allocation");
    let end = offset
        .checked_add(size)
        .filter(|end| size != 0 && *end <= buffer_size && buffer_size <= allocation_size)
        .ok_or_else(invalid)?;
    let physical_start = base.checked_add(offset).ok_or_else(invalid)?;
    let physical_end = base.checked_add(end).ok_or_else(invalid)?;
    let allocation_end = base.checked_add(allocation_size).ok_or_else(invalid)?;
    let atom = atom.max(1);
    let start = physical_start - physical_start % atom;
    let end = physical_end
        .checked_add((atom - physical_end % atom) % atom)
        .ok_or_else(invalid)?
        .min(allocation_end);
    if start < base || (!dedicated && !end.is_multiple_of(atom)) {
        return Err(Error::unsupported(
            "mapped allocation does not isolate non-coherent atoms",
        ));
    }
    Ok(HostMemoryRange {
        memory_offset: start,
        memory_size: if !((end - start).is_multiple_of(atom)) {
            vk::WHOLE_SIZE
        } else {
            end - start
        },
        buffer_offset: start - base,
        buffer_size: (end - base).min(buffer_size) - (start - base),
    })
}

impl Drop for MemoryBlock {
    fn drop(&mut self) {
        if let Some(backing) = self.shared.take() {
            backing.retire_member();
            drop(backing);
            return;
        }
        #[cfg(feature = "suballocation")]
        match self.sub.take() {
            Some(allocation) => {
                drop(allocation);
                return;
            }
            None => unsafe {
                self.device.device.free_memory(self.handle, None);
            },
        }
        #[cfg(not(feature = "suballocation"))]
        unsafe {
            self.device.device.free_memory(self.handle, None)
        };
        self.device
            .allocator_stats
            .release(self.accounting_id)
            .expect("内存登记与释放必须一一对应");
    }
}

/// 原 MemoryLocation 的公开依赖语义：控制 GPU/宿主可见属性，不实现次分配算法。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryLocation {
    GpuOnly,
    CpuToGpu,
    GpuToCpu,
    Unknown,
}
impl MemoryLocation {
    pub fn properties(self) -> (vk::MemoryPropertyFlags, vk::MemoryPropertyFlags) {
        match self {
            Self::GpuOnly => (
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
            ),
            Self::CpuToGpu => (
                vk::MemoryPropertyFlags::HOST_VISIBLE,
                vk::MemoryPropertyFlags::HOST_COHERENT,
            ),
            Self::GpuToCpu => (
                vk::MemoryPropertyFlags::HOST_VISIBLE,
                vk::MemoryPropertyFlags::HOST_CACHED | vk::MemoryPropertyFlags::HOST_COHERENT,
            ),
            Self::Unknown => (
                vk::MemoryPropertyFlags::empty(),
                vk::MemoryPropertyFlags::empty(),
            ),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AllocationTotals {
    pub total_allocated_bytes: u64,
    pub device_local_bytes: u64,
    pub system_bytes: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AllocationRecord {
    pub id: u64,
    pub size: u64,
    pub memory_type: u32,
    pub heap: u32,
    pub properties: u32,
    pub name: String,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AllocatorReport {
    pub totals: AllocationTotals,
    pub allocations: Vec<AllocationRecord>,
}
/// 设备级专用 VkDeviceMemory 的统计；这是适配器的独立分配统计，不冒充原池预留量。
#[derive(Default)]
pub struct AllocatorStats {
    update_after_bind_descriptors: std::sync::atomic::AtomicU64,
    state: Mutex<StatisticsState>,
}
/// Known host allocations participate in the same budget, but never become
/// VkDeviceMemory records or physical-GPU allocation totals. The ticket follows
/// the actual owner; cache removal cannot credit bytes held by an active lease.
pub(crate) struct CpuMemoryReservation {
    stats: std::sync::Arc<AllocatorStats>,
    bytes: u64,
    category: crate::device::BudgetCategory,
}
/// Known payload plus the two reference counters of a shared allocation. The
/// reservation stays in the payload, including when its Arc escapes a cache.
pub(crate) fn shared_cpu_bytes<T>() -> Result<u64> {
    let (layout, _) = std::alloc::Layout::new::<[usize; 2]>()
        .extend(std::alloc::Layout::new::<T>())
        .map_err(|_| Error::OutOfHostMemory)?;
    u64::try_from(layout.pad_to_align().size()).map_err(|_| Error::OutOfHostMemory)
}
/// Admit renderer-owned Arc payload and separately owned CPU buffers before a
/// native object is created. Keep this ticket in the actual allocation owner.
pub(crate) fn reserve_shared_cpu_metadata<T>(
    stats: &std::sync::Arc<AllocatorStats>,
    extra_bytes: u64,
) -> Result<CpuMemoryReservation> {
    let bytes = shared_cpu_bytes::<T>()?
        .checked_add(extra_bytes)
        .ok_or(Error::OutOfHostMemory)?;
    stats.reserve_cpu_bytes(bytes, crate::device::BudgetCategory::Live)
}
pub(crate) fn cpu_vec_bytes<T>(capacity: usize) -> Result<u64> {
    let layout = std::alloc::Layout::array::<T>(capacity).map_err(|_| Error::OutOfHostMemory)?;
    u64::try_from(layout.size()).map_err(|_| Error::OutOfHostMemory)
}
impl CpuMemoryReservation {
    pub(crate) fn resize(&mut self, bytes: u64) -> Result<()> {
        let mut state = self.stats.state.lock();
        if bytes > self.bytes {
            state
                .budget
                .reserve(self.category, bytes - self.bytes)
                .map_err(|_| Error::OutOfHostMemory)?;
        } else {
            state.budget.release(self.category, self.bytes - bytes);
        }
        match self.category {
            crate::device::BudgetCategory::Live => {
                state.cpu_usage.live = state.cpu_usage.live - self.bytes + bytes
            }
            crate::device::BudgetCategory::Cache => {
                state.cpu_usage.cache = state.cpu_usage.cache - self.bytes + bytes
            }
            _ => unreachable!("CPU allocations have no GPU completion domain"),
        }
        self.bytes = bytes;
        state.cpu_peak_bytes = state.cpu_peak_bytes.max(state.cpu_usage.total());
        Ok(())
    }
    pub(crate) fn reclassify(&mut self, category: crate::device::BudgetCategory) {
        use crate::device::BudgetCategory;
        debug_assert!(matches!(
            category,
            BudgetCategory::Live | BudgetCategory::Cache
        ));
        if category == self.category {
            return;
        }
        let mut state = self.stats.state.lock();
        state.budget.reclassify(self.category, category, self.bytes);
        match self.category {
            BudgetCategory::Live => state.cpu_usage.live -= self.bytes,
            BudgetCategory::Cache => state.cpu_usage.cache -= self.bytes,
            _ => unreachable!("CPU allocations have no GPU completion domain"),
        }
        match category {
            BudgetCategory::Live => state.cpu_usage.live += self.bytes,
            BudgetCategory::Cache => state.cpu_usage.cache += self.bytes,
            _ => unreachable!("CPU allocations have no GPU completion domain"),
        }
        self.category = category;
    }
}
impl Drop for CpuMemoryReservation {
    fn drop(&mut self) {
        let mut state = self.stats.state.lock();
        state.budget.release(self.category, self.bytes);
        match self.category {
            crate::device::BudgetCategory::Live => state.cpu_usage.live -= self.bytes,
            crate::device::BudgetCategory::Cache => state.cpu_usage.cache -= self.bytes,
            _ => unreachable!("CPU allocations have no GPU completion domain"),
        }
    }
}
pub(crate) struct DescriptorReservation {
    stats: std::sync::Arc<AllocatorStats>,
    count: u64,
}
impl Drop for DescriptorReservation {
    fn drop(&mut self) {
        self.stats
            .update_after_bind_descriptors
            .fetch_sub(self.count, std::sync::atomic::Ordering::AcqRel);
    }
}
#[derive(Default)]
struct StatisticsState {
    budget: crate::device::MemoryBudgetController,
    cpu_usage: crate::device::BudgetUsage,
    cpu_peak_bytes: u64,
    phases: BTreeMap<u64, AllocationPhase>,
    next_id: u64,
    records: BTreeMap<u64, AllocationRecord>,
    totals: AllocationTotals,
    peak: AllocationTotals,
    peak_report: Option<AllocatorReport>,
    /// 峰值刚被刷新、快照尚未物化。峰值之后只可能出现登记（继续刷新峰值）或释放；
    /// 因此在首次释放前或被查询时物化，得到的状态与“刷新当刻”完全一致，登记路径保持 O(1)。
    peak_dirty: bool,
    heap_budgets: BTreeMap<u32, u64>,
    /// 各堆当前占用，增量维护，避免每次登记遍历全部记录。
    heap_used: BTreeMap<u32, u64>,
    /// Native allocator capacity not occupied by a live resource range. It is
    /// known GPU cache memory, distinct from host cache reservations.
    suballocator_spare: [u64; 32],
}
struct AllocationPhase {
    category: crate::device::BudgetCategory,
    last_use: crate::sync::SyncSet,
}
impl StatisticsState {
    fn report(&self) -> AllocatorReport {
        AllocatorReport {
            totals: self.totals,
            allocations: self.records.values().cloned().collect(),
        }
    }
    fn materialize_peak(&mut self) {
        if self.peak_dirty {
            self.peak_report = Some(self.report());
            self.peak_dirty = false;
        }
    }
}
impl AllocatorStats {
    pub(crate) fn reserve_cpu_bytes(
        self: &std::sync::Arc<Self>,
        bytes: u64,
        category: crate::device::BudgetCategory,
    ) -> Result<CpuMemoryReservation> {
        use crate::device::BudgetCategory;
        if !matches!(category, BudgetCategory::Live | BudgetCategory::Cache) {
            return Err(Error::invalid_usage(
                "CPU allocation must be live or cached",
            ));
        }
        let mut state = self.state.lock();
        state
            .budget
            .reserve(category, bytes)
            .map_err(|_| Error::OutOfHostMemory)?;
        match category {
            BudgetCategory::Live => state.cpu_usage.live += bytes,
            BudgetCategory::Cache => state.cpu_usage.cache += bytes,
            _ => unreachable!(),
        }
        state.cpu_peak_bytes = state.cpu_peak_bytes.max(state.cpu_usage.total());
        Ok(CpuMemoryReservation {
            stats: self.clone(),
            bytes,
            category,
        })
    }
    pub fn cpu_memory_usage(&self) -> crate::device::BudgetUsage {
        self.state.lock().cpu_usage
    }
    pub fn cpu_peak_bytes(&self) -> u64 {
        self.state.lock().cpu_peak_bytes
    }
    pub(crate) fn reserve_update_after_bind_descriptors(
        self: &std::sync::Arc<Self>,
        count: u64,
        limit: u64,
    ) -> Result<DescriptorReservation> {
        use std::sync::atomic::Ordering;
        self.update_after_bind_descriptors
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |used| {
                used.checked_add(count).filter(|next| *next <= limit)
            })
            .map_err(|_| Error::OutOfDeviceMemory)?;
        Ok(DescriptorReservation {
            stats: self.clone(),
            count,
        })
    }
    pub fn set_total_budget(&self, bytes: u64) -> Result<()> {
        self.state
            .lock()
            .budget
            .set_limit(bytes)
            .map_err(|_| Error::OutOfDeviceMemory)
    }
    pub fn budget_usage(&self) -> crate::device::BudgetUsage {
        self.state.lock().budget.usage()
    }
    #[cfg(feature = "suballocation")]
    pub(crate) fn budget_limit(&self) -> u64 {
        self.state.lock().budget.limit()
    }
    #[cfg(feature = "suballocation")]
    pub(crate) fn budget_available(&self) -> u64 {
        self.state.lock().budget.available()
    }
    pub fn suballocator_spare_bytes(&self) -> u64 {
        self.state.lock().suballocator_spare.iter().sum()
    }
    pub fn budget_action(&self) -> crate::device::BudgetAction {
        self.state.lock().budget.tick()
    }
    pub(crate) fn record_gpu_use(&self, id: u64, point: crate::sync::SyncPoint) {
        let mut state = self.state.lock();
        if let Some(phase) = state.phases.get_mut(&id) {
            phase.last_use.insert(point);
        }
        transition_phase(&mut state, id, crate::device::BudgetCategory::InFlight);
    }
    pub(crate) fn mark_retiring(&self, id: u64) {
        transition_phase(
            &mut self.state.lock(),
            id,
            crate::device::BudgetCategory::Retiring,
        );
    }
    pub(crate) fn mark_cached(&self, id: u64, completed: &crate::sync::CompletedValues) {
        let mut state = self.state.lock();
        if state
            .phases
            .get(&id)
            .is_some_and(|phase| phase.last_use.is_reached(completed))
        {
            transition_phase(&mut state, id, crate::device::BudgetCategory::Cache);
        }
    }
    /// Reusing completed cached storage changes its category, not its real GPU
    /// history. A retiring allocation stays terminal in transition_phase.
    pub(crate) fn mark_live(&self, id: u64, completed: &crate::sync::CompletedValues) {
        let mut state = self.state.lock();
        if state
            .phases
            .get(&id)
            .is_some_and(|phase| phase.last_use.is_reached(completed))
        {
            transition_phase(&mut state, id, crate::device::BudgetCategory::Live);
        }
    }
    pub(crate) fn observe_completed(&self, completed: &crate::sync::CompletedValues) {
        let mut state = self.state.lock();
        // Transition one entry at a time without allocating on completion.
        let StatisticsState {
            budget,
            phases,
            records,
            ..
        } = &mut *state;
        for (id, phase) in phases {
            if phase.category == crate::device::BudgetCategory::InFlight
                && phase.last_use.is_reached(completed)
            {
                let bytes = records[id].size;
                budget.reclassify(phase.category, crate::device::BudgetCategory::Live, bytes);
                phase.category = crate::device::BudgetCategory::Live;
            }
        }
    }
    pub fn report(&self) -> AllocatorReport {
        self.state.lock().report()
    }
    pub fn peak_allocator_report(&self) -> Option<AllocatorReport> {
        let mut state = self.state.lock();
        state.materialize_peak();
        state.peak_report.clone()
    }
    /// Cumulative owned allocation ranges. A suballocator can place many ranges
    /// in one VkDeviceMemory; native block counts are reported separately.
    pub fn total_registered(&self) -> u64 {
        self.state.lock().next_id
    }
    pub fn peak_totals(&self) -> AllocationTotals {
        self.state.lock().peak
    }
    /// 显式候选预算策略。设置失败不改变旧预算或任何统计；不声称原游戏采用此限制。
    pub fn set_heap_budget(&self, heap: u32, bytes: u64) -> Result<()> {
        let mut state = self.state.lock();
        let used = state.heap_used.get(&heap).copied().unwrap_or(0);
        if used > bytes {
            return Err(Error::invalid_usage("预算小于当前分配"));
        }
        state.heap_budgets.insert(heap, bytes);
        Ok(())
    }
    pub fn register(
        &self,
        size: u64,
        memory_type: u32,
        heap: u32,
        properties: vk::MemoryPropertyFlags,
        name: String,
    ) -> Result<u64> {
        self.register_with_spare(size, memory_type, heap, properties, name, None)
    }
    #[cfg(any(feature = "suballocation", test))]
    pub(crate) fn register_suballocation(
        &self,
        size: u64,
        memory_type: u32,
        heap: u32,
        properties: vk::MemoryPropertyFlags,
        spare: u64,
    ) -> Result<u64> {
        self.register_with_spare(
            size,
            memory_type,
            heap,
            properties,
            String::new(),
            Some(spare),
        )
    }
    fn register_with_spare(
        &self,
        size: u64,
        memory_type: u32,
        heap: u32,
        properties: vk::MemoryPropertyFlags,
        name: String,
        spare: Option<u64>,
    ) -> Result<u64> {
        if size == 0 {
            return Err(Error::invalid_usage("不能登记零大小分配"));
        }
        let mut state = self.state.lock();
        let total = state
            .totals
            .total_allocated_bytes
            .checked_add(size)
            .ok_or_else(|| Error::invalid_usage("内存统计溢出"))?;
        let old_spare = spare.map_or(0, |_| state.suballocator_spare[memory_type as usize]);
        let new_spare = spare.unwrap_or(0);
        let used = state
            .heap_used
            .get(&heap)
            .copied()
            .unwrap_or(0)
            .checked_sub(old_spare)
            .and_then(|used| used.checked_add(new_spare))
            .and_then(|used| used.checked_add(size))
            .ok_or_else(|| Error::invalid_usage("内存统计溢出"))?;
        if state
            .heap_budgets
            .get(&heap)
            .is_some_and(|budget| used > *budget)
        {
            return Err(Error::OutOfDeviceMemory);
        }
        let id = state
            .next_id
            .checked_add(1)
            .ok_or_else(|| Error::invalid_usage("内存ID耗尽"))?;
        let mut totals = state.totals;
        totals.total_allocated_bytes = total;
        if properties.contains(vk::MemoryPropertyFlags::DEVICE_LOCAL) {
            totals.device_local_bytes = totals
                .device_local_bytes
                .checked_add(size)
                .ok_or_else(|| Error::invalid_usage("内存统计溢出"))?;
        } else {
            totals.system_bytes = totals
                .system_bytes
                .checked_add(size)
                .ok_or_else(|| Error::invalid_usage("内存统计溢出"))?;
        }
        let mut budget = state.budget;
        budget.release(crate::device::BudgetCategory::Cache, old_spare);
        budget
            .reserve(crate::device::BudgetCategory::Cache, new_spare)
            .map_err(|_| Error::OutOfDeviceMemory)?;
        budget
            .reserve(crate::device::BudgetCategory::Live, size)
            .map_err(|_| Error::OutOfDeviceMemory)?;
        state.budget = budget;
        if spare.is_some() {
            state.suballocator_spare[memory_type as usize] = new_spare;
        }
        state.next_id = id;
        state.phases.insert(
            id,
            AllocationPhase {
                category: crate::device::BudgetCategory::Live,
                last_use: crate::sync::SyncSet::EMPTY,
            },
        );
        state.records.insert(
            id,
            AllocationRecord {
                id,
                size,
                memory_type,
                heap,
                properties: properties.as_raw(),
                name,
            },
        );
        state.totals = totals;
        state.heap_used.insert(heap, used);
        state.peak.device_local_bytes =
            state.peak.device_local_bytes.max(totals.device_local_bytes);
        state.peak.system_bytes = state.peak.system_bytes.max(totals.system_bytes);
        if total > state.peak.total_allocated_bytes {
            state.peak.total_allocated_bytes = total;
            state.peak_dirty = true;
        }
        Ok(id)
    }
    pub fn release(&self, id: u64) -> Result<()> {
        self.release_with_spare(id, None)
    }
    #[cfg(any(feature = "suballocation", test))]
    pub(crate) fn release_suballocation(&self, id: u64, spare: u64) -> Result<()> {
        self.release_with_spare(id, Some(spare))
    }
    fn release_with_spare(&self, id: u64, spare: Option<u64>) -> Result<()> {
        let mut state = self.state.lock();
        if !state.records.contains_key(&id) {
            return Err(Error::invalid_usage("未知或已释放的内存ID"));
        }
        // 释放会改变状态：先把刚到达的峰值物化。
        state.materialize_peak();
        let record = state
            .records
            .remove(&id)
            .ok_or_else(|| Error::invalid_usage("未知或已释放的内存ID"))?;
        if let Some(phase) = state.phases.remove(&id) {
            state.budget.release(phase.category, record.size);
        }
        if let Some(spare) = spare {
            replace_spare(&mut state, record.memory_type, record.heap, spare);
        }
        if let Some(used) = state.heap_used.get_mut(&record.heap) {
            *used -= record.size;
        }
        state.totals.total_allocated_bytes -= record.size;
        if record.properties & vk::MemoryPropertyFlags::DEVICE_LOCAL.as_raw() != 0 {
            state.totals.device_local_bytes -= record.size;
        } else {
            state.totals.system_bytes -= record.size;
        }
        Ok(())
    }
    /// Only unused native blocks may be trimmed. No live range changes here.
    #[cfg(any(feature = "suballocation", test))]
    pub(crate) fn trim_suballocator_spare(&self, memory_type: u32, heap: u32) {
        replace_spare(&mut self.state.lock(), memory_type, heap, 0);
    }
    pub(crate) fn rename(&self, id: u64, name: String) {
        let mut state = self.state.lock();
        if let Some(record) = state.records.get_mut(&id) {
            record.name = name.clone();
        }
        // 名称在分配成功后立即生产；同步补齐刚捕获的峰值快照中对应记录。
        // 快照未物化时，之后物化会直接带上新名字；已物化时按 id（有序）二分定位。
        if let Some(report) = &mut state.peak_report
            && let Ok(index) = report
                .allocations
                .binary_search_by_key(&id, |record| record.id)
        {
            report.allocations[index].name = name;
        }
    }
}

fn replace_spare(state: &mut StatisticsState, memory_type: u32, heap: u32, spare: u64) {
    use crate::device::BudgetCategory;
    let previous = state.suballocator_spare[memory_type as usize];
    state.budget.release(BudgetCategory::Cache, previous);
    state
        .budget
        .reserve(BudgetCategory::Cache, spare)
        .expect("freeing or trimming native memory cannot increase its admitted total");
    state.suballocator_spare[memory_type as usize] = spare;
    let used = state.heap_used.entry(heap).or_default();
    *used = used
        .checked_sub(previous)
        .and_then(|value| value.checked_add(spare))
        .expect("native spare accounting is balanced");
}
fn transition_phase(state: &mut StatisticsState, id: u64, category: crate::device::BudgetCategory) {
    let Some(phase) = state.phases.get_mut(&id) else {
        return;
    };
    if phase.category == category {
        return;
    }
    // Retiring is terminal: accounting can never make an unowned allocation
    // eligible for a cache just because another completion snapshot arrived.
    if phase.category == crate::device::BudgetCategory::Retiring {
        return;
    }
    let bytes = state.records[&id].size;
    state.budget.reclassify(phase.category, category, bytes);
    phase.category = category;
}

pub struct AllocationRequest {
    pub name: String,
    pub requirements: vk::MemoryRequirements,
    pub location: MemoryLocation,
    pub flags: vk::MemoryAllocateFlags,
}
/// 原 gpu-allocator 分叉的外供边界。未来直接接原库；不复制其池或空闲块算法。
pub trait AllocationBackend {
    type Allocation;
    fn allocate(&mut self, request: &AllocationRequest) -> Result<Self::Allocation>;
    fn free(&mut self, allocation: Self::Allocation) -> Result<()>;
    fn report(&self) -> AllocatorReport;
}
/// 游戏私有峰值包装。原 +0x688 flag 为 1 时每次 allocate 返回后采样；free 不采样。
pub struct GpuAllocator<B: AllocationBackend> {
    backend: B,
    track_peaks: bool,
    peaks: AllocationTotals,
    peak_report: Option<AllocatorReport>,
}
impl<B: AllocationBackend> GpuAllocator<B> {
    pub fn new(backend: B, track_peaks: bool) -> Self {
        Self {
            backend,
            track_peaks,
            peaks: AllocationTotals::default(),
            peak_report: None,
        }
    }
    pub fn inner(&self) -> &B {
        &self.backend
    }
    pub fn inner_mut(&mut self) -> &mut B {
        &mut self.backend
    }
    pub fn allocate(&mut self, request: &AllocationRequest) -> Result<B::Allocation> {
        let result = self.backend.allocate(request);
        if self.track_peaks {
            self.observe_report(self.backend.report());
        }
        result
    }
    /// 保留完整报表快照，只在 total 严格增加时替换；device/system 独立取最大。
    pub fn observe_report(&mut self, report: AllocatorReport) {
        self.peaks.device_local_bytes = self
            .peaks
            .device_local_bytes
            .max(report.totals.device_local_bytes);
        self.peaks.system_bytes = self.peaks.system_bytes.max(report.totals.system_bytes);
        if report.totals.total_allocated_bytes > self.peaks.total_allocated_bytes {
            self.peaks.total_allocated_bytes = report.totals.total_allocated_bytes;
            self.peak_report = Some(report);
        }
    }
    pub fn free(&mut self, allocation: B::Allocation) -> Result<()> {
        self.backend.free(allocation)
    }
    pub fn peak_total_allocated_bytes(&self) -> u64 {
        self.peaks.total_allocated_bytes
    }
    pub fn peak_device_local_bytes(&self) -> u64 {
        self.peaks.device_local_bytes
    }
    pub fn peak_system_bytes(&self) -> u64 {
        self.peaks.system_bytes
    }
    pub fn peak_allocator_report(&self) -> Option<&AllocatorReport> {
        self.peak_report.as_ref()
    }
}
pub struct DirectMemoryBackend {
    pub device: Arc<Device>,
}
impl AllocationBackend for DirectMemoryBackend {
    type Allocation = MemoryBlock;
    fn allocate(&mut self, request: &AllocationRequest) -> Result<MemoryBlock> {
        let (required, preferred) = request.location.properties();
        let block = MemoryBlock::allocate_dedicated(
            self.device.clone(),
            request.requirements,
            required,
            preferred,
            request.flags,
        )?;
        self.device
            .allocator_stats
            .rename(block.accounting_id, request.name.clone());
        Ok(block)
    }
    fn free(&mut self, allocation: MemoryBlock) -> Result<()> {
        if !Arc::ptr_eq(&self.device, &allocation.device) {
            return Err(Error::invalid_usage("不能向其他设备释放内存"));
        }
        drop(allocation);
        Ok(())
    }
    fn report(&self) -> AllocatorReport {
        self.device.allocator_stats.report()
    }
}

#[cfg(test)]
mod budget_tests {
    use super::*;
    #[test]
    fn pending_descriptor_pool_owners_keep_the_global_quota_charged() {
        let stats = std::sync::Arc::new(AllocatorStats::default());
        let first = stats.reserve_update_after_bind_descriptors(6, 10).unwrap();
        assert!(stats.reserve_update_after_bind_descriptors(5, 10).is_err());
        let second = stats.reserve_update_after_bind_descriptors(4, 10).unwrap();
        drop(first);
        let third = stats.reserve_update_after_bind_descriptors(6, 10).unwrap();
        assert!(stats.reserve_update_after_bind_descriptors(1, 10).is_err());
        drop((second, third));
        assert!(stats.reserve_update_after_bind_descriptors(10, 10).is_ok());
    }
    use crate::{
        device::BudgetCategory,
        sync::{CompletedValues, QueueId, SyncPoint},
    };
    #[test]
    fn allocation_budget_tracks_real_lifetime_and_rejects_overcommit_atomically() {
        let stats = AllocatorStats::default();
        stats.set_total_budget(100).unwrap();
        let id = stats
            .register(
                60,
                0,
                0,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
                "resource".into(),
            )
            .unwrap();
        assert_eq!(stats.budget_usage().live, 60);
        let point = SyncPoint {
            queue: QueueId::new(0).unwrap(),
            value: 3,
        };
        stats.record_gpu_use(id, point);
        assert_eq!(stats.budget_usage().in_flight, 60);
        assert!(matches!(
            stats.register(
                50,
                0,
                0,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
                "over budget".into()
            ),
            Err(Error::OutOfDeviceMemory)
        ));
        assert_eq!(stats.total_registered(), 1);
        assert_eq!(stats.report().allocations.len(), 1);
        let mut completed = CompletedValues::default();
        completed.raise(point.queue, 2);
        stats.observe_completed(&completed);
        stats.mark_cached(id, &completed);
        stats.mark_live(id, &completed);
        assert_eq!(stats.budget_usage().in_flight, 60);
        completed.raise(point.queue, 3);
        stats.observe_completed(&completed);
        assert_eq!(stats.budget_usage().live, 60);
        stats.mark_cached(id, &completed);
        assert_eq!(stats.budget_usage().cache, 60);
        stats.mark_live(id, &completed);
        assert_eq!(stats.budget_usage().live, 60);
        assert_eq!(
            stats.state.lock().phases[&id].last_use.points().next(),
            Some(point)
        );
        stats.mark_cached(id, &completed);
        stats.mark_retiring(id);
        stats.observe_completed(&completed);
        stats.mark_cached(id, &completed);
        stats.mark_live(id, &completed);
        assert_eq!(
            stats.state.lock().phases[&id].category,
            BudgetCategory::Retiring
        );
        assert_eq!(stats.budget_usage().retiring, 60);
        stats.release(id).unwrap();
        assert_eq!(stats.budget_usage().total(), 0);
    }
}

#[cfg(test)]
mod native_spare_tests {
    use super::*;
    use crate::device::BudgetUsage;
    #[test]
    fn native_spare_moves_to_ranges_and_back_without_double_charging() {
        let stats = AllocatorStats::default();
        stats.set_total_budget(100).unwrap();
        let a = stats
            .register_suballocation(20, 0, 0, vk::MemoryPropertyFlags::DEVICE_LOCAL, 80)
            .unwrap();
        assert_eq!(
            stats.budget_usage(),
            BudgetUsage {
                live: 20,
                cache: 80,
                ..Default::default()
            }
        );
        let b = stats
            .register_suballocation(30, 0, 0, vk::MemoryPropertyFlags::DEVICE_LOCAL, 50)
            .unwrap();
        assert_eq!(
            stats.budget_usage(),
            BudgetUsage {
                live: 50,
                cache: 50,
                ..Default::default()
            }
        );
        stats.mark_retiring(a);
        stats.release_suballocation(a, 70).unwrap();
        assert_eq!(
            stats.budget_usage(),
            BudgetUsage {
                live: 30,
                cache: 70,
                ..Default::default()
            }
        );
        stats.release_suballocation(b, 100).unwrap();
        assert_eq!(stats.report().totals.total_allocated_bytes, 0);
        assert_eq!(stats.budget_usage().cache, 100);
        assert!(stats.set_heap_budget(0, 99).is_err());
        stats.trim_suballocator_spare(0, 0);
        assert_eq!(stats.budget_usage().total(), 0);
        stats.set_heap_budget(0, 0).unwrap();
    }
    #[test]
    fn failed_native_growth_preserves_the_entire_previous_ledger() {
        let stats = AllocatorStats::default();
        stats.set_total_budget(100).unwrap();
        let a = stats
            .register_suballocation(20, 0, 0, vk::MemoryPropertyFlags::DEVICE_LOCAL, 80)
            .unwrap();
        let before = stats.report();
        let usage = stats.budget_usage();
        assert!(matches!(
            stats.register_suballocation(40, 0, 0, vk::MemoryPropertyFlags::DEVICE_LOCAL, 70),
            Err(Error::OutOfDeviceMemory)
        ));
        assert_eq!(stats.report(), before);
        assert_eq!(stats.budget_usage(), usage);
        assert_eq!(stats.suballocator_spare_bytes(), 80);
        assert_eq!(stats.total_registered(), 1);
        stats.release_suballocation(a, 0).unwrap();
        assert_eq!(stats.budget_usage().total(), 0);
    }
    #[test]
    fn host_reservations_and_native_capacity_share_the_budget_without_sharing_gpu_totals() {
        let stats = Arc::new(AllocatorStats::default());
        stats.set_total_budget(100).unwrap();
        let host = stats
            .reserve_cpu_bytes(20, crate::device::BudgetCategory::Cache)
            .unwrap();
        let a = stats
            .register_suballocation(20, 0, 0, vk::MemoryPropertyFlags::DEVICE_LOCAL, 60)
            .unwrap();
        assert_eq!(stats.budget_usage().total(), 100);
        assert_eq!(stats.report().totals.total_allocated_bytes, 20);
        stats.release_suballocation(a, 80).unwrap();
        assert_eq!(stats.budget_usage().cache, 100);
        drop(host);
        assert_eq!(stats.budget_usage().cache, 80);
        stats.trim_suballocator_spare(0, 0);
        assert_eq!(stats.budget_usage().total(), 0);
    }
}

#[cfg(test)]
mod host_subrange_tests {
    use super::*;
    #[test]
    fn noncoherent_ranges_use_the_physical_base_and_isolate_neighboring_atoms() {
        let range = mapped_host_subrange(1024, 512, 500, 65, 1, 64, false).unwrap();
        assert_eq!((range.memory_offset, range.memory_size), (1088, 64));
        assert_eq!((range.buffer_offset, range.buffer_size), (64, 64));
        let range = mapped_host_subrange(1024, 512, 500, 65, 1, 1, false).unwrap();
        assert_eq!((range.memory_offset, range.memory_size), (1089, 1));
        assert_eq!((range.buffer_offset, range.buffer_size), (65, 1));
        assert!(matches!(
            mapped_host_subrange(1056, 512, 500, 0, 1, 64, false),
            Err(Error::Unsupported(_))
        ));
    }
    #[test]
    fn only_dedicated_memory_can_use_a_non_aligned_end_of_object() {
        let range = mapped_host_subrange(0, 1000, 1000, 999, 1, 256, true).unwrap();
        assert_eq!(
            (range.memory_offset, range.memory_size),
            (768, vk::WHOLE_SIZE)
        );
        assert_eq!((range.buffer_offset, range.buffer_size), (768, 232));
        assert!(mapped_host_subrange(0, 1000, 1000, 999, 1, 256, false).is_err());
        assert!(mapped_host_subrange(0, 1000, 1000, 999, 2, 256, true).is_err());
        assert!(mapped_host_subrange(u64::MAX - 1, 4, 4, 0, 1, 1, true).is_err());
    }
}
