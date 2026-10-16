// 候选实现迁移来源：reconstruction/render/execution.rs.
// 这是行为重建，不是原作者源码。
//! 将已恢复的私有 CPU 契约连成主链；Vulkan 创建、命令结束与提交交给后端。此入口为候选整合，不声称原函数 ABI。
use crate::raw::managed::lifecycle as sequence;
use crate::raw::submit;
use crate::rg::recorder as record;
use ash::vk::{self, Handle};
use std::collections::{BTreeMap, BTreeSet};

pub struct ExecutionChain {
    pub counters: sequence::DeviceCounters,
    pub lifecycle: sequence::SequenceRuntime,
    queues: Vec<vk::Queue>,
    command_snapshots: BTreeMap<u64, sequence::Command>,
    recorded_sequences: Vec<u64>,
    graph_queue_domain: Option<sequence::Queue>,
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
    compiled: &'a crate::rg::graph::CompiledGraph,
    logical_handles: &'a [Option<sequence::ResourceHandle>],
    managed: Option<&'a std::sync::Arc<crate::rg::pass_bindings::ManagedPassResources>>,
    pass_names: &'a [PassPgoNameId],
    family: u32,
    queue: sequence::Queue,
}
/// 单块的独占工作单元。
struct ChunkWork<'a, C> {
    command: &'a sequence::Command,
    start: usize,
    callbacks: &'a mut [C],
    transitions: &'a [Vec<ChunkTransition>],
}
struct ChunkResult {
    ok: bool,
    durations: Vec<PassPgoDuration>,
    owners: Vec<crate::raw::binding::ResourceOwner>,
}

#[derive(Debug)]
pub enum Error {
    /// 并行录制的分块或规划无效，或后端未提供并行视图。
    ChunkPlan,
    CallbackPanic,
    Begin(sequence::BeginFailure),
    Lifecycle(sequence::Failure),
    Record(record::Outcome),
    MissingCommand(u64),
    EmptyCommands,
    InvalidLastState,
    EndFailed,
    Compile(crate::Error),
    ResourceUnavailable(usize),
    CallbackCount,
    ManagedResourceUnresolved(usize),
    QueueUnavailable(sequence::Queue),
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
/// 外部生产的批次同步与图命令分别生成，避免调用者手填内部 Command/sequence。
#[derive(Clone, Debug, Default)]
pub struct GraphBatchSync {
    pub waits: Vec<GraphTimelineWait>,
    pub signals: Vec<GraphTimelineSignal>,
}
impl GraphBatchSync {
    fn valid(&self) -> bool {
        let waits = self
            .waits
            .iter()
            .map(|w| w.semaphore.as_raw())
            .collect::<BTreeSet<_>>();
        let signals = self
            .signals
            .iter()
            .map(|s| s.semaphore.as_raw())
            .collect::<BTreeSet<_>>();
        waits.len() == self.waits.len()
            && signals.len() == self.signals.len()
            && self
                .waits
                .iter()
                .all(|w| w.semaphore != vk::Semaphore::null() && !w.stage.is_empty())
            && self
                .signals
                .iter()
                .all(|s| s.semaphore != vk::Semaphore::null())
    }
}

/// 并行块内发出的一条 pass 前屏障；`token` 为物理资源 token。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChunkTransition {
    pub token: u64,
    pub from: u8,
    pub to: u8,
}

/// 后端在并行段暴露的只读录制视图：多个块同时调用，每次只写入调用方独占的命令缓冲。
pub trait ChunkRecorder: Sync {
    /// 把一个 pass 的全部屏障合成一次屏障命令写入 `command`；返回 false 表示资源或访问码无效。
    fn record_transitions(&self, command: u64, transitions: &[ChunkTransition]) -> bool;
}

/// 一次图执行的阶段耗时与分块统计（性能追踪用）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ExecutionStats {
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

pub trait ExecutionBackend: crate::rg::graph::ResourceBackend {
    /// 兼容录制器 bool/生命周期接口期间，取出设备调用的原始错误，避免丢失错误类别。
    fn take_error(&mut self) -> Option<crate::Error> {
        None
    }
    /// 包括 Pool 命中与导入资源；运行回调前重新发布真实物理映射。
    fn prepare_compiled_resources(&mut self, _compiled: &crate::rg::graph::CompiledGraph) -> bool {
        true
    }
    /// 物理对象与 generation 由本次后端解析；未提供时 callback 明确看到 None。
    fn managed_pass_resources(
        &self,
    ) -> Option<std::sync::Arc<crate::rg::pass_bindings::ManagedPassResources>> {
        None
    }
    fn command_started(&mut self, _command: &sequence::Command) -> Result<(), sequence::Failure> {
        Ok(())
    }
    fn retain_command_owners(
        &mut self,
        _handle: u64,
        owners: Vec<crate::raw::binding::ResourceOwner>,
    ) -> bool {
        owners.is_empty()
    }
    fn collect_managed_waste(&mut self) -> bool {
        true
    }
    fn graph_queue(&self) -> (sequence::Queue, u32) {
        (sequence::Queue::Primary, 0)
    }
    fn batch_synchronization(&self, _index: usize, _total: usize) -> GraphBatchSync {
        GraphBatchSync::default()
    }
    /// 物理注册表使用 u64 token。托管对象分配与 generation 须由后端明确提供；无法解析时返回 None。
    fn managed_resource(
        &mut self,
        token: u64,
        lifecycle: &mut sequence::SequenceRuntime,
    ) -> Option<sequence::ResourceHandle>;
    /// 设备和命令创建由后端提供；真实实现见 rg/vulkan.rs。
    fn command_handle(&mut self) -> Result<u64, sequence::Failure>;
    /// 并行录制时第 `chunk` 块使用的命令；真实后端须保证不同块来自不同命令池。
    fn chunk_command_handle(&mut self, _chunk: usize) -> Result<u64, sequence::Failure> {
        self.command_handle()
    }
    fn begin_command(&mut self, handle: u64) -> Result<(), sequence::Failure>;
    fn end_command(&mut self, handle: vk::CommandBuffer) -> bool;
    /// CPU 后端可只记录投影；实际后端必须发出 Vulkan 屏障。
    fn transition(&mut self, _command: u64, _token: u64, _from: u8, _to: u8) -> bool {
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
    fn wait_submission(&self, _receipt: sequence::SubmissionReceipt) -> crate::Result<()> {
        Ok(())
    }
}

pub struct GraphExecution {
    pub compiled: crate::rg::graph::CompiledGraph,
    pub receipts: Vec<sequence::SubmissionReceipt>,
    pub resources: Vec<sequence::ResourceHandle>,
    pub command_sequences: Vec<u64>,
    pub stats: ExecutionStats,
}

/// 本次录制的命令和编译图；绑定消费者不能另外分配序号或重新构造资源映射。
pub struct PassContext<'a> {
    pub pass_index: usize,
    pub command: &'a sequence::Command,
    pub compiled: &'a crate::rg::graph::CompiledGraph,
    pub logical_resources: &'a [Option<sequence::ResourceHandle>],
    pub managed: Option<&'a crate::rg::pass_bindings::ManagedPassResources>,
    /// 由 recorder 收集并交给同一命令拥有者，callback 返回后仍需保活。
    pub owners: &'a std::cell::RefCell<Vec<crate::raw::binding::ResourceOwner>>,
}
impl PassContext<'_> {
    pub fn command_state(&self) -> crate::raw::managed::tracking::CommandState {
        crate::raw::managed::tracking::CommandState {
            tag: self.command.state(),
            sequence: self.command.sequence().0,
        }
    }
    pub fn keep_alive<T: Send + Sync + 'static>(&self, owner: std::sync::Arc<T>) {
        self.owners.borrow_mut().push(owner);
    }
}

fn access(
    record: &crate::rg::graph::ResourceAccessRecord,
    write: bool,
) -> crate::raw::barrier::Access {
    crate::raw::barrier::Access {
        code: record.access_code,
        group: record.synchronization_group,
        write,
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

    /// 在 compiled→record 的同一调用中提供真实 CommandState 和 managed 注册表。
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
        validate: impl FnOnce(&crate::rg::graph::CompiledGraph) -> crate::Result<()>,
    ) -> Result<GraphExecution, Error> {
        let result =
            self.execute_graph_inner(graph, pool, dependency, backend, callbacks, validate);
        // 原 end_frame：本帧耗时成为下一帧的分块依据。
        self.pgo.end_frame();
        result
    }

    fn execute_graph_inner(
        &mut self,
        graph: crate::rg::graph::GraphDeclaration,
        pool: &mut crate::rg::graph::Pool,
        dependency: &mut impl crate::rg::graph::AccessDependency,
        backend: &mut impl ExecutionBackend,
        callbacks: &mut [impl FnMut(PassContext<'_>) -> bool + Send],
        validate: impl FnOnce(&crate::rg::graph::CompiledGraph) -> crate::Result<()>,
    ) -> Result<GraphExecution, Error> {
        self.recorded_sequences.clear();
        let mut stats = ExecutionStats::default();
        if callbacks.len() != graph.passes.len() {
            return Err(Error::CallbackCount);
        }
        let (queue, family) = backend.graph_queue();
        if family >= self.pool_family_count {
            return Err(Error::Lifecycle(sequence::Failure::PoolFamilyBounds));
        }
        if matches!(queue, sequence::Queue::Auxiliary(i) if i as usize + 1 >= self.queues.len()) {
            return Err(Error::QueueUnavailable(queue));
        }
        // 已恢复的 resource bucket key 仍为全局 u64。跨队列同号可能冲突，不能伪造域映射。
        if self
            .graph_queue_domain
            .is_some_and(|previous| previous != queue)
        {
            return Err(Error::UnsupportedQueueDomainSwitch);
        }
        self.graph_queue_domain = Some(queue);
        let compile_start = std::time::Instant::now();
        // population 失败时 compile_graph 已归还本次取出/新建的资源。
        let mut compiled = crate::rg::graph::compile_graph(graph, pool, backend, dependency)
            .map_err(Error::Compile)?;
        stats.compile_ns = compile_start.elapsed().as_nanos() as u64;
        let existing = self
            .command_snapshots
            .keys()
            .copied()
            .collect::<BTreeSet<_>>();
        let mut receipts = Vec::new();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            validate(&compiled)
                .map_err(Error::Validation)
                .and_then(|()| {
                    self.record_compiled(
                        &compiled,
                        dependency,
                        backend,
                        callbacks,
                        (queue, family),
                        &mut stats,
                        &mut receipts,
                    )
                })
        }))
        .unwrap_or(Err(Error::CallbackPanic));
        match result {
            Ok(resources) => Ok(GraphExecution {
                compiled,
                receipts,
                resources,
                command_sequences: self.recorded_sequences.clone(),
                stats,
            }),
            Err(error) => {
                let error = backend.take_error().map(Error::Validation).unwrap_or(error);
                // 已经成功提交的批次先完成，再回滚本次资源；未提交命令直接取消。
                for receipt in &receipts {
                    backend
                        .wait_submission(*receipt)
                        .map_err(Error::Validation)?;
                    self.complete(receipt.queue, receipt.last_timeline)?;
                }
                self.abandon_unsubmitted(&error, &existing, backend);
                crate::rg::graph::reclaim_failed_resources(
                    &mut compiled.registry,
                    &compiled.declarations.resources,
                    &compiled.snapshot.pool_hits,
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
            .command_snapshots
            .keys()
            .copied()
            .filter(|handle| !existing.contains(handle))
            .collect::<Vec<_>>();
        for handle in &handles {
            if let Some(command) = self.command_snapshots.remove(handle) {
                let _ = self.lifecycle.abandon(&command);
            }
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
        compiled: &crate::rg::graph::CompiledGraph,
        dependency: &mut impl crate::rg::graph::AccessDependency,
        backend: &mut impl ExecutionBackend,
        callbacks: &mut [impl FnMut(PassContext<'_>) -> bool + Send],
        (queue, family): (sequence::Queue, u32),
        stats: &mut ExecutionStats,
        receipts: &mut Vec<sequence::SubmissionReceipt>,
    ) -> Result<Vec<sequence::ResourceHandle>, Error> {
        let pass_names = compiled
            .declarations
            .passes
            .iter()
            .map(|pass| PassPgoNameId::from_name(&pass.name))
            .collect::<Vec<_>>();
        if !backend.prepare_compiled_resources(compiled) {
            return Err(Error::ResourceUnavailable(usize::MAX));
        }
        let managed = backend.managed_pass_resources();
        let synchronization = (0..compiled.snapshot.batches.len())
            .map(|i| backend.batch_synchronization(i, compiled.snapshot.batches.len()))
            .collect::<Vec<_>>();
        if synchronization.iter().any(|s| !s.valid()) {
            return Err(Error::InvalidSynchronization);
        }
        // 读写分类按访问码查一次，闭包里不再借用 dependency。
        let writes: Vec<bool> = compiled
            .info
            .accesses
            .iter()
            .map(|record| dependency.is_write(record.access_code))
            .collect();
        let resolve_access = |index: u32| {
            (index != crate::rg::graph::NO_ACCESS).then(|| {
                access(
                    &compiled.info.accesses[index as usize],
                    writes[index as usize],
                )
            })
        };
        let mut resources = Vec::new();
        let mut logical_handles = Vec::new();
        let mut logical = Vec::new();
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
            // 未访问且未导出的 create 资源已被编译器裁剪；保留逻辑索引但不创建虚假托管对象。
            let handle = match payload.and_then(|r| crate::rg::graph::payload_projection(r).1) {
                Some(token) => Some(
                    backend
                        .managed_resource(token, &mut self.lifecycle)
                        .ok_or(Error::ManagedResourceUnresolved(id))?,
                ),
                None => None,
            };
            resources.extend(handle);
            logical_handles.push(handle);
            logical.push(record::LogicalResource {
                metadata: crate::raw::barrier::Resource {
                    kind: compiled.declarations.resources[id].descriptor.kind_code(),
                    initial: compiled.registry.resource_initial_access[id],
                    reused: mapping.reused,
                    validity: 0,
                },
                first: resolve_access(lifetime.first_access_index),
                last: resolve_access(lifetime.last_access_index),
                export_access: compiled
                    .declarations
                    .exports
                    .iter()
                    .find(|e| e.resource_index as usize == id)
                    .map(|e| e.access_code),
                last_command_sequence: 0,
            });
        }
        let mut passes = compiled
            .info
            .pass_access_ranges
            .iter()
            .map(|(start, end)| record::Pass {
                accesses: compiled.info.accesses[start.0 as usize..end.0 as usize]
                    .iter()
                    .enumerate()
                    .map(|(offset, current)| record::PassAccess {
                        resource: current.resource_index as usize,
                        previous: resolve_access(current.previous_access_index),
                        current: access(current, writes[start.0 as usize + offset]),
                    })
                    .collect(),
                callback_present: true,
            })
            .collect::<Vec<_>>();
        if compiled.snapshot.batches.is_empty() {
            return Ok(resources);
        }
        let first_handle = backend.command_handle().map_err(Error::Lifecycle)?;
        let first = self.begin(
            first_handle,
            family,
            queue,
            backend.begin_command(first_handle),
        )?;
        backend.command_started(&first).map_err(Error::Lifecycle)?;
        let mut recorder = record::Recorder {
            resources: logical,
            pending_initial: compiled
                .pending_initial
                .iter()
                .map(|i| *i as usize)
                .collect(),
            commands: vec![first.record_command()],
        };
        for (batch_index, batch) in compiled.snapshot.batches.iter().enumerate() {
            let synchronization = synchronization[batch_index].clone();
            // 初始屏障消费上一条活动命令；新命令分配前更新已恢复的跟踪状态。
            if let Some(active) = recorder.commands.last() {
                let original = &self.command_snapshots[&active.handle];
                for &id in &recorder.pending_initial {
                    if recorder.resources[id].first.is_some()
                        && !recorder.resources[id].metadata.reused
                    {
                        self.lifecycle
                            .track(
                                logical_handles[id].ok_or(Error::ResourceUnavailable(id))?,
                                original,
                            )
                            .map_err(Error::Lifecycle)?;
                    }
                }
            }
            let record_start = std::time::Instant::now();
            stats.batches += 1;
            let worth_it = self.pgo.previous.total_ns() >= self.parallel.min_total_ns
                && (self.parallel.min_total_ns == 0 || !self.pgo.previous.is_empty());
            let chunks = if worth_it && backend.chunk_recorder().is_some() {
                crate::rg::parallel::plan_chunks(
                    self.parallel.strategy,
                    &pass_names[batch.pass_range.clone()],
                    &self.pgo.previous,
                    // 候选约束位：当前每个图 pass 自成完整 render pass 实例，任意边界都可切。
                    &[],
                )
            } else {
                std::iter::once(0..batch.pass_range.len()).collect()
            };
            let last_command = if chunks.len() > 1 {
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
                        logical_handles: &logical_handles,
                        managed: managed.as_ref(),
                        pass_names: &pass_names,
                        family,
                        queue,
                    },
                    backend,
                )?;
                stats.count_barriers(&outcome.1);
                outcome.0
            } else {
                let new = if matches!(
                    self.parallel.strategy,
                    crate::rg::parallel::ChunkStrategy::Serial
                ) {
                    let handle = recorder.commands.last().ok_or(Error::EmptyCommands)?.handle;
                    self.command_snapshots
                        .get(&handle)
                        .ok_or(Error::MissingCommand(handle))?
                        .clone()
                } else {
                    let handle = backend.command_handle().map_err(Error::Lifecycle)?;
                    let command =
                        self.begin(handle, family, queue, backend.begin_command(handle))?;
                    backend
                        .command_started(&command)
                        .map_err(Error::Lifecycle)?;
                    command
                };
                let handle = new.handle();
                let start = batch.pass_range.start;
                let owners = std::cell::RefCell::new(Vec::new());
                let mut durations = Vec::new();
                if let Some(resources) = &managed {
                    owners
                        .borrow_mut()
                        .push(resources.clone() as crate::raw::binding::ResourceOwner);
                }
                let outcome = self.record_optional_resources(
                    &mut recorder,
                    &mut passes[batch.pass_range.clone()],
                    batch.export_barriers,
                    &new,
                    &logical_handles,
                    |action| match action {
                        record::RecordAction::Pass(offset) => {
                            if !backend.flush_barriers(handle) {
                                return false;
                            }
                            let pass_index = start + offset;
                            let timer = std::time::Instant::now();
                            let ok = callbacks[pass_index](PassContext {
                                pass_index,
                                command: &new,
                                compiled,
                                logical_resources: &logical_handles,
                                managed: managed.as_deref(),
                                owners: &owners,
                            });
                            durations.push(PassPgoDuration::new(
                                pass_names[pass_index],
                                timer.elapsed().as_nanos() as u64,
                            ));
                            ok
                        }
                        record::RecordAction::End(command) => {
                            backend.end_command(vk::CommandBuffer::from_raw(command))
                        }
                        record::RecordAction::Transition {
                            command,
                            resource,
                            from,
                            to,
                        } => {
                            let backing = compiled.registry.resources[resource].backing_index;
                            let token = crate::rg::graph::payload_projection(
                                &compiled.registry.backing_resources[backing as usize],
                            )
                            .1
                            .unwrap();
                            backend.transition(command, token, from, to)
                        }
                    },
                );
                if !backend.retain_command_owners(handle, owners.into_inner()) {
                    return Err(Error::MissingCommand(handle));
                }
                self.pgo.current.extend(durations);
                let outcome = outcome?;
                stats.count_barriers(&outcome);
                new
            };
            stats.record_ns += record_start.elapsed().as_nanos() as u64;
            if batch.export_barriers {
                for export in &compiled.declarations.exports {
                    self.lifecycle
                        .track(
                            logical_handles[export.resource_index as usize].ok_or(
                                Error::ResourceUnavailable(export.resource_index as usize),
                            )?,
                            &last_command,
                        )
                        .map_err(Error::Lifecycle)?;
                }
            }
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
                sequence::Queue::Primary => self.queues[0],
                sequence::Queue::Auxiliary(index) => self.queues[index as usize + 1],
            };
            let point = backend
                .submit(raw_queue, &[prepared])
                .map_err(Error::Validation)?;
            let receipt = sequence::SubmissionReceipt {
                queue,
                last_timeline: sequence::Timeline(point.value),
            };
            let pending = commands
                .iter()
                .map(|c| {
                    let mut command = self
                        .command_snapshots
                        .remove(&c.handle)
                        .expect("本批命令已登记");
                    command.set_state(3);
                    sequence::PendingCommand {
                        command,
                        threshold: sequence::Timeline(point.value),
                    }
                })
                .collect();
            self.lifecycle.accept_submission(receipt, pending);
            stats.submit_ns += submit_start.elapsed().as_nanos() as u64;
            receipts.push(receipt);
            if !backend.collect_managed_waste() {
                return Err(Error::ManagedResourceUnresolved(usize::MAX));
            }
            if !batch.export_barriers {
                let handle = backend.command_handle().map_err(Error::Lifecycle)?;
                let active = self.begin(handle, family, queue, backend.begin_command(handle))?;
                backend.command_started(&active).map_err(Error::Lifecycle)?;
                recorder.commands.push(active.record_command());
            }
        }
        Ok(resources)
    }
    pub fn new(primary: vk::Queue, auxiliary: Vec<vk::Queue>, family_count: u32) -> Self {
        let auxiliary_count = auxiliary.len() as u32;
        Self {
            counters: sequence::DeviceCounters::new(auxiliary_count),
            lifecycle: sequence::SequenceRuntime::new(family_count),
            queues: std::iter::once(primary).chain(auxiliary).collect(),
            command_snapshots: BTreeMap::new(),
            recorded_sequences: Vec::new(),
            graph_queue_domain: None,
            pool_family_count: family_count,
            parallel: Default::default(),
            pgo: Default::default(),
        }
    }

    pub fn begin(
        &mut self,
        handle: u64,
        family: u32,
        queue: sequence::Queue,
        backend: Result<(), sequence::Failure>,
    ) -> Result<sequence::Command, Error> {
        let command = self
            .lifecycle
            .begin_after_backend(
                &self.counters,
                sequence::Command::new(handle, family),
                queue,
                backend,
            )
            .map_err(Error::Begin)?;
        self.recorded_sequences.push(command.sequence().0);
        self.command_snapshots.insert(handle, command.clone());
        Ok(command)
    }

    /// 图声明驱动真实命令序号、顺序录制器和资源消费者。
    pub fn record(
        &mut self,
        recorder: &mut record::Recorder,
        passes: &mut [record::Pass],
        final_batch: bool,
        command: &sequence::Command,
        resources: &[sequence::ResourceHandle],
        boundary: impl FnMut(record::RecordAction) -> bool,
    ) -> Result<record::Outcome, Error> {
        let resources = resources.iter().copied().map(Some).collect::<Vec<_>>();
        self.record_optional_resources(recorder, passes, final_batch, command, &resources, boundary)
    }

    fn record_optional_resources(
        &mut self,
        recorder: &mut record::Recorder,
        passes: &mut [record::Pass],
        final_batch: bool,
        command: &sequence::Command,
        resources: &[Option<sequence::ResourceHandle>],
        boundary: impl FnMut(record::RecordAction) -> bool,
    ) -> Result<record::Outcome, Error> {
        // zip 不允许静默截断，否则托管对象会与图内逻辑槽错位并漏掉 GPU 跟踪。
        if resources.len() != recorder.resources.len() {
            return Err(Error::ResourceCountMismatch {
                expected: recorder.resources.len(),
                actual: resources.len(),
            });
        }
        let outcome =
            recorder.record_batch(passes, final_batch, command.record_command(), boundary);
        self.finish_record(recorder, outcome, resources)
    }

    /// 录制后处理：导入资源跟踪（回调失败也不回滚）、panic 转错误、同步命令状态快照。
    fn finish_record(
        &mut self,
        recorder: &record::Recorder,
        outcome: record::Outcome,
        resources: &[Option<sequence::ResourceHandle>],
    ) -> Result<record::Outcome, Error> {
        for (logical, handle) in recorder.resources.iter().zip(resources) {
            let Some(handle) = handle else {
                continue;
            };
            self.lifecycle
                .import_record_tracker(*handle, sequence::Sequence(logical.last_command_sequence))
                .map_err(Error::Lifecycle)?;
        }
        if outcome.panic {
            return Err(Error::Record(outcome));
        }
        for recorded in &recorder.commands {
            let snapshot = self
                .command_snapshots
                .get_mut(&recorded.handle)
                .ok_or(Error::MissingCommand(recorded.handle))?;
            snapshot.set_state(recorded.state);
        }
        Ok(outcome)
    }

    /// 并行录制一个 batch：串行开命令并规划屏障 → 任务池按块并行录制 → 串行收尾。
    /// 与原 `record_batch` 同形：串行段构造屏障，每块一条命令缓冲，块内逐 pass 发屏障、调回调、计时。
    fn record_chunked_batch<C>(
        &mut self,
        batch: ChunkedBatch<'_, C>,
        backend: &mut impl ExecutionBackend,
    ) -> Result<(sequence::Command, record::Outcome), Error>
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
            logical_handles,
            managed,
            pass_names,
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
            backend
                .command_started(&command)
                .map_err(Error::Lifecycle)?;
            begun.push(command);
        }
        let record_commands = begun.iter().map(|c| c.record_command()).collect::<Vec<_>>();
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
                        resource,
                        from,
                        to,
                    } => backend.transition(command, token_of(resource), from, to),
                    _ => true,
                }
            });
        if outcome.panic {
            self.finish_record(recorder, outcome, logical_handles)?;
            return Err(Error::ChunkPlan);
        }
        let transitions = plan
            .iter()
            .map(|list| {
                list.iter()
                    .map(|t| ChunkTransition {
                        token: token_of(t.resource),
                        from: t.from,
                        to: t.to,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        // 并行：块间互不重叠，各自独占命令缓冲、回调切片与 owner 列表。
        let mut work = Vec::with_capacity(chunks.len());
        let mut rest = callbacks;
        for (range, command) in chunks.iter().zip(&begun) {
            let (head, tail) = std::mem::take(&mut rest).split_at_mut(range.len());
            rest = tail;
            work.push(ChunkWork {
                command,
                start: range.start,
                callbacks: head,
                transitions: &transitions[range.clone()],
            });
        }
        let results = {
            let view = backend.chunk_recorder().ok_or(Error::ChunkPlan)?;
            let view = &*view;
            self.parallel.runner.map_chunks(&mut work, |_, work| {
                let owners = std::cell::RefCell::new(Vec::new());
                if let Some(resources) = managed {
                    owners
                        .borrow_mut()
                        .push(resources.clone() as crate::raw::binding::ResourceOwner);
                }
                let mut durations = Vec::new();
                let mut ok = true;
                for (offset, callback) in work.callbacks.iter_mut().enumerate() {
                    if !view.record_transitions(work.command.handle(), &work.transitions[offset]) {
                        ok = false;
                        break;
                    }
                    let pass_index = first_pass + work.start + offset;
                    let timer = std::time::Instant::now();
                    let good = callback(PassContext {
                        pass_index,
                        command: work.command,
                        compiled,
                        logical_resources: logical_handles,
                        managed: managed.map(|m| &**m),
                        owners: &owners,
                    });
                    durations.push(PassPgoDuration::new(
                        pass_names[pass_index],
                        timer.elapsed().as_nanos() as u64,
                    ));
                    if !good {
                        ok = false;
                        break;
                    }
                }
                ChunkResult {
                    ok,
                    durations,
                    owners: owners.into_inner(),
                }
            })
        };
        // 串行：按块顺序合并耗时与 owner。
        let mut failed = false;
        for (command, result) in begun.iter().zip(results) {
            self.pgo.current.extend(result.durations);
            if !backend.retain_command_owners(command.handle(), result.owners) {
                return Err(Error::MissingCommand(command.handle()));
            }
            failed |= !result.ok;
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
                        resource,
                        from,
                        to,
                    } => backend.transition(command, token_of(resource), from, to),
                    record::RecordAction::Pass(_) => true,
                }
            });
        }
        let outcome = self.finish_record(recorder, outcome, logical_handles)?;
        let last = begun.last().cloned().ok_or(Error::ChunkPlan)?;
        Ok((last, outcome))
    }

    pub fn complete(
        &mut self,
        queue: sequence::Queue,
        value: sequence::Timeline,
    ) -> Result<(), Error> {
        self.lifecycle.observe_timeline(queue, value);
        self.lifecycle
            .reclaim_ready(queue)
            .map_err(Error::Lifecycle)
    }
}

/// PGO 名称→编号映射。原作 `PassPgoNameId` 就是 pass 名 `&'static str` 的指针，
/// `from_name` 恒等（0x14067dd80 `mov rax,rcx; ret`，ICF 折叠；record_batch 0x1402c22ba 把
/// `(pass+0x18, pass+0x20)` 这个 `&str` 的指针直接当键），哈希发生在统计表 `hash_one`（0x1405b0d70），
/// 桶比较的也只是这个 u64（0x140bea270 `cmp [r14], rsi`）。
///
/// 这里 pass 名是自有 `String`，自身指针跨帧不稳定，所以用**驻留池**：同名字符串只保留一份
/// `&'static str`（首次见到时泄漏一份 `Box<str>`，上限为不同 pass 名的个数），`PassPgoNameId` 取该份的
/// 数据指针。因此同名跨帧稳定、不同名必不同，且与原版一样是指针恒等；数值本身仍无原版对应物。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PassPgoNameId(pub usize);
fn intern_pass_name(name: &str, stat: Option<&'static str>) -> &'static str {
    use std::collections::HashSet;
    use std::sync::{Mutex, OnceLock};
    static POOL: OnceLock<Mutex<HashSet<&'static str>>> = OnceLock::new();
    let mut pool = POOL
        .get_or_init(|| Mutex::new(HashSet::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(existing) = pool.get(name) {
        return existing;
    }
    let interned: &'static str = match stat {
        Some(s) => s,
        None => Box::leak(name.to_owned().into_boxed_str()),
    };
    pool.insert(interned);
    interned
}
impl PassPgoNameId {
    /// 原版调用形态：传入 `&'static str`，不分配、不泄漏。
    pub fn from_static(name: &'static str) -> Self {
        Self(intern_pass_name(name, Some(name)).as_ptr() as usize)
    }
    /// 自有 `String`/`&str` 入口：经驻留池取稳定指针。
    pub fn from_name(name: &str) -> Self {
        Self(intern_pass_name(name, None).as_ptr() as usize)
    }
}

/// 单个 pass 的一次 PGO 时长观测。
#[derive(Clone, Copy, Debug)]
pub struct PassPgoDuration {
    pub name: PassPgoNameId,
    pub duration_ns: u64,
}
impl PassPgoDuration {
    pub fn new(name: PassPgoNameId, duration_ns: u64) -> Self {
        Self { name, duration_ns }
    }
}

/// 一帧的 pass PGO 统计。原作布局（0x40 字节）：按名字的耗时表、`total_ns@0x30`、`max_ns@0x38`；
/// `record_batch` 读上一帧统计做分块，合并各块耗时时 `total += d`、`max = max(max, d)`。
#[derive(Clone, Debug, Default)]
pub struct PassPgoStats {
    counts: std::collections::HashMap<PassPgoNameId, u64>,
    durations: std::collections::HashMap<PassPgoNameId, u64>,
    total_ns: u64,
    max_ns: u64,
}
impl PassPgoStats {
    /// 对应 `end_frame`（0x140bec400）里对新 `current` 的就地清空：控制字节 memset 0xff、`len = 0`、
    /// 重算 `growth_left`，并把 `total_ns@0x30`、`max_ns@0x38` 置 0——哈希种子（`+0x20..0x30`）保留。
    /// `counts` 是候选实现自加的辅助表，原 0x40 字节结构里没有。
    pub fn reset(&mut self) {
        self.counts.clear();
        self.durations.clear();
        self.total_ns = 0;
        self.max_ns = 0;
    }
    /// ✅ 原 `extend` 无独立符号，是 `record_batch`（0x1402c1d40）里的内联循环（0x1402c2xxx / 0x1402c2f..，
    /// 串行与并行两条路径各一份）：对块返回的每个 `(name:u64, d:u64)`（16 字节 `PassPgoDuration`）依次
    /// ① `FUN_1402ca150(current, name, d)`——hashbrown `insert`，命中时**覆盖**旧值（机器码
    /// `0x1402ca255: mov rdx,[r15-8]; ...; 0x1402ca2a1: mov [r15],rsi`，不是累加）；
    /// ② `total@0x30 += d`；③ `max@0x38 = max(max, d)`。
    pub fn extend(&mut self, durations: impl IntoIterator<Item = PassPgoDuration>) {
        for duration in durations {
            self.record(duration);
        }
    }
    /// 原 `duration_ns`（0x140bea1e0）实返 `Option<u64>`（命中时 rax=1、rdx=桶内值；Ghidra 伪代码把它写成了
    /// bool，是丢了 rdx）。唯一调用点 record_batch 0x1402c22ce 做 `unwrap_or(0)`，这里的 `u64` 版就是该折叠；
    /// 需要原 `Option` 形态用 [`Self::get`]。
    pub fn duration_ns(&self, id: PassPgoNameId) -> u64 {
        self.get(id).unwrap_or(0)
    }
    pub fn get(&self, id: PassPgoNameId) -> Option<u64> {
        self.durations.get(&id).copied()
    }
    pub fn call_count(&self, id: PassPgoNameId) -> u64 {
        self.counts.get(&id).copied().unwrap_or(0)
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
    /// 单条记录 = `record_batch` 内联体的一次迭代：`insert`（覆盖，不累加）+ `total += d` + `max = max(max, d)`。
    /// 同名 pass 一帧内出现多次时，表里只留最后一次，`total` 仍累加全部；`counts` 是候选辅助表（原版没有）。
    pub fn record(&mut self, duration: PassPgoDuration) {
        *self.counts.entry(duration.name).or_default() += 1;
        self.durations.insert(duration.name, duration.duration_ns);
        self.total_ns = self.total_ns.wrapping_add(duration.duration_ns);
        self.max_ns = self.max_ns.max(duration.duration_ns);
    }
}

/// 原 `GraphExecutionRecorder::end_frame`（0x140bec400）交换两份统计并清空新的本帧：
/// 分块严格依据上一帧，统计不跨帧累积。
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
