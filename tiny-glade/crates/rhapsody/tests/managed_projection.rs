//! Production managed projection reuse, immutable external snapshots and bounds.
mod gpu_test_support;
use ash::vk::{self, Handle};
use rhapsody::{
    device::{Device, DeviceConfig},
    raw::buffer::{Buffer, BufferDesc},
    rg::{
        executor::ExecutionBackend,
        graph::{BuiltGraph, RenderGraph},
        pass_bindings::ManagedProjectionConfig,
        resource::ResourceHandle,
        vulkan::{AccessTable, GpuResource, ResourceSpec, VulkanGraphBackend, VulkanGraphSession},
    },
};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

const SIZE: u64 = 16;
const HOST: u8 = 0x26;
fn fixture() -> Option<(VulkanGraphSession, [Arc<Buffer>; 3])> {
    let device = gpu_test_support::initialize(
        "managed projection",
        Device::new_headless(&DeviceConfig::headless()),
    )?;
    let desc = BufferDesc::new_gpu_to_cpu(
        SIZE,
        vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST,
    );
    let buffers = std::array::from_fn(|index| {
        let buffer = Arc::new(Buffer::new(device.clone(), &desc).unwrap());
        buffer
            .write_bytes(0, &[index as u8 + 1; SIZE as usize])
            .unwrap();
        buffer
    });
    let mut backend = VulkanGraphBackend::new(device, AccessTable::original()).unwrap();
    for (index, buffer) in buffers.iter().enumerate() {
        backend
            .bind_import_named(
                ["first", "second", "output"][index],
                index as u64 + 1,
                GpuResource::Buffer(buffer.clone()),
            )
            .unwrap();
    }
    Some((VulkanGraphSession::new(backend), buffers))
}
fn graph(
    mut graph: RenderGraph,
    buffers: &[Arc<Buffer>; 3],
    source: usize,
    recorded: &Arc<AtomicUsize>,
) -> (BuiltGraph, [ResourceHandle; 2]) {
    let descriptor = ResourceSpec::Buffer(buffers[0].desc).descriptor();
    let input = graph
        .import_unowned("input", descriptor, source as u64 + 1, HOST)
        .unwrap();
    let output = graph.import_unowned("output", descriptor, 3, HOST).unwrap();
    let mut pass = graph.add_pass("projection copy");
    pass.add_resource_access(input, 0x25, 0).unwrap();
    pass.add_resource_access(output, 0x34, 1).unwrap();
    let source = buffers[source].clone();
    let destination = buffers[2].clone();
    let recorded = recorded.clone();
    pass.render_with_result_context(move |context| {
        // These same-device physical owners are declared above and retained by
        // the real production command's SyncCells, independently of snapshots.
        unsafe {
            source.device.raw_device().cmd_copy_buffer(
                vk::CommandBuffer::from_raw(context.command.handle()),
                source.handle,
                destination.handle,
                &[vk::BufferCopy {
                    src_offset: 0,
                    dst_offset: 0,
                    size: SIZE,
                }],
            );
        }
        recorded.fetch_add(1, Ordering::Relaxed);
        Ok(())
    });
    graph.export(output, HOST).unwrap();
    (graph.build(), [input, output])
}
fn run(
    session: &mut VulkanGraphSession,
    buffers: &[Arc<Buffer>; 3],
    source: usize,
    recorded: &Arc<AtomicUsize>,
) -> [ResourceHandle; 2] {
    let (graph, handles) = graph(session.begin_frame().unwrap(), buffers, source, recorded);
    let receipt = session.execute_frame(graph).unwrap();
    session.wait_frame(receipt.frame, 10_000_000_000).unwrap();
    let mut actual = [0; SIZE as usize];
    buffers[2].read_bytes(0, &mut actual).unwrap();
    assert_eq!(actual, [source as u8 + 1; SIZE as usize]);
    handles
}

#[test]
fn stable_physical_resources_reuse_managed_generations_and_projection_storage() {
    let Some((mut session, buffers)) = fixture() else {
        return;
    };
    let recorded = Arc::new(AtomicUsize::new(0));
    let handles = run(&mut session, &buffers, 0, &recorded);
    let snapshot = session.backend.managed_pass_resources().unwrap();
    let key = snapshot.buffer_for(handles[0].raw()).unwrap().key();
    drop(snapshot);
    // Both staging registries must warm before checking fixed capacity.
    run(&mut session, &buffers, 0, &recorded);
    let warm = session.backend.managed_projection_stats();
    for _ in 0..8 {
        let handles = run(&mut session, &buffers, 0, &recorded);
        let snapshot = session.backend.managed_pass_resources().unwrap();
        assert_eq!(snapshot.buffer_for(handles[0].raw()).unwrap().key(), key);
    }
    let after = session.backend.managed_projection_stats();
    assert_eq!(after.managed_allocations, 2);
    assert_eq!(after.managed_allocations, warm.managed_allocations);
    assert_eq!(after.capacity_growths, warm.capacity_growths);
    assert_eq!(after.bytes, warm.bytes);
    assert_eq!(recorded.load(Ordering::Relaxed), 10);
    session.collect_after_completion(&[]).unwrap();
    assert!(session.backend.managed_pass_resources().is_none());
}

#[test]
fn externally_held_snapshot_is_immutable_and_capacity_recovers_after_release() {
    let Some((mut session, buffers)) = fixture() else {
        return;
    };
    session
        .backend
        .set_managed_projection_limits(ManagedProjectionConfig {
            max_snapshots: 1,
            max_resources: 2,
            max_bytes: 1 << 20,
        })
        .unwrap();
    let recorded = Arc::new(AtomicUsize::new(0));
    let handles = run(&mut session, &buffers, 0, &recorded);
    let held = session.backend.managed_pass_resources().unwrap();
    let old_key = held.buffer_for(handles[0].raw()).unwrap().key();
    let old_raw = held.buffer_for(handles[0].raw()).unwrap().shared();
    let (next, _) = graph(session.begin_frame().unwrap(), &buffers, 1, &recorded);
    assert!(matches!(
        session.execute_frame(next),
        Err(rhapsody::Error::Timeout)
    ));
    assert_eq!(recorded.load(Ordering::Relaxed), 1);
    assert_eq!(held.buffer_for(handles[0].raw()).unwrap().key(), old_key);
    assert!(Arc::ptr_eq(&old_raw, &buffers[0]));
    assert_eq!(
        session.backend.managed_projection_stats().capacity_stalls,
        1
    );
    let weak = Arc::downgrade(&held);
    drop(held);
    let (next, _) = graph(session.begin_frame().unwrap(), &buffers, 1, &recorded);
    assert!(matches!(
        session.execute_frame(next),
        Err(rhapsody::Error::Timeout)
    ));
    assert!(weak.upgrade().is_some());
    assert_eq!(recorded.load(Ordering::Relaxed), 1);
    drop(weak);
    let next_handles = run(&mut session, &buffers, 1, &recorded);
    let next = session.backend.managed_pass_resources().unwrap();
    assert!(Arc::ptr_eq(
        &next.buffer_for(next_handles[0].raw()).unwrap().shared(),
        &buffers[1]
    ));
    assert_eq!(recorded.load(Ordering::Relaxed), 2);
    // Removing a backend import clears idle projections. An actual external
    // snapshot remains an owner until its own lifetime ends.
    let observer = Arc::downgrade(&buffers[1]);
    assert!(session.release_import(2).unwrap());
    drop(buffers);
    assert!(observer.upgrade().is_some());
    drop(next);
    session.collect_after_completion(&[1, 3]).unwrap();
    assert!(observer.upgrade().is_none());
}

#[test]
fn resource_bound_failure_precedes_recording_and_a_larger_limit_recovers() {
    let Some((mut session, buffers)) = fixture() else {
        return;
    };
    session
        .backend
        .set_managed_projection_limits(ManagedProjectionConfig {
            max_snapshots: 1,
            max_resources: 1,
            max_bytes: 1 << 20,
        })
        .unwrap();
    let recorded = Arc::new(AtomicUsize::new(0));
    let (built, _) = graph(session.begin_frame().unwrap(), &buffers, 0, &recorded);
    assert!(matches!(
        session.execute_frame(built),
        Err(rhapsody::Error::Timeout)
    ));
    assert_eq!(recorded.load(Ordering::Relaxed), 0);
    session
        .backend
        .set_managed_projection_limits(ManagedProjectionConfig {
            max_snapshots: 1,
            max_resources: 2,
            max_bytes: 1 << 20,
        })
        .unwrap();
    run(&mut session, &buffers, 0, &recorded);
    assert_eq!(recorded.load(Ordering::Relaxed), 1);
}
