//! Bounded dynamic constants pages with CPU leases and precise GPU completion.
//! 设备实际支持并启用 BDA 时提供真实地址；其余设备明确返回 Unsupported。
use crate::{Error, Result};
use crate::resource::buffer::{Buffer, BufferDesc};
use crate::device::Device;
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
    fn use_cell(&self) -> &Arc<crate::tracking::SyncCell>;
    fn backing_bytes(&self) -> u64;
}
impl ConstantPage for Buffer {
    fn use_cell(&self) -> &Arc<crate::tracking::SyncCell> {
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
    bank: usize,
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
    _metadata: crate::accounting::CpuMemoryReservation,
    config: DynamicConstantsConfig,
    current: Option<usize>,
    frame_bank: usize,
    completed: crate::sync::CompletedValues,
    stats: DynamicConstantsStats,
}
impl<B: ConstantPage> PageRing<B> {
    fn new(
        config: DynamicConstantsConfig,
        budget: &Arc<crate::accounting::AllocatorStats>,
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
            budget.reserve_cpu_bytes(metadata_bytes as u64, crate::accounting::BudgetCategory::Live)?;
        Ok(Self {
            pages,
            _metadata: metadata,
            config,
            current: None,
            frame_bank: 0,
            completed: Default::default(),
            stats: DynamicConstantsStats {
                metadata_bytes,
                ..Default::default()
            },
        })
    }
    fn begin_frame(&mut self, banks: usize) {
        self.frame_bank = (self.frame_bank + 1) % banks;
        self.current = None;
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
            if page.bank != self.frame_bank { continue; }
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
            bank: self.frame_bank,
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
    sharing: crate::tracking::ResourceSharing,
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
    /// Select this display frame's arena once before allocating its constants.
    /// Completed GPU uses and exclusive CPU ownership still govern page resets.
    pub fn begin_frame(&mut self) {
        self.ring.begin_frame(self.device.frames_in_flight() as usize);
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
            self.device.memory.stats.budget_action() == crate::accounting::BudgetAction::EvictCache;
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
                crate::tracking::ResourceSharing::Exclusive => Buffer::new(device, &desc),
                crate::tracking::ResourceSharing::Concurrent(families) => {
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
) -> Result<crate::tracking::ResourceSharing> {
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
        Ok(crate::tracking::ResourceSharing::Concurrent(
            crate::tracking::QueueFamilies::new(&families[..count])?,
        ))
    } else {
        Ok(crate::tracking::ResourceSharing::Exclusive)
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
    let offset = crate::collections::align::align_up(cursor, alignment)
        .ok_or_else(|| Error::invalid_usage("常量区段对齐溢出"))?;
    Ok(offset
        .checked_add(size)
        .filter(|&end| end <= DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES)
        .map(|_| offset))
}
#[cfg(test)]
mod tests;
