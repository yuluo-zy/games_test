//! Graph recording and submission using typed CPU identities and actual queue completion points.
use super::transaction::FrameTransaction;
use crate::command::recording;
use crate::command::{
    CpuArenaLease, CpuArenaPool,
    cpu_arena::{scratch_borrows, scratch_slice},
};
use crate::raw::submit;
use crate::rg::recorder as record;
use ash::vk::{self, Handle};
use std::collections::{BTreeMap, BTreeSet};

pub struct ExecutionChain {
    subgraph_cache: super::subgraph::SubgraphPlanCache,
    plan_cache: super::cache::BoundedCache<
        super::plan::GraphDefinition,
        std::sync::Arc<super::plan::CompiledPlan>,
    >,
    plan_cache_enabled: bool,
    recording_ids: recording::RecordingCounters,
    pending_commands: Vec<(crate::sync::SyncPoint, recording::Command)>,
    completed_commands: Vec<recording::Command>,
    queues: Vec<vk::Queue>,
    command_records: BTreeMap<u64, recording::Command>,
    recorded_commands: u32,
    pool_family_count: u32,
    /// 并行录制配置；默认串行（与旧行为、原作开关关闭时一致）。
    pub parallel: crate::rg::parallel::ParallelRecording,
    /// 原作双缓冲 PGO：分块读 `previous`，本帧耗时写入 `current`，每次执行结束交换。
    pub pgo: PassPgoFrames,
}

/// 一个 batch 的并行录制输入。
struct ChunkedBatch<'a, C> {
    recorder: &'a mut record::Recorder,
    passes: &'a mut [record::Pass],
    callbacks: &'a mut [C],
    first_pass: usize,
    chunks: &'a [std::ops::Range<usize>],
    final_batch: bool,
    compiled: &'a crate::rg::graph::FrameInstance,
    managed: Option<&'a std::sync::Arc<crate::rg::pass_bindings::ManagedPassResources>>,
    pass_names: &'a [&'a std::sync::Arc<str>],
    cpu: Option<&'a CpuArenaLease>,
    family: u32,
    queue: recording::Queue,
}
/// 单块的独占工作单元。
struct ChunkWork<'a, C> {
    command: &'a recording::Command,
    start: usize,
    callbacks: &'a mut [C],
    durations: &'a mut [Option<u64>],
    transitions: &'a [Vec<ChunkTransition>],
    gpu_timings: &'a [Option<super::profiling::GpuPassTimestamp>],
}
struct ChunkResult {
    ok: bool,
    error: Option<crate::Error>,
    recorded_durations: usize,
    owners: super::owner_storage::ResourceOwners,
}

#[derive(Debug)]
pub enum Error {
    /// Some batches succeeded. The caller must retain the execution until its
    /// completion set is reached; error cleanup never blocks or rolls back GPU work.
    PartialSubmission {
        original: Box<Error>,
        execution: Box<GraphExecution>,
    },
    /// 并行录制的分块或规划无效，或后端未提供并行视图。
    ChunkPlan,
    CallbackPanic,
    Begin(recording::BeginFailure),
    Lifecycle(recording::Failure),
    Record(record::Outcome),
    MissingCommand(u64),
    EmptyCommands,
    InvalidLastState,
    EndFailed,
    Compile(crate::Error),
    ResourceUnavailable(usize),
    CallbackCount,
    ManagedResourceUnresolved(usize),
    QueueUnavailable(recording::Queue),
    InvalidSynchronization,
    UnsupportedQueueDomainSwitch,
    ResourceCountMismatch {
        expected: usize,
        actual: usize,
    },
    /// 调用方对编译结果的校验失败（如 temporal 导出映射）；发生在任何命令开始之前。
    Validation(crate::Error),
}

#[derive(Clone, Copy, Debug)]
pub struct GraphTimelineWait {
    pub semaphore: vk::Semaphore,
    pub value: u64,
    pub stage: vk::PipelineStageFlags,
}
#[derive(Clone, Copy, Debug)]
pub struct GraphTimelineSignal {
    pub semaphore: vk::Semaphore,
    pub value: u64,
}
/// 外部生产的批次同步与图命令分别生成，避免调用者手填CPU recording identities。
#[derive(Clone, Debug, Default)]
pub struct GraphBatchSync {
    pub waits: submit::SubmitArray<GraphTimelineWait>,
    pub signals: submit::SubmitArray<GraphTimelineSignal>,
}
impl GraphBatchSync {
    fn valid(&self) -> bool {
        self.waits.iter().enumerate().all(|(index, wait)| {
            wait.semaphore != vk::Semaphore::null()
                && !wait.stage.is_empty()
                && !self.waits[..index]
                    .iter()
                    .any(|previous| previous.semaphore == wait.semaphore)
        }) && self.signals.iter().enumerate().all(|(index, signal)| {
            signal.semaphore != vk::Semaphore::null()
                && !self.signals[..index]
                    .iter()
                    .any(|previous| previous.semaphore == signal.semaphore)
        })
    }
}

/// 并行块内发出的一条 pass 前屏障；`token` 为物理资源 token。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChunkTransition {
    pub token: u64,
    pub transition: super::sync_plan::BoundTransition,
}

/// 后端在并行段暴露的只读录制视图：多个块同时调用，每次只写入调用方独占的命令缓冲。
pub trait ChunkRecorder: Sync {
    /// # Safety
    /// The command must be exclusively recording, and the pair must have been
    /// reserved for this command by this backend in the current CPU recording scope.
    unsafe fn write_pass_timestamp(
        &self,
        _command: u64,
        _pair: super::profiling::GpuPassTimestamp,
        _end: bool,
    ) {
    }
    fn owner_storage_pool(
        &self,
    ) -> Option<&crate::raw::temp_list_pool::TempListPool<crate::raw::binding::RetainedResource>>
    {
        None
    }
    fn cpu_arena_pool(&self) -> Option<&CpuArenaPool> {
        None
    }
    /// Emit one pass's scope. Native/OOM errors retain their public categories.
    fn record_transitions(
        &self,
        command: u64,
        transitions: &[ChunkTransition],
        owners: &mut super::owner_storage::ResourceOwners,
    ) -> crate::Result<()>;
}

/// 一次图执行的阶段耗时与分块统计（性能追踪用）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ExecutionStats {
    /// Commands successfully begun for this execution, including ownership preludes.
    pub commands_recorded: u32,
    pub plan_cache_hit: bool,
    pub culled_passes: u32,
    pub compile_ns: u64,
    pub record_ns: u64,
    pub submit_ns: u64,
    pub batches: u32,
    /// 走并行路径的 batch 数及其总块数（串行 batch 不计块）。
    pub parallel_batches: u32,
    pub chunks: u32,
    /// 本次执行发出的屏障命令数（每个非空屏障作用域一次 `vkCmdPipelineBarrier`）。
    pub barrier_commands: u32,
    /// 本次执行包含的访问转换总数（屏障命令里的 buffer/image 屏障条目）。
    pub barrier_transitions: u32,
}
impl ExecutionStats {
    fn count_barriers(&mut self, outcome: &record::Outcome) {
        self.barrier_commands += outcome.barrier_commands;
        self.barrier_transitions += outcome.barrier_transitions;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct QueuePrelude {
    pub domain: super::queue_schedule::QueueDomain,
    pub tag: u64,
}

pub trait ExecutionBackend: crate::rg::graph::ResourceBackend {
    /// Reserve before recording workers run. Disabled profiling returns None
    /// and creates no query objects, owners or temporary heap allocations.
    fn reserve_pass_timestamp(
        &mut self,
        _command: u64,
        _pass: usize,
    ) -> Option<super::profiling::GpuPassTimestamp> {
        None
    }
    /// # Safety
    /// The command must be exclusively recording, and the pair must have been
    /// reserved for this command by this backend in the current CPU recording scope.
    unsafe fn write_pass_timestamp(
        &self,
        _command: u64,
        _pair: super::profiling::GpuPassTimestamp,
        _end: bool,
    ) {
    }
    fn requires_graph_completion(&self, _batch: usize, _total: usize) -> bool {
        false
    }
    fn select_queue_domain(
        &mut self,
        domain: super::queue_schedule::QueueDomain,
    ) -> crate::Result<()> {
        let context = self.plan_context();
        if context.queue_domain(crate::device::QueueClass::Graphics) != domain {
            return Err(crate::Error::unsupported(
                "backend does not support the requested queue domain",
            ));
        }
        Ok(())
    }
    fn next_queue_prelude(
        &mut self,
        _compiled: &crate::rg::graph::FrameInstance,
        _batch: usize,
    ) -> crate::Result<Option<QueuePrelude>> {
        Ok(None)
    }
    fn record_queue_prelude(&mut self, _command: u64, _prelude: QueuePrelude) -> crate::Result<()> {
        Ok(())
    }
    fn select_main_batch(
        &mut self,
        _compiled: &crate::rg::graph::FrameInstance,
        _batch: usize,
    ) -> crate::Result<()> {
        Ok(())
    }
    fn timeline_wait(&self, _point: crate::sync::SyncPoint) -> crate::Result<GraphTimelineWait> {
        Err(crate::Error::unsupported(
            "backend has no device-owned queue timeline",
        ))
    }

    fn cpu_arena_pool(&self) -> Option<&CpuArenaPool> {
        None
    }
    fn plan_context(&self) -> super::plan::PlanContext {
        let (queue, family) = self.graph_queue();
        super::plan::PlanContext {
            queue_family: family,
            queue_index: match queue {
                recording::Queue::Primary => 0,
                recording::Queue::Auxiliary(i) => i + 1,
            },
            ..Default::default()
        }
    }
    /// Shared preflight for sessions and direct execution. Runs before allocation,
    /// recording, or submission; production backends repair partial-frame imports
    /// and validate physical identity, ownership and synchronization here.
    fn prepare_declaration(
        &mut self,
        declaration: &mut crate::rg::graph::GraphDeclaration,
    ) -> crate::Result<()> {
        declaration.validate().map_err(Into::into)
    }
    /// The same bounded projection drives serial, parallel and ownership recording.
    fn synchronization_plan(&self) -> std::sync::Arc<super::sync_plan::SynchronizationPlanner>;
    /// Attach pre-reserved physical state deltas to the actual raw batch before
    /// submission. Batch references resolve only to successful actual receipts.
    fn prepare_batch_submission(&mut self, commands: &[u64], batch: usize) -> crate::Result<()>;
    /// 兼容录制器 bool/生命周期接口期间，取出设备调用的原始错误，避免丢失错误类别。
    fn take_error(&mut self) -> Option<crate::Error> {
        None
    }
    /// 包括 Pool 命中与导入资源；运行回调前重新发布真实物理映射。
    fn prepare_compiled_resources(&mut self, _compiled: &crate::rg::graph::FrameInstance) -> bool {
        true
    }
    /// 物理对象与 generation 由本次后端解析；未提供时 callback 明确看到 None。
    fn managed_pass_resources(
        &self,
    ) -> Option<std::sync::Arc<crate::rg::pass_bindings::ManagedPassResources>> {
        None
    }
    fn owner_storage_pool(
        &self,
    ) -> Option<&crate::raw::temp_list_pool::TempListPool<crate::raw::binding::RetainedResource>>
    {
        None
    }
    fn retain_command_owners(
        &mut self,
        _handle: u64,
        owners: &mut super::owner_storage::ResourceOwners,
    ) -> bool {
        owners.is_empty()
    }
    fn graph_queue(&self) -> (recording::Queue, u32) {
        (recording::Queue::Primary, 0)
    }
    fn batch_synchronization(&self, _index: usize, _total: usize) -> GraphBatchSync {
        GraphBatchSync::default()
    }
    /// 设备和命令创建由后端提供；真实实现见 rg/vulkan.rs。
    fn command_handle(&mut self) -> Result<u64, recording::Failure>;
    /// 并行录制时第 `chunk` 块使用的命令；真实后端须保证不同块来自不同命令池。
    fn chunk_command_handle(&mut self, _chunk: usize) -> Result<u64, recording::Failure> {
        self.command_handle()
    }
    fn begin_command(&mut self, handle: u64) -> Result<(), recording::Failure>;
    fn end_command(&mut self, handle: vk::CommandBuffer) -> bool;
    /// CPU 后端可只记录投影；实际后端必须发出 Vulkan 屏障。
    fn transition(
        &mut self,
        _command: u64,
        _token: u64,
        _transition: &super::sync_plan::BoundTransition,
    ) -> bool {
        true
    }
    fn flush_barriers(&mut self, _command: u64) -> bool {
        true
    }
    fn submit(
        &mut self,
        queue: vk::Queue,
        prepared: &[submit::PreparedSubmit],
    ) -> crate::Result<crate::sync::SyncPoint>;
    /// 支持并行录制的后端返回只读视图；返回 None 时执行器始终走串行路径。
    fn chunk_recorder(&self) -> Option<Box<dyn ChunkRecorder + '_>> {
        None
    }
    /// 失败路径：已分配或已开始、但不会提交的命令交还后端（它从未进入 GPU）。默认不做任何事。
    fn abandon_command(&mut self, _handle: u64) {}
    fn wait_submission(&self, _receipt: crate::sync::SyncPoint) -> crate::Result<()> {
        Ok(())
    }
}

/// An allocated graph before command recording. It is bound to its originating
/// backend and resource pool; cancel it there if the consumer abandons the frame.
pub struct PreparedGraph {
    pub compiled: crate::rg::graph::FrameInstance,
    pub stats: ExecutionStats,
    cpu_arena: Option<CpuArenaLease>,
    queue: (recording::Queue, u32),
    original_pass_count: usize,
}

pub struct GraphExecution {
    pub compiled: crate::rg::graph::FrameInstance,
    pub transaction: FrameTransaction,
    pub stats: ExecutionStats,
}

impl std::fmt::Debug for GraphExecution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GraphExecution")
            .field("transaction", &self.transaction)
            .field("stats", &self.stats)
            .finish_non_exhaustive()
    }
}
impl GraphExecution {
    pub fn completion(&self) -> crate::sync::SyncSet {
        self.transaction.completion()
    }
}

/// 本次录制的命令和编译图；绑定消费者不能另外分配序号或重新构造资源映射。
pub struct PassContext<'a> {
    pub pass_index: usize,
    /// CPU-only scratch, valid through this callback; never GPU upload backing.
    pub cpu: Option<&'a CpuArenaLease>,
    pub command: &'a recording::Command,
    pub compiled: &'a crate::rg::graph::FrameInstance,
    pub managed: Option<&'a crate::rg::pass_bindings::ManagedPassResources>,
    /// 由 recorder 收集并交给同一命令拥有者，callback 返回后仍需保活。
    pub owners: &'a std::cell::RefCell<super::owner_storage::ResourceOwners>,
    pub errors: &'a std::cell::RefCell<Option<crate::Error>>,
}
impl PassContext<'_> {
    pub fn history_valid(&self, texture: crate::rg::resource::TextureId) -> crate::Result<bool> {
        self.named_history_valid(texture.raw(), true)
    }
    pub fn buffer_history_valid(
        &self,
        buffer: crate::rg::resource::BufferId,
    ) -> crate::Result<bool> {
        self.named_history_valid(buffer.raw(), false)
    }
    fn named_history_valid(
        &self,
        handle: crate::rg::resource::ResourceHandle,
        image: bool,
    ) -> crate::Result<bool> {
        let graph = &self.compiled.declarations;
        if graph
            .passes
            .get(self.pass_index)
            .and_then(|pass| pass.graph_id)
            != Some(handle.graph_id())
        {
            return Err(crate::Error::invalid_usage(
                "history handle belongs to another graph",
            ));
        }
        let index = handle.raw() as usize;
        let declaration = graph
            .resources
            .get(index)
            .ok_or_else(|| crate::Error::invalid_usage("history handle is out of range"))?;
        if matches!(
            declaration.descriptor,
            crate::rg::graph::Descriptor::Image(_)
        ) != image
        {
            return Err(crate::Error::invalid_usage(
                "history handle has the wrong resource type",
            ));
        }
        self.compiled
            .history_validity
            .as_ref()
            .and_then(|validity| validity.get(index))
            .copied()
            .flatten()
            .ok_or_else(|| {
                crate::Error::invalid_usage("resource is not a session-owned named resource")
            })
    }

    pub fn report_error(&self, error: crate::Error) {
        self.errors.borrow_mut().get_or_insert(error);
    }
    /// Retain a pipeline, query or other native object with no image/buffer
    /// memory domain. Graph memory must be declared before recording begins.
    pub fn retain_object<T: crate::resource::GpuUse + 'static>(
        &self,
        owner: std::sync::Arc<T>,
    ) -> crate::Result<()> {
        let result = owner
            .visit_uses(&mut |cell| {
                if cell.ownership_range().is_some() {
                    Err(crate::Error::invalid_usage(
                        "graph memory references require a declared resource access",
                    ))
                } else {
                    Ok(())
                }
            })
            .and_then(|()| self.owners.borrow_mut().add_object(owner));
        self.retention_result(result)
    }
    /// # Safety
    /// Every native memory reference must be covered by this pass's validated
    /// declarations or the formal immutable upload protocol.
    pub(crate) unsafe fn retain_recorded<T: crate::resource::GpuUse + 'static>(
        &self,
        owner: std::sync::Arc<T>,
    ) -> crate::Result<()> {
        let result = self.owners.borrow_mut().add_recorded(owner);
        self.retention_result(result)
    }
    pub(crate) fn retain_upload_read(
        &self,
        buffer: std::sync::Arc<crate::raw::buffer::Buffer>,
        offset: u64,
        size: u64,
        scope: crate::resource::range_state::AccessScope,
    ) -> crate::Result<()> {
        let result = self
            .owners
            .borrow_mut()
            .add_upload_read(buffer, offset, size, scope);
        self.retention_result(result)
    }
    fn retention_result(&self, result: crate::Result<()>) -> crate::Result<()> {
        result.map_err(|error| {
            let shared = std::sync::Arc::new(error);
            self.report_error(shared.clone().into());
            shared.into()
        })
    }
}

impl ExecutionChain {
    /// 候选公开入口只从图声明生成编译和录制状态；原游戏构图闭包及完整原链 oracle 尚未恢复。
    pub fn execute_graph(
        &mut self,
        graph: crate::rg::graph::GraphDeclaration,
        pool: &mut crate::rg::graph::Pool,
        dependency: &mut impl crate::rg::graph::AccessDependency,
        backend: &mut impl ExecutionBackend,
        callbacks: &mut [impl FnMut(u64) -> bool + Send],
    ) -> Result<GraphExecution, Error> {
        let mut contextual = callbacks
            .iter_mut()
            .map(|callback| move |context: PassContext<'_>| callback(context.command.handle()))
            .collect::<Vec<_>>();
        self.execute_graph_with_context(graph, pool, dependency, backend, &mut contextual)
    }

    /// Supply the recorded command and physical resource registry together.
    pub fn execute_graph_with_context(
        &mut self,
        graph: crate::rg::graph::GraphDeclaration,
        pool: &mut crate::rg::graph::Pool,
        dependency: &mut impl crate::rg::graph::AccessDependency,
        backend: &mut impl ExecutionBackend,
        callbacks: &mut [impl FnMut(PassContext<'_>) -> bool + Send],
    ) -> Result<GraphExecution, Error> {
        self.execute_graph_validated(graph, pool, dependency, backend, callbacks, |_| Ok(()))
    }

    /// 同 [`Self::execute_graph_with_context`]，但在编译之后、任何命令开始之前先交给 `validate` 校验编译结果；
    /// 校验失败返回 [`Error::Validation`]，并按失败路径归还本次取出/新建的资源。
    pub fn execute_graph_validated(
        &mut self,
        graph: crate::rg::graph::GraphDeclaration,
        pool: &mut crate::rg::graph::Pool,
        dependency: &mut impl crate::rg::graph::AccessDependency,
        backend: &mut impl ExecutionBackend,
        callbacks: &mut [impl FnMut(PassContext<'_>) -> bool + Send],
        validate: impl FnOnce(&crate::rg::graph::FrameInstance) -> crate::Result<()>,
    ) -> Result<GraphExecution, Error> {
        let result =
            self.execute_graph_inner(graph, pool, dependency, backend, callbacks, validate);
        // 原 end_frame：本帧耗时成为下一帧的分块依据。
        self.pgo.end_frame();
        result
    }

    /// Original compile/execute separation: compilation allocates and validates
    /// the physical frame, but never records a callback or submits a command.
    /// B RenderGraphState::compile instances 0x140161cd0 / 0x1401647b0 /
    /// 0x140166510 precede execute_compiled 0x1400fa190 in the game consumer.
    pub fn compile_graph(
        &mut self,
        mut graph: crate::rg::graph::GraphDeclaration,
        pool: &mut crate::rg::graph::Pool,
        dependency: &mut impl crate::rg::graph::AccessDependency,
        backend: &mut impl ExecutionBackend,
        callback_count: usize,
    ) -> Result<PreparedGraph, Error> {
        let mut stats = ExecutionStats::default();
        if callback_count != graph.passes.len() {
            return Err(Error::CallbackCount);
        }
        backend
            .prepare_declaration(&mut graph)
            .map_err(Error::Validation)?;
        let (queue, family) = backend.graph_queue();
        if family >= self.pool_family_count {
            return Err(Error::Lifecycle(recording::Failure::PoolFamilyBounds));
        }
        if matches!(queue, recording::Queue::Auxiliary(i) if i as usize + 1 >= self.queues.len()) {
            return Err(Error::QueueUnavailable(queue));
        }
        let compile_start = std::time::Instant::now();
        let context = backend.plan_context();
        let cached = if self.plan_cache_enabled {
            self.plan_cache
                .try_get_matching(|definition| {
                    definition.matches_declaration(&graph, dependency, context)
                })
                .map_err(Error::Compile)?
                .cloned()
        } else {
            self.plan_cache
                .record_bypass(super::cache::CacheReason::Disabled);
            None
        };
        let plan = if let Some(plan) = cached {
            stats.plan_cache_hit = true;
            plan
        } else {
            let definition =
                super::plan::GraphDefinition::from_declaration(&graph, dependency, context)
                    .map_err(Error::Compile)?;
            if self.plan_cache_enabled {
                let reason = self
                    .plan_cache
                    .iter()
                    .map(|(old, _)| old.differing_key_reason(&definition))
                    .min_by_key(|reason| *reason as usize)
                    .unwrap_or(super::cache::CacheReason::Cold);
                self.plan_cache.classify_miss(reason);
            }
            let plan = std::sync::Arc::new(
                super::plan::compile_with_subgraphs(definition.clone(), &mut self.subgraph_cache)
                    .map_err(Error::Compile)?,
            );
            if self.plan_cache_enabled {
                // Count both the normalized key and the immutable plan's storage.
                let bytes = plan.estimated_bytes().saturating_mul(2);
                self.plan_cache.insert(definition, plan.clone(), bytes);
            }
            plan
        };
        stats.culled_passes = graph
            .passes
            .len()
            .saturating_sub(plan.retained_passes.len()) as u32;
        let cpu_arena = backend
            .cpu_arena_pool()
            .map(CpuArenaPool::checkout)
            .transpose()
            .map_err(Error::Validation)?;
        // Allocation is a frame binding operation, never part of plan compilation.
        let compiled =
            crate::rg::graph::instantiate_validated_plan(plan, graph, pool, backend, dependency)
                .map_err(Error::Compile)?;
        stats.compile_ns = compile_start.elapsed().as_nanos() as u64;
        let original_pass_count = callback_count;
        Ok(PreparedGraph { compiled, stats, cpu_arena, queue: (queue, family), original_pass_count })
    }

    fn execute_graph_inner(
        &mut self,
        graph: crate::rg::graph::GraphDeclaration,
        pool: &mut crate::rg::graph::Pool,
        dependency: &mut impl crate::rg::graph::AccessDependency,
        backend: &mut impl ExecutionBackend,
        callbacks: &mut [impl FnMut(PassContext<'_>) -> bool + Send],
        validate: impl FnOnce(&crate::rg::graph::FrameInstance) -> crate::Result<()>,
    ) -> Result<GraphExecution, Error> {
        let prepared = self.compile_graph(graph, pool, dependency, backend, callbacks.len())?;
        self.execute_prepared_inner(prepared, pool, dependency, backend, callbacks, validate)
    }

    pub fn execute_prepared_validated(
        &mut self,
        prepared: PreparedGraph,
        pool: &mut crate::rg::graph::Pool,
        dependency: &mut impl crate::rg::graph::AccessDependency,
        backend: &mut impl ExecutionBackend,
        callbacks: &mut [impl FnMut(PassContext<'_>) -> bool + Send],
        validate: impl FnOnce(&crate::rg::graph::FrameInstance) -> crate::Result<()>,
    ) -> Result<GraphExecution, Error> {
        let result = self.execute_prepared_inner(prepared, pool, dependency, backend, callbacks, validate);
        self.pgo.end_frame();
        result
    }

    fn execute_prepared_inner(
        &mut self,
        prepared: PreparedGraph,
        pool: &mut crate::rg::graph::Pool,
        dependency: &mut impl crate::rg::graph::AccessDependency,
        backend: &mut impl ExecutionBackend,
        callbacks: &mut [impl FnMut(PassContext<'_>) -> bool + Send],
        validate: impl FnOnce(&crate::rg::graph::FrameInstance) -> crate::Result<()>,
    ) -> Result<GraphExecution, Error> {
        let PreparedGraph { mut compiled, mut stats, cpu_arena, queue: (queue, family), original_pass_count } = prepared;
        self.recorded_commands = 0;
        if callbacks.len() != original_pass_count {
            crate::rg::graph::reclaim_failed_resources(&mut compiled.tables.registry,
                &compiled.declarations.resources, &compiled.tables.snapshot.pool_hits, pool, backend);
            return Err(Error::CallbackCount);
        }
        let mut selected_callbacks = {
            let mut selected = callbacks
                .iter_mut()
                .enumerate()
                .filter(|(i, _)| compiled.plan.retained_passes.binary_search(i).is_ok())
                .map(|(_, callback)| callback);
            let selected = scratch_borrows(cpu_arena.as_ref(), compiled.plan.retained_passes.len(), |_| {
                selected.next().expect("validated retained callback index")
            });
            match selected {
                Ok(selected) => selected,
                Err(error) => {
                    crate::rg::graph::reclaim_failed_resources(&mut compiled.tables.registry,
                        &compiled.declarations.resources, &compiled.tables.snapshot.pool_hits, pool, backend);
                    return Err(Error::Validation(error));
                }
            }
        };
        let existing = self
            .command_records
            .keys()
            .copied()
            .collect::<BTreeSet<_>>();
        let mut transaction = FrameTransaction::default();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            validate(&compiled)
                .map_err(Error::Validation)
                .and_then(|()| {
                    self.record_compiled(
                        &compiled,
                        dependency,
                        backend,
                        &mut selected_callbacks,
                        cpu_arena.as_ref(),
                        (queue, family),
                        &mut stats,
                        &mut transaction,
                    )
                })
        }))
        .unwrap_or(Err(Error::CallbackPanic));
        stats.commands_recorded = self.recorded_commands;
        match result {
            Ok(()) => Ok(GraphExecution {
                compiled,
                transaction,
                stats,
            }),
            Err(error) => {
                let error = backend.take_error().map(Error::Validation).unwrap_or(error);
                // Unsubmitted commands are always abandoned, even if waiting for
                // earlier successful batches fails. Never release their resources
                // or replace the primary error merely because cleanup also failed.
                self.abandon_unsubmitted(&error, &existing, backend);
                if !transaction.receipts().is_empty() {
                    return Err(Error::PartialSubmission {
                        original: Box::new(error),
                        execution: Box::new(GraphExecution {
                            compiled,
                            transaction,
                            stats,
                        }),
                    });
                }
                crate::rg::graph::reclaim_failed_resources(
                    &mut compiled.tables.registry,
                    &compiled.declarations.resources,
                    &compiled.tables.snapshot.pool_hits,
                    pool,
                    backend,
                );
                Err(error)
            }
        }
    }

    /// 失败路径：本次执行开始但未提交的命令移出快照表，并交还后端；`Begin` 失败的命令同样交还。
    fn abandon_unsubmitted(
        &mut self,
        error: &Error,
        existing: &BTreeSet<u64>,
        backend: &mut impl ExecutionBackend,
    ) {
        let mut handles = self
            .command_records
            .keys()
            .copied()
            .filter(|handle| !existing.contains(handle))
            .collect::<Vec<_>>();
        for handle in &handles {
            self.command_records.remove(handle);
        }
        if let Error::Begin(failure) = error
            && !handles.contains(&failure.command.handle())
        {
            handles.push(failure.command.handle());
        }
        for handle in handles {
            backend.abandon_command(handle);
        }
    }

    /// 编译之后的资源映射、录制与提交；`receipts` 由调用方持有，失败时据此判断是否已有批次进入 GPU。
    #[allow(clippy::too_many_arguments)]
    fn record_compiled(
        &mut self,
        compiled: &crate::rg::graph::FrameInstance,
        _dependency: &mut impl crate::rg::graph::AccessDependency,
        backend: &mut impl ExecutionBackend,
        callbacks: &mut [impl FnMut(PassContext<'_>) -> bool + Send],
        cpu_arena: Option<&CpuArenaLease>,
        (_queue, _family): (recording::Queue, u32),
        stats: &mut ExecutionStats,
        transaction: &mut FrameTransaction,
    ) -> Result<(), Error> {
        *transaction = FrameTransaction::prepare(compiled).map_err(Error::Validation)?;
        let pass_names = scratch_slice(cpu_arena, compiled.declarations.passes.len(), |index| {
            &compiled.declarations.passes[index].name
        })
        .map_err(Error::Validation)?;
        if !backend.prepare_compiled_resources(compiled) {
            return Err(Error::ResourceUnavailable(usize::MAX));
        }
        let managed = backend.managed_pass_resources();
        for (id, lifetime) in compiled.info.lifetimes.iter().enumerate() {
            let mapping = &compiled.registry.resources[id];
            // 首次命令 begin 前校验资源映射；有访问或导出的资源必须存在。
            let payload = compiled
                .registry
                .backing_resources
                .get(mapping.backing_index as usize);
            if (lifetime.first_access_index != crate::rg::graph::NO_ACCESS
                || compiled
                    .declarations
                    .exports
                    .iter()
                    .any(|e| e.resource_index as usize == id))
                && payload.is_none_or(|r| crate::rg::graph::payload_projection(r).1.is_none())
            {
                return Err(Error::ResourceUnavailable(id));
            }
        }
        let mut passes = scratch_slice(cpu_arena, compiled.info.pass_access_ranges.len(), |_| {
            record::Pass {
                callback_present: true,
            }
        })
        .map_err(Error::Validation)?;
        if compiled.snapshot.batches.is_empty() {
            return Ok(());
        }
        let mut recorder = record::Recorder {
            synchronization: backend.synchronization_plan(),
            batch: 0,
            first_pass: 0,
            commands: Default::default(),
        };
        let mut batch_points = scratch_slice(cpu_arena, compiled.snapshot.batches.len(), |_| None)
            .map_err(Error::Validation)?;
        for (batch_index, batch) in compiled.snapshot.batches.iter().enumerate() {
            self.submit_queue_preludes(compiled, batch_index, backend, transaction, stats)?;
            backend
                .select_main_batch(compiled, batch_index)
                .map_err(Error::Validation)?;
            let domain = compiled.plan.queue_schedule.domains[batch_index];
            backend
                .select_queue_domain(domain)
                .map_err(Error::Validation)?;
            let family = domain.family;
            let queue = if domain.queue_index == 0 {
                recording::Queue::Primary
            } else {
                recording::Queue::Auxiliary(domain.queue_index - 1)
            };
            let handle = backend.command_handle().map_err(Error::Lifecycle)?;
            let first = self.begin(handle, family, queue, backend.begin_command(handle))?;
            recorder.commands.push(first.clone());
            recorder.batch = batch_index;
            recorder.first_pass = batch.pass_range.start;
            let mut synchronization =
                backend.batch_synchronization(batch_index, compiled.snapshot.batches.len());
            if !synchronization.valid() {
                return Err(Error::InvalidSynchronization);
            }
            for &source in &compiled.plan.queue_schedule.dependencies[batch_index] {
                // The bound program proves the exact destination stages of each
                // cross-queue dependency; same-queue order needs no semaphore.
                let stages = recorder
                    .synchronization
                    .bound()
                    .waits()
                    .iter()
                    .find(|wait| {
                        wait.batch == batch_index
                            && wait.dependency
                                == crate::rg::sync_plan::DependencyRef::Batch(source)
                    })
                    .map(|wait| wait.stages);
                if stages.is_none()
                    && compiled.plan.queue_schedule.domains[source].queue_index
                        == domain.queue_index
                {
                    continue;
                }
                let point = batch_points[source].ok_or(Error::InvalidSynchronization)?;
                let mut wait = backend.timeline_wait(point).map_err(Error::Validation)?;
                if let Some(stages) = stages {
                    wait.stage = stages;
                }
                if let Some(old) = synchronization
                    .waits
                    .iter_mut()
                    .find(|old| old.semaphore == wait.semaphore)
                {
                    old.value = old.value.max(wait.value);
                    old.stage |= wait.stage;
                } else {
                    synchronization.waits.push(wait);
                }
            }
            // The graph-completion signal joins every other queue's work, so these
            // waits have no single consumer stage and stay at ALL_COMMANDS.
            if backend.requires_graph_completion(batch_index, compiled.snapshot.batches.len()) {
                for (source, point) in batch_points[..batch_index].iter().enumerate() {
                    if compiled.plan.queue_schedule.domains[source].queue_index
                        == domain.queue_index
                    {
                        continue;
                    }
                    let wait = backend
                        .timeline_wait(point.ok_or(Error::InvalidSynchronization)?)
                        .map_err(Error::Validation)?;
                    if let Some(old) = synchronization
                        .waits
                        .iter_mut()
                        .find(|old| old.semaphore == wait.semaphore)
                    {
                        old.value = old.value.max(wait.value);
                        old.stage |= wait.stage;
                    } else {
                        synchronization.waits.push(wait);
                    }
                }
            }
            let record_start = std::time::Instant::now();
            stats.batches += 1;
            let worth_it = !matches!(
                self.parallel.strategy,
                crate::rg::parallel::ChunkStrategy::Serial
            ) && self.pgo.previous.total_ns() >= self.parallel.min_total_ns
                && (self.parallel.min_total_ns == 0 || !self.pgo.previous.is_empty());
            let chunks: smallvec::SmallVec<[_; 8]> =
                if worth_it && backend.chunk_recorder().is_some() {
                    crate::rg::parallel::plan_chunks(
                        self.parallel.strategy,
                        &pass_names[batch.pass_range.clone()],
                        &self.pgo.previous,
                        // 候选约束位：当前每个图 pass 自成完整 render pass 实例，任意边界都可切。
                        &[],
                    )
                    .into()
                } else {
                    std::iter::once(0..batch.pass_range.len()).collect()
                };
            if chunks.len() > 1 {
                stats.parallel_batches += 1;
                stats.chunks += chunks.len() as u32;
                let outcome = self.record_chunked_batch(
                    ChunkedBatch {
                        recorder: &mut recorder,
                        passes: &mut passes[batch.pass_range.clone()],
                        callbacks: &mut callbacks[batch.pass_range.clone()],
                        first_pass: batch.pass_range.start,
                        chunks: &chunks,
                        final_batch: batch.export_barriers,
                        compiled,
                        managed: managed.as_ref(),
                        pass_names: &pass_names,
                        cpu: cpu_arena,
                        family,
                        queue,
                    },
                    backend,
                )?;
                stats.count_barriers(&outcome.1);
            } else {
                // A single chunk records into the batch's first command, which
                // already holds the initial barriers.
                let new = {
                    let handle = recorder.commands.last().ok_or(Error::EmptyCommands)?.handle;
                    self.command_records
                        .get(&handle)
                        .ok_or(Error::MissingCommand(handle))?
                        .clone()
                };
                let handle = new.handle();
                let start = batch.pass_range.start;
                let owners = std::cell::RefCell::new(
                    super::owner_storage::ResourceOwners::new(backend.owner_storage_pool())
                        .map_err(Error::Validation)?,
                );
                let errors = std::cell::RefCell::new(None);
                let mut durations = scratch_slice(cpu_arena, batch.pass_range.len(), |_| None)
                    .map_err(Error::Validation)?;
                let mut duration_count = 0;
                let outcome = self.record(
                    &mut recorder,
                    &mut passes[batch.pass_range.clone()],
                    batch.export_barriers,
                    &new,
                    |action| match action {
                        record::RecordAction::Pass(offset) => {
                            if !backend.flush_barriers(handle) {
                                return false;
                            }
                            let pass_index = start + offset;
                            if let Some(resources) = &managed
                                && let Err(error) = resources.retain_pass_resources(
                                    &compiled.declarations.passes[pass_index],
                                    &mut owners.borrow_mut(),
                                )
                            {
                                errors.borrow_mut().get_or_insert(error);
                                return false;
                            }
                            let gpu = backend.reserve_pass_timestamp(handle, pass_index);
                            if let Some(pair) = gpu {
                                // SAFETY: this pair was reserved for this exclusively recording command.
                                unsafe { backend.write_pass_timestamp(handle, pair, false) };
                            }
                            let timer = std::time::Instant::now();
                            let ok = callbacks[pass_index](PassContext {
                                pass_index,
                                cpu: cpu_arena,
                                command: &new,
                                compiled,
                                managed: managed.as_deref(),
                                owners: &owners,
                                errors: &errors,
                            });
                            if let Some(pair) = gpu {
                                // SAFETY: the same command and reserved pair remain exclusively owned.
                                unsafe { backend.write_pass_timestamp(handle, pair, true) };
                            }
                            durations[duration_count] = Some(timer.elapsed().as_nanos() as u64);
                            duration_count += 1;
                            ok && errors.borrow().is_none()
                        }
                        record::RecordAction::End(command) => {
                            backend.end_command(vk::CommandBuffer::from_raw(command))
                        }
                        record::RecordAction::Transition {
                            command,
                            transition,
                        } => {
                            let backing = compiled.registry.resources[transition.resource as usize]
                                .backing_index;
                            let token = crate::rg::graph::payload_projection(
                                &compiled.registry.backing_resources[backing as usize],
                            )
                            .1
                            .unwrap();
                            backend.transition(command, token, &transition)
                        }
                    },
                );
                if !backend.retain_command_owners(handle, &mut owners.into_inner()) {
                    return Err(Error::MissingCommand(handle));
                }
                self.pgo
                    .current
                    .extend(durations[..duration_count].iter().enumerate().map(
                        |(offset, duration)| {
                            PassPgoDuration::new(
                                PassPgoNameId::from_shared(std::sync::Arc::clone(
                                    pass_names[start + offset],
                                )),
                                duration.expect("recorded pass duration"),
                            )
                        },
                    ));
                if let Some(error) = errors.into_inner() {
                    return Err(Error::Validation(error));
                }
                let outcome = outcome?;
                stats.count_barriers(&outcome);
            };
            stats.record_ns += record_start.elapsed().as_nanos() as u64;
            // 各批次之间提交；保留批次中最后一条开放命令，并恢复 end 后的提交数组构造。
            let ended = recorder
                .commands
                .last()
                .map(|c| c.handle)
                .ok_or(Error::EmptyCommands)?;
            let submit_start = std::time::Instant::now();
            let end_ok = backend.end_command(vk::CommandBuffer::from_raw(ended));
            if !end_ok {
                return Err(Error::EndFailed);
            }
            let commands = std::mem::take(&mut recorder.commands);
            let handles = commands
                .iter()
                .map(|c| vk::CommandBuffer::from_raw(c.handle))
                .collect();
            let state_commands = commands
                .iter()
                .map(|command| command.handle)
                .collect::<smallvec::SmallVec<[_; 8]>>();
            backend
                .prepare_batch_submission(&state_commands, batch_index)
                .map_err(Error::Validation)?;
            let prepared = submit::PreparedSubmit {
                commands: handles,
                wait_semaphores: synchronization.waits.iter().map(|w| w.semaphore).collect(),
                wait_stages: synchronization.waits.iter().map(|w| w.stage).collect(),
                wait_values: synchronization.waits.iter().map(|w| w.value).collect(),
                signal_semaphores: synchronization
                    .signals
                    .iter()
                    .map(|s| s.semaphore)
                    .collect(),
                signal_values: synchronization.signals.iter().map(|s| s.value).collect(),
            };
            let raw_queue = match queue {
                recording::Queue::Primary => self.queues[0],
                recording::Queue::Auxiliary(index) => self.queues[index as usize + 1],
            };
            // Reserve before entering the driver. Successful submission publishes only
            // prevalidated commands and a completion point; this path cannot allocate.
            self.pending_commands
                .try_reserve(commands.len())
                .map_err(|_| Error::Validation(crate::Error::OutOfHostMemory))?;
            for command in &commands {
                if !self.command_records.contains_key(&command.handle) {
                    return Err(Error::MissingCommand(command.handle));
                }
            }
            transaction.reserve_receipt().map_err(Error::Validation)?;
            let point = backend
                .submit(raw_queue, &[prepared])
                .map_err(Error::Validation)?;
            for recorded in commands {
                let mut command = self
                    .command_records
                    .remove(&recorded.handle)
                    .expect("validated before submission");
                command.state = recording::CommandState::Submitted;
                self.pending_commands.push((point, command));
            }
            stats.submit_ns += submit_start.elapsed().as_nanos() as u64;
            transaction.publish(point);
            batch_points[batch_index] = Some(point);
        }
        Ok(())
    }
    fn submit_queue_preludes(
        &mut self,
        compiled: &crate::rg::graph::FrameInstance,
        batch: usize,
        backend: &mut impl ExecutionBackend,
        transaction: &mut FrameTransaction,
        stats: &mut ExecutionStats,
    ) -> Result<(), Error> {
        let limit = compiled
            .declarations
            .resources
            .len()
            .saturating_mul(4)
            .saturating_add(1);
        for _ in 0..limit {
            let Some(prelude) = backend
                .next_queue_prelude(compiled, batch)
                .map_err(Error::Validation)?
            else {
                return Ok(());
            };
            backend
                .select_queue_domain(prelude.domain)
                .map_err(Error::Validation)?;
            let queue = if prelude.domain.queue_index == 0 {
                recording::Queue::Primary
            } else {
                recording::Queue::Auxiliary(prelude.domain.queue_index - 1)
            };
            let handle = backend.command_handle().map_err(Error::Lifecycle)?;
            let command = self.begin(
                handle,
                prelude.domain.family,
                queue,
                backend.begin_command(handle),
            )?;
            backend
                .record_queue_prelude(handle, prelude)
                .map_err(Error::Validation)?;
            if !backend.end_command(vk::CommandBuffer::from_raw(handle)) {
                return Err(Error::EndFailed);
            }
            self.command_records
                .get_mut(&handle)
                .ok_or(Error::MissingCommand(handle))?
                .state = recording::CommandState::Recorded;
            self.pending_commands
                .try_reserve(1)
                .map_err(|_| Error::Validation(crate::Error::OutOfHostMemory))?;
            transaction.reserve_receipt().map_err(Error::Validation)?;
            let prepared = submit::PreparedSubmit {
                commands: [vk::CommandBuffer::from_raw(handle)].into_iter().collect(),
                ..Default::default()
            };
            let raw_queue = *self
                .queues
                .get(prelude.domain.queue_index as usize)
                .ok_or(Error::InvalidSynchronization)?;
            let start = std::time::Instant::now();
            let point = backend
                .submit(raw_queue, &[prepared])
                .map_err(Error::Validation)?;
            let mut snapshot = self
                .command_records
                .remove(&command.handle())
                .expect("prevalidated ownership command");
            snapshot.state = recording::CommandState::Submitted;
            self.pending_commands.push((point, snapshot));
            transaction.publish_auxiliary(point);
            stats.submit_ns += start.elapsed().as_nanos() as u64;
        }
        Err(Error::Validation(crate::Error::invalid_usage(
            "ownership prelude did not converge",
        )))
    }
    pub fn new(primary: vk::Queue, auxiliary: Vec<vk::Queue>, family_count: u32) -> Self {
        let auxiliary_count = auxiliary.len() as u32;
        Self {
            subgraph_cache: super::subgraph::SubgraphPlanCache::default(),
            plan_cache: super::cache::BoundedCache::new(128, 16 * 1024 * 1024),
            plan_cache_enabled: true,
            recording_ids: recording::RecordingCounters::new(auxiliary_count),
            pending_commands: Vec::new(),
            completed_commands: Vec::new(),
            queues: std::iter::once(primary).chain(auxiliary).collect(),
            command_records: BTreeMap::new(),
            recorded_commands: 0,
            pool_family_count: family_count,
            parallel: Default::default(),
            pgo: Default::default(),
        }
    }

    pub fn set_plan_cache_enabled(&mut self, enabled: bool) {
        self.plan_cache_enabled = enabled;
        if !enabled {
            self.plan_cache
                .clear_with_reason(super::cache::CacheReason::Disabled);
        }
    }
    pub fn set_subgraph_cache_enabled(&mut self, enabled: bool) {
        self.subgraph_cache.set_enabled(enabled);
    }
    pub fn subgraph_cache_stats(&self) -> super::subgraph::SubgraphCacheStats {
        self.subgraph_cache.stats()
    }
    pub fn plan_cache_stats(&self) -> super::cache::CacheStats {
        self.plan_cache.stats()
    }
    pub fn plan_cache_bytes(&self) -> usize {
        self.plan_cache.bytes()
    }

    pub fn begin(
        &mut self,
        handle: u64,
        family: u32,
        queue: recording::Queue,
        backend: Result<(), recording::Failure>,
    ) -> Result<recording::Command, Error> {
        let mut command = recording::Command::new(handle, family, queue);
        if let Err(reason) = backend {
            return Err(Error::Begin(recording::BeginFailure { reason, command }));
        }
        let recording_id = self.recording_ids.next_id(queue).ok_or_else(|| {
            Error::Begin(recording::BeginFailure {
                reason: recording::Failure::CounterBounds,
                command: command.clone(),
            })
        })?;
        command.state = recording::CommandState::Recording;
        command.recording_id = recording_id;
        self.recorded_commands = self.recorded_commands.saturating_add(1);
        self.command_records.insert(handle, command.clone());
        Ok(command)
    }

    /// Record graph commands with CPU recording identities and actual resource owners.
    pub fn record(
        &mut self,
        recorder: &mut record::Recorder,
        passes: &mut [record::Pass],
        final_batch: bool,
        command: &recording::Command,
        boundary: impl FnMut(record::RecordAction) -> bool,
    ) -> Result<record::Outcome, Error> {
        let outcome = recorder.record_batch(passes, final_batch, command.clone(), boundary);
        self.finish_record(recorder, outcome)
    }

    /// Propagate callback failure and synchronize the typed CPU command states.
    fn finish_record(
        &mut self,
        recorder: &record::Recorder,
        outcome: record::Outcome,
    ) -> Result<record::Outcome, Error> {
        if outcome.panic {
            return Err(Error::Record(outcome));
        }
        for recorded in &recorder.commands {
            let snapshot = self
                .command_records
                .get_mut(&recorded.handle)
                .ok_or(Error::MissingCommand(recorded.handle))?;
            snapshot.state = recorded.state;
        }
        Ok(outcome)
    }

    /// 并行录制一个 batch：串行开命令并规划屏障 → 任务池按块并行录制 → 串行收尾。
    /// 与原 `record_batch` 同形：串行段构造屏障，每块一条命令缓冲，块内逐 pass 发屏障、调回调、计时。
    fn record_chunked_batch<C>(
        &mut self,
        batch: ChunkedBatch<'_, C>,
        backend: &mut impl ExecutionBackend,
    ) -> Result<(recording::Command, record::Outcome), Error>
    where
        C: FnMut(PassContext<'_>) -> bool + Send,
    {
        let ChunkedBatch {
            recorder,
            passes,
            callbacks,
            first_pass,
            chunks,
            final_batch,
            compiled,
            managed,
            pass_names,
            cpu,
            family,
            queue,
        } = batch;
        if chunks.iter().map(|r| r.len()).sum::<usize>() != passes.len()
            || chunks.iter().any(|r| r.is_empty())
        {
            return Err(Error::ChunkPlan);
        }
        let token_of = |resource: usize| {
            let backing = compiled.registry.resources[resource].backing_index;
            crate::rg::graph::payload_projection(
                &compiled.registry.backing_resources[backing as usize],
            )
            .1
            .unwrap()
        };
        // 串行：每块一条命令缓冲（每块一个独立命令池，Vulkan 要求命令池外部同步）。
        let mut begun = Vec::with_capacity(chunks.len());
        for index in 0..chunks.len() {
            let handle = backend
                .chunk_command_handle(index)
                .map_err(Error::Lifecycle)?;
            let command = self.begin(handle, family, queue, backend.begin_command(handle))?;
            begun.push(command);
        }
        let record_commands = begun.to_vec();
        let mut chunk_of_pass = vec![0usize; passes.len()];
        for (index, range) in chunks.iter().enumerate() {
            for pass in range.clone() {
                chunk_of_pass[pass] = index;
            }
        }
        // 串行：初始屏障、资源跟踪、pass 前屏障规划。
        let (plan, mut outcome) =
            recorder.plan_chunked_batch(passes, &record_commands, &chunk_of_pass, |action| {
                match action {
                    record::RecordAction::Transition {
                        command,
                        transition,
                    } => backend.transition(
                        command,
                        token_of(transition.resource as usize),
                        &transition,
                    ),
                    _ => true,
                }
            });
        if outcome.panic {
            self.finish_record(recorder, outcome)?;
            return Err(Error::ChunkPlan);
        }
        let transitions = plan
            .iter()
            .map(|list| {
                list.iter()
                    .map(|t| ChunkTransition {
                        token: token_of(t.resource as usize),
                        transition: *t,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        // 并行：块间互不重叠，各自独占命令缓冲、回调切片与 owner 列表。
        let mut work = Vec::with_capacity(chunks.len());
        let mut durations =
            scratch_slice(cpu, callbacks.len(), |_| None).map_err(Error::Validation)?;
        let mut gpu_timings =
            scratch_slice(cpu, callbacks.len(), |_| None).map_err(Error::Validation)?;
        for (range, command) in chunks.iter().zip(&begun) {
            for index in range.clone() {
                gpu_timings[index] =
                    backend.reserve_pass_timestamp(command.handle(), first_pass + index);
            }
        }
        let mut remaining_durations = &mut *durations;
        let mut rest = callbacks;
        for (range, command) in chunks.iter().zip(&begun) {
            let (head, tail) = std::mem::take(&mut rest).split_at_mut(range.len());
            rest = tail;
            let (duration_slice, duration_tail) =
                std::mem::take(&mut remaining_durations).split_at_mut(range.len());
            remaining_durations = duration_tail;
            work.push(ChunkWork {
                command,
                start: range.start,
                callbacks: head,
                durations: duration_slice,
                transitions: &transitions[range.clone()],
                gpu_timings: &gpu_timings[range.clone()],
            });
        }
        let results = {
            let view = backend.chunk_recorder().ok_or(Error::ChunkPlan)?;
            let view = &*view;
            self.parallel.runner.map_chunks(&mut work, |_, work| {
                let owners =
                    match super::owner_storage::ResourceOwners::new(view.owner_storage_pool()) {
                        Ok(owners) => std::cell::RefCell::new(owners),
                        Err(error) => {
                            return ChunkResult {
                                ok: false,
                                error: Some(error),
                                recorded_durations: 0,
                                owners: super::owner_storage::ResourceOwners::empty(),
                            };
                        }
                    };
                let errors = std::cell::RefCell::new(None);
                let cpu = match view
                    .cpu_arena_pool()
                    .map(CpuArenaPool::checkout)
                    .transpose()
                {
                    Ok(cpu) => cpu,
                    Err(error) => {
                        return ChunkResult {
                            ok: false,
                            error: Some(error),
                            recorded_durations: 0,
                            owners: super::owner_storage::ResourceOwners::empty(),
                        };
                    }
                };
                let mut duration_count = 0;
                let mut ok = true;
                for (offset, callback) in work.callbacks.iter_mut().enumerate() {
                    if let Err(error) = view.record_transitions(
                        work.command.handle(),
                        &work.transitions[offset],
                        &mut owners.borrow_mut(),
                    ) {
                        errors.borrow_mut().get_or_insert(error);
                        ok = false;
                        break;
                    }
                    let pass_index = first_pass + work.start + offset;
                    if let Some(resources) = managed
                        && let Err(error) = resources.retain_pass_resources(
                            &compiled.declarations.passes[pass_index],
                            &mut owners.borrow_mut(),
                        )
                    {
                        errors.borrow_mut().get_or_insert(error);
                        ok = false;
                        break;
                    }
                    let gpu = work.gpu_timings[offset];
                    if let Some(pair) = gpu {
                        // SAFETY: each worker owns its command and its disjoint reserved query pairs.
                        unsafe { view.write_pass_timestamp(work.command.handle(), pair, false) };
                    }
                    let timer = std::time::Instant::now();
                    let good = callback(PassContext {
                        pass_index,
                        cpu: cpu.as_ref(),
                        command: work.command,
                        compiled,
                        managed: managed.map(|m| &**m),
                        owners: &owners,
                        errors: &errors,
                    });
                    if let Some(pair) = gpu {
                        // SAFETY: the callback has returned on the same exclusive worker command.
                        unsafe { view.write_pass_timestamp(work.command.handle(), pair, true) };
                    }
                    work.durations[duration_count] = Some(timer.elapsed().as_nanos() as u64);
                    duration_count += 1;
                    if !good || errors.borrow().is_some() {
                        ok = false;
                        break;
                    }
                }
                ChunkResult {
                    ok,
                    error: errors.into_inner(),
                    recorded_durations: duration_count,
                    owners: owners.into_inner(),
                }
            })
        };
        // 串行：按块顺序合并耗时与 owner。
        let mut failed = false;
        let mut callback_error = None;
        for ((command, work), mut result) in begun.iter().zip(&work).zip(results) {
            self.pgo.current.extend(
                work.durations[..result.recorded_durations]
                    .iter()
                    .enumerate()
                    .map(|(offset, duration)| {
                        PassPgoDuration::new(
                            PassPgoNameId::from_shared(std::sync::Arc::clone(
                                pass_names[first_pass + work.start + offset],
                            )),
                            duration.expect("recorded pass duration"),
                        )
                    }),
            );
            if !backend.retain_command_owners(command.handle(), &mut result.owners) {
                return Err(Error::MissingCommand(command.handle()));
            }
            failed |= !result.ok;
            if let Some(error) = result.error {
                callback_error.get_or_insert(error);
            }
        }
        if failed {
            outcome.panic = true;
        } else {
            recorder.finish_chunked_batch(&mut outcome, record_commands, final_batch, |action| {
                match action {
                    record::RecordAction::End(command) => {
                        backend.end_command(vk::CommandBuffer::from_raw(command))
                    }
                    record::RecordAction::Transition {
                        command,
                        transition,
                    } => backend.transition(
                        command,
                        token_of(transition.resource as usize),
                        &transition,
                    ),
                    record::RecordAction::Pass(_) => true,
                }
            });
        }
        let outcome = self.finish_record(recorder, outcome);
        if let Some(error) = callback_error {
            return Err(Error::Validation(error));
        }
        let outcome = outcome?;
        let last = begun.last().cloned().ok_or(Error::ChunkPlan)?;
        Ok((last, outcome))
    }

    /// Called only after the backend has observed this queue completion.
    fn complete(&mut self, point: crate::sync::SyncPoint) {
        let mut index = 0;
        while index < self.pending_commands.len() {
            let (required, _) = &self.pending_commands[index];
            if required.queue == point.queue && required.value <= point.value {
                let (_, command) = self.pending_commands.swap_remove(index);
                self.completed_commands.push(command);
            } else {
                index += 1;
            }
        }
    }
    pub(crate) fn observe_completion(&mut self, point: crate::sync::SyncPoint) {
        self.complete(point);
    }
    pub(crate) fn recycle(&mut self) -> Option<recording::Command> {
        self.completed_commands.pop()
    }
    pub fn pending_command_count(&self) -> usize {
        self.pending_commands.len()
    }
    pub fn recording_command_count(&self) -> usize {
        self.command_records.len()
    }
    pub fn completed_command_count(&self) -> usize {
        self.completed_commands.len()
    }
}

/// Owned full-content PGO identity. Cloning a declaration's name shares the
/// existing allocation, and names disappear with the last plan or observation.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PassPgoNameId(std::sync::Arc<str>);
impl PassPgoNameId {
    pub fn from_shared(name: std::sync::Arc<str>) -> Self {
        Self(name)
    }
    /// Cold-path convenience; production recording shares the declaration Arc.
    pub fn from_static(name: &'static str) -> Self {
        Self(name.into())
    }
    pub fn from_name(name: &str) -> Self {
        Self(name.into())
    }
}
impl AsRef<str> for PassPgoNameId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl std::borrow::Borrow<str> for PassPgoNameId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
#[derive(Clone, Debug)]
pub struct PassPgoDuration {
    pub name: PassPgoNameId,
    pub duration_ns: u64,
}
impl PassPgoDuration {
    pub fn new(name: PassPgoNameId, duration_ns: u64) -> Self {
        Self { name, duration_ns }
    }
}
/// One frame's observations. Repeated names keep their final duration; totals
/// include every recorded invocation. Reset releases names while reusing backing.
#[derive(Clone, Debug, Default)]
pub struct PassPgoStats {
    counts: std::collections::HashMap<PassPgoNameId, u64>,
    durations: std::collections::HashMap<PassPgoNameId, u64>,
    total_ns: u64,
    max_ns: u64,
}
impl PassPgoStats {
    pub fn reset(&mut self) {
        self.counts.clear();
        self.durations.clear();
        self.total_ns = 0;
        self.max_ns = 0;
    }
    pub fn extend(&mut self, durations: impl IntoIterator<Item = PassPgoDuration>) {
        for duration in durations {
            self.record(duration);
        }
    }
    pub fn duration_ns(&self, id: impl AsRef<str>) -> u64 {
        self.get(id).unwrap_or(0)
    }
    pub fn get(&self, id: impl AsRef<str>) -> Option<u64> {
        self.durations.get(id.as_ref()).copied()
    }
    pub fn call_count(&self, id: impl AsRef<str>) -> u64 {
        self.counts.get(id.as_ref()).copied().unwrap_or(0)
    }
    pub fn is_empty(&self) -> bool {
        self.durations.is_empty()
    }
    pub fn total_ns(&self) -> u64 {
        self.total_ns
    }
    pub fn max_ns(&self) -> u64 {
        self.max_ns
    }
    pub fn record(&mut self, duration: PassPgoDuration) {
        *self.counts.entry(duration.name.clone()).or_default() += 1;
        self.durations.insert(duration.name, duration.duration_ns);
        self.total_ns = self.total_ns.wrapping_add(duration.duration_ns);
        self.max_ns = self.max_ns.max(duration.duration_ns);
    }
}

/// Chunk planning reads the preceding frame. At frame end the buffers swap
/// and the new current observations are cleared, releasing obsolete names.
#[derive(Clone, Debug, Default)]
pub struct PassPgoFrames {
    pub previous: PassPgoStats,
    pub current: PassPgoStats,
}
impl PassPgoFrames {
    pub fn end_frame(&mut self) {
        std::mem::swap(&mut self.previous, &mut self.current);
        self.current.reset();
    }
}

/// 执行后按逻辑索引提取的导出资源；物理 token 由后端资源表持有。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExportedResource {
    pub logical: u32,
    pub token: u64,
}
impl ExportedResource {
    pub fn new(logical: u32, token: u64) -> Self {
        Self { logical, token }
    }
}

/// 一次执行导出的资源容器；`take_exported` 一次取全部，`take_resource` 按逻辑索引取单个。
#[derive(Clone, Debug, Default)]
pub struct ExportedRenderGraphResources {
    resources: Vec<ExportedResource>,
}
impl ExportedRenderGraphResources {
    pub fn with_resources(resources: impl IntoIterator<Item = ExportedResource>) -> Self {
        Self {
            resources: resources.into_iter().collect(),
        }
    }
    pub fn take_exported(&mut self) -> Vec<ExportedResource> {
        std::mem::take(&mut self.resources)
    }
    pub fn take_resource(&mut self, logical: u32) -> Option<ExportedResource> {
        let index = self.resources.iter().position(|r| r.logical == logical)?;
        Some(self.resources.swap_remove(index))
    }
    pub fn len(&self) -> usize {
        self.resources.len()
    }
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }
}

/// 已编译并可退役的执行结果；retire 触发退役回调（资源池回收等）。
pub struct ExecutedRenderGraph {
    pub execution: GraphExecution,
    pub retire: Vec<crate::rg::graph::RetireCallback>,
}
impl ExecutedRenderGraph {
    pub fn new(execution: GraphExecution, retire: Vec<crate::rg::graph::RetireCallback>) -> Self {
        Self { execution, retire }
    }
    pub fn retire(&mut self) {
        for callback in std::mem::take(&mut self.retire) {
            callback();
        }
    }
    pub fn take_retire(&mut self) -> Vec<crate::rg::graph::RetireCallback> {
        std::mem::take(&mut self.retire)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}
