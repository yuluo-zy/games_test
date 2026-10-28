//! Typed resource and parameter bindings for the production shader heap ABI.
use super::{BindlessHeap, BindlessLease, DescriptorKind};
use crate::profile::{RecordCount, RecordPhase, RecordSpan, RecordTally, record_count};
use crate::{Error, Result};
use crate::pipeline::shader::{BindlessShaderAbi, PipelineLayout, VulkanPipeline};
use crate::resource::buffer::Buffer;
use crate::resource::dynamic_constants::DynamicConstantsChunk;
use crate::resource::sampler::Sampler;
use crate::resource::texture::TextureView;
use ash::vk;
use shader_compiler::bindless::BindingSlot;
use shader_pipeline::{
    compiled::{CompiledShaderPipeline, CompiledShaderStage},
    reflection::{BindingArray, BindingKind, BindingLocation},
};
use smallvec::SmallVec;
use std::sync::Arc;
pub(super) mod cpu;

/// A lowered package uses the renderer's shared layout. Input serialization is
/// an asset format, rather than a second runtime descriptor implementation.
pub struct BindlessShaderPackage {
    pub abi: Arc<BindlessShaderAbi>,
    pub stages: Vec<CompiledShaderStage>,
    pub layout: Arc<PipelineLayout>,
    _stage_memory: crate::accounting::CpuMemoryReservation,
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
            .reserve_cpu_bytes(maximum_lowering_bytes, crate::accounting::BudgetCategory::Live)?;
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
            .reserve_cpu_bytes(abi_bytes, crate::accounting::BudgetCategory::Live)?;
        let parameter_count = abi.bindings.iter().try_fold(0usize, |count, slot|
            count.checked_add(slot.parameter_count as usize).ok_or(Error::OutOfHostMemory))?;
        let entry_bytes = std::mem::size_of::<bool>() + std::mem::size_of::<Arc<BindlessLease>>()
            + std::mem::size_of::<(ResourceKey, Arc<BindlessLease>)>();
        let parameter_bytes = (abi.parameter_bytes as usize).checked_mul(2).ok_or(Error::OutOfHostMemory)?;
        let parameter_cpu_bytes = parameter_count.checked_add(1)
            .and_then(|n| n.checked_mul(2 * entry_bytes))
            .and_then(|n| n.checked_add(parameter_bytes))
            .and_then(|n| n.checked_add(std::mem::size_of::<BindlessParameters>()))
            .ok_or(Error::OutOfHostMemory)?;
        let abi = Arc::new(BindlessShaderAbi {
            parameter_count,
            parameter_cpu_bytes,
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
    _memory: cpu::ParameterCpuCredit,
}
impl BindlessParameters {
    pub(super) fn from_credit(
        heap: Arc<BindlessHeap>,
        abi: Arc<BindlessShaderAbi>,
        original_push: &[u8],
        credit: impl FnOnce(usize) -> Result<cpu::ParameterCpuCredit>,
    ) -> Result<Self> {
        // The package lowerer admitted this immutable ABI before publishing its
        // private owner. Only this draw's push bytes need checking here.
        if heap.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        if original_push.len() != abi.original_push_bytes as usize {
            return Err(Error::invalid_usage(
                "shader push parameter size does not match its ABI",
            ));
        }
        let count = abi.parameter_count;
        let memory = credit(abi.parameter_cpu_bytes)?;
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
    fn location(&self, location: BindingLocation, element: u32) -> Result<(usize, usize, DescriptorKind, bool)> {
        let slot = self
            .abi
            .slot(location)
            .ok_or_else(|| Error::invalid_usage("binding is absent from the shader heap ABI"))?;
        if element >= slot.parameter_count {
            return Err(Error::invalid_usage(
                "binding array element exceeds the shader heap ABI",
            ));
        }
        let offset = (slot.parameter_offset + element * 4) as usize;
        let base = self.abi.original_push_bytes.div_ceil(4) as usize * 4;
        let written = (offset - base) / 4;
        if self.written[written] {
            return Err(Error::invalid_usage("shader resource element was bound twice"));
        }
        Ok((offset, written, expected(slot)?, slot.kind == BindingKind::UniformBuffer))
    }
    pub fn write(
        &mut self,
        location: BindingLocation,
        element: u32,
        resource: BindlessResource,
    ) -> Result<()> {
        let (offset, written, kind, uniform_slot) = self.location(location, element)?;
        if resource.kind() != kind {
            return Err(Error::invalid_usage(
                "resource type does not match the shader heap ABI",
            ));
        }
        let uniform_upload = uniform_slot
            && matches!(&resource, BindlessResource::Buffer {buffer, ..} if buffer.is_dynamic_constants_page());
        let key = resource.key(uniform_upload);
        // `commit_lease` borrows `self` mutably, so the cached lease is cloned out first.
        if let Some(lease) = self.cache.iter().find(|(old, _)| *old == key).map(|(_, lease)| Arc::clone(lease)) {
            record_count(RecordCount::LocalCacheHits, 1);
            return self.commit_lease(offset, written, &lease);
        }
        record_count(RecordCount::LocalCacheMisses, 1);
        self.cache.try_reserve(1).map_err(|_| Error::OutOfHostMemory)?;
        let lease = if uniform_upload {
            resource.allocate(&self.heap, true)?
        } else {
            let heap = self.heap.clone();
            self.heap.resident_lease(key, || resource.allocate(&heap, false))?
        };
        self.commit_lease(offset, written, &lease)?;
        self.cache.push((key, lease));
        Ok(())
    }
    pub fn write_lease(
        &mut self,
        location: BindingLocation,
        element: u32,
        lease: Arc<BindlessLease>,
    ) -> Result<()> {
        let (offset, written, kind, _) = self.location(location, element)?;
        if !Arc::ptr_eq(&lease.slot.heap, &self.heap) || lease.slot.index.kind != kind {
            return Err(Error::invalid_usage(
                "binding lease belongs to another heap or type",
            ));
        }
        self.commit_lease(offset, written, &lease)
    }
    fn commit_lease(&mut self, offset: usize, written: usize, lease: &Arc<BindlessLease>) -> Result<()> {
        if !self.leases.iter().any(|old| Arc::ptr_eq(old, lease)) {
            self.leases
                .try_reserve(1)
                .map_err(|_| Error::OutOfHostMemory)?;
            self.leases.push(lease.clone());
        }
        self.bytes[offset..offset + 4].copy_from_slice(&lease.slot.index.index.to_le_bytes());
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
        let parameter_index = lease.slot.index.index;
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
    pub(crate) heap: Arc<BindlessHeap>,
    abi: Arc<BindlessShaderAbi>,
    bytes: SmallVec<[u8; 512]>,
    parameter_index: u32,
    resource_lease_count: usize,
    pub(crate) leases: SmallVec<[Arc<BindlessLease>; 16]>,
    _chunk: DynamicConstantsChunk,
    _memory: cpu::ParameterCpuCredit,
}
impl PreparedBindlessParameters {
    /// Exact original payload; reuse is valid while this callback's pipeline
    /// and resource group remain unchanged.
    pub fn matches_original_push(&self, push: &[u8]) -> bool {
        self.bytes[..self.abi.original_push_bytes as usize] == *push
    }

    /// 只用于游标判断原参数是否仍属于当前真实页；数值下标本身不能证明同页。
    pub fn belongs_to(&self, heap: &Arc<BindlessHeap>) -> bool {
        Arc::ptr_eq(&self.heap, heap)
    }
    /// Reuse the same resource leases while allocating a fresh parameter slice.
    /// Earlier dispatches retain their immutable parameter block until completion.
    pub(super) fn with_push_credit(
        &self,
        original_push: &[u8],
        allocate: impl FnOnce(u64) -> Result<DynamicConstantsChunk>,
        credit: impl FnOnce(usize) -> Result<cpu::ParameterCpuCredit>,
    ) -> Result<Self> {
        let _trace = RecordSpan::new(RecordPhase::WithPush);
        record_count(RecordCount::WithPushCalls, 1);

        let mut next = BindlessParameters::from_credit(self.heap.clone(), self.abi.clone(), original_push, credit)?;
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
    /// The command must have matching resource barriers and a declared shader access contract.
    pub unsafe fn bind_command(
        &self,
        command: &mut crate::command::command_buffer::VulkanCommandBuffer,
        pipeline: &Arc<VulkanPipeline>,
    ) -> Result<()> {
        self.validate_pipeline(pipeline)?;
        unsafe {
            self.heap
                .bind_command(command, pipeline, 0, &self.leases)?;
        }
        self.push(command.handle, pipeline);
        Ok(())
    }
    pub(crate) fn validate_pipeline(&self, pipeline: &VulkanPipeline) -> Result<()> {
        if pipeline.layout.push_constants.len() != 1
            || pipeline.layout.push_constants[0].offset != 0
            || pipeline.layout.push_constants[0].size != 4
            || pipeline
                .layout
                .bindless_abi()
                .is_none_or(|abi| !Arc::ptr_eq(abi, &self.abi) && **abi != *self.abi)
        {
            return Err(Error::invalid_usage(
                "pipeline does not use the bindless parameter ABI",
            ));
        }
        Ok(())
    }
    pub(crate) fn push(&self, command: vk::CommandBuffer, pipeline: &VulkanPipeline) {
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
