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
            let _ = self.0.device.device.signal_semaphore(
                &vk::SemaphoreSignalInfo::default()
                    .semaphore(self.0.handle)
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
    let buffer = target(&device);
    buffer.write_bytes(0, &0u32.to_le_bytes()).unwrap();
    let lease = heap.storage_buffer(buffer.clone(), 0, 4).unwrap();
    let first = lease.index();
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
    let pool = VulkanCommandPool::new(device.clone()).unwrap();
    let mut command = pool.allocate().unwrap();
    command.begin().unwrap();
    unsafe {
        pipeline.bind_pipeline(command.handle);
        heap.bind_command(
            &mut command,
            pipeline.clone(),
            0,
            std::slice::from_ref(&lease),
        )
        .unwrap();
        device.device.cmd_push_constants(
            command.handle,
            pipeline.layout.handle,
            vk::ShaderStageFlags::COMPUTE,
            0,
            &first.index.to_le_bytes(),
        );
        device.device.cmd_dispatch(command.handle, 1, 1, 1);
        let barrier = vk::BufferMemoryBarrier::default()
            .buffer(buffer.handle)
            .offset(0)
            .size(4)
            .src_access_mask(vk::AccessFlags::SHADER_WRITE)
            .dst_access_mask(vk::AccessFlags::HOST_READ)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED);
        device.device.cmd_pipeline_barrier(
            command.handle,
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
                handle: device.queue,
                family: device.queue_family,
                index: 0,
            },
            &[(gate.clone(), 1, vk::PipelineStageFlags::ALL_COMMANDS)],
            &[],
        )
        .unwrap();
    let point = pending.point;
    drop((lease, pending));
    assert_eq!(heap.stats().retiring, 1);
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
    let next = heap.storage_buffer(target(&device), 0, 4).unwrap();
    assert_eq!(next.index().index, first.index);
    assert!(next.index().generation > first.generation);
    assert_eq!(heap.stats().retiring, 0);
    assert_eq!(device.garbage_stats().idle_waits, 0);
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
                device.device.cmd_push_constants(
                    command,
                    pipeline.layout.handle,
                    vk::ShaderStageFlags::COMPUTE,
                    0,
                    &lease.index().index.to_le_bytes(),
                );
                device.device.cmd_dispatch(command, 1, 1, 1);
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
    let baseline = device.allocator_stats.cpu_memory_usage().total();
    device
        .allocator_stats
        .set_total_budget(baseline + 4096)
        .unwrap();
    assert!(matches!(
        BindlessHeap::new(device.clone(), BindlessConfig::default()),
        Err(rhapsody::Error::OutOfHostMemory)
    ));
    assert_eq!(device.allocator_stats.cpu_memory_usage().total(), baseline);
    assert_eq!(device.memory_budget_usage().unwrap().total(), baseline);
    // The full heap also charges its descriptor layout, pool, set and cells.
    // Do not assume their combined native-wrapper metadata fits in 4 KiB.
    device.allocator_stats.set_total_budget(u64::MAX).unwrap();
    let heap = BindlessHeap::new(device.clone(), small()).unwrap();
    let bytes = device.allocator_stats.cpu_memory_usage().total();
    assert!(bytes > baseline);
    assert_eq!(device.memory_budget_usage().unwrap().total(), bytes);
    drop(heap);
    device.try_collect_all_completed().unwrap();
    assert_eq!(device.allocator_stats.cpu_memory_usage().total(), baseline);
    assert_eq!(device.memory_budget_usage().unwrap().total(), baseline);
}
