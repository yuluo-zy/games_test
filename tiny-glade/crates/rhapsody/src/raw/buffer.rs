//! Buffer 创建、内存绑定、映射读写候选；`BufferDesc` 对齐原 16 字节描述符布局。
use super::{
    allocator::MemoryBlock,
    device::VulkanDevice,
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

/// 原版 `Buffer` 的 40 字节值布局（`create_buffer` 的 Ok 载荷，0x140bee6d1 / 0x140c035af 机器码）。
/// 游戏侧（`TypedStorageBuffer`/`MappedStorageBuffer`）整块搬运它；rhapsody 内部的 [`Buffer`]
/// 自带 `Arc` 所有权，两者之间用 [`Buffer::record`] 转换。
/// `ctx`/`slot` 在原版是 `ManagedResources` 与槽位的裸指针；这里由调用方的槽表给出标识。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BufferRecord {
    /// `*(context)`：退役队列头指针（游戏面称 id）。
    pub ctx: u64,
    /// 槽位（`ManagedResource<ManagedBuffer>`）。
    pub slot: u64,
    /// 分配内偏移（`*(slot+0xc)`）。
    pub offset: u32,
    /// owning_device 代号（`*(context+0xa50)`）。
    pub device_id: u16,
    /// 恒为 1（valid/ready）。
    pub ready: u8,
    pub _pad: u8,
    pub size: u64,
    /// `BufferDesc+0x08..0x10`：`{usage u32, alignment u16, memory_location u8, pad}`。
    pub desc_tail: u64,
}
const _: () = assert!(std::mem::size_of::<BufferRecord>() == 40);
impl BufferRecord {
    pub fn usage(&self) -> vk::BufferUsageFlags {
        vk::BufferUsageFlags::from_raw(self.desc_tail as u32)
    }
    pub fn alignment(&self) -> u16 {
        (self.desc_tail >> 32) as u16
    }
    pub fn memory_location(&self) -> Option<MemoryLocation> {
        MemoryLocation::from_raw((self.desc_tail >> 48) as u8)
    }
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
    requirement_alignment.max(u64::from(desc_alignment)).max(floor)
}

/// 原版 `slot`（`ManagedResource<ManagedBuffer>`，`inner`）里被各消费函数读取的字段视图。
/// 偏移来自 0x140bd4c00 / 0x140bd4cd0 / 0x140beec30 / 0x140beefa0 的机器码读取点：
/// `+0x0c` 偏移、`+0x10/+0x18` 名字 `&str`、`+0x20` 容量、`+0x28` `{usage,align,mem}`、
/// `+0x30` **VkBuffer**（device_address 与命名都把它当缓冲句柄）、`+0x38` 内存块、`+0x7c` 分配状态字节。
#[derive(Clone, Copy, Debug)]
pub struct BufferSlotView {
    /// `inner+0x0c`。
    pub offset: u32,
    /// `inner+0x20`：`BufferDesc.size`。
    pub capacity: u64,
    /// `inner+0x28`：`BufferDesc+0x08..0x10`，低 32 位是 usage。
    pub desc_tail: u64,
    /// `inner+0x30`。
    pub vk_buffer: vk::Buffer,
    /// `0x140c113b0(inner+0x38)`：分配的主机映射指针，空表示不可映射。
    pub mapped_ptr: *mut u8,
    /// `0x140c02280(inner+0x38)`（= 块 `+0x10`）：分配块大小。
    pub block_size: u64,
    /// `inner+0x7c`；`2` 表示槽里的分配已被取走（Option 空位的 niche，见报告 U#4）。
    pub allocation_state: u8,
}
/// `inner+0x7c == 2` 的哨兵。机器码证据：`inner+0x38` 起是内存块（`0x140c02280(inner+0x38)` 读 `+0x10` 块大小），
/// `+0x7c` 恰是该块的 `+0x44` 字节；创建纹理（0x140bdc2d0）时对同一字节的判据是
/// `*(byte*)(allocation + 0x44) != 2`（为 2 则跳过分配释放、纹理里写 3 的状态字节在 `+0xbc = 0x78+0x44`），
/// 所以它是 `Option<分配>` 的 niche：值域 `{0,1,…}` 为有分配、`2` 为 `None`。
/// 取走后调 `mapped_slice_mut` 走 `managed/buffer.rs:40:34` 的单参 `unwrap_failed`（0x140bd4dc9，`rcx=0x142d02388`）。
/// 全 .text 扫描：没有任何 `mov byte/dword [reg+0x7c], 2` 立即数写入槽（只有 5 处写栈局部），
/// 所以“谁把它置成 2”（取走分配的函数）未定位；这里只落“读判据”。
pub const SLOT_ALLOCATION_TAKEN: u8 = 2;
impl BufferSlotView {
    /// `inner+0x28 & 2`：`update_buffer_bytes` 的门控位 = `usage` 的 `TRANSFER_DST`，不是内存属性。
    pub fn has_transfer_dst(&self) -> bool {
        self.desc_tail & u64::from(vk::BufferUsageFlags::TRANSFER_DST.as_raw()) != 0
    }
}

/// 缓冲句柄/槽校验失败。
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum BufferAccessError {
    #[error("缓冲句柄属于设备 {handle}，当前设备为 {device}")]
    DeviceMismatch { handle: u16, device: u16 },
    #[error("缓冲句柄未就绪（flag={flag}）")]
    NotReady { flag: u8 },
    #[error("缓冲句柄已过期：句柄槽 {handle}，当前槽 {slot}")]
    StaleSlot { handle: u32, slot: u32 },
    #[error("缓冲的内存分配已被取走")]
    AllocationTaken,
    #[error("缓冲大小 {size} 超过所在内存块 {block}")]
    SizeExceedsBlock { size: u64, block: u64 },
}

impl From<BufferAccessError> for crate::Error {
    fn from(error: BufferAccessError) -> Self {
        crate::Error::invalid_usage_with("缓冲访问无效", error)
    }
}

/// 缓冲更新（`update_buffer_bytes`）的参数校验失败。
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum BufferUpdateError {
    #[error(transparent)]
    Access(#[from] BufferAccessError),
    #[error("更新区间 {start}..{end} 起点大于终点")]
    RangeInverted { start: usize, end: usize },
    #[error("更新区间终点 {end} 超过源数据长度 {len}")]
    RangeOutOfData { end: usize, len: usize },
    #[error("更新越界：{total} 字节写到偏移 {dst_offset}，缓冲容量 {capacity}")]
    ExceedsCapacity { total: u64, dst_offset: u64, capacity: u64 },
    #[error("缓冲必须带 TRANSFER_DST 用途才能更新")]
    MissingTransferDst,
    #[error("缓冲更新的总字节数为 0")]
    Empty,
}

impl From<BufferUpdateError> for crate::Error {
    fn from(error: BufferUpdateError) -> Self {
        crate::Error::invalid_usage_with("缓冲更新无效", error)
    }
}

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

/// 原版 `update_buffer_bytes` 的判定顺序：句柄三查 → 逐区间（`start<=end`、`end<=data_len`、累计后
/// `total + dst_offset <= capacity`）→ `TRANSFER_DST` → `total > 0`。
///
/// 原版只比较 `total + dst_offset`，不核对每个区域的 `dst_end`；这里**额外**拒绝
/// `dst_offset + range.end > capacity`（那会让 `vkCmdCopyBuffer` 越界），属于有意的超集。
pub fn plan_buffer_update(
    record: &BufferRecord,
    device_id: u16,
    slot: &BufferSlotView,
    data_len: usize,
    ranges: &[std::ops::Range<usize>],
    dst_offset: u64,
) -> std::result::Result<BufferUpdatePlan, BufferUpdateError> {
    record.validate(device_id, slot)?;
    let mut total = 0u64;
    let mut regions = Vec::with_capacity(ranges.len());
    for r in ranges {
        if r.start > r.end {
            return Err(BufferUpdateError::RangeInverted { start: r.start, end: r.end });
        }
        if r.end > data_len {
            return Err(BufferUpdateError::RangeOutOfData { end: r.end, len: data_len });
        }
        let len = (r.end - r.start) as u64;
        regions.push(BufferRegion {
            src_offset: total,
            dst_offset: r.start as u64 + dst_offset,
            size: len,
        });
        total += len;
        if total.saturating_add(dst_offset) > slot.capacity
            || (r.end as u64).saturating_add(dst_offset) > slot.capacity
        {
            return Err(BufferUpdateError::ExceedsCapacity {
                total,
                dst_offset,
                capacity: slot.capacity,
            });
        }
    }
    if !slot.has_transfer_dst() {
        return Err(BufferUpdateError::MissingTransferDst);
    }
    if total == 0 {
        return Err(BufferUpdateError::Empty);
    }
    Ok(BufferUpdatePlan { total, regions })
}

impl BufferRecord {
    /// 三查（0x140bd4c00 / 0x140bd4cd0 / 0x140beefa0 开头一致）：设备代号 → `flag == 1` → 槽偏移。
    pub fn validate(
        &self,
        device_id: u16,
        slot: &BufferSlotView,
    ) -> std::result::Result<(), BufferAccessError> {
        if self.device_id != device_id {
            return Err(BufferAccessError::DeviceMismatch {
                handle: self.device_id,
                device: device_id,
            });
        }
        if self.ready != 1 {
            return Err(BufferAccessError::NotReady { flag: self.ready });
        }
        if slot.offset != self.offset {
            return Err(BufferAccessError::StaleSlot {
                handle: self.offset,
                slot: slot.offset,
            });
        }
        Ok(())
    }

    /// `Buffer::device_address`（0x140bd4c00）：三查后以 `inner+0x30` 的 VkBuffer 构造
    /// `VkBufferDeviceAddressInfo`（sType 1000244001）并调 `device+0x508`（`vkGetBufferDeviceAddress`）。
    /// **没有**缓存：每次都走设备函数（修正旧注“inner+0x30 缓存地址”）。
    pub fn device_address(
        &self,
        device_id: u16,
        slot: &BufferSlotView,
        get_buffer_device_address: impl FnOnce(vk::Buffer) -> vk::DeviceAddress,
    ) -> std::result::Result<vk::DeviceAddress, BufferAccessError> {
        self.validate(device_id, slot)?;
        Ok(get_buffer_device_address(slot.vk_buffer))
    }

    /// `Buffer::mapped_slice_mut`（0x140bd4cd0）：三查 → `inner+0x7c == 2` 失败 → 取块映射指针，
    /// 空指针返回 `None` → `Buffer+0x18 > 块大小` 失败 → 返回 `(ptr, Buffer+0x18)` 全长切片。
    pub fn mapped_slice_parts(
        &self,
        device_id: u16,
        slot: &BufferSlotView,
    ) -> std::result::Result<Option<(*mut u8, usize)>, BufferAccessError> {
        self.validate(device_id, slot)?;
        if slot.allocation_state == SLOT_ALLOCATION_TAKEN {
            return Err(BufferAccessError::AllocationTaken);
        }
        if slot.mapped_ptr.is_null() {
            return Ok(None);
        }
        if self.size > slot.block_size {
            return Err(BufferAccessError::SizeExceedsBlock {
                size: self.size,
                block: slot.block_size,
            });
        }
        Ok(Some((slot.mapped_ptr, self.size as usize)))
    }
}

/// `create_mapped_buffer`（0x140beea80）的返回值：40B Buffer + 映射 `(ptr, len)`（0x40B = tag 8 +
/// 40 + 16；旧文档的“额外 24B”不对，机器码 `param_1[6]=impl+0x38`、`param_1[7]=impl+0x40` 只有两个 qword）。
pub struct MappedBufferParts {
    pub ptr: *mut u8,
    pub len: usize,
}

pub struct Buffer {
    pub device: Arc<VulkanDevice>,
    pub handle: vk::Buffer,
    pub size: u64,
    pub usage: vk::BufferUsageFlags,
    /// 创建时的原 16 字节描述（图导入需要完整的 alignment/memory_location）。
    pub desc: BufferDesc,
    allocation: std::mem::ManuallyDrop<MemoryBlock>,
    mapped_ptr: Option<usize>,
}
impl Buffer {
    /// 转成原版 40 字节记录；`ctx`/`slot`/`offset`/`device_id` 由托管槽表提供。
    pub fn record(&self, ctx: u64, slot: u64, offset: u32, device_id: u16) -> BufferRecord {
        BufferRecord {
            ctx,
            slot,
            offset,
            device_id,
            ready: 1,
            _pad: 0,
            size: self.size,
            desc_tail: u64::from(self.usage.as_raw())
                | (u64::from(self.desc.alignment) << 32)
                | (u64::from(self.desc.memory_location.as_raw()) << 48),
        }
    }
    pub fn new(device: Arc<VulkanDevice>, desc: &BufferDesc) -> Result<Self> {
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
        let info = vk::BufferCreateInfo::default()
            .size(desc.size)
            .usage(desc.usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        let handle = unsafe { device.device.create_buffer(&info, None) }?;
        let mut requirements = unsafe { device.device.get_buffer_memory_requirements(handle) };
        // 0x140bee200：max(memreq.alignment, desc.alignment, usage&0x400 ? 0x40 : 0x10)。
        requirements.alignment = buffer_alignment(
            requirements.alignment,
            desc.alignment,
            desc.usage.as_raw(),
        );
        let flags = if desc
            .usage
            .contains(vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS)
        {
            vk::MemoryAllocateFlags::DEVICE_ADDRESS
        } else {
            vk::MemoryAllocateFlags::empty()
        };
        let (required_memory, preferred_memory) = desc.memory_location.memory_properties();
        let allocation = match MemoryBlock::allocate_with_flags(
            Arc::clone(&device),
            requirements,
            required_memory,
            preferred_memory,
            flags,
        ) {
            Ok(value) => value,
            Err(error) => {
                unsafe {
                    device.device.destroy_buffer(handle, None);
                }
                return Err(error);
            }
        };
        if let Err(error) = unsafe {
            device
                .device
                .bind_buffer_memory(handle, allocation.handle, allocation.offset)
        } {
            unsafe {
                device.device.destroy_buffer(handle, None);
            }
            return Err(error.into());
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
                    return Err(error.into());
                }
            }
        } else {
            None
        };
        Ok(Self {
            device,
            handle,
            size: desc.size,
            usage: desc.usage,
            desc: *desc,
            allocation: std::mem::ManuallyDrop::new(allocation),
            mapped_ptr,
        })
    }

    /// 槽视图（见 [`BufferSlotView`]）；`offset` 是调用方槽表里的分配内偏移（`inner+0x0c`）。
    pub fn slot_view(&self, offset: u32) -> BufferSlotView {
        let mapped = self
            .allocation
            .host_ptr()
            .or(self.mapped_ptr.map(|a| a as *mut u8))
            .unwrap_or(std::ptr::null_mut());
        BufferSlotView {
            offset,
            capacity: self.size,
            desc_tail: u64::from(self.desc.usage.as_raw())
                | (u64::from(self.desc.alignment) << 32)
                | (u64::from(self.desc.memory_location.as_raw()) << 48),
            vk_buffer: self.handle,
            mapped_ptr: mapped,
            block_size: self.allocation.size,
            allocation_state: 0,
        }
    }

    /// 原版 `Buffer::mapped_slice_mut`（0x140bd4cd0）的全长形态：不可映射（设备本地内存）返回
    /// `None`，否则返回覆盖整个缓冲（`desc.size`）的切片。原签名是 `&self → &mut [u8]`，
    /// 别名纪律由调用方保证（与原版一致）。旧的 `(offset,size)` 形态见 [`Self::mapped_slice_mut`]。
    #[allow(clippy::mut_from_ref)]
    pub fn mapped_full_slice_mut(&self) -> Option<&mut [u8]> {
        let record = self.record(0, 0, 0, 0);
        let slot = self.slot_view(0);
        let parts = record.mapped_slice_parts(0, &slot).ok()??;
        // SAFETY: 指针来自本缓冲仍存活的持久映射，长度 = desc.size <= 块大小（mapped_slice_parts 已校验）。
        Some(unsafe { std::slice::from_raw_parts_mut(parts.0, parts.1) })
    }

    /// 请求持久映射；首次调用建立映射并保留到 `drop`（或下一次 `unmap`）。
    /// 返回的可变切片在下次映射/释放前保持有效，调用者负责同步边界。
    pub fn mapped_slice_mut(&mut self, offset: u64, size: u64) -> Result<&mut [u8]> {
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
                let _queue = self.device.queue_lock.lock();
                self.device.idle_if_in_flight_locked()?;
                let pointer = unsafe {
                    self.device.device.map_memory(
                        self.allocation.handle,
                        0,
                        vk::WHOLE_SIZE,
                        vk::MemoryMapFlags::empty(),
                    )
                }?;
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
        self.allocation.host_ptr().map(|p| p.cast())
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
    fn transfer(&self, offset: u64, bytes: &mut [u8], write: bool, sync: bool) -> Result<()> {
        if offset
            .checked_add(bytes.len() as u64)
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
        let _queue = self.device.queue_lock.lock();
        if sync {
            self.device.idle_if_in_flight_locked()?;
        }
        // 子分配直接用已持久映射的指针；专用分配每次自己 map/unmap。
        let shared_pointer = self
            .allocation
            .host_ptr()
            .or(self.mapped_ptr.map(|a| a as *mut u8));
        let pointer = match shared_pointer {
            Some(pointer) => pointer.cast(),
            None => unsafe {
                self.device.device.map_memory(
                    self.allocation.handle,
                    0,
                    vk::WHOLE_SIZE,
                    vk::MemoryMapFlags::empty(),
                )
            }?,
        };
        let (range_offset, range_size) = self.allocation.host_range();
        let range = [vk::MappedMemoryRange::default()
            .memory(self.allocation.handle)
            .offset(range_offset)
            .size(range_size)];
        let unmap = || {
            if shared_pointer.is_none() {
                unsafe { self.device.device.unmap_memory(self.allocation.handle) };
            }
        };
        let coherent = self
            .allocation
            .properties
            .contains(vk::MemoryPropertyFlags::HOST_COHERENT);
        let result = if !write && !coherent {
            unsafe { self.device.device.invalidate_mapped_memory_ranges(&range) }
        } else {
            Ok(())
        };
        if let Err(error) = result {
            unmap();
            return Err(error.into());
        }
        unsafe {
            let region = pointer.cast::<u8>().add(offset as usize);
            if write {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), region, bytes.len());
            } else {
                std::ptr::copy_nonoverlapping(region, bytes.as_mut_ptr(), bytes.len());
            }
        }
        let result = if write && !coherent {
            unsafe { self.device.device.flush_mapped_memory_ranges(&range) }
        } else {
            Ok(())
        };
        unmap();
        Ok(result?)
    }
    pub fn write_bytes(&self, offset: u64, bytes: &[u8]) -> Result<()> {
        self.transfer(offset, &mut bytes.to_vec(), true, true)
    }
    /// 不等待在飞 GPU 工作的写入。
    ///
    /// 仅用于刚创建、GPU 从未访问过的缓冲（如上传暂存缓冲）；对可能被 GPU 使用的缓冲请用 `write_bytes`。
    pub fn write_bytes_unsynchronized(&self, offset: u64, bytes: &[u8]) -> Result<()> {
        self.transfer(offset, &mut bytes.to_vec(), true, false)
    }
    /// 在已确认本缓冲 GPU 操作完成后读回，保留非 coherent 内存失效处理，不做整设备等待。
    ///
    /// # Safety
    /// 调用者必须等待覆盖本缓冲全部在飞访问的 fence/timeline 提交凭据，
    /// 并保证复制期间没有其他 GPU 或宿主并发访问。GPU 写入还需已建立 HOST_READ 可见性。
    /// 此调用只依赖本缓冲完成，不推进设备全局完成 epoch。
    pub unsafe fn read_bytes_after_completion(&self, offset: u64, bytes: &mut [u8]) -> Result<()> {
        self.transfer(offset, bytes, false, false)
    }
    pub fn read_bytes(&self, offset: u64, bytes: &mut [u8]) -> Result<()> {
        self.transfer(offset, bytes, false, true)
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
        self.device.bury(Doomed::Buffer(self.handle, memory));
    }
}

#[cfg(test)]
mod record_tests {
    use super::*;
    use std::mem::offset_of;

    #[test]
    fn buffer_record_offsets_match_the_exe() {
        assert_eq!(offset_of!(BufferRecord, ctx), 0x00);
        assert_eq!(offset_of!(BufferRecord, slot), 0x08);
        assert_eq!(offset_of!(BufferRecord, offset), 0x10);
        assert_eq!(offset_of!(BufferRecord, device_id), 0x14);
        assert_eq!(offset_of!(BufferRecord, ready), 0x16);
        assert_eq!(offset_of!(BufferRecord, size), 0x18);
        assert_eq!(offset_of!(BufferRecord, desc_tail), 0x20);
    }

    #[test]
    fn desc_tail_packs_usage_alignment_location() {
        let r = BufferRecord {
            ctx: 0,
            slot: 0,
            offset: 0,
            device_id: 0,
            ready: 1,
            _pad: 0,
            size: 0,
            desc_tail: 0x0002_0010_0000_0020,
        };
        assert_eq!(r.usage().as_raw(), 0x20);
        assert_eq!(r.alignment(), 0x10);
        assert_eq!(r.memory_location(), Some(MemoryLocation::CpuToGpu));
    }
}
