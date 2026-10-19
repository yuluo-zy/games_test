//! A renderer-owned update-after-bind descriptor table. A lease is immutable:
//! changing a resource allocates a different slot. Its slot returns to the free
//! list only after every registered GPU use completes. See the Vulkan
//! [descriptor binding rules](https://docs.vulkan.org/spec/latest/chapters/descriptorsets.html).
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
    resource::{GpuUse, SyncCell},
    sync::{CompletedValues, SyncSet},
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
    Retiring,
    Exhausted,
}
struct Slot {
    generation: u64,
    state: SlotState,
}
/// Slot bookkeeping. Retiring slots wait in a queue keyed by their completion
/// set, so collection inspects only retiring slots, never the whole table.
struct Slots {
    live: usize,
    retiring_count: usize,
    exhausted: usize,
    peak_live: usize,
    peak_retiring: usize,
    entries: [Vec<Slot>; 4],
    free: [VecDeque<u32>; 4],
    retiring: [VecDeque<(u32, SyncSet)>; 4],
}
impl Slots {
    fn new(counts: [u32; 4]) -> Result<Self> {
        let mut entries = std::array::from_fn(|_| Vec::new());
        let mut free = std::array::from_fn(|_| VecDeque::new());
        let mut retiring = std::array::from_fn(|_| VecDeque::new());
        for table in 0..4 {
            let count = counts[table] as usize;
            entries[table]
                .try_reserve_exact(count)
                .map_err(|_| Error::OutOfHostMemory)?;
            free[table]
                .try_reserve_exact(count)
                .map_err(|_| Error::OutOfHostMemory)?;
            retiring[table]
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
            retiring_count: 0,
            exhausted: 0,
            peak_live: 0,
            peak_retiring: 0,
            entries,
            free,
            retiring,
        })
    }
    fn host_bytes(&self) -> Result<u64> {
        Ok(crate::raw::allocator::shared_cpu_bytes::<BindlessHeap>()?
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
                + self
                    .retiring
                    .iter()
                    .map(|v| v.capacity() * std::mem::size_of::<(u32, SyncSet)>())
                    .sum::<usize>()) as u64)
    }
    fn has_free(&self, kind: DescriptorKind) -> bool {
        !self.free[kind as usize].is_empty()
    }
    fn allocate(&mut self, kind: DescriptorKind) -> Result<BindlessIndex> {
        let table = kind as usize;
        while let Some(index) = self.free[table].pop_front() {
            let slot = &mut self.entries[table][index as usize];
            let Some(generation) = slot.generation.checked_add(1) else {
                slot.state = SlotState::Exhausted;
                self.exhausted += 1;
                continue;
            };
            slot.generation = generation;
            slot.state = SlotState::Live;
            self.live += 1;
            self.peak_live = self.peak_live.max(self.live);
            return Ok(BindlessIndex {
                kind,
                index,
                generation,
            });
        }
        Err(Error::exhausted("bindless descriptor table is full"))
    }
    fn valid(&self, index: BindlessIndex) -> bool {
        self.entries[index.kind as usize]
            .get(index.index as usize)
            .is_some_and(|slot| {
                slot.generation == index.generation && matches!(slot.state, SlotState::Live)
            })
    }
    fn retire(&mut self, index: BindlessIndex, after: SyncSet) {
        debug_assert!(self.valid(index));
        let table = index.kind as usize;
        self.live -= 1;
        self.entries[table][index.index as usize].state = SlotState::Retiring;
        if after.is_empty() {
            // Never used by the GPU: immediately reusable.
            self.entries[table][index.index as usize].state = SlotState::Free;
            self.free[table].push_back(index.index);
            return;
        }
        self.retiring_count += 1;
        self.peak_retiring = self.peak_retiring.max(self.retiring_count);
        // Capacity equals the table size, so this never reallocates.
        self.retiring[table].push_back((index.index, after));
    }
    /// Move every retiring slot whose completion set is reached to the free
    /// list. Cost is proportional to the retiring slots only.
    fn collect(&mut self, completed: &CompletedValues) -> usize {
        let mut count = 0;
        for table in 0..4 {
            let (retiring, free, entries) = (
                &mut self.retiring[table],
                &mut self.free[table],
                &mut self.entries[table],
            );
            retiring.retain(|(index, after)| {
                if !after.is_reached(completed) {
                    return true;
                }
                entries[*index as usize].state = SlotState::Free;
                free.push_back(*index);
                count += 1;
                false
            });
        }
        self.retiring_count -= count;
        count
    }
    fn has_retiring(&self) -> bool {
        self.retiring_count != 0
    }
}

pub struct BindlessHeap {
    pub layout: Arc<VulkanDescriptorSetLayout>,
    set: Arc<VulkanDescriptorSet>,
    device: Arc<Device>,
    slots: Mutex<Slots>,
    // Declared after slots: release credit only after the actual CPU buffers.
    _slot_memory: crate::raw::allocator::CpuMemoryReservation,
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
    heap: Arc<BindlessHeap>,
    index: BindlessIndex,
    last_use: Arc<SyncCell>,
    owner: Owner,
    _memory: crate::raw::allocator::CpuMemoryReservation,
}
impl BindlessLease {
    pub fn index(&self) -> BindlessIndex {
        self.index
    }
}
impl GpuUse for BindlessLease {
    fn visit_uses(&self, visit: &mut crate::resource::UseVisitor<'_>) -> Result<()> {
        visit(&self.last_use)?;
        self.owner.visit_uses(visit)?;
        self.heap.visit_uses(visit)
    }
}
impl Drop for BindlessLease {
    fn drop(&mut self) {
        self.heap
            .slots
            .lock()
            .retire(self.index, self.last_use.snapshot());
    }
}
impl GpuUse for BindlessHeap {
    fn visit_uses(&self, visit: &mut crate::resource::UseVisitor<'_>) -> Result<()> {
        self.set.visit_uses(visit)
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct BindlessStats {
    pub peak_live: usize,
    pub peak_retiring: usize,
    pub capacity: usize,
    pub live: usize,
    pub retiring: usize,
    pub free: usize,
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
                .instance
                .instance
                .get_physical_device_properties2(device.physical, &mut properties);
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
            + u64::from(total) * (std::mem::size_of::<Slot>() + std::mem::size_of::<u32>()) as u64;
        let mut slot_memory = device
            .allocator_stats
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
            _slot_memory: slot_memory,
        }))
    }
    /// Recycle retired slots whose GPU uses have completed. Call once per
    /// frame tick; allocation also collects lazily when a table runs dry.
    pub fn collect(&self) -> Result<usize> {
        if !self.slots.lock().has_retiring() {
            return Ok(0);
        }
        let completed = self.device.poll()?;
        Ok(self.slots.lock().collect(&completed))
    }
    pub fn stats(&self) -> BindlessStats {
        let slots = self.slots.lock();
        BindlessStats {
            peak_live: slots.peak_live,
            peak_retiring: slots.peak_retiring,
            capacity: slots.entries.iter().map(Vec::len).sum(),
            live: slots.live,
            retiring: slots.retiring_count,
            free: slots.free.iter().map(VecDeque::len).sum(),
        }
    }
    fn reserve(self: &Arc<Self>, kind: DescriptorKind, owner: Owner) -> Result<Arc<BindlessLease>> {
        if !self.slots.lock().has_free(kind) {
            self.collect()?;
        }
        let memory = self.device.allocator_stats.reserve_cpu_bytes(
            crate::raw::allocator::shared_cpu_bytes::<BindlessLease>()?,
            crate::device::BudgetCategory::Live,
        )?;
        let last_use =
            SyncCell::for_device_budgeted(self.device.generation(), &self.device.allocator_stats)?;
        let index = self.slots.lock().allocate(kind)?;
        Ok(Arc::new(BindlessLease {
            heap: self.clone(),
            index,
            owner,
            last_use,
            _memory: memory,
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
            .dst_array_element(lease.index.index)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .buffer_info(&info)];
        let _update = self.slots.lock();
        unsafe {
            self.device.device.update_descriptor_sets(&write, &[]);
        }
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
                && layout != vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
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
            .dst_array_element(lease.index.index)
            .descriptor_type(kind.ty())
            .image_info(&info)];
        let _update = self.slots.lock();
        unsafe {
            self.device.device.update_descriptor_sets(&write, &[]);
        }
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
            .dst_array_element(lease.index.index)
            .descriptor_type(vk::DescriptorType::SAMPLER)
            .image_info(&info)];
        let _update = self.slots.lock();
        unsafe {
            self.device.device.update_descriptor_sets(&write, &[]);
        }
        Ok(lease)
    }
    fn validate_binding(
        &self,
        pipeline: &VulkanPipeline,
        set: u32,
        leases: &[Arc<BindlessLease>],
    ) -> Result<()> {
        if self.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        if !Arc::ptr_eq(&pipeline.device, &self.device)
            || pipeline
                .layout
                .sets
                .get(set as usize)
                .is_none_or(|layout| layout.handle != self.layout.handle)
            || leases.iter().any(|lease| !std::ptr::eq(self, &*lease.heap))
        {
            return Err(Error::invalid_usage(
                "bindless table/lease/pipeline mismatch",
            ));
        }
        let slots = self.slots.lock();
        if leases.iter().any(|lease| !slots.valid(lease.index)) {
            return Err(Error::invalid_usage("stale bindless lease"));
        }
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
        command.retain_object(self.clone())?;
        command.retain_object(pipeline.clone())?;
        for lease in leases {
            if let Owner::Buffer {
                buffer,
                offset,
                size,
                parameters: true,
            } = &lease.owner
            {
                command.retain_upload_read(buffer.clone(), *offset, *size, Self::upload_scope())?;
                // SAFETY: the formal upload intent covers this immutable ring range.
                unsafe {
                    command.retain_recorded(lease.clone())?;
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
                }
            } else {
                command.retain_opaque(lease.clone())?;
            }
        }
        unsafe {
            self.device.device.cmd_bind_descriptor_sets(
                command.handle,
                pipeline.point,
                pipeline.layout.handle,
                set,
                &[self.set.handle],
                &[],
            );
        }
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
        context.retain_object(self.clone())?;
        context.retain_object(pipeline.clone())?;
        for lease in leases {
            if let Owner::Buffer {
                buffer,
                offset,
                size,
                parameters: true,
            } = &lease.owner
            {
                context.retain_upload_read(buffer.clone(), *offset, *size, Self::upload_scope())?;
            }
            // SAFETY: validate_pass_lease checked every memory range and role.
            unsafe {
                context.retain_recorded(lease.clone())?;
            }
        }
        unsafe {
            self.device.device.cmd_bind_descriptor_sets(
                vk::CommandBuffer::from_raw(context.command.handle()),
                pipeline.point,
                pipeline.layout.handle,
                set,
                &[self.set.handle],
                &[],
            );
        }
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
    fn validate_pass_lease(
        context: &crate::rg::executor::PassContext<'_>,
        lease: &BindlessLease,
    ) -> Result<()> {
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
        let semantics = &context.compiled.plan.definition.access_semantics;
        let ranges =
            pass.accesses
                .iter()
                .filter(|access| {
                    let Some(native) = semantics
                        .iter()
                        .find(|entry| entry.code == access.access_code)
                    else {
                        return false;
                    };
                    if !Self::is_shader_memory_access(native.access) {
                        return false;
                    }
                    match &lease.owner {
                        Owner::Buffer { buffer, .. } => managed
                            .buffer_for(access.resource_index)
                            .is_ok_and(|candidate| std::ptr::eq(candidate.resource(), &**buffer)),
                        Owner::View { view, layout } => {
                            native.layout == *layout
                                && managed.texture_for(access.resource_index).is_ok_and(
                                    |candidate| std::ptr::eq(candidate.resource(), &*view.texture),
                                )
                        }
                        Owner::Sampler(_) => false,
                    }
                })
                .map(|access| access.range);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::{QueueId, SyncPoint};
    #[test]
    fn slots_require_all_queues_to_complete_and_stale_generations_never_match() {
        let mut slots = Slots::new([1; 4]).unwrap();
        let first = slots.allocate(DescriptorKind::StorageBuffer).unwrap();
        let mut after = SyncSet::EMPTY;
        after.insert(SyncPoint {
            queue: QueueId::new(0).unwrap(),
            value: 3,
        });
        after.insert(SyncPoint {
            queue: QueueId::new(1).unwrap(),
            value: 7,
        });
        slots.retire(first, after);
        let mut complete = CompletedValues::default();
        complete.raise(QueueId::new(0).unwrap(), 3);
        assert_eq!(slots.collect(&complete), 0);
        assert!(matches!(
            slots.allocate(DescriptorKind::StorageBuffer),
            Err(Error::Exhausted(_))
        ));
        complete.raise(QueueId::new(1).unwrap(), 7);
        assert_eq!(slots.collect(&complete), 1);
        let second = slots.allocate(DescriptorKind::StorageBuffer).unwrap();
        assert_eq!(first.index, second.index);
        assert_ne!(first.generation, second.generation);
        assert!(!slots.valid(first));
        assert!(slots.valid(second));
    }
    #[test]
    fn collection_visits_only_retiring_slots_and_unused_leases_free_immediately() {
        let queue = QueueId::new(0).unwrap();
        let mut slots = Slots::new([4; 4]).unwrap();
        let a = slots.allocate(DescriptorKind::SampledImage).unwrap();
        let b = slots.allocate(DescriptorKind::SampledImage).unwrap();
        let c = slots.allocate(DescriptorKind::SampledImage).unwrap();
        slots.retire(a, SyncSet::from_point(SyncPoint { queue, value: 9 }));
        slots.retire(b, SyncSet::from_point(SyncPoint { queue, value: 2 }));
        slots.retire(c, SyncSet::EMPTY);
        assert_eq!(slots.retiring_count, 2);
        assert_eq!(
            slots.free[0].len(),
            2,
            "unused lease is free without collection"
        );
        let mut complete = CompletedValues::default();
        complete.raise(queue, 2);
        // A later-retired but earlier-completed slot is not blocked by `a`.
        assert_eq!(slots.collect(&complete), 1);
        assert_eq!(
            slots.retiring[0].front().map(|entry| entry.0),
            Some(a.index)
        );
        complete.raise(queue, 9);
        assert_eq!(slots.collect(&complete), 1);
        assert!(!slots.has_retiring());
        assert_eq!(slots.free[0].len(), 4);
    }
}
