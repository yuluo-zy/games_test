//! Aliased transient buffers: the second member of a shared backing must be
//! ordered after the first member's last access by a real barrier, and the
//! cached backing must be reused across retired frames.
mod gpu_test_support;
use ash::vk::{self, Handle};
use rhapsody::{
    device::{AllocationStrategy, Device, DeviceConfig},
    raw::buffer::{Buffer, BufferDesc},
    rg::{
        executor::{ExecutionBackend, PassContext},
        graph::RenderGraph,
        sync_plan::BarrierKind,
        vulkan::{AccessTable, GpuResource, ResourceSpec, VulkanGraphBackend, VulkanGraphSession},
    },
};
use std::sync::Arc;
const READ: u8 = 0x25;
const WRITE: u8 = 0x34;
const HOST: u8 = 0x26;
const WORDS: u64 = 4096;
const BYTES: u64 = WORDS * 4;
const TIMEOUT: u64 = 10_000_000_000;

fn device() -> Option<Arc<Device>> {
    gpu_test_support::initialize(
        "transient buffer aliasing",
        Device::new_headless(&DeviceConfig {
            allocation: AllocationStrategy::Dedicated,
            ..DeviceConfig::headless()
        }),
    )
}

/// fill(a)=0xA; copy a->out[0]; fill(b)=0xB; copy b->out[1]. A bridge buffer
/// orders "copy a" before "fill b", which is what makes a/b alias candidates.
fn graph(session: &mut VulkanGraphSession, readback: Arc<Buffer>) -> RenderGraph {
    let mut graph = session.begin_frame().unwrap();
    graph.batch_per_pass(true);
    let spec = ResourceSpec::Buffer(BufferDesc::new_gpu_only(
        BYTES,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::TRANSFER_SRC,
    ));
    let a = session.backend.declare(&mut graph, "a", spec).unwrap();
    let b = session.backend.declare(&mut graph, "b", spec).unwrap();
    let bridge = session
        .backend
        .declare(
            &mut graph,
            "bridge",
            ResourceSpec::Buffer(BufferDesc::new_gpu_only(
                16,
                vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::TRANSFER_SRC,
            )),
        )
        .unwrap();
    let rb_token = session
        .backend
        .bind_import(GpuResource::Buffer(readback.clone()))
        .unwrap();
    let out = graph
        .import_unowned(
            "out",
            ResourceSpec::Buffer(readback.desc).descriptor(),
            rb_token,
            HOST,
        )
        .unwrap();
    let device = session.backend.device.clone();
    for (buffer_id, value, offset) in [(a, 0xAu32, 0), (b, 0xBu32, BYTES)] {
        let mut pass = graph.add_pass(format!("fill {offset}")).with_side_effect();
        pass.add_resource_access(buffer_id, WRITE, 0).unwrap();
        if offset != 0 {
            pass.add_resource_access(bridge, READ, 0).unwrap();
        }
        let fill_device = device.clone();
        let index = buffer_id.raw();
        pass.render_with_result_context(move |ctx: PassContext<'_>| {
            let managed = ctx
                .managed
                .ok_or_else(|| rhapsody::Error::invalid_usage("missing managed resources"))?;
            let buffer = managed
                .buffer_for(index)
                .map_err(|e| rhapsody::Error::invalid_usage_with("invalid pass binding", e))?;
            unsafe {
                fill_device.raw_device().cmd_fill_buffer(
                    vk::CommandBuffer::from_raw(ctx.command.handle()),
                    buffer.raw(),
                    0,
                    BYTES,
                    value,
                );
            }
            Ok(())
        });
        let mut pass = graph.add_pass(format!("copy {offset}")).with_side_effect();
        pass.add_resource_access(buffer_id, READ, 0).unwrap();
        pass.add_resource_access(out, WRITE, 0).unwrap();
        if offset == 0 {
            pass.add_resource_access(bridge, WRITE, 0).unwrap();
        }
        let copy_device = device.clone();
        let out_id = out.raw();
        let bridge_id = bridge.raw();
        pass.render_with_result_context(move |ctx: PassContext<'_>| {
            let managed = ctx
                .managed
                .ok_or_else(|| rhapsody::Error::invalid_usage("missing managed resources"))?;
            let source = managed
                .buffer_for(index)
                .map_err(|e| rhapsody::Error::invalid_usage_with("invalid pass binding", e))?;
            let output = managed
                .buffer_for(out_id)
                .map_err(|e| rhapsody::Error::invalid_usage_with("invalid pass binding", e))?;
            let command = vk::CommandBuffer::from_raw(ctx.command.handle());
            unsafe {
                copy_device.raw_device().cmd_copy_buffer(
                    command,
                    source.raw(),
                    output.raw(),
                    &[vk::BufferCopy {
                        src_offset: 0,
                        dst_offset: offset,
                        size: BYTES,
                    }],
                );
            }
            if offset == 0 {
                let bridge = managed
                    .buffer_for(bridge_id)
                    .map_err(|e| rhapsody::Error::invalid_usage_with("invalid pass binding", e))?;
                unsafe {
                    copy_device
                        .raw_device()
                        .cmd_fill_buffer(command, bridge.raw(), 0, 16, 1);
                }
            }
            Ok(())
        });
    }
    graph.export(out, HOST).unwrap();
    graph
}

fn logical(built: &rhapsody::rg::graph::BuiltGraph, name: &str) -> u32 {
    built
        .declaration
        .resources
        .iter()
        .position(|resource| resource.name == name)
        .unwrap() as u32
}

#[test]
fn aliased_buffers_get_a_backing_barrier_and_correct_contents() {
    let Some(device) = device() else { return };
    let readback = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_to_cpu(BYTES * 2, vk::BufferUsageFlags::TRANSFER_DST),
        )
        .unwrap(),
    );
    // Transient aliasing is on by default.
    let backend = VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap();
    let counters = backend.transient_backing_counters();
    let mut session = VulkanGraphSession::new(backend);
    let mut allocation_ids = Vec::new();
    for frame in 0..3 {
        let built = graph(&mut session, readback.clone()).build();
        let (a, b) = (logical(&built, "a"), logical(&built, "b"));
        let context = session.backend.plan_context();
        let definition = rhapsody::rg::plan::GraphDefinition::from_declaration(
            &built.declaration,
            &mut session.backend.accesses,
            context,
        )
        .unwrap();
        let program = rhapsody::rg::plan::compile_definition(definition)
            .unwrap()
            .range_program
            .clone();
        session.execute_frame(built).unwrap();
        let plan = session.backend.transient_memory_plan();
        let shared = plan
            .backings
            .iter()
            .filter(|backing| backing.resources.len() > 1)
            .collect::<Vec<_>>();
        assert_eq!(
            shared.len(),
            1,
            "a and b share one backing:\n{}",
            plan.dump()
        );
        assert!(shared[0].resources.contains(&a) && shared[0].resources.contains(&b));
        allocation_ids.push(shared[0].allocation_id.expect("allocated backing"));
        let sync = session.last_synchronization();
        // B's first access is a write to a never-written buffer: without the
        // alias dependency the planner had nothing to wait for.
        let alias = sync
            .transitions()
            .iter()
            .find(|transition| transition.resource == b && transition.kind == BarrierKind::Memory)
            .expect("aliased buffer first use emits a barrier");
        assert!(
            alias
                .source
                .stages
                .contains(vk::PipelineStageFlags::TRANSFER),
            "alias barrier waits for the previous member's transfer accesses: {alias:?}"
        );
        assert!(
            sync.aliases()
                .iter()
                .any(|edge| edge.previous == a && edge.next == b),
            "bound program records the alias edge for verification"
        );
        rhapsody::rg::plan_check::check_bound_synchronization(&program, sync).unwrap();
        session.wait_submits_done(TIMEOUT).unwrap();
        let mut words = vec![0u8; BYTES as usize * 2];
        unsafe {
            readback.read_bytes_after_completion(0, &mut words).unwrap();
        }
        for (index, word) in words.as_chunks::<4>().0.iter().enumerate() {
            let expected = if (index as u64) < WORDS { 0xA } else { 0xB };
            assert_eq!(
                u32::from_le_bytes(*word),
                expected,
                "frame {frame} word {index}"
            );
        }
    }
    assert!(
        allocation_ids.windows(2).all(|pair| pair[0] == pair[1]),
        "retired alias backings are reused instead of reallocated: {allocation_ids:?}"
    );
    assert_eq!(counters.snapshot().allocations, 1);
    drop(session);
    drop(readback);
    device.try_collect_all_completed().unwrap();
    let actual = counters.snapshot();
    assert_eq!(actual.allocations, actual.frees);
    assert_eq!(
        device
            .allocator_stats()
            .report()
            .totals
            .total_allocated_bytes,
        0
    );
}
