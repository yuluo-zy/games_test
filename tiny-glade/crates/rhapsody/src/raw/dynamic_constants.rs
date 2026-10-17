//! Bounded dynamic constants pages with CPU leases and precise GPU completion.
//! 设备实际支持并启用 BDA 时提供真实地址；其余设备明确返回 Unsupported。
use super::{
    binding::{DescriptorSetBinding, ResourceOwner},
    buffer::{Buffer, BufferDesc},
    device::Device,
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
        self.buffer.write_bytes_unsynchronized(self.offset, bytes)
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
/// TRANSFER_SRC | TRANSFER_DST | UNIFORM_BUFFER | STORAGE_BUFFER | SHADER_DEVICE_ADDRESS。
/// Creation removes only the optional address bit when BDA is not enabled;
/// descriptor and parameter bindings do not require buffer addresses.
pub const DYNAMIC_CONSTANTS_USAGE: vk::BufferUsageFlags = vk::BufferUsageFlags::from_raw(0x20033);
/// 默认对齐：显式对齐 <= 16 或未请求时取 16。
const DEFAULT_ALIGNMENT: u64 = 16;

#[derive(Clone, Copy, Debug)]
pub struct DynamicConstantsConfig {
    pub max_pages: usize,
    /// Actual bound buffer allocation ranges; native allocator block slack is
    /// accounted separately by the device's global memory controller.
    pub max_backing_bytes: u64,
    pub max_idle_pages: usize,
}
impl Default for DynamicConstantsConfig {
    fn default() -> Self {
        Self {
            max_pages: 32,
            max_backing_bytes: DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES * 32,
            max_idle_pages: 4,
        }
    }
}
impl DynamicConstantsConfig {
    fn validate(self) -> Result<()> {
        if !(1..=256).contains(&self.max_pages)
            || self.max_idle_pages > self.max_pages
            || self.max_backing_bytes < DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES
        {
            return Err(Error::invalid_usage(
                "invalid bounded dynamic constants configuration",
            ));
        }
        Ok(())
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct DynamicConstantsStats {
    pub pages: usize,
    pub backing_bytes: u64,
    pub referenced_pages: usize,
    pub in_flight_pages: usize,
    pub idle_pages: usize,
    pub page_creations: u64,
    pub page_reuses: u64,
    pub allocations: u64,
    pub capacity_stalls: u64,
    pub trimmed_pages: u64,
    pub peak_backing_bytes: u64,
    pub metadata_bytes: usize,
}
trait ConstantPage {
    fn use_cell(&self) -> &Arc<crate::resource::SyncCell>;
    fn backing_bytes(&self) -> u64;
}
impl ConstantPage for Buffer {
    fn use_cell(&self) -> &Arc<crate::resource::SyncCell> {
        &self.last_use
    }
    fn backing_bytes(&self) -> u64 {
        self.allocation_bytes()
    }
}
struct Page<B> {
    buffer: Arc<B>,
    cursor: u64,
}
struct PageRing<B> {
    // Free the actual page table storage before crediting the host reservation.
    pages: Vec<Page<B>>,
    _metadata: super::allocator::CpuMemoryReservation,
    config: DynamicConstantsConfig,
    current: Option<usize>,
    completed: crate::sync::CompletedValues,
    stats: DynamicConstantsStats,
}
impl<B: ConstantPage> PageRing<B> {
    fn new(
        config: DynamicConstantsConfig,
        budget: &Arc<super::allocator::AllocatorStats>,
    ) -> Result<Self> {
        config.validate()?;
        let mut pages = Vec::new();
        pages
            .try_reserve_exact(config.max_pages)
            .map_err(|_| Error::OutOfHostMemory)?;
        let metadata_bytes = pages
            .capacity()
            .checked_mul(std::mem::size_of::<Page<B>>())
            .ok_or_else(|| Error::invalid_usage("dynamic page table capacity overflow"))?;
        let metadata =
            budget.reserve_cpu_bytes(metadata_bytes as u64, crate::device::BudgetCategory::Live)?;
        Ok(Self {
            pages,
            _metadata: metadata,
            config,
            current: None,
            completed: Default::default(),
            stats: DynamicConstantsStats {
                metadata_bytes,
                ..Default::default()
            },
        })
    }
    fn can_append(&self, size: u64, alignment: Option<u64>, device_alignment: u64) -> Result<bool> {
        let Some(index) = self.current else {
            return Ok(false);
        };
        let page = &self.pages[index];
        Ok(page.buffer.use_cell().is_reached(&self.completed)
            && allocation_offset(page.cursor, size, alignment, device_alignment)?.is_some())
    }
    fn allocate(
        &mut self,
        size: u64,
        alignment: Option<u64>,
        device_alignment: u64,
        create: impl FnOnce() -> Result<B>,
    ) -> Result<(Arc<B>, u64)> {
        allocation_offset(0, size, alignment, device_alignment)?;
        if self.can_append(size, alignment, device_alignment)? {
            let index = self.current.expect("appendable current page");
            let offset =
                allocation_offset(self.pages[index].cursor, size, alignment, device_alignment)?
                    .expect("checked placement");
            self.pages[index].cursor = offset + size;
            self.stats.allocations += 1;
            return Ok((self.pages[index].buffer.clone(), offset));
        }
        for (index, page) in self.pages.iter_mut().enumerate() {
            let cell = page.buffer.use_cell().clone();
            let _submission = cell.lock_submission();
            // get_mut also rejects outstanding Weak pointers. A Weak upgrade
            // must not resurrect an old range while this page is being reset.
            if Arc::get_mut(&mut page.buffer).is_some() && cell.is_reached(&self.completed) {
                page.cursor = size;
                self.current = Some(index);
                self.stats.page_reuses += 1;
                self.stats.allocations += 1;
                return Ok((page.buffer.clone(), 0));
            }
        }
        if self.pages.len() >= self.config.max_pages
            || self
                .config
                .max_backing_bytes
                .saturating_sub(self.stats.backing_bytes)
                < DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES
        {
            self.stats.capacity_stalls += 1;
            return Err(Error::Timeout);
        }
        let buffer = Arc::new(create()?);
        let bytes = buffer.backing_bytes();
        let total = self
            .stats
            .backing_bytes
            .checked_add(bytes)
            .filter(|total| *total <= self.config.max_backing_bytes);
        let Some(total) = total else {
            self.stats.capacity_stalls += 1;
            return Err(Error::Timeout);
        };
        self.current = Some(self.pages.len());
        self.pages.push(Page {
            buffer: buffer.clone(),
            cursor: size,
        });
        self.stats.backing_bytes = total;
        self.stats.pages = self.pages.len();
        self.stats.peak_backing_bytes = self.stats.peak_backing_bytes.max(total);
        self.stats.page_creations += 1;
        self.stats.allocations += 1;
        Ok((buffer, 0))
    }
    fn trim_idle(&mut self, keep: usize, protect_current: bool) -> usize {
        let mut remaining = self
            .pages
            .iter()
            .filter(|page| {
                Arc::strong_count(&page.buffer) == 1
                    && Arc::weak_count(&page.buffer) == 0
                    && page.buffer.use_cell().is_reached(&self.completed)
            })
            .count();
        let mut removed = 0;
        let mut index = 0;
        while index < self.pages.len() && remaining > keep {
            if protect_current && self.current == Some(index) {
                index += 1;
                continue;
            }
            let cell = self.pages[index].buffer.use_cell().clone();
            let _submission = cell.lock_submission();
            if Arc::get_mut(&mut self.pages[index].buffer).is_none()
                || !cell.is_reached(&self.completed)
            {
                index += 1;
                continue;
            }
            self.stats.backing_bytes -= self.pages[index].buffer.backing_bytes();
            self.pages.remove(index);
            if let Some(current) = self.current {
                self.current = if current == index {
                    None
                } else {
                    Some(current - usize::from(current > index))
                };
            }
            removed += 1;
            remaining -= 1;
        }
        self.stats.pages = self.pages.len();
        self.stats.trimmed_pages += removed as u64;
        removed
    }
    fn stats(&self) -> DynamicConstantsStats {
        let mut stats = self.stats;
        for page in &self.pages {
            let referenced =
                Arc::strong_count(&page.buffer) > 1 || Arc::weak_count(&page.buffer) > 0;
            let pending = !page.buffer.use_cell().is_reached(&self.completed);
            stats.referenced_pages += usize::from(referenced);
            stats.in_flight_pages += usize::from(pending);
            stats.idle_pages += usize::from(!referenced && !pending);
        }
        stats
    }
}
/// Pages append only into unused ranges. An in-flight current page is sealed;
/// reset requires unique CPU ownership and every real queue completion point.
/// No frame number or queue-idle approximation controls reuse.
pub struct DynamicConstants {
    device: Arc<Device>,
    device_alignment: u64,
    sharing: crate::resource::ResourceSharing,
    ring: PageRing<Buffer>,
}
impl DynamicConstants {
    pub fn new(device: Arc<Device>) -> Result<Self> {
        Self::with_config(device, Default::default())
    }
    pub fn with_config(device: Arc<Device>, config: DynamicConstantsConfig) -> Result<Self> {
        if device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        config.validate()?;
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
        if device_alignment >= 0x10000 || !device_alignment.is_power_of_two() {
            return Err(Error::invalid_usage("常量页对齐无效"));
        }
        let sharing = constant_page_sharing(&device.queue_plan.queues)?;
        let ring = PageRing::new(config, &device.allocator_stats)?;
        Ok(Self {
            device,
            device_alignment,
            sharing,
            ring,
        })
    }
    pub fn config(&self) -> DynamicConstantsConfig {
        self.ring.config
    }
    pub fn stats(&self) -> DynamicConstantsStats {
        self.ring.stats()
    }
    fn refresh(&mut self) -> Result<()> {
        self.ring.completed = self.device.poll()?;
        for page in &mut self.ring.pages {
            let cell = page.buffer.last_use.clone();
            let _submission = cell.lock_submission();
            if Arc::get_mut(&mut page.buffer).is_some() && cell.is_reached(&self.ring.completed) {
                self.device
                    .allocator_stats
                    .mark_cached(page.buffer.allocation_id(), &self.ring.completed);
            }
        }
        Ok(())
    }
    pub fn trim_idle(&mut self, keep_pages: usize) -> Result<usize> {
        self.refresh()?;
        Ok(self.ring.trim_idle(keep_pages, false))
    }
    pub fn allocate(&mut self, size: u64) -> Result<DynamicConstantsChunk> {
        self.allocate_aligned(size, None)
    }
    pub fn allocate_aligned(
        &mut self,
        size: u64,
        alignment: Option<u64>,
    ) -> Result<DynamicConstantsChunk> {
        allocation_offset(0, size, alignment, self.device_alignment)?;
        if self.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        let pressure =
            self.device.allocator_stats.budget_action() == crate::device::BudgetAction::EvictCache;
        if pressure
            || !self
                .ring
                .can_append(size, alignment, self.device_alignment)?
        {
            self.refresh()?;
        }
        let device = self.device.clone();
        let device_alignment = self.device_alignment;
        let sharing = self.sharing;
        let (buffer, offset) = self.ring.allocate(size, alignment, device_alignment, || {
            let desc = BufferDesc::new_cpu_to_gpu(
                DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES,
                constant_page_usage(device.capabilities.buffer_device_address),
            )
            .with_alignment(device_alignment as u16);
            match sharing {
                crate::resource::ResourceSharing::Exclusive => Buffer::new(device, &desc),
                crate::resource::ResourceSharing::Concurrent(families) => {
                    Buffer::new_concurrent(device, &desc, families.as_slice())
                }
            }
            .map(Buffer::into_dynamic_constants_page)
        })?;
        self.device
            .allocator_stats
            .mark_live(buffer.allocation_id(), &self.ring.completed);
        self.ring.trim_idle(
            if pressure {
                0
            } else {
                self.ring.config.max_idle_pages
            },
            true,
        );
        Ok(DynamicConstantsChunk {
            buffer,
            offset,
            size,
        })
    }
    pub fn allocate_with_data(&mut self, bytes: &[u8]) -> Result<DynamicConstantsChunk> {
        let chunk = self.allocate(bytes.len() as u64)?;
        chunk.write_bytes(bytes)?;
        Ok(chunk)
    }
    pub fn push(&mut self, size: u64) -> Result<DynamicConstantsChunk> {
        self.allocate(size)
    }
    pub fn push_slice(&mut self, bytes: &[u8]) -> Result<DynamicConstantsChunk> {
        self.allocate_with_data(bytes)
    }
    pub fn page_count(&self) -> usize {
        self.ring.pages.len()
    }
}
fn constant_page_usage(addresses_enabled: bool) -> vk::BufferUsageFlags {
    if addresses_enabled {
        DYNAMIC_CONSTANTS_USAGE
    } else {
        DYNAMIC_CONSTANTS_USAGE & !vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS
    }
}
fn constant_page_sharing(
    queues: &[crate::device::PhysicalQueue],
) -> Result<crate::resource::ResourceSharing> {
    let mut families = [0; crate::sync::MAX_QUEUES];
    let mut count = 0;
    for queue in queues {
        if !families[..count].contains(&queue.family) {
            if count == families.len() {
                return Err(Error::invalid_usage(
                    "too many dynamic constant queue families",
                ));
            }
            families[count] = queue.family;
            count += 1;
        }
    }
    if count > 1 {
        Ok(crate::resource::ResourceSharing::Concurrent(
            crate::resource::QueueFamilies::new(&families[..count])?,
        ))
    } else {
        Ok(crate::resource::ResourceSharing::Exclusive)
    }
}

/// None requests a fresh page; invalid requests never mutate the allocator.
fn allocation_offset(
    cursor: u64,
    size: u64,
    requested: Option<u64>,
    device_alignment: u64,
) -> Result<Option<u64>> {
    if size == 0 || size > DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES {
        return Err(Error::invalid_usage("常量区段大小必须在页范围内且非零"));
    }
    let requested = requested.unwrap_or(DEFAULT_ALIGNMENT);
    if !requested.is_power_of_two() || !device_alignment.is_power_of_two() {
        return Err(Error::invalid_usage("常量区段对齐必须是 2 的幂"));
    }
    let alignment = requested.max(DEFAULT_ALIGNMENT).max(device_alignment);
    let offset = align_up(cursor, alignment)?;
    Ok(offset
        .checked_add(size)
        .filter(|&end| end <= DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES)
        .map(|_| offset))
}
fn align_up(value: u64, alignment: u64) -> Result<u64> {
    value
        .checked_add(alignment - 1)
        .map(|v| v & !(alignment - 1))
        .ok_or(Error::invalid_usage("常量区段对齐溢出"))
}

#[cfg(test)]
mod tests {
    use super::*;
    struct CpuPage {
        cell: Arc<crate::resource::SyncCell>,
        bytes: u64,
    }
    impl ConstantPage for CpuPage {
        fn use_cell(&self) -> &Arc<crate::resource::SyncCell> {
            &self.cell
        }
        fn backing_bytes(&self) -> u64 {
            self.bytes
        }
    }
    fn cpu_page() -> Result<CpuPage> {
        Ok(CpuPage {
            cell: Arc::new(crate::resource::SyncCell::new()),
            bytes: DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES,
        })
    }
    fn config(pages: usize) -> DynamicConstantsConfig {
        DynamicConstantsConfig {
            max_pages: pages,
            max_backing_bytes: DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES * pages as u64,
            max_idle_pages: pages,
        }
    }
    #[test]
    fn production_ring_requires_every_queue_and_cpu_or_weak_owner_before_reset() {
        use crate::sync::{QueueId, SyncPoint};
        let budget = Arc::new(super::super::allocator::AllocatorStats::default());
        let mut ring = PageRing::new(config(1), &budget).unwrap();
        let (first, _) = ring
            .allocate(DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES, None, 16, cpu_page)
            .unwrap();
        let pointer = Arc::as_ptr(&first);
        let cell = first.cell.clone();
        let queue0 = QueueId::new(0).unwrap();
        let queue1 = QueueId::new(1).unwrap();
        cell.insert(SyncPoint {
            queue: queue0,
            value: 7,
        });
        cell.insert(SyncPoint {
            queue: queue1,
            value: 9,
        });
        ring.completed.raise(queue0, 7);
        ring.completed.raise(queue1, 9);
        assert!(
            matches!(ring.allocate(1, None, 16, cpu_page), Err(Error::Timeout)),
            "an unsubmitted CPU lease still owns the page"
        );
        let weak = Arc::downgrade(&first);
        drop(first);
        assert!(
            matches!(ring.allocate(1, None, 16, cpu_page), Err(Error::Timeout)),
            "Weak cannot resurrect an old allocation after reset"
        );
        drop(weak);
        ring.completed = Default::default();
        ring.completed.raise(queue0, 7);
        assert!(
            matches!(ring.allocate(1, None, 16, cpu_page), Err(Error::Timeout)),
            "the independent queue remains pending"
        );
        ring.completed.raise(queue1, 9);
        for _ in 0..1000 {
            let (page, offset) = ring
                .allocate(DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES, None, 16, cpu_page)
                .unwrap();
            assert_eq!(offset, 0);
            assert_eq!(Arc::as_ptr(&page), pointer);
            assert_eq!(
                page.cell.snapshot().get(queue1),
                9,
                "reuse does not erase real completion history"
            );
            drop(page);
        }
        assert_eq!(ring.stats().page_creations, 1);
        assert_eq!(ring.stats().page_reuses, 1000);
    }
    #[test]
    fn production_ring_caps_actual_bytes_and_preserves_oom_or_device_loss() {
        let budget = Arc::new(super::super::allocator::AllocatorStats::default());
        let mut ring = PageRing::<CpuPage>::new(config(1), &budget).unwrap();
        for error in [
            Error::OutOfHostMemory,
            Error::OutOfDeviceMemory,
            Error::DeviceLost,
        ] {
            let kind = error.kind();
            assert_eq!(
                ring.allocate(1, None, 16, || Err(error))
                    .err()
                    .unwrap()
                    .kind(),
                kind
            );
            assert_eq!(ring.stats().pages, 0);
        }
        assert!(matches!(
            ring.allocate(1, None, 16, || Ok(CpuPage {
                cell: Arc::new(crate::resource::SyncCell::new()),
                bytes: DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES + 1
            })),
            Err(Error::Timeout)
        ));
        assert_eq!(ring.stats().pages, 0);
        assert_eq!(ring.stats().backing_bytes, 0);
        assert!(ring.allocate(1, None, 16, cpu_page).is_ok());
    }
    #[test]
    fn production_ring_pressure_trims_only_idle_pages_and_keeps_metadata_accounted() {
        let budget = Arc::new(super::super::allocator::AllocatorStats::default());
        let mut ring = PageRing::new(config(2), &budget).unwrap();
        let metadata = ring.stats().metadata_bytes as u64;
        let (first, _) = ring
            .allocate(DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES, None, 16, cpu_page)
            .unwrap();
        let (second, _) = ring
            .allocate(DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES, None, 16, cpu_page)
            .unwrap();
        assert_eq!(ring.trim_idle(0, false), 0);
        let cell = first.cell.clone();
        cell.insert(crate::sync::SyncPoint {
            queue: crate::sync::QueueId::new(0).unwrap(),
            value: 1,
        });
        drop(first);
        drop(second);
        assert_eq!(ring.trim_idle(0, false), 1);
        assert_eq!(
            ring.stats().backing_bytes,
            DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES
        );
        ring.completed
            .raise(crate::sync::QueueId::new(0).unwrap(), 1);
        assert_eq!(ring.trim_idle(0, false), 1);
        assert_eq!(ring.stats().backing_bytes, 0);
        assert_eq!(budget.cpu_memory_usage().live, metadata);
        drop(ring);
        assert_eq!(budget.cpu_memory_usage().total(), 0);
    }

    #[test]
    fn constant_pages_use_real_concurrent_sharing_only_for_distinct_configured_families() {
        use crate::device::PhysicalQueue;
        let aliased = [
            PhysicalQueue {
                family: 2,
                index: 0,
            },
            PhysicalQueue {
                family: 2,
                index: 1,
            },
        ];
        assert_eq!(
            constant_page_sharing(&aliased).unwrap(),
            crate::resource::ResourceSharing::Exclusive
        );
        let distinct = [
            aliased[0],
            PhysicalQueue {
                family: 4,
                index: 0,
            },
            aliased[1],
        ];
        let sharing = constant_page_sharing(&distinct).unwrap();
        assert!(sharing.is_concurrent());
        assert!(sharing.allows_family(2));
        assert!(sharing.allows_family(4));
        assert!(!sharing.allows_family(3));
    }

    #[test]
    fn constant_pages_support_uniform_and_bindless_storage_bindings() {
        assert!(DYNAMIC_CONSTANTS_USAGE.contains(vk::BufferUsageFlags::UNIFORM_BUFFER));
        assert!(DYNAMIC_CONSTANTS_USAGE.contains(vk::BufferUsageFlags::STORAGE_BUFFER));
        assert_eq!(constant_page_usage(true), DYNAMIC_CONSTANTS_USAGE);
        let no_address = constant_page_usage(false);
        assert!(
            no_address.contains(
                vk::BufferUsageFlags::UNIFORM_BUFFER | vk::BufferUsageFlags::STORAGE_BUFFER
            )
        );
        assert!(!no_address.contains(vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS));
    }

    #[test]
    fn invalid_allocation_requests_fail_before_page_creation() {
        for (size, alignment) in [
            (0, None),
            (DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES + 1, None),
            (1, Some(0)),
            (1, Some(3)),
        ] {
            assert_eq!(
                allocation_offset(0, size, alignment, 256)
                    .unwrap_err()
                    .kind(),
                crate::error::ErrorKind::InvalidUsage,
            );
        }
        assert!(allocation_offset(u64::MAX, 1, None, 256).is_err());
    }

    #[test]
    fn page_boundary_and_device_alignment_determine_placement() {
        assert_eq!(allocation_offset(1, 16, None, 256).unwrap(), Some(256));
        assert_eq!(
            allocation_offset(0, DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES, None, 256).unwrap(),
            Some(0)
        );
        assert_eq!(
            allocation_offset(DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES - 16, 32, None, 16).unwrap(),
            None
        );
    }
}
