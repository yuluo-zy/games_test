//! Typed resource and parameter bindings for the production shader heap ABI.
use super::{BindlessHeap, BindlessLease, DescriptorKind};
use crate::rg::profiling::{RecordCount, RecordPhase, RecordSpan, RecordTally, record_count};
use crate::{
    Error, Result,
    raw::{
        buffer::Buffer,
        dynamic_constants::DynamicConstantsChunk,
        sampler::Sampler,
        shader::{PipelineLayout, VulkanPipeline},
        texture::TextureView,
    },
};
use ash::vk;
use shader_compiler::bindless::{BindingSlot, BindlessAbi};
use shader_pipeline::{
    compiled::{CompiledShaderPipeline, CompiledShaderStage},
    reflection::{BindingArray, BindingKind, BindingLocation},
};
use smallvec::SmallVec;
use std::sync::Arc;

/// A lowered package uses the renderer's shared layout. Input serialization is
/// an asset format, rather than a second runtime descriptor implementation.
pub struct BindlessShaderPackage {
    pub abi: Arc<BindlessShaderAbi>,
    pub stages: Vec<CompiledShaderStage>,
    pub layout: Arc<PipelineLayout>,
    _stage_memory: crate::raw::allocator::CpuMemoryReservation,
}
pub struct BindlessShaderAbi {
    definition: BindlessAbi,
    _memory: crate::raw::allocator::CpuMemoryReservation,
}
impl std::ops::Deref for BindlessShaderAbi {
    type Target = BindlessAbi;
    fn deref(&self) -> &Self::Target {
        &self.definition
    }
}
impl PartialEq for BindlessShaderAbi {
    fn eq(&self, other: &Self) -> bool {
        self.definition == other.definition
    }
}
impl Eq for BindlessShaderAbi {}
impl std::fmt::Debug for BindlessShaderAbi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.definition.fmt(f)
    }
}
impl BindlessShaderPackage {
    pub fn new(
        heap: &Arc<BindlessHeap>,
        input: &CompiledShaderPipeline,
        runtime_array_limit: u32,
    ) -> Result<Self> {
        if heap.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        let maximum_lowering_bytes = input
            .stages
            .iter()
            .map(|stage| stage.spirv.len() as u64 * 4)
            .sum::<u64>()
            .saturating_mul(96)
            .saturating_add(
                input
                    .reflection
                    .descriptor_sets
                    .iter()
                    .map(|set| set.len() as u64)
                    .sum::<u64>()
                    .saturating_mul(256),
            );
        let mut stage_memory = heap
            .device
            .memory
            .stats
            .reserve_cpu_bytes(maximum_lowering_bytes, crate::device::BudgetCategory::Live)?;
        let (abi, stages) =
            shader_compiler::bindless::lower_pipeline_with_limit(input, runtime_array_limit)
                .map_err(|error| Error::invalid_usage_with("bindless shader ABI", error))?;
        let flags = input
            .stages
            .iter()
            .fold(vk::ShaderStageFlags::empty(), |flags, stage| {
                flags
                    | match stage.stage {
                        shader_pipeline::compiled::ShaderStage::Compute => {
                            vk::ShaderStageFlags::COMPUTE
                        }
                        shader_pipeline::compiled::ShaderStage::Vertex => {
                            vk::ShaderStageFlags::VERTEX
                        }
                        shader_pipeline::compiled::ShaderStage::Fragment => {
                            vk::ShaderStageFlags::FRAGMENT
                        }
                    }
            });
        let abi_bytes = (std::mem::size_of::<BindlessShaderAbi>()
            + abi.bindings.capacity() * std::mem::size_of::<BindingSlot>())
            as u64;
        let memory = heap
            .device
            .memory
            .stats
            .reserve_cpu_bytes(abi_bytes, crate::device::BudgetCategory::Live)?;
        let abi = Arc::new(BindlessShaderAbi {
            definition: abi,
            _memory: memory,
        });
        stage_memory.resize(
            (stages.capacity() * std::mem::size_of::<CompiledShaderStage>()
                + stages
                    .iter()
                    .map(|stage| stage.spirv.capacity() * 4 + stage.entry_point.capacity())
                    .sum::<usize>()) as u64,
        )?;
        let layout = PipelineLayout::new_bindless(
            heap.device.clone(),
            vec![heap.layout.clone()],
            vec![vk::PushConstantRange {
                stage_flags: flags,
                offset: 0,
                size: 4,
            }],
            abi.clone(),
        )?;
        Ok(Self {
            abi,
            stages,
            layout,
            _stage_memory: stage_memory,
        })
    }
}

#[derive(Clone)]
pub enum BindlessResource {
    Buffer {
        buffer: Arc<Buffer>,
        offset: u64,
        size: u64,
    },
    SampledImage {
        view: Arc<TextureView>,
        layout: vk::ImageLayout,
    },
    StorageImage(Arc<TextureView>),
    Sampler(Arc<Sampler>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum ResourceKey {
    Buffer(u64, u64, u64, bool),
    SampledImage(u64, i32),
    StorageImage(u64),
    Sampler(u64),
}
impl BindlessResource {
    fn key(&self, uniform_upload: bool) -> ResourceKey {
        use ash::vk::Handle;
        match self {
            Self::Buffer {
                buffer,
                offset,
                size,
            } => ResourceKey::Buffer(buffer.handle.as_raw(), *offset, *size, uniform_upload),
            Self::SampledImage { view, layout } => {
                ResourceKey::SampledImage(view.handle.as_raw(), layout.as_raw())
            }
            Self::StorageImage(view) => ResourceKey::StorageImage(view.handle.as_raw()),
            Self::Sampler(sampler) => ResourceKey::Sampler(sampler.handle.as_raw()),
        }
    }
    fn kind(&self) -> DescriptorKind {
        match self {
            Self::Buffer { .. } => DescriptorKind::StorageBuffer,
            Self::SampledImage { .. } => DescriptorKind::SampledImage,
            Self::StorageImage(_) => DescriptorKind::StorageImage,
            Self::Sampler(_) => DescriptorKind::Sampler,
        }
    }
    fn allocate(
        self,
        heap: &Arc<BindlessHeap>,
        uniform_upload: bool,
    ) -> Result<Arc<BindlessLease>> {
        match self {
            Self::Buffer {
                buffer,
                offset,
                size,
            } => {
                if uniform_upload {
                    heap.uniform_upload_buffer(buffer, offset, size)
                } else {
                    heap.storage_buffer(buffer, offset, size)
                }
            }
            Self::SampledImage { view, layout } => heap.sampled_image(view, layout),
            Self::StorageImage(view) => heap.storage_image(view),
            Self::Sampler(sampler) => heap.sampler(sampler),
        }
    }
}
fn expected(slot: &BindingSlot) -> Result<DescriptorKind> {
    Ok(match slot.kind {
        BindingKind::UniformBuffer | BindingKind::StorageBuffer => DescriptorKind::StorageBuffer,
        BindingKind::SampledImage => DescriptorKind::SampledImage,
        BindingKind::StorageImage => DescriptorKind::StorageImage,
        BindingKind::Sampler => DescriptorKind::Sampler,
        BindingKind::CombinedImageSampler => {
            return Err(Error::unsupported(
                "combined sampler input requires separate image and sampler bindings",
            ));
        }
    })
}

/// CPU preparation is fallible. The fixed common case stays in inline storage;
/// larger parameter tables are charged before any buffer growth.
pub struct BindlessParameters {
    heap: Arc<BindlessHeap>,
    abi: Arc<BindlessShaderAbi>,
    bytes: SmallVec<[u8; 512]>,
    written: SmallVec<[bool; 128]>,
    leases: SmallVec<[Arc<BindlessLease>; 16]>,
    cache: SmallVec<[(ResourceKey, Arc<BindlessLease>); 16]>,
    _memory: crate::raw::allocator::CpuMemoryReservation,
}
impl BindlessParameters {
    pub fn new(
        heap: Arc<BindlessHeap>,
        abi: Arc<BindlessShaderAbi>,
        original_push: &[u8],
    ) -> Result<Self> {
        abi.validate()
            .map_err(|error| Error::invalid_usage_with("bindless parameter ABI", error))?;
        if heap.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        if original_push.len() != abi.original_push_bytes as usize {
            return Err(Error::invalid_usage(
                "shader push parameter size does not match its ABI",
            ));
        }
        let count = abi
            .bindings
            .iter()
            .map(|slot| slot.parameter_count as usize)
            .sum::<usize>();
        let maximum_bytes = (std::mem::size_of::<Self>() as u64)
            .saturating_add(u64::from(abi.parameter_bytes) * 2)
            .saturating_add(
                (count + 1) as u64
                    * 2
                    * (std::mem::size_of::<bool>()
                        + std::mem::size_of::<Arc<BindlessLease>>()
                        + std::mem::size_of::<(ResourceKey, Arc<BindlessLease>)>())
                        as u64,
            );
        let memory = heap
            .device
            .memory
            .stats
            .reserve_cpu_bytes(maximum_bytes, crate::device::BudgetCategory::Live)?;
        let mut bytes = SmallVec::new();
        bytes
            .try_reserve(abi.parameter_bytes as usize)
            .map_err(|_| Error::OutOfHostMemory)?;
        bytes.resize(abi.parameter_bytes as usize, 0xff);
        bytes[..original_push.len()].copy_from_slice(original_push);
        let mut written = SmallVec::new();
        written
            .try_reserve(count)
            .map_err(|_| Error::OutOfHostMemory)?;
        written.resize(count, false);
        Ok(Self {
            heap,
            abi,
            bytes,
            written,
            leases: SmallVec::new(),
            cache: SmallVec::new(),
            _memory: memory,
        })
    }
    pub fn abi(&self) -> &BindlessShaderAbi {
        &self.abi
    }
    /// 与 write 的局部缓存相同的 key/上传角色规则；只计算尚未分配的普通资源。
    /// 外部 write_lease 与参数块由完整组调用者另计，不能假定它们进入局部缓存。
    pub fn resource_requirements<'a>(
        abi: &BindlessShaderAbi,
        resources: impl IntoIterator<Item = (BindingLocation, u32, &'a BindlessResource)>,
    ) -> Result<[usize; 4]> {
        let _trace = RecordSpan::new(RecordPhase::Requirements);
        record_count(RecordCount::RequirementsCalls, 1);
        let work = RecordTally::new([
            RecordCount::RequirementResources,
            RecordCount::RequirementUnique,
        ]);

        let mut keys = SmallVec::<[ResourceKey; 16]>::new();
        let mut counts = [0; 4];
        for (location, element, resource) in resources {
            work.add(0);
            let slot = abi.slot(location).ok_or_else(|| {
                Error::invalid_usage("binding is absent from the shader heap ABI")
            })?;
            if element >= slot.parameter_count || expected(slot)? != resource.kind() {
                return Err(Error::invalid_usage(
                    "resource requirement differs from the shader heap ABI",
                ));
            }
            let uniform_upload = slot.kind == BindingKind::UniformBuffer
                && matches!(resource, BindlessResource::Buffer {buffer, ..} if buffer.is_dynamic_constants_page());
            let key = resource.key(uniform_upload);
            if !keys.contains(&key) {
                keys.try_reserve(1).map_err(|_| Error::OutOfHostMemory)?;
                keys.push(key);
                work.add(1);
                counts[resource.kind() as usize] += 1;
            }
        }
        Ok(counts)
    }
    fn location(&self, location: BindingLocation, element: u32) -> Result<(usize, DescriptorKind)> {
        let slot = self
            .abi
            .slot(location)
            .ok_or_else(|| Error::invalid_usage("binding is absent from the shader heap ABI"))?;
        if element >= slot.parameter_count {
            return Err(Error::invalid_usage(
                "binding array element exceeds the shader heap ABI",
            ));
        }
        Ok((
            (slot.parameter_offset + element * 4) as usize,
            expected(slot)?,
        ))
    }
    pub fn write(
        &mut self,
        location: BindingLocation,
        element: u32,
        resource: BindlessResource,
    ) -> Result<()> {
        let (_, kind) = self.location(location, element)?;
        if resource.kind() != kind {
            return Err(Error::invalid_usage(
                "resource type does not match the shader heap ABI",
            ));
        }
        let uniform_upload = self
            .abi
            .slot(location)
            .is_some_and(|slot| slot.kind == BindingKind::UniformBuffer)
            && matches!(&resource, BindlessResource::Buffer {buffer, ..} if buffer.is_dynamic_constants_page());
        let key = resource.key(uniform_upload);
        let lease = match self.cache.iter().find(|(old, _)| *old == key) {
            Some((_, lease)) => {
                record_count(RecordCount::LocalCacheHits, 1);
                lease.clone()
            }
            None => {
                record_count(RecordCount::LocalCacheMisses, 1);
                self.cache
                    .try_reserve(1)
                    .map_err(|_| Error::OutOfHostMemory)?;
                // Per-draw constant uploads change every draw; every other
                // descriptor stays resident in its page across draws and frames.
                let lease = if uniform_upload {
                    resource.allocate(&self.heap, true)?
                } else {
                    let heap = self.heap.clone();
                    self.heap
                        .resident_lease(key, || resource.allocate(&heap, false))?
                };
                self.cache.push((key, lease.clone()));
                lease
            }
        };
        self.write_lease(location, element, lease)
    }
    pub fn write_lease(
        &mut self,
        location: BindingLocation,
        element: u32,
        lease: Arc<BindlessLease>,
    ) -> Result<()> {
        let (offset, kind) = self.location(location, element)?;
        if !Arc::ptr_eq(&lease.heap, &self.heap) || lease.index.kind != kind {
            return Err(Error::invalid_usage(
                "binding lease belongs to another heap or type",
            ));
        }
        let base = self.abi.original_push_bytes.div_ceil(4) as usize * 4;
        let written = (offset - base) / 4;
        if self.written[written] {
            return Err(Error::invalid_usage(
                "shader resource element was bound twice",
            ));
        }
        if !self.leases.iter().any(|old| Arc::ptr_eq(old, &lease)) {
            self.leases
                .try_reserve(1)
                .map_err(|_| Error::OutOfHostMemory)?;
            self.leases.push(lease.clone());
        }
        self.bytes[offset..offset + 4].copy_from_slice(&lease.index.index.to_le_bytes());
        self.written[written] = true;
        Ok(())
    }
    /// Reserve the immutable parameter slice through its owning allocator.
    /// The allocator's lock ends before host writes and descriptor creation.
    /// Fixed arrays must be fully bound. Runtime arrays may be sparse; the
    /// unsafe bind operation requires every shader-indexed element to be bound.
    pub fn finish(
        mut self,
        allocate: impl FnOnce(u64) -> Result<DynamicConstantsChunk>,
    ) -> Result<PreparedBindlessParameters> {
        let _trace = RecordSpan::new(RecordPhase::Finish);
        record_count(RecordCount::FinishCalls, 1);

        let base = self.abi.original_push_bytes.div_ceil(4) as usize * 4;
        for slot in &self.abi.bindings {
            if matches!(slot.array, BindingArray::Fixed(_))
                && (0..slot.parameter_count).any(|element| {
                    !self.written[(slot.parameter_offset as usize - base) / 4 + element as usize]
                })
            {
                return Err(Error::invalid_usage(format!(
                    "required shader resource element is unbound: {:?}",
                    slot.location,
                )));
            }
        }
        self.leases
            .try_reserve(1)
            .map_err(|_| Error::OutOfHostMemory)?;
        let span = RecordSpan::new(RecordPhase::FinishConstants);
        let chunk = allocate(self.bytes.len() as u64)?;
        chunk.write_bytes(&self.bytes)?;
        drop(span);
        let _span = RecordSpan::new(RecordPhase::FinishLease);
        let lease = self.heap.parameter_buffer(&chunk)?;
        let parameter_index = lease.index.index;
        let resource_lease_count = self.leases.len();
        self.leases.push(lease);
        Ok(PreparedBindlessParameters {
            heap: self.heap.clone(),
            abi: self.abi,
            bytes: self.bytes,
            parameter_index,
            resource_lease_count,
            leases: self.leases,
            _chunk: chunk,
            _memory: self._memory,
        })
    }
}

pub struct PreparedBindlessParameters {
    heap: Arc<BindlessHeap>,
    abi: Arc<BindlessShaderAbi>,
    bytes: SmallVec<[u8; 512]>,
    parameter_index: u32,
    resource_lease_count: usize,
    leases: SmallVec<[Arc<BindlessLease>; 16]>,
    _chunk: DynamicConstantsChunk,
    _memory: crate::raw::allocator::CpuMemoryReservation,
}
impl PreparedBindlessParameters {
    /// 只用于游标判断原参数是否仍属于当前真实页；数值下标本身不能证明同页。
    pub fn belongs_to(&self, heap: &Arc<BindlessHeap>) -> bool {
        Arc::ptr_eq(&self.heap, heap)
    }
    /// Reuse the same resource leases while allocating a fresh parameter slice.
    /// Earlier dispatches retain their immutable parameter block until completion.
    pub fn with_push(
        &self,
        original_push: &[u8],
        allocate: impl FnOnce(u64) -> Result<DynamicConstantsChunk>,
    ) -> Result<Self> {
        let _trace = RecordSpan::new(RecordPhase::WithPush);
        record_count(RecordCount::WithPushCalls, 1);

        let mut next = BindlessParameters::new(self.heap.clone(), self.abi.clone(), original_push)?;
        let base = self.abi.original_push_bytes.div_ceil(4) as usize * 4;
        next.bytes[base..].copy_from_slice(&self.bytes[base..]);
        for (index, flag) in next.written.iter_mut().enumerate() {
            let offset = base + index * 4;
            *flag = self.bytes[offset..offset + 4] != [0xff; 4];
        }
        next.leases
            .try_reserve(self.resource_lease_count)
            .map_err(|_| Error::OutOfHostMemory)?;
        next.leases
            .extend(self.leases[..self.resource_lease_count].iter().cloned());
        next.finish(allocate)
    }
    /// # Safety
    /// All resources the shader actually indexes must be bound and graph access
    /// declarations/layout barriers must describe those accesses.
    pub unsafe fn bind_pass(
        &self,
        context: &crate::rg::executor::PassContext<'_>,
        pipeline: Arc<VulkanPipeline>,
    ) -> Result<()> {
        self.validate_pipeline(&pipeline)?;
        unsafe {
            self.heap
                .bind_pass(context, pipeline.clone(), 0, &self.leases)?;
        }
        self.push(
            vk::CommandBuffer::from_raw(context.command.handle()),
            &pipeline,
        );
        Ok(())
    }
    /// # Safety
    /// The command must have matching resource barriers and a declared shader access contract.
    pub unsafe fn bind_command(
        &self,
        command: &mut crate::raw::command_buffer::VulkanCommandBuffer,
        pipeline: Arc<VulkanPipeline>,
    ) -> Result<()> {
        self.validate_pipeline(&pipeline)?;
        unsafe {
            self.heap
                .bind_command(command, pipeline.clone(), 0, &self.leases)?;
        }
        self.push(command.handle, &pipeline);
        Ok(())
    }
    fn validate_pipeline(&self, pipeline: &VulkanPipeline) -> Result<()> {
        if pipeline.layout.push_constants.len() != 1
            || pipeline.layout.push_constants[0].offset != 0
            || pipeline.layout.push_constants[0].size != 4
            || pipeline
                .layout
                .bindless_abi()
                .is_none_or(|abi| **abi != *self.abi)
        {
            return Err(Error::invalid_usage(
                "pipeline does not use the bindless parameter ABI",
            ));
        }
        Ok(())
    }
    fn push(&self, command: vk::CommandBuffer, pipeline: &VulkanPipeline) {
        let _trace = RecordSpan::new(RecordPhase::NativePush);

        unsafe {
            self.heap.device.core.device.cmd_push_constants(
                command,
                pipeline.layout.handle,
                pipeline.layout.push_constants[0].stage_flags,
                0,
                &self.parameter_index.to_le_bytes(),
            );
        }
        record_count(RecordCount::NativePushCalls, 1);
    }
}
use ash::vk::Handle;
