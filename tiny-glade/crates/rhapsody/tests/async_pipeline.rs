mod draw_support;
use ash::vk;
use draw_support::gpu_test_support;
use rhapsody::{device::{Device, DeviceConfig}, pipeline::{AsyncPipelineCompiler, CompilePriority, CompilerConfig, DiskCacheStatus, HotPipeline, PersistentPipelineCache, RequestOptions}};
use rhapsody::command::command_buffer::VulkanCommandPool;
use rhapsody::pipeline::shader::{ComputePipelineDesc, DynamicRenderingFormats, PipelineCache, PipelineLayout, RasterPipelineDesc, ShaderModule};
use std::{
    sync::{Arc, Barrier},
    time::Duration,
};

const WORDS: &[u32] = &[
    0x07230203, 0x00010000, 0, 6, 0, 0x00020011, 1, 0x0003000e, 0, 1, 0x0005000f, 5, 4, 0x6e69616d,
    0, 0x00060010, 4, 17, 1, 1, 1, 0x00020013, 1, 0x00030021, 2, 1, 0x00050036, 1, 4, 0, 2,
    0x000200f8, 5, 0x000100fd, 0x00010038,
];
fn bytes() -> Vec<u8> {
    WORDS.iter().flat_map(|word| word.to_le_bytes()).collect()
}
fn bytes_with_local_size(size: [u32; 3]) -> Vec<u8> {
    use rspirv::{binary::Assemble, dr::Operand};
    let mut module = rspirv::dr::load_words(WORDS).unwrap();
    let mode = &mut module.execution_modes[0];
    for (operand, value) in mode.operands[2..].iter_mut().zip(size) {
        *operand = Operand::LiteralInt32(value);
    }
    module
        .assemble()
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect()
}
fn changed_shader_interface(binding: bool, push: bool) -> Vec<u8> {
    shader_interface_with_offset(binding, push, 0)
}
fn shader_interface_with_offset(binding: bool, push: bool, offset: u32) -> Vec<u8> {
    use rspirv::{
        binary::Assemble,
        dr::{Builder, Operand},
        spirv::*,
    };
    let mut module = Builder::new();
    module.set_version(1, 3);
    module.capability(Capability::Shader);
    module.memory_model(AddressingModel::Logical, MemoryModel::GLSL450);
    let void = module.type_void();
    let function = module.type_function(void, vec![]);
    let uint = module.type_int(32, 0);
    let zero = module.constant_u32(uint, 0);
    let value = module.constant_u32(uint, 77);
    let block = module.type_struct([uint]);
    module.decorate(block, Decoration::Block, []);
    module.member_decorate(
        block,
        0,
        Decoration::Offset,
        [Operand::LiteralInt32(offset)],
    );
    let storage = if binding {
        let pointer = module.type_pointer(None, StorageClass::StorageBuffer, block);
        let global = module.variable(pointer, None, StorageClass::StorageBuffer, None);
        module.decorate(
            global,
            Decoration::DescriptorSet,
            [Operand::LiteralInt32(0)],
        );
        module.decorate(global, Decoration::Binding, [Operand::LiteralInt32(0)]);
        Some(global)
    } else {
        None
    };
    let push_global = if push {
        let pointer = module.type_pointer(None, StorageClass::PushConstant, block);
        Some(module.variable(pointer, None, StorageClass::PushConstant, None))
    } else {
        None
    };
    let storage_uint = module.type_pointer(None, StorageClass::StorageBuffer, uint);
    let push_uint = module.type_pointer(None, StorageClass::PushConstant, uint);
    let main = module
        .begin_function(void, None, FunctionControl::NONE, function)
        .unwrap();
    module.begin_block(None).unwrap();
    let data = if let Some(global) = push_global {
        let member = module
            .access_chain(push_uint, None, global, [zero])
            .unwrap();
        module.load(uint, None, member, None, []).unwrap()
    } else {
        value
    };
    if let Some(global) = storage {
        let member = module
            .access_chain(storage_uint, None, global, [zero])
            .unwrap();
        module.store(member, data, None, []).unwrap();
    }
    module.ret().unwrap();
    module.end_function().unwrap();
    module.entry_point(ExecutionModel::GLCompute, main, "main", []);
    module.execution_mode(main, ExecutionMode::LocalSize, [1, 1, 1]);
    module
        .module()
        .assemble()
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect()
}
fn shader_with_an_unused_binding_and_a_different_entry() -> Vec<u8> {
    use rspirv::{
        binary::Assemble,
        dr::{Builder, Operand},
        spirv::*,
    };
    let mut module = Builder::new();
    module.set_version(1, 3);
    module.capability(Capability::Shader);
    module.memory_model(AddressingModel::Logical, MemoryModel::GLSL450);
    let void = module.type_void();
    let function = module.type_function(void, vec![]);
    let uint = module.type_int(32, 0);
    let block = module.type_struct([uint]);
    module.decorate(block, Decoration::Block, []);
    module.member_decorate(block, 0, Decoration::Offset, [Operand::LiteralInt32(0)]);
    let ptr = module.type_pointer(None, StorageClass::StorageBuffer, block);
    let scalar = module.type_pointer(None, StorageClass::StorageBuffer, uint);
    let buffer = module.variable(ptr, None, StorageClass::StorageBuffer, None);
    module.decorate(
        buffer,
        Decoration::DescriptorSet,
        [Operand::LiteralInt32(0)],
    );
    module.decorate(buffer, Decoration::Binding, [Operand::LiteralInt32(0)]);
    let zero = module.constant_u32(uint, 0);
    let main = module
        .begin_function(void, None, FunctionControl::NONE, function)
        .unwrap();
    module.begin_block(None).unwrap();
    module.ret().unwrap();
    module.end_function().unwrap();
    let other = module
        .begin_function(void, None, FunctionControl::NONE, function)
        .unwrap();
    module.begin_block(None).unwrap();
    let destination = module.access_chain(scalar, None, buffer, [zero]).unwrap();
    module.store(destination, zero, None, []).unwrap();
    module.ret().unwrap();
    module.end_function().unwrap();
    for (id, name) in [(main, "main"), (other, "other")] {
        module.entry_point(ExecutionModel::GLCompute, id, name, []);
        module.execution_mode(id, ExecutionMode::LocalSize, [1, 1, 1]);
    }
    module
        .module()
        .assemble()
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect()
}
fn device() -> Option<Arc<Device>> {
    gpu_test_support::initialize(
        "async pipeline compiler device",
        Device::new_headless(&DeviceConfig::headless()),
    )
}
fn idle(compiler: &AsyncPipelineCompiler) {
    let deadline = std::time::Instant::now() + Duration::from_secs(10);
    while compiler.stats().pending != 0 {
        assert!(std::time::Instant::now() < deadline);
        std::thread::yield_now();
    }
}

#[test]
fn concurrent_async_requests_merge_and_bounded_keys_do_not_accumulate() {
    let Some(device) = device() else {
        return;
    };
    let cache = Arc::new(PipelineCache::with_capacity(device.clone(), &[], 2).unwrap());
    let compiler = Arc::new(
        AsyncPipelineCompiler::new(
            cache.clone(),
            CompilerConfig {
                workers: 2,
                max_cached_requests: 2,
                ..Default::default()
            },
        )
        .unwrap(),
    );
    let shader = ShaderModule::from_spirv(device.clone(), &bytes()).unwrap();
    let layout = PipelineLayout::new(device.clone(), vec![], vec![]).unwrap();
    let barrier = Arc::new(Barrier::new(8));
    let tickets = std::thread::scope(|scope| {
        let threads = (0..8)
            .map(|_| {
                let (compiler, shader, layout, barrier) = (
                    compiler.clone(),
                    shader.clone(),
                    layout.clone(),
                    barrier.clone(),
                );
                scope.spawn(move || {
                    barrier.wait();
                    compiler
                        .request_compute(ComputePipelineDesc::new(shader, layout))
                        .unwrap()
                })
            })
            .collect::<Vec<_>>();
        threads
            .into_iter()
            .map(|thread| thread.join().unwrap())
            .collect::<Vec<_>>()
    });
    assert!(
        tickets
            .iter()
            .all(|ticket| ticket.same_request(&tickets[0]))
    );
    let original = tickets[0].wait(Duration::from_secs(10)).unwrap();
    idle(&compiler);
    assert_eq!(compiler.stats().compiled, 1);
    assert_eq!(compiler.stats().merged, 7);
    for _ in 0..12 {
        let shader = ShaderModule::from_spirv(device.clone(), &bytes()).unwrap();
        compiler
            .request_compute(ComputePipelineDesc::new(shader, layout.clone()))
            .unwrap()
            .wait(Duration::from_secs(10))
            .unwrap();
    }
    idle(&compiler);
    assert!(compiler.stats().cached_requests <= 2);
    assert!(compiler.stats().cached_key_bytes <= CompilerConfig::default().max_request_bytes);
    assert!(cache.cached_compute_count() <= 2);
    // An evicted pipeline remains a real, usable object through external Arc ownership.
    let pool = VulkanCommandPool::new(device.clone()).unwrap();
    let mut command = pool.allocate().unwrap();
    command.begin().unwrap();
    command.retain_object(original.clone()).unwrap();
    unsafe {
        original.bind_pipeline(command.raw_handle());
        device
            .raw_device()
            .cmd_dispatch(command.raw_handle(), 1, 1, 1);
    }
    command.end().unwrap();
    let pending = command.submit().unwrap();
    pending.wait(10_000_000_000).unwrap();
}

#[test]
fn concurrent_raster_requests_share_one_ticket_compile_and_stable_cache_budget() {
    let Some(device) = device() else { return };
    let (vertex, fragment) = draw_support::shaders();
    let vertex = ShaderModule::from_spirv(device.clone(), &vertex).unwrap();
    let fragment = ShaderModule::from_spirv(device.clone(), &fragment).unwrap();
    let layout = PipelineLayout::new(device.clone(), vec![], vec![]).unwrap();
    let mut desc = RasterPipelineDesc::new(vertex, layout);
    desc.fragment = Some(fragment);
    desc.fragment_entry = Some(c"ps_main".to_owned());
    desc.use_dynamic_rendering(DynamicRenderingFormats::color(vk::Format::R8G8B8A8_UNORM));
    desc.vertex_bindings
        .push(vk::VertexInputBindingDescription {
            binding: 0,
            stride: 16,
            input_rate: vk::VertexInputRate::VERTEX,
        });
    desc.vertex_attributes
        .push(vk::VertexInputAttributeDescription {
            location: 0,
            binding: 0,
            format: vk::Format::R32G32B32A32_SFLOAT,
            offset: 0,
        });
    desc.blend_attachments
        .push(vk::PipelineColorBlendAttachmentState::default());
    let compiler = Arc::new(
        AsyncPipelineCompiler::new(
            Arc::new(PipelineCache::with_capacity(device.clone(), &[], 2).unwrap()),
            CompilerConfig {
                workers: 2,
                max_cached_requests: 2,
                ..Default::default()
            },
        )
        .unwrap(),
    );
    let barrier = Arc::new(Barrier::new(8));
    let tickets = std::thread::scope(|scope| {
        let threads = (0..8)
            .map(|_| {
                let (compiler, barrier, desc) = (compiler.clone(), barrier.clone(), desc.clone());
                scope.spawn(move || {
                    barrier.wait();
                    compiler.request_raster(desc).unwrap()
                })
            })
            .collect::<Vec<_>>();
        threads
            .into_iter()
            .map(|thread| thread.join().unwrap())
            .collect::<Vec<_>>()
    });
    assert!(
        tickets
            .iter()
            .all(|ticket| ticket.same_request(&tickets[0]))
    );
    let pipeline = tickets[0].wait(Duration::from_secs(10)).unwrap();
    idle(&compiler);
    let warm = compiler.stats();
    let cpu = device.allocator_stats().cpu_memory_usage();
    assert_eq!(warm.compiled, 1);
    assert_eq!(warm.merged, 7);
    assert_eq!(warm.cached_requests, 1);
    for _ in 0..64 {
        let ticket = compiler.request_raster(desc.clone()).unwrap();
        assert!(ticket.same_request(&tickets[0]));
        assert!(Arc::ptr_eq(&ticket.try_get().unwrap(), &pipeline));
    }
    let after = compiler.stats();
    assert_eq!(after.compiled, 1);
    assert_eq!(after.merged, 71);
    assert_eq!(after.pending, 0);
    assert_eq!(after.request_bytes, 0);
    assert_eq!(after.cached_requests, 1);
    assert_eq!(after.cached_key_bytes, warm.cached_key_bytes);
    assert_eq!(device.allocator_stats().cpu_memory_usage(), cpu);
}

#[test]
fn failed_hot_reload_keeps_the_old_pipeline_and_only_frame_boundary_publishes() {
    let Some(device) = device() else {
        return;
    };
    let cache = Arc::new(PipelineCache::new(device.clone(), &[]).unwrap());
    let compiler = AsyncPipelineCompiler::new(cache, CompilerConfig::default()).unwrap();
    let layout = PipelineLayout::new(device.clone(), vec![], vec![]).unwrap();
    let original = compiler
        .request_compute_spirv(bytes(), c"main".to_owned(), layout.clone())
        .unwrap()
        .wait(Duration::from_secs(10))
        .unwrap();
    let mut hot = HotPipeline::new(original.clone());
    let failed = compiler
        .request_compute_spirv(vec![0, 1, 2, 3], c"main".to_owned(), layout.clone())
        .unwrap();
    assert!(failed.wait(Duration::from_secs(10)).is_err());
    hot.stage(failed);
    assert!(!hot.publish_at_frame_boundary());
    assert!(Arc::ptr_eq(&hot.current(), &original));
    assert_eq!(
        hot.last_error().unwrap().kind(),
        rhapsody::ErrorKind::InvalidUsage
    );
    assert!(!device.is_device_lost());
    // A new module gives the same valid interface but a different pipeline object.
    let shader = ShaderModule::from_spirv(device.clone(), &bytes()).unwrap();
    let replacement = compiler
        .request_compute(ComputePipelineDesc::new(shader, layout.clone()))
        .unwrap();
    let next = replacement.wait(Duration::from_secs(10)).unwrap();
    hot.stage(replacement);
    assert!(Arc::ptr_eq(&hot.current(), &original));
    assert!(hot.publish_at_frame_boundary());
    assert!(Arc::ptr_eq(&hot.current(), &next));
    assert_eq!(hot.revision(), 1);
    assert!(hot.last_error().is_none());
    let incompatible_layout = PipelineLayout::new(
        device,
        vec![],
        vec![vk::PushConstantRange {
            stage_flags: vk::ShaderStageFlags::COMPUTE,
            offset: 0,
            size: 4,
        }],
    )
    .unwrap();
    let incompatible = compiler
        .request_compute_spirv(bytes(), c"main".to_owned(), incompatible_layout)
        .unwrap();
    incompatible.wait(Duration::from_secs(10)).unwrap();
    hot.stage(incompatible);
    assert!(!hot.publish_at_frame_boundary());
    assert!(Arc::ptr_eq(&hot.current(), &next));
}

#[test]
fn hot_reload_rejects_changed_workgroup_dimensions_for_spirv_and_prebuilt_shaders() {
    let Some(device) = device() else { return };
    let layout = PipelineLayout::new(device.clone(), vec![], vec![]).unwrap();
    let compiler = AsyncPipelineCompiler::new(
        Arc::new(PipelineCache::new(device.clone(), &[]).unwrap()),
        CompilerConfig::default(),
    )
    .unwrap();
    let original = compiler
        .request_compute_spirv(bytes(), c"main".to_owned(), layout.clone())
        .unwrap()
        .wait(Duration::from_secs(10))
        .unwrap();
    let mut hot = HotPipeline::new(original.clone());
    for (size, prebuilt) in [([2, 1, 1], false), ([1, 2, 1], true), ([1, 1, 2], false)] {
        let candidate = bytes_with_local_size(size);
        let words = ash::util::read_spv(&mut std::io::Cursor::new(candidate.as_slice())).unwrap();
        let reflected = rhapsody::pipeline::shader::reflect_spirv_entries(&words).unwrap();
        assert_eq!(reflected[0].local_size, Some(size));
        let ticket = if prebuilt {
            compiler.request_compute(ComputePipelineDesc::new(
                ShaderModule::from_spirv(device.clone(), &candidate).unwrap(),
                layout.clone(),
            ))
        } else {
            compiler.request_compute_spirv(candidate, c"main".to_owned(), layout.clone())
        }
        .unwrap();
        ticket.wait(Duration::from_secs(10)).unwrap();
        hot.stage(ticket);
        assert!(
            !hot.publish_at_frame_boundary(),
            "dispatch dimensions changed to {size:?}"
        );
        assert!(Arc::ptr_eq(&hot.current(), &original));
        assert_eq!(hot.revision(), 0);
        assert_eq!(
            hot.last_error().unwrap().kind(),
            rhapsody::ErrorKind::InvalidUsage
        );
    }
    let ticket = compiler
        .request_compute(ComputePipelineDesc::new(
            ShaderModule::from_spirv(device, &bytes()).unwrap(),
            layout,
        ))
        .unwrap();
    let replacement = ticket.wait(Duration::from_secs(10)).unwrap();
    hot.stage(ticket);
    assert!(hot.publish_at_frame_boundary());
    assert!(Arc::ptr_eq(&hot.current(), &replacement));
    assert_eq!(hot.revision(), 1);
    assert!(hot.last_error().is_none());
}

#[test]
fn known_device_loss_stops_cached_requests_and_hot_publication_without_replacing_the_interface() {
    let Some(device) = device() else {
        return;
    };
    let cache = Arc::new(PipelineCache::new(device.clone(), &[]).unwrap());
    let compiler = AsyncPipelineCompiler::new(cache.clone(), CompilerConfig::default()).unwrap();
    let layout = PipelineLayout::new(device.clone(), vec![], vec![]).unwrap();
    let shader = ShaderModule::from_spirv(device.clone(), &bytes()).unwrap();
    let original = compiler
        .request_compute(ComputePipelineDesc::new(shader.clone(), layout.clone()))
        .unwrap()
        .wait(Duration::from_secs(10))
        .unwrap();
    let replacement = compiler
        .request_compute_spirv(bytes(), c"main".to_owned(), layout.clone())
        .unwrap();
    replacement.wait(Duration::from_secs(10)).unwrap();
    idle(&compiler);
    let mut hot = HotPipeline::new(original.clone());
    hot.stage(replacement);
    // This changes the CPU adapter's terminal state. No work is submitted to
    // the GPU and the driver itself remains healthy for normal destruction.
    device.timelines().mark_device_lost();
    assert!(!hot.publish_at_frame_boundary());
    assert!(Arc::ptr_eq(&hot.current(), &original));
    assert_eq!(hot.revision(), 0);
    assert_eq!(
        hot.last_error().unwrap().kind(),
        rhapsody::ErrorKind::DeviceLost
    );
    assert!(!hot.publish_at_frame_boundary());
    assert_eq!(hot.revision(), 0);
    assert!(
        matches!(
            compiler.request_compute(ComputePipelineDesc::new(shader.clone(), layout.clone())),
            Err(rhapsody::Error::DeviceLost)
        ),
        "a ready cache entry cannot bypass terminal device loss"
    );
    assert!(matches!(
        compiler.request_compute_spirv(bytes(), c"main".to_owned(), layout.clone()),
        Err(rhapsody::Error::DeviceLost)
    ));
    assert!(matches!(cache.data(), Err(rhapsody::Error::DeviceLost)));
    assert!(matches!(
        rhapsody::pipeline::shader::VulkanPipeline::create_compute_pipeline(
            &cache,
            ComputePipelineDesc::new(shader, layout)
        ),
        Err(rhapsody::Error::DeviceLost)
    ));
    assert!(matches!(
        PipelineCache::new(device.clone(), &[]),
        Err(rhapsody::Error::DeviceLost)
    ));
    assert!(matches!(
        ShaderModule::from_spirv(device.clone(), &bytes()),
        Err(rhapsody::Error::DeviceLost)
    ));
    assert!(matches!(
        PipelineLayout::new(device, vec![], vec![]),
        Err(rhapsody::Error::DeviceLost)
    ));
    assert_eq!(compiler.stats().pending, 0);
    assert_eq!(compiler.stats().compiled, 2);
}

#[test]
fn an_observed_failed_request_can_immediately_retry_with_a_new_completion() {
    let Some(device) = device() else {
        return;
    };
    let cache = Arc::new(PipelineCache::with_capacity(device.clone(), &[], 2).unwrap());
    let compiler = AsyncPipelineCompiler::new(
        cache,
        CompilerConfig {
            workers: 2,
            max_cached_requests: 2,
            ..Default::default()
        },
    )
    .unwrap();
    let layout = PipelineLayout::new(device.clone(), vec![], vec![]).unwrap();
    let retry = || {
        compiler
            .request_compute_spirv(vec![0, 1, 2, 3], c"main".to_owned(), layout.clone())
            .unwrap()
    };
    let mut previous = retry();
    for _ in 0..64 {
        let error = previous
            .wait(Duration::from_secs(10))
            .err()
            .expect("invalid SPIR-V must fail");
        assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
        let next = retry(); // No polling/sleep may hide the completion-cleanup race.
        assert!(
            !previous.same_request(&next),
            "an already observed failure must not be merged into a retry"
        );
        previous = next;
    }
    assert!(previous.wait(Duration::from_secs(10)).is_err());
    idle(&compiler);
    assert_eq!(compiler.stats().compiled, 0);
    assert_eq!(compiler.stats().failed, 65);
    assert_eq!(compiler.stats().merged, 0);
    assert_eq!(compiler.stats().cached_requests, 0);
    assert!(
        !device.is_device_lost(),
        "bad edits must not poison the live device"
    );
}

#[test]
fn valid_shader_edits_that_change_actual_bindings_or_push_ranges_are_rejected_before_vulkan() {
    use rhapsody::native::instance::{InstanceBuilder, ValidationMessenger};
    let mut builder = InstanceBuilder::default();
    let validation = builder.enable_validation().unwrap();
    let Some(instance) = gpu_test_support::initialize("shader-interface instance", builder.build())
    else {
        return;
    };
    let messenger = ValidationMessenger::new(&instance).unwrap();
    let Some(device) = gpu_test_support::initialize(
        "shader-interface device",
        Device::new(
            &instance,
            None,
            &DeviceConfig {
                validation,
                ..DeviceConfig::headless()
            },
        ),
    ) else {
        return;
    };
    let compiler = AsyncPipelineCompiler::new(
        Arc::new(PipelineCache::new(device.clone(), &[]).unwrap()),
        CompilerConfig::default(),
    )
    .unwrap();
    let layout = PipelineLayout::new(device.clone(), vec![], vec![]).unwrap();
    let original = compiler
        .request_compute_spirv(bytes(), c"main".to_owned(), layout.clone())
        .unwrap()
        .wait(Duration::from_secs(10))
        .unwrap();
    let mut hot = HotPipeline::new(original.clone());
    for (binding, push) in [(true, false), (false, true), (true, true)] {
        let candidate = changed_shader_interface(binding, push);
        // The edits are valid SPIR-V; the incompatibility is with the existing
        // interface, rather than malformed input or an absent entry point.
        let words = ash::util::read_spv(&mut std::io::Cursor::new(&candidate)).unwrap();
        let module = rspirv::dr::load_words(&words).unwrap();
        assert!(
            module.debug_names.is_empty(),
            "unnamed valid modules must not evade reflection"
        );
        for (class, expected) in [
            (rspirv::spirv::StorageClass::StorageBuffer, binding),
            (rspirv::spirv::StorageClass::PushConstant, push),
        ] {
            let present = module.types_global_values.iter().any(|instruction| instruction.class.opcode == rspirv::spirv::Op::Variable
                && matches!(instruction.operands.first(), Some(rspirv::dr::Operand::StorageClass(found)) if *found == class));
            assert_eq!(present, expected);
        }
        let ticket = compiler
            .request_compute_spirv(candidate, c"main".to_owned(), layout.clone())
            .unwrap();
        let error = ticket
            .wait(Duration::from_secs(10))
            .err()
            .expect("changed interface must fail");
        assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
        hot.stage(ticket);
        assert!(!hot.publish_at_frame_boundary());
        assert!(Arc::ptr_eq(&hot.current(), &original));
        assert_eq!(hot.revision(), 0);
        assert_eq!(
            hot.last_error().unwrap().kind(),
            rhapsody::ErrorKind::InvalidUsage
        );
        assert!(!device.is_device_lost());
    }
    // A matching declared layout, including a nonzero first push offset, must
    // still compile successfully; the guard cannot reject all edited modules.
    use rhapsody::command::descriptor_set::{LayoutBinding, LayoutPlan, VulkanDescriptorSetLayout};
    let set = VulkanDescriptorSetLayout::new(
        device.clone(),
        &LayoutPlan {
            flags: 0,
            binding_flags: vec![0],
            bindings: vec![LayoutBinding {
                binding: 0,
                descriptor_type: vk::DescriptorType::STORAGE_BUFFER.as_raw(),
                descriptor_count: 1,
                stage_flags: vk::ShaderStageFlags::COMPUTE.as_raw(),
                immutable_sampler: None,
            }],
        },
    )
    .unwrap();
    for (binding, push, offset) in [(true, false, 0), (false, true, 16), (true, true, 0)] {
        let matched_layout = PipelineLayout::new(
            device.clone(),
            if binding { vec![set.clone()] } else { vec![] },
            if push {
                vec![vk::PushConstantRange {
                    stage_flags: vk::ShaderStageFlags::COMPUTE,
                    offset,
                    size: 4,
                }]
            } else {
                vec![]
            },
        )
        .unwrap();
        compiler
            .request_compute_spirv(
                shader_interface_with_offset(binding, push, offset),
                c"main".to_owned(),
                matched_layout,
            )
            .unwrap()
            .wait(Duration::from_secs(10))
            .unwrap();
    }
    let unrelated = shader_with_an_unused_binding_and_a_different_entry();
    let replacement = compiler
        .request_compute_spirv(unrelated.clone(), c"main".to_owned(), layout.clone())
        .unwrap();
    replacement.wait(Duration::from_secs(10)).unwrap();
    hot.stage(replacement);
    assert!(
        hot.publish_at_frame_boundary(),
        "another entry's resource is not part of main's actual interface"
    );
    let raw = ShaderModule::from_spirv(device.clone(), &unrelated).unwrap();
    compiler
        .request_compute(ComputePipelineDesc::new(raw, layout.clone()))
        .unwrap()
        .wait(Duration::from_secs(10))
        .unwrap();
    idle(&compiler);
    let errors = messenger.collector.errors();
    assert!(
        errors.is_empty(),
        "incompatible shader must be rejected before driver validation: {errors:?}"
    );
}

#[test]
fn prebuilt_shader_requests_cannot_publish_a_changed_interface_under_the_same_wide_layout() {
    use rhapsody::command::descriptor_set::{LayoutBinding, LayoutPlan, VulkanDescriptorSetLayout};
    let Some(device) = device() else {
        return;
    };
    let set = VulkanDescriptorSetLayout::new(
        device.clone(),
        &LayoutPlan {
            flags: 0,
            binding_flags: vec![0],
            bindings: vec![LayoutBinding {
                binding: 0,
                descriptor_type: vk::DescriptorType::STORAGE_BUFFER.as_raw(),
                descriptor_count: 1,
                stage_flags: vk::ShaderStageFlags::COMPUTE.as_raw(),
                immutable_sampler: None,
            }],
        },
    )
    .unwrap();
    let layout = PipelineLayout::new(device.clone(), vec![set], vec![]).unwrap();
    let compiler = AsyncPipelineCompiler::new(
        Arc::new(PipelineCache::new(device.clone(), &[]).unwrap()),
        CompilerConfig::default(),
    )
    .unwrap();
    let original_shader = ShaderModule::from_spirv(device.clone(), &bytes()).unwrap();
    let original = compiler
        .request_compute(ComputePipelineDesc::new(original_shader, layout.clone()))
        .unwrap()
        .wait(Duration::from_secs(10))
        .unwrap();
    let changed = ShaderModule::from_spirv(device, &changed_shader_interface(true, false)).unwrap();
    let ticket = compiler
        .request_compute(ComputePipelineDesc::new(changed, layout))
        .unwrap();
    ticket.wait(Duration::from_secs(10)).unwrap(); // Both pipelines fit this layout.
    let mut hot = HotPipeline::new(original.clone());
    hot.stage(ticket);
    assert!(
        !hot.publish_at_frame_boundary(),
        "the actual shader contract changed even though VkPipelineLayout stayed identical"
    );
    assert!(Arc::ptr_eq(&hot.current(), &original));
    assert_eq!(hot.revision(), 0);
    assert_eq!(
        hot.last_error().unwrap().kind(),
        rhapsody::ErrorKind::InvalidUsage
    );
}

#[test]
fn driver_cache_roundtrip_and_corruption_rebuild_are_explicit() {
    let Some(device) = device() else {
        return;
    };
    let path = std::env::temp_dir().join(format!(
        "rhapsody-pipeline-test-{}.bin",
        device.generation().value()
    ));
    let disk = PersistentPipelineCache::open(device.clone(), &path, 4).unwrap();
    assert_eq!(disk.status, DiskCacheStatus::Missing);
    let compiler =
        AsyncPipelineCompiler::new(disk.cache.clone(), CompilerConfig::default()).unwrap();
    let layout = PipelineLayout::new(device.clone(), vec![], vec![]).unwrap();
    compiler
        .request_compute_spirv(bytes(), c"main".to_owned(), layout)
        .unwrap()
        .wait(Duration::from_secs(10))
        .unwrap();
    disk.save().unwrap();
    assert_eq!(
        PersistentPipelineCache::open(device.clone(), &path, 4)
            .unwrap()
            .status,
        DiskCacheStatus::Loaded
    );
    let mut corrupted = std::fs::read(&path).unwrap();
    *corrupted.last_mut().unwrap() ^= 1;
    std::fs::write(&path, corrupted).unwrap();
    let rebuilt = PersistentPipelineCache::open(device.clone(), &path, 4).unwrap();
    assert_eq!(rebuilt.status, DiskCacheStatus::Rejected);
    rebuilt.save().unwrap();
    assert_eq!(
        PersistentPipelineCache::open(device, &path, 4)
            .unwrap()
            .status,
        DiskCacheStatus::Loaded
    );
    std::fs::remove_file(path).unwrap();
}

#[test]
fn oversized_requests_apply_backpressure_without_a_pipeline_call() {
    let Some(device) = device() else {
        return;
    };
    let cache = Arc::new(PipelineCache::new(device.clone(), &[]).unwrap());
    let compiler = AsyncPipelineCompiler::new(
        cache,
        CompilerConfig {
            max_request_bytes: 256,
            ..Default::default()
        },
    )
    .unwrap();
    let layout = PipelineLayout::new(device, vec![], vec![]).unwrap();
    assert!(matches!(
        compiler.request_compute_spirv(bytes(), c"main".to_owned(), layout),
        Err(rhapsody::Error::Timeout)
    ));
    assert_eq!(compiler.stats().pending, 0);
    assert_eq!(compiler.stats().compiled, 0);
}

#[test]
fn deferred_spirv_owns_capacity_budget_and_keeps_a_stable_completion_ticket() {
    let Some(device) = device() else {
        return;
    };
    let compiler = AsyncPipelineCompiler::new(
        Arc::new(PipelineCache::new(device.clone(), &[]).unwrap()),
        CompilerConfig {
            max_request_bytes: 1024,
            ..Default::default()
        },
    )
    .unwrap();
    let layout = PipelineLayout::new(device, vec![], vec![]).unwrap();
    let mut overallocated = Vec::with_capacity(4096);
    overallocated.extend_from_slice(&bytes());
    assert!(
        matches!(
            compiler.defer_compute_spirv(overallocated, c"main".to_owned(), layout.clone()),
            Err(rhapsody::Error::OutOfHostMemory)
        ),
        "budget must charge retained capacity, not byte length"
    );
    assert_eq!(compiler.stats().pending, 0);
    let ticket = compiler
        .defer_compute_spirv(bytes(), c"main".to_owned(), layout.clone())
        .unwrap();
    let duplicate = compiler
        .defer_compute_spirv(bytes(), c"main".to_owned(), layout)
        .unwrap();
    assert!(ticket.same_request(&duplicate));
    let pipeline = ticket.wait(Duration::from_secs(10)).unwrap();
    assert!(Arc::ptr_eq(&pipeline, &duplicate.try_get().unwrap()));
    idle(&compiler);
    assert_eq!(compiler.stats().compiled, 1);
    assert_eq!(compiler.stats().request_bytes, 0);
}

#[test]
fn low_priority_compilation_defers_under_resource_pressure_but_critical_work_can_progress() {
    let mut config = DeviceConfig::headless();
    config.memory_budget_bytes = 65536;
    let Some(device) =
        gpu_test_support::initialize("pipeline pressure device", Device::new_headless(&config))
    else {
        return;
    };
    let _live = rhapsody::resource::buffer::Buffer::new(
        device.clone(),
        &rhapsody::resource::buffer::BufferDesc::new_gpu_only(60000, vk::BufferUsageFlags::TRANSFER_DST),
    )
    .unwrap();
    let compiler = AsyncPipelineCompiler::new(
        Arc::new(PipelineCache::new(device.clone(), &[]).unwrap()),
        CompilerConfig::default(),
    )
    .unwrap();
    let layout = PipelineLayout::new(device, vec![], vec![]).unwrap();
    assert!(matches!(
        compiler.request_compute_spirv_with_options(
            bytes(),
            c"main".to_owned(),
            layout.clone(),
            RequestOptions {
                priority: CompilePriority::Background,
                epoch: 1,
            }
        ),
        Err(rhapsody::Error::Timeout)
    ));
    compiler
        .request_compute_spirv_with_options(
            bytes(),
            c"main".to_owned(),
            layout,
            RequestOptions {
                priority: CompilePriority::Critical,
                epoch: 2,
            },
        )
        .unwrap()
        .wait(Duration::from_secs(10))
        .unwrap();
    idle(&compiler);
    assert_eq!(compiler.stats().compiled, 1);
}
