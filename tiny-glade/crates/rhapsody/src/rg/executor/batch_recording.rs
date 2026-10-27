//! Batch recording, chunking and queue preludes.
use ash::vk;
use ash::vk::Handle;
use crate::command::CpuArenaLease;
use crate::command::CpuArenaPool;
use crate::command::cpu_arena::scratch_slice;
use crate::command::recording;
use crate::native::submit;
use crate::profile::RecordScope;
use crate::rg::recorder as record;
use crate::trace::SubmitTrace;
use super::ChunkResult;
use super::ChunkWork;
use super::ChunkedBatch;
use super::ExecutionChain;
use super::ExecutionStats;
use super::backend_api::ChunkTransition;
use super::backend_api::ExecutionBackend;
use super::error::Error;
use super::pass_context::PassContext;
use super::pass_context::PassRecord;
use super::pgo::PassPgoDuration;
use super::pgo::PassPgoNameId;
use super::super::transaction::FrameTransaction;

impl ExecutionChain {
    /// 编译之后的资源映射、录制与提交；`receipts` 由调用方持有，失败时据此判断是否已有批次进入 GPU。
    #[allow(clippy::too_many_arguments)]
    pub(super) fn record_compiled(
        &mut self,
        compiled: &crate::rg::graph::FrameInstance,
        _dependency: &impl crate::rg::graph::AccessDependency,
        backend: &mut impl ExecutionBackend,
        callbacks: &mut [impl PassRecord],
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
        backend.prepare_compiled_resources(compiled).map_err(Error::Validation)?;
        let managed = backend.managed_pass_resources();
        let mut exported = scratch_slice(cpu_arena, compiled.info.lifetimes.len(), |_| false)
            .map_err(Error::Validation)?;
        for export in &compiled.declarations.exports {
            if let Some(slot) = exported.get_mut(export.resource_index as usize) {
                *slot = true;
            }
        }
        for (id, lifetime) in compiled.info.lifetimes.iter().enumerate() {
            let mapping = &compiled.registry.resources[id];
            // 首次命令 begin 前校验资源映射；有访问或导出的资源必须存在。
            let payload = compiled
                .registry
                .backing_resources
                .get(mapping.backing_index as usize);
            if (lifetime.first_access_index != crate::rg::resource_info::NO_ACCESS || exported[id])
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
            let handle = backend.command_handle().map_err(Error::Validation)?;
            let first = self.begin(handle, family, queue, backend.begin_command(handle))?;
            recorder.commands.push(first.clone());
            stats.coalesced_acquires += backend
                .drain_pending_acquires(handle)
                .map_err(Error::Validation)?;
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
                            && wait.dependency == crate::rg::sync_plan::DependencyRef::Batch(source)
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
            let mut chunks = crate::rg::parallel::Chunks::new();
            crate::rg::parallel::plan_chunks_into(
                self.parallel.strategy,
                &pass_names[batch.pass_range.clone()],
                &self.pgo.previous,
                &[],
                &mut self.chunk_scratch.plan_costs,
                &mut chunks,
            );
            // An export-only batch still dispatches its empty recording scope
            // and lets the same boundary planner emit the final transitions.
            if chunks.is_empty() { chunks.push(0..0); }
            {
                stats.parallel_batches += 1;
                stats.chunks += chunks.len() as u32;
                let batch = ChunkedBatch {
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
                    };
                #[cfg(feature = "cpu-probe")]
                let outcome = self.record_chunked_batch(batch, backend, &mut stats.command_workload)?;
                #[cfg(not(feature = "cpu-probe"))]
                let outcome = self.record_chunked_batch(batch, backend)?;
                stats.count_barriers(&outcome.1);
            };
            stats.record_ns += record_start.elapsed().as_nanos() as u64;
            // 各批次之间提交；保留批次中最后一条开放命令，并恢复 end 后的提交数组构造。
            let ended = recorder
                .commands
                .last()
                .map(|c| c.handle)
                .ok_or(Error::EmptyCommands)?;
            let submit_start = std::time::Instant::now();
            backend.end_command(vk::CommandBuffer::from_raw(ended)).map_err(Error::Validation)?;
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
        let limit = backend.synchronization_plan().bound().batches().len();
        for _ in 0..=limit {
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
            let handle = backend.command_handle().map_err(Error::Validation)?;
            let command = self.begin(
                handle,
                prelude.domain.family,
                queue,
                backend.begin_command(handle),
            )?;
            backend
                .record_queue_prelude(handle, prelude)
                .map_err(Error::Validation)?;
            backend.end_command(vk::CommandBuffer::from_raw(handle)).map_err(Error::Validation)?;
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

    pub fn begin(
        &mut self,
        handle: u64,
        family: u32,
        queue: recording::Queue,
        backend: crate::Result<()>,
    ) -> Result<recording::Command, Error> {
        let mut command = recording::Command::new(handle, family, queue);
        backend.map_err(Error::Validation)?;
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
    /// 串行段构造屏障，每块一条命令缓冲，块内逐 pass 发屏障、调回调、计时。
    fn record_chunked_batch<C>(
        &mut self,
        batch: ChunkedBatch<'_, C>,
        backend: &mut impl ExecutionBackend,
        #[cfg(feature = "cpu-probe")] command_workload: &mut crate::profile::CommandWorkload,
    ) -> Result<(recording::Command, record::Outcome), Error>
    where
        C: PassRecord,
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
            || (!passes.is_empty() && chunks.iter().any(|r| r.is_empty()))
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
        let mut scratch = std::mem::take(&mut self.chunk_scratch);
        scratch.begun.clear();
        // The owner has finished planning the first command before leasing its
        // pool to chunk zero. Reuse it instead of submitting an empty prefix.
        let initial = recorder.commands.last().ok_or(Error::EmptyCommands)?.handle;
        scratch.begun.push(self.command_records.get(&initial).ok_or(Error::MissingCommand(initial))?.clone());
        for index in 1..chunks.len() {
            let handle = backend.chunk_command_handle(index - 1).map_err(Error::Validation)?;
            let command = self.begin(handle, family, queue, backend.begin_command(handle))?;
            scratch.begun.push(command);
        }
        scratch.chunk_of_pass.clear();
        scratch.chunk_of_pass.resize(passes.len(), 0);
        for (index, range) in chunks.iter().enumerate() {
            for pass in range.clone() {
                scratch.chunk_of_pass[pass] = index;
            }
        }
        // 串行：初始屏障、资源跟踪、pass 前屏障规划。
        let mut outcome = recorder.plan_chunked_batch(
            passes,
            &scratch.begun,
            &scratch.chunk_of_pass,
            &mut scratch.planned,
            &mut scratch.ranges,
            |action| match action {
                record::RecordAction::Transition {
                    command,
                    transition,
                } => {
                    backend.transition(command, token_of(transition.resource as usize), &transition)
                }
                record::RecordAction::End(_) => unreachable!("planning does not end commands"),
            },
        ).map_err(Error::Validation)?;
        // Initial transitions must precede worker callbacks in the reused command.
        backend.flush_barriers(initial).map_err(Error::Validation)?;
        scratch.transitions.clear();
        scratch
            .transitions
            .extend(scratch.planned.iter().map(|t| ChunkTransition {
                token: token_of(t.resource as usize),
                transition: *t,
            }));
        let begun = &scratch.begun;
        // 并行：块间互不重叠，各自独占命令缓冲、回调切片与 owner 列表。
        let mut durations =
            scratch_slice(cpu, callbacks.len(), |_| None).map_err(Error::Validation)?;
        let mut gpu_timings =
            scratch_slice(cpu, callbacks.len(), |_| None).map_err(Error::Validation)?;
        for (range, command) in chunks.iter().zip(begun) {
            for index in range.clone() {
                gpu_timings[index] =
                    backend.reserve_pass_timestamp(
                        command.handle(),
                        first_pass + index,
                        pass_names[first_pass + index],
                    );
            }
        }
        let handles = begun
            .iter()
            .map(|command| command.handle())
            .collect::<smallvec::SmallVec<[u64; 8]>>();
        let (view, leases) = match backend.chunk_recording(&handles) {
            Ok(recording) if recording.1.len() == handles.len() => recording,
            Ok(_) => {
                self.chunk_scratch = scratch;
                return Err(Error::ChunkPlan);
            }
            Err(error) => {
                self.chunk_scratch = scratch;
                return Err(Error::Validation(error));
            }
        };
        let mut work = smallvec::SmallVec::<[ChunkWork<'_, '_, C>; 8]>::with_capacity(chunks.len());
        let mut remaining_durations = &mut *durations;
        let mut rest = callbacks;
        for ((range, command), lease) in chunks.iter().zip(begun).zip(leases) {
            debug_assert_eq!(lease.command(), command.handle());
            let (head, tail) = std::mem::take(&mut rest).split_at_mut(range.len());
            rest = tail;
            let (duration_slice, duration_tail) =
                std::mem::take(&mut remaining_durations).split_at_mut(range.len());
            remaining_durations = duration_tail;
            work.push(ChunkWork {
                command,
                lease,
                start: range.start,
                callbacks: head,
                durations: duration_slice,
                transitions: &scratch.transitions,
                ranges: &scratch.ranges[range.clone()],
                gpu_timings: &gpu_timings[range.clone()],
            });
        }
        #[cfg(feature = "cpu-probe")]
        let mut batch_workload = crate::profile::CommandWorkload::default();
        let results = {
            let view = &*view;
            super::super::parallel::map_chunks(&mut work, |_, work| {
                let owners =
                    match super::super::owner_storage::ResourceOwners::new(view.owner_storage_pool()) {
                        Ok(owners) => std::cell::RefCell::new(owners),
                        Err(error) => {
                            return ChunkResult {
                                ok: false,
                                error: Some(error),
                                recorded_durations: 0,
                                #[cfg(feature = "cpu-probe")]
                                command_workload: crate::profile::CommandWorkload::invalid(),
                                owners: super::super::owner_storage::ResourceOwners::empty(),
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
                            #[cfg(feature = "cpu-probe")]
                            command_workload: crate::profile::CommandWorkload::invalid(),
                            owners: super::super::owner_storage::ResourceOwners::empty(),
                        };
                    }
                };
                #[cfg(feature = "cpu-probe")]
                let mut chunk_workload = crate::profile::CommandWorkload::default();
                let mut duration_count = 0;
                let mut ok = true;
                for (offset, callback) in work.callbacks.iter_mut().enumerate() {
                    if let Err(error) = view.record_transitions(
                        &mut work.lease,
                        &work.transitions[work.ranges[offset].clone()],
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
                        unsafe { view.write_pass_timestamp(&mut work.lease, pair, false) };
                    }
                    let mut record_trace = RecordScope::new(
                        compiled.declarations.passes[pass_index]
                            .graph_id.map_or(0, |id| id.raw()),
                        pass_index,
                        work.command.handle(),
                        pass_names[pass_index],
                    );
                    #[cfg(feature = "cpu-probe")]
                    let pass_workload = std::cell::Cell::new(crate::profile::CommandWorkload::default());
                    let timer = std::time::Instant::now();
                    let good = callback.record(PassContext {
                        #[cfg(feature = "cpu-probe")]
                        command_workload: &pass_workload,
                        pass_index,
                        cpu: cpu.as_ref(),
                        command: work.command,
                        compiled,
                        managed: managed.map(|m| &**m),
                        owners: &owners,
                        errors: &errors,
                        coverage: Default::default(),
                    });
                    if let Some(pair) = gpu {
                        // SAFETY: the callback has returned on the same exclusive worker command.
                        unsafe { view.write_pass_timestamp(&mut work.lease, pair, true) };
                    }
                    work.durations[duration_count] = Some(timer.elapsed().as_nanos() as u64);
                    duration_count += 1;
                    let good = good && errors.borrow().is_none();
                    #[cfg(feature = "cpu-probe")]
                    record_trace.complete_with_workload(good, &pass_workload);
                    #[cfg(not(feature = "cpu-probe"))]
                    record_trace.complete(good);
                    drop(record_trace);
                    #[cfg(feature = "cpu-probe")]
                    if good { chunk_workload.merge(pass_workload.get()); }
                    if !good {
                        ok = false;
                        break;
                    }
                }
                #[cfg(feature = "cpu-probe")]
                if !ok { chunk_workload.invalidate(); }
                ChunkResult {
                    #[cfg(feature = "cpu-probe")]
                    command_workload: chunk_workload,
                    ok,
                    error: errors.into_inner(),
                    recorded_durations: duration_count,
                    owners: owners.into_inner(),
                }
            })
        };
        // The leases (and the view) end with the parallel section; the pools
        // return to the backend before the owner records submission preludes.
        drop(work);
        drop(view);
        // 串行：按块顺序合并耗时与 owner。
        let mut failed = false;
        let mut callback_error = None;
        for ((command, range), mut result) in begun.iter().zip(chunks).zip(results) {
            self.pgo.current.extend(
                durations[range.clone()][..result.recorded_durations]
                    .iter()
                    .enumerate()
                    .map(|(offset, duration)| {
                        PassPgoDuration::new(
                            PassPgoNameId::from_resource_name(
                                *pass_names[first_pass + range.start + offset],
                            ),
                            duration.expect("recorded pass duration"),
                        )
                    }),
            );
            let mut transfer_trace = SubmitTrace::new("record_owner_transfer");
            if transfer_trace.enabled() {
                transfer_trace.count("owners", result.owners.len());
                transfer_trace.count("command", command.handle() as usize);
                transfer_trace.count(
                    "graph",
                    compiled.declarations.passes.get(first_pass + range.start)
                        .and_then(|pass| pass.graph_id).map_or(0, |id| id.raw()) as usize,
                );
            }
            // Use the existing diagnostic event's three unused fixed count
            // slots. No extra event, allocation, lock or clean-path arithmetic.
            #[cfg(feature = "cpu-probe")]
            if transfer_trace.enabled() {
                let dispatches = usize::try_from(result.command_workload.direct_dispatch_api_calls).ok();
                transfer_trace.count("direct_dispatch_api_calls", dispatches.unwrap_or(usize::MAX));
                transfer_trace.count("native_valid", usize::from(result.ok && result.command_workload.valid && dispatches.is_some()));
                transfer_trace.count("recorded_passes", result.recorded_durations);
            }
            let retained = backend.retain_command_owners(command.handle(), &mut result.owners);
            transfer_trace.mark("retain_resources");
            drop(transfer_trace);
            retained.map_err(Error::Validation)?;
            #[cfg(feature = "cpu-probe")]
            if result.ok { batch_workload.merge(result.command_workload); }
            failed |= !result.ok;
            if let Some(error) = result.error {
                callback_error.get_or_insert(error);
            }
        }
        if failed {
            outcome.panic = true;
        } else {
            recorder.finish_chunked_batch(
                &mut outcome,
                begun.iter().cloned(),
                final_batch,
                |action| match action {
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
                },
            ).map_err(Error::Validation)?;
        }
        let outcome = self.finish_record(recorder, outcome);
        if let Some(error) = callback_error {
            return Err(Error::Validation(error));
        }
        let outcome = outcome?;
        let last = begun.last().cloned().ok_or(Error::ChunkPlan)?;
        self.chunk_scratch = scratch;
        #[cfg(feature = "cpu-probe")]
        command_workload.merge(batch_workload);
        Ok((last, outcome))
    }
}
