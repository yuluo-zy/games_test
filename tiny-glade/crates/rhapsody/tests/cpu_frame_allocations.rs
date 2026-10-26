//! Counts actual Rust allocator calls on the production CPU graph/execution path.
//! The backend performs no driver work; this is not a GPU or whole FrameLoop
//! performance result. Callback bodies and report serialization are allocation-free
//! or outside the counted interval. Run this binary alone with --nocapture.
use ash::vk;
use rhapsody::{
    raw::submit,
    rg::{
        executor::{ExecutionBackend, ExecutionChain},
        graph::{self, Descriptor, RenderGraph, ResourceBackend},
        parallel::ParallelRecording,
    },
};
mod allocation_counter;
mod cpu_graph_support;
#[cfg(unix)]
use allocation_counter::allocation_trace;
use allocation_counter::{Sample, measured};
#[derive(Default)]
struct CpuBackend {
    submitted: u64,
    next_command: u64,
    chunk_pools: Vec<u8>,
    cpu: Option<rhapsody::command::CpuArenaPool>,
    frames: Option<rhapsody::rg::frame_storage::FrameStoragePool>,
    transitions: std::sync::Mutex<Vec<(u64, u64, rhapsody::rg::sync_plan::BoundTransition)>>,
    trace: bool,
    ranges: cpu_graph_support::CpuRanges,
}
impl ResourceBackend for CpuBackend {
    fn frame_storage_pool(&self) -> Option<&rhapsody::rg::frame_storage::FrameStoragePool> {
        self.frames.as_ref()
    }

    fn create_without_initial_data(&mut self, _: Descriptor, _: &str) -> rhapsody::Result<u64> {
        unreachable!("fixed CPU control graph has no GPU resources")
    }
    fn set_debug_name(&mut self, _: u64, _: Descriptor, _: &str) {}
}
impl ExecutionBackend for CpuBackend {
    fn chunk_recording(
        &mut self,
        commands: &[u64],
    ) -> rhapsody::Result<rhapsody::rg::executor::ChunkRecording<'_>> {
        struct View<'a> {
            transitions:
                &'a std::sync::Mutex<Vec<(u64, u64, rhapsody::rg::sync_plan::BoundTransition)>>,
            trace: bool,
            cpu: Option<&'a rhapsody::command::CpuArenaPool>,
        }
        impl rhapsody::rg::executor::ChunkRecorder for View<'_> {
            fn cpu_arena_pool(&self) -> Option<&rhapsody::command::CpuArenaPool> {
                self.cpu
            }
            fn record_transitions(
                &self,
                lease: &mut rhapsody::rg::executor::ChunkLease<'_>,
                transitions: &[rhapsody::rg::executor::ChunkTransition],
                _: &mut rhapsody::rg::owner_storage::ResourceOwners,
            ) -> rhapsody::Result<()> {
                if self.trace {
                    self.transitions.lock().unwrap().extend(
                        transitions
                            .iter()
                            .map(|t| (lease.command(), t.token, t.transition)),
                    );
                }
                Ok(())
            }
        }
        self.chunk_pools.resize(commands.len(), 0);
        let leases = self
            .chunk_pools
            .iter_mut()
            .zip(commands)
            .map(|(pool, &command)| {
                // SAFETY: each command has a distinct exclusively borrowed test pool.
                unsafe { rhapsody::rg::executor::ChunkLease::new(pool, command) }
            })
            .collect();
        Ok((
            Box::new(View {
                transitions: &self.transitions,
                trace: self.trace,
                cpu: self.cpu.as_ref(),
            }),
            leases,
        ))
    }
    fn cpu_arena_pool(&self) -> Option<&rhapsody::command::CpuArenaPool> {
        self.cpu.as_ref()
    }
    fn command_handle(&mut self) -> rhapsody::Result<u64> {
        self.next_command += 1;
        Ok(self.next_command)
    }
    fn begin_command(&mut self, _: u64) -> rhapsody::Result<()> {
        Ok(())
    }
    fn end_command(&mut self, _: vk::CommandBuffer) -> rhapsody::Result<()> {
        Ok(())
    }
    fn transition(
        &mut self,
        command: u64,
        token: u64,
        transition: &rhapsody::rg::sync_plan::BoundTransition,
    ) -> rhapsody::Result<()> {
        if self.trace {
            self.transitions
                .lock()
                .unwrap()
                .push((command, token, *transition));
        }
        Ok(())
    }
    fn synchronization_plan(
        &self,
    ) -> std::sync::Arc<rhapsody::rg::sync_plan::SynchronizationPlanner> {
        self.ranges.planner.clone()
    }
    fn prepare_batch_submission(&mut self, _: &[u64], batch: usize) -> rhapsody::Result<()> {
        self.ranges.prepare(batch)
    }
    fn prepare_compiled_resources(&mut self, compiled: &graph::FrameInstance) -> rhapsody::Result<()> {
        self.ranges.bind(compiled)?;
        Ok(())
    }
    fn submit(
        &mut self,
        _: vk::Queue,
        _: &[submit::PreparedSubmit],
    ) -> rhapsody::Result<rhapsody::sync::SyncPoint> {
        self.submitted += 1;
        let point = rhapsody::sync::SyncPoint {
            queue: rhapsody::sync::QueueId::new(0).unwrap(),
            value: self.submitted,
        };
        self.ranges.publish(point);
        Ok(point)
    }
}
fn fixed_graph() -> graph::BuiltGraph {
    let mut graph = RenderGraph::new();
    for name in [
        "stage0", "stage1", "stage2", "stage3", "stage4", "stage5", "stage6", "stage7",
    ] {
        graph
            .add_pass(name)
            .with_side_effect()
            .render_with_result_context(|_| Ok(()));
    }
    graph.build()
}
fn fixed_resource_graph() -> graph::BuiltGraph {
    let mut graph = RenderGraph::new();
    let descriptor = Descriptor::Buffer(graph::BufferDesc {
        word_at_00: 64,
        word_at_08: 0,
        bytes_at_0c_through_0e: [0; 3],
    });
    let input = graph.import_unowned("input", descriptor, 11, 0x25).unwrap();
    let output = graph
        .import_unowned("output", descriptor, 12, 0x25)
        .unwrap();
    for name in [
        "stage0", "stage1", "stage2", "stage3", "stage4", "stage5", "stage6", "stage7",
    ] {
        let mut pass = graph.add_pass(name);
        pass.side_effect();
        pass.add_resource_access(input, 0x25, 0).unwrap();
        pass.add_resource_access(output, 0x34, 1).unwrap();
        pass.render_with_result_context(|_| Ok(()));
    }
    graph.export(output, 0x25).unwrap();
    graph.build()
}
#[test]
#[ignore = "explicit allocation measurement; run this test binary alone"]
fn fixed_graph_reports_real_build_and_cached_execution_allocations() {
    let mut reports = Vec::new();
    for (resources, enabled) in [(false, false), (false, true), (true, false), (true, true)] {
        let factory: fn() -> graph::BuiltGraph = if resources {
            fixed_resource_graph
        } else {
            fixed_graph
        };
        let mut chain = ExecutionChain::new(vk::Queue::null(), vec![], 1);
        chain.parallel = ParallelRecording::PARALLEL;
        let mut backend = CpuBackend {
            submitted: 0,
            cpu: enabled.then(|| {
                rhapsody::command::CpuArenaPool::new(
                    Default::default(),
                    std::sync::Arc::new(rhapsody::raw::allocator::AllocatorStats::default()),
                )
                .unwrap()
            }),
            frames: enabled.then(|| {
                rhapsody::rg::frame_storage::FrameStoragePool::new(
                    Default::default(),
                    std::sync::Arc::new(Default::default()),
                )
                .unwrap()
            }),
            ..Default::default()
        };
        let mut pool = graph::Pool::default();
        let mut dependency = rhapsody::rg::vulkan::AccessTable::original();
        // The CPU-only adapter has no completion poll. Keep the finite measurement
        // below the already warmed pending-vector capacity; no Vulkan liveness claim.
        for _ in 0..300 {
            let (declaration, mut callbacks, _, declaration_error) = factory().into_parts();
            assert!(declaration_error.is_none());
            let frame = chain
                .execute_graph_with_context(
                    declaration,
                    &mut pool,
                    &mut dependency,
                    &mut backend,
                    &mut callbacks,
                )
                .unwrap();
            drop(frame);
        }
        #[cfg(unix)]
        if resources && enabled && std::env::var_os("RHAPSODY_TRACE_CPU_ALLOCATIONS").is_some() {
            let (declaration, mut callbacks, _, declaration_error) = factory().into_parts();
            assert!(declaration_error.is_none());
            allocation_trace::ENABLED.with(|enabled| enabled.set(true));
            let (frame, sample) = measured(|| {
                chain
                    .execute_graph_with_context(
                        declaration,
                        &mut pool,
                        &mut dependency,
                        &mut backend,
                        &mut callbacks,
                    )
                    .unwrap()
            });
            allocation_trace::ENABLED.with(|enabled| enabled.set(false));
            drop(frame);
            drop(callbacks);
            allocation_trace::report(&sample);
        }
        let mut builds = Vec::with_capacity(128);
        let mut executions = Vec::with_capacity(128);
        for _ in 0..128 {
            let (built, build) = measured(factory);
            let ((frame, callbacks), execute) = measured(|| {
                let (declaration, mut callbacks, _, declaration_error) = built.into_parts();
                assert!(declaration_error.is_none());
                let frame = chain
                    .execute_graph_with_context(
                        declaration,
                        &mut pool,
                        &mut dependency,
                        &mut backend,
                        &mut callbacks,
                    )
                    .unwrap();
                (frame, callbacks)
            });
            assert!(frame.stats.plan_cache_hit);
            drop(frame);
            drop(callbacks);
            builds.push(build);
            executions.push(execute);
        }
        if enabled {
            assert!(
                executions.iter().all(|sample| sample.allocations == 0),
                "warmed fixed graph execution allocated: {:?}",
                executions
                    .iter()
                    .map(|sample| sample.allocation_sizes)
                    .find(|sizes| sizes.iter().any(|size| *size != 0))
            );
        }
        let summarize = |samples: &mut [Sample]| {
            samples.sort_by_key(|sample| sample.elapsed_ns);
            let mean_allocations = samples.iter().map(|sample| sample.allocations).sum::<u64>()
                as f64
                / samples.len() as f64;
            serde_json::json!({ "mean_allocations": mean_allocations, "max_allocations": samples.iter().map(|sample| sample.allocations).max(), "first_sample_request_sizes":samples[0].allocation_sizes.into_iter().filter(|size|*size!=0).collect::<Vec<_>>(), "mean_requested_bytes": samples.iter().map(|sample| sample.allocated_bytes).sum::<u64>() / samples.len() as u64,
            "p95_ns": samples[(samples.len()-1)*95/100].elapsed_ns, "p99_ns": samples[(samples.len()-1)*99/100].elapsed_ns })
        };
        let report = serde_json::json!({ "scope":"production ExecutionChain with parallel CPU no-driver backend; eight fixed side-effect passes; optional two imported CPU-projected buffers; no GPU or FrameLoop claim", "captured_request_size_limit":32,"warmup_frames":300, "sampled_frames":128, "build":summarize(&mut builds), "cached_execute":summarize(&mut executions) });
        reports.push(serde_json::json!({"resource_graph":resources,"cpu_arena_enabled":enabled,"measurement":report,"arena_backing_allocations":backend.cpu.as_ref().map(|pool|pool.stats().backing_allocations),"frame_storage_capacity_growths":backend.frames.as_ref().map(|pool|pool.stats().capacity_growths)}));
    }
    let report = serde_json::json!({"variants":reports});
    println!("{report}");
    if let Some(path) = std::env::var_os("RHAPSODY_CPU_ALLOCATION_REPORT") {
        std::fs::write(path, serde_json::to_vec_pretty(&report).unwrap()).unwrap();
    }
}

#[test]
fn warmed_cpu_frame_scratch_reuses_actual_storage_without_heap_allocation() {
    let pool = rhapsody::command::CpuArenaPool::new(
        Default::default(),
        std::sync::Arc::new(rhapsody::raw::allocator::AllocatorStats::default()),
    )
    .unwrap();
    for _ in 0..50 {
        let arena = pool.checkout().unwrap();
        let _ = arena.slice_fill(100, 5u64).unwrap();
    }
    let (_, sample) = measured(|| {
        for _ in 0..1000 {
            let arena = pool.checkout().unwrap();
            let values = arena.slice_with(100, |index| index as u64).unwrap();
            assert_eq!(values[99], 99);
        }
    });
    assert_eq!(
        sample.allocations, 0,
        "this assertion counts real System allocator calls, not arena request counters"
    );
    assert_eq!(pool.stats().backing_allocations, 1);
    assert_eq!(pool.stats().checked_out, 0);
}

#[test]
fn production_recorder_arena_matches_vec_events_and_returns_cpu_memory_after_callback_failure() {
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };
    let mut traces = Vec::new();
    for enabled in [false, true] {
        let mut chain = ExecutionChain::new(vk::Queue::null(), vec![], 1);
        chain.parallel = ParallelRecording::PARALLEL;
        let mut backend = CpuBackend {
            trace: true,
            cpu: enabled.then(|| {
                rhapsody::command::CpuArenaPool::new(
                    Default::default(),
                    Arc::new(Default::default()),
                )
                .unwrap()
            }),
            frames: enabled.then(|| {
                rhapsody::rg::frame_storage::FrameStoragePool::new(
                    Default::default(),
                    Arc::new(Default::default()),
                )
                .unwrap()
            }),
            ..Default::default()
        };
        let calls = Arc::new(AtomicUsize::new(0));
        let mut pool = graph::Pool::default();
        for frame in 0..18 {
            let mut graph = RenderGraph::new();
            let descriptor = Descriptor::Buffer(graph::BufferDesc {
                word_at_00: 64,
                word_at_08: 0,
                bytes_at_0c_through_0e: [0; 3],
            });
            let input = graph.import_unowned("input", descriptor, 11, 0x25).unwrap();
            let output = graph
                .import_unowned("output", descriptor, 12, 0x25)
                .unwrap();
            let mut pass = graph.add_pass("write");
            pass.add_resource_access(input, 0x25, 0).unwrap();
            pass.add_resource_access(output, 0x34, 1).unwrap();
            let writes = calls.clone();
            pass.render_with_result_context(move |context| {
                writes.fetch_add(1, Ordering::Relaxed);
                assert_eq!(context.cpu.is_some(), enabled);
                if frame == 16 {
                    return Err(rhapsody::Error::OutOfDeviceMemory);
                }
                Ok(())
            });
            let mut pass = graph.add_pass("consume");
            pass.side_effect();
            pass.add_resource_access(output, 0x25, 2).unwrap();
            let reads = calls.clone();
            pass.render_with_result_context(move |_| {
                reads.fetch_add(1, Ordering::Relaxed);
                Ok(())
            });
            graph.export(output, 0x25).unwrap();
            let (declaration, mut callbacks, _, declaration_error) = graph.build().into_parts();
            assert!(declaration_error.is_none());
            let result = chain.execute_graph_with_context(
                declaration,
                &mut pool,
                &mut rhapsody::rg::vulkan::AccessTable::original(),
                &mut backend,
                &mut callbacks,
            );
            if frame == 16 {
                assert!(matches!(
                    result,
                    Err(rhapsody::rg::executor::Error::Validation(
                        rhapsody::Error::OutOfDeviceMemory
                    ))
                ));
            } else {
                assert!(result.is_ok());
            }
            drop(result);
            if let Some(cpu) = &backend.cpu {
                assert_eq!(cpu.stats().checked_out, 0);
            }
            if let Some(frames) = &backend.frames {
                assert_eq!(frames.stats().checked_out, 0);
            }
        }
        // Both independent callbacks are dispatched before the worker error is joined.
        assert_eq!(calls.load(Ordering::Relaxed), 36);
        assert_eq!(backend.submitted, 17);
        assert!(
            backend
                .transitions
                .lock()
                .unwrap()
                .iter()
                .any(|(_, token, transition)| {
                    *token == 12
                        && transition
                            .source
                            .access
                            .contains(vk::AccessFlags::TRANSFER_WRITE)
                        && transition
                            .destination
                            .access
                            .contains(vk::AccessFlags::TRANSFER_READ)
                })
        );
        if let Some(cpu) = &backend.cpu {
            assert!(
                cpu.stats().backing_allocations <= 3,
                "one owner arena and two bounded worker arenas"
            );
        }
        let mut trace = backend.transitions.into_inner().unwrap();
        // Worker arrival order is arbitrary; compare the ordered events within
        // each command, which is the unit submitted to the queue.
        trace.sort_by_key(|(command, _, _)| *command);
        traces.push(trace);
    }
    assert_eq!(traces[0], traces[1]);
}
