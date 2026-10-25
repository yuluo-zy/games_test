mod gpu_test_support;
use ash::vk;
use rhapsody::{
    bindless::BindlessConfig, bindless::BindlessHeap, bindless::BindlessParameters,
    bindless::BindlessResource, bindless::BindlessShaderPackage, device::Device,
    device::DeviceConfig, pipeline::AsyncPipelineCompiler, pipeline::CompilerConfig,
    pipeline::HotPipeline, raw::buffer::Buffer, raw::buffer::BufferDesc,
    raw::command_buffer::VulkanCommandPool, raw::dynamic_constants::DynamicConstants,
    raw::shader::PipelineCache,
};
use shader_pipeline::{
    compiled::CompiledShaderPipeline, compiled::CompiledShaderStage, compiled::ShaderStage,
    reflection::BindingLocation,
};
use std::{sync::Arc, time::Duration};

fn input() -> CompiledShaderPipeline {
    use rspirv::{
        binary::Assemble,
        dr::{Builder, Operand},
        spirv::*,
    };
    let mut b = Builder::new();
    b.set_version(1, 3);
    b.capability(Capability::Shader);
    b.memory_model(AddressingModel::Logical, MemoryModel::GLSL450);
    let void = b.type_void();
    let function = b.type_function(void, []);
    let uint = b.type_int(32, 0);
    let sint = b.type_int(32, 1);
    let zero = b.constant_u32(uint, 0);
    let two = b.constant_u32(uint, 2);
    let one = b.constant_u32(sint, 1);
    // Intentionally share one struct across storage, UBO and push declarations:
    // lowering must clone its parameter schema without changing buffer layouts.
    let block = b.type_struct([uint]);
    b.name(block, "Data");
    b.member_name(block, 0, "value");
    b.decorate(block, Decoration::Block, []);
    b.member_decorate(block, 0, Decoration::Offset, [Operand::LiteralInt32(0)]);
    let array = b.type_array(block, two);
    let array_pointer = b.type_pointer(None, StorageClass::StorageBuffer, array);
    let buffers = b.variable(array_pointer, None, StorageClass::StorageBuffer, None);
    b.name(buffers, "outputs");
    b.decorate(
        buffers,
        Decoration::DescriptorSet,
        [Operand::LiteralInt32(0)],
    );
    b.decorate(buffers, Decoration::Binding, [Operand::LiteralInt32(5)]);
    let uniform_pointer = b.type_pointer(None, StorageClass::Uniform, block);
    let uniform = b.variable(uniform_pointer, None, StorageClass::Uniform, None);
    b.name(uniform, "constants");
    b.decorate(
        uniform,
        Decoration::DescriptorSet,
        [Operand::LiteralInt32(1)],
    );
    b.decorate(uniform, Decoration::Binding, [Operand::LiteralInt32(3)]);
    let push_pointer = b.type_pointer(None, StorageClass::PushConstant, block);
    let push = b.variable(push_pointer, None, StorageClass::PushConstant, None);
    b.name(push, "push");
    let storage_scalar = b.type_pointer(None, StorageClass::StorageBuffer, uint);
    let uniform_scalar = b.type_pointer(None, StorageClass::Uniform, uint);
    let push_scalar = b.type_pointer(None, StorageClass::PushConstant, uint);
    let main = b
        .begin_function(void, None, FunctionControl::NONE, function)
        .unwrap();
    b.begin_block(None).unwrap();
    let uniform_field = b
        .access_chain(uniform_scalar, None, uniform, [zero])
        .unwrap();
    let from_uniform = b.load(uint, None, uniform_field, None, []).unwrap();
    let push_field = b.access_chain(push_scalar, None, push, [zero]).unwrap();
    let from_push = b.load(uint, None, push_field, None, []).unwrap();
    let value = b.i_add(uint, None, from_uniform, from_push).unwrap();
    let out = b
        .access_chain(storage_scalar, None, buffers, [one, zero])
        .unwrap();
    b.store(out, value, None, []).unwrap();
    b.ret().unwrap();
    b.end_function().unwrap();
    b.entry_point(ExecutionModel::GLCompute, main, "main", []);
    b.execution_mode(main, ExecutionMode::LocalSize, [1, 1, 1]);
    let stage = CompiledShaderStage {
        stage: ShaderStage::Compute,
        entry_point: "main".into(),
        spirv: b.module().assemble(),
    };
    let reflection =
        shader_compiler::reflect::generate_reflection(std::slice::from_ref(&stage)).unwrap();
    CompiledShaderPipeline {
        stages: vec![stage],
        reflection,
        source_dependencies: vec![],
        trailing_bytes: vec![],
    }
}

#[test]
fn lower_preserves_shared_types_and_bounds_an_independent_array_index_table() {
    let input = input();
    let (abi, stages) = shader_compiler::bindless::lower_pipeline(&input).unwrap();
    assert_eq!(abi.original_push_bytes, 4);
    assert_eq!(abi.parameter_bytes, 16);
    assert_eq!(abi.bindings[0].parameter_count, 2);
    assert_eq!(abi.bindings[1].parameter_count, 1);
    let actual = rhapsody::raw::shader::reflect_spirv_entries(&stages[0].spirv).unwrap();
    assert_eq!(actual.len(), 1);
    assert!(
        actual[0]
            .descriptors
            .iter()
            .all(|binding| binding.set == 0 && binding.binding == 2)
    );
    assert!(shader_compiler::bindless::lower_pipeline_with_limit(&input, 0).is_err());
    let mut malformed = input.stages[0].clone();
    malformed.spirv[3] = u32::MAX;
    assert!(shader_compiler::bindless::lower_stage(&malformed, &abi).is_err());
    let mut malformed_abi = abi.clone();
    malformed_abi.bindings[0].parameter_offset += 4;
    assert!(malformed_abi.validate().is_err());
}

#[test]
fn real_uniform_push_and_descriptor_array_execute_through_one_shared_heap_set() {
    let Some(device) = gpu_test_support::initialize(
        "bindless ABI",
        Device::new_headless(&DeviceConfig::headless()),
    ) else {
        return;
    };
    let heap = BindlessHeap::new(
        device.clone(),
        BindlessConfig {
            sampled_images: 4,
            storage_images: 4,
            storage_buffers: 16,
            samplers: 4,
        },
    )
    .unwrap();
    let package = BindlessShaderPackage::new(&heap, &input(), 64).unwrap();
    let compiler = AsyncPipelineCompiler::new(
        Arc::new(PipelineCache::new(device.clone(), &[]).unwrap()),
        CompilerConfig::default(),
    )
    .unwrap();
    let pipeline = compiler
        .request_compute_spirv(
            package.stages[0].spirv_bytes(),
            c"main".into(),
            package.layout,
        )
        .unwrap()
        .wait(Duration::from_secs(10))
        .unwrap();
    let buffer = || {
        Arc::new(
            Buffer::new(
                device.clone(),
                &BufferDesc::new_cpu_to_gpu(
                    4,
                    vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::UNIFORM_BUFFER,
                ),
            )
            .unwrap(),
        )
    };
    let first = buffer();
    let second = buffer();
    let uniform = buffer();
    first.write_bytes(0, &123u32.to_le_bytes()).unwrap();
    second.write_bytes(0, &[0; 4]).unwrap();
    uniform.write_bytes(0, &25u32.to_le_bytes()).unwrap();
    let mut constants = DynamicConstants::new(device.clone()).unwrap();
    let pool = VulkanCommandPool::new(device.clone()).unwrap();
    let pages = rhapsody::bindless::BindlessPagePool::new(heap.clone()).unwrap();
    let mut cursor = pages.cursor();
    for value in 17..33u32 {
        let prepared = cursor.prepare(|group| {
        group.require_slots([0, 0, 4, 0])?;
        let mut bindings = group.parameters(package.abi.clone(), &value.to_le_bytes())?;
        for (location, element, owner) in [
            (BindingLocation { set: 0, binding: 5 }, 0, &first),
            (BindingLocation { set: 0, binding: 5 }, 1, &second),
            (BindingLocation { set: 1, binding: 3 }, 0, &uniform),
        ] {
            bindings
                .write(
                    location,
                    element,
                    BindlessResource::Buffer {
                        buffer: owner.clone(),
                        offset: 0,
                        size: 4,
                    },
                )
                .unwrap();
            if location.binding == 5 && element == 0 {
                let live = heap.stats().live;
                for _ in 0..64 {
                    assert!(matches!(bindings.write(location, element, BindlessResource::Buffer {
                        buffer: second.clone(), offset: 0, size: 4,
                    }), Err(rhapsody::Error::InvalidUsage { .. })));
                }
                assert_eq!(heap.stats().live, live, "rejected duplicates cannot allocate/cache another resource");
            }
        }
        bindings.finish(|size| constants.allocate(size)).map_err(Into::into)
        }).unwrap();
        let changed_push = cursor.prepare(|group| group.with_push(&prepared, &(value + 100).to_le_bytes(),
            |size| constants.allocate(size))?.ok_or_else(|| rhapsody::Error::invalid_usage("test group unexpectedly changed page").into())).unwrap();
        let mut command = pool.allocate().unwrap();
        command.begin().unwrap();
        unsafe {
            pipeline.bind_pipeline(command.raw_handle());
            prepared
                .bind_command(&mut command, pipeline.clone())
                .unwrap();
            device
                .raw_device()
                .cmd_dispatch(command.raw_handle(), 1, 1, 1);
            device.raw_device().cmd_pipeline_barrier(
                command.raw_handle(),
                vk::PipelineStageFlags::COMPUTE_SHADER,
                vk::PipelineStageFlags::COMPUTE_SHADER,
                vk::DependencyFlags::empty(),
                &[],
                &[vk::BufferMemoryBarrier::default()
                    .buffer(second.raw_handle())
                    .offset(0)
                    .size(4)
                    .src_access_mask(vk::AccessFlags::SHADER_WRITE)
                    .dst_access_mask(vk::AccessFlags::SHADER_WRITE)
                    .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                    .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)],
                &[],
            );
            changed_push
                .bind_command(&mut command, pipeline.clone())
                .unwrap();
            device
                .raw_device()
                .cmd_dispatch(command.raw_handle(), 1, 1, 1);
            device.raw_device().cmd_pipeline_barrier(
                command.raw_handle(),
                vk::PipelineStageFlags::COMPUTE_SHADER,
                vk::PipelineStageFlags::HOST,
                vk::DependencyFlags::empty(),
                &[],
                &[vk::BufferMemoryBarrier::default()
                    .buffer(second.raw_handle())
                    .offset(0)
                    .size(4)
                    .src_access_mask(vk::AccessFlags::SHADER_WRITE)
                    .dst_access_mask(vk::AccessFlags::HOST_READ)
                    .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                    .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)],
                &[],
            );
        }
        command.end().unwrap();
        let pending = command.submit().unwrap();
        drop((prepared, changed_push));
        device
            .wait(
                &rhapsody::sync::SyncSet::from_point(pending.point),
                10_000_000_000,
            )
            .unwrap();
        drop(pending);
        let mut actual = [0; 4];
        second.read_bytes(0, &mut actual).unwrap();
        assert_eq!(u32::from_le_bytes(actual), value + 125);
        first.read_bytes(0, &mut actual).unwrap();
        assert_eq!(u32::from_le_bytes(actual), 123);
        device.collect_garbage();
        heap.collect().unwrap();
    }
    assert_eq!(compiler.stats().compiled, 1);
    assert_eq!(device.retirement_stats().idle_waits, 0);
}

#[test]
fn complete_groups_page_without_changing_the_abi_or_immutable_push_values() {
    use rhapsody::bindless::{BindlessPagePool, DescriptorKind};
    let Some(device) = gpu_test_support::initialize(
        "paged bindless ABI",
        Device::new_headless(&DeviceConfig::headless()),
    ) else {
        return;
    };
    let heap = BindlessHeap::new(
        device.clone(),
        BindlessConfig {
            sampled_images: 1,
            storage_images: 1,
            storage_buffers: 5,
            samplers: 1,
        },
    )
    .unwrap();
    let pages = BindlessPagePool::new(heap.clone()).unwrap();
    let package = BindlessShaderPackage::new(&heap, &input(), 64).unwrap();
    let compiler = AsyncPipelineCompiler::new(
        Arc::new(PipelineCache::new(device.clone(), &[]).unwrap()),
        CompilerConfig::default(),
    )
    .unwrap();
    let pipeline = compiler
        .request_compute_spirv(
            package.stages[0].spirv_bytes(),
            c"main".into(),
            package.layout,
        )
        .unwrap()
        .wait(Duration::from_secs(10))
        .unwrap();
    let buffer = || {
        Arc::new(
            Buffer::new(
                device.clone(),
                &BufferDesc::new_cpu_to_gpu(
                    4,
                    vk::BufferUsageFlags::STORAGE_BUFFER
                        | vk::BufferUsageFlags::UNIFORM_BUFFER
                        | vk::BufferUsageFlags::TRANSFER_SRC,
                ),
            )
            .unwrap(),
        )
    };
    let owners = [buffer(), buffer(), buffer()];
    owners[0].write_bytes(0, &123u32.to_le_bytes()).unwrap();
    owners[1].write_bytes(0, &[0; 4]).unwrap();
    owners[2].write_bytes(0, &25u32.to_le_bytes()).unwrap();
    let locations = [
        (BindingLocation { set: 0, binding: 5 }, 0),
        (BindingLocation { set: 0, binding: 5 }, 1),
        (BindingLocation { set: 1, binding: 3 }, 0),
    ];
    let resources: Vec<_> = owners
        .iter()
        .map(|buffer| BindlessResource::Buffer {
            buffer: buffer.clone(),
            offset: 0,
            size: 4,
        })
        .collect();
    let mut required = BindlessParameters::resource_requirements(
        &package.abi,
        locations
            .iter()
            .zip(&resources)
            .map(|((location, element), resource)| (*location, *element, resource)),
    )
    .unwrap();
    assert_eq!(required, [0, 0, 3, 0]);
    required[2] += 1;
    let mut cursor = pages.cursor();
    // 超过新页容量的完整组立即报告逐类需求，不创建页或分配半组。
    let oversized = cursor.prepare(|group| {
        group.require_slots([0, 0, 6, 0])?;
        unreachable!("oversized group must fail admission")
    });
    assert!(matches!(oversized, Err(rhapsody::Error::Unsupported(_))));
    assert_eq!(pages.page_count(), 1);
    // 实际分配后的普通错误也必须原样退出；未提交租约立即归还原页。
    let failed = cursor.prepare(|group| {
        group.require_slots([0, 0, 1, 0])?;
        let _lease = group.heap().storage_buffer(owners[0].clone(), 0, 4)?;
        Err(rhapsody::Error::invalid_usage("intentional group preparation failure").into())
    });
    assert!(matches!(failed, Err(rhapsody::Error::InvalidUsage { .. })));
    assert_eq!(heap.stats().table(DescriptorKind::StorageBuffer).free, 5);
    let mut constants = DynamicConstants::new(device.clone()).unwrap();
    let mut prepared: Vec<rhapsody::bindless::PreparedBindlessParameters> = Vec::new();
    // Recording workers may keep their cursors alive together. Their presence
    // must not strand free slots and grow one physical page per cursor.
    let mut cursors = [cursor, pages.cursor(), pages.cursor()];
    for (index, (cursor, value)) in cursors.iter_mut().zip([17u32, 18, 19]).enumerate() {
        if index == 2 {
            pages.advance_frame();
            *cursor = pages.cursor();
        }
        let previous = prepared.last();
        let next = cursor
            .prepare(|group| {
                if let Some(previous) = previous
                    && let Some(prepared) = group.with_push(previous, &value.to_le_bytes(), |size| constants.allocate(size))? {
                    return Ok(prepared);
                }
                group.require_slots(required)?;
                let mut bindings = group.parameters(
                    package.abi.clone(),
                    &value.to_le_bytes(),
                )?;
                for ((location, element), resource) in locations.iter().zip(&resources) {
                    bindings.write(*location, *element, resource.clone())?;
                }
                bindings
                    .finish(|size| constants.allocate(size))
                    .map_err(Into::into)
            })
            .unwrap();
        prepared.push(next);
    }
    assert_eq!(pages.page_count(), 2);
    assert!(prepared[0].belongs_to(&heap));
    assert!(prepared[1].belongs_to(&heap));
    assert!(!prepared[2].belongs_to(&heap));
    // A valid foreign set makes an omitted interop invalidation produce a
    // different value, rather than accessing uninitialized descriptors.
    let foreign_parameters = Arc::new(Buffer::new(device.clone(),
        &BufferDesc::new_cpu_to_gpu(16, vk::BufferUsageFlags::STORAGE_BUFFER)).unwrap());
    let foreign_words: Vec<u8> = [99u32, 0, 1, 2].into_iter()
        .flat_map(u32::to_le_bytes).collect();
    foreign_parameters.write_bytes(0, &foreign_words).unwrap();
    let foreign_pool = rhapsody::raw::descriptor_set::VulkanDescriptorPool::new(
        device.clone(), heap.layout.plan(), 1).unwrap();
    let mut foreign = foreign_pool.allocate(heap.layout.clone()).unwrap();
    for (index, owner) in owners.iter().cloned()
        .chain([foreign_parameters.clone(), foreign_parameters.clone()]).enumerate() {
        unsafe { foreign.update(2, index as u32, &rhapsody::raw::binding::DescriptorSetBinding::Buffer {
            ty: vk::DescriptorType::STORAGE_BUFFER,
            info: vk::DescriptorBufferInfo::default().buffer(owner.raw_handle()).offset(0)
                .range(if index < 3 { 4 } else { 16 }),
            owner,
        }) }.unwrap();
    }
    let foreign = Arc::new(foreign);
    let readback = Buffer::new(device.clone(),
        &BufferDesc::new_gpu_to_cpu(20, vk::BufferUsageFlags::TRANSFER_DST)).unwrap();
    let pool = VulkanCommandPool::new(device.clone()).unwrap();
    let mut command = pool.allocate().unwrap();
    command.begin().unwrap();
    unsafe {
        pipeline.bind_pipeline(command.raw_handle());
        command.retain_opaque(foreign.clone()).unwrap();
        for (index, selection) in [0usize, 1, 2, 0, 0].into_iter().enumerate() {
            if index == 4 {
                device.raw_device().cmd_bind_descriptor_sets(command.raw_handle(),
                    vk::PipelineBindPoint::COMPUTE, pipeline.layout.raw_handle(), 0,
                    &[foreign.raw_handle()], &[]);
            }
            prepared[selection]
                .bind_command(&mut command, pipeline.clone())
                .unwrap();
            device
                .raw_device()
                .cmd_dispatch(command.raw_handle(), 1, 1, 1);
            device.raw_device().cmd_pipeline_barrier(
                command.raw_handle(),
                vk::PipelineStageFlags::COMPUTE_SHADER,
                vk::PipelineStageFlags::TRANSFER,
                vk::DependencyFlags::empty(),
                &[],
                &[vk::BufferMemoryBarrier::default()
                    .buffer(owners[1].raw_handle())
                    .offset(0)
                    .size(4)
                    .src_access_mask(vk::AccessFlags::SHADER_WRITE)
                    .dst_access_mask(vk::AccessFlags::TRANSFER_READ)
                    .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                    .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)],
                &[],
            );
            device.raw_device().cmd_copy_buffer(
                command.raw_handle(),
                owners[1].raw_handle(),
                readback.raw_handle(),
                &[vk::BufferCopy {
                    src_offset: 0,
                    dst_offset: index as u64 * 4,
                    size: 4,
                }],
            );
            device.raw_device().cmd_pipeline_barrier(
                command.raw_handle(),
                vk::PipelineStageFlags::TRANSFER,
                vk::PipelineStageFlags::COMPUTE_SHADER,
                vk::DependencyFlags::empty(),
                &[],
                &[vk::BufferMemoryBarrier::default()
                    .buffer(owners[1].raw_handle())
                    .offset(0)
                    .size(4)
                    .src_access_mask(vk::AccessFlags::TRANSFER_READ)
                    .dst_access_mask(vk::AccessFlags::SHADER_WRITE)
                    .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                    .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)],
                &[],
            );
        }
        device.raw_device().cmd_pipeline_barrier(
            command.raw_handle(),
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::HOST,
            vk::DependencyFlags::empty(),
            &[],
            &[vk::BufferMemoryBarrier::default()
                .buffer(readback.raw_handle())
                .offset(0)
                .size(20)
                .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                .dst_access_mask(vk::AccessFlags::HOST_READ)
                .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)],
            &[],
        );
    }
    command.end().unwrap();
    let pending = command.submit().unwrap();
    drop(prepared);
    device
        .wait(
            &rhapsody::sync::SyncSet::from_point(pending.point),
            10_000_000_000,
        )
        .unwrap();
    drop(pending);
    let mut snapshots = [0; 20];
    readback.read_bytes(0, &mut snapshots).unwrap();
    let values: Vec<_> = snapshots
        .chunks_exact(4)
        .map(|bytes| u32::from_le_bytes(bytes.try_into().unwrap()))
        .collect();
    assert_eq!(values, [42, 43, 44, 42, 42]);
    let mut actual = [0; 4];
    owners[1].read_bytes(0, &mut actual).unwrap();
    assert_eq!(u32::from_le_bytes(actual), 42);
    owners[0].read_bytes(0, &mut actual).unwrap();
    assert_eq!(u32::from_le_bytes(actual), 123);
    for _ in 1..device.frames_in_flight() {
        pages.advance_frame();
    }
    let mut next_frame = pages.cursor();
    let reused = next_frame
        .prepare(|group| {
            group.require_slots(required)?;
            let mut bindings = group.parameters(
                package.abi.clone(),
                &20u32.to_le_bytes(),
            )?;
            for ((location, element), resource) in locations.iter().zip(&resources) {
                bindings.write(*location, *element, resource.clone())?;
            }
            bindings
                .finish(|size| constants.allocate(size))
                .map_err(Into::into)
        })
        .unwrap();
    assert!(
        reused.belongs_to(&heap),
        "completed frame bank must reuse its original arena"
    );
    assert_eq!(
        pages.page_count(),
        2,
        "rotating back must not strand completed slots"
    );
    assert_eq!(compiler.stats().compiled, 1);
    assert_eq!(device.retirement_stats().idle_waits, 0);
}

#[test]
fn hot_reload_rejects_a_changed_logical_abi_even_when_lowered_spirv_is_identical() {
    let Some(device) = gpu_test_support::initialize(
        "bindless hot ABI",
        Device::new_headless(&DeviceConfig::headless()),
    ) else {
        return;
    };
    let heap = BindlessHeap::new(
        device.clone(),
        BindlessConfig {
            sampled_images: 4,
            storage_images: 4,
            storage_buffers: 16,
            samplers: 4,
        },
    )
    .unwrap();
    let original = input();
    let mut renamed = original.clone();
    let mut module = rspirv::dr::load_words(&renamed.stages[0].spirv).unwrap();
    use rspirv::{
        binary::Assemble,
        dr::Operand,
        spirv::{Decoration, Op},
    };
    let variable = module
        .annotations
        .iter()
        .find_map(|inst| {
            if inst.class.opcode == Op::Decorate
                && inst.operands.get(1) == Some(&Operand::Decoration(Decoration::DescriptorSet))
                && inst.operands.get(2) == Some(&Operand::LiteralInt32(1))
                && let Operand::IdRef(id) = inst.operands[0]
            {
                return Some(id);
            }
            None
        })
        .unwrap();
    for inst in &mut module.annotations {
        if inst.class.opcode == Op::Decorate
            && inst.operands.first() == Some(&Operand::IdRef(variable))
            && inst.operands.get(1) == Some(&Operand::Decoration(Decoration::Binding))
        {
            inst.operands[2] = Operand::LiteralInt32(4);
        }
    }
    renamed.stages[0].spirv = module.assemble();
    renamed.reflection.descriptor_sets[1][0].0 = 4;
    if let Some(uniform) = &mut renamed.reflection.uniform_block {
        uniform.binding.binding = 4;
    }
    for (_, location) in &mut renamed.reflection.binding_names {
        if location.set == 1 && location.binding == 3 {
            location.binding = 4;
        }
    }
    for (_, location) in &mut renamed.reflection.binding_name_hashes {
        if location.set == 1 && location.binding == 3 {
            location.binding = 4;
        }
    }
    let first = BindlessShaderPackage::new(&heap, &original, 64).unwrap();
    let next = BindlessShaderPackage::new(&heap, &renamed, 64).unwrap();
    assert_eq!(first.stages[0].spirv, next.stages[0].spirv);
    assert_ne!(first.abi, next.abi);
    let compiler = AsyncPipelineCompiler::new(
        Arc::new(PipelineCache::new(device, &[]).unwrap()),
        CompilerConfig::default(),
    )
    .unwrap();
    let old = compiler
        .request_compute_spirv(first.stages[0].spirv_bytes(), c"main".into(), first.layout)
        .unwrap()
        .wait(Duration::from_secs(10))
        .unwrap();
    let ticket = compiler
        .request_compute_spirv(next.stages[0].spirv_bytes(), c"main".into(), next.layout)
        .unwrap();
    ticket.wait(Duration::from_secs(10)).unwrap();
    let mut hot = HotPipeline::new(old.clone());
    hot.stage(ticket);
    assert!(!hot.publish_at_frame_boundary());
    assert_eq!(hot.revision(), 0);
    assert!(Arc::ptr_eq(&hot.current(), &old));
    assert!(matches!(
        hot.last_error().unwrap().as_ref(),
        rhapsody::Error::InvalidUsage { .. }
    ));
}

/// GPU-driven draws read per-draw parameters through the `DrawIndex` built-in.
/// Lowering rewrites descriptor access only; it must keep the `DrawParameters`
/// capability and the `BuiltIn DrawIndex` decoration intact.
#[test]
fn lowering_keeps_the_draw_index_builtin_and_its_capability() {
    use rspirv::{
        binary::Assemble,
        dr::{Builder, Operand},
        spirv::*,
    };
    let mut b = Builder::new();
    b.set_version(1, 3);
    b.capability(Capability::Shader);
    b.capability(Capability::DrawParameters);
    b.memory_model(AddressingModel::Logical, MemoryModel::GLSL450);
    let void = b.type_void();
    let function = b.type_function(void, []);
    let uint = b.type_int(32, 0);
    let float = b.type_float(32);
    let vec4 = b.type_vector(float, 4);
    let zero = b.constant_u32(uint, 0);
    let block = b.type_struct([uint]);
    b.decorate(block, Decoration::Block, []);
    b.member_decorate(block, 0, Decoration::Offset, [Operand::LiteralInt32(0)]);
    let array = b.type_runtime_array(block);
    b.decorate(array, Decoration::ArrayStride, [Operand::LiteralInt32(4)]);
    let params_block = b.type_struct([array]);
    b.decorate(params_block, Decoration::Block, []);
    b.member_decorate(
        params_block,
        0,
        Decoration::Offset,
        [Operand::LiteralInt32(0)],
    );
    let params_pointer = b.type_pointer(None, StorageClass::StorageBuffer, params_block);
    let params = b.variable(params_pointer, None, StorageClass::StorageBuffer, None);
    b.name(params, "draw_params");
    b.name(block, "DrawParams");
    b.member_name(block, 0, "value");
    b.name(params_block, "DrawParamsBuffer");
    b.member_name(params_block, 0, "items");
    b.decorate(
        params,
        Decoration::DescriptorSet,
        [Operand::LiteralInt32(0)],
    );
    b.decorate(params, Decoration::Binding, [Operand::LiteralInt32(30)]);
    let input_uint = b.type_pointer(None, StorageClass::Input, uint);
    let draw_index = b.variable(input_uint, None, StorageClass::Input, None);
    b.decorate(
        draw_index,
        Decoration::BuiltIn,
        [Operand::BuiltIn(BuiltIn::DrawIndex)],
    );
    b.decorate(draw_index, Decoration::Flat, []);
    let output_vec4 = b.type_pointer(None, StorageClass::Output, vec4);
    let position = b.variable(output_vec4, None, StorageClass::Output, None);
    b.decorate(
        position,
        Decoration::BuiltIn,
        [Operand::BuiltIn(BuiltIn::Position)],
    );
    let storage_uint = b.type_pointer(None, StorageClass::StorageBuffer, uint);
    let main = b
        .begin_function(void, None, FunctionControl::NONE, function)
        .unwrap();
    b.begin_block(None).unwrap();
    let id = b.load(uint, None, draw_index, None, []).unwrap();
    let field = b
        .access_chain(storage_uint, None, params, [zero, id, zero])
        .unwrap();
    let value = b.load(uint, None, field, None, []).unwrap();
    let value = b.convert_u_to_f(float, None, value).unwrap();
    let vector = b
        .composite_construct(vec4, None, [value, value, value, value])
        .unwrap();
    b.store(position, vector, None, []).unwrap();
    b.ret().unwrap();
    b.end_function().unwrap();
    b.entry_point(ExecutionModel::Vertex, main, "main", [draw_index, position]);
    let stage = CompiledShaderStage {
        stage: ShaderStage::Vertex,
        entry_point: "main".into(),
        spirv: b.module().assemble(),
    };
    let reflection =
        shader_compiler::reflect::generate_reflection(std::slice::from_ref(&stage)).unwrap();
    let input = CompiledShaderPipeline {
        stages: vec![stage],
        reflection,
        source_dependencies: vec![],
        trailing_bytes: vec![],
    };
    let (_, stages) = shader_compiler::bindless::lower_pipeline(&input).unwrap();
    let lowered = rspirv::dr::load_words(&stages[0].spirv).unwrap();
    assert!(
        lowered
            .capabilities
            .iter()
            .any(|inst| inst.operands == [Operand::Capability(Capability::DrawParameters)])
    );
    assert!(lowered.annotations.iter().any(|inst| {
        inst.operands.get(1) == Some(&Operand::Decoration(Decoration::BuiltIn))
            && inst.operands.get(2) == Some(&Operand::BuiltIn(BuiltIn::DrawIndex))
    }));
}
