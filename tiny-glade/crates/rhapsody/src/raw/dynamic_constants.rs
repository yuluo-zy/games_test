//! 动态常量页的可运行候选实现；原页大小/复用策略仍待整链对照。
//! 设备实际支持并启用 BDA 时提供真实地址；其余设备明确返回 Unsupported。
use super::{
    binding::{DescriptorSetBinding, ResourceOwner},
    buffer::{Buffer, BufferDesc},
    device::VulkanDevice,
    error::{Error, Result},
};
use ash::vk;
use std::sync::Arc;

pub struct DynamicConstantsChunk {
    pub buffer: Arc<Buffer>,
    pub offset: u64,
    pub size: u64,
}
impl DynamicConstantsChunk {
    pub fn write_bytes(&self, bytes: &[u8]) -> Result<()> {
        if bytes.len() as u64 > self.size {
            return Err(Error::invalid_usage("数据超过常量区段范围"));
        }
        self.buffer.write_bytes(self.offset, bytes)
    }
    /// 原入口名；与 `write_bytes` 同一实现。
    pub fn write(&self, bytes: &[u8]) -> Result<()> {
        self.write_bytes(bytes)
    }
    /// 用既有 buffer 与偏移构造带数据区段。
    pub fn with_data(buffer: Arc<Buffer>, offset: u64, size: u64, bytes: &[u8]) -> Result<Self> {
        let chunk = Self {
            buffer,
            offset,
            size,
        };
        chunk.write_bytes(bytes)?;
        Ok(chunk)
    }
    pub fn device_address(&self) -> Result<vk::DeviceAddress> {
        self.buffer
            .device_address()?
            .checked_add(self.offset)
            .ok_or(Error::invalid_usage("设备地址溢出"))
    }
    pub fn into_gpu_lifetime_tracker(self) -> ResourceOwner {
        self.buffer
    }
    fn binding(&self, ty: vk::DescriptorType) -> DescriptorSetBinding {
        DescriptorSetBinding::Buffer {
            ty,
            info: vk::DescriptorBufferInfo {
                buffer: self.buffer.handle,
                offset: self.offset,
                range: self.size,
            },
            owner: self.buffer.clone(),
        }
    }
    pub fn into_uniform_buffer_binding(self) -> DescriptorSetBinding {
        self.binding(vk::DescriptorType::UNIFORM_BUFFER)
    }
    pub fn into_uniform_and_storage_buffer_binding(
        self,
    ) -> (DescriptorSetBinding, DescriptorSetBinding) {
        (
            self.binding(vk::DescriptorType::UNIFORM_BUFFER),
            self.binding(vk::DescriptorType::STORAGE_BUFFER),
        )
    }
    /// 候选接口返回安全的数据副本；不暴露受 GPU 生命周期约束的裸映射指针。
    pub fn into_uniform_buffer_binding_and_data(self) -> Result<(DescriptorSetBinding, Vec<u8>)> {
        let mut bytes = vec![
            0;
            usize::try_from(self.size)
                .map_err(|_| Error::invalid_usage("区段长度无法表示"))?
        ];
        self.buffer.read_bytes(self.offset, &mut bytes)?;
        Ok((self.binding(vk::DescriptorType::UNIFORM_BUFFER), bytes))
    }
}

/// 原 `DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES`：每页大小，也是单次分配上限
/// （0x140c02740 开头 `cmp r9,0x40001; jae` → `assert!(bytes <= DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES)`）。
pub const DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES: u64 = 0x40000;
/// 页缓冲用途（0x140c029f0 写入 `BufferDesc.usage = 0x20033`）：
/// TRANSFER_SRC | TRANSFER_DST | UNIFORM_BUFFER | STORAGE_BUFFER | SHADER_DEVICE_ADDRESS，
/// 不随设备能力裁剪（原版要求 BDA）。
pub const DYNAMIC_CONSTANTS_USAGE: vk::BufferUsageFlags = vk::BufferUsageFlags::from_raw(0x20033);
/// 默认对齐：显式对齐 <= 16 或未请求时取 16。
const DEFAULT_ALIGNMENT: u64 = 16;

struct Page {
    buffer: Arc<Buffer>,
}
/// 原 `DynamicConstants`（0x140c02740 `allocate`）：只有一张“当前页”，线性 bump；放不下就新建
/// 一页（256 KiB、CpuToGpu、持久映射、名字 `"dynamic constants buffer"`）并把旧页留在页列表里，
/// 由持有它的 chunk / descriptor 通过 `Arc` 延续到 GPU 用完。没有 reset，也没有整设备等待：
/// 页的回收完全交给资源生命周期（原版 0x140c02370 `into_gpu_lifetime_tracker` 同理）。
pub struct DynamicConstants {
    device: Arc<VulkanDevice>,
    /// 设备下限对齐（原版缓存在 `self+0x20`，值取自设备属性 `+0x340` 处的标量；具体对应哪个
    /// Vulkan limit 未证实，这里取 uniform/storage 偏移对齐的较大者）。
    device_alignment: u64,
    cursor: u64,
    pages: Vec<Page>,
}
impl DynamicConstants {
    pub fn new(device: Arc<VulkanDevice>) -> Result<Self> {
        let limits = unsafe {
            device
                .instance
                .instance
                .get_physical_device_properties(device.physical)
        }
        .limits;
        let device_alignment = limits
            .min_uniform_buffer_offset_alignment
            .max(limits.min_storage_buffer_offset_alignment)
            .max(1);
        // 原版把该值放进 `BufferDesc.alignment`（u16），>= 0x10000 时 panic。
        if device_alignment >= 0x10000 || !device_alignment.is_power_of_two() {
            return Err(Error::invalid_usage("常量页对齐无效"));
        }
        Ok(Self {
            device,
            device_alignment,
            cursor: 0,
            pages: Vec::new(),
        })
    }
    pub fn allocate(&mut self, size: u64) -> Result<DynamicConstantsChunk> {
        self.allocate_aligned(size, None)
    }
    /// `alignment > 16` 才生效，再与设备对齐取较大者。超过页大小返回 `InvalidUsage`。
    pub fn allocate_aligned(
        &mut self,
        size: u64,
        alignment: Option<u64>,
    ) -> Result<DynamicConstantsChunk> {
        if size > DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES {
            return Err(Error::invalid_usage(format!(
                "常量区段 {size} 字节超过页大小 {DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES}"
            )));
        }
        let requested = match alignment {
            Some(a) if a > DEFAULT_ALIGNMENT => a,
            _ => DEFAULT_ALIGNMENT,
        };
        let alignment = requested.max(self.device_alignment);
        if !alignment.is_power_of_two() {
            return Err(Error::invalid_usage("常量区段对齐必须是 2 的幂"));
        }
        let mut offset = align_up(self.cursor, alignment)?;
        let page = match self.pages.last() {
            Some(page) if offset + size <= DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES => page.buffer.clone(),
            _ => {
                let buffer = Arc::new(Buffer::new(
                    self.device.clone(),
                    &BufferDesc::new_cpu_to_gpu(DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES, DYNAMIC_CONSTANTS_USAGE)
                        .with_alignment(self.device_alignment as u16),
                )?);
                self.pages.push(Page { buffer: buffer.clone() });
                offset = 0;
                buffer
            }
        };
        self.cursor = offset + size;
        Ok(DynamicConstantsChunk {
            buffer: page,
            offset,
            size,
        })
    }
    pub fn allocate_with_data(&mut self, bytes: &[u8]) -> Result<DynamicConstantsChunk> {
        let chunk = self.allocate(bytes.len() as u64)?;
        chunk.write_bytes(bytes)?;
        Ok(chunk)
    }
    /// 原入口名；与 `allocate` 同一 bump 分配语义。
    pub fn push(&mut self, size: u64) -> Result<DynamicConstantsChunk> {
        self.allocate(size)
    }
    /// 原入口名；与 `allocate_with_data` 同一实现。
    pub fn push_slice(&mut self, bytes: &[u8]) -> Result<DynamicConstantsChunk> {
        self.allocate_with_data(bytes)
    }
    pub fn page_count(&self) -> usize {
        self.pages.len()
    }
}
fn align_up(value: u64, alignment: u64) -> Result<u64> {
    value
        .checked_add(alignment - 1)
        .map(|v| v & !(alignment - 1))
        .ok_or(Error::invalid_usage("常量区段对齐溢出"))
}
