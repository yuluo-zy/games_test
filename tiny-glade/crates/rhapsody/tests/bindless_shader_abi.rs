mod gpu_test_support;
use ash::vk;
use rhapsody::{
    bindless::{
        BindlessConfig, BindlessHeap, BindlessParameters, BindlessResource, BindlessShaderPackage,
    },
    device::{Device, DeviceConfig},
    pipeline::{AsyncPipelineCompiler, CompilerConfig, HotPipeline},
    raw::{
        buffer::{Buffer, BufferDesc},
        command_buffer::VulkanCommandPool,
        dynamic_constants::DynamicConstants,
        shader::PipelineCache,
    },
    shader_pipeline::{
        compiled::{CompiledShaderPipeline, CompiledShaderStage, ShaderStage},
        reflection::BindingLocation,
    },
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
    for value in 17..33u32 {
        let mut bindings =
            BindlessParameters::new(heap.clone(), package.abi.clone(), &value.to_le_bytes())
                .unwrap();
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
        }
        let prepared = bindings.finish(&mut constants).unwrap();
        let changed_push = prepared
            .with_push(&(value + 100).to_le_bytes(), &mut constants)
            .unwrap();
        let mut command = pool.allocate().unwrap();
        command.begin().unwrap();
        unsafe {
            pipeline.bind_pipeline(command.handle);
            prepared
                .bind_command(&mut command, pipeline.clone())
                .unwrap();
            device.raw_device().cmd_dispatch(command.handle, 1, 1, 1);
            device.raw_device().cmd_pipeline_barrier(
                command.handle,
                vk::PipelineStageFlags::COMPUTE_SHADER,
                vk::PipelineStageFlags::COMPUTE_SHADER,
                vk::DependencyFlags::empty(),
                &[],
                &[vk::BufferMemoryBarrier::default()
                    .buffer(second.handle)
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
            device.raw_device().cmd_dispatch(command.handle, 1, 1, 1);
            device.raw_device().cmd_pipeline_barrier(
                command.handle,
                vk::PipelineStageFlags::COMPUTE_SHADER,
                vk::PipelineStageFlags::HOST,
                vk::DependencyFlags::empty(),
                &[],
                &[vk::BufferMemoryBarrier::default()
                    .buffer(second.handle)
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
    assert_eq!(device.garbage_stats().idle_waits, 0);
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
