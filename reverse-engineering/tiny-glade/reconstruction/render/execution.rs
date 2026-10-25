//! Explicit CPU integration of separately recovered private contracts.
//! Vulkan creation, command ending and submission callbacks are supplied.
//! This adapter is reconstruction glue, not claimed as an original Rust function.
use ash::vk::{self, Handle};
use std::collections::BTreeMap;
use tiny_glade_queue_submit_r8 as submit;
use tiny_glade_record_resources_r7 as record;
use tiny_glade_sequence_runtime_r8 as sequence;

pub struct ExecutionChain {
    pub counters: sequence::DeviceCounters,
    pub lifecycle: sequence::SequenceRuntime,
    pub device: submit::DeviceState,
    command_snapshots: BTreeMap<u64, sequence::Command>,
    queue_failure: bool,
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
    Compile(crate::graph::PopulationFailure),
    ResourceUnavailable(usize),
    CallbackCount,
    ManagedResourceUnresolved(usize),
    CompletionAfterQueueFailure,
}

pub trait ExecutionBackend: crate::graph::ResourceBackend {
    /// Current physical registry uses u64 tokens. The true managed allocation
    /// producer/generation must be supplied explicitly; no generation is guessed.
    /// Return None when this private integration boundary is not available.
    fn managed_resource(
        &mut self,
        token: u64,
        lifecycle: &mut sequence::SequenceRuntime,
    ) -> Option<sequence::ResourceHandle>;
    /// Device/command creation is still an explicit private-wrapper gap supplied
    /// by the caller; no Vulkan device implementation is claimed here.
    fn command_handle(&mut self) -> Result<u64, sequence::Failure>;
    fn begin_command(&mut self, handle: u64) -> Result<(), sequence::Failure>;
    fn end_command(&mut self, handle: vk::CommandBuffer) -> bool;
    fn submit(&mut self, queue: vk::Queue, prepared: &[submit::PreparedSubmit]) -> (i32, bool);
}

pub struct GraphExecution {
    pub compiled: crate::graph::CompiledGraph,
    pub receipts: Vec<sequence::SubmissionReceipt>,
    pub resources: Vec<sequence::ResourceHandle>,
    pub recorder_events: Vec<String>,
}

/// 当前 pass 的真实录制上下文。命令来自 counter/begin，资源来自本次图编译。
/// 绑定消费者应使用这些状态，不自行填入 sequence、queue 或资源映射。
pub struct PassContext<'a> {
    pub pass_index: usize,
    pub command: &'a sequence::Command,
    pub compiled: &'a crate::graph::CompiledGraph,
    pub resources: &'a [sequence::ResourceHandle],
}

fn access(
    record: &crate::graph::ResourceAccessRecord,
    dependency: &mut impl crate::graph::AccessDependency,
) -> tiny_glade_barrier_record_r5::Access {
    tiny_glade_barrier_record_r5::Access {
        code: record.access_code,
        group: record.synchronization_group,
        write: dependency.is_write(record.access_code),
    }
}

impl ExecutionChain {
    /// 保留仅消费命令句柄的入口；完整绑定消费者使用 execute_graph_with_context。
    pub fn execute_graph(
        &mut self,
        graph: crate::graph::GraphDeclaration,
        pool: &mut crate::graph::Pool,
        dependency: &mut impl crate::graph::AccessDependency,
        backend: &mut impl ExecutionBackend,
        callbacks: &mut [impl FnMut(u64) -> bool],
    ) -> Result<GraphExecution, Error> {
        let mut contextual = callbacks
            .iter_mut()
            .map(|callback| move |context: PassContext<'_>| callback(context.command.handle()))
            .collect::<Vec<_>>();
        self.execute_graph_with_context(graph, pool, dependency, backend, &mut contextual)
    }

    /// 从图声明生成 compiled/record 状态，并在真正录制 pass 时借出上下文。
    /// 原游戏构图闭包、物理 managed 生产者及原整链等价性仍未恢复。
    pub fn execute_graph_with_context(
        &mut self,
        graph: crate::graph::GraphDeclaration,
        pool: &mut crate::graph::Pool,
        dependency: &mut impl crate::graph::AccessDependency,
        backend: &mut impl ExecutionBackend,
        callbacks: &mut [impl FnMut(PassContext<'_>) -> bool],
    ) -> Result<GraphExecution, Error> {
        if callbacks.len() != graph.passes.len() {
            return Err(Error::CallbackCount);
        }
        let compiled = crate::graph::compile_graph(graph, pool, backend, dependency)
            .map_err(Error::Compile)?;
        let resolve_access = |index: u32, dependency: &mut _| {
            (index != crate::graph::NO_ACCESS)
                .then(|| access(&compiled.info.accesses[index as usize], dependency))
        };
        let mut resources = Vec::new();
        let mut logical = Vec::new();
        for (id, lifetime) in compiled.info.lifetimes.iter().enumerate() {
            let mapping = &compiled.registry.resources[id];
            // Validate before the first side effecting command begin. Empty
            // unused resources may be retained but accessed resources must exist.
            let payload = compiled
                .registry
                .backing_resources
                .get(mapping.backing_index as usize);
            if (lifetime.first_access_index != crate::graph::NO_ACCESS
                || compiled
                    .declarations
                    .exports
                    .iter()
                    .any(|e| e.resource_index as usize == id))
                && payload.is_none_or(|r| crate::graph::payload_projection(r).1.is_none())
            {
                return Err(Error::ResourceUnavailable(id));
            }
            let token = payload
                .and_then(|r| crate::graph::payload_projection(r).1)
                .ok_or(Error::ResourceUnavailable(id))?;
            let handle = backend
                .managed_resource(token, &mut self.lifecycle)
                .ok_or(Error::ManagedResourceUnresolved(id))?;
            resources.push(handle);
            logical.push(record::LogicalResource {
                metadata: tiny_glade_barrier_record_r5::Resource {
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
            0,
            sequence::Queue::Primary,
            backend.begin_command(first_handle),
        )?;
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
        for batch in &compiled.snapshot.batches {
            // Initial barriers consume the last active command before allocation
            // of the next recording command. This calls the recovered tracker.
            if let Some(active) = recorder.commands.last() {
                let original = &self.command_snapshots[&active.handle];
                for &id in &recorder.pending_initial {
                    if recorder.resources[id].first.is_some()
                        && !recorder.resources[id].metadata.reused
                    {
                        self.lifecycle
                            .track(resources[id], original)
                            .map_err(Error::Lifecycle)?;
                    }
                }
            }
            let handle = backend.command_handle().map_err(Error::Lifecycle)?;
            let new = self.begin(
                handle,
                0,
                sequence::Queue::Primary,
                backend.begin_command(handle),
            )?;
            let start = batch.pass_range.start;
            let outcome = self.record(
                &mut recorder,
                &mut passes[batch.pass_range.clone()],
                batch.export_barriers,
                &new,
                &resources,
                |offset| {
                    let pass_index = start + offset;
                    callbacks[pass_index](PassContext {
                        pass_index,
                        command: &new,
                        compiled: &compiled,
                        resources: &resources,
                    })
                },
            )?;
            events.extend(outcome.events);
            if batch.export_barriers {
                for export in &compiled.declarations.exports {
                    self.lifecycle
                        .track(resources[export.resource_index as usize], &new)
                        .map_err(Error::Lifecycle)?;
                }
            }
            // Original executor submits between batches, preserving the last
            // open CB within a batch. End then queue-array assembly is restored.
            let ended = recorder
                .commands
                .last()
                .map(|c| c.handle)
                .ok_or(Error::EmptyCommands)?;
            let end_ok = backend.end_command(vk::CommandBuffer::from_raw(ended));
            let (receipt, _) = self.submit_recorded(
                &mut recorder,
                |_| end_ok,
                |queue, prepared| backend.submit(queue, prepared),
            )?;
            receipts.push(receipt);
            if !batch.export_barriers {
                let handle = backend.command_handle().map_err(Error::Lifecycle)?;
                let active = self.begin(
                    handle,
                    0,
                    sequence::Queue::Primary,
                    backend.begin_command(handle),
                )?;
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

    /// The graph inputs are externally recovered; this connects real sequences
    /// to the previously validated sequential recorder and resource consumers.
    pub fn record(
        &mut self,
        recorder: &mut record::Recorder,
        passes: &mut [record::Pass],
        final_batch: bool,
        command: &sequence::Command,
        resources: &[sequence::ResourceHandle],
        callback: impl FnMut(usize) -> bool,
    ) -> Result<record::Outcome, Error> {
        let outcome =
            recorder.record_batch(passes, final_batch, command.record_command(), callback);
        // Import observed tracker results even on callback failure: original
        // tracking occurs before the callback and is not rolled back.
        for (logical, handle) in recorder.resources.iter().zip(resources) {
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

    /// Normal submit adapter: take command ownership, end the last open command,
    /// build the original queue contract, then transfer its real per-CB thresholds.
    pub fn submit_recorded(
        &mut self,
        recorder: &mut record::Recorder,
        mut end: impl FnMut(vk::CommandBuffer) -> bool,
        backend: impl FnMut(vk::Queue, &[submit::PreparedSubmit]) -> (i32, bool),
    ) -> Result<(sequence::SubmissionReceipt, submit::Outcome), Error> {
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
        let out = self.device.queue_submit(
            queue_id,
            submit::SubmissionInput {
                flushed_command: None,
                prior: None,
                current: submit::Batch {
                    commands: queue_commands,
                    ..Default::default()
                },
            },
            backend,
        );
        if out.panic.is_some() {
            // Registration/state3 mutations belong to DeviceState before the
            // backend call. No fake receipt/rollback or later reuse is permitted.
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
        // Move newly registered queue commands; do not recreate their timelines.
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;
    fn chain(next_timeline: u64) -> ExecutionChain {
        let queue = submit::QueueState {
            handle: vk::Queue::from_raw(0xabcd),
            timeline_semaphore: vk::Semaphore::from_raw(0xf000),
            pending_timelines: VecDeque::new(),
            pending_commands: VecDeque::new(),
        };
        ExecutionChain::new(
            0,
            1,
            submit::DeviceState {
                primary: queue,
                auxiliary: vec![],
                next_timeline,
                oldest_pending: None,
            },
        )
    }
    fn graph(
        chain: &mut ExecutionChain,
    ) -> (
        record::Recorder,
        Vec<record::Pass>,
        sequence::Command,
        sequence::ResourceHandle,
    ) {
        let first = chain
            .begin(99, 0, sequence::Queue::Primary, Ok(()))
            .unwrap();
        let next = chain
            .begin(100, 0, sequence::Queue::Primary, Ok(()))
            .unwrap();
        let handle = chain.lifecycle.create_resource(0, 7);
        let (mut recorder, passes) =
            record::fixture(&[1, 2, 3, 0, 1, 0, 1, 999, 1, 999, 0, 0, 0, 1]);
        recorder.commands = vec![first.record_command()];
        recorder.resources[0].last_command_sequence = 0;
        (recorder, passes, next, handle)
    }
    #[test]
    fn producer_to_record_to_actual_queue_payload_to_reclaim() {
        let mut c = chain(41);
        let (mut recorder, mut passes, next, resource) = graph(&mut c);
        assert_eq!(next.sequence(), sequence::Sequence(2));
        c.record(&mut recorder, &mut passes, true, &next, &[resource], |_| {
            true
        })
        .unwrap();
        assert_eq!(c.lifecycle.resource_state(resource), Some((2, 1)));
        c.lifecycle.cpu_retire(resource);
        c.lifecycle.drop_handle(resource).unwrap();
        c.lifecycle.collect_waste();
        let (receipt, _) = c
            .submit_recorded(
                &mut recorder,
                |_| true,
                |queue, prepared| {
                    assert_eq!(queue.as_raw(), 0xabcd);
                    assert_eq!(prepared.len(), 1);
                    assert_eq!(prepared[0].signal_values, [41, 42]);
                    assert_eq!(
                        prepared[0]
                            .commands
                            .iter()
                            .map(|h| h.as_raw())
                            .collect::<Vec<_>>(),
                        [99, 100]
                    );
                    prepared[0].with_vk(|info| unsafe {
                        assert_eq!(info.command_buffer_count, 2);
                        let timeline = &*(info.p_next as *const vk::TimelineSemaphoreSubmitInfo);
                        assert_eq!(
                            std::slice::from_raw_parts(timeline.p_signal_semaphore_values, 2),
                            [41, 42]
                        );
                    });
                    (0, false)
                },
            )
            .unwrap();
        assert_eq!(receipt.last_timeline, sequence::Timeline(42));
        assert!(recorder.commands.is_empty());
        c.complete(sequence::Queue::Primary, sequence::Timeline(41))
            .unwrap();
        assert_eq!(c.lifecycle.pool_sequences(), [1]);
        assert!(!c.lifecycle.events.contains(&"dispose:0".into()));
        c.complete(sequence::Queue::Primary, receipt.last_timeline)
            .unwrap();
        assert_eq!(c.lifecycle.pool_sequences(), [1, 2]);
        assert_eq!(c.lifecycle.prefix(sequence::Queue::Primary), 2);
        assert!(c.lifecycle.events.contains(&"dispose:0".into()));
    }
    #[test]
    fn callback_failure_keeps_tracker_and_prevents_submit() {
        let mut c = chain(41);
        let (mut recorder, mut passes, next, resource) = graph(&mut c);
        assert!(matches!(
            c.record(&mut recorder, &mut passes, true, &next, &[resource], |p| p
                != 1),
            Err(Error::Record(_))
        ));
        assert_eq!(c.lifecycle.resource_state(resource), Some((2, 1)));
        assert_eq!(recorder.commands.len(), 1);
        assert_eq!(c.device.next_timeline, 41);
    }
    #[test]
    fn end_failure_takes_recorder_without_reserving_timeline() {
        let mut c = chain(41);
        let (mut recorder, mut passes, next, resource) = graph(&mut c);
        c.record(&mut recorder, &mut passes, true, &next, &[resource], |_| {
            true
        })
        .unwrap();
        assert!(matches!(
            c.submit_recorded(&mut recorder, |_| false, |_, _| panic!("unreachable")),
            Err(Error::EndFailed)
        ));
        assert!(recorder.commands.is_empty());
        assert_eq!(c.device.next_timeline, 41);
        assert!(c.device.primary.pending_commands.is_empty());
    }
    #[test]
    fn queue_error_preserves_registered_pending_without_success_receipt() {
        let mut c = chain(41);
        let (mut recorder, mut passes, next, resource) = graph(&mut c);
        c.record(&mut recorder, &mut passes, true, &next, &[resource], |_| {
            true
        })
        .unwrap();
        let err = c
            .submit_recorded(&mut recorder, |_| true, |_, _| (-1, false))
            .unwrap_err();
        let Error::Queue(out) = err else {
            panic!("expected queue failure")
        };
        assert_eq!(out.receipt, None);
        assert_eq!(c.device.next_timeline, 43);
        assert_eq!(c.device.primary.pending_commands.len(), 2);
        assert_eq!(c.lifecycle.pending_len(sequence::Queue::Primary), 0);
        // No recovery/complete is claimed after fatal device error.
    }
}
