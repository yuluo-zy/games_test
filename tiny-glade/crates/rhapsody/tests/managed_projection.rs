//! Production managed projection reuse, immutable external snapshots and bounds.
mod gpu_test_support;
use ash::vk::{self, Handle};
use rhapsody::{device::{Device, DeviceConfig}, rg::{executor::ExecutionBackend, graph::{BuiltGraph, RenderGraph}, pass_bindings::ManagedProjectionConfig, resource::ResourceHandle, vulkan::{AccessTable, GpuResource, ResourceSpec, VulkanGraphBackend, VulkanGraphSession}}};
use rhapsody::resource::buffer::{Buffer, BufferDesc};
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
                (1u64 << 62) | (index as u64 + 1),
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
        .import_unowned("input", descriptor, (1u64 << 62) | (source as u64 + 1), HOST)
        .unwrap();
    let output = graph.import_unowned("output", descriptor, (1u64 << 62) | 3, HOST).unwrap();
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
            source.device().raw_device().cmd_copy_buffer(
                vk::CommandBuffer::from_raw(context.command.handle()),
                source.raw_handle(),
                destination.raw_handle(),
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
    assert!(session.release_import((1u64 << 62) | 2).unwrap());
    drop(buffers);
    assert!(observer.upgrade().is_some());
    drop(next);
    session.collect_after_completion(&[(1u64 << 62) | 1, (1u64 << 62) | 3]).unwrap();
    assert!(observer.upgrade().is_none());
}

#[test]
fn constants_pages_retire_only_after_external_snapshot_and_weak_release() {
    use rhapsody::resource::{
        DYNAMIC_CONSTANTS_CHUNK_SIZE_BYTES as PAGE, DynamicConstants, DynamicConstantsConfig,
    };
    let Some((mut session, buffers)) = fixture() else { return; };
    let device = session.backend.device().clone();
    let idle_waits = device.retirement_stats().idle_waits;
    let mut constants = DynamicConstants::with_config(device, DynamicConstantsConfig {
        max_pages: 1, max_backing_bytes: PAGE, max_idle_pages: 1,
    }).unwrap();
    let chunk = constants.allocate(16).unwrap();
    chunk.write_bytes(&[0x37; 16]).unwrap();
    let pointer = Arc::as_ptr(&chunk.buffer) as usize;
    let aligned = constants.allocate(0).unwrap();
    let tail = constants.allocate(PAGE - aligned.offset).unwrap();
    drop((aligned, tail));
    let mut graph = session.begin_frame().unwrap();
    let input = session.import_frame_buffer(&mut graph, chunk.buffer.clone(), 0x35).unwrap();
    drop(chunk);
    let output = graph.import_unowned("snapshot output",
        ResourceSpec::Buffer(buffers[2].desc).descriptor(), (1u64 << 62) | 3, HOST).unwrap();
    let mut pass = graph.add_pass("constants snapshot lease").with_side_effect();
    pass.add_resource_access(input, 0x25, 0).unwrap();
    pass.add_resource_access(output, 0x34, 1).unwrap();
    let native = session.backend.device().clone();
    pass.render_with_result_context(move |context| {
        let managed = context.managed.unwrap();
        let input = managed.buffer_for(input.raw()).unwrap();
        let output = managed.buffer_for(output.raw()).unwrap();
        unsafe { native.raw_device().cmd_copy_buffer(
            vk::CommandBuffer::from_raw(context.command.handle()), input.raw(), output.raw(),
            &[vk::BufferCopy { src_offset: 0, dst_offset: 0, size: SIZE }]); }
        Ok(())
    });
    graph.export(input, HOST).unwrap();
    graph.export(output, HOST).unwrap();
    let receipt = session.execute_frame(graph.build()).unwrap();
    assert!(!receipt.receipts.is_empty());
    let held = session.backend.managed_pass_resources().unwrap();
    let key = held.buffer_for(input.raw()).unwrap().key();
    session.wait_frame(receipt.frame, 10_000_000_000).unwrap();
    let mut actual = [0; SIZE as usize];
    buffers[2].read_bytes(0, &mut actual).unwrap();
    assert_eq!(actual, [0x37; SIZE as usize]);
    assert_eq!(session.frame_import_count(), 0);
    assert!(matches!(constants.allocate(16), Err(rhapsody::Error::Timeout)));
    session.collect_completed_frames().unwrap();
    assert_eq!(held.buffer_for(input.raw()).unwrap().key(), key);
    let weak = Arc::downgrade(&held);
    drop(held);
    session.collect_completed_frames().unwrap();
    assert!(matches!(constants.allocate(16), Err(rhapsody::Error::Timeout)));
    assert_eq!(weak.upgrade().unwrap().buffer_for(input.raw()).unwrap().key(), key);
    drop(weak);
    // 没有新导入退役也必须重试空闲快照；不能靠下一帧重建或设备 idle 才归还页。
    session.collect_completed_frames().unwrap();
    let reused = constants.allocate(16).unwrap();
    assert_eq!(Arc::as_ptr(&reused.buffer) as usize, pointer);
    assert_eq!(reused.offset, 0);
    assert_eq!(constants.stats().page_reuses, 1);
    assert_eq!(session.backend.device().retirement_stats().idle_waits, idle_waits);
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

fn imported_shape(
    session: &mut VulkanGraphSession,
    buffers: &[Arc<Buffer>; 3],
    wide: bool,
) -> [rhapsody::resource::managed::resources::ManagedKey<Buffer>; 2] {
    let mut graph = session.begin_frame().unwrap();
    let source = usize::from(!wide);
    let input = session.import_frame_buffer(&mut graph, buffers[source].clone(), HOST).unwrap();
    let extra = wide.then(|| session.import_frame_buffer(&mut graph, buffers[1].clone(), HOST).unwrap());
    let output = session.import_frame_buffer(&mut graph, buffers[2].clone(), HOST).unwrap();
    let device = session.backend.device().clone();
    let keys = Arc::new(parking_lot::Mutex::new(None));
    let observed = keys.clone();
    let mut pass = graph.add_pass(if wide { "wide import shape" } else { "narrow import shape" })
        .with_side_effect();
    pass.add_resource_access(input, 0x25, 0).unwrap();
    if let Some(extra) = extra { pass.add_resource_access(extra, 0x25, 0).unwrap(); }
    pass.add_resource_access(output, 0x34, 1).unwrap();
    pass.render_with_result_context(move |context| {
        let managed = context.managed.unwrap();
        let input = managed.buffer_for(input.raw()).unwrap();
        let output = managed.buffer_for(output.raw()).unwrap();
        *observed.lock() = Some([input.key(), output.key()]);
        unsafe { device.raw_device().cmd_copy_buffer(
            vk::CommandBuffer::from_raw(context.command.handle()), input.raw(), output.raw(),
            &[vk::BufferCopy { src_offset: 0, dst_offset: 0, size: SIZE }]); }
        Ok(())
    });
    let receipt = session.execute_frame(graph.build()).unwrap();
    session.wait_frame(receipt.frame, 10_000_000_000).unwrap();
    assert_eq!(session.frame_import_count(), 0);
    assert!(session.backend.managed_pass_resources().is_none(), "expired declarations cannot be reused");
    let mut actual = [0; SIZE as usize];
    buffers[2].read_bytes(0, &mut actual).unwrap();
    assert_eq!(actual, [source as u8 + 1; SIZE as usize]);
    
    keys.lock().unwrap()
}

fn release_named_fixture_imports(session: &mut VulkanGraphSession) {
    for token in [(1u64 << 62) | 1, (1u64 << 62) | 2, (1u64 << 62) | 3] {
        assert!(session.release_import(token).unwrap());
    }
}

#[test]
fn alternating_frame_import_shapes_reuse_both_physical_owner_projections() {
    let Some((mut session, buffers)) = fixture() else { return; };
    release_named_fixture_imports(&mut session);
    session.backend.set_managed_projection_limits(ManagedProjectionConfig {
        max_snapshots: 2, max_resources: 3, max_bytes: 1 << 16,
    }).unwrap();
    let a = imported_shape(&mut session, &buffers, true);
    let b = imported_shape(&mut session, &buffers, false);
    assert_eq!(imported_shape(&mut session, &buffers, true), a);
    assert_eq!(imported_shape(&mut session, &buffers, false), b);
    let warm = session.backend.managed_projection_stats();
    assert_eq!(warm.snapshots, 2);
    assert_eq!(warm.managed_allocations, 5);
    for _ in 0..8 {
        assert_eq!(imported_shape(&mut session, &buffers, true), a);
        assert_eq!(imported_shape(&mut session, &buffers, false), b);
    }
    let after = session.backend.managed_projection_stats();
    assert_eq!(after.snapshots, 2);
    assert_eq!(after.managed_allocations, warm.managed_allocations);
    assert_eq!(after.capacity_growths, warm.capacity_growths);
    assert_eq!(after.bytes, warm.bytes);
}

#[test]
fn projection_only_native_owners_are_reclaimed_even_with_zero_reported_cache_bytes() {
    let Some((mut session, buffers)) = fixture() else { return; };
    release_named_fixture_imports(&mut session);
    imported_shape(&mut session, &buffers, true);
    let observed = buffers.each_ref().map(Arc::downgrade);
    drop(buffers);
    let device = session.backend.device().clone();
    // Advance the actual command-pool ring so completed commands relinquish
    // their owners. The bounded idle projection is then the remaining cache.
    for _ in 0..device.frames_in_flight() + 1 {
        let mut graph = session.begin_frame().unwrap();
        graph.add_pass("empty owner maintenance").with_side_effect().render(|_| true);
        let receipt = session.execute_frame(graph.build()).unwrap();
        session.wait_frame(receipt.frame, 10_000_000_000).unwrap();
    }
    assert!(observed.iter().all(|owner| owner.upgrade().is_some()));
    assert_eq!(session.cache_stats().parked_resources, 0);
    let before = device.allocator_stats().device_memory_usage().total();
    assert_eq!(device.evict_idle_caches(u64::MAX), 0);
    assert!(observed.iter().all(|owner| owner.upgrade().is_none()));
    assert!(device.allocator_stats().device_memory_usage().total() < before);
    assert_eq!(device.retirement_stats().idle_waits, 0);
}
