mod gpu_test_support;
use ash::vk;
use rhapsody::{
    bindless::{BindlessConfig, BindlessHeap},
    device::{Device, DeviceConfig},
    pipeline::{AsyncPipelineCompiler, CompilerConfig},
    raw::{
        buffer::{Buffer, BufferDesc},
        command_buffer::VulkanCommandPool,
        semaphore::TimelineSemaphore,
        shader::{PipelineCache, PipelineLayout},
    },
};
use std::{sync::Arc, time::Duration};

fn shader() -> Vec<u8> {
    use rspirv::{
        binary::Assemble,
        dr::{Builder, Operand},
        spirv::*,
    };
    let mut b = Builder::new();
    b.set_version(1, 3);
    b.capability(Capability::Shader);
    b.capability(Capability::RuntimeDescriptorArray);
    b.extension("SPV_EXT_descriptor_indexing");
    b.memory_model(AddressingModel::Logical, MemoryModel::GLSL450);
    let void = b.type_void();
    let function = b.type_function(void, vec![]);
    let uint = b.type_int(32, 0);
    let zero = b.constant_u32(uint, 0);
    let value = b.constant_u32(uint, 77);
    let block = b.type_struct([uint]);
    b.decorate(block, Decoration::Block, []);
    b.member_decorate(block, 0, Decoration::Offset, [Operand::LiteralInt32(0)]);
    let array = b.type_runtime_array(block);
    let array_ptr = b.type_pointer(None, StorageClass::StorageBuffer, array);
    let buffers = b.variable(array_ptr, None, StorageClass::StorageBuffer, None);
    b.decorate(
        buffers,
        Decoration::DescriptorSet,
        [Operand::LiteralInt32(0)],
    );
    b.decorate(buffers, Decoration::Binding, [Operand::LiteralInt32(2)]);
    // Builder interns identical types; emit their Block/Offset decorations once.
    let pcblock = block;
    let pcptr = b.type_pointer(None, StorageClass::PushConstant, pcblock);
    let pc = b.variable(pcptr, None, StorageClass::PushConstant, None);
    let pcuint = b.type_pointer(None, StorageClass::PushConstant, uint);
    let bufferuint = b.type_pointer(None, StorageClass::StorageBuffer, uint);
    let main = b
        .begin_function(void, None, FunctionControl::NONE, function)
        .unwrap();
    b.begin_block(None).unwrap();
    let index_ptr = b.access_chain(pcuint, None, pc, [zero]).unwrap();
    let index = b.load(uint, None, index_ptr, None, []).unwrap();
    let destination = b
        .access_chain(bufferuint, None, buffers, [index, zero])
        .unwrap();
    b.store(destination, value, None, []).unwrap();
    b.ret().unwrap();
    b.end_function().unwrap();
    b.entry_point(ExecutionModel::GLCompute, main, "main", []);
    b.execution_mode(main, ExecutionMode::LocalSize, [1, 1, 1]);
    b.module()
        .assemble()
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect()
}
fn device() -> Option<Arc<Device>> {
    gpu_test_support::initialize(
        "bindless device",
        Device::new_headless(&DeviceConfig::headless()),
    )
}
fn small() -> BindlessConfig {
    BindlessConfig {
        sampled_images: 1,
        storage_images: 1,
        storage_buffers: 1,
        samplers: 1,
    }
}
struct ReleaseGate(Arc<TimelineSemaphore>);
impl Drop for ReleaseGate {
    fn drop(&mut self) {
        unsafe {
            let _ = self.0.device().raw_device().signal_semaphore(
                &vk::SemaphoreSignalInfo::default()
                    .semaphore(self.0.raw_handle())
                    .value(1),
            );
        }
    }
}
fn target(device: &Arc<Device>) -> Arc<Buffer> {
    Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_cpu_to_gpu(4, vk::BufferUsageFlags::STORAGE_BUFFER),
        )
        .unwrap(),
    )
}

#[test]
fn a_real_pending_shader_keeps_the_bindless_slot_until_actual_completion() {
    let Some(device) = device() else {
        return;
    };
    let heap = BindlessHeap::new(device.clone(), small()).unwrap();
    pending_shader_retires_its_actual_page(device, heap.clone(), heap.layout.clone(), None);
}

#[test]
fn a_pending_layout_shared_page_rejects_foreign_indices_and_retires_independently() {
    let Some(device) = device() else {
        return;
    };
    let template = BindlessHeap::new(device.clone(), small()).unwrap();
    let page = template.new_page().unwrap();
    assert!(Arc::ptr_eq(&template.layout, &page.layout));
    let foreign = template.storage_buffer(target(&device), 0, 4).unwrap();
    assert_eq!(foreign.index().index, 0);
    let layout = template.layout.clone();
    let template_owner = Arc::downgrade(&template);
    drop(template);
    pending_shader_retires_its_actual_page(device, page.clone(), layout, Some(foreign));
    assert!(
        template_owner.upgrade().is_none(),
        "new page must not retain its template heap"
    );
    assert_eq!(
        page.stats()
            .table(rhapsody::bindless::DescriptorKind::StorageBuffer)
            .live,
        0
    );
}

#[test]
fn a_layout_shared_page_has_independent_slots_and_unused_leases() {
    let Some(device) = device() else {
        return;
    };
    let template = BindlessHeap::new(device.clone(), small()).unwrap();
    let page = template.new_page().unwrap();
    assert!(Arc::ptr_eq(&template.layout, &page.layout));
    let template_lease = template.storage_buffer(target(&device), 0, 4).unwrap();
    let page_lease = page.storage_buffer(target(&device), 0, 4).unwrap();
    assert_eq!(template_lease.index(), page_lease.index());
    assert!(matches!(
        page.storage_buffer(target(&device), 0, 4),
        Err(rhapsody::Error::Exhausted(_))
    ));
    drop(template_lease);
    let kind = rhapsody::bindless::DescriptorKind::StorageBuffer;
    assert_eq!(template.stats().table(kind).free, 1);
    assert_eq!(page.stats().table(kind).free, 0);
    let template_owner = Arc::downgrade(&template);
    drop(template);
    assert!(template_owner.upgrade().is_none());
    drop(page_lease);
    assert_eq!(page.stats().table(kind).free, 1);
    assert_eq!(device.retirement_stats().idle_waits, 0);
}

fn pending_shader_retires_its_actual_page(
    device: Arc<Device>,
    heap: Arc<BindlessHeap>,
    layout: Arc<rhapsody::raw::descriptor_set::VulkanDescriptorSetLayout>,
    foreign: Option<Arc<rhapsody::bindless::BindlessLease>>,
) {
    let buffer = target(&device);
    buffer.write_bytes(0, &0u32.to_le_bytes()).unwrap();
    let lease = heap.storage_buffer(buffer.clone(), 0, 4).unwrap();
    let first = lease.index();
    let layout = PipelineLayout::new(
        device.clone(),
        vec![layout],
        vec![vk::PushConstantRange {
            stage_flags: vk::ShaderStageFlags::COMPUTE,
            offset: 0,
            size: 4,
        }],
    )
    .unwrap();
    let cache = Arc::new(PipelineCache::new(device.clone(), &[]).unwrap());
    let compiler = AsyncPipelineCompiler::new(cache, CompilerConfig::default()).unwrap();
    let pipeline = compiler
        .request_compute_spirv(shader(), c"main".to_owned(), layout)
        .unwrap()
        .wait(Duration::from_secs(10))
        .unwrap();
    let pool = VulkanCommandPool::new(device.clone()).unwrap();
    let mut command = pool.allocate().unwrap();
    command.begin().unwrap();
    if let Some(foreign) = foreign {
        // 数字下标相同不构成同页证明；拒绝后还必须能正常绑定真实页。
        assert!(matches!(
            unsafe { heap.bind_command(&mut command, pipeline.clone(), 0, &[foreign]) },
            Err(rhapsody::Error::InvalidUsage { .. })
        ));
    }
    unsafe {
        pipeline.bind_pipeline(command.raw_handle());
        heap.bind_command(
            &mut command,
            pipeline.clone(),
            0,
            std::slice::from_ref(&lease),
        )
        .unwrap();
        device.raw_device().cmd_push_constants(
            command.raw_handle(),
            pipeline.layout.raw_handle(),
            vk::ShaderStageFlags::COMPUTE,
            0,
            &first.index.to_le_bytes(),
        );
        device
            .raw_device()
            .cmd_dispatch(command.raw_handle(), 1, 1, 1);
        let barrier = vk::BufferMemoryBarrier::default()
            .buffer(buffer.raw_handle())
            .offset(0)
            .size(4)
            .src_access_mask(vk::AccessFlags::SHADER_WRITE)
            .dst_access_mask(vk::AccessFlags::HOST_READ)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED);
        device.raw_device().cmd_pipeline_barrier(
            command.raw_handle(),
            vk::PipelineStageFlags::COMPUTE_SHADER,
            vk::PipelineStageFlags::HOST,
            vk::DependencyFlags::empty(),
            &[],
            &[barrier],
            &[],
        );
    }
    command.end().unwrap();
    let gate = TimelineSemaphore::new(device.clone(), 0).unwrap();
    let release_gate = ReleaseGate(gate.clone());
    let pending = command
        .submit_to(
            rhapsody::raw::device::DeviceQueue {
                handle: device.primary_queue(),
                family: device.queue_family_index(),
                index: 0,
            },
            &[(gate.clone(), 1, vk::PipelineStageFlags::ALL_COMMANDS)],
            &[],
        )
        .unwrap();
    let point = pending.point;
    drop((lease, pending));
    // The dropped command keeps its lease until completion, so the slot is
    // still live (not merely retiring) and cannot be handed out again.
    assert_eq!(heap.stats().live, 1);
    assert_eq!(heap.stats().retiring, 0);
    assert!(matches!(
        heap.storage_buffer(target(&device), 0, 4),
        Err(rhapsody::Error::Exhausted(_))
    ));
    assert_eq!(heap.collect().unwrap(), 0);
    drop(release_gate);
    device
        .wait(&rhapsody::sync::SyncSet::from_point(point), 10_000_000_000)
        .unwrap();
    let mut bytes = [0; 4];
    buffer.read_bytes(0, &mut bytes).unwrap();
    assert_eq!(u32::from_le_bytes(bytes), 77);
    assert_eq!(heap.collect().unwrap(), 1);
    let next = heap.storage_buffer(target(&device), 0, 4).unwrap();
    assert_eq!(next.index().index, first.index);
    assert!(next.index().generation > first.generation);
    assert_eq!(heap.stats().retiring, 0);
    assert_eq!(device.retirement_stats().idle_waits, 0);
}

#[test]
fn graph_binding_rejects_undeclared_lease_bytes_and_accepts_the_complete_shader_range() {
    use ash::vk::Handle;
    use rhapsody::rg::{
        executor::PassContext,
        resource::AccessRange,
        vulkan::{AccessTable, GpuResource, ResourceSpec, VulkanGraphBackend, VulkanGraphSession},
    };
    let Some(device) = device() else { return };
    let heap = BindlessHeap::new(device.clone(), small()).unwrap();
    let buffer = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_cpu_to_gpu(64, vk::BufferUsageFlags::STORAGE_BUFFER),
        )
        .unwrap(),
    );
    buffer.write_bytes(0, &[0; 64]).unwrap();
    let lease = heap.storage_buffer(buffer.clone(), 0, 64).unwrap();
    let layout = PipelineLayout::new(
        device.clone(),
        vec![heap.layout.clone()],
        vec![vk::PushConstantRange {
            stage_flags: vk::ShaderStageFlags::COMPUTE,
            offset: 0,
            size: 4,
        }],
    )
    .unwrap();
    let cache = Arc::new(PipelineCache::new(device.clone(), &[]).unwrap());
    let compiler = AsyncPipelineCompiler::new(cache, CompilerConfig::default()).unwrap();
    let pipeline = compiler
        .request_compute_spirv(shader(), c"main".to_owned(), layout)
        .unwrap()
        .wait(Duration::from_secs(10))
        .unwrap();
    let mut session = VulkanGraphSession::new(
        VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap(),
    );
    let token = session
        .backend
        .bind_import(GpuResource::Buffer(buffer.clone()))
        .unwrap();
    for (complete, access) in [(false, 0x32), (true, 0x32), (true, 0x37)] {
        let mut graph = session.begin_frame().unwrap();
        let target = graph
            .import_unowned(
                "heap target",
                ResourceSpec::Buffer(buffer.desc).descriptor(),
                token,
                0x26,
            )
            .unwrap();
        let mut pass = graph.add_pass("checked heap range").with_side_effect();
        pass.add_resource_access_range(
            target,
            access,
            0,
            AccessRange::Buffer {
                offset: 0,
                size: if complete { 64 } else { 32 },
            },
        )
        .unwrap();
        let (heap, lease, pipeline, device) = (
            heap.clone(),
            lease.clone(),
            pipeline.clone(),
            device.clone(),
        );
        pass.render_with_result_context(move |ctx: PassContext<'_>| {
            unsafe {
                heap.bind_pass(&ctx, pipeline.clone(), 0, std::slice::from_ref(&lease))?;
                let command = vk::CommandBuffer::from_raw(ctx.command.handle());
                pipeline.bind_pipeline(command);
                device.raw_device().cmd_push_constants(
                    command,
                    pipeline.layout.raw_handle(),
                    vk::ShaderStageFlags::COMPUTE,
                    0,
                    &lease.index().index.to_le_bytes(),
                );
                device.raw_device().cmd_dispatch(command, 1, 1, 1);
            }
            Ok(())
        });
        if complete {
            graph.export(target, 0x26).unwrap();
        }
        let result = session.execute_frame(graph.build());
        if complete {
            result.unwrap();
            session.wait_submits_done(10_000_000_000).unwrap();
            let mut actual = [0; 64];
            unsafe {
                buffer.read_bytes_after_completion(0, &mut actual).unwrap();
            }
            assert_eq!(&actual[..4], &77u32.to_le_bytes());
            assert!(actual[4..].iter().all(|&byte| byte == 0));
        } else {
            let error = result.unwrap_err();
            assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
            assert!(
                session.backend.submissions.is_empty(),
                "rejected binding cannot publish a GPU point"
            );
        }
    }
}

#[test]
fn table_capacity_bad_ranges_and_foreign_devices_are_rejected_before_update() {
    let Some(device) = device() else {
        return;
    };
    assert!(
        BindlessHeap::new(
            device.clone(),
            BindlessConfig {
                storage_images: 0,
                ..small()
            }
        )
        .is_err()
    );
    let heap = BindlessHeap::new(device.clone(), small()).unwrap();
    let buffer = target(&device);
    assert!(heap.storage_buffer(buffer.clone(), 0, 0).is_err());
    assert!(heap.storage_buffer(buffer.clone(), 1, 4).is_err());
    let first = heap.storage_buffer(buffer.clone(), 0, 4).unwrap();
    assert!(matches!(
        heap.storage_buffer(buffer.clone(), 0, 4),
        Err(rhapsody::Error::Exhausted(_))
    ));
    drop(first);
    // A lease that no command retained is reusable without collection.
    assert_eq!(heap.collect().unwrap(), 0);
    let next = heap.storage_buffer(buffer, 0, 4).unwrap();
    let other = Device::new_headless(&DeviceConfig::headless()).unwrap();
    assert!(heap.storage_buffer(target(&other), 0, 4).is_err());
    drop(next);
}

#[test]
fn known_host_slot_storage_is_admitted_and_released_by_the_unified_budget() {
    let Some(device) = device() else {
        return;
    };
    let baseline = device.allocator_stats().cpu_memory_usage().total();
    device
        .allocator_stats()
        .set_total_budget(baseline + 4096)
        .unwrap();
    assert!(matches!(
        BindlessHeap::new(device.clone(), BindlessConfig::default()),
        Err(rhapsody::Error::OutOfHostMemory)
    ));
    assert_eq!(
        device.allocator_stats().cpu_memory_usage().total(),
        baseline
    );
    assert_eq!(device.memory_budget_usage().unwrap().total(), baseline);
    // The full heap also charges its descriptor layout, pool, set and cells.
    // Do not assume their combined native-wrapper metadata fits in 4 KiB.
    device.allocator_stats().set_total_budget(u64::MAX).unwrap();
    let heap = BindlessHeap::new(device.clone(), small()).unwrap();
    let bytes = device.allocator_stats().cpu_memory_usage().total();
    assert!(bytes > baseline);
    assert_eq!(device.memory_budget_usage().unwrap().total(), bytes);
    drop(heap);
    device.try_collect_all_completed().unwrap();
    assert_eq!(
        device.allocator_stats().cpu_memory_usage().total(),
        baseline
    );
    assert_eq!(device.memory_budget_usage().unwrap().total(), baseline);
}

/// 只做真实布局支持诊断；由唯一 GPU 验证 owner 显式运行，不改变游戏资源或绘制数。
#[test]
#[ignore = "由 root 显式运行真实设备布局矩阵"]
fn actual_bindless_layout_support_matrix() {
    use rhapsody::raw::descriptor_set::{LayoutBinding, LayoutPlan, VulkanDescriptorSetLayout};
    // 矩阵包含变长绑定查询，必须先启用该 feature，不能用未启用特性的输入测支持。
    let mut config = DeviceConfig::headless();
    config.required_features = config.required_features.with(
        rhapsody::device::Feature::DescriptorBindingVariableDescriptorCount,
    );
    let Some(device) = gpu_test_support::initialize(
        "bindless layout matrix",
        Device::new_headless(&config),
    ) else { return };
    let kinds = [
        vk::DescriptorType::SAMPLED_IMAGE,
        vk::DescriptorType::STORAGE_IMAGE,
        vk::DescriptorType::STORAGE_BUFFER,
        vk::DescriptorType::SAMPLER,
    ];
    for (name, counts, variable_kind) in [
        ("production_fixed", [8192, 256, 16384, 64], None),
        ("default_fixed", [256, 256, 256, 64], None),
        ("below_1212", [1, 1, 1208, 1], None),
        ("at_1212", [1, 1, 1209, 1], None),
        ("above_1212", [1, 1, 1210, 1], None),
        ("page_query_only", [128, 64, 960, 32], None),
        ("single_storage_fixed_16384", [0, 0, 16384, 0], None),
        ("single_storage_variable_16384", [0, 0, 16384, 0], Some(2)),
        ("single_storage_variable_31", [0, 0, 31, 0], Some(2)),
        ("single_storage_variable_32", [0, 0, 32, 0], Some(2)),
    ] {
        let nonzero = counts.iter().enumerate().filter(|(_, count)| **count != 0);
        let bindings: Vec<_> = nonzero
            .clone()
            .map(|(kind, count)| {
                vk::DescriptorSetLayoutBinding::default()
                    .binding(kind as u32)
                    .descriptor_type(kinds[kind])
                    .descriptor_count(*count)
                    .stage_flags(vk::ShaderStageFlags::ALL)
            })
            .collect();
        let common = vk::DescriptorBindingFlags::UPDATE_AFTER_BIND
            | vk::DescriptorBindingFlags::PARTIALLY_BOUND
            | vk::DescriptorBindingFlags::UPDATE_UNUSED_WHILE_PENDING;
        let flags: Vec<_> = nonzero
            .map(|(kind, _)| {
                if variable_kind == Some(kind) {
                    common | vk::DescriptorBindingFlags::VARIABLE_DESCRIPTOR_COUNT
                } else {
                    common
                }
            })
            .collect();
        let mut flag_info =
            vk::DescriptorSetLayoutBindingFlagsCreateInfo::default().binding_flags(&flags);
        let info = vk::DescriptorSetLayoutCreateInfo::default()
            .flags(vk::DescriptorSetLayoutCreateFlags::UPDATE_AFTER_BIND_POOL)
            .bindings(&bindings)
            .push_next(&mut flag_info);
        let mut variable_support = vk::DescriptorSetVariableDescriptorCountLayoutSupport::default();
        let mut support = vk::DescriptorSetLayoutSupport::default();
        if variable_kind.is_some() {
            support = support.push_next(&mut variable_support);
        }
        unsafe {
            device
                .raw_device()
                .get_descriptor_set_layout_support(&info, &mut support)
        };
        let supported = support.supported != vk::FALSE;
        eprintln!(
            "LAYOUT_MATRIX name={name} counts={counts:?} total={} variable={variable_kind:?} supported={supported} max_variable={}",
            counts.iter().sum::<u32>(),
            variable_support.max_variable_descriptor_count
        );
        if variable_kind.is_none() {
            let plan = LayoutPlan {
                flags: info.flags.as_raw(),
                bindings: bindings
                    .iter()
                    .map(|binding| LayoutBinding {
                        binding: binding.binding,
                        descriptor_type: binding.descriptor_type.as_raw(),
                        descriptor_count: binding.descriptor_count,
                        stage_flags: binding.stage_flags.as_raw(),
                        immutable_sampler: None,
                    })
                    .collect(),
                binding_flags: flags.iter().map(|flag| flag.as_raw()).collect(),
            };
            let created = VulkanDescriptorSetLayout::new(device.clone(), &plan);
            if supported {
                assert!(created.is_ok(), "supported layout {name} failed creation");
            } else {
                assert!(
                    matches!(created, Err(rhapsody::Error::Unsupported(_))),
                    "unsupported layout {name} was not rejected"
                );
            }
        }
    }
    assert_eq!(device.retirement_stats().idle_waits, 0);
}
