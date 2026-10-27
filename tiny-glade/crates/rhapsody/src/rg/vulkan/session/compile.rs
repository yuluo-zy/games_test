//! Frame begin, cancellation and compilation.
use std::sync::Arc;
use crate::rg::graph;
use super::PreparedSessionFrame;
use super::SessionFrameReceipt;
use super::VulkanGraphSession;
use super::run_retire_callbacks;

impl VulkanGraphSession {
    pub fn begin_frame(&mut self) -> Result<graph::RenderGraph, crate::Error> {
        if self.active_prepared.is_some() {
            return Err(crate::Error::invalid_usage(
                "execute or cancel the compiled graph before beginning another frame",
            ));
        }
        self.collect_frame_imports();
        self.reap_presentation();
        self.result_cache.abandon_pending();
        if self.result_cache.has_work() {
            let completed = self.backend.device.poll()?;
            if self.backend.device.memory.stats.budget_action()
                == crate::accounting::BudgetAction::EvictCache
            {
                self.result_cache.evict_completed(&completed);
            }
            self.collect_cached_results(&completed)?;
        }
        self.forget_evicted_pool_entries();
        self.evict_idle_resources_under_pressure()?;
        // 显示帧内的图共用一个槽：已有显式显示帧时节流已在其开始处完成；
        // 否则本图自成一个显示帧（与旧行为一致：每图占一个槽）。
        let display = match self.active_display {
            Some(display) => {
                // The display-frame throttle counts groups, so a display frame
                // with more graphs than expected (a retry, a partial failure kept
                // in `pending`) could otherwise exhaust the command slots. Bound
                // the graphs themselves as well.
                let slots = self.backend.command_slot_count();
                while self.pending.in_flight() >= slots {
                    let frame = self.pending.front().unwrap().frame;
                    self.wait_frame(frame, 10_000_000_000)?;
                }
                display
            }
            None => {
                self.throttle_in_flight()?;
                self.allocate_display()
            }
        };
        if self.is_poisoned() {
            return Err(crate::Error::DeviceLost);
        }
        self.current_display = display;
        if let Some(named) = &mut self.named {
            named.state.begin_frame();
        }
        self.collect_named_retired();
        self.backend.history_validity.clear();
        self.ledger.begin();
        self.backend.specifications.clear();
        self.backend.synchronization.clear();
        self.backend.graph_waits.clear();
        self.backend.graph_signals.clear();
        self.backend.submissions.clear();
        let graph = graph::RenderGraph::with_shape(self.declaration_shape);
        if let Some(named) = &mut self.named {
            named.graph_id = Some(graph.graph_id());
        }
        Ok(graph)
    }

    /// Cancel a graph that has never been submitted. Declaration leases and
    /// callbacks must be retired even when a game-side builder returns an error.
    /// Already submitted frames are owned by the pending queue and are untouched.
    pub fn cancel_frame(&mut self, built: graph::BuiltGraph) -> crate::Result<()> {
        let belongs_to_frame = self
            .named
            .as_ref()
            .is_some_and(|named| named.graph_id == Some(built.graph_id()));
        self.cancel_presentation_of(built.graph_id());
        let (_, callbacks, retire, _cancelled_rejection) = built.into_parts();
        drop(callbacks);
        let retired = run_retire_callbacks(retire);
        self.collect_frame_imports();
        if !belongs_to_frame {
            return Err(crate::Error::invalid_usage(
                "cancelled graph does not belong to the active frame",
            ));
        }
        if let Some(named) = &mut self.named {
            named.state.abandon_frame();
            named.graph_id = None;
        }
        self.result_cache.abandon_pending();
        self.backend.specifications.clear();
        self.backend.history_validity.clear();
        self.backend.synchronization.clear();
        self.backend.graph_waits.clear();
        self.backend.graph_signals.clear();
        retired
    }

    /// Compile/allocate without issuing Vulkan commands.
    pub fn compile_frame(
        &mut self,
        mut built: graph::BuiltGraph,
    ) -> crate::Result<PreparedSessionFrame> {
        if self.active_prepared.is_some() {
            let (_, _, retire, _rejected_input) = built.into_parts();
            let _ = run_retire_callbacks(retire);
            return Err(crate::Error::invalid_usage(
                "a compiled frame already owns this session",
            ));
        }
        let Some(mut named) = self.named.take() else {
            let (_, _, retire, _rejected_input) = built.into_parts();
            let _ = run_retire_callbacks(retire);
            return Err(crate::Error::invalid_usage(
                "named frame is already compiling",
            ));
        };
        let graph_id = built.graph_id();
        let result = (|| {
            // Known producer failures precede named imports, graph validation,
            // result-cache preparation and all native/CPU frame allocation.
            if let Some(error) = built.take_rejection() {
                let error = if self.is_poisoned() { crate::Error::DeviceLost } else { error };
                let (_, callbacks, retire, rejection) = built.into_parts();
                debug_assert!(rejection.is_none());
                if error.is_device_lost() {
                    self.backend.device.queues.timelines.mark_device_lost();
                }
                drop(callbacks);
                let _ = run_retire_callbacks(retire);
                return Err(error);
            }
            if named.state.requested().is_empty() {
                return self.compile_built(built, |_| Ok(()));
            }
            if named.graph_id != Some(graph_id) {
                let (_, _, retire, _rejected_input) = built.into_parts();
                let _ = run_retire_callbacks(retire);
                return Err(crate::Error::invalid_usage(
                    "named resources belong to another frame graph",
                ));
            }
            let mut graph = built.into_graph();
            let preparation = (|| -> crate::Result<()> {
                for (name, resource) in named.state.requested() {
                    named
                        .resources
                        .import_into(&mut self.backend, &name, resource.token)?;
                }
                named.state.export_temporal(&mut graph)?;
                Ok(())
            })();
            if let Err(error) = preparation {
                let (_, _, retire, _rejected_input) = graph.build().into_parts();
                let _ = run_retire_callbacks(retire);
                return Err(error);
            }
            self.compile_built(graph.build(), |compiled| {
                named
                    .state
                    .validate_compiled_exports(compiled)
                    .map_err(crate::Error::from)
            })
        })();
        if result.is_ok() {
            self.active_prepared = Some(graph_id);
        } else {
            named.state.abandon_frame();
            if named.graph_id == Some(graph_id) {
                named.graph_id = None;
            }
            self.result_cache.abort(graph_id);
            self.cancel_presentation_of(graph_id);
        }
        self.named = Some(named);
        self.collect_frame_imports();
        result
    }

    fn compile_built(
        &mut self,
        built: graph::BuiltGraph,
        validate: impl FnOnce(&graph::FrameInstance) -> crate::Result<()>,
    ) -> crate::Result<PreparedSessionFrame> {
        let graph_id = built.graph_id();
        self.declaration_shape = self
            .declaration_shape
            .union(graph::DeclarationShape::of(&built.declaration));
        let (declaration, callbacks, retire, rejection) = built.into_parts();
        let result = (|| -> crate::Result<(crate::rg::executor::PreparedGraph, crate::rg::receipt_storage::ReceiptLease)> {
            if self.is_poisoned() {
                return Err(crate::Error::DeviceLost);
            }
            if let Some(error) = rejection {
                return Err(error);
            }
            self.result_cache.prepare(
                graph_id,
                &declaration,
                self.backend.device.generation().value(),
            )?;
            self.pending.reserve_frame()?;
            if self.backend.receipt_storage.is_none() {
                self.backend.receipt_storage = Some(crate::rg::receipt_storage::ReceiptStorage::new(
                    Default::default(),
                    self.backend.device.memory.stats.clone(),
                )?);
            }
            let access = Arc::clone(&self.backend.accesses);
            let prepared = self
                .chain
                .compile_graph(
                    declaration,
                    &mut self.pool,
                    &access,
                    &mut self.backend,
                    callbacks.len(),
                );
            let mut prepared = prepared.map_err(crate::Error::from)?;
            let reservation = (|| {
                validate(&prepared.compiled)?;
                let capacity = prepared.compiled.snapshot.batches.len();
                self.backend.receipt_storage
                    .as_mut()
                    .expect("receipt storage initialized")
                    .prepare(&prepared.compiled.snapshot, capacity)
            })();
            match reservation {
                Ok(slot) => Ok((prepared, slot)),
                Err(error) => {
                    graph::reclaim_failed_resources(
                        &mut prepared.compiled.tables.registry,
                        &prepared.compiled.declarations.resources,
                        &prepared.compiled.tables.snapshot.pool_hits,
                        &mut self.pool,
                        &mut self.backend,
                    );
                    Err(error)
                }
            }
        })();
        match result {
            Ok((prepared, receipt_slot)) => Ok(PreparedSessionFrame {
                graph_id,
                generation: self.backend.device.generation(),
                prepared,
                callbacks,
                retire,
                receipt_slot,
            }),
            Err(error) => {
                self.result_cache.abort(graph_id);
                if error.is_device_lost() {
                    self.backend.device.queues.timelines.mark_device_lost();
                }
                let _ = run_retire_callbacks(retire);
                Err(error)
            }
        }
    }

    /// Compile and execute through the session frame protocol.
    pub fn execute_frame(
        &mut self,
        built: graph::BuiltGraph,
    ) -> crate::Result<SessionFrameReceipt> {
        let prepared = self.compile_frame(built)?;
        self.execute_prepared(prepared)
    }

    pub fn cancel_prepared(&mut self, mut frame: PreparedSessionFrame) -> crate::Result<()> {
        if self.active_prepared != Some(frame.graph_id)
            || frame.generation != self.backend.device.generation()
        {
            return Err(crate::Error::invalid_usage(
                "compiled frame belongs to another session",
            ));
        }
        self.active_prepared = None;
        self.result_cache.abort(frame.graph_id);
        self.cancel_presentation_of(frame.graph_id);
        graph::reclaim_failed_resources(
            &mut frame.prepared.compiled.tables.registry,
            &frame.prepared.compiled.declarations.resources,
            &frame.prepared.compiled.tables.snapshot.pool_hits,
            &mut self.pool,
            &mut self.backend,
        );
        drop(frame.callbacks);
        let result = run_retire_callbacks(frame.retire);
        if let Some(named) = &mut self.named {
            named.state.abandon_frame();
            named.graph_id = None;
        }
        self.collect_frame_imports();
        result
    }
}
