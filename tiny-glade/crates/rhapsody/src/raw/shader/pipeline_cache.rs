//! Driver pipeline cache with bounded per-key creation slots.
use super::*;

/// Vulkan driver cache with bounded object tables. Parallel creations use a
/// shared guard; cache-data extraction takes an exclusive guard so it cannot
/// overlap creation. Each key has a separate creation slot.
///
/// 每个键一个槽位：有界表只在查找/插入时短暂持锁，创建在槽位锁内进行——同键的并发调用者
/// 排队等待同一次创建（去重），不同键互不阻塞；创建失败不缓存，下次调用重试。
pub(super) type ComputePipelineKey = (u64, u64, Vec<u8>);
pub(super) type PipelineSlot = Arc<Mutex<PipelineSlotState>>;
pub(super) struct PipelineSlotState {
    pub(super) pipeline: Option<Arc<VulkanPipeline>>,
    _memory: CpuMemoryReservation,
}
pub(super) struct BudgetedKey<K> {
    pub(super) value: K,
    pub(super) memory: CpuMemoryReservation,
}
pub(super) struct PipelineCacheEntry<K> {
    pub(super) key: K,
    slot: PipelineSlot,
    _key_memory: CpuMemoryReservation,
}
pub struct PipelineCache {
    pub device: Arc<Device>,
    pub(super) handle: vk::PipelineCache,
    pub(super) lock: RwLock<()>,
    pub(super) capacity: usize,
    pub(super) compute: Mutex<TempList<PipelineCacheEntry<ComputePipelineKey>>>,
    pub(super) raster: Mutex<TempList<PipelineCacheEntry<Vec<u64>>>>,
}
/// Driver cache bytes retain their CPU budget until the last payload owner is
/// dropped, including when a caller keeps them after the cache itself is gone.
pub struct PipelineCacheData {
    bytes: Vec<u8>,
    _memory: crate::raw::allocator::CpuMemoryReservation,
}
impl PipelineCacheData {
    pub(crate) fn zeroed(
        budget: &Arc<crate::raw::allocator::AllocatorStats>,
        size: usize,
    ) -> Result<Self> {
        let mut memory =
            budget.reserve_cpu_bytes(size as u64, crate::device::BudgetCategory::Live)?;
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(size)
            .map_err(|_| crate::Error::OutOfHostMemory)?;
        memory.resize(bytes.capacity() as u64)?;
        bytes.resize(size, 0);
        Ok(Self {
            bytes,
            _memory: memory,
        })
    }
    pub(crate) fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.bytes
    }
}
impl std::ops::Deref for PipelineCacheData {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.bytes
    }
}
/// Full borrowed equality is checked before allocating a key. Slot metadata is
/// owned by the final Arc, including compilers that outlive table eviction.
pub(super) fn slot_for<K>(
    table: &Mutex<TempList<PipelineCacheEntry<K>>>,
    matches: impl Fn(&K) -> bool,
    create_key: impl FnOnce() -> Result<BudgetedKey<K>>,
    capacity: usize,
    budget: &Arc<AllocatorStats>,
) -> Result<PipelineSlot> {
    let mut slots = table.lock();
    if let Some(entry) = slots.as_slice().iter().find(|entry| matches(&entry.key)) {
        return Ok(entry.slot.clone());
    }
    let victim = if slots.as_slice().len() >= capacity {
        Some(
            slots
                .as_slice()
                .iter()
                .position(|entry| {
                    Arc::strong_count(&entry.slot) == 1 && entry.slot.try_lock().is_some()
                })
                .ok_or_else(|| {
                    crate::Error::exhausted("every pipeline cache slot is referenced")
                })?,
        )
    } else {
        let needed = slots.as_slice().len() + 1;
        slots.reserve(needed)?;
        None
    };
    // All fallible admission precedes eviction. Failure preserves every old key.
    let key = create_key()?;
    let memory = reserve_shared_cpu_metadata::<Mutex<PipelineSlotState>>(budget, 0)?;
    let slot = Arc::new(Mutex::new(PipelineSlotState {
        pipeline: None,
        _memory: memory,
    }));
    if let Some(victim) = victim {
        let mut index = 0;
        slots.retain(|_| {
            let keep = index != victim;
            index += 1;
            keep
        });
    }
    slots.add(PipelineCacheEntry {
        key: key.value,
        slot: slot.clone(),
        _key_memory: key.memory,
    })?;
    Ok(slot)
}
pub(super) fn budgeted_key_vec<T: Copy>(
    budget: &Arc<AllocatorStats>,
    values: &[T],
) -> Result<BudgetedKey<Vec<T>>> {
    let mut memory = budget.reserve_cpu_bytes(
        cpu_vec_bytes::<T>(values.len())?,
        crate::device::BudgetCategory::Live,
    )?;
    let mut key = Vec::new();
    key.try_reserve_exact(values.len())
        .map_err(|_| crate::Error::OutOfHostMemory)?;
    memory.resize(cpu_vec_bytes::<T>(key.capacity())?)?;
    key.extend_from_slice(values);
    Ok(BudgetedKey { value: key, memory })
}
pub(super) fn budgeted_raster_key(
    budget: &Arc<AllocatorStats>,
    desc: &RasterPipelineDesc,
) -> Result<BudgetedKey<Vec<u64>>> {
    let size = raster_pipeline_key_len(desc);
    let mut memory = budget.reserve_cpu_bytes(
        cpu_vec_bytes::<u64>(size)?,
        crate::device::BudgetCategory::Live,
    )?;
    let mut key = Vec::new();
    key.try_reserve_exact(size)
        .map_err(|_| crate::Error::OutOfHostMemory)?;
    memory.resize(cpu_vec_bytes::<u64>(key.capacity())?)?;
    pipeline_key::visit_raster_pipeline_key(desc, &mut |word| key.push(word));
    Ok(BudgetedKey { value: key, memory })
}
impl PipelineCache {
    pub fn new(device: Arc<Device>, initial_data: &[u8]) -> Result<Self> {
        Self::with_capacity(device, initial_data, 256)
    }
    /// Bounds each in-memory pipeline table, independently of driver cache data.
    pub fn with_capacity(
        device: Arc<Device>,
        initial_data: &[u8],
        capacity: usize,
    ) -> Result<Self> {
        if device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        if capacity == 0 || capacity > 65_536 {
            return Err(crate::Error::invalid_usage(
                "pipeline cache capacity must be in 1..=65536",
            ));
        }
        if !initial_data.is_empty() && !valid_pipeline_cache_data(&device, initial_data) {
            return Err(crate::Error::invalid_usage(
                "pipeline cache header does not match this device",
            ));
        }
        let compute = TempList::new(capacity, device.allocator_stats.clone())?;
        let raster = TempList::new(capacity, device.allocator_stats.clone())?;
        let info = vk::PipelineCacheCreateInfo::default().initial_data(initial_data);
        let handle = observe_vulkan(&device, unsafe {
            device.device.create_pipeline_cache(&info, None)
        })?;
        Ok(Self {
            device,
            handle,
            lock: RwLock::new(()),
            capacity,
            compute: Mutex::new(compute),
            raster: Mutex::new(raster),
        })
    }
    pub fn data(&self) -> Result<PipelineCacheData> {
        if self.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        let _lock = self.lock.write();
        let get = self.device.device.fp_v1_0().get_pipeline_cache_data;
        // Query and copy under the same exclusive cache lock. Native creation
        // holds a read guard, so a compiler cannot invalidate the queried size.
        for _ in 0..3 {
            let mut size = 0;
            let result = unsafe {
                get(
                    self.device.device.handle(),
                    self.handle,
                    &mut size,
                    std::ptr::null_mut(),
                )
            };
            observe_vulkan(&self.device, result.result())?;
            if size as u64 > MAX_PIPELINE_CACHE_BYTES {
                return Err(crate::Error::invalid_usage(
                    "pipeline cache data exceeds its byte limit",
                ));
            }
            let mut data = PipelineCacheData::zeroed(&self.device.allocator_stats, size)?;
            let mut written = size;
            let result = unsafe {
                get(
                    self.device.device.handle(),
                    self.handle,
                    &mut written,
                    data.bytes.as_mut_ptr().cast(),
                )
            };
            if result == vk::Result::INCOMPLETE {
                continue;
            }
            observe_vulkan(&self.device, result.result())?;
            if written > size {
                return Err(crate::Error::invalid_usage(
                    "pipeline cache returned an invalid data length",
                ));
            }
            data.bytes.truncate(written);
            return Ok(data);
        }
        Err(crate::Error::Timeout)
    }
    /// 管线缓存持有布局与 shader；显式失效后已有提交仍由外部 Arc 保持有效。
    pub fn invalidate(&self) {
        self.compute
            .lock()
            .retain(|entry| Arc::strong_count(&entry.slot) > 1 || entry.slot.try_lock().is_none());
        self.raster
            .lock()
            .retain(|entry| Arc::strong_count(&entry.slot) > 1 || entry.slot.try_lock().is_none());
    }
    pub fn cached_compute_count(&self) -> usize {
        self.compute
            .lock()
            .as_slice()
            .iter()
            .filter(|entry| entry.slot.lock().pipeline.is_some())
            .count()
    }
}
/// Upper bound for driver pipeline-cache blobs, in memory and on disk.
pub(crate) const MAX_PIPELINE_CACHE_BYTES: u64 = 64 << 20;
pub(crate) fn valid_pipeline_cache_data(device: &Device, bytes: &[u8]) -> bool {
    if bytes.len() < 32 || bytes.len() as u64 > MAX_PIPELINE_CACHE_BYTES {
        return false;
    }
    let word = |offset| {
        u32::from_le_bytes(
            bytes[offset..offset + 4]
                .try_into()
                .expect("checked Vulkan cache header"),
        )
    };
    let properties = device.properties();
    word(0) >= 32
        && word(0) as usize <= bytes.len()
        && word(4) == vk::PipelineCacheHeaderVersion::ONE.as_raw() as u32
        && word(8) == properties.vendor_id
        && word(12) == properties.device_id
        && bytes[16..32] == properties.pipeline_cache_uuid
}
impl Drop for PipelineCache {
    fn drop(&mut self) {
        unsafe { self.device.device.destroy_pipeline_cache(self.handle, None) }
    }
}
