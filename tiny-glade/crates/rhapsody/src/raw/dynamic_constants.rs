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
    /// Write without waiting. On non-coherent memory the written atoms are
    /// queued and flushed with every other dirty constants range in one
    /// `vkFlushMappedMemoryRanges` right before the next queue submit; coherent
    /// memory is never flushed.
    pub fn write_bytes(&self, bytes: &[u8]) -> Result<()> {
        if bytes.len() as u64 > self.size {
            return Err(Error::invalid_usage("数据超过常量区段范围"));
        }
        if self.buffer.is_dynamic_constants_page() {
            self.buffer.write_bytes_deferred_flush(self.offset, bytes)
        } else {
            self.buffer.write_bytes_unsynchronized(self.offset, bytes)
        }
    }
    /// Alias of [`Self::write_bytes`].
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
            .ok_or_else(|| Error::invalid_usage("设备地址溢出"))
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
    /// Returns a copy of the data rather than a GPU-lifetime-bound mapping.
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

/// Page size, which is also the largest single allocation.
pub const DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES: u64 = 0x40000;
/// Page buffer usage:
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
/// Every host write into a page is its own native range cell (each write bumps the revision,
/// so neighbours never coalesce) and a sequential page holds up to two cells per write.
/// `RangeStateConfig::max_cells` is 4096; stay below half of it so a page can never exhaust
/// its range table before it is full or reused.
const MAX_ALLOCATIONS_PER_PAGE: u32 = 1000;
struct Page<B> {
    buffer: Arc<B>,
    cursor: u64,
    /// Allocations handed out since the page was created or last reset.
    allocations: u32,
    /// The page's recorded GPU uses when it was created or last reset. Any later registered use
    /// (a submission referenced the page) seals it: a used page is never appended to again, only
    /// reset once its uses completed. Appending after a use would make chunk offsets depend on
    /// whether the GPU happened to finish first, and offsets are part of the render-graph plan key.
    use_mark: crate::sync::SyncSet,
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
        Ok(page.allocations < MAX_ALLOCATIONS_PER_PAGE
            && page.buffer.use_cell().snapshot() == page.use_mark
            && page.buffer.use_cell().is_reached(&self.completed)
            && allocation_offset(page.cursor, size, alignment, device_alignment)?.is_some())
    }
    /// Place the chunk in the current page when that page is still idle and has
    /// room: one check, no allocation of a page.
    fn try_append(
        &mut self,
        size: u64,
        alignment: Option<u64>,
        device_alignment: u64,
    ) -> Result<Option<(Arc<B>, u64)>> {
        if !self.can_append(size, alignment, device_alignment)? {
            return Ok(None);
        }
        let index = self.current.expect("appendable current page");
        let offset = allocation_offset(self.pages[index].cursor, size, alignment, device_alignment)?
            .expect("checked placement");
        self.pages[index].cursor = offset + size;
        self.pages[index].allocations += 1;
        self.stats.allocations += 1;
        Ok(Some((self.pages[index].buffer.clone(), offset)))
    }
    fn allocate(
        &mut self,
        size: u64,
        alignment: Option<u64>,
        device_alignment: u64,
        create: impl FnOnce() -> Result<B>,
    ) -> Result<(Arc<B>, u64)> {
        allocation_offset(0, size, alignment, device_alignment)?;
        if let Some(appended) = self.try_append(size, alignment, device_alignment)? {
            return Ok(appended);
        }
        for (index, page) in self.pages.iter_mut().enumerate() {
            let cell = page.buffer.use_cell().clone();
            // get_mut also rejects outstanding Weak pointers. A Weak upgrade
            // must not resurrect an old range while this page is being reset.
            // The reset page's contents are discarded: its native range table
            // (one cell per distinct host write, as every write bumps the
            // revision) returns to a single cell once all GPU uses completed,
            // otherwise it only grows across frames until `max_cells`.
            if Arc::get_mut(&mut page.buffer).is_some()
                && cell.discard_idle_range_state(&self.completed)
            {
                page.cursor = size;
                page.allocations = 1;
                page.use_mark = cell.snapshot();
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
        let use_mark = buffer.use_cell().snapshot();
        self.pages.push(Page {
            buffer: buffer.clone(),
            cursor: size,
            allocations: 1,
            use_mark,
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
                .core
                .instance
                .instance
                .get_physical_device_properties(device.core.physical)
        }
        .limits;
        let device_alignment = limits
            .min_uniform_buffer_offset_alignment
            .max(limits.min_storage_buffer_offset_alignment)
            .max(1);
        if device_alignment >= 0x10000 || !device_alignment.is_power_of_two() {
            return Err(Error::invalid_usage("常量页对齐无效"));
        }
        let sharing = constant_page_sharing(&device.queues.plan.queues)?;
        let ring = PageRing::new(config, &device.memory.stats)?;
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
                    .memory
                    .stats
                    .mark_cached(page.buffer.allocation_id(), &self.ring.completed);
            }
        }
        Ok(())
    }
    pub fn trim_idle(&mut self, keep_pages: usize) -> Result<usize> {
        self.refresh()?;
        let removed = self.ring.trim_idle(keep_pages, false);
        // `refresh` classifies every idle page as cached, the current one
        // included. Appending to it later skips `mark_live`, so restore its
        // live classification here.
        if let Some(index) = self.ring.current {
            let id = self.ring.pages[index].buffer.allocation_id();
            self.device.memory.stats.mark_live(id, &self.ring.completed);
        }
        Ok(removed)
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
            self.device.memory.stats.budget_action() == crate::device::BudgetAction::EvictCache;
        // Steady state: the page is idle and has room. Appending touches no
        // statistics lock and trims nothing (the page was marked live when it
        // became current, and trimming only reacts to pages going idle).
        if !pressure
            && let Some((buffer, offset)) =
                self.ring
                    .try_append(size, alignment, self.device_alignment)?
        {
            return Ok(DynamicConstantsChunk {
                buffer,
                offset,
                size,
            });
        }
        self.refresh()?;
        let device = self.device.clone();
        let device_alignment = self.device_alignment;
        let sharing = self.sharing;
        let (buffer, offset) = self.ring.allocate(size, alignment, device_alignment, || {
            let desc = BufferDesc::new_cpu_to_gpu(
                DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES,
                constant_page_usage(device.core.capabilities.buffer_device_address),
            )
            .with_alignment(device_alignment as u16);
            match sharing {
                crate::resource::ResourceSharing::Exclusive => Buffer::new(device, &desc),
                crate::resource::ResourceSharing::Concurrent(families) => {
                    Buffer::new_concurrent(device, &desc, families.as_slice())
                }
            }
            .and_then(Buffer::into_dynamic_constants_page)
        })?;
        self.device
            .memory
            .stats
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
    // Only requests larger than one page are rejected. Empty chunks retain a
    // real aligned page/address; they do not allocate a zero-sized VkBuffer.
    if size > DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES {
        return Err(Error::invalid_usage("常量区段大小不能超过页范围"));
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
        .ok_or_else(|| Error::invalid_usage("常量区段对齐溢出"))
}

/// When a host write to mapped memory must be flushed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum HostFlush {
    /// Coherent memory: never flushed.
    None,
    /// Flush the written atoms right after the copy.
    Now,
    /// Queue the written atoms on the device; flushed once before the next submit.
    Deferred,
}
pub(crate) fn host_write_flush(properties: vk::MemoryPropertyFlags, deferred: bool) -> HostFlush {
    if properties.contains(vk::MemoryPropertyFlags::HOST_COHERENT) {
        HostFlush::None
    } else if deferred {
        HostFlush::Deferred
    } else {
        HostFlush::Now
    }
}

/// Per-page merged ranges beyond which a page flushes its hull instead.
const MAX_DIRTY_RANGES_PER_PAGE: usize = 16;

struct DirtyPage {
    id: u64,
    memory: vk::DeviceMemory,
    /// Sorted, disjoint, non-adjacent `[start, end)` memory ranges; atom
    /// aligned by construction. `end == u64::MAX` means `VK_WHOLE_SIZE`.
    ranges: smallvec::SmallVec<[(u64, u64); 4]>,
}
impl DirtyPage {
    fn insert(&mut self, start: u64, end: u64) {
        let (mut start, mut end) = (start, end);
        self.ranges.retain(|&mut (s, e)| {
            if s <= end && start <= e {
                start = start.min(s);
                end = end.max(e);
                false
            } else {
                true
            }
        });
        let at = self.ranges.partition_point(|&(s, _)| s < start);
        self.ranges.insert(at, (start, end));
        if self.ranges.len() > MAX_DIRTY_RANGES_PER_PAGE {
            let hull = (self.ranges[0].0, self.ranges[self.ranges.len() - 1].1);
            self.ranges.clear();
            self.ranges.push(hull);
        }
    }
}

/// Device-wide queue of non-coherent dynamic-constants writes whose flush is
/// deferred to the next queue submit. Each page keeps its dirty atoms merged;
/// all pages are flushed with one `vkFlushMappedMemoryRanges` call.
#[derive(Default)]
pub(crate) struct HostFlushQueue {
    dirty: std::sync::atomic::AtomicBool,
    pages: parking_lot::Mutex<Vec<DirtyPage>>,
    flush_calls: std::sync::atomic::AtomicU64,
}
impl HostFlushQueue {
    /// Queue `[offset, offset + size)` of `memory` (atom-aligned, or
    /// `VK_WHOLE_SIZE`) written for allocation `id`.
    pub(crate) fn record(
        &self,
        id: u64,
        memory: vk::DeviceMemory,
        offset: u64,
        size: u64,
    ) -> Result<()> {
        if size == 0 {
            return Ok(());
        }
        let end = if size == vk::WHOLE_SIZE {
            u64::MAX
        } else {
            offset
                .checked_add(size)
                .ok_or_else(|| Error::invalid_usage("flush range overflows"))?
        };
        let mut pages = self.pages.lock();
        match pages.iter_mut().find(|page| page.id == id) {
            Some(page) => page.insert(offset, end),
            None => {
                pages.try_reserve(1).map_err(|_| Error::OutOfHostMemory)?;
                let mut ranges = smallvec::SmallVec::new();
                ranges.push((offset, end));
                pages.push(DirtyPage { id, memory, ranges });
            }
        }
        self.dirty.store(true, std::sync::atomic::Ordering::Release);
        Ok(())
    }
    /// Drop the pending ranges of a freed allocation.
    pub(crate) fn forget(&self, id: u64) {
        if !self.dirty.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }
        self.pages.lock().retain(|page| page.id != id);
    }
    /// Flush every pending range with one call of `flush`. Ranges stay
    /// pending when it fails.
    pub(crate) fn flush_with(
        &self,
        flush: impl FnOnce(&[vk::MappedMemoryRange<'_>]) -> Result<()>,
    ) -> Result<()> {
        if !self.dirty.swap(false, std::sync::atomic::Ordering::AcqRel) {
            return Ok(());
        }
        let mut pages = self.pages.lock();
        let count = pages.iter().map(|page| page.ranges.len()).sum();
        let mut ranges: smallvec::SmallVec<[vk::MappedMemoryRange<'_>; 8]> =
            smallvec::SmallVec::new();
        ranges.try_reserve(count).map_err(|_| {
            self.dirty.store(true, std::sync::atomic::Ordering::Release);
            Error::OutOfHostMemory
        })?;
        for page in pages.iter() {
            for &(start, end) in &page.ranges {
                ranges.push(
                    vk::MappedMemoryRange::default()
                        .memory(page.memory)
                        .offset(start)
                        .size(if end == u64::MAX {
                            vk::WHOLE_SIZE
                        } else {
                            end - start
                        }),
                );
            }
        }
        if ranges.is_empty() {
            return Ok(());
        }
        self.flush_calls
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        match flush(&ranges) {
            Ok(()) => {
                pages.clear();
                Ok(())
            }
            Err(error) => {
                self.dirty.store(true, std::sync::atomic::Ordering::Release);
                Err(error)
            }
        }
    }
    /// Number of `vkFlushMappedMemoryRanges` calls issued (statistics).
    pub(crate) fn flush_calls(&self) -> u64 {
        self.flush_calls.load(std::sync::atomic::Ordering::Relaxed)
    }
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
    fn production_ring_page_used_by_a_submission_is_sealed_even_after_completion() {
        use crate::sync::{QueueId, SyncPoint};
        let budget = Arc::new(super::super::allocator::AllocatorStats::default());
        let mut ring = PageRing::new(config(2), &budget).unwrap();
        let queue = QueueId::new(0).unwrap();
        let (first, first_offset) = ring.allocate(64, None, 16, cpu_page).unwrap();
        let (second, second_offset) = ring.allocate(64, None, 16, cpu_page).unwrap();
        assert_eq!(first_offset, 0);
        assert_eq!(second_offset, 64, "unused page keeps appending");
        assert!(Arc::ptr_eq(&first, &second));
        // A submission uses the page and the GPU finishes before the next allocation.
        first.cell.insert(SyncPoint { queue, value: 1 });
        ring.completed.raise(queue, 1);
        let cell = first.cell.clone();
        drop((first, second));
        // Without sealing, completion timing would decide between offset 128 here and a restart.
        let (third, third_offset) = ring.allocate(64, None, 16, cpu_page).unwrap();
        assert_eq!(third_offset, 0, "a used page restarts instead of appending");
        assert!(Arc::ptr_eq(&third.cell, &cell), "the idle used page is reset in place");
        assert_eq!(ring.stats().page_reuses, 1);
        // The reset page is appendable again until its next use.
        let (_, fourth_offset) = ring.allocate(64, None, 16, cpu_page).unwrap();
        assert_eq!(fourth_offset, 64);
        // Still in flight: a second page is created at offset 0 instead.
        cell.insert(SyncPoint { queue, value: 2 });
        let (fifth, fifth_offset) = ring.allocate(64, None, 16, cpu_page).unwrap();
        assert_eq!(fifth_offset, 0);
        assert!(!Arc::ptr_eq(&fifth.cell, &cell));
        assert_eq!(ring.stats().page_creations, 2);
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
    fn native_empty_chunks_keep_real_pages_and_the_aligned_cursor() {
        let budget = Arc::new(super::super::allocator::AllocatorStats::default());
        let mut ring = PageRing::new(config(1), &budget).unwrap();
        let (empty, offset) = ring.allocate(0, None, 16, cpu_page).unwrap();
        assert_eq!(offset, 0);
        assert_eq!(ring.pages[0].cursor, 0);
        let (payload, payload_offset) = ring.allocate(17, None, 16, cpu_page).unwrap();
        assert!(Arc::ptr_eq(&empty, &payload));
        assert_eq!(payload_offset, 0);
        let (aligned_empty, aligned_offset) = ring.allocate(0, Some(64), 16, cpu_page).unwrap();
        assert!(Arc::ptr_eq(&empty, &aligned_empty));
        assert_eq!(aligned_offset, 64);
        assert_eq!(ring.pages[0].cursor, 64);
        let (next, next_offset) = ring.allocate(1, None, 16, cpu_page).unwrap();
        assert!(Arc::ptr_eq(&empty, &next));
        assert_eq!(next_offset, 64);
        assert_eq!(ring.stats().page_creations, 1);
        // A retained empty chunk is still a genuine CPU owner of this page.
        drop(payload);
        drop(aligned_empty);
        drop(next);
        assert!(matches!(
            ring.allocate(DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES, None, 16, cpu_page),
            Err(Error::Timeout)
        ));
        drop(empty);
        assert!(
            ring.allocate(DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES, None, 16, cpu_page)
                .is_ok()
        );
        assert_eq!(
            allocation_offset(DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES, 0, None, 16).unwrap(),
            Some(DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES)
        );
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

    #[test]
    fn coherent_memory_is_never_flushed_and_noncoherent_constants_defer() {
        let noncoherent = vk::MemoryPropertyFlags::HOST_VISIBLE;
        let coherent = noncoherent | vk::MemoryPropertyFlags::HOST_COHERENT;
        assert_eq!(host_write_flush(coherent, true), HostFlush::None);
        assert_eq!(host_write_flush(coherent, false), HostFlush::None);
        assert_eq!(host_write_flush(noncoherent, true), HostFlush::Deferred);
        assert_eq!(host_write_flush(noncoherent, false), HostFlush::Now);
    }

    #[test]
    fn many_noncoherent_writes_flush_once_per_page_with_merged_atoms() {
        use ash::vk::Handle;
        let queue = HostFlushQueue::default();
        let atom = 64u64;
        let (page_a, page_b) = (
            vk::DeviceMemory::from_raw(0x10),
            vk::DeviceMemory::from_raw(0x20),
        );
        // 1000 consecutive 48-byte constants written into page A (atom
        // aligned by the writer), and a few scattered writes into page B.
        for index in 0..1000u64 {
            let start = index * 48 / atom * atom;
            let end = ((index + 1) * 48).div_ceil(atom) * atom;
            queue.record(1, page_a, start, end - start).unwrap();
        }
        queue.record(2, page_b, 0, atom).unwrap();
        queue.record(2, page_b, 4 * atom, atom).unwrap();
        queue.record(2, page_b, atom, atom).unwrap();
        let mut calls = 0;
        let mut seen = Vec::new();
        queue
            .flush_with(|ranges| {
                calls += 1;
                seen.extend(ranges.iter().map(|r| (r.memory, r.offset, r.size)));
                Ok(())
            })
            .unwrap();
        assert_eq!(calls, 1, "one flush call covers every dirty page");
        assert_eq!(
            seen,
            [
                (page_a, 0, 48_000u64.div_ceil(atom) * atom),
                (page_b, 0, 2 * atom),
                (page_b, 4 * atom, atom),
            ]
        );
        // Nothing dirty: the next submit performs no flush call.
        queue
            .flush_with(|_| panic!("clean queue must not flush"))
            .unwrap();
        assert_eq!(queue.flush_calls(), 1);
    }

    #[test]
    fn failed_flush_keeps_ranges_and_freed_pages_are_forgotten() {
        use ash::vk::Handle;
        let queue = HostFlushQueue::default();
        let memory = vk::DeviceMemory::from_raw(0x30);
        queue.record(7, memory, 0, 256).unwrap();
        queue.record(8, memory, 512, vk::WHOLE_SIZE).unwrap();
        assert!(queue.flush_with(|_| Err(Error::OutOfHostMemory)).is_err());
        queue.forget(8);
        let mut seen = Vec::new();
        queue
            .flush_with(|ranges| {
                seen.extend(ranges.iter().map(|r| (r.offset, r.size)));
                Ok(())
            })
            .unwrap();
        assert_eq!(seen, [(0, 256)]);
    }

    #[test]
    fn scattered_writes_collapse_to_a_bounded_hull() {
        let mut page = DirtyPage {
            id: 1,
            memory: vk::DeviceMemory::null(),
            ranges: smallvec::SmallVec::new(),
        };
        for index in 0..(MAX_DIRTY_RANGES_PER_PAGE as u64 + 1) {
            page.insert(index * 128, index * 128 + 64);
        }
        assert_eq!(
            page.ranges.as_slice(),
            [(0, MAX_DIRTY_RANGES_PER_PAGE as u64 * 128 + 64)]
        );
    }
}
