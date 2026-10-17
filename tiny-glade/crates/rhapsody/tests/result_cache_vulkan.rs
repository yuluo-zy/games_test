//! Actual persistent outputs, asynchronous hits, invalidation and failed frames.
mod gpu_test_support;
use ash::vk::{self, Handle};
use rhapsody::{
    raw::{
        buffer::{Buffer, BufferDesc},
        device::Device,
        init::create_headless_device,
        semaphore::TimelineSemaphore,
    },
    rg::{
        graph::RenderGraph,
        resource::ResourceHandle,
        result_cache::{
            ContentDependency, DependencyId, ResultDependencies, ResultLookup, ResultNodeId,
            ResultRequest,
        },
        vulkan::{AccessTable, GpuResource, ResourceSpec, VulkanGraphBackend, VulkanGraphSession},
    },
};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
const READ: u8 = 0x25;
const WRITE: u8 = 0x34;
const HOST: u8 = 0x26;
fn session() -> Option<VulkanGraphSession> {
    let device = gpu_test_support::initialize("result-cache device", create_headless_device())?;
    let backend = gpu_test_support::initialize(
        "result-cache backend",
        VulkanGraphBackend::new(device, AccessTable::original()),
    )?;
    Some(VulkanGraphSession::new(backend))
}
fn request(node: u64, version: u64) -> ResultRequest {
    ResultRequest {
        node: ResultNodeId(node),
        dependencies: ResultDependencies::Complete(vec![ContentDependency::external(1, version)]),
        parameters: Vec::new(),
        shader_implementation: 1,
        shader_interface: 1,
    }
}
fn spec() -> ResourceSpec {
    ResourceSpec::Buffer(BufferDesc::new_gpu_only(
        64,
        vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST,
    ))
}
fn filled(
    session: &mut VulkanGraphSession,
    graph: &mut RenderGraph,
    request: ResultRequest,
    value: u32,
    calls: Arc<AtomicUsize>,
) -> ResultLookup {
    let device = session.backend.device.clone();
    session
        .cached_result(
            graph,
            request,
            spec(),
            READ,
            move |_session, graph, output| {
                let mut pass = graph.add_pass("produce cached fill");
                pass.add_resource_access(output, WRITE, 0)?;
                pass.render_with_context(move |ctx| {
                    calls.fetch_add(1, Ordering::SeqCst);
                    let Some(managed) = ctx.managed else {
                        return false;
                    };
                    let Ok(buffer) = managed.buffer_for(output.raw()) else {
                        return false;
                    };

                    unsafe {
                        device.device.cmd_fill_buffer(
                            vk::CommandBuffer::from_raw(ctx.command.handle()),
                            buffer.raw(),
                            0,
                            64,
                            value,
                        );
                    }
                    true
                });
                Ok(())
            },
        )
        .unwrap()
}
fn copy(
    graph: &mut RenderGraph,
    device: Arc<Device>,
    source: ResourceHandle,
    target: ResourceHandle,
) {
    let mut pass = graph.add_pass("consume persistent result");
    pass.add_resource_access(source, READ, 0).unwrap();
    pass.add_resource_access(target, WRITE, 0).unwrap();
    pass.render_with_context(move |ctx| {
        let Some(managed) = ctx.managed else {
            return false;
        };
        let (Ok(source), Ok(target)) = (
            managed.buffer_for(source.raw()),
            managed.buffer_for(target.raw()),
        ) else {
            return false;
        };

        unsafe {
            device.device.cmd_copy_buffer(
                vk::CommandBuffer::from_raw(ctx.command.handle()),
                source.raw(),
                target.raw(),
                &[vk::BufferCopy {
                    src_offset: 0,
                    dst_offset: 0,
                    size: 64,
                }],
            );
        }
        true
    });
}
fn readback(session: &mut VulkanGraphSession) -> (Arc<Buffer>, u64) {
    let buffer = Arc::new(
        Buffer::new(
            session.backend.device.clone(),
            &BufferDesc::new_gpu_to_cpu(64, vk::BufferUsageFlags::TRANSFER_DST),
        )
        .unwrap(),
    );
    let token = session
        .backend
        .bind_import(GpuResource::Buffer(buffer.clone()))
        .unwrap();
    (buffer, token)
}
fn consume(
    session: &VulkanGraphSession,
    graph: &mut RenderGraph,
    result: &ResultLookup,
    buffer: &Arc<Buffer>,
    token: u64,
) {
    let target = graph
        .import_unowned(
            "readback",
            ResourceSpec::Buffer(buffer.desc).descriptor(),
            token,
            HOST,
        )
        .unwrap();
    copy(graph, session.backend.device.clone(), result.handle, target);
    graph.export(target, HOST).unwrap();
}
fn check(buffer: &Buffer, expected: u32) {
    let mut bytes = [0; 64];
    unsafe {
        buffer.read_bytes_after_completion(0, &mut bytes).unwrap();
    }
    for word in bytes.as_chunks::<4>().0 {
        assert_eq!(u32::from_ne_bytes(*word), expected);
    }
}
struct Gate(Arc<TimelineSemaphore>);
impl Gate {
    fn signal(&self) {
        if self.0.completed().unwrap_or(1) < 1 {
            unsafe {
                self.0
                    .device
                    .device
                    .signal_semaphore(
                        &vk::SemaphoreSignalInfo::default()
                            .semaphore(self.0.handle)
                            .value(1),
                    )
                    .unwrap();
            }
        }
    }
}
impl Drop for Gate {
    fn drop(&mut self) {
        self.signal();
    }
}

#[test]
fn an_in_flight_hit_skips_production_keeps_its_real_completion_and_survives_eviction() {
    let Some(mut session) = session() else {
        return;
    };
    let (buffer, token) = readback(&mut session);
    let calls = Arc::new(AtomicUsize::new(0));
    let gate = Gate(TimelineSemaphore::new(session.backend.device.clone(), 0).unwrap());
    let mut graph = session.begin_frame().unwrap();
    let fresh = filled(
        &mut session,
        &mut graph,
        request(10, 0),
        0x1122_3344,
        calls.clone(),
    );
    assert!(!fresh.hit);
    consume(&session, &mut graph, &fresh, &buffer, token);
    session
        .backend
        .set_graph_synchronization(
            vec![(gate.0.clone(), 1, vk::PipelineStageFlags::ALL_COMMANDS)],
            Vec::new(),
        )
        .unwrap();
    let producer = session.execute_frame(graph.build()).unwrap();
    let mut graph = session.begin_frame().unwrap();
    let hit = filled(
        &mut session,
        &mut graph,
        request(10, 0),
        0x1122_3344,
        calls.clone(),
    );
    let not_completed = !hit
        .produced
        .is_reached(&session.backend.device.poll().unwrap());
    consume(&session, &mut graph, &hit, &buffer, token);
    session.execute_frame(graph.build()).unwrap();
    session.clear_result_cache();
    session.collect_completed_frames().unwrap();
    let retained_before_signal = session.result_cache_stats().retired;
    gate.signal(); // Always release before assertions and on panic via Drop.
    assert!(hit.hit);
    assert!(
        not_completed,
        "the external gate proves this hit was still in flight"
    );
    assert!(!hit.produced.is_empty());
    assert!(hit.produced.get(producer.receipts[0].queue) >= producer.receipts[0].value);
    assert_eq!(
        calls.load(Ordering::SeqCst),
        1,
        "a cache hit must skip the actual producer callback"
    );
    assert_eq!(
        retained_before_signal, 1,
        "eviction must preserve the real in-flight output"
    );
    session.wait_submits_done(10_000_000_000).unwrap();
    check(&buffer, 0x1122_3344);
    assert_eq!(session.result_cache_stats().retired, 0);
    assert_eq!(
        session.resource_count(),
        1,
        "only the caller-owned readback remains after eviction and completion"
    );
    session.release_import(token).unwrap();
}

#[test]
fn failed_or_partial_productions_never_replace_a_valid_cached_result() {
    let Some(mut session) = session() else {
        return;
    };
    let (buffer, token) = readback(&mut session);
    let calls = Arc::new(AtomicUsize::new(0));
    let mut graph = session.begin_frame().unwrap();
    let old = filled(&mut session, &mut graph, request(20, 0), 17, calls.clone());
    consume(&session, &mut graph, &old, &buffer, token);
    session.execute_frame(graph.build()).unwrap();
    session.wait_submits_done(10_000_000_000).unwrap();
    for partial in [false, true] {
        let mut graph = session.begin_frame().unwrap();
        graph.batch_per_pass(true);
        let device = session.backend.device.clone();
        session
            .cached_result(
                &mut graph,
                request(20, 1),
                spec(),
                READ,
                move |_session, graph, output| {
                    if partial {
                        let mut pass = graph.add_pass("submitted write");
                        pass.add_resource_access(output, WRITE, 0)?;
                        pass.render_with_context(move |ctx| {
                            let Some(managed) = ctx.managed else {
                                return false;
                            };
                            let Ok(buffer) = managed.buffer_for(output.raw()) else {
                                return false;
                            };

                            unsafe {
                                device.device.cmd_fill_buffer(
                                    vk::CommandBuffer::from_raw(ctx.command.handle()),
                                    buffer.raw(),
                                    0,
                                    64,
                                    99,
                                );
                            }
                            true
                        });
                    }
                    let mut fail = graph.add_pass("failed producer").with_side_effect();
                    fail.add_resource_access(output, if partial { READ } else { WRITE }, 0)?;
                    fail.render_with_result_context(|_| Err(rhapsody::Error::OutOfDeviceMemory));
                    Ok(())
                },
            )
            .unwrap();
        assert_eq!(
            session.execute_frame(graph.build()).unwrap_err().kind(),
            rhapsody::ErrorKind::OutOfMemory
        );
        assert_eq!(session.pending_frame_count(), usize::from(partial));
        assert_eq!(session.result_cache_stats().entries, 1);
        assert_eq!(session.result_cache_stats().publications, 1);
        session.wait_submits_done(10_000_000_000).unwrap();
        let mut graph = session.begin_frame().unwrap();
        let hit = filled(&mut session, &mut graph, request(20, 0), 17, calls.clone());
        assert!(hit.hit);
        consume(&session, &mut graph, &hit, &buffer, token);
        session.execute_frame(graph.build()).unwrap();
        session.wait_submits_done(10_000_000_000).unwrap();
        check(&buffer, 17);
    }
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    session.clear_result_cache();
    session.wait_submits_done(10_000_000_000).unwrap();
    session.release_import(token).unwrap();
}

#[test]
fn unknown_dependencies_do_not_reuse_or_publish_and_incomplete_outputs_are_rejected() {
    let Some(mut session) = session() else {
        return;
    };
    let (buffer, token) = readback(&mut session);
    let calls = Arc::new(AtomicUsize::new(0));
    for value in 0..3 {
        let mut graph = session.begin_frame().unwrap();
        let mut unknown = request(30, 0);
        unknown.dependencies = ResultDependencies::Unknown;
        let result = filled(&mut session, &mut graph, unknown, value, calls.clone());
        assert!(!result.hit);
        consume(&session, &mut graph, &result, &buffer, token);
        session.execute_frame(graph.build()).unwrap();
        session.wait_submits_done(10_000_000_000).unwrap();
        check(&buffer, value);
        assert_eq!(session.result_cache_stats().entries, 0);
        assert_eq!(session.result_cache_stats().publications, 0);
        assert_eq!(session.resource_count(), 1);
    }
    assert_eq!(calls.load(Ordering::SeqCst), 3);
    let mut graph = session.begin_frame().unwrap();
    let mut unknown = request(32, 0);
    unknown.dependencies = ResultDependencies::Unknown;
    let parent = filled(&mut session, &mut graph, unknown, 77, calls.clone());
    let mut child_request = request(33, 0);
    child_request.dependencies = ResultDependencies::Complete(vec![parent.dependency()]);
    let device = session.backend.device.clone();
    let child = session
        .cached_result(
            &mut graph,
            child_request,
            spec(),
            READ,
            |_session, graph, output| {
                copy(graph, device, parent.handle, output);
                Ok(())
            },
        )
        .unwrap();
    assert!(!child.hit);
    consume(&session, &mut graph, &child, &buffer, token);
    session.execute_frame(graph.build()).unwrap();
    session.wait_submits_done(10_000_000_000).unwrap();
    check(&buffer, 77);
    assert_eq!(
        session.result_cache_stats().entries,
        0,
        "unknown provenance must propagate through a complete child request"
    );
    let mut graph = session.begin_frame().unwrap();
    session
        .cached_result(
            &mut graph,
            request(31, 0),
            spec(),
            READ,
            |_session, graph, _| {
                graph
                    .add_pass("did not initialize")
                    .with_side_effect()
                    .render(|_| panic!("preflight must reject before recording"));
                Ok(())
            },
        )
        .unwrap();
    assert_eq!(
        session.execute_frame(graph.build()).unwrap_err().kind(),
        rhapsody::ErrorKind::InvalidUsage
    );
    assert_eq!(session.pending_frame_count(), 0);
    session.begin_frame().unwrap();
    assert_eq!(session.resource_count(), 1);
    session.release_import(token).unwrap();
}

#[test]
fn transitive_invalidation_survives_parent_eviction_and_content_changes_reuse_structure() {
    let Some(mut session) = session() else {
        return;
    };
    session.set_result_cache_limits(1, 1024 * 1024);
    let (buffer, token) = readback(&mut session);
    let calls = Arc::new(AtomicUsize::new(0));
    for version in 0..2 {
        let mut graph = session.begin_frame().unwrap();
        let parent = filled(
            &mut session,
            &mut graph,
            request(40, version),
            version as u32 + 71,
            calls.clone(),
        );
        let mut child_request = request(41, version);
        child_request.dependencies = ResultDependencies::Complete(vec![parent.dependency()]);
        let device = session.backend.device.clone();
        let child = session
            .cached_result(
                &mut graph,
                child_request,
                spec(),
                READ,
                |_session, graph, output| {
                    copy(graph, device, parent.handle, output);
                    Ok(())
                },
            )
            .unwrap();
        consume(&session, &mut graph, &child, &buffer, token);
        let receipt = session.execute_frame(graph.build()).unwrap();
        assert_eq!(
            receipt.stats.plan_cache_hit,
            version > 0,
            "input content versions must not recompile graph structure"
        );
        session.wait_submits_done(10_000_000_000).unwrap();
        check(&buffer, version as u32 + 71);
        assert_eq!(
            session.result_cache_stats().entries,
            1,
            "parent is evicted by the one-entry budget"
        );
        session.invalidate_result_dependency(DependencyId::External(1), version + 1);
        assert_eq!(
            session.result_cache_stats().entries,
            0,
            "the child must retain the evicted parent's provenance"
        );
    }
    assert_eq!(calls.load(Ordering::SeqCst), 2);
    session.wait_submits_done(10_000_000_000).unwrap();
    session.release_import(token).unwrap();
}

#[test]
fn immutable_hits_reject_writes_and_device_lost_stops_cached_work() {
    let Some(mut session) = session() else {
        return;
    };
    let calls = Arc::new(AtomicUsize::new(0));
    let mut graph = session.begin_frame().unwrap();
    let fresh = filled(&mut session, &mut graph, request(50, 0), 55, calls.clone());
    let receipt = session.execute_frame(graph.build()).unwrap();
    session.wait_submits_done(10_000_000_000).unwrap();
    let token = receipt.snapshot.physical_mapping[fresh.handle.raw() as usize]
        .token
        .unwrap();
    assert!(
        session.release_import(token).is_err(),
        "a live cache owns its stable import token"
    );
    let mut graph = session.begin_frame().unwrap();
    let hit = filled(&mut session, &mut graph, request(50, 0), 55, calls.clone());
    assert!(hit.hit);
    let mut illegal = graph.add_pass("overwrite cached output");
    illegal.add_resource_access(hit.handle, WRITE, 0).unwrap();
    illegal.render(|_| panic!("immutable result validation must happen before callbacks"));
    assert_eq!(
        session.execute_frame(graph.build()).unwrap_err().kind(),
        rhapsody::ErrorKind::InvalidUsage
    );
    assert_eq!(session.result_cache_stats().entries, 1);
    let mut graph = session.begin_frame().unwrap();
    session.backend.device.timelines.mark_device_lost();
    assert_eq!(
        session
            .cached_result(
                &mut graph,
                request(50, 0),
                spec(),
                READ,
                |_, _, _| unreachable!()
            )
            .unwrap_err()
            .kind(),
        rhapsody::ErrorKind::DeviceLost
    );
    assert_eq!(
        session.execute_frame(graph.build()).unwrap_err().kind(),
        rhapsody::ErrorKind::DeviceLost
    );
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}
