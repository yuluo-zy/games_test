//! GPU-only lowering of indexed indirect count on devices that do not support
//! DrawIndirectCount. This is a platform capability adapter, not game logic.
//! The caller declares scratch through its existing graph/session and uses its
//! existing bindless heap, pipeline compiler and dynamic-constants pool.
use crate::{
    Error, Result,
    bindless::{
        BindlessParameters, BindlessResource, BindlessShaderAbi,
        BindlessShaderPackage,
    },
    device::{Device, Feature},
    pipeline::{AsyncPipelineCompiler, PipelineTicket},
    raw::{buffer::BufferDesc, dynamic_constants::DynamicConstants},
    rg::{
        graph::{Descriptor, RenderGraph},
        resource::{AccessRange, ResourceHandle},
    },
};
use ash::vk::{self, Handle};
use parking_lot::Mutex;
use shader_pipeline::{
    compiled::{CompiledShaderPipeline, CompiledShaderStage, ShaderStage},
    reflection::BindingLocation,
};
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IndexedCountRequest {
    pub command_offset: u64,
    pub command_stride: u32,
    pub count_offset: u64,
    pub maximum_draws: u32,
}
impl IndexedCountRequest {
    /// Exact tight VkDrawIndexedIndirectCommand output, no inactive tail bytes
    /// are inherited from either the original or a previous scratch allocation.
    pub fn scratch_desc(self) -> Result<BufferDesc> {
        if self.maximum_draws == 0 {
            return Err(Error::invalid_usage(
                "zero maximum count has no scratch dispatch; caller records a zero-count draw",
            ));
        }
        Ok(BufferDesc::new_gpu_only(
            u64::from(self.maximum_draws) * 20,
            vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::INDIRECT_BUFFER,
        ))
    }
    fn input_span(self) -> Result<u64> {
        if !self.command_offset.is_multiple_of(4)
            || !self.count_offset.is_multiple_of(4)
            || self.command_stride < 20
            || !self.command_stride.is_multiple_of(4)
        {
            return Err(Error::invalid_usage(
                "indexed count command offset/stride is invalid",
            ));
        }
        if self.maximum_draws == 0 {
            return Ok(0);
        }
        u64::from(self.maximum_draws - 1)
            .checked_mul(u64::from(self.command_stride))
            .and_then(|v| v.checked_add(20))
            .ok_or_else(|| Error::invalid_usage("indexed count command span overflows"))
    }
    fn push_bytes(self, command_base: u64, count_base: u64) -> Result<[u8; 16]> {
        let values = [
            u32::try_from((self.command_offset - command_base) / 4).map_err(|_| {
                Error::unsupported("indexed count local command address exceeds32bits")
            })?,
            self.command_stride / 4,
            u32::try_from((self.count_offset - count_base) / 4).map_err(|_| {
                Error::unsupported("indexed count local counter address exceeds32bits")
            })?,
            self.maximum_draws,
        ];
        let mut bytes = [0; 16];
        for (dst, value) in bytes.chunks_exact_mut(4).zip(values) {
            dst.copy_from_slice(&value.to_le_bytes());
        }
        Ok(bytes)
    }
}

pub struct IndexedCountAdapter {
    device: Arc<Device>,
    pages: Arc<crate::bindless::BindlessPagePool>,
    abi: Arc<BindlessShaderAbi>,
    pipeline: PipelineTicket,
}
impl IndexedCountAdapter {
    pub fn new(
        device: Arc<Device>,
        pages: Arc<crate::bindless::BindlessPagePool>,
        compiler: &AsyncPipelineCompiler,
    ) -> Result<Arc<Self>> {
        let heap = pages.template();
        if !Arc::ptr_eq(&device, &heap.layout.device) {
            return Err(Error::invalid_usage(
                "indirect count adapter heap belongs to another device",
            ));
        }
        let words: &[u32] = include!("indirect_count_spirv.rs");
        let stage = CompiledShaderStage {
            stage: ShaderStage::Compute,
            entry_point: "main".to_owned(),
            spirv: words.to_vec(),
        };
        let reflection = shader_compiler::reflect::generate_reflection(std::slice::from_ref(
            &stage,
        ))
        .map_err(|e| Error::invalid_usage(format!("indirect count adapter reflection: {e}")))?;
        let input = CompiledShaderPipeline {
            stages: vec![stage],
            reflection,
            source_dependencies: vec![],
            trailing_bytes: vec![],
        };
        let package = BindlessShaderPackage::new(&heap, &input, 1)?;
        let pipeline = compiler.defer_compute_spirv(
            package.stages[0].spirv_bytes(),
            c"main".into(),
            package.layout,
        )?;
        Ok(Arc::new(Self {
            device,
            pages,
            abi: package.abi,
            pipeline,
        }))
    }
    /// The enclosing host includes this ticket in its ordinary explicit prewarm.
    /// The GPU callback never blocks for compilation.
    pub fn ticket(&self) -> PipelineTicket {
        self.pipeline.clone()
    }

    pub fn add_pass(
        self: &Arc<Self>,
        graph: &mut RenderGraph,
        commands: ResourceHandle,
        counter: ResourceHandle,
        scratch: ResourceHandle,
        request: IndexedCountRequest,
        constants: Arc<Mutex<DynamicConstants>>,
    ) -> Result<()> {
        let scratch_desc = request.scratch_desc()?;
        if request.maximum_draws > self.device.max_draw_indirect_count() {
            return Err(Error::invalid_usage(
                "indexed count maximum exceeds device draw limit",
            ));
        }
        if request.maximum_draws > 1 {
            self.device
                .caps()
                .require_enabled(Feature::MultiDrawIndirect)?;
        }
        let limits = self.device.limits();
        if request.maximum_draws.div_ceil(64) > limits.max_compute_work_group_count[0] {
            return Err(Error::unsupported(
                "indexed count adaptation exceeds device dispatch limit",
            ));
        }
        if commands == scratch || counter == scratch {
            return Err(Error::invalid_usage(
                "indexed count scratch aliases an input",
            ));
        }
        let size = |handle: ResourceHandle| -> Result<u64> {
            graph.buffer_id(handle)?; // checks actual graph generation and kind
            match graph.declaration().resources[handle.raw() as usize].descriptor {
                Descriptor::Buffer(desc) => Ok(desc.word_at_00),
                _ => Err(Error::invalid_usage(
                    "indexed count resource is not a buffer",
                )),
            }
        };
        let source_size = size(commands)?;
        let counter_size = size(counter)?;
        let scratch_size = size(scratch)?;
        let source_span = request.input_span()?;
        if request
            .command_offset
            .checked_add(source_span)
            .is_none_or(|end| end > source_size)
            || request
                .count_offset
                .checked_add(4)
                .is_none_or(|end| end > counter_size)
            || scratch_desc.size > scratch_size
        {
            return Err(Error::invalid_usage(
                "indexed count adaptation exceeds actual graph buffer range",
            ));
        }
        // Descriptor offsets obey the actual device alignment. The few prefix
        // bytes included by alignment are conservatively declared as reads;
        // shader word offsets remain relative to these very same ranges.
        let alignment = limits.min_storage_buffer_offset_alignment.max(4);
        let command_base = request.command_offset / alignment * alignment;
        let count_base = request.count_offset / alignment * alignment;
        let command_range = (request.command_offset - command_base) + source_span;
        let count_range = (request.count_offset - count_base) + 4;
        if [command_range, count_range, scratch_desc.size]
            .iter()
            .any(|size| *size > u64::from(limits.max_storage_buffer_range))
        {
            return Err(Error::unsupported(
                "indexed count descriptor exceeds device storage range",
            ));
        }
        let push = request.push_bytes(command_base, count_base)?;
        let mut pass = graph.add_pass("platform indexed indirect count adaptation");
        pass.add_resource_access_range(
            commands,
            0x1a,
            1,
            AccessRange::Buffer {
                offset: command_base,
                size: command_range,
            },
        )?;
        pass.add_resource_access_range(
            counter,
            0x1a,
            1,
            AccessRange::Buffer {
                offset: count_base,
                size: count_range,
            },
        )?;
        pass.add_resource_access_range(
            scratch,
            0x37,
            0,
            AccessRange::Buffer {
                offset: 0,
                size: scratch_desc.size,
            },
        )?;
        let adapter = self.clone();
        pass.render_with_result_context(move |ctx| {
            let managed = ctx.managed.ok_or_else(|| {
                Error::invalid_usage("indexed count adaptation has no graph owners")
            })?;
            let source = managed.buffer_for(commands.raw()).map_err(Error::from)?;
            let count = managed.buffer_for(counter.raw()).map_err(Error::from)?;
            let target = managed.buffer_for(scratch.raw()).map_err(Error::from)?;
            let resources = [
                BindlessResource::Buffer {
                    buffer: source.shared(),
                    offset: command_base,
                    size: command_range,
                },
                BindlessResource::Buffer {
                    buffer: count.shared(),
                    offset: count_base,
                    size: count_range,
                },
                BindlessResource::Buffer {
                    buffer: target.shared(),
                    offset: 0,
                    size: scratch_desc.size,
                },
            ];
            let mut required = BindlessParameters::resource_requirements(
                &adapter.abi,
                resources.iter().enumerate().map(|(binding, resource)| {
                    (
                        BindingLocation {
                            set: 0,
                            binding: binding as u32,
                        },
                        0,
                        resource,
                    )
                }),
            )?;
            required[2] += 1;
            let mut cursor = adapter.pages.cursor();
            let prepared = cursor.prepare(|group| {
                    group.require_slots(required)?;
                    let mut parameters = group.parameters(adapter.abi.clone(), &push)?;
                    for (binding, resource) in resources.iter().enumerate() {
                        parameters.write(
                            BindingLocation {
                                set: 0,
                                binding: binding as u32,
                            },
                            0,
                            resource.clone(),
                        )?;
                    }
                    parameters.finish(|size| constants.lock().allocate(size)).map_err(Into::into)
            })?;
            let pipeline = adapter.pipeline.try_get()?;
            unsafe {
                let command = vk::CommandBuffer::from_raw(ctx.command.handle());
                pipeline.bind_pipeline(command);
                prepared.bind_pass(&ctx, pipeline)?;
                adapter.device.core.device.cmd_dispatch(
                    command,
                    request.maximum_draws.div_ceil(64),
                    1,
                    1,
                );
            }
            Ok(())
        });
        Ok(())
    }
}
