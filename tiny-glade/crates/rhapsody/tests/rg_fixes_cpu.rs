//! rg 缺陷修复的 CPU 回归：导出但未访问的 Create 资源、失败路径的资源归还、temporal 记录的放弃/确认、
//! pass 数上限的构图期错误。不需要 Vulkan 设备。
use ash::vk;
use rhapsody::rg::{
    executor::{Error, ExecutionBackend, ExecutionChain},
    graph::{
        self, AccessDependency, BackendFailure, BufferDesc, BuildError, Descriptor, EventBackend,
        OriginSource, Pool, RenderGraph, ResourceBackend,
    },
    resource::ResourceOrigin,
};
use rhapsody::{command::recording, raw::submit};
mod cpu_graph_support;

const WRITE: u8 = 0x34;
/// 测试用访问掩码：缓冲 → STORAGE_BUFFER(0x20)。
struct Dependency;
impl AccessDependency for Dependency {
    fn access_mask(&mut self, _code: u8) -> ash::vk::AccessFlags {
        ash::vk::AccessFlags::SHADER_READ
    }
    fn is_write(&mut self, code: u8) -> bool {
        code == WRITE
    }
}

fn buffer() -> Descriptor {
    Descriptor::Buffer(BufferDesc {
        word_at_00: 64,
        word_at_08: 0,
        bytes_at_0c_through_0e: [0; 3],
    })
}
fn pooled_buffer(handle: u64) -> graph::Buffer<u64> {
    let Descriptor::Buffer(desc) = buffer().with_usage(0x20) else {
        unreachable!()
    };
    graph::Buffer { handle, desc }
}
fn create(graph: &mut RenderGraph, name: &str) -> rhapsody::rg::resource::ResourceHandle {
    graph
        .create_raw_resource(
            name,
            ResourceOrigin {
                descriptor: buffer(),
                source: OriginSource::Create,
            },
        )
        .unwrap()
}

/// CPU 后端：记录创建、释放与放弃的命令；可让第 N 次创建、prepare 或回调失败。
#[derive(Default)]
struct Mock {
    next_token: u64,
    creations: usize,
    fail_creation_at: Option<usize>,
    creation_error: Option<rhapsody::Error>,
    fail_prepare: bool,
    fail_begin: bool,
    fail_end_at: Option<usize>,
    end_attempts: usize,
    released: Vec<u64>,
    abandoned: Vec<u64>,
    next_command: u64,
    next_submit: u64,
    submit_attempts: usize,
    fail_submit_at: Option<usize>,
    waits: std::cell::RefCell<Vec<rhapsody::sync::SyncPoint>>,
    queue: Option<recording::Queue>,
    fail_wait: bool,
    ranges: cpu_graph_support::CpuRanges,
    /// One stand-in command pool per parallel worker.
    chunk_pools: Vec<u8>,
    /// Commands recorded through each lease, in chunk order.
    leased: std::sync::Mutex<Vec<u64>>,
}
impl ResourceBackend for Mock {
    fn create_without_initial_data(
        &mut self,
        _descriptor: Descriptor,
        name: &str,
    ) -> Result<u64, BackendFailure> {
        let attempt = self.creations;
        self.creations += 1;
        if self.fail_creation_at == Some(attempt) {
            return Err(self
                .creation_error
                .take()
                .unwrap_or_else(|| BackendFailure::invalid_usage(format!("fail {name}"))));
        }
        self.next_token += 1;
        Ok(500 + self.next_token)
    }
    fn set_debug_name(&mut self, _token: u64, _descriptor: Descriptor, _name: &str) {}
    fn release_created_resource(&mut self, token: u64) {
        self.released.push(token);
    }
}
impl ExecutionBackend for Mock {
    fn chunk_recording(
        &mut self,
        commands: &[u64],
    ) -> rhapsody::Result<rhapsody::rg::executor::ChunkRecording<'_>> {
        struct View<'a>(&'a std::sync::Mutex<Vec<u64>>);
        impl rhapsody::rg::executor::ChunkRecorder for View<'_> {
            fn record_transitions(
                &self,
                lease: &mut rhapsody::rg::executor::ChunkLease<'_>,
                _: &[rhapsody::rg::executor::ChunkTransition],
                _: &mut rhapsody::rg::owner_storage::ResourceOwners,
            ) -> rhapsody::Result<()> {
                self.0.lock().unwrap().push(lease.command());
                Ok(())
            }
        }
        if self.chunk_pools.len() < commands.len() {
            self.chunk_pools.resize(commands.len(), 0);
        }
        // Disjoint `&mut` pool borrows: one lease per pool.
        let leases = self
            .chunk_pools
            .iter_mut()
            .zip(commands)
            .map(|(pool, &command)| unsafe {
                rhapsody::rg::executor::ChunkLease::new(pool, command)
            })
            .collect();
        Ok((Box::new(View(&self.leased)), leases))
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
        if self.fail_prepare {
            return Err(rhapsody::Error::OutOfHostMemory);
        }
        self.ranges.bind(compiled)?;
        Ok(())
    }
    fn graph_queue(&self) -> (recording::Queue, u32) {
        (self.queue.unwrap_or(recording::Queue::Primary), 0)
    }
    fn wait_submission(&self, point: rhapsody::sync::SyncPoint) -> rhapsody::Result<()> {
        self.waits.borrow_mut().push(point);
        if self.fail_wait {
            Err(rhapsody::Error::Timeout)
        } else {
            Ok(())
        }
    }
    fn command_handle(&mut self) -> rhapsody::Result<u64> {
        self.next_command += 1;
        Ok(1000 + self.next_command)
    }
    fn begin_command(&mut self, _handle: u64) -> rhapsody::Result<()> {
        if self.fail_begin { Err(rhapsody::Error::OutOfHostMemory) } else { Ok(()) }
    }
    fn end_command(&mut self, _handle: vk::CommandBuffer) -> rhapsody::Result<()> {
        self.end_attempts += 1;
        if self.fail_end_at == Some(self.end_attempts) { Err(rhapsody::Error::DeviceLost) } else { Ok(()) }
    }
    fn submit(
        &mut self,
        _queue: vk::Queue,
        _prepared: &[submit::PreparedSubmit],
    ) -> rhapsody::Result<rhapsody::sync::SyncPoint> {
        self.submit_attempts += 1;
        if self.fail_submit_at == Some(self.submit_attempts) {
            return Err(rhapsody::Error::OutOfDeviceMemory);
        }
        self.next_submit += 1;
        let point = rhapsody::sync::SyncPoint {
            queue: rhapsody::sync::QueueId::new(
                match self.queue.unwrap_or(recording::Queue::Primary) {
                    recording::Queue::Primary => 0,
                    recording::Queue::Auxiliary(index) => index as usize + 1,
                },
            )
            .unwrap(),
            value: self.next_submit,
        };
        self.ranges.publish(point);
        Ok(point)
    }
    fn abandon_command(&mut self, handle: u64) {
        self.abandoned.push(handle);
    }
}

fn chain() -> ExecutionChain {
    ExecutionChain::new(vk::Queue::null(), vec![], 1)
}

#[test]
fn result_callbacks_preserve_device_errors_across_parallel_chunk_strategies() {
    use rhapsody::rg::parallel::ParallelRecording;
    for parallel in [
        ParallelRecording::PARALLEL,
        ParallelRecording {
            ..ParallelRecording::EVERY_PASS
        },
    ] {
        for device_lost in [false, true] {
            let mut backend = Mock::default();
            let mut chain = chain();
            chain.parallel = parallel;
            let mut graph = RenderGraph::new();
            graph
                .add_pass("successful sibling")
                .with_side_effect()
                .render_with_result_context(|_| Ok(()));
            graph
                .add_pass("device error")
                .with_side_effect()
                .render_with_result_context(move |_| {
                    Err(if device_lost {
                        rhapsody::Error::DeviceLost
                    } else {
                        rhapsody::Error::OutOfDeviceMemory
                    })
                });
            let (declaration, mut callbacks, _) = graph.build().into_parts();
            let error = chain
                .execute_graph_with_context(
                    declaration,
                    &mut Pool::default(),
                    &mut Dependency,
                    &mut backend,
                    &mut callbacks,
                )
                .unwrap_err();
            assert_eq!(
                rhapsody::Error::from(error).kind(),
                if device_lost {
                    rhapsody::ErrorKind::DeviceLost
                } else {
                    rhapsody::ErrorKind::OutOfMemory
                }
            );
            assert_eq!(backend.submit_attempts, 0);
            assert_eq!(chain.recording_command_count(), 0);
            assert!(!backend.abandoned.is_empty());
        }
    }
}

#[test]
fn reported_callback_errors_cannot_be_ignored_by_returning_true() {
    use rhapsody::rg::parallel::ParallelRecording;
    for parallel in [
        ParallelRecording::PARALLEL,
        ParallelRecording {
            ..ParallelRecording::EVERY_PASS
        },
    ] {
        let mut backend = Mock::default();
        let mut chain = chain();
        chain.parallel = parallel;
        let mut graph = RenderGraph::new();
        graph
            .add_pass("reported error")
            .with_side_effect()
            .render_with_context(|context| {
                context.report_error(rhapsody::Error::OutOfHostMemory);
                true
            });
        graph
            .add_pass("sibling")
            .with_side_effect()
            .render(|_| true);
        let (declaration, mut callbacks, _) = graph.build().into_parts();
        let error = chain
            .execute_graph_with_context(
                declaration,
                &mut Pool::default(),
                &mut Dependency,
                &mut backend,
                &mut callbacks,
            )
            .unwrap_err();
        assert_eq!(
            rhapsody::Error::from(error).kind(),
            rhapsody::ErrorKind::OutOfMemory
        );
        assert_eq!(backend.submit_attempts, 0);
    }
}

#[test]
fn result_callback_error_after_a_successful_batch_keeps_the_partial_transaction() {
    let mut backend = Mock::default();
    let mut chain = chain();
    let mut graph = RenderGraph::new();
    graph.batch_per_pass(true);
    graph
        .add_pass("first batch")
        .with_side_effect()
        .render_with_result_context(|_| Ok(()));
    graph
        .add_pass("second batch")
        .with_side_effect()
        .render_with_result_context(|_| Err(rhapsody::Error::OutOfDeviceMemory));
    let (declaration, mut callbacks, _) = graph.build().into_parts();
    let error = chain
        .execute_graph_with_context(
            declaration,
            &mut Pool::default(),
            &mut Dependency,
            &mut backend,
            &mut callbacks,
        )
        .unwrap_err();
    let Error::PartialSubmission {
        original,
        execution,
    } = error
    else {
        panic!("successful batch must remain pending");
    };
    assert!(matches!(
        *original,
        Error::Validation(rhapsody::Error::OutOfDeviceMemory)
    ));
    assert_eq!(execution.transaction.receipts().len(), 1);
    assert_eq!(backend.submit_attempts, 1);
    assert_eq!(chain.pending_command_count(), 1);
}

#[test]
fn cached_plans_cull_dead_callbacks_and_rebind_each_frames_imports() {
    use std::sync::{Arc, Mutex};
    let calls = Arc::new(Mutex::new(Vec::new()));
    let mut chain = chain();
    let mut pool = Pool::default();
    let mut backend = Mock::default();
    let mut first_plan = None;
    for frame in 0..3u64 {
        let mut graph = RenderGraph::new();
        let input = graph
            .import_unowned("input", buffer(), 50 + frame, 0x35)
            .unwrap();
        let output = create(&mut graph, "output");
        let dead = create(&mut graph, "unused");
        let mut pass = graph.add_pass("unused computation");
        pass.add_resource_access(dead, WRITE, 0).unwrap();
        pass.render(|_| panic!("a culled callback must never run"));
        let mut pass = graph.add_pass("compute output");
        pass.add_resource_access(input, 0x25, 0).unwrap();
        pass.add_resource_access(output, WRITE, 0).unwrap();
        let trace = calls.clone();
        pass.render(move |_| {
            trace.lock().unwrap().push((frame, "output"));
            true
        });
        let trace = calls.clone();
        graph
            .add_pass("external effect")
            .with_side_effect()
            .render(move |_| {
                trace.lock().unwrap().push((frame, "effect"));
                true
            });
        graph.export(output, 0x25).unwrap();
        let (declaration, mut callbacks, _) = graph.build().into_parts();
        let execution = chain
            .execute_graph_with_context(
                declaration,
                &mut pool,
                &mut Dependency,
                &mut backend,
                &mut callbacks,
            )
            .unwrap();
        assert_eq!(execution.compiled.plan.retained_passes, [1, 2]);
        assert!(Arc::ptr_eq(
            &execution.compiled.info,
            &execution.compiled.plan.info
        ));
        assert!(Arc::ptr_eq(
            &execution.compiled.snapshot.shared,
            &execution.compiled.plan.snapshot
        ));
        assert_eq!(execution.stats.culled_passes, 1);
        assert_eq!(execution.stats.plan_cache_hit, frame != 0);
        assert_eq!(
            graph::payload_projection(execution.compiled.registry.resource(input.raw()).0).1,
            Some(50 + frame)
        );
        assert!(!execution.compiled.registry.is_populated(dead.raw()));
        if let Some(plan) = &first_plan {
            assert!(Arc::ptr_eq(plan, &execution.compiled.plan));
        } else {
            first_plan = Some(execution.compiled.plan.clone());
        }
    }
    assert_eq!(
        *calls.lock().unwrap(),
        [
            (0, "output"),
            (0, "effect"),
            (1, "output"),
            (1, "effect"),
            (2, "output"),
            (2, "effect")
        ]
    );
    assert_eq!(chain.plan_cache_stats().hits, 2);
    assert!(chain.plan_cache_bytes() <= 16 * 1024 * 1024);
}

#[test]
fn stable_subgraph_analysis_is_reused_on_the_real_executor_whole_plan_miss_path() {
    use rhapsody::rg::resource::AccessRange;
    use std::sync::{Arc, Mutex};
    let calls = Arc::new(Mutex::new(Vec::new()));
    let mut chain = chain();
    let mut pool = Pool::default();
    let mut backend = Mock::default();
    for frame in 0..3u64 {
        let mut graph = RenderGraph::new();
        let input = graph
            .import_unowned("input", buffer(), 70 + frame, 0x25)
            .unwrap();
        let middle = create(&mut graph, "middle");
        let output = create(&mut graph, "output");
        graph
            .subgraph("stable", |subgraph| {
                subgraph.input(input, AccessRange::Whole, 0x25)?;
                let mut pass = subgraph.graph().add_pass("stable copy");
                pass.add_resource_access(input, 0x25, 0)?;
                pass.add_resource_access(middle, WRITE, 0)?;
                let trace = calls.clone();
                pass.render(move |_| {
                    trace.lock().unwrap().push((frame, "stable"));
                    true
                });
                subgraph.output(middle, AccessRange::Whole, WRITE)?;
                Ok(())
            })
            .unwrap();
        graph
            .subgraph("changing", |subgraph| {
                subgraph.interface_mut().implementation_signature = frame;
                subgraph.input(middle, AccessRange::Whole, 0x25)?;
                let mut pass = subgraph.graph().add_pass(format!("changed frame {frame}"));
                pass.add_resource_access(middle, 0x25, 0)?;
                pass.add_resource_access(output, WRITE, 0)?;
                let trace = calls.clone();
                pass.render(move |_| {
                    trace.lock().unwrap().push((frame, "changing"));
                    true
                });
                subgraph.output(output, AccessRange::Whole, WRITE)?;
                Ok(())
            })
            .unwrap();
        graph.export(output, 0x25).unwrap();
        let (declaration, mut callbacks, _) = graph.build().into_parts();
        let execution = chain
            .execute_graph_with_context(
                declaration,
                &mut pool,
                &mut Dependency,
                &mut backend,
                &mut callbacks,
            )
            .unwrap();
        assert!(
            !execution.stats.plan_cache_hit,
            "the changing subgraph must invalidate the whole plan"
        );
        assert!(
            execution.compiled.plan.dependency_edges.contains(&(0, 1)),
            "cross-subgraph write/read dependency must survive local reuse"
        );
        assert!(
            backend
                .synchronization_plan()
                .bound()
                .transitions()
                .iter()
                .any(|transition| transition.phase
                    == rhapsody::rg::sync_plan::BarrierPhase::BeforePass(1)
                    && transition.source.write
                    && !transition.destination.write),
            "global coordination must restore the boundary barrier"
        );
        assert_eq!(
            graph::payload_projection(execution.compiled.registry.resource(input.raw()).0).1,
            Some(70 + frame)
        );
        let stats = chain.subgraph_cache_stats();
        assert_eq!(stats.compilations, frame + 2);
        assert_eq!(stats.hits, frame);
        assert!(stats.bytes <= 16 * 1024 * 1024);
    }
    assert_eq!(
        *calls.lock().unwrap(),
        [
            (0, "stable"),
            (0, "changing"),
            (1, "stable"),
            (1, "changing"),
            (2, "stable"),
            (2, "changing")
        ]
    );
}

/// 三个缓冲在同一 pass 中写入：r0 命中资源池，r1 新建，r2 第二次创建（可配置失败）。
fn three_buffer_graph(callback_ok: bool) -> graph::GraphDeclaration {
    let mut graph = RenderGraph::new();
    let handles = ["r0", "r1", "r2"].map(|name| create(&mut graph, name));
    let mut pass = graph.add_pass("write").with_side_effect();
    for handle in handles {
        pass.add_resource_access(handle, WRITE, 0).unwrap();
    }
    pass.render(move |_| callback_ok);
    graph.build().into_parts().0
}

fn run(
    chain: &mut ExecutionChain,
    declaration: graph::GraphDeclaration,
    pool: &mut Pool,
    backend: &mut Mock,
    ok: bool,
) -> Result<rhapsody::rg::executor::GraphExecution, Error> {
    let mut callbacks = (0..declaration.passes.len())
        .map(|_| move |_: u64| ok)
        .collect::<Vec<_>>();
    chain.execute_graph(declaration, pool, &mut Dependency, backend, &mut callbacks)
}

#[test]
fn native_recording_errors_propagate_directly_and_do_not_poison_the_next_frame() {
    for fail_begin in [true, false] {
        let mut backend = Mock { fail_begin, fail_end_at: (!fail_begin).then_some(1), ..Default::default() };
        let mut chain = chain();
        let mut pool = Pool::default();
        let error = run(&mut chain, three_buffer_graph(true), &mut pool, &mut backend, true).unwrap_err();
        assert!(if fail_begin {
            matches!(error, Error::Validation(rhapsody::Error::OutOfHostMemory))
        } else {
            matches!(error, Error::Validation(rhapsody::Error::DeviceLost))
        });
        assert_eq!(backend.submit_attempts, 0);
        assert_eq!(chain.recording_command_count(), 0);
        backend.fail_begin = false;
        backend.fail_end_at = None;
        run(&mut chain, three_buffer_graph(true), &mut pool, &mut backend, true).unwrap();
        assert_eq!(backend.submit_attempts, 1);
    }
}

#[test]
fn an_end_error_after_submission_keeps_the_original_error_and_gpu_receipt() {
    let mut backend = Mock { fail_end_at: Some(2), ..Default::default() };
    let mut graph = RenderGraph::new();
    graph.batch_per_pass(true);
    for name in ["first", "second"] {
        graph.add_pass(name).with_side_effect().render(|_| true);
    }
    let mut chain = chain();
    let error = run(&mut chain, graph.build().into_parts().0, &mut Pool::default(), &mut backend, true).unwrap_err();
    let Error::PartialSubmission { original, execution } = error else { panic!("successful prefix must remain owned") };
    assert!(matches!(*original, Error::Validation(rhapsody::Error::DeviceLost)));
    assert_eq!(execution.transaction.receipts().len(), 1);
    assert_eq!(chain.pending_command_count(), 1);
    assert_eq!(chain.recording_command_count(), 0);
    assert_eq!(backend.abandoned, [1002]);
}

#[test]
fn cached_execution_rejects_aliased_imports_at_the_single_declaration_boundary() {
    let mut graph = RenderGraph::new();
    let first = graph.import_unowned("first import", buffer(), 77, 0x25).unwrap();
    let second = graph.import_unowned("second import", buffer(), 88, 0x25).unwrap();
    let mut pass = graph.add_pass("read both").with_side_effect();
    pass.add_resource_access(first, 0x25, 0).unwrap();
    pass.add_resource_access(second, 0x25, 0).unwrap();
    pass.render(|_| true);
    let mut declaration = graph.build().into_parts().0;
    let mut chain = chain();
    let mut backend = Mock::default();
    let mut pool = Pool::default();
    run(&mut chain, declaration.clone(), &mut pool, &mut backend, true).unwrap();
    let submitted = backend.submit_attempts;
    declaration.resources[1].source = declaration.resources[0].source;
    let error = run(&mut chain, declaration, &mut pool, &mut backend, true).unwrap_err();
    assert_eq!(rhapsody::Error::from(error).kind(), rhapsody::ErrorKind::InvalidUsage);
    assert_eq!(backend.submit_attempts, submitted);
    assert_eq!(backend.creations, 0);
}

// ---------------------------------------------------------------- 1. 导出但从未访问的 Create 资源

#[test]
fn an_exported_but_unaccessed_create_resource_is_rejected_before_allocation() {
    let mut graph = RenderGraph::new();
    let touched = create(&mut graph, "touched");
    let exported = create(&mut graph, "export only");
    let mut pass = graph.add_pass("touch").with_side_effect();
    pass.add_resource_access(touched, WRITE, 0).unwrap();
    pass.render(|_| true);
    graph.export(exported, 0x26).unwrap();
    let declaration = graph.build().into_parts().0;
    let mut backend = EventBackend::default();
    let error = graph::compile_graph(
        declaration,
        &mut Pool::default(),
        &mut backend,
        &mut Dependency,
    )
    .err()
    .expect("uninitialized exports must be rejected");
    assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
    assert!(
        backend.events.is_empty(),
        "validation runs before physical allocation"
    );
}

#[test]
fn executing_an_uninitialized_export_records_and_allocates_nothing() {
    let mut graph = RenderGraph::new();
    let touched = create(&mut graph, "touched");
    let exported = create(&mut graph, "export only");
    let mut pass = graph.add_pass("touch").with_side_effect();
    pass.add_resource_access(touched, WRITE, 0).unwrap();
    pass.render(|_| true);
    graph.export(exported, 0x26).unwrap();
    let declaration = graph.build().into_parts().0;
    let mut backend = Mock::default();
    let mut pool = Pool::default();
    let error = run(&mut chain(), declaration, &mut pool, &mut backend, true)
        .err()
        .unwrap();
    assert!(
        matches!(error, Error::Compile(ref source) if source.kind() == rhapsody::ErrorKind::InvalidUsage),
        "{error:?}"
    );
    assert_eq!(backend.creations, 0);
    assert!(backend.released.is_empty());
    assert_eq!(backend.next_command, 0);
}

// ---------------------------------------------------------------- 2. 失败路径归还资源

#[test]
fn a_population_failure_returns_pool_resources_and_releases_created_tokens() {
    let mut pool = Pool::default();
    pool.insert_buffer(pooled_buffer(900));
    let mut backend = Mock {
        fail_creation_at: Some(1),
        ..Default::default()
    };
    let error = run(
        &mut chain(),
        three_buffer_graph(true),
        &mut pool,
        &mut backend,
        true,
    )
    .err()
    .unwrap();
    assert!(matches!(error, Error::Compile(_)), "{error:?}");
    let Descriptor::Buffer(desc) = buffer().with_usage(0x20) else {
        unreachable!()
    };
    assert_eq!(
        pool.take_buffer(&desc).map(|b| b.handle),
        Some(900),
        "资源池命中必须归还"
    );
    assert_eq!(
        backend.released,
        vec![501],
        "失败前已创建的 token 必须交还后端"
    );
}

#[test]
fn a_failure_after_compilation_returns_pool_resources_and_releases_created_tokens() {
    let mut pool = Pool::default();
    pool.insert_buffer(pooled_buffer(900));
    let mut backend = Mock {
        fail_prepare: true,
        ..Default::default()
    };
    let error = run(
        &mut chain(),
        three_buffer_graph(true),
        &mut pool,
        &mut backend,
        true,
    )
    .err()
    .unwrap();
    assert!(
        matches!(error, Error::Validation(rhapsody::Error::OutOfHostMemory)),
        "{error:?}"
    );
    let Descriptor::Buffer(desc) = buffer().with_usage(0x20) else {
        unreachable!()
    };
    assert_eq!(pool.take_buffer(&desc).map(|b| b.handle), Some(900));
    let mut released = backend.released.clone();
    released.sort_unstable();
    assert_eq!(released, vec![501, 502]);
    assert!(
        backend.abandoned.is_empty(),
        "命令开始前失败不应有命令需要放弃"
    );
}

#[test]
fn a_recording_failure_abandons_begun_commands_and_reclaims_resources() {
    let mut pool = Pool::default();
    pool.insert_buffer(pooled_buffer(900));
    let mut backend = Mock::default();
    let mut chain = chain();
    let error = run(
        &mut chain,
        three_buffer_graph(false),
        &mut pool,
        &mut backend,
        false,
    )
    .err()
    .unwrap();
    assert!(matches!(error, Error::Record(_)), "{error:?}");
    let mut abandoned = backend.abandoned.clone();
    abandoned.sort_unstable();
    assert_eq!(abandoned, vec![1001], "已开始但未提交的命令全部放弃");
    let Descriptor::Buffer(desc) = buffer().with_usage(0x20) else {
        unreachable!()
    };
    assert_eq!(
        pool.take_buffer(&desc).map(|b| b.handle),
        Some(900),
        "未提交任何批次时资源池命中可安全归还"
    );
    assert_eq!(backend.released.len(), 2);
}

// ---------------------------------------------------------------- 7. pass 数上限

#[test]
fn exceeding_the_pass_packing_domain_is_a_build_error() {
    let mut graph = RenderGraph::new();
    let resource = create(&mut graph, "r");
    for _ in 0..32768 {
        graph.add_pass("p").render(|_| true);
    }
    assert!(matches!(
        graph.try_add_pass("overflow"),
        Err(BuildError::PassLimit)
    ));
    let mut pass = graph.add_pass("overflow").with_side_effect();
    assert!(matches!(
        pass.add_resource_access(resource, WRITE, 0),
        Err(BuildError::PassLimit)
    ));
}

#[test]
fn malformed_declarations_fail_before_creating_resources() {
    use rhapsody::rg::graph::{AccessDeclaration, ExportDeclaration, PassDeclaration};
    let invalid_access = graph::GraphDeclaration {
        passes: vec![PassDeclaration {
            queue_class: rhapsody::device::QueueClass::Graphics,
            graph_id: None,
            side_effect: false,
            name: "invalid".into(),
            accesses: vec![AccessDeclaration {
                resource_index: u32::MAX,
                synchronization_group: 0,
                access_code: 0x34,
                ..Default::default()
            }],
        }],
        ..Default::default()
    };
    let invalid_export = graph::GraphDeclaration {
        exports: vec![ExportDeclaration {
            tag: 0,
            resource_index: 0,
            access_code: 0x26,
            range: rhapsody::rg::resource::AccessRange::Whole,
        }],
        ..Default::default()
    };
    for declaration in [invalid_access, invalid_export] {
        let mut backend = Mock::default();
        let error = graph::compile_graph(
            declaration,
            &mut graph::Pool::default(),
            &mut backend,
            &mut rhapsody::rg::vulkan::AccessTable::original(),
        )
        .err()
        .unwrap();
        assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
        assert_eq!(backend.creations, 0);
    }
}

#[test]
fn resource_creation_preserves_device_error_categories() {
    for injected in [
        rhapsody::Error::DeviceLost,
        rhapsody::Error::OutOfHostMemory,
        rhapsody::Error::OutOfDeviceMemory,
    ] {
        let expected = injected.kind();
        let mut backend = Mock {
            fail_creation_at: Some(1),
            creation_error: Some(injected),
            ..Default::default()
        };
        let error = run(
            &mut chain(),
            three_buffer_graph(true),
            &mut Pool::default(),
            &mut backend,
            true,
        )
        .err()
        .unwrap();
        assert_eq!(rhapsody::Error::from(error).kind(), expected);
        assert_eq!(
            backend.released,
            [501],
            "部分创建的资源在返回设备错误前仍须归还"
        );
    }
}

#[test]
fn failed_submit_does_not_publish_a_point_or_pending_commands() {
    let mut backend = Mock {
        fail_submit_at: Some(1),
        ..Default::default()
    };
    let mut chain = chain();
    let mut pool = Pool::default();
    let error = run(
        &mut chain,
        three_buffer_graph(true),
        &mut pool,
        &mut backend,
        true,
    )
    .err()
    .unwrap();
    assert!(matches!(
        error,
        Error::Validation(rhapsody::Error::OutOfDeviceMemory)
    ));
    assert_eq!(chain.pending_command_count(), 0);
    assert_eq!(chain.recording_command_count(), 0);
    assert_eq!(backend.next_submit, 0);
    assert!(backend.waits.borrow().is_empty());
    assert_eq!(backend.abandoned, [1001]);
    let execution = run(
        &mut chain,
        three_buffer_graph(true),
        &mut pool,
        &mut backend,
        true,
    )
    .unwrap();
    assert_eq!(execution.transaction.receipts()[0].value, 1);
    assert_eq!(chain.pending_command_count(), 1);
}

#[test]
fn partial_submit_failure_defers_published_points_and_abandons_the_rest() {
    let mut backend = Mock {
        fail_submit_at: Some(2),
        ..Default::default()
    };
    let mut chain = chain();
    let mut pool = Pool::default();
    let mut graph = RenderGraph::new();
    graph.batch_per_pass(true);
    let resource = create(&mut graph, "scratch");
    for name in ["first", "second"] {
        let mut pass = graph.add_pass(name).with_side_effect();
        pass.add_resource_access(resource, WRITE, 0).unwrap();
        pass.render(|_| true);
    }
    let (declaration, mut callbacks, _) = graph.build().into_parts();
    let error = chain
        .execute_graph_with_context(
            declaration,
            &mut pool,
            &mut Dependency,
            &mut backend,
            &mut callbacks,
        )
        .err()
        .unwrap();
    let Error::PartialSubmission {
        original,
        execution,
    } = error
    else {
        panic!("partial submission expected")
    };
    assert!(matches!(
        *original,
        Error::Validation(rhapsody::Error::OutOfDeviceMemory)
    ));
    assert!(
        backend.waits.borrow().is_empty(),
        "error cleanup must not block"
    );
    assert_eq!(execution.transaction.receipts().len(), 1);
    assert_eq!(chain.pending_command_count(), 1);
    assert_eq!(chain.completed_command_count(), 0);
    assert_eq!(chain.recording_command_count(), 0);
    assert_eq!(backend.abandoned, [1002]);
}

#[test]
fn queue_domains_do_not_share_command_completion() {
    let mut backend = Mock::default();
    let mut chain = ExecutionChain::new(vk::Queue::null(), vec![vk::Queue::null()], 1);
    let mut pool = Pool::default();
    let first = run(
        &mut chain,
        three_buffer_graph(true),
        &mut pool,
        &mut backend,
        true,
    )
    .unwrap();
    backend.queue = Some(recording::Queue::Auxiliary(0));
    let second = run(
        &mut chain,
        three_buffer_graph(true),
        &mut pool,
        &mut backend,
        true,
    )
    .unwrap();
    assert_ne!(
        first.transaction.receipts()[0].queue,
        second.transaction.receipts()[0].queue
    );
    assert_eq!(chain.pending_command_count(), 2);
    // A third graph fails on its second batch; completing only its queue must
    // leave the primary queue's earlier command pending.
    backend.fail_submit_at = Some(4);
    let mut graph = RenderGraph::new();
    graph.batch_per_pass(true);
    let resource = create(&mut graph, "scratch");
    for name in ["first", "second"] {
        let mut pass = graph.add_pass(name).with_side_effect();
        pass.add_resource_access(resource, WRITE, 0).unwrap();
        pass.render(|_| true);
    }
    let (declaration, mut callbacks, _) = graph.build().into_parts();
    assert!(
        chain
            .execute_graph_with_context(
                declaration,
                &mut pool,
                &mut Dependency,
                &mut backend,
                &mut callbacks
            )
            .is_err()
    );
    assert_eq!(chain.pending_command_count(), 3);
    assert_eq!(chain.completed_command_count(), 0);
    assert!(backend.waits.borrow().is_empty());
}

#[test]
fn partial_submit_never_attempts_a_failing_cleanup_wait() {
    let mut backend = Mock {
        fail_submit_at: Some(2),
        fail_wait: true,
        ..Default::default()
    };
    let mut chain = chain();
    let mut pool = Pool::default();
    let mut graph = RenderGraph::new();
    graph.batch_per_pass(true);
    let resource = create(&mut graph, "scratch");
    for name in ["first", "second"] {
        let mut pass = graph.add_pass(name).with_side_effect();
        pass.add_resource_access(resource, WRITE, 0).unwrap();
        pass.render(|_| true);
    }
    let (declaration, mut callbacks, _) = graph.build().into_parts();
    let error = chain
        .execute_graph_with_context(
            declaration,
            &mut pool,
            &mut Dependency,
            &mut backend,
            &mut callbacks,
        )
        .unwrap_err();
    let Error::PartialSubmission {
        original,
        execution,
    } = error
    else {
        panic!("partial execution must survive cleanup failure")
    };
    assert!(matches!(
        *original,
        Error::Validation(rhapsody::Error::OutOfDeviceMemory)
    ));
    assert_eq!(execution.transaction.receipts().len(), 1);
    assert_eq!(chain.pending_command_count(), 1);
    assert_eq!(chain.recording_command_count(), 0);
    assert_eq!(backend.abandoned, [1002]);
    assert!(
        backend.released.is_empty(),
        "GPU resources cannot be reclaimed on a failed wait"
    );
    assert_eq!(
        execution
            .completion()
            .get(execution.transaction.receipts()[0].queue),
        1
    );
}

#[test]
fn transaction_keeps_partial_import_state_and_does_not_publish_failed_exports() {
    for failure in [Some(1), Some(2), None] {
        let mut backend = Mock {
            fail_submit_at: failure,
            ..Default::default()
        };
        let mut graph = RenderGraph::new();
        graph.batch_per_pass(true);
        let resource = graph
            .import_unowned("persistent", buffer(), 77, 0x25)
            .unwrap();
        for (name, code) in [("write", WRITE), ("read", 0x25)] {
            let mut pass = graph.add_pass(name).with_side_effect();
            pass.add_resource_access(resource, code, 0).unwrap();
            pass.render(|_| true);
        }
        graph.export(resource, 0x26).unwrap();
        let (declaration, mut callbacks, _) = graph.build().into_parts();
        let result = chain().execute_graph_with_context(
            declaration,
            &mut Pool::default(),
            &mut Dependency,
            &mut backend,
            &mut callbacks,
        );
        let cells = backend.ranges.states[&77].cells();
        assert_eq!(cells.len(), 1);
        let state = cells[0].state;
        match failure {
            Some(1) => {
                assert!(result.is_err());
                assert_eq!(state.revision, 0);
                assert_eq!(
                    state.readers.combined().access,
                    vk::AccessFlags::TRANSFER_READ
                );
                assert!(state.writer.points.is_empty());
            }
            Some(2) => {
                let Error::PartialSubmission { execution, .. } = result.unwrap_err() else {
                    panic!("partial")
                };
                assert!(state.writer.combined().write);
                assert_eq!(
                    state
                        .writer
                        .points
                        .get(execution.transaction.receipts()[0].queue),
                    1
                );
                assert!(
                    state.readers.points.is_empty(),
                    "failed export cannot publish a reader"
                );
                assert_eq!(execution.transaction.submitted_batches(), 1);
            }
            None => {
                let execution = result.unwrap();
                assert_eq!(
                    state.readers.combined().access,
                    vk::AccessFlags::MEMORY_READ
                );
                assert_eq!(
                    state
                        .readers
                        .points
                        .get(execution.transaction.receipts()[1].queue),
                    2
                );
                assert_eq!(execution.transaction.submitted_batches(), 2);
            }
            _ => unreachable!(),
        }
    }
}

#[test]
fn duplicate_imports_are_rejected_before_recording_or_allocation() {
    let mut graph = RenderGraph::new();
    graph.import_unowned("source", buffer(), 77, 0x25).unwrap();
    graph
        .import_unowned("other logical name", buffer(), 77, 0x25)
        .unwrap();
    let (declaration, mut callbacks, _) = graph.build().into_parts();
    assert!(matches!(
        declaration.validate(),
        Err(BuildError::AliasedImport(77))
    ));
    let mut backend = Mock::default();
    let error = chain()
        .execute_graph_with_context(
            declaration,
            &mut Pool::default(),
            &mut Dependency,
            &mut backend,
            &mut callbacks,
        )
        .unwrap_err();
    assert_eq!(
        rhapsody::Error::from(error).kind(),
        rhapsody::ErrorKind::InvalidUsage
    );
    assert_eq!(backend.creations, 0);
    assert_eq!(backend.submit_attempts, 0);
    assert_eq!(backend.next_command, 0);
}

#[test]
fn parallel_chunks_record_only_through_their_own_pool_leases() {
    use rhapsody::rg::parallel::ParallelRecording;
    {
        let mut backend = Mock::default();
        let mut chain = chain();
        chain.parallel = ParallelRecording {
            ..ParallelRecording::EVERY_PASS
        };
        let mut graph = RenderGraph::new();
        for name in ["a", "b", "c"] {
            graph.add_pass(name).with_side_effect().render(|_| true);
        }
        let (declaration, mut callbacks, _) = graph.build().into_parts();
        let result = chain.execute_graph_with_context(
            declaration,
            &mut Pool::default(),
            &mut Dependency,
            &mut backend,
            &mut callbacks,
        );
        let chunks = result.unwrap().stats.chunks;
        assert!(chunks > 1);
        // Every chunk recorded through a lease of its own command; a pool
        // shared by two leases cannot be expressed (see ChunkLease docs).
        let mut leased = backend.leased.into_inner().unwrap();
        assert_eq!(leased.len(), chunks as usize);
        leased.sort_unstable();
        leased.dedup();
        assert_eq!(leased.len(), chunks as usize);
    }
}
