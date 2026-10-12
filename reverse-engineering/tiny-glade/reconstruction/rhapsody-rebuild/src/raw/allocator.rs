//! 最小 Vulkan 专用分配适配。原 gpu-allocator 0.26 本地分叉来源待恢复，
//! 此处只调用分配原语，不重写该依赖的次分配/池策略。
use super::{
    device::VulkanDevice,
    error::{Error, Result},
};
use ash::vk;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};
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
    pub device: Arc<VulkanDevice>,
    pub handle: vk::DeviceMemory,
    pub size: u64,
    pub properties: vk::MemoryPropertyFlags,
    accounting_id: u64,
}
impl MemoryBlock {
    pub fn allocate(
        device: Arc<VulkanDevice>,
        requirements: vk::MemoryRequirements,
        required: vk::MemoryPropertyFlags,
        preferred: vk::MemoryPropertyFlags,
    ) -> Result<Self> {
        Self::allocate_with_flags(
            device,
            requirements,
            required,
            preferred,
            vk::MemoryAllocateFlags::empty(),
        )
    }

    pub fn allocate_with_flags(
        device: Arc<VulkanDevice>,
        requirements: vk::MemoryRequirements,
        required: vk::MemoryPropertyFlags,
        preferred: vk::MemoryPropertyFlags,
        flags: vk::MemoryAllocateFlags,
    ) -> Result<Self> {
        let index = choose_memory_type(
            &device.memory,
            requirements.memory_type_bits,
            required,
            preferred,
        )
        .ok_or(Error::Unsupported("没有匹配的内存类型"))?;
        let properties = device.memory.memory_types[index as usize].property_flags;
        let mut allocation_flags = vk::MemoryAllocateFlagsInfo::default().flags(flags);
        let mut info = vk::MemoryAllocateInfo::default()
            .allocation_size(requirements.size)
            .memory_type_index(index);
        if !flags.is_empty() {
            info = info.push_next(&mut allocation_flags);
        }
        if requirements.size == 0
            || (!flags.is_empty()
                && flags.contains(vk::MemoryAllocateFlags::DEVICE_ADDRESS)
                && !device.capabilities.buffer_device_address)
        {
            return Err(Error::Invalid("内存大小或设备地址分配能力无效"));
        }
        let heap = device.memory.memory_types[index as usize].heap_index;
        let handle = unsafe { device.device.allocate_memory(&info, None) }?;
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
            size: requirements.size,
            properties,
            accounting_id,
        })
    }
}
impl Drop for MemoryBlock {
    fn drop(&mut self) {
        unsafe {
            self.device.device.free_memory(self.handle, None);
        }
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
    state: Mutex<StatisticsState>,
}
#[derive(Default)]
struct StatisticsState {
    next_id: u64,
    records: BTreeMap<u64, AllocationRecord>,
    totals: AllocationTotals,
    peak: AllocationTotals,
    peak_report: Option<AllocatorReport>,
    heap_budgets: BTreeMap<u32, u64>,
}
impl StatisticsState {
    fn report(&self) -> AllocatorReport {
        AllocatorReport {
            totals: self.totals,
            allocations: self.records.values().cloned().collect(),
        }
    }
}
impl AllocatorStats {
    pub fn report(&self) -> AllocatorReport {
        self.state
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .report()
    }
    pub fn peak_allocator_report(&self) -> Option<AllocatorReport> {
        self.state
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .peak_report
            .clone()
    }
    pub fn peak_totals(&self) -> AllocationTotals {
        self.state.lock().unwrap_or_else(|p| p.into_inner()).peak
    }
    /// 显式候选预算策略。设置失败不改变旧预算或任何统计；不声称原游戏采用此限制。
    pub fn set_heap_budget(&self, heap: u32, bytes: u64) -> Result<()> {
        let mut state = self.state.lock().unwrap_or_else(|p| p.into_inner());
        let used = state
            .records
            .values()
            .filter(|record| record.heap == heap)
            .try_fold(0u64, |total, record| total.checked_add(record.size))
            .ok_or(Error::Invalid("内存统计溢出"))?;
        if used > bytes {
            return Err(Error::Invalid("预算小于当前分配"));
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
        if size == 0 {
            return Err(Error::Invalid("不能登记零大小分配"));
        }
        let mut state = self.state.lock().unwrap_or_else(|p| p.into_inner());
        let total = state
            .totals
            .total_allocated_bytes
            .checked_add(size)
            .ok_or(Error::Invalid("内存统计溢出"))?;
        let used = state
            .records
            .values()
            .filter(|record| record.heap == heap)
            .try_fold(size, |total, record| total.checked_add(record.size))
            .ok_or(Error::Invalid("内存统计溢出"))?;
        if state
            .heap_budgets
            .get(&heap)
            .is_some_and(|budget| used > *budget)
        {
            return Err(Error::Vulkan(vk::Result::ERROR_OUT_OF_DEVICE_MEMORY));
        }
        let id = state
            .next_id
            .checked_add(1)
            .ok_or(Error::Invalid("内存ID耗尽"))?;
        let mut totals = state.totals;
        totals.total_allocated_bytes = total;
        if properties.contains(vk::MemoryPropertyFlags::DEVICE_LOCAL) {
            totals.device_local_bytes = totals
                .device_local_bytes
                .checked_add(size)
                .ok_or(Error::Invalid("内存统计溢出"))?;
        } else {
            totals.system_bytes = totals
                .system_bytes
                .checked_add(size)
                .ok_or(Error::Invalid("内存统计溢出"))?;
        }
        state.next_id = id;
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
        state.peak.device_local_bytes =
            state.peak.device_local_bytes.max(totals.device_local_bytes);
        state.peak.system_bytes = state.peak.system_bytes.max(totals.system_bytes);
        if total > state.peak.total_allocated_bytes {
            state.peak.total_allocated_bytes = total;
            state.peak_report = Some(state.report());
        }
        Ok(id)
    }
    pub fn release(&self, id: u64) -> Result<()> {
        let mut state = self.state.lock().unwrap_or_else(|p| p.into_inner());
        let record = state
            .records
            .remove(&id)
            .ok_or(Error::Invalid("未知或已释放的内存ID"))?;
        state.totals.total_allocated_bytes -= record.size;
        if record.properties & vk::MemoryPropertyFlags::DEVICE_LOCAL.as_raw() != 0 {
            state.totals.device_local_bytes -= record.size;
        } else {
            state.totals.system_bytes -= record.size;
        }
        Ok(())
    }
    fn rename(&self, id: u64, name: String) {
        let mut state = self.state.lock().unwrap_or_else(|p| p.into_inner());
        if let Some(record) = state.records.get_mut(&id) {
            record.name = name.clone();
        }
        // 名称在分配成功后立即生产；同步补齐刚捕获的峰值快照中对应记录。
        if let Some(report) = &mut state.peak_report {
            if let Some(record) = report.allocations.iter_mut().find(|record| record.id == id) {
                record.name = name;
            }
        }
    }
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
    pub device: Arc<VulkanDevice>,
}
impl AllocationBackend for DirectMemoryBackend {
    type Allocation = MemoryBlock;
    fn allocate(&mut self, request: &AllocationRequest) -> Result<MemoryBlock> {
        let (required, preferred) = request.location.properties();
        let block = MemoryBlock::allocate_with_flags(
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
            return Err(Error::Invalid("不能向其他设备释放内存"));
        }
        drop(allocation);
        Ok(())
    }
    fn report(&self) -> AllocatorReport {
        self.device.allocator_stats.report()
    }
}
