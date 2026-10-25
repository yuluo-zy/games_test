//! A renderer-owned update-after-bind descriptor table. A lease is immutable:
//! changing a resource allocates a different slot. Its slot returns to the free
//! list only after every registered GPU use completes. See the Vulkan
//! [descriptor binding rules](https://docs.vulkan.org/spec/latest/chapters/descriptorsets.html).
use crate::rg::profiling::{RecordCount, RecordPhase, RecordSpan, record_count};
use crate::{
    Error, Result,
    device::{Device, FeatureSet},
    raw::{
        buffer::Buffer,
        command_buffer::VulkanCommandBuffer,
        descriptor_set::{
            LayoutBinding, LayoutPlan, VulkanDescriptorPool, VulkanDescriptorSet,
            VulkanDescriptorSetLayout,
        },
        sampler::Sampler,
        shader::VulkanPipeline,
        texture::TextureView,
    },
    resource::GpuUse,
};
use ash::vk;
use parking_lot::Mutex;
use std::{collections::VecDeque, sync::Arc};
#[path = "bindless_parameters.rs"]
mod parameters;
pub use parameters::{
    BindlessParameters, BindlessResource, BindlessShaderAbi, BindlessShaderPackage,
    PreparedBindlessParameters,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum DescriptorKind {
    SampledImage = 0,
    StorageImage = 1,
    StorageBuffer = 2,
    Sampler = 3,
}
impl DescriptorKind {
    const ALL: [Self; 4] = [
        Self::SampledImage,
        Self::StorageImage,
        Self::StorageBuffer,
        Self::Sampler,
    ];
    fn ty(self) -> vk::DescriptorType {
        match self {
            Self::SampledImage => vk::DescriptorType::SAMPLED_IMAGE,
            Self::StorageImage => vk::DescriptorType::STORAGE_IMAGE,
            Self::StorageBuffer => vk::DescriptorType::STORAGE_BUFFER,
            Self::Sampler => vk::DescriptorType::SAMPLER,
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct BindlessConfig {
    pub sampled_images: u32,
    pub storage_images: u32,
    pub storage_buffers: u32,
    pub samplers: u32,
}
impl Default for BindlessConfig {
    fn default() -> Self {
        Self {
            sampled_images: 256,
            storage_images: 256,
            storage_buffers: 256,
            samplers: 64,
        }
    }
}
impl BindlessConfig {
    fn counts(self) -> [u32; 4] {
        [
            self.sampled_images,
            self.storage_images,
            self.storage_buffers,
            self.samplers,
        ]
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BindlessIndex {
    pub kind: DescriptorKind,
    pub index: u32,
    pub generation: u64,
}
enum SlotState {
    Free,
    Live,
    Exhausted,
}
struct Slot {
    generation: u64,
    state: SlotState,
}
/// A slot stays live while any CPU or submitted-command Arc holds its lease.
/// The last lease drop releases it; completion belongs to the command owner.
struct Slots {
    live: usize,
    peak_live: usize,
    table_live: [usize; 4],
    table_exhausted: [usize; 4],
    table_peak_live: [usize; 4],
    entries: [Vec<Slot>; 4],
    free: [VecDeque<u32>; 4],
}
impl Slots {
    fn new(counts: [u32; 4]) -> Result<Self> {
        let mut entries = std::array::from_fn(|_| Vec::new());
        let mut free = std::array::from_fn(|_| VecDeque::new());
        for table in 0..4 {
            let count = counts[table] as usize;
            entries[table]
                .try_reserve_exact(count)
                .map_err(|_| Error::OutOfHostMemory)?;
            free[table]
                .try_reserve_exact(count)
                .map_err(|_| Error::OutOfHostMemory)?;
            entries[table].resize_with(count, || Slot {
                generation: 0,
                state: SlotState::Free,
            });
            free[table].extend(0..counts[table]);
        }
        Ok(Self {
            live: 0,
            peak_live: 0,
            table_live: [0; 4],
            table_exhausted: [0; 4],
            table_peak_live: [0; 4],
            entries,
            free,
        })
    }
    fn statistics(&self) -> BindlessStats {
        BindlessStats {
            peak_live: self.peak_live,
            capacity: self.entries.iter().map(Vec::len).sum(),
            live: self.live,
            free: self.free.iter().map(VecDeque::len).sum(),
            tables: DescriptorKind::ALL.map(|kind| self.table_stats(kind)),
        }
    }
    fn host_bytes(&self) -> Result<u64> {
        Ok(crate::raw::allocator::shared_cpu_bytes::<BindlessHeap>()?
            + self.entries.iter().map(Vec::len).sum::<usize>() as u64
                * crate::raw::allocator::shared_cpu_bytes::<BindlessLease>()?
            + (self
                .entries
                .iter()
                .map(|v| v.capacity() * std::mem::size_of::<Slot>())
                .sum::<usize>()
                + self
                    .free
                    .iter()
                    .map(|v| v.capacity() * std::mem::size_of::<u32>())
                    .sum::<usize>()
) as u64)
    }
    fn allocate(&mut self, kind: DescriptorKind) -> Result<BindlessIndex> {
        let table = kind as usize;
        while let Some(index) = self.free[table].pop_front() {
            let slot = &mut self.entries[table][index as usize];
            let Some(generation) = slot.generation.checked_add(1) else {
                slot.state = SlotState::Exhausted;
                self.table_exhausted[table] += 1;
                continue;
            };
            slot.generation = generation;
            slot.state = SlotState::Live;
            self.live += 1;
            self.peak_live = self.peak_live.max(self.live);
            self.table_live[table] += 1;
            self.table_peak_live[table] = self.table_peak_live[table].max(self.table_live[table]);
            return Ok(BindlessIndex {
                kind,
                index,
                generation,
            });
        }
        Err(Error::exhausted(format!(
            "bindless {kind:?} descriptor table is full (live={} free={} exhausted={} capacity={}; heap_live={})",
            self.table_live[table], self.free[table].len(), self.table_exhausted[table],
            self.entries[table].len(), self.live,
        )))
    }
    fn valid(&self, index: BindlessIndex) -> bool {
        self.entries[index.kind as usize]
            .get(index.index as usize)
            .is_some_and(|slot| {
                slot.generation == index.generation && matches!(slot.state, SlotState::Live)
            })
    }
    fn release(&mut self, index: BindlessIndex) {
        debug_assert!(self.valid(index));
        let table = index.kind as usize;
        self.live -= 1;
        self.table_live[table] -= 1;
        self.entries[table][index.index as usize].state = SlotState::Free;
        self.free[table].push_back(index.index);
    }
    fn table_stats(&self, kind: DescriptorKind) -> BindlessTableStats {
        let table = kind as usize;
        BindlessTableStats {
            capacity: self.entries[table].len(),
            live: self.table_live[table],
            free: self.free[table].len(),
            exhausted: self.table_exhausted[table],
            peak_live: self.table_peak_live[table],
        }
    }

}
pub struct BindlessHeap {
    pub layout: Arc<VulkanDescriptorSetLayout>,
    set: Arc<VulkanDescriptorSet>,
    device: Arc<Device>,
    slots: Mutex<Slots>,
    /// Reuse descriptors while a prepared group or submitted command owns the
    /// lease. The index cache must not keep its own descriptors/resources alive.
    resident: Mutex<Option<ResidentLeases>>,
    // Covers slot storage and one live lease payload per slot. Leases retain
    // this page, so the capacity charge survives every CPU/GPU lease owner.
    // Declared after slots: release credit only after the actual CPU buffers.
    _slot_memory: crate::raw::allocator::CpuMemoryReservation,
}
#[derive(Default)]
struct ResidentLeases {
    leases: std::collections::HashMap<parameters::ResourceKey, std::sync::Weak<BindlessLease>>,
    inserts_since_trim: usize,
    memory: Option<crate::raw::allocator::CpuMemoryReservation>,
}
/// Expired weak keys are swept periodically; a fixed limit also bounds live
/// keys when unusually many groups are retained by a caller.
const RESIDENT_TRIM_INTERVAL: usize = 512;
const MAX_RESIDENT_KEYS: usize = 4096;
impl ResidentLeases {
    fn trim(&mut self) {
        self.inserts_since_trim = 0;
        self.leases.retain(|_, lease| lease.strong_count() != 0);
    }
}
enum Owner {
    Buffer {
        buffer: Arc<Buffer>,
        offset: u64,
        size: u64,
        parameters: bool,
    },
    View {
        view: Arc<TextureView>,
        layout: vk::ImageLayout,
    },
    Sampler(Arc<Sampler>),
}
impl GpuUse for Owner {
    fn visit_uses(&self, visit: &mut crate::resource::UseVisitor<'_>) -> Result<()> {
        match self {
            Self::Buffer { buffer, .. } => buffer.visit_uses(visit),
            Self::View { view, .. } => view.visit_uses(visit),
            Self::Sampler(owner) => owner.visit_uses(visit),
        }
    }
}
/// Keep this Arc in every command that can dynamically access its index.
/// Copying the numeric shader index does not extend the lease lifetime.
pub struct BindlessLease {
    // Drop the resource before returning its descriptor slot to the page.
    owner: Owner,
    slot: LeaseSlot,
}
struct LeaseSlot {
    heap: Arc<BindlessHeap>,
    index: BindlessIndex,
}
impl BindlessLease {
    pub fn index(&self) -> BindlessIndex {
        self.slot.index
    }
}
impl GpuUse for BindlessLease {
    fn visit_uses(&self, visit: &mut crate::resource::UseVisitor<'_>) -> Result<()> {
        self.owner.visit_uses(visit)?;
        self.slot.heap.visit_uses(visit)
    }
    fn held_until_completion(&self) -> bool {
        true
    }
}
/// Every command that uses a lease retains its Arc until the command completes
/// (`Doomed::Owners`), so the last drop proves the GPU no longer reads the slot.
impl Drop for LeaseSlot {
    fn drop(&mut self) {
        self.heap.slots.lock().release(self.index);
    }
}
impl GpuUse for BindlessHeap {
    fn visit_uses(&self, visit: &mut crate::resource::UseVisitor<'_>) -> Result<()> {
        self.set.visit_uses(visit)
    }
}
/// 单类描述符的寿命账目；统计只读取计数器，不扫描整张描述符表。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BindlessTableStats {
    pub capacity: usize,
    pub live: usize,
    pub free: usize,
    /// generation 已溢出的槽永久不可复用，不能计入可分配容量。
    pub exhausted: usize,
    pub peak_live: usize,
}
#[derive(Clone, Copy, Debug, Default)]
pub struct BindlessStats {
    pub peak_live: usize,
    pub capacity: usize,
    pub live: usize,
    pub free: usize,
    /// 顺序与 DescriptorKind 一致；优先通过 table() 按类别读取。
    pub tables: [BindlessTableStats; 4],
}
impl BindlessStats {
    pub fn table(&self, kind: DescriptorKind) -> BindlessTableStats {
        self.tables[kind as usize]
    }
}
/// Current occupancy sampled independently from the pool's physical heaps.
/// No global historical peak or simultaneous cross-heap snapshot is maintained.
#[derive(Clone, Copy, Debug, Default)]
pub struct BindlessOccupancy {
    pub capacity: usize,
    pub live: usize,
    pub free: usize,
    pub tables: [BindlessTableOccupancy; 4],
}
#[derive(Clone, Copy, Debug, Default)]
pub struct BindlessTableOccupancy {
    pub capacity: usize,
    pub live: usize,
    pub free: usize,
    pub exhausted: usize,
}
impl BindlessOccupancy {
    pub fn table(&self, kind: DescriptorKind) -> BindlessTableOccupancy { self.tables[kind as usize] }
    fn include(&mut self, page: BindlessStats) {
        self.capacity += page.capacity;
        self.live += page.live;
        self.free += page.free;
        for (sum, value) in self.tables.iter_mut().zip(page.tables) {
            sum.capacity += value.capacity;
            sum.live += value.live;
            sum.free += value.free;
            sum.exhausted += value.exhausted;
        }
    }
}
impl BindlessHeap {
    pub fn new(device: Arc<Device>, config: BindlessConfig) -> Result<Arc<Self>> {
        if device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        let counts = config.counts();
        if counts.contains(&0)
            || counts
                .iter()
                .try_fold(0u32, |sum, count| sum.checked_add(*count))
                .is_none_or(|total| total > 1 << 20)
        {
            return Err(Error::invalid_usage(
                "bindless table counts must be nonzero and bounded",
            ));
        }
        let required = crate::device::policy::BINDLESS_REQUIRED;
        device.caps().require_all_enabled(
            required
                .into_iter()
                .fold(FeatureSet::EMPTY, |set, feature| set.with(feature)),
        )?;
        let mut indexing = vk::PhysicalDeviceDescriptorIndexingProperties::default();
        let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut indexing);
        unsafe {
            device
                .core
                .instance
                .instance
                .get_physical_device_properties2(device.core.physical, &mut properties);
        }
        let limits = [
            indexing
                .max_descriptor_set_update_after_bind_sampled_images
                .min(indexing.max_per_stage_descriptor_update_after_bind_sampled_images),
            indexing
                .max_descriptor_set_update_after_bind_storage_images
                .min(indexing.max_per_stage_descriptor_update_after_bind_storage_images),
            indexing
                .max_descriptor_set_update_after_bind_storage_buffers
                .min(indexing.max_per_stage_descriptor_update_after_bind_storage_buffers),
            indexing
                .max_descriptor_set_update_after_bind_samplers
                .min(indexing.max_per_stage_descriptor_update_after_bind_samplers),
        ];
        let total: u32 = counts.iter().sum();
        if counts
            .iter()
            .zip(limits)
            .any(|(count, limit)| *count > limit)
            || total > indexing.max_per_stage_update_after_bind_resources
            || total > indexing.max_update_after_bind_descriptors_in_all_pools
        {
            return Err(Error::unsupported(
                "bindless table exceeds enabled descriptor limits",
            ));
        }
        let host_bytes = crate::raw::allocator::shared_cpu_bytes::<Self>()?
            + u64::from(total) * ((std::mem::size_of::<Slot>() + std::mem::size_of::<u32>()) as u64
                + crate::raw::allocator::shared_cpu_bytes::<BindlessLease>()?);
        let mut slot_memory = device
            .memory
            .stats
            .reserve_cpu_bytes(host_bytes, crate::device::BudgetCategory::Live)?;
        let slots = Slots::new(counts)?;
        slot_memory.resize(slots.host_bytes()?)?;
        let flags = vk::DescriptorBindingFlags::UPDATE_AFTER_BIND
            | vk::DescriptorBindingFlags::PARTIALLY_BOUND
            | vk::DescriptorBindingFlags::UPDATE_UNUSED_WHILE_PENDING;
        let plan = LayoutPlan {
            flags: vk::DescriptorSetLayoutCreateFlags::UPDATE_AFTER_BIND_POOL.as_raw(),
            binding_flags: vec![flags.as_raw(); 4],
            bindings: DescriptorKind::ALL
                .into_iter()
                .map(|kind| LayoutBinding {
                    binding: kind as u32,
                    descriptor_type: kind.ty().as_raw(),
                    descriptor_count: counts[kind as usize],
                    stage_flags: vk::ShaderStageFlags::ALL.as_raw(),
                    immutable_sampler: None,
                })
                .collect(),
        };
        let layout = VulkanDescriptorSetLayout::new(device.clone(), &plan)?;
        let pool = VulkanDescriptorPool::new(device.clone(), &plan, 1)?;
        let set = pool.allocate_shared(layout.clone())?;
        Ok(Arc::new(Self {
            layout,
            set,
            device,
            slots: Mutex::new(slots),
            resident: Mutex::new(None),
            _slot_memory: slot_memory,
        }))
    }
    /// 建立共享同一真实布局的新物理页；槽、描述符池和集合各自独立。
    /// 本入口只提供页原语，完整绑定组准入与页选择仍由调用方完成。
    pub fn new_page(&self) -> Result<Arc<Self>> {
        if self.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        let counts: [u32; 4] = {
            let slots = self.slots.lock();
            // 各表容量只在 new 中建立且受 1<<20 总数限制，之后不会调整。
            std::array::from_fn(|kind| slots.entries[kind].len() as u32)
        };
        let plan = self.layout.plan();
        let flags = vk::DescriptorBindingFlags::UPDATE_AFTER_BIND
            | vk::DescriptorBindingFlags::PARTIALLY_BOUND
            | vk::DescriptorBindingFlags::UPDATE_UNUSED_WHILE_PENDING;
        // layout 是公开 owner，仍验证其实际设备和结构，不能依赖外部未替换它。
        if !Arc::ptr_eq(&self.device, self.layout.device())
            || plan.flags != vk::DescriptorSetLayoutCreateFlags::UPDATE_AFTER_BIND_POOL.as_raw()
            || plan.bindings.len() != 4
            || plan.binding_flags.len() != 4
            || plan
                .binding_flags
                .iter()
                .any(|value| *value != flags.as_raw())
            || plan
                .bindings
                .iter()
                .zip(DescriptorKind::ALL)
                .any(|(binding, kind)| {
                    binding.binding != kind as u32
                        || binding.descriptor_type != kind.ty().as_raw()
                        || binding.descriptor_count != counts[kind as usize]
                        || binding.stage_flags != vk::ShaderStageFlags::ALL.as_raw()
                        || binding.immutable_sampler.is_some()
                })
        {
            return Err(Error::invalid_usage(
                "bindless page layout differs from its tables",
            ));
        }
        let total: u32 = counts.iter().sum();
        // 与 new 相同，先准入并分配实际槽存储，再进入不可回滚的 native 创建。
        let host_bytes = crate::raw::allocator::shared_cpu_bytes::<Self>()?
            + u64::from(total) * ((std::mem::size_of::<Slot>() + std::mem::size_of::<u32>()) as u64
                + crate::raw::allocator::shared_cpu_bytes::<BindlessLease>()?);
        let mut slot_memory = self
            .device
            .memory
            .stats
            .reserve_cpu_bytes(host_bytes, crate::device::BudgetCategory::Live)?;
        let slots = Slots::new(counts)?;
        slot_memory.resize(slots.host_bytes()?)?;
        // 复用已实际创建成功的布局，不重复查询，也不复制模板页的占用或 generation。
        let pool = VulkanDescriptorPool::new(self.device.clone(), plan, 1)?;
        let set = pool.allocate_shared(self.layout.clone())?;
        Ok(Arc::new(Self {
            layout: self.layout.clone(),
            set,
            device: self.device.clone(),
            slots: Mutex::new(slots),
            resident: Mutex::new(None),
            _slot_memory: slot_memory,
        }))
    }
    /// Collect completed command owners. Their last lease drops release slots;
    /// descriptors still referenced by pending commands remain live.
    pub fn collect(&self) -> Result<usize> {
        let free = |slots: &Slots| slots.free.iter().map(VecDeque::len).sum::<usize>();
        let before = free(&self.slots.lock());
        self.device.try_collect_garbage()?;
        Ok(free(&self.slots.lock()).saturating_sub(before))
    }
    pub fn stats(&self) -> BindlessStats {
        self.slots.lock().statistics()
    }
    /// The resident lease for `key`, created by `make` on first use.
    pub(crate) fn resident_lease(
        &self,
        key: parameters::ResourceKey,
        make: impl FnOnce() -> Result<Arc<BindlessLease>>,
    ) -> Result<Arc<BindlessLease>> {
        let mut resident = self.resident.lock();
        let Some(cache) = resident.as_mut() else {
            drop(resident);
            return make();
        };
        if let Some(lease) = cache.leases.get(&key).and_then(std::sync::Weak::upgrade) {
            record_count(RecordCount::LocalCacheHits, 1);
            return Ok(lease);
        }
        if cache.inserts_since_trim >= RESIDENT_TRIM_INTERVAL
            || cache.leases.len() >= MAX_RESIDENT_KEYS
        {
            cache.trim();
        }
        drop(resident);
        let lease = make()?;
        if let Some(cache) = self.resident.lock().as_mut() {
            if let Some(existing) = cache.leases.get(&key).and_then(std::sync::Weak::upgrade) {
                return Ok(existing);
            }
            if cache.leases.len() >= MAX_RESIDENT_KEYS {
                return Ok(lease);
            }
            if cache.leases.len() == cache.leases.capacity() {
                // Account a conservative bucket/control-byte allocation bound
                // before growing the optional index. A rejected cache admission
                // still returns the valid uncached descriptor.
                let capacity = cache.leases.capacity().saturating_mul(2).max(3);
                let bytes = (capacity + 1).next_power_of_two()
                    * (std::mem::size_of::<(
                        parameters::ResourceKey,
                        std::sync::Weak<BindlessLease>,
                    )>() + 1)
                    + 16;
                let Ok(memory) = self
                    .device
                    .memory
                    .stats
                    .reserve_cpu_bytes(bytes as u64, crate::device::BudgetCategory::Cache)
                else {
                    return Ok(lease);
                };
                if cache.leases.try_reserve(1).is_err() {
                    return Ok(lease);
                }
                cache.memory = Some(memory);
            }
            cache.leases.insert(key, Arc::downgrade(&lease));
            cache.inserts_since_trim += 1;
        }
        Ok(lease)
    }
    fn set_residency(&self, enabled: bool) {
        let old = std::mem::replace(
            &mut *self.resident.lock(),
            enabled.then(ResidentLeases::default),
        );
        drop(old);
    }
    /// Remove expired index entries; GPU owners govern the actual lease lifetime.
    fn trim_resident(&self) {
        if let Some(cache) = self.resident.lock().as_mut() {
            cache.trim();
        }
    }
    fn reserve(self: &Arc<Self>, kind: DescriptorKind, owner: Owner) -> Result<Arc<BindlessLease>> {
        let _trace = RecordSpan::new(RecordPhase::DescriptorReserve);
        record_count(RecordCount::DescriptorReserveCalls, 1);

        let allocation = self.slots.lock().allocate(kind);
        let index = match allocation {
            Ok(index) => index,
            Err(Error::Exhausted(_)) => {
                self.trim_resident();
                self.collect()?;
                self.slots.lock().allocate(kind)?
            }
            Err(error) => return Err(error),
        };
        Ok(Arc::new(BindlessLease {
            owner,
            slot: LeaseSlot { heap: self.clone(), index },
        }))
    }
    pub fn storage_buffer(
        self: &Arc<Self>,
        buffer: Arc<Buffer>,
        offset: u64,
        size: u64,
    ) -> Result<Arc<BindlessLease>> {
        self.buffer_binding(buffer, offset, size, false)
    }
    /// Only the immutable block allocated by BindlessParameters::finish uses
    /// this role. It is framework upload data rather than a graph resource.
    pub(crate) fn parameter_buffer(
        self: &Arc<Self>,
        chunk: &crate::raw::dynamic_constants::DynamicConstantsChunk,
    ) -> Result<Arc<BindlessLease>> {
        if !chunk.buffer.is_dynamic_constants_page() {
            return Err(Error::invalid_usage(
                "parameter storage must come from the formal constants ring",
            ));
        }
        self.buffer_binding(chunk.buffer.clone(), chunk.offset, chunk.size, true)
    }
    pub(crate) fn uniform_upload_buffer(
        self: &Arc<Self>,
        buffer: Arc<Buffer>,
        offset: u64,
        size: u64,
    ) -> Result<Arc<BindlessLease>> {
        if !buffer.is_dynamic_constants_page() {
            return Err(Error::invalid_usage(
                "uniform upload must come from the formal constants ring",
            ));
        }
        self.buffer_binding(buffer, offset, size, true)
    }
    fn buffer_binding(
        self: &Arc<Self>,
        buffer: Arc<Buffer>,
        offset: u64,
        size: u64,
        parameters: bool,
    ) -> Result<Arc<BindlessLease>> {
        if self.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        let limits = self.device.limits();
        if !Arc::ptr_eq(&buffer.device, &self.device)
            || !buffer
                .desc
                .usage
                .contains(vk::BufferUsageFlags::STORAGE_BUFFER)
            || size == 0
            || offset
                .checked_add(size)
                .is_none_or(|end| end > buffer.desc.size)
            || !offset.is_multiple_of(limits.min_storage_buffer_offset_alignment.max(1))
            || size > u64::from(limits.max_storage_buffer_range)
        {
            return Err(Error::invalid_usage(
                "invalid bindless storage buffer device/usage/range",
            ));
        }
        let info = [vk::DescriptorBufferInfo::default()
            .buffer(buffer.handle)
            .offset(offset)
            .range(size)];
        let lease = self.reserve(
            DescriptorKind::StorageBuffer,
            Owner::Buffer {
                buffer,
                offset,
                size,
                parameters,
            },
        )?;
        let write = [vk::WriteDescriptorSet::default()
            .dst_set(self.set.handle)
            .dst_binding(2)
            .dst_array_element(lease.slot.index.index)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .buffer_info(&info)];
        {
            let _update_driver = RecordSpan::new(RecordPhase::UpdateDriver);
            unsafe {
                self.device.core.device.update_descriptor_sets(&write, &[]);
            }
        }
        record_count(RecordCount::StorageBufferWrites, 1);
        Ok(lease)
    }
    pub fn sampled_image(
        self: &Arc<Self>,
        view: Arc<TextureView>,
        layout: vk::ImageLayout,
    ) -> Result<Arc<BindlessLease>> {
        self.image(DescriptorKind::SampledImage, view, layout)
    }
    pub fn storage_image(self: &Arc<Self>, view: Arc<TextureView>) -> Result<Arc<BindlessLease>> {
        self.image(DescriptorKind::StorageImage, view, vk::ImageLayout::GENERAL)
    }
    fn image(
        self: &Arc<Self>,
        kind: DescriptorKind,
        view: Arc<TextureView>,
        layout: vk::ImageLayout,
    ) -> Result<Arc<BindlessLease>> {
        if self.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        let usage = if kind == DescriptorKind::StorageImage {
            vk::ImageUsageFlags::STORAGE
        } else {
            vk::ImageUsageFlags::SAMPLED
        };
        if !Arc::ptr_eq(&view.texture.device, &self.device)
            || !view.desc.usage.contains(usage)
            || (layout != vk::ImageLayout::GENERAL
                && layout != vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL
                // 深度/模板图同时作只读附件与采样时，访问计划的布局就是
                // DEPTH_STENCIL_READ_ONLY_OPTIMAL（access 0x1e/0x14），描述符必须同布局。
                && !(kind == DescriptorKind::SampledImage
                    && layout == vk::ImageLayout::DEPTH_STENCIL_READ_ONLY_OPTIMAL
                    && view
                        .desc
                        .aspect
                        .intersects(vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL)))
        {
            return Err(Error::invalid_usage(
                "invalid bindless image device/usage/layout",
            ));
        }
        let info = [vk::DescriptorImageInfo::default()
            .image_view(view.handle)
            .image_layout(layout)];
        let lease = self.reserve(kind, Owner::View { view, layout })?;
        let write = [vk::WriteDescriptorSet::default()
            .dst_set(self.set.handle)
            .dst_binding(kind as u32)
            .dst_array_element(lease.slot.index.index)
            .descriptor_type(kind.ty())
            .image_info(&info)];
        {
            let _update_driver = RecordSpan::new(RecordPhase::UpdateDriver);
            unsafe {
                self.device.core.device.update_descriptor_sets(&write, &[]);
            }
        }
        record_count(
            if kind == DescriptorKind::SampledImage {
                RecordCount::SampledImageWrites
            } else {
                RecordCount::StorageImageWrites
            },
            1,
        );
        Ok(lease)
    }
    pub fn sampler(self: &Arc<Self>, sampler: Arc<Sampler>) -> Result<Arc<BindlessLease>> {
        if self.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        if !Arc::ptr_eq(&sampler.device, &self.device) {
            return Err(Error::invalid_usage(
                "bindless sampler belongs to another device",
            ));
        }
        let info = [vk::DescriptorImageInfo::default().sampler(sampler.handle)];
        let lease = self.reserve(DescriptorKind::Sampler, Owner::Sampler(sampler))?;
        let write = [vk::WriteDescriptorSet::default()
            .dst_set(self.set.handle)
            .dst_binding(3)
            .dst_array_element(lease.slot.index.index)
            .descriptor_type(vk::DescriptorType::SAMPLER)
            .image_info(&info)];
        {
            let _update_driver = RecordSpan::new(RecordPhase::UpdateDriver);
            unsafe {
                self.device.core.device.update_descriptor_sets(&write, &[]);
            }
        }
        record_count(RecordCount::SamplerWrites, 1);
        Ok(lease)
    }
    fn validate_binding(
        &self,
        pipeline: &VulkanPipeline,
        set: u32,
        leases: &[Arc<BindlessLease>],
    ) -> Result<()> {
        let _trace = RecordSpan::new(RecordPhase::ManagedValidate);
        record_count(RecordCount::BindingChecks, 1);
        record_count(RecordCount::BindingLeaseInputs, leases.len());

        if self.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        if !Arc::ptr_eq(&pipeline.device, &self.device)
            || pipeline
                .layout
                .sets
                .get(set as usize)
                .is_none_or(|layout| layout.handle != self.layout.handle)
            || leases.iter().any(|lease| !std::ptr::eq(self, &*lease.slot.heap))
        {
            return Err(Error::invalid_usage(
                "bindless table/lease/pipeline mismatch",
            ));
        }
        // A lease's private index is immutable and only its final Arc drop
        // retires the slot. Holding these Arcs already proves slot liveness.
        Ok(())
    }
    /// # Safety
    /// Every index dynamically accessed by the shader must have its lease in
    /// `leases`. Image layouts and resource access barriers must match the draw.
    pub unsafe fn bind_command(
        self: &Arc<Self>,
        command: &mut VulkanCommandBuffer,
        pipeline: Arc<VulkanPipeline>,
        set: u32,
        leases: &[Arc<BindlessLease>],
    ) -> Result<()> {
        self.validate_binding(&pipeline, set, leases)?;
        if !command.is_recording() || !Arc::ptr_eq(&command.pool.device, &self.device) {
            return Err(Error::invalid_usage(
                "bindless command is not recording on this device",
            ));
        }
        // The heap and pipeline contain no native image/buffer accesses. Every
        // lease is checked independently against this command's actual patches.
        let retain_trace = RecordSpan::new(RecordPhase::Retain);
        command.retain_object(self.clone())?;
        record_count(RecordCount::RetainSuccesses, 1);
        command.retain_object(pipeline.clone())?;
        record_count(RecordCount::RetainSuccesses, 1);
        for lease in leases {
            if let Owner::Buffer {
                buffer,
                offset,
                size,
                parameters: true,
            } = &lease.owner
            {
                command.retain_upload_read(buffer.clone(), *offset, *size, Self::upload_scope())?;
                record_count(RecordCount::RetainSuccesses, 1);
                // SAFETY: the formal upload intent covers this immutable ring range.
                unsafe {
                    command.retain_recorded(lease.clone())?;
                    record_count(RecordCount::RetainSuccesses, 1);
                }
                continue;
            }
            let known = match &lease.owner {
                Owner::Sampler(_) => true,
                Owner::Buffer {
                    buffer,
                    offset,
                    size,
                    ..
                } => command.recorded_access_covers(
                    &buffer.last_use,
                    crate::resource::OwnershipRange::Buffer {
                        offset: *offset,
                        size: *size,
                    },
                    |scope, _| Self::is_shader_memory_access(scope.access),
                )?,
                Owner::View { view, layout } => command.recorded_access_covers(
                    &view.texture.last_use,
                    crate::resource::OwnershipRange::Image {
                        aspects: view.desc.aspect.as_raw(),
                        base_mip: view.desc.base_mip_level,
                        mip_count: view.desc.level_count,
                        base_layer: view.desc.base_layer,
                        layer_count: view.desc.layer_count,
                    },
                    |scope, current_layout| {
                        current_layout == *layout && Self::is_shader_memory_access(scope.access)
                    },
                )?,
            };
            if known {
                unsafe {
                    command.retain_recorded(lease.clone())?;
                    record_count(RecordCount::RetainSuccesses, 1);
                }
            } else {
                command.retain_opaque(lease.clone())?;
                record_count(RecordCount::RetainSuccesses, 1);
            }
        }
        drop(retain_trace);
        let _native_bind = RecordSpan::new(RecordPhase::NativeBind);
        unsafe { self.device.core.device.cmd_bind_descriptor_sets(command.handle,
            pipeline.point, pipeline.layout.handle, set, &[self.set.handle], &[]); }
        record_count(RecordCount::NativeBindCalls, 1);
        Ok(())
    }
    /// # Safety
    /// In addition to bind_command's contract, declare all dynamically accessed
    /// resources and subranges in this graph pass before recording it.
    pub unsafe fn bind_pass(
        self: &Arc<Self>,
        context: &crate::rg::executor::PassContext<'_>,
        pipeline: Arc<VulkanPipeline>,
        set: u32,
        leases: &[Arc<BindlessLease>],
    ) -> Result<()> {
        self.validate_binding(&pipeline, set, leases)?;
        if context.compiled.plan.definition.context.device_generation
            != self.device.generation().value()
        {
            return Err(Error::invalid_usage(
                "bindless pass belongs to another device generation",
            ));
        }
        for lease in leases {
            Self::validate_pass_lease(context, lease)?;
        }
        let retain_trace = RecordSpan::new(RecordPhase::Retain);
        context.retain_object(self.clone())?;
        record_count(RecordCount::RetainSuccesses, 1);
        context.retain_object(pipeline.clone())?;
        record_count(RecordCount::RetainSuccesses, 1);
        for lease in leases {
            if let Owner::Buffer {
                buffer,
                offset,
                size,
                parameters: true,
            } = &lease.owner
            {
                context.retain_upload_read(buffer.clone(), *offset, *size, Self::upload_scope())?;
                record_count(RecordCount::RetainSuccesses, 1);
            }
            // SAFETY: validate_pass_lease checked every memory range and role.
            unsafe {
                context.retain_recorded(lease.clone())?;
                record_count(RecordCount::RetainSuccesses, 1);
            }
        }
        drop(retain_trace);
        let _native_bind = RecordSpan::new(RecordPhase::NativeBind);
        unsafe { self.device.core.device.cmd_bind_descriptor_sets(
            vk::CommandBuffer::from_raw(context.command.handle()), pipeline.point,
            pipeline.layout.handle, set, &[self.set.handle], &[]); }
        record_count(RecordCount::NativeBindCalls, 1);
        Ok(())
    }
    fn is_shader_memory_access(access: vk::AccessFlags) -> bool {
        // MEMORY_* includes the corresponding shader accesses in Vulkan.
        // Host/transfer/attachment scopes alone cannot cover a shader binding.
        access.intersects(
            vk::AccessFlags::SHADER_READ
                | vk::AccessFlags::SHADER_WRITE
                | vk::AccessFlags::MEMORY_READ
                | vk::AccessFlags::MEMORY_WRITE,
        )
    }
    fn upload_scope() -> crate::resource::range_state::AccessScope {
        crate::resource::range_state::AccessScope {
            stages: vk::PipelineStageFlags::ALL_COMMANDS,
            access: vk::AccessFlags::SHADER_READ,
            write: false,
        }
    }
    /// Every declared access of the pass that a shader may read through a
    /// bindless descriptor: its native scope is a shader memory access and the
    /// logical resource resolves to a physical one. Stable-sorted by physical
    /// resource so one lease only visits its own accesses.
    fn build_coverage(
        context: &crate::rg::executor::PassContext<'_>,
        managed: &crate::rg::pass_bindings::ManagedPassResources,
        pass: &crate::rg::graph::PassDeclaration,
    ) -> Vec<crate::rg::executor::CoverageEntry> {
        use crate::rg::executor::CoverageEntry;
        let semantics = &context.compiled.plan.definition.access_semantics;
        let mut entries = Vec::with_capacity(pass.accesses.len());
        for access in &pass.accesses {
            let Some(native) = semantics
                .iter()
                .find(|entry| entry.code == access.access_code)
            else {
                continue;
            };
            if !Self::is_shader_memory_access(native.scope.access) {
                continue;
            }
            if let Ok(buffer) = managed.buffer_for(access.resource_index) {
                entries.push(CoverageEntry {
                    resource: buffer.resource() as *const _ as usize,
                    layout: None,
                    range: access.range,
                });
            } else if let Ok(texture) = managed.texture_for(access.resource_index) {
                entries.push(CoverageEntry {
                    resource: texture.resource() as *const _ as usize,
                    layout: Some(native.layout),
                    range: access.range,
                });
            }
        }
        entries.sort_by_key(|entry| entry.resource);
        entries
    }
    fn validate_pass_lease(
        context: &crate::rg::executor::PassContext<'_>,
        lease: &BindlessLease,
    ) -> Result<()> {
        let _trace = RecordSpan::new(RecordPhase::ManagedValidate);
        record_count(RecordCount::PassLeaseChecks, 1);

        use crate::rg::{resource::AccessRange, vulkan::ResourceSpec};
        let (range, descriptor) = match &lease.owner {
            Owner::Sampler(_)
            | Owner::Buffer {
                parameters: true, ..
            } => return Ok(()),
            Owner::Buffer {
                buffer,
                offset,
                size,
                ..
            } => (
                AccessRange::Buffer {
                    offset: *offset,
                    size: *size,
                },
                ResourceSpec::Buffer(buffer.desc).descriptor(),
            ),
            Owner::View { view, .. } => (
                AccessRange::Texture {
                    aspect_mask: view.desc.aspect.as_raw(),
                    base_mip_level: view.desc.base_mip_level,
                    level_count: view.desc.level_count,
                    base_array_layer: view.desc.base_layer,
                    layer_count: view.desc.layer_count,
                },
                ResourceSpec::Image(view.texture.desc).descriptor(),
            ),
        };
        let managed = context.managed.ok_or_else(|| {
            Error::invalid_usage(
                "bindless graph resources require the production managed projection",
            )
        })?;
        let pass = context
            .compiled
            .declarations
            .passes
            .get(context.pass_index)
            .ok_or_else(|| Error::invalid_usage("bindless pass index is invalid"))?;
        let entries = context
            .coverage
            .entries(|| Self::build_coverage(context, managed, pass));
        // Entries are sorted by resource address; only this resource's accesses matter.
        let (key, layout) = match &lease.owner {
            Owner::Buffer { buffer, .. } => (Arc::as_ptr(buffer) as usize, None),
            Owner::View { view, layout } => (Arc::as_ptr(&view.texture) as usize, Some(*layout)),
            Owner::Sampler(_) => return Ok(()),
        };
        let first = entries.partition_point(|entry| entry.resource < key);
        record_count(RecordCount::CoverageAccesses, entries.len());
        let ranges = entries[first..]
            .iter()
            .take_while(|entry| entry.resource == key)
            .filter(|entry| entry.layout == layout)
            .map(|entry| entry.range);
        if !crate::rg::resource_contracts::covers_iter(ranges, range, descriptor)
            .map_err(|_| Error::invalid_usage("bindless lease range is invalid"))?
        {
            return Err(Error::invalid_usage(
                "bindless lease range or image layout is not covered by this pass's shader declarations",
            ));
        }
        Ok(())
    }
}
use ash::vk::Handle;

/// 页选择只服务现有绑定 ABI；每个物理页仍是独立 BindlessHeap。
pub struct BindlessPagePool {
    template: Arc<BindlessHeap>,
    pages: Mutex<crate::raw::temp_list::TempList<Arc<BindlessPage>>>,
    growth: Mutex<()>,
    frame_bank: std::sync::atomic::AtomicUsize,
    parameter_cpu: parameters::cpu::ParameterCpuPool,
    _memory: crate::raw::allocator::CpuMemoryReservation,
}
struct BindlessPage {
    heap: Arc<BindlessHeap>,
    bank: usize,
    group: Mutex<()>,
    _memory: crate::raw::allocator::CpuMemoryReservation,
}
/// 只在完整组准备期间借用；实际 leases 返回后仍由命令完成点保护。
pub struct GroupReservation<'a> {
    heap: &'a Arc<BindlessHeap>,
    cpu_pool: &'a parameters::cpu::ParameterCpuPool,
    cpu: &'a mut parameters::cpu::ParameterCpuCursor,
}
#[derive(Debug)]
pub enum GroupPreparationError {
    InsufficientSlots { required: [usize; 4] },
    Error(Error),
}
impl From<Error> for GroupPreparationError {
    fn from(error: Error) -> Self {
        Self::Error(error)
    }
}
pub type GroupPreparationResult<T> = std::result::Result<T, GroupPreparationError>;
impl GroupReservation<'_> {
    pub fn heap(&self) -> &Arc<BindlessHeap> {
        self.heap
    }
    pub fn parameters(&mut self, abi: Arc<BindlessShaderAbi>, push: &[u8]) -> Result<BindlessParameters> {
        BindlessParameters::from_credit(self.heap.clone(), abi, push,
            |bytes| self.cpu.claim(self.cpu_pool, bytes))
    }
    /// A previous block can only be reused inside its actual physical page.
    pub fn with_push(&mut self, previous: &PreparedBindlessParameters, push: &[u8],
        allocate: impl FnOnce(u64) -> Result<crate::resource::DynamicConstantsChunk>,
    ) -> GroupPreparationResult<Option<PreparedBindlessParameters>> {
        if !previous.belongs_to(self.heap) { return Ok(None); }
        self.require_slots([0, 0, 1, 0])?;
        previous.with_push_credit(push, allocate, |bytes| self.cpu.claim(self.cpu_pool, bytes))
            .map(Some).map_err(Into::into)
    }
    /// 所有页 writer 都在同一互斥事务内；准入后其他 writer 不能抢占空槽。
    /// 需求必须包含本次缺失缓存、实际新增资源和不可变参数块。
    pub fn require_slots(&self, required: [usize; 4]) -> GroupPreparationResult<()> {
        let _trace = RecordSpan::new(RecordPhase::RequireSlots);

        let check = || {
            let slots = self.heap.slots.lock();
            let capacity = std::array::from_fn(|kind| slots.entries[kind].len());
            let free = std::array::from_fn(|kind| slots.free[kind].len());
            (capacity, free)
        };
        let (capacity, free): ([usize; 4], [usize; 4]) = check();
        if required.iter().zip(capacity).any(|(need, cap)| *need > cap) {
            return Err(Error::unsupported(format!(
                "complete bindless group exceeds a fresh page: required={required:?}, capacity={capacity:?}"
            )).into());
        }
        if required.iter().zip(free).all(|(need, free)| *need <= free) {
            return Ok(());
        }
        // manager 锁已释放；只读取实际完成值，不等待 GPU 或设备 idle。
        self.heap.collect()?;
        let (_, free) = check();
        if required.iter().zip(free).all(|(need, free)| *need <= free) {
            Ok(())
        } else {
            Err(GroupPreparationError::InsufficientSlots { required })
        }
    }
}
impl BindlessPagePool {
    pub fn new(template: Arc<BindlessHeap>) -> Result<Arc<Self>> {
        let budget = &template.device.memory.stats;
        let memory = crate::raw::allocator::reserve_shared_cpu_metadata::<Self>(budget, 0)?;
        let page = Self::page(template.clone(), 0)?;
        // 页数另受 CPU/native descriptor 全池预算约束；此限只约束元数据地址空间。
        let mut pages = crate::raw::temp_list::TempList::new(65536, budget.clone())?;
        pages.add(page)?;
        let parameter_cpu = parameters::cpu::ParameterCpuPool::new(budget.clone())?;
        Ok(Arc::new(Self {
            template,
            pages: Mutex::new(pages),
            growth: Mutex::new(()),
            frame_bank: std::sync::atomic::AtomicUsize::new(0),
            parameter_cpu,
            _memory: memory,
        }))
    }
    fn page(heap: Arc<BindlessHeap>, bank: usize) -> Result<Arc<BindlessPage>> {
        heap.set_residency(true);
        let memory = crate::raw::allocator::reserve_shared_cpu_metadata::<BindlessPage>(
            &heap.device.memory.stats,
            0,
        )?;
        Ok(Arc::new(BindlessPage {
            heap,
            bank,
            group: Mutex::new(()),
            _memory: memory,
        }))
    }
    pub fn template(&self) -> &Arc<BindlessHeap> {
        &self.template
    }
    pub fn cursor(self: &Arc<Self>) -> BindlessPageCursor {
        BindlessPageCursor {
            pool: self.clone(),
            current: None,
            bank: self.frame_bank.load(std::sync::atomic::Ordering::Relaxed),
            cpu: parameters::cpu::ParameterCpuCursor::default(),
        }
    }
    /// Rotate descriptor arenas with the host's display-frame boundary. Each
    /// arena learns its capacity during startup even when the GPU happens to
    /// finish cold frames immediately. Leases still govern actual reuse; this
    /// never resets live slots or guesses GPU completion from a frame index.
    pub fn advance_frame(&self) {
        self.parameter_cpu.trim();
        let count = self.template.device.frames_in_flight() as usize;
        self.frame_bank
            .fetch_update(
                std::sync::atomic::Ordering::Relaxed,
                std::sync::atomic::Ordering::Relaxed,
                |bank| Some((bank + 1) % count),
            )
            .unwrap();
    }
    /// Sample the pages present at request start. Each heap is read once,
    /// outside the manager lock. Pages only append while this pool is alive;
    /// newly appended pages are included by the next request.
    pub fn sampled_occupancy(&self) -> BindlessOccupancy {
        let count = self.page_count();
        let mut sampled = BindlessOccupancy::default();
        for index in 0..count {
            let heap = {
                let pages = self.pages.lock();
                pages.as_slice()[index].heap.clone()
            };
            sampled.include(heap.stats());
        }
        sampled
    }
    pub fn page_count(&self) -> usize {
        self.pages.lock().as_slice().len()
    }
    fn available(&self, excluded: &[Arc<BindlessPage>], bank: usize) -> Option<Arc<BindlessPage>> {
        let pages = self.pages.lock();
        pages
            .as_slice()
            .iter()
            .filter(|page| {
                page.bank == bank && !excluded.iter().any(|used| Arc::ptr_eq(used, page))
            })
            // Prefer an idle cursor's page, but a held cursor does not imply
            // that its descriptor slots are full. Group admission is already
            // serialized by page.group; try those slots before growing.
            .min_by_key(|page| usize::from(Arc::strong_count(page) != 1))
            .cloned()
    }
    fn append_page(
        &self,
        excluded: &[Arc<BindlessPage>],
        bank: usize,
    ) -> Result<(Arc<BindlessPage>, bool)> {
        let _growth = self.growth.lock();
        // Another recorder may have added usable capacity since our search.
        if let Some(page) = self.available(excluded, bank) {
            return Ok((page, false));
        }
        // 原生创建和潜在失败清理都在 manager 锁外。
        let page = Self::page(self.template.new_page()?, bank)?;
        let admitted = {
            let mut pages = self.pages.lock();
            pages.add(page.clone()).map(|_| ())
        };
        // add 失败时 manager 内只释放一个 clone，最后的 page owner 在锁外释放。
        admitted?;
        Ok((page, true))
    }
}
impl Drop for BindlessPagePool {
    fn drop(&mut self) {
        // 最后一个 pool Arc 消失后没有 cursor；此处不持 manager 锁获取 Slots。
        for page in self.pages.get_mut().as_slice() {
            // Resident leases hold their heap: break the cycle with the pool.
            page.heap.set_residency(false);
        }
    }
}
/// 一个录制 callback 的页游标；同页快路径不访问 manager，也不按 draw 扫描页池。
pub struct BindlessPageCursor {
    pool: Arc<BindlessPagePool>,
    current: Option<Arc<BindlessPage>>,
    bank: usize,
    cpu: parameters::cpu::ParameterCpuCursor,
}
impl BindlessPageCursor {
    pub fn prepare(
        &mut self,
        mut build: impl FnMut(
            &mut GroupReservation<'_>,
        ) -> GroupPreparationResult<PreparedBindlessParameters>,
    ) -> Result<PreparedBindlessParameters> {
        let _trace = RecordSpan::new(RecordPhase::CompleteGroup);
        record_count(RecordCount::CompleteGroupCalls, 1);

        let mut excluded = Vec::new();
        let mut created = false;
        loop {
            record_count(RecordCount::GroupAttempts, 1);
            let page = if let Some(current) = self.current.take() {
                record_count(RecordCount::CurrentPageReuse, 1);
                current
            } else if let Some(page) = {
                record_count(RecordCount::PageSearches, 1);
                self.pool.available(&excluded, self.bank)
            } {
                page
            } else if !created {
                let (page, appended) = self.pool.append_page(&excluded, self.bank)?;
                created = appended;
                if appended {
                    record_count(RecordCount::PagesCreated, 1);
                }
                page
            } else {
                return Err(Error::invalid_usage(
                    "fresh bindless page did not satisfy the admitted group",
                ));
            };
            let result = {
                let group_wait = RecordSpan::new(RecordPhase::GroupLockWait);
                let _guard = page.group.lock();
                drop(group_wait);
                build(&mut GroupReservation { heap: &page.heap,
                    cpu_pool: &self.pool.parameter_cpu, cpu: &mut self.cpu })
            };
            match result {
                Ok(prepared) => {
                    self.current = Some(page);
                    return Ok(prepared);
                }
                Err(GroupPreparationError::InsufficientSlots { .. }) => {
                    record_count(RecordCount::InsufficientRetries, 1);
                    excluded
                        .try_reserve(1)
                        .map_err(|_| Error::OutOfHostMemory)?;
                    excluded.push(page);
                }
                Err(GroupPreparationError::Error(error)) => {
                    self.current = Some(page);
                    return Err(error);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_index_reuses_live_leases_without_retaining_retired_ranges_or_itself() {
        let device = match Device::new_headless(&crate::device::DeviceConfig::headless()) {
            Ok(device) => device,
            Err(error) => {
                assert!(
                    !std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|v| v == "1"),
                    "{error}"
                );
                eprintln!("SKIP descriptor cache GPU initialization: {error}");
                return;
            }
        };
        let heap = BindlessHeap::new(
            device.clone(),
            BindlessConfig {
                sampled_images: 4,
                storage_images: 4,
                storage_buffers: 4,
                samplers: 4,
            },
        )
        .unwrap();
        heap.set_residency(true);
        let buffer = Arc::new(
            Buffer::new(
                device,
                &crate::raw::buffer::BufferDesc::new_cpu_to_gpu(
                    8192 * 256,
                    vk::BufferUsageFlags::STORAGE_BUFFER,
                ),
            )
            .unwrap(),
        );
        // Thousands of independent ranges of one still-live allocation exposed
        // the old cache's mistaken resource strong-count liveness test.
        for range in 0..8192u64 {
            let offset = range * 256;
            let key =
                parameters::ResourceKey::Buffer(buffer.raw_handle().as_raw(), offset, 4, false);
            let first = heap
                .resident_lease(key, || heap.storage_buffer(buffer.clone(), offset, 4))
                .unwrap();
            let second = heap
                .resident_lease(key, || panic!("live lease must be reused"))
                .unwrap();
            assert!(Arc::ptr_eq(&first, &second));
            drop(first);
            assert_eq!(heap.stats().live, 1);
            drop(second);
            assert_eq!(heap.stats().live, 0);
            assert!(heap.resident.lock().as_ref().unwrap().leases.len() <= MAX_RESIDENT_KEYS);
        }
        let weak = Arc::downgrade(&heap);
        drop(heap);
        assert!(
            weak.upgrade().is_none(),
            "an index cannot form a heap/lease ownership cycle"
        );
    }

    #[test]
    fn slots_reuse_only_after_release_and_stale_generations_never_match() {
        let mut slots = Slots::new([1; 4]).unwrap();
        let first = slots.allocate(DescriptorKind::StorageBuffer).unwrap();
        assert!(matches!(slots.allocate(DescriptorKind::StorageBuffer), Err(Error::Exhausted(_))));
        slots.release(first);
        let second = slots.allocate(DescriptorKind::StorageBuffer).unwrap();
        assert_eq!(first.index, second.index);
        assert_ne!(first.generation, second.generation);
        assert!(!slots.valid(first));
        assert!(slots.valid(second));
    }
    #[test]
    fn table_statistics_distinguish_live_free_and_exhausted_kinds() {
        let mut slots = Slots::new([2, 1, 2, 1]).unwrap();
        let first = slots.allocate(DescriptorKind::StorageBuffer).unwrap();
        let second = slots.allocate(DescriptorKind::StorageBuffer).unwrap();
        assert!(slots.allocate(DescriptorKind::StorageBuffer).is_err());
        assert_eq!(slots.table_stats(DescriptorKind::StorageBuffer).peak_live, 2);
        slots.release(first);
        slots.release(second);
        slots.entries[DescriptorKind::Sampler as usize][0].generation = u64::MAX;
        assert!(slots.allocate(DescriptorKind::Sampler).is_err());
        for kind in DescriptorKind::ALL {
            let stats = slots.table_stats(kind);
            assert_eq!(stats.capacity, stats.live + stats.free + stats.exhausted);
        }
        assert_eq!(slots.table_stats(DescriptorKind::StorageBuffer).free, 2);
    }

    #[test]
    fn sampled_occupancy_uses_current_slots_and_preserves_exhausted_capacity() {
        let mut a = Slots::new([2, 1, 4, 2]).unwrap();
        let historical: Vec<_> = (0..4).map(|_| a.allocate(DescriptorKind::StorageBuffer).unwrap()).collect();
        for index in historical { a.release(index); }
        let _image = a.allocate(DescriptorKind::SampledImage).unwrap();
        let mut b = Slots::new([1; 4]).unwrap();
        b.entries[DescriptorKind::Sampler as usize][0].generation = u64::MAX;
        assert!(b.allocate(DescriptorKind::Sampler).is_err());
        let _buffer = b.allocate(DescriptorKind::StorageBuffer).unwrap();
        let mut sampled = BindlessOccupancy::default();
        sampled.include(a.statistics()); sampled.include(b.statistics());
        assert_eq!(a.statistics().peak_live, 4);
        assert_eq!(sampled.live, 2);
        assert_eq!(sampled.capacity, 13);
        assert_eq!(sampled.free, 10);
        for table in sampled.tables { assert_eq!(table.capacity, table.live + table.free + table.exhausted); }
        assert_eq!(sampled.table(DescriptorKind::Sampler).exhausted, 1);
    }
}
