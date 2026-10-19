//! S05 exercises real alias bindings through the production session recorder.
mod gpu_test_support;
use ash::vk::{self, Handle};
use rhapsody::{
    device::{AllocationStrategy, Device, DeviceConfig},
    raw::{
        buffer::{Buffer, BufferDesc},
        texture::TextureDesc,
    },
    rg::{
        executor::PassContext,
        graph::RenderGraph,
        vulkan::{AccessTable, GpuResource, ResourceSpec, VulkanGraphBackend, VulkanGraphSession},
    },
};
use std::sync::Arc;
const READ: u8 = 0x25;
const WRITE: u8 = 0x34;
const HOST: u8 = 0x26;
const SIZE: u32 = 64;
const BYTES: u64 = (SIZE * SIZE * 4) as u64;
const TIMEOUT: u64 = 10_000_000_000;
fn device() -> Option<Arc<Device>> {
    device_with_strategy(AllocationStrategy::Dedicated)
}
fn device_with_strategy(allocation: AllocationStrategy) -> Option<Arc<Device>> {
    gpu_test_support::initialize(
        "transient aliasing",
        Device::new_headless(&DeviceConfig {
            allocation,
            ..DeviceConfig::headless()
        }),
    )
}
fn graph(
    session: &mut VulkanGraphSession,
    readback: Arc<Buffer>,
    serial: bool,
    fail: Option<u64>,
) -> RenderGraph {
    let mut graph = session.begin_frame().unwrap();
    graph.batch_per_pass(true);
    let image = TextureDesc::new_2d(SIZE, SIZE, vk::Format::R8G8B8A8_UNORM);
    let a = session
        .backend
        .declare(&mut graph, "a", ResourceSpec::Image(image))
        .unwrap();
    let b = session
        .backend
        .declare(&mut graph, "b", ResourceSpec::Image(image))
        .unwrap();
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
    for (image_id, color, offset) in [(a, [1., 0., 0., 1.], 0), (b, [0., 1., 0., 1.], BYTES)] {
        let mut pass = graph.add_pass(format!("clear {offset}")).with_side_effect();
        pass.add_resource_access(image_id, WRITE, 0).unwrap();
        if offset != 0 && serial {
            pass.add_resource_access(bridge, READ, 0).unwrap();
        }
        let device = device.clone();
        let index = image_id.raw();
        pass.render_with_result_context(move |ctx: PassContext<'_>| {
            if fail == Some(offset) {
                return Err(rhapsody::Error::invalid_usage(
                    "injected alias first-use recording failure",
                ));
            }
            let managed = ctx
                .managed
                .ok_or_else(|| rhapsody::Error::invalid_usage("missing managed resources"))?;
            let image = managed
                .texture_for(index)
                .map_err(|e| rhapsody::Error::invalid_usage_with("invalid pass binding", e))?;

            unsafe {
                device.device.cmd_clear_color_image(
                    vk::CommandBuffer::from_raw(ctx.command.handle()),
                    image.raw(),
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    &vk::ClearColorValue { float32: color },
                    &[vk::ImageSubresourceRange::default()
                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                        .level_count(1)
                        .layer_count(1)],
                );
            }
            Ok(())
        });
        let mut pass = graph.add_pass(format!("copy {offset}")).with_side_effect();
        pass.add_resource_access(image_id, READ, 0).unwrap();
        pass.add_resource_access(out, WRITE, 0).unwrap();
        if offset == 0 {
            pass.add_resource_access(bridge, WRITE, 0).unwrap();
        }
        let device = session.backend.device.clone();
        let out_id = out.raw();
        let bridge_id = bridge.raw();
        pass.render_with_result_context(move |ctx: PassContext<'_>| {
            let managed = ctx
                .managed
                .ok_or_else(|| rhapsody::Error::invalid_usage("missing managed resources"))?;
            let image = managed
                .texture_for(index)
                .map_err(|e| rhapsody::Error::invalid_usage_with("invalid pass binding", e))?;
            let output = managed
                .buffer_for(out_id)
                .map_err(|e| rhapsody::Error::invalid_usage_with("invalid pass binding", e))?;

            let command = vk::CommandBuffer::from_raw(ctx.command.handle());
            unsafe {
                device.device.cmd_copy_image_to_buffer(
                    command,
                    image.raw(),
                    vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                    output.raw(),
                    &[vk::BufferImageCopy {
                        buffer_offset: offset,
                        image_subresource: vk::ImageSubresourceLayers {
                            aspect_mask: vk::ImageAspectFlags::COLOR,
                            mip_level: 0,
                            base_array_layer: 0,
                            layer_count: 1,
                        },
                        image_extent: vk::Extent3D {
                            width: SIZE,
                            height: SIZE,
                            depth: 1,
                        },
                        ..Default::default()
                    }],
                );
            }
            if offset == 0 {
                let buffer = managed
                    .buffer_for(bridge_id)
                    .map_err(|e| rhapsody::Error::invalid_usage_with("invalid pass binding", e))?;

                unsafe {
                    device
                        .device
                        .cmd_fill_buffer(command, buffer.raw(), 0, 16, 1);
                }
            }
            Ok(())
        });
    }
    graph.export(out, HOST).unwrap();
    graph
}
struct Run {
    pixels: Vec<u8>,
    committed: u64,
    saved: u64,
    allocations: u64,
    frees: u64,
    edges: Vec<(usize, usize)>,
    undefined_transitions: usize,
    native_blocks: usize,
}
fn run(enabled: bool) -> Option<Run> {
    run_with_strategy(enabled, AllocationStrategy::Dedicated)
}
fn run_with_strategy(enabled: bool, allocation: AllocationStrategy) -> Option<Run> {
    let device = device_with_strategy(allocation)?;
    let readback = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_to_cpu(BYTES * 2, vk::BufferUsageFlags::TRANSFER_DST),
        )
        .unwrap(),
    );
    let mut backend = VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap();
    backend.set_transient_aliasing(enabled);
    let counters = backend.transient_backing_counters();
    let mut session = VulkanGraphSession::new(backend);
    let graph = graph(&mut session, readback.clone(), true, None);
    let _receipt = session.execute_frame(graph.build()).unwrap();
    let physical = session.backend.transient_memory_plan();
    eprintln!("alias={enabled} {}", physical.dump());
    let saved = physical.stats.reuse_bytes;
    let committed = device.allocator_stats.report().totals.total_allocated_bytes;
    let native = device.memory_usage_snapshot();
    assert_eq!(
        native.suballocator_unused_bytes,
        device.allocator_stats.suballocator_spare_bytes()
    );
    let budget = device.allocator_stats.budget_usage().total();
    assert_eq!(
        budget,
        native.application_vk_memory_bytes + device.allocator_stats.cpu_memory_usage().total()
    );
    let undefined_transitions = session
        .last_synchronization()
        .transitions()
        .iter()
        .filter(|transition| {
            transition.resource < 2
                && transition.old_layout == vk::ImageLayout::UNDEFINED
                && matches!(
                    transition.range,
                    rhapsody::resource::OwnershipRange::Image { .. }
                )
        })
        .count();
    assert_eq!(
        undefined_transitions, 2,
        "both real VkImages require their own initial layout transition"
    );
    let edges = physical.dependency_edges.clone();
    if enabled {
        assert_eq!(
            physical
                .backings
                .iter()
                .filter(|b| b.resources.len() > 1)
                .count(),
            1
        );
        assert!(saved >= BYTES);
        for resource in [0, 1] {
            assert!(
                session
                    .last_synchronization()
                    .transitions()
                    .iter()
                    .any(|transition| transition.resource == resource
                        && transition.old_layout == vk::ImageLayout::UNDEFINED
                        && transition.new_layout == vk::ImageLayout::TRANSFER_DST_OPTIMAL
                        && matches!(
                            transition.range,
                            rhapsody::resource::OwnershipRange::Image {
                                aspects: 1,
                                base_mip: 0,
                                mip_count: 1,
                                base_layer: 0,
                                layer_count: 1
                            }
                        )),
                "aliasing memory never replaces the exact VkImage layout dependency"
            );
        }
    }
    session.wait_submits_done(TIMEOUT).unwrap();
    let mut pixels = vec![0; BYTES as usize * 2];
    unsafe {
        readback
            .read_bytes_after_completion(0, &mut pixels)
            .unwrap();
    }
    for (index, pixel) in pixels.as_chunks::<4>().0.iter().enumerate() {
        assert_eq!(
            pixel,
            if index < SIZE as usize * SIZE as usize {
                &[255, 0, 0, 255]
            } else {
                &[0, 255, 0, 255]
            }
        );
    }
    drop(session);
    drop(readback);
    device.try_collect_all_completed().unwrap();
    let actual = counters.snapshot();
    assert_eq!(
        actual.allocations, actual.frees,
        "backing freed exactly once after all raw members and command owners retire"
    );
    assert_eq!(actual.allocated_bytes, actual.freed_bytes);
    assert_eq!(
        device.allocator_stats.report().totals.total_allocated_bytes,
        0,
        "shared allocation accounting is not leaked or released twice"
    );
    device.trim_unused_suballocations();
    assert_eq!(
        device.memory_usage_snapshot().application_vk_memory_bytes,
        0
    );
    Some(Run {
        pixels,
        committed,
        saved,
        allocations: actual.allocations,
        frees: actual.frees,
        edges,
        undefined_transitions,
        native_blocks: native.native_memory_blocks,
    })
}
#[cfg(feature = "suballocation")]
#[test]
fn s05_default_suballocator_reuses_native_blocks_for_shared_alias_backings() {
    let Some(reference) = run_with_strategy(false, AllocationStrategy::Suballocated) else {
        return;
    };
    let colored = run_with_strategy(true, AllocationStrategy::Suballocated)
        .expect("same available Vulkan device");
    assert_eq!(colored.pixels, reference.pixels);
    assert_eq!(colored.edges, reference.edges);
    assert_eq!(colored.allocations, 1);
    assert_eq!(colored.allocations, colored.frees);
    assert_eq!(reference.committed - colored.committed, colored.saved);
    assert!(
        colored.native_blocks <= reference.native_blocks,
        "shared backing must not add a dedicated allocation to the default suballocator"
    );
}
#[test]
fn s05_real_alias_binding_matches_pixels_and_reduces_committed_bytes() {
    let Some(reference) = run(false) else { return };
    let colored = run(true).expect("same available Vulkan device");
    assert_eq!(colored.pixels, reference.pixels);
    assert_eq!(colored.edges, reference.edges);
    assert_eq!(colored.allocations, 1);
    assert_eq!(colored.frees, 1);
    assert_eq!(reference.allocations, 0);
    assert_eq!(reference.committed - colored.committed, colored.saved);
    assert_eq!(
        reference.undefined_transitions,
        colored.undefined_transitions
    );
    assert!(colored.native_blocks < reference.native_blocks);
    eprintln!(
        "S05 committed baseline={} colored={} saved={} actual_backings={} freed={}",
        reference.committed, colored.committed, colored.saved, colored.allocations, colored.frees
    );
}

#[test]
fn unrelated_passes_do_not_alias_even_when_recording_indices_are_disjoint() {
    let Some(device) = device() else { return };
    let readback = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_to_cpu(BYTES * 2, vk::BufferUsageFlags::TRANSFER_DST),
        )
        .unwrap(),
    );
    let mut backend = VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap();
    backend.set_transient_aliasing(true);
    let mut session = VulkanGraphSession::new(backend);
    let graph = graph(&mut session, readback, false, None);
    session.execute_frame(graph.build()).unwrap();
    assert!(
        session
            .backend
            .transient_memory_plan()
            .backings
            .iter()
            .all(|b| b.resources.len() == 1)
    );
    assert_eq!(
        session.backend.transient_backing_statistics().allocations,
        0
    );
    session.wait_submits_done(TIMEOUT).unwrap();
}

struct Release(Arc<rhapsody::raw::semaphore::TimelineSemaphore>, bool);
impl Release {
    fn signal(&mut self) {
        if !self.1 {
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
            self.1 = true;
        }
    }
}
impl Drop for Release {
    fn drop(&mut self) {
        if !self.1 {
            let _ = unsafe {
                self.0.device.device.signal_semaphore(
                    &vk::SemaphoreSignalInfo::default()
                        .semaphore(self.0.handle)
                        .value(1),
                )
            };
        }
    }
}
#[test]
fn simultaneously_pending_frames_allocate_distinct_backings_and_release_once() {
    let Some(device) = device() else { return };
    let mut backend = VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap();
    backend.set_transient_aliasing(true);
    let counters = backend.transient_backing_counters();
    let mut session = VulkanGraphSession::new(backend);
    // This guard drops before Session on assertion failure, so pending submits
    // can always finish during Session cleanup.
    let mut release = Release(
        rhapsody::raw::semaphore::TimelineSemaphore::new(device.clone(), 0).unwrap(),
        false,
    );
    let mut allocations = Vec::new();
    for _ in 0..2 {
        let readback = Arc::new(
            Buffer::new(
                device.clone(),
                &BufferDesc::new_gpu_to_cpu(BYTES * 2, vk::BufferUsageFlags::TRANSFER_DST),
            )
            .unwrap(),
        );
        let graph = graph(&mut session, readback, true, None);
        session
            .backend
            .set_graph_synchronization(
                vec![(release.0.clone(), 1, vk::PipelineStageFlags::ALL_COMMANDS)],
                vec![],
            )
            .unwrap();
        session.execute_frame(graph.build()).unwrap();
        let ids = session
            .backend
            .transient_memory_plan()
            .backings
            .iter()
            .filter_map(|b| b.allocation_id)
            .collect::<Vec<_>>();
        assert_eq!(ids.len(), 1);
        allocations.push(ids[0]);
    }
    assert_eq!(session.pending_frame_count(), 2);
    assert_ne!(allocations[0], allocations[1]);
    assert_eq!(counters.snapshot().allocations, 2);
    assert_eq!(counters.snapshot().frees, 0);
    release.signal();
    session.wait_submits_done(TIMEOUT).unwrap();
    drop(session);
    device.try_collect_all_completed().unwrap();
    assert_eq!(counters.snapshot().allocations, counters.snapshot().frees);
    assert_eq!(
        device.allocator_stats.report().totals.total_allocated_bytes,
        0
    );
}

#[test]
fn partial_submit_does_not_publish_an_unreached_alias_initial_layout() {
    use rhapsody::rg::executor::ExecutionBackend;
    let Some(device) = device() else { return };
    let readback = Arc::new(
        Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_to_cpu(BYTES * 2, vk::BufferUsageFlags::TRANSFER_DST),
        )
        .unwrap(),
    );
    let mut backend = VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap();
    backend.set_transient_aliasing(true);
    let mut session = VulkanGraphSession::new(backend);
    let graph = graph(&mut session, readback, true, Some(BYTES));
    let built = graph.build();
    let logical_a = built
        .declaration
        .resources
        .iter()
        .position(|resource| resource.name == "a")
        .unwrap() as u32;
    let logical_b = built
        .declaration
        .resources
        .iter()
        .position(|resource| resource.name == "b")
        .unwrap() as u32;
    assert_eq!(
        session.execute_frame(built).unwrap_err().kind(),
        rhapsody::ErrorKind::InvalidUsage
    );
    assert!(
        session.pending_frame_count() > 0,
        "earlier batches really submitted"
    );
    let managed = session.backend.managed_pass_resources().unwrap();
    let a = managed.texture_for(logical_a).unwrap().raw();
    let b = managed.texture_for(logical_b).unwrap().raw();
    let token_for = |image| {
        *session.backend.resources.iter().find(|(_,resource)|matches!(resource,GpuResource::Image(actual) if actual.handle==image)).unwrap().0
    };
    let (a, b) = (token_for(a), token_for(b));
    session
        .backend
        .resource_native_state(a, |cells| {
            assert!(
                cells
                    .iter()
                    .all(|cell| cell.state.layout == vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
            );
        })
        .unwrap();
    session
        .backend
        .resource_native_state(b, |cells| {
            assert!(
                cells
                    .iter()
                    .all(|cell| cell.state.layout == vk::ImageLayout::UNDEFINED),
                "unsubmitted first-use barrier cannot publish B's scheduled layout"
            );
            assert!(
                cells.iter().all(|cell| cell.state.writer.points.is_empty()
                    && cell.state.readers.points.is_empty())
            );
        })
        .unwrap();
    session.wait_submits_done(TIMEOUT).unwrap();
}

fn assert_clean_alias_frame(session: &VulkanGraphSession, readback: &Buffer) {
    use rhapsody::rg::sync_plan::BarrierKind;
    let synchronization = session.last_synchronization();
    assert!(
        synchronization
            .transitions()
            .iter()
            .all(|transition| matches!(transition.kind, BarrierKind::Memory)
                && transition.source_family == vk::QUEUE_FAMILY_IGNORED
                && transition.destination_family == vk::QUEUE_FAMILY_IGNORED),
        "no ownership transfer barrier after a failed frame"
    );
    for resource in [0, 1] {
        let first = synchronization
            .transitions()
            .iter()
            .find(|transition| transition.resource == resource)
            .expect("alias member first use");
        assert_eq!(
            first.old_layout,
            vk::ImageLayout::UNDEFINED,
            "alias member contents are discarded at first use"
        );
    }
    let mut pixels = vec![0; BYTES as usize * 2];
    unsafe {
        readback
            .read_bytes_after_completion(0, &mut pixels)
            .unwrap();
    }
    for (index, pixel) in pixels.as_chunks::<4>().0.iter().enumerate() {
        assert_eq!(
            pixel,
            if index < SIZE as usize * SIZE as usize {
                &[255, 0, 0, 255]
            } else {
                &[0, 255, 0, 255]
            }
        );
    }
}

#[test]
fn failed_frames_return_alias_sets_only_when_idle_and_later_frames_discard_stale_state() {
    let Some(device) = device() else { return };
    let mut backend = VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap();
    backend.set_transient_aliasing(true);
    let counters = backend.transient_backing_counters();
    let mut session = VulkanGraphSession::new(backend);
    let readback = || {
        Arc::new(
            Buffer::new(
                device.clone(),
                &BufferDesc::new_gpu_to_cpu(BYTES * 2, vk::BufferUsageFlags::TRANSFER_DST),
            )
            .unwrap(),
        )
    };
    // 1. Fails before any submission: the borrow returns immediately and the
    //    next frame reuses the same set without another backing allocation.
    let graph_a = graph(&mut session, readback(), true, Some(0));
    assert!(session.execute_frame(graph_a.build()).is_err());
    assert_eq!(session.pending_frame_count(), 0, "nothing was submitted");
    assert_eq!(session.backend.transient_alias_retiring(), 0);
    assert_eq!(counters.snapshot().allocations, 1);
    // 2. Fails mid-frame after earlier batches really submitted.
    let graph_b = graph(&mut session, readback(), true, Some(BYTES));
    assert!(session.execute_frame(graph_b.build()).is_err());
    assert!(session.pending_frame_count() > 0, "a prefix was submitted");
    assert_eq!(
        counters.snapshot().allocations,
        1,
        "a never-submitted borrow left the set reusable"
    );
    // 3. Following frames reuse the set only after the failed prefix retired,
    //    then render correct data without any stale ownership transfer.
    for _ in 0..3 {
        let target = readback();
        let frame = graph(&mut session, target.clone(), true, None);
        session.execute_frame(frame.build()).unwrap();
        session.wait_submits_done(TIMEOUT).unwrap();
        assert_clean_alias_frame(&session, &target);
    }
    assert_eq!(session.backend.transient_alias_retiring(), 0);
    assert!(
        counters.snapshot().allocations <= 2,
        "retired sets are reused instead of reallocated every frame"
    );
    drop(session);
    device.try_collect_all_completed().unwrap();
    assert_eq!(counters.snapshot().allocations, counters.snapshot().frees);
}
