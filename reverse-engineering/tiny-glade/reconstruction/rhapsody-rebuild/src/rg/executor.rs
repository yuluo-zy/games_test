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
    pub device: submit::DeviceState,
    command_snapshots: BTreeMap<u64, sequence::Command>,
    queue_failure: bool,
    graph_queue_domain: Option<sequence::Queue>,
    pool_family_count: u32,
}

#[derive(Debug)]
pub enum Error {
    Begin(sequence::BeginFailure),
    Lifecycle(sequence::Failure),
    Record(record::Outcome),
    MissingCommand(u64),
    EmptyCommands,
    InvalidLastState,
    EndFailed,
    Queue(submit::Outcome),
    Compile(crate::rg::graph::PopulationFailure),
    ResourceUnavailable(usize),
    CallbackCount,
    ManagedResourceUnresolved(usize),
    CompletionAfterQueueFailure,
    QueueUnavailable(sequence::Queue),
    InvalidSynchronization,
    UnsupportedQueueDomainSwitch,
    ResourceCountMismatch { expected: usize, actual: usize },
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

pub trait ExecutionBackend: crate::rg::graph::ResourceBackend {
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
    fn begin_command(&mut self, handle: u64) -> Result<(), sequence::Failure>;
    fn end_command(&mut self, handle: vk::CommandBuffer) -> bool;
    /// CPU 后端可只记录投影；实际后端必须发出 Vulkan 屏障。
    fn transition(&mut self, _command: u64, _token: u64, _from: u8, _to: u8) -> bool {
        true
    }
    fn flush_barriers(&mut self, _command: u64) -> bool {
        true
    }
    fn submit(&mut self, queue: vk::Queue, prepared: &[submit::PreparedSubmit]) -> (i32, bool);
}

pub struct GraphExecution {
    pub compiled: crate::rg::graph::CompiledGraph,
    pub receipts: Vec<sequence::SubmissionReceipt>,
    pub resources: Vec<sequence::ResourceHandle>,
    pub recorder_events: Vec<String>,
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
    dependency: &mut impl crate::rg::graph::AccessDependency,
) -> crate::raw::barrier::Access {
    crate::raw::barrier::Access {
        code: record.access_code,
        group: record.synchronization_group,
        write: dependency.is_write(record.access_code),
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
        callbacks: &mut [impl FnMut(u64) -> bool],
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
        callbacks: &mut [impl FnMut(PassContext<'_>) -> bool],
    ) -> Result<GraphExecution, Error> {
        if callbacks.len() != graph.passes.len() {
            return Err(Error::CallbackCount);
        }
        let (queue, family) = backend.graph_queue();
        if family >= self.pool_family_count {
            return Err(Error::Lifecycle(sequence::Failure::PoolFamilyBounds));
        }
        if matches!(queue, sequence::Queue::Auxiliary(i) if i as usize >= self.device.auxiliary.len())
        {
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
        let compiled = crate::rg::graph::compile_graph(graph, pool, backend, dependency)
            .map_err(Error::Compile)?;
        if !backend.prepare_compiled_resources(&compiled) {
            return Err(Error::ResourceUnavailable(usize::MAX));
        }
        let managed = backend.managed_pass_resources();
        let synchronization = (0..compiled.snapshot.batches.len())
            .map(|i| backend.batch_synchronization(i, compiled.snapshot.batches.len()))
            .collect::<Vec<_>>();
        if synchronization.iter().any(|s| !s.valid()) {
            return Err(Error::InvalidSynchronization);
        }
        let resolve_access = |index: u32, dependency: &mut _| {
            (index != crate::rg::graph::NO_ACCESS)
                .then(|| access(&compiled.info.accesses[index as usize], dependency))
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
                first: resolve_access(lifetime.first_access_index, dependency),
                last: resolve_access(lifetime.last_access_index, dependency),
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
                    .map(|current| record::PassAccess {
                        resource: current.resource_index as usize,
                        previous: resolve_access(current.previous_access_index, dependency),
                        current: access(current, dependency),
                    })
                    .collect(),
                callback_present: true,
            })
            .collect::<Vec<_>>();
        if compiled.snapshot.batches.is_empty() {
            return Ok(GraphExecution {
                compiled,
                receipts: vec![],
                resources,
                recorder_events: vec![],
            });
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
        let mut receipts = Vec::new();
        let mut events = Vec::new();
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
            let handle = backend.command_handle().map_err(Error::Lifecycle)?;
            let new = self.begin(handle, family, queue, backend.begin_command(handle))?;
            backend.command_started(&new).map_err(Error::Lifecycle)?;
            let start = batch.pass_range.start;
            let owners = std::cell::RefCell::new(Vec::new());
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
                        callbacks[pass_index](PassContext {
                            pass_index,
                            command: &new,
                            compiled: &compiled,
                            logical_resources: &logical_handles,
                            managed: managed.as_deref(),
                            owners: &owners,
                        })
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
            let outcome = outcome?;
            events.extend(outcome.events);
            if batch.export_barriers {
                for export in &compiled.declarations.exports {
                    self.lifecycle
                        .track(
                            logical_handles[export.resource_index as usize].ok_or(
                                Error::ResourceUnavailable(export.resource_index as usize),
                            )?,
                            &new,
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
            let end_ok = backend.end_command(vk::CommandBuffer::from_raw(ended));
            let (receipt, _) = self.submit_recorded_with_sync(
                &mut recorder,
                synchronization,
                |_| end_ok,
                |queue, prepared| backend.submit(queue, prepared),
            )?;
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
        Ok(GraphExecution {
            compiled,
            receipts,
            resources,
            recorder_events: events,
        })
    }
    pub fn new(auxiliary_count: u32, family_count: u32, device: submit::DeviceState) -> Self {
        Self {
            counters: sequence::DeviceCounters::new(auxiliary_count),
            lifecycle: sequence::SequenceRuntime::new(family_count),
            device,
            command_snapshots: BTreeMap::new(),
            queue_failure: false,
            graph_queue_domain: None,
            pool_family_count: family_count,
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
        // 回调失败仍导入之前已发生的资源跟踪，不回滚。
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

    /// 接管命令所有权、结束最后开放命令、构造队列契约，再转移真实每命令阈值。
    pub fn submit_recorded(
        &mut self,
        recorder: &mut record::Recorder,
        end: impl FnMut(vk::CommandBuffer) -> bool,
        backend: impl FnMut(vk::Queue, &[submit::PreparedSubmit]) -> (i32, bool),
    ) -> Result<(sequence::SubmissionReceipt, submit::Outcome), Error> {
        self.submit_recorded_with_sync(recorder, GraphBatchSync::default(), end, backend)
    }
    pub fn submit_recorded_with_sync(
        &mut self,
        recorder: &mut record::Recorder,
        synchronization: GraphBatchSync,
        mut end: impl FnMut(vk::CommandBuffer) -> bool,
        backend: impl FnMut(vk::Queue, &[submit::PreparedSubmit]) -> (i32, bool),
    ) -> Result<(sequence::SubmissionReceipt, submit::Outcome), Error> {
        if !synchronization.valid() {
            return Err(Error::InvalidSynchronization);
        }
        let mut commands = std::mem::take(&mut recorder.commands);
        let last = commands.last_mut().ok_or(Error::EmptyCommands)?;
        if last.state != 1 {
            return Err(Error::InvalidLastState);
        }
        if !end(vk::CommandBuffer::from_raw(last.handle)) {
            return Err(Error::EndFailed);
        }
        last.state = 2;
        let first = self
            .command_snapshots
            .get(&commands[0].handle)
            .ok_or(Error::MissingCommand(commands[0].handle))?;
        let queue = first.queue().map_err(Error::Lifecycle)?;
        let (tag, index) = queue.raw();
        let queue_id = submit::QueueId { tag, index };
        let queue_commands = commands
            .iter()
            .map(|recorded| {
                let original = self
                    .command_snapshots
                    .get_mut(&recorded.handle)
                    .ok_or(Error::MissingCommand(recorded.handle))?;
                original.set_state(recorded.state);
                let (tag, index) = original.queue().map_err(Error::Lifecycle)?.raw();
                Ok(submit::Command {
                    state: u64::from(recorded.state),
                    sequence: original.sequence().0,
                    queue: submit::QueueId { tag, index },
                    handle: vk::CommandBuffer::from_raw(recorded.handle),
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let synchronized = !synchronization.waits.is_empty() || !synchronization.signals.is_empty();
        let batch = submit::Batch {
            commands: queue_commands,
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
        // 原 current 槽只有单个 binary wait/signal（值 0）；完整 timeline 数组由 prior Batch 槽承载。
        // 外部同步批次保留原 prior→current 的顺序，末尾 current 为空；默认图不改变既有行为。
        let input = if synchronized {
            submit::SubmissionInput {
                flushed_command: None,
                prior: Some(batch),
                current: submit::Batch::default(),
            }
        } else {
            submit::SubmissionInput {
                flushed_command: None,
                prior: None,
                current: batch,
            }
        };
        let out = self.device.queue_submit(queue_id, input, backend);
        if out.panic.is_some() {
            // 注册与状态 3 写入发生在后端调用前；失败不伪造回执或回滚，也不继续复用。
            self.queue_failure = true;
            return Err(Error::Queue(out));
        }
        let (returned_queue, last_timeline) =
            out.receipt.ok_or_else(|| Error::Queue(out.clone()))?;
        let queue = sequence::Queue::from_raw(returned_queue.tag, returned_queue.index)
            .map_err(Error::Lifecycle)?;
        let receipt = sequence::SubmissionReceipt {
            queue,
            last_timeline: sequence::Timeline(last_timeline),
        };
        let selected = match queue {
            sequence::Queue::Primary => &mut self.device.primary,
            sequence::Queue::Auxiliary(index) => &mut self.device.auxiliary[index as usize],
        };
        // 转移新注册的队列命令，不重新生成其时间线。
        let pending = selected
            .pending_commands
            .drain(..)
            .map(|item| {
                let mut original = self
                    .command_snapshots
                    .remove(&item.command.handle.as_raw())
                    .ok_or(Error::MissingCommand(item.command.handle.as_raw()))?;
                original.set_state(item.command.state as u32);
                Ok(sequence::PendingCommand {
                    command: original,
                    threshold: sequence::Timeline(item.timeline),
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        self.lifecycle.accept_submission(receipt, pending);
        Ok((receipt, out))
    }

    pub fn complete(
        &mut self,
        queue: sequence::Queue,
        value: sequence::Timeline,
    ) -> Result<(), Error> {
        if self.queue_failure {
            return Err(Error::CompletionAfterQueueFailure);
        }
        let selected = match queue {
            sequence::Queue::Primary => &mut self.device.primary,
            sequence::Queue::Auxiliary(index) => &mut self.device.auxiliary[index as usize],
        };
        while selected
            .pending_timelines
            .front()
            .is_some_and(|v| *v <= value.0)
        {
            selected.pending_timelines.pop_front();
        }
        self.lifecycle.observe_timeline(queue, value);
        self.lifecycle
            .reclaim_ready(queue)
            .map_err(Error::Lifecycle)
    }
}

/// PGO 名称→编号映射。原作按 pass 名收集每 pass 统计；候选用 FNV-1a 生成稳定编号，
/// 不声称等价于原内部字符串表的具体编号。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PassPgoNameId(pub u32);
impl PassPgoNameId {
    pub fn from_name(name: &str) -> Self {
        let mut hash = 0x811c9dc5u32;
        for byte in name.bytes() {
            hash ^= u32::from(byte);
            hash = hash.wrapping_mul(0x0100_0193);
        }
        Self(hash)
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

/// 跨帧聚合的 pass PGO 统计；reset 清空、extend 合并另一份统计。
#[derive(Debug, Default)]
pub struct PassPgoStats {
    counts: std::collections::HashMap<PassPgoNameId, u64>,
    total_ns: std::collections::HashMap<PassPgoNameId, u64>,
}
impl PassPgoStats {
    pub fn reset(&mut self) {
        self.counts.clear();
        self.total_ns.clear();
    }
    pub fn extend(&mut self, other: &Self) {
        for (id, count) in &other.counts {
            *self.counts.entry(*id).or_default() += count;
        }
        for (id, ns) in &other.total_ns {
            *self.total_ns.entry(*id).or_default() += ns;
        }
    }
    pub fn duration_ns(&self, id: PassPgoNameId) -> u64 {
        self.total_ns.get(&id).copied().unwrap_or(0)
    }
    pub fn call_count(&self, id: PassPgoNameId) -> u64 {
        self.counts.get(&id).copied().unwrap_or(0)
    }
    pub fn record(&mut self, duration: PassPgoDuration) {
        *self.counts.entry(duration.name).or_default() += 1;
        *self.total_ns.entry(duration.name).or_default() += duration.duration_ns;
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
