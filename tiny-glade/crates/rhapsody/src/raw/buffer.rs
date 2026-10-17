//! Buffer 创建、内存绑定、映射读写候选；`BufferDesc` 对齐原 16 字节描述符布局。
use super::{
    allocator::MemoryBlock,
    device::Device,
    error::{Error, Result},
    graveyard::Doomed,
};
use ash::vk;
use std::sync::Arc;
/// 原作 `BufferDesc.memory_location` 枚举（取值 1..=4，0 非法）。
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryLocation {
    GpuOnly = 1,
    CpuToGpu = 2,
    /// HOST_VISIBLE 但非低延迟映射，不要求 HOST_COHERENT。
    CpuToGpuLowPrio = 3,
    GpuToCpu = 4,
}
impl MemoryLocation {
    pub const fn as_raw(self) -> u8 {
        self as u8
    }
    pub const fn from_raw(raw: u8) -> Option<Self> {
        match raw {
            1 => Some(Self::GpuOnly),
            2 => Some(Self::CpuToGpu),
            3 => Some(Self::CpuToGpuLowPrio),
            4 => Some(Self::GpuToCpu),
            _ => None,
        }
    }
    /// 返回 (必需, 偏好) 内存属性；映射集中在此处，创建路径不再自行拼装。
    pub fn memory_properties(self) -> (vk::MemoryPropertyFlags, vk::MemoryPropertyFlags) {
        use vk::MemoryPropertyFlags as F;
        match self {
            Self::GpuOnly => (F::DEVICE_LOCAL, F::DEVICE_LOCAL),
            Self::CpuToGpu => (F::HOST_VISIBLE, F::HOST_COHERENT),
            Self::CpuToGpuLowPrio => (F::HOST_VISIBLE, F::empty()),
            Self::GpuToCpu => (F::HOST_VISIBLE, F::HOST_CACHED | F::HOST_COHERENT),
        }
    }
}
/// 原 16 字节 ABI：`{u64 size, u32 usage, u16 alignment, u8 memory_location}`。
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BufferDesc {
    pub size: u64,
    pub usage: vk::BufferUsageFlags,
    /// 请求的对齐（字节）；0 表示无显式要求。
    pub alignment: u16,
    pub memory_location: MemoryLocation,
}
const _: () = assert!(std::mem::size_of::<BufferDesc>() == 16);
impl BufferDesc {
    pub fn new(size: u64, usage: vk::BufferUsageFlags, memory_location: MemoryLocation) -> Self {
        Self {
            size,
            usage,
            alignment: 0,
            memory_location,
        }
    }
    pub fn new_gpu_only(size: u64, usage: vk::BufferUsageFlags) -> Self {
        Self::new(size, usage, MemoryLocation::GpuOnly)
    }
    pub fn new_cpu_to_gpu(size: u64, usage: vk::BufferUsageFlags) -> Self {
        Self::new(size, usage, MemoryLocation::CpuToGpu)
    }
    /// 原 memory_location=3 语义：HOST_VISIBLE 但非低延迟映射（不要求 HOST_COHERENT）。
    pub fn new_cpu_to_gpu_low_prio(size: u64, usage: vk::BufferUsageFlags) -> Self {
        Self::new(size, usage, MemoryLocation::CpuToGpuLowPrio)
    }
    pub fn new_gpu_to_cpu(size: u64, usage: vk::BufferUsageFlags) -> Self {
        Self::new(size, usage, MemoryLocation::GpuToCpu)
    }
    /// 显式对齐请求；由调用方在满足 Vulkan memory-requirement 的前提下设置。
    pub fn with_alignment(mut self, alignment: u16) -> Self {
        self.alignment = alignment;
        self
    }
    pub fn alignment(&self) -> u16 {
        self.alignment
    }
}
/// `create_buffer_impl`（0x140bee200）里各 Vulkan 调用失败时的 `.expect(msg)` 文案
/// （原版 panic；crates/rhapsody/src/raw/buffer.rs 0xc0 / 0xc9 / 0xf9 行）。拼法见
/// [`super::texture::create_failure_text`]。
pub mod create_failure {
    /// 0x142b157f0（13B）。
    pub const CREATE_BUFFER: &str = "create_buffer";
    /// 0x142b15888（18B）。
    pub const BIND_BUFFER_MEMORY: &str = "bind_buffer_memory";
    /// 0x142b15850（27B）。
    pub const SET_DEBUG_UTILS_OBJECT_NAME: &str = "set_debug_utils_object_name";
}

/// `create_buffer_impl`（0x140bee200）里对 `BufferDesc.usage` 做的唯一原始位测试：`usage & 0x400`。
/// 该位在公开 Vulkan 枚举里没有名字（0x140bee2b0 附近 `test r?, 0x400`），保持原始数值，不赋语义。
pub const USAGE_ALIGN_64_BIT_RAW: u32 = 0x400;

/// 原版对缓冲内存对齐的计算（0x140bee200）：`max(memreq.alignment, desc.alignment,
/// (usage & 0x400) ? 0x40 : 0x10)`。
pub fn buffer_alignment(requirement_alignment: u64, desc_alignment: u16, usage: u32) -> u64 {
    let floor = if usage & USAGE_ALIGN_64_BIT_RAW != 0 {
        0x40
    } else {
        0x10
    };
    requirement_alignment
        .max(u64::from(desc_alignment))
        .max(floor)
}

/// `update_buffer_bytes`（0x140beefa0）的断言失败，文本取自原 .rdata。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BufferUpdateError {
    /// 区间 `start > end`。
    RangeInverted { start: usize, end: usize },
    /// 区间 `end > src_data.len()`。
    RangeOutOfData { end: usize, len: usize },
    /// `assert!(total_bytes + src_data.len() + dst_offset as usize <= dst_buffer.desc.size)`（0x142b15b00）。
    ExceedsCapacity {
        total: u64,
        dst_offset: u64,
        capacity: u64,
    },
    /// `"Buffer {} must be created with vk::BufferUsageFlags::TRANSFER_DST in order to support updates"`。
    MissingTransferDst,
    /// `assert!(size > 0)`：合计字节为 0（0x142b0f830）。
    Empty,
}
impl std::fmt::Display for BufferUpdateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RangeInverted { start, end } => write!(f, "更新区间 {start}..{end} 起点大于终点"),
            Self::RangeOutOfData { end, len } => write!(f, "更新区间终点 {end} 超过源数据长度 {len}"),
            Self::ExceedsCapacity { total, dst_offset, capacity } => write!(
                f,
                "assertion failed: total_bytes + src_data.len() + dst_offset as usize <= dst_buffer.desc.size（{total}+{dst_offset}>{capacity}）"
            ),
            Self::MissingTransferDst => f.write_str(
                "Buffer must be created with vk::BufferUsageFlags::TRANSFER_DST in order to support updates",
            ),
            Self::Empty => f.write_str("assertion failed: size > 0"),
        }
    }
}
impl std::error::Error for BufferUpdateError {}
/// 一条 `vkCmdCopyBuffer` 区域（`VkBufferCopy` 的 `{src,dst,size}`，不带 `Eq` 依赖）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BufferRegion {
    pub src_offset: u64,
    pub dst_offset: u64,
    pub size: u64,
}

/// `update_buffer_bytes` 的执行计划：暂存缓冲大小与 `vkCmdBufferCopy` 区域。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BufferUpdatePlan {
    /// 暂存缓冲大小 = 各区间长度之和。
    pub total: u64,
    /// 区域 i：`{src = 前面各区间累计, dst = range.start + dst_offset, size = end - start}`
    /// （0x140c03a00 闭包 `+0x00/+0x08/+0x10` 的写入）。
    pub regions: Vec<BufferRegion>,
}

/// Validate sparse updates against the actual buffer description before staging.
/// Source ranges preserve their destination positions; staging bytes are packed.
pub fn plan_buffer_update(
    desc: &BufferDesc,
    data_len: usize,
    ranges: &[std::ops::Range<usize>],
    dst_offset: u64,
) -> Result<BufferUpdatePlan> {
    let mut total = 0u64;
    let mut regions = Vec::new();
    regions
        .try_reserve(ranges.len())
        .map_err(|_| Error::OutOfHostMemory)?;
    for r in ranges {
        if r.start > r.end {
            return Err(BufferUpdateError::RangeInverted {
                start: r.start,
                end: r.end,
            }
            .into());
        }
        if r.end > data_len {
            return Err(BufferUpdateError::RangeOutOfData {
                end: r.end,
                len: data_len,
            }
            .into());
        }
        let len = (r.end - r.start) as u64;
        let exceeds = || BufferUpdateError::ExceedsCapacity {
            total: total.saturating_add(len),
            dst_offset,
            capacity: desc.size,
        };
        let next = total
            .checked_add(len)
            .ok_or_else(|| Error::from(exceeds()))?;
        if next
            .checked_add(dst_offset)
            .is_none_or(|end| end > desc.size)
        {
            return Err(exceeds().into());
        }
        let dst_end = (r.end as u64)
            .checked_add(dst_offset)
            .filter(|&end| end <= desc.size)
            .ok_or_else(|| Error::from(exceeds()))?;
        if len != 0 {
            regions.push(BufferRegion {
                src_offset: total,
                dst_offset: dst_end - len,
                size: len,
            });
        }
        total = next;
    }
    if !desc.usage.contains(vk::BufferUsageFlags::TRANSFER_DST) {
        return Err(BufferUpdateError::MissingTransferDst.into());
    }
    if total == 0 {
        return Err(BufferUpdateError::Empty.into());
    }
    Ok(BufferUpdatePlan { total, regions })
}

/// `create_mapped_buffer`（0x140beea80）的返回值：40B Buffer + 映射 `(ptr, len)`（0x40B = tag 8 +
/// 40 + 16；旧文档的“额外 24B”不对，机器码 `param_1[6]=impl+0x38`、`param_1[7]=impl+0x40` 只有两个 qword）。
pub struct MappedBufferParts {
    pub ptr: *mut u8,
    pub len: usize,
}

pub struct Buffer {
    dynamic_constants_page: bool,
    pub(crate) sharing: crate::resource::ResourceSharing,
    allocation_use: Option<Arc<crate::resource::SyncCell>>,
    pub(crate) last_use: Arc<crate::resource::SyncCell>,
    pub device: Arc<Device>,
    pub handle: vk::Buffer,
    pub size: u64,
    pub usage: vk::BufferUsageFlags,
    /// 创建时的原 16 字节描述（图导入需要完整的 alignment/memory_location）。
    pub desc: BufferDesc,
    allocation: std::mem::ManuallyDrop<MemoryBlock>,
    mapped_ptr: Option<usize>,
    host_access: parking_lot::Mutex<()>,
    _memory: super::allocator::CpuMemoryReservation,
}
enum HostTransfer<'a> {
    Read(&'a mut [u8]),
    Write(&'a [u8]),
}
struct HostRangeState<'a> {
    cell: &'a crate::resource::SyncCell,
    accessed: bool,
}
impl Drop for HostRangeState<'_> {
    fn drop(&mut self) {
        if self.accessed {
            self.cell.publish_buffer_host_range();
        } else {
            self.cell.cancel_range_state();
        }
    }
}
impl HostTransfer<'_> {
    fn len(&self) -> usize {
        match self {
            Self::Read(bytes) => bytes.len(),
            Self::Write(bytes) => bytes.len(),
        }
    }
}
impl Buffer {
    pub(crate) fn into_dynamic_constants_page(mut self) -> Self {
        self.dynamic_constants_page = true;
        self
    }
    pub(crate) fn is_dynamic_constants_page(&self) -> bool {
        self.dynamic_constants_page
    }
    pub(crate) fn allocation_bytes(&self) -> u64 {
        self.allocation.size
    }
    pub(crate) fn allocation_id(&self) -> u64 {
        self.allocation.accounting_id()
    }
    /// The caller owns an already-bound handle and transfers its sole raw-object ownership.
    pub(crate) unsafe fn from_bound_transient(
        device: Arc<Device>,
        desc: BufferDesc,
        handle: vk::Buffer,
        allocation: MemoryBlock,
        sharing: crate::resource::ResourceSharing,
        storage: crate::resource::tracked::ResourceStorage,
    ) -> Self {
        let allocation_use = allocation.shared_use();
        let last_use = (if sharing.is_concurrent() {
            crate::resource::SyncCell::for_device(device.generation())
                .with_concurrent_families(match sharing {
                    crate::resource::ResourceSharing::Concurrent(families) => families,
                    _ => unreachable!("concurrent branch"),
                })
                .with_ownership_range(crate::resource::OwnershipRange::Buffer {
                    offset: 0,
                    size: desc.size,
                })
        } else {
            crate::resource::SyncCell::for_device(device.generation()).with_ownership_range(
                crate::resource::OwnershipRange::Buffer {
                    offset: 0,
                    size: desc.size,
                },
            )
        })
        .into_reserved_arc(storage.synchronization);
        Self {
            dynamic_constants_page: false,
            _memory: storage.object,
            sharing,
            allocation_use,
            last_use,
            device,
            handle,
            size: desc.size,
            usage: desc.usage,
            desc,
            allocation: std::mem::ManuallyDrop::new(allocation),
            mapped_ptr: None,
            host_access: parking_lot::Mutex::new(()),
        }
    }
    pub fn new(device: Arc<Device>, desc: &BufferDesc) -> Result<Self> {
        Self::new_with_memory(
            device,
            desc,
            false,
            crate::resource::ResourceSharing::Exclusive,
        )
    }
    pub(crate) fn new_dedicated(device: Arc<Device>, desc: &BufferDesc) -> Result<Self> {
        Self::new_with_memory(
            device,
            desc,
            true,
            crate::resource::ResourceSharing::Exclusive,
        )
    }
    pub fn new_concurrent(
        device: Arc<Device>,
        desc: &BufferDesc,
        families: &[u32],
    ) -> Result<Self> {
        Self::new_with_memory(
            device,
            desc,
            false,
            crate::resource::ResourceSharing::Concurrent(crate::resource::QueueFamilies::new(
                families,
            )?),
        )
    }
    pub fn sharing(&self) -> crate::resource::ResourceSharing {
        self.sharing
    }
    fn new_with_memory(
        device: Arc<Device>,
        desc: &BufferDesc,
        dedicated: bool,
        sharing: crate::resource::ResourceSharing,
    ) -> Result<Self> {
        if device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        sharing.validate_device(&device)?;
        if desc.size == 0 {
            return Err(Error::invalid_usage("缓冲区大小不能为零"));
        }
        if !device.capabilities.buffer_device_address
            && desc
                .usage
                .contains(vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS)
        {
            return Err(Error::unsupported("此路径未启用 bufferDeviceAddress"));
        }
        let sync_storage = crate::resource::SyncCell::reserve_arc_storage(&device.allocator_stats)?;
        let memory = device.allocator_stats.reserve_cpu_bytes(
            super::allocator::shared_cpu_bytes::<Self>()?,
            crate::device::BudgetCategory::Live,
        )?;
        let info = vk::BufferCreateInfo::default()
            .size(desc.size)
            .usage(desc.usage)
            .sharing_mode(if sharing.is_concurrent() {
                vk::SharingMode::CONCURRENT
            } else {
                vk::SharingMode::EXCLUSIVE
            })
            .queue_family_indices(sharing.families());
        let handle = super::error::observe_vulkan(&device, unsafe {
            device.device.create_buffer(&info, None)
        })?;
        let mut dedicated_requirements = vk::MemoryDedicatedRequirements::default();
        let mut requirements2 =
            vk::MemoryRequirements2::default().push_next(&mut dedicated_requirements);
        unsafe {
            device.device.get_buffer_memory_requirements2(
                &vk::BufferMemoryRequirementsInfo2::default().buffer(handle),
                &mut requirements2,
            );
        }
        let mut requirements = requirements2.memory_requirements;
        // 0x140bee200：max(memreq.alignment, desc.alignment, usage&0x400 ? 0x40 : 0x10)。
        requirements.alignment =
            buffer_alignment(requirements.alignment, desc.alignment, desc.usage.as_raw());
        let flags = if desc
            .usage
            .contains(vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS)
        {
            vk::MemoryAllocateFlags::DEVICE_ADDRESS
        } else {
            vk::MemoryAllocateFlags::empty()
        };
        let (required_memory, preferred_memory) = desc.memory_location.memory_properties();
        let allocated = if dedicated_requirements.requires_dedicated_allocation != 0 {
            MemoryBlock::allocate_for_resource(
                device.clone(),
                requirements,
                required_memory,
                preferred_memory,
                flags,
                handle,
                vk::Image::null(),
            )
        } else if dedicated {
            MemoryBlock::allocate_dedicated(
                device.clone(),
                requirements,
                required_memory,
                preferred_memory,
                flags,
            )
        } else {
            MemoryBlock::allocate_with_flags(
                Arc::clone(&device),
                requirements,
                required_memory,
                preferred_memory,
                flags,
            )
        };
        let allocation = match allocated {
            Ok(value) => value,
            Err(error) => {
                unsafe {
                    device.device.destroy_buffer(handle, None);
                }
                return Err(error);
            }
        };
        let last_use = if sharing.is_concurrent() {
            crate::resource::SyncCell::for_allocation(
                device.generation(),
                device.allocator_stats.clone(),
                allocation.accounting_id(),
            )
            .with_concurrent_families(match sharing {
                crate::resource::ResourceSharing::Concurrent(families) => families,
                _ => unreachable!("concurrent branch"),
            })
            .with_ownership_range(crate::resource::OwnershipRange::Buffer {
                offset: 0,
                size: desc.size,
            })
        } else {
            crate::resource::SyncCell::for_allocation(
                device.generation(),
                device.allocator_stats.clone(),
                allocation.accounting_id(),
            )
            .with_ownership_range(crate::resource::OwnershipRange::Buffer {
                offset: 0,
                size: desc.size,
            })
        };
        let last_use = if allocation
            .properties
            .contains(vk::MemoryPropertyFlags::HOST_VISIBLE)
        {
            match last_use.with_host_buffer_ranges(desc.size, &device.allocator_stats) {
                Ok(cell) => cell,
                Err(error) => {
                    unsafe {
                        device.device.destroy_buffer(handle, None);
                    }
                    return Err(error);
                }
            }
        } else {
            last_use
        };
        if let Err(error) = unsafe {
            device
                .device
                .bind_buffer_memory(handle, allocation.handle, allocation.offset)
        } {
            unsafe {
                device.device.destroy_buffer(handle, None);
            }
            return super::error::observe_vulkan(&device, Err(error));
        }
        // 原版缓冲创建时就取得映射指针（0x140bee5f0 `0x140c113d0(block)` 返回 ptr+len），
        // 主机可见的专用分配在这里一次性映射；子分配沿用 gpu-allocator 式持久映射。
        let mapped_ptr = if allocation
            .properties
            .contains(vk::MemoryPropertyFlags::HOST_VISIBLE)
            && allocation.host_ptr().is_none()
        {
            match unsafe {
                device.device.map_memory(
                    allocation.handle,
                    0,
                    vk::WHOLE_SIZE,
                    vk::MemoryMapFlags::empty(),
                )
            } {
                Ok(pointer) => Some(pointer as usize),
                Err(error) => {
                    unsafe { device.device.destroy_buffer(handle, None) };
                    return super::error::observe_vulkan(&device, Err(error));
                }
            }
        } else {
            None
        };
        Ok(Self {
            dynamic_constants_page: false,
            _memory: memory,
            sharing,
            allocation_use: None,
            last_use: last_use.into_reserved_arc(sync_storage),
            device,
            handle,
            size: desc.size,
            usage: desc.usage,
            desc: *desc,
            allocation: std::mem::ManuallyDrop::new(allocation),
            mapped_ptr,
            host_access: parking_lot::Mutex::new(()),
        })
    }

    /// 原版 `Buffer::mapped_slice_mut`（0x140bd4cd0）的全长形态：不可映射（设备本地内存）返回
    /// `None`，否则返回覆盖整个缓冲（`desc.size`）的切片。原签名是 `&self → &mut [u8]`，
    /// 别名纪律由调用方保证（与原版一致）。旧的 `(offset,size)` 形态见 [`Self::mapped_slice_mut`]。
    /// # Safety
    /// The caller must exclude all host/GPU accesses for the returned borrow,
    /// wait for previous uses, and flush non-coherent writes before submission.
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn mapped_full_slice_mut(&self) -> Option<&mut [u8]> {
        if self.device.is_device_lost() {
            return None;
        }
        if self.allocation.is_shared() {
            return None;
        }
        let len = usize::try_from(self.size).ok()?;
        if self.size > self.allocation.size {
            return None;
        }
        let ptr = self.persistent_host_ptr()?;
        // SAFETY: The mapping covers this live allocation; the caller provides
        // exclusive host/GPU access and non-coherent visibility.
        Some(unsafe { std::slice::from_raw_parts_mut(ptr, len) })
    }

    /// 请求持久映射；首次调用建立映射并保留到 `drop`（或下一次 `unmap`）。
    /// 返回的可变切片在下次映射/释放前保持有效，调用者负责同步边界。
    /// # Safety
    /// The caller must establish GPU completion and non-coherent visibility,
    /// and prevent submissions from using the mapped region during this borrow.
    pub unsafe fn mapped_slice_mut(&mut self, offset: u64, size: u64) -> Result<&mut [u8]> {
        if self.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        if self.allocation.is_shared() {
            return Err(Error::unsupported(
                "graph transient memory cannot be mapped",
            ));
        }
        let end = offset
            .checked_add(size)
            .ok_or(Error::invalid_usage("映射范围上溢"))?;
        if end > self.size {
            return Err(Error::invalid_usage("映射访问越过缓冲区边界"));
        }
        if !self
            .allocation
            .properties
            .contains(vk::MemoryPropertyFlags::HOST_VISIBLE)
        {
            return Err(Error::unsupported("内存不可映射"));
        }
        let address = match self.mapped_ptr {
            Some(address) => address,
            // 子分配：gpu-allocator 已持久映射整块，指针已含偏移；不能再 vkMapMemory 同一块内存。
            None if self.allocation.host_ptr().is_some() => {
                self.allocation.host_ptr().map_or(0, |p| p as usize)
            }
            None => {
                self.device.wait(&self.last_use.snapshot(), u64::MAX)?;
                let pointer = super::error::observe_vulkan(&self.device, unsafe {
                    self.device.device.map_memory(
                        self.allocation.handle,
                        0,
                        vk::WHOLE_SIZE,
                        vk::MemoryMapFlags::empty(),
                    )
                })?;
                let address = pointer as usize;
                self.mapped_ptr = Some(address);
                address
            }
        };
        // SAFETY: 上面已建立/复用本块持续映射；映射至少覆盖整个内存对象，
        // 返回的借用只在调用方持有时有效，期间不允许再 unmap 或 drop。
        let start = address as *mut u8;
        let start = unsafe { start.add(offset as usize) };
        Ok(unsafe { std::slice::from_raw_parts_mut(start, size as usize) })
    }

    /// 子分配（gpu-allocator 持久映射）的主机指针，已含块内偏移；与原版 `mapped_slice_mut`
    /// 同源（原版缓冲恒持久映射）。专用分配或不可映射内存返回 `None`。
    /// 调用者负责同步，并保证指针只在本缓冲存活期间使用。
    pub fn persistent_host_ptr(&self) -> Option<*mut u8> {
        if self.device.is_device_lost() {
            return None;
        }
        self.allocation
            .host_ptr()
            .map(|p| p.cast())
            .or(self.mapped_ptr.map(|address| address as *mut u8))
    }

    /// 释放持久映射；不撤销借用（调用者必须先结束对切片的借用）。
    pub fn unmap(&mut self) {
        if self.mapped_ptr.take().is_some() {
            unsafe {
                self.device.device.unmap_memory(self.allocation.handle);
            }
        }
    }

    /// 宿主刷新/失效范围 `(偏移, 大小)`，供测试核对子分配的对齐前提。
    #[doc(hidden)]
    pub fn host_range_for_test(&self) -> (u64, u64) {
        self.allocation.host_range()
    }
    /// 地址只能用于具有 DEVICE_ADDRESS 分配标记且设备已启用 BDA 的缓冲。
    pub fn device_address(&self) -> Result<vk::DeviceAddress> {
        if self.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        if !self.device.capabilities.buffer_device_address
            || !self
                .usage
                .contains(vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS)
        {
            return Err(Error::unsupported("缓冲区未启用设备地址用途"));
        }
        let info = vk::BufferDeviceAddressInfo::default().buffer(self.handle);
        Ok(unsafe { self.device.device.get_buffer_device_address(&info) })
    }
    fn transfer(&self, offset: u64, mut transfer: HostTransfer<'_>, sync: bool) -> Result<()> {
        if self.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        if self.allocation.is_shared() {
            return Err(Error::unsupported(
                "graph transient memory cannot be mapped",
            ));
        }
        let write = matches!(transfer, HostTransfer::Write(_));
        let len = transfer.len();
        if offset
            .checked_add(len as u64)
            .is_none_or(|end| end > self.size)
        {
            return Err(Error::invalid_usage("映射访问越过缓冲区边界"));
        }
        if !self
            .allocation
            .properties
            .contains(vk::MemoryPropertyFlags::HOST_VISIBLE)
        {
            return Err(Error::unsupported("内存不可映射"));
        }
        if len == 0 {
            return Ok(());
        }
        let coherent = self
            .allocation
            .properties
            .contains(vk::MemoryPropertyFlags::HOST_COHERENT);
        let host_range = self.allocation.host_subrange(
            self.size,
            offset,
            len as u64,
            if coherent {
                1
            } else {
                self.device.limits().non_coherent_atom_size.max(1)
            },
        )?;

        let _host = self.host_access.lock();
        let _reservation = self.last_use.lock_submission();
        let completed = self.device.poll()?;
        let last_use = self.last_use.buffer_range_snapshot(
            host_range.buffer_offset,
            host_range.buffer_size,
            write,
            &completed,
        )?;
        if sync {
            self.device.wait(&last_use, u64::MAX)?;
        } else if !last_use.is_reached(&completed) {
            return Err(Error::Timeout);
        }
        self.last_use.prepare_buffer_host_range(
            host_range.buffer_offset,
            host_range.buffer_size,
            write,
        )?;
        let mut host_state = HostRangeState {
            cell: &self.last_use,
            accessed: false,
        };
        // 子分配直接用已持久映射的指针；专用分配每次自己 map/unmap。
        let shared_pointer = self
            .allocation
            .host_ptr()
            .or(self.mapped_ptr.map(|a| a as *mut u8));
        let pointer = match shared_pointer {
            Some(pointer) => pointer.cast(),
            None => super::error::observe_vulkan(&self.device, unsafe {
                self.device.device.map_memory(
                    self.allocation.handle,
                    0,
                    vk::WHOLE_SIZE,
                    vk::MemoryMapFlags::empty(),
                )
            })?,
        };
        let range = [vk::MappedMemoryRange::default()
            .memory(self.allocation.handle)
            .offset(host_range.memory_offset)
            .size(host_range.memory_size)];
        let unmap = || {
            if shared_pointer.is_none() {
                unsafe { self.device.device.unmap_memory(self.allocation.handle) };
            }
        };
        let result = if !write && !coherent {
            unsafe { self.device.device.invalidate_mapped_memory_ranges(&range) }
        } else {
            Ok(())
        };
        if let Err(error) = super::error::observe_vulkan(&self.device, result) {
            unmap();
            return Err(error);
        }
        unsafe {
            let region = pointer.cast::<u8>().add(offset as usize);
            match &mut transfer {
                HostTransfer::Write(bytes) => {
                    std::ptr::copy_nonoverlapping(bytes.as_ptr(), region, len)
                }
                HostTransfer::Read(bytes) => {
                    std::ptr::copy_nonoverlapping(region, bytes.as_mut_ptr(), len)
                }
            }
        }
        // A failing flush does not undo the CPU copy. Publish the conservative
        // host revision even on that error, so bound native programs go stale.
        host_state.accessed = true;
        let result = if write && !coherent {
            unsafe { self.device.device.flush_mapped_memory_ranges(&range) }
        } else {
            Ok(())
        };
        unmap();
        super::error::observe_vulkan(&self.device, result)
    }
    pub fn write_bytes(&self, offset: u64, bytes: &[u8]) -> Result<()> {
        self.transfer(offset, HostTransfer::Write(bytes), true)
    }
    /// 不等待在飞 GPU 工作的写入。
    ///
    /// 未完成的重叠 GPU 使用（未知范围按整缓冲处理）返回 Timeout；非 coherent 内存按原子粒度检查。
    pub fn write_bytes_unsynchronized(&self, offset: u64, bytes: &[u8]) -> Result<()> {
        self.transfer(offset, HostTransfer::Write(bytes), false)
    }
    /// 在已确认本缓冲 GPU 操作完成后读回，保留非 coherent 内存失效处理，不做整设备等待。
    ///
    /// # Safety
    /// 调用者必须等待覆盖本缓冲全部在飞访问的 fence/timeline 提交凭据，
    /// 并保证复制期间没有其他 GPU 或宿主并发访问。GPU 写入还需已建立 HOST_READ 可见性。
    /// 此调用只依赖本缓冲完成，不推进设备全局完成 epoch。
    pub unsafe fn read_bytes_after_completion(&self, offset: u64, bytes: &mut [u8]) -> Result<()> {
        self.transfer(offset, HostTransfer::Read(bytes), false)
    }
    pub fn read_bytes(&self, offset: u64, bytes: &mut [u8]) -> Result<()> {
        self.transfer(offset, HostTransfer::Read(bytes), true)
    }
}
impl Drop for Buffer {
    fn drop(&mut self) {
        if self.mapped_ptr.take().is_some() {
            unsafe {
                self.device.device.unmap_memory(self.allocation.handle);
            }
        }
        // SAFETY: allocation 只在此处取出一次，之后不会再被访问。
        let memory = unsafe { std::mem::ManuallyDrop::take(&mut self.allocation) }.into_deferred();
        self.device.retire(
            self.last_use.snapshot(),
            Doomed::Buffer(self.handle, memory),
        );
    }
}

impl crate::resource::GpuUse for Buffer {
    fn visit_uses(&self, visitor: &mut crate::resource::UseVisitor<'_>) -> crate::Result<()> {
        visitor(&self.last_use)?;
        if let Some(cell) = &self.allocation_use {
            visitor(cell)?;
        }

        Ok(())
    }
}
