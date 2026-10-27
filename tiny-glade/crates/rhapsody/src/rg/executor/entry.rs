//! Graph execution entry points and compilation.
use crate::command::CpuArenaPool;
use crate::command::cpu_arena::scratch_borrows;
use crate::command::recording;
use std::collections::BTreeSet;
use super::ExecutionChain;
use super::ExecutionStats;
use super::GraphExecution;
use super::PlanKey;
use super::PreparedGraph;
use super::backend_api::ExecutionBackend;
use super::error::Error;
use super::pass_context::PassContext;
use super::pass_context::PassRecord;
use super::pgo::PassPgoFrames;
use super::pgo::PgoKey;
use super::super::transaction::FrameTransaction;

impl ExecutionChain {
    /// 从图声明编译、绑定、录制并提交一帧。
    pub fn execute_graph(
        &mut self,
        graph: crate::rg::graph::GraphDeclaration,
        pool: &mut crate::rg::graph::Pool,
        dependency: &impl crate::rg::graph::AccessDependency,
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
        dependency: &impl crate::rg::graph::AccessDependency,
        backend: &mut impl ExecutionBackend,
        callbacks: &mut [impl PassRecord],
    ) -> Result<GraphExecution, Error> {
        self.execute_graph_validated(graph, pool, dependency, backend, callbacks, |_| Ok(()))
    }

    /// 同 [`Self::execute_graph_with_context`]，但在编译之后、任何命令开始之前先交给 `validate` 校验编译结果；
    /// 校验失败返回 `Error::Validation`，并按失败路径归还本次取出/新建的资源。
    pub fn execute_graph_validated(
        &mut self,
        graph: crate::rg::graph::GraphDeclaration,
        pool: &mut crate::rg::graph::Pool,
        dependency: &impl crate::rg::graph::AccessDependency,
        backend: &mut impl ExecutionBackend,
        callbacks: &mut [impl PassRecord],
        validate: impl FnOnce(&crate::rg::graph::FrameInstance) -> crate::Result<()>,
    ) -> Result<GraphExecution, Error> {
        let result =
            self.execute_graph_inner(graph, pool, dependency, backend, callbacks, validate);
        // 本帧耗时成为下一帧的分块依据。
        self.pgo.end_frame();
        result
    }

    /// Compile/execute separation: compilation allocates and validates the
    /// physical frame, but never records a callback or submits a command.
    pub fn compile_graph(
        &mut self,
        graph: crate::rg::graph::GraphDeclaration,
        pool: &mut crate::rg::graph::Pool,
        dependency: &impl crate::rg::graph::AccessDependency,
        backend: &mut impl ExecutionBackend,
        callback_count: usize,
    ) -> Result<PreparedGraph, Error> {
        let mut stats = ExecutionStats::default();
        if callback_count != graph.passes.len() {
            return Err(Error::CallbackCount);
        }
        graph.validate().map_err(|error| Error::Compile(error.into()))?;
        backend
            .prepare_declaration(&graph)
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
            let mut found = None;
            self.plan_cache
                .try_get_matching(|key| {
                    let matched = key
                        .0
                        .definition
                        .matches_declaration(&graph, dependency, context)?;
                    if matched {
                        found = Some(key.0.clone());
                    }
                    Ok(matched)
                })
                .map_err(Error::Compile)?;
            found
        } else {
            self.plan_cache
                .record_bypass(crate::collections::cache::CacheReason::Disabled);
            None
        };
        let plan = if let Some(plan) = cached {
            stats.plan_cache_hit = true;
            plan
        } else {
            let definition =
                super::super::plan::GraphDefinition::from_validated_declaration(&graph, dependency, context)
                    .map_err(Error::Compile)?;
            if self.plan_cache_enabled {
                let reason = self
                    .plan_cache
                    .iter()
                    .map(|(old, _)| old.0.definition.differing_key_reason(&definition))
                    .min_by_key(|reason| *reason as usize)
                    .unwrap_or(crate::collections::cache::CacheReason::Cold);
                self.plan_cache.classify_miss(reason);
            }
            let plan = std::sync::Arc::new(
                super::super::plan::compile_with_subgraphs(definition, &mut self.subgraph_cache)
                    .map_err(Error::Compile)?,
            );
            if self.plan_cache_enabled {
                // The key is the plan itself; its definition is stored once.
                let bytes = plan.estimated_bytes();
                self.plan_cache.insert(PlanKey(plan.clone()), (), bytes);
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
        let pgo_key = PgoKey(std::sync::Arc::as_ptr(&plan) as *const () as usize as u64);
        // Allocation is a frame binding operation, never part of plan compilation.
        let compiled =
            crate::rg::graph::instantiate_validated_plan(plan, graph, pool, backend, dependency)
                .map_err(Error::Compile)?;
        stats.compile_ns = compile_start.elapsed().as_nanos() as u64;
        // Compilation is not a successful native execution.
        #[cfg(feature = "cpu-probe")]
        stats.command_workload.invalidate();
        let original_pass_count = callback_count;
        Ok(PreparedGraph {
            compiled,
            stats,
            cpu_arena,
            queue: (queue, family),
            original_pass_count,
            pgo_key,
        })
    }

    fn execute_graph_inner(
        &mut self,
        graph: crate::rg::graph::GraphDeclaration,
        pool: &mut crate::rg::graph::Pool,
        dependency: &impl crate::rg::graph::AccessDependency,
        backend: &mut impl ExecutionBackend,
        callbacks: &mut [impl PassRecord],
        validate: impl FnOnce(&crate::rg::graph::FrameInstance) -> crate::Result<()>,
    ) -> Result<GraphExecution, Error> {
        let prepared = self.compile_graph(graph, pool, dependency, backend, callbacks.len())?;
        self.execute_prepared_inner(prepared, pool, dependency, backend, callbacks, validate)
    }

    /// Swaps in the statistics of the graph about to execute, parking the
    /// previous graph's. The first graph adopts whatever `pgo` already holds.
    pub(super) fn activate_pgo(&mut self, key: PgoKey) {
        const MAX_PARKED: usize = 8;
        if self.pgo_key == Some(key) {
            return;
        }
        self.pgo_clock += 1;
        let adopt = self.pgo_key.is_none();
        if let Some(old) = self.pgo_key {
            if self.pgo_parked.len() >= MAX_PARKED
                && let Some(oldest) = self
                    .pgo_parked
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, slot)| slot.2)
                    .map(|(index, _)| index)
            {
                self.pgo_parked.swap_remove(oldest);
            }
            let frames = std::mem::take(&mut self.pgo);
            self.pgo_parked.push((old, frames, self.pgo_clock));
        }
        self.pgo = if adopt {
            std::mem::take(&mut self.pgo)
        } else if let Some(index) = self.pgo_parked.iter().position(|slot| slot.0 == key) {
            self.pgo_parked.swap_remove(index).1
        } else {
            PassPgoFrames::default()
        };
        self.pgo_key = Some(key);
    }

    pub fn execute_prepared_validated(
        &mut self,
        prepared: PreparedGraph,
        pool: &mut crate::rg::graph::Pool,
        dependency: &impl crate::rg::graph::AccessDependency,
        backend: &mut impl ExecutionBackend,
        callbacks: &mut [impl PassRecord],
        validate: impl FnOnce(&crate::rg::graph::FrameInstance) -> crate::Result<()>,
    ) -> Result<GraphExecution, Error> {
        let result =
            self.execute_prepared_inner(prepared, pool, dependency, backend, callbacks, validate);
        self.pgo.end_frame();
        result
    }

    fn execute_prepared_inner(
        &mut self,
        prepared: PreparedGraph,
        pool: &mut crate::rg::graph::Pool,
        dependency: &impl crate::rg::graph::AccessDependency,
        backend: &mut impl ExecutionBackend,
        callbacks: &mut [impl PassRecord],
        validate: impl FnOnce(&crate::rg::graph::FrameInstance) -> crate::Result<()>,
    ) -> Result<GraphExecution, Error> {
        self.activate_pgo(prepared.pgo_key);
        let PreparedGraph {
            mut compiled,
            mut stats,
            cpu_arena,
            queue: (queue, family),
            original_pass_count,
            pgo_key: _,
        } = prepared;
        #[cfg(feature = "cpu-probe")]
        { stats.command_workload = crate::profile::CommandWorkload::default(); }
        self.recorded_commands = 0;
        if callbacks.len() != original_pass_count {
            crate::rg::graph::reclaim_failed_resources(
                &mut compiled.tables.registry,
                &compiled.declarations.resources,
                &compiled.tables.snapshot.pool_hits,
                pool,
                backend,
            );
            return Err(Error::CallbackCount);
        }
        let mut selected_callbacks = {
            let mut selected = callbacks
                .iter_mut()
                .enumerate()
                .filter(|(i, _)| compiled.plan.retained_passes.binary_search(i).is_ok())
                .map(|(_, callback)| callback as &mut dyn PassRecord);
            let selected = scratch_borrows(
                cpu_arena.as_ref(),
                compiled.plan.retained_passes.len(),
                |_| selected.next().expect("validated retained callback index"),
            );
            match selected {
                Ok(selected) => selected,
                Err(error) => {
                    crate::rg::graph::reclaim_failed_resources(
                        &mut compiled.tables.registry,
                        &compiled.declarations.resources,
                        &compiled.tables.snapshot.pool_hits,
                        pool,
                        backend,
                    );
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
                #[cfg(feature = "cpu-probe")]
                stats.command_workload.invalidate();
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
}
