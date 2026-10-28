//! Frame execution, completion and retirement.
use std::sync::Arc;
use crate::rg::graph;
use super::PendingSessionFrame;
use super::PreparedSessionFrame;
use super::SessionFrameReceipt;
use super::VulkanGraphSession;
use super::keep_first_error;
use super::run_retire_callbacks;
use super::super::resource_spec::GpuResource;

impl VulkanGraphSession {
    pub fn execute_prepared(
        &mut self,
        frame: PreparedSessionFrame,
    ) -> crate::Result<SessionFrameReceipt> {
        if self.active_prepared != Some(frame.graph_id)
            || frame.generation != self.backend.device.generation()
        {
            return Err(crate::Error::invalid_usage(
                "compiled frame belongs to another session",
            ));
        }
        if self.is_poisoned() {
            self.cancel_prepared(frame)?;
            return Err(crate::Error::DeviceLost);
        }
        if let Err(error) = self.begin_presentation_execution(frame.graph_id) {
            self.cancel_prepared(frame)?;
            return Err(error);
        }
        self.active_prepared = None;
        let PreparedSessionFrame {
            graph_id,
            prepared,
            mut callbacks,
            retire,
            receipt_slot,
            ..
        } = frame;
        self.last_execution_partial = false;
        self.backend.submissions.clear();
        if self.ledger.has_ended() {
            self.ledger.begin();
        }
        let access = Arc::clone(&self.backend.accesses);
        self.backend.active_receipt_slot = Some(receipt_slot);
        let executed = self.chain.execute_prepared_validated(
            prepared,
            &mut self.pool,
            &access,
            &mut self.backend,
            &mut callbacks,
            |_| Ok(()),
        );
        let receipt_slot = self.backend.active_receipt_slot.take().expect("active frame receipt lease");
        self.finish_presentation_execution();
        let result = (|| match executed {
            Ok(execution) => {
                let frame = self.next_frame;
                self.next_frame += 1;
                let storage = receipt_slot.publish(execution.transaction.receipts());
                let receipt = SessionFrameReceipt {
                    frame,
                    storage,
                    stats: execution.stats,
                };
                self.end_frame(PendingSessionFrame {
                    failed: false,
                    frame,
                    display: self.current_display,
                    execution,
                    retire,
                    started_at: std::time::Instant::now(),
                });
                Ok(receipt)
            }
            Err(error) => {
                let (error, deferred) = match error {
                    crate::rg::executor::Error::PartialSubmission {
                        original,
                        execution,
                    } => (crate::Error::from(*original), Some(*execution)),
                    error => (crate::Error::from(error), None),
                };
                self.device_lost =
                    error.is_device_lost() || self.backend.device.queues.timelines.is_device_lost();
                if self.device_lost {
                    self.backend.device.queues.timelines.mark_device_lost();
                }
                if let Some(execution) = deferred {
                    self.last_execution_partial = true;
                    let frame = self.next_frame;
                    self.next_frame += 1;
                    self.end_frame(PendingSessionFrame {
                        failed: true,
                        frame,
                        display: self.current_display,
                        execution,
                        retire,
                        started_at: std::time::Instant::now(),
                    });
                    return Err(error);
                }
                self.backend.clear_pass_resources();
                self.recycle_completed_commands();
                if let Err(cleanup) = self.backend.collect_completed_managed() {
                    log::warn!("失败帧清理失败: {cleanup}");
                }
                let _ = run_retire_callbacks(retire);
                Err(error)
            }
        })();
        match &result {
            Ok(receipt) if !receipt.receipts.is_empty() => {
                self.result_cache.commit(graph_id);
                if let Some(named) = &mut self.named {
                    named.state.confirm_exports();
                }
            }
            _ => {
                self.result_cache.abort(graph_id);
                if let Some(named) = &mut self.named {
                    if self.last_execution_partial {
                        named.state.invalidate_submitted_frame();
                    } else {
                        named.state.abandon_frame();
                    }
                }
            }
        }
        // The declaration was consumed regardless of submit success. Pending
        // GPU work and committed history have their own independent owners.
        if let Some(named) = &mut self.named {
            named.graph_id = None;
        }
        // Recording has transferred every used physical resource to command
        // owners, including a partially submitted prefix. The declaration's
        // token lookup can be reclaimed, except exports which post-submit
        // consumers (screenshots, presentation) still resolve by receipt.
        let exports = result.as_ref().map_or(&[][..], |receipt| receipt.snapshot.exports.as_slice());
        self.collect_recorded_imports(Some((graph_id, exports)));
        result
    }

    fn recycle_completed_commands(&mut self) {
        while let Some(command) = self.chain.recycle() {
            if let Some(raw) = self.backend.commands.get_mut(&command.handle()) {
                unsafe { raw.release_completed_owners() };
            }
            self.backend.recycled_commands.push(command.handle());
        }
    }

    fn end_frame(&mut self, pending: PendingSessionFrame) {
        // A frame must contain an actual submitted receipt before completion is published.
        // 这里的 execute_frame 只在有回执时才到这一步，空图（无 pass）可能没有回执，所以只告警不 panic。
        if !pending.execution.transaction.receipts().is_empty() {
            self.ledger.note_submission();
        }
        if let Err(error) = self.ledger.end() {
            log::warn!("end_frame 账本异常: {error:?}");
        }
        self.pending.end_frame(pending);
    }

    pub(super) fn finish_frame(&mut self, pending: PendingSessionFrame) -> crate::Result<()> {
        let mut first_error = None;
        keep_first_error(
            &mut first_error,
            self.backend.poll_gpu_timings().map(|_| ()),
        );
        for point in pending.execution.transaction.receipts() {
            self.chain.observe_completion(*point);
        }
        // 真正完成后才把 Command 交回后端；下次 reset/重录时仍使用递增的 per-queue counter。
        self.recycle_completed_commands();
        keep_first_error(&mut first_error, self.backend.collect_completed_managed());
        let mut compiled = pending.execution.compiled;
        let completed = match self.backend.device.poll() {
            Ok(completed) => Some(completed),
            Err(error) => { keep_first_error(&mut first_error, Err(error)); None }
        };
        if pending.failed || completed.is_none() {
            graph::reclaim_failed_resources(
                &mut compiled.tables.registry,
                &compiled.declarations.resources,
                &compiled.tables.snapshot.pool_hits,
                &mut self.pool,
                &mut self.backend,
            );
        } else {
            let completed = completed.expect("completion queried before cache publication");
            // A sorted vector instead of a tree: one allocation, no node churn.
            let mut created = compiled
                .declarations
                .resources
                .iter()
                .enumerate()
                .filter(|(_, r)| matches!(r.source, graph::OriginSource::Create))
                .filter_map(|(i, _)| compiled.snapshot.physical_mapping[i].token)
                .collect::<Vec<u64>>();
            created.sort_unstable();
            created.dedup();
            // 只有本图创建的非导出资源加入 Pool；外部导入不被误当作可任意覆盖的暂存资源。
            let registry = &mut compiled.tables.registry;
            let mut exported: Vec<_> = registry.exported_logical_handles.iter()
                .filter_map(|logical| registry.resources.get(*logical as usize).map(|mapping| mapping.backing_index as usize)).collect();
            exported.sort_unstable(); exported.dedup();
            for (physical, resource) in registry.backing_resources.drain(..).enumerate() {
                if exported.binary_search(&physical).is_ok() { continue; }
                match resource {
                    crate::rg::resource_registry::RegistryResource::OwnedBuffer(buffer)
                        if created.binary_search(&buffer.handle).is_ok() =>
                    {
                        if graph::ResourceBackend::is_transient_alias(&self.backend, buffer.handle)
                        {
                            graph::ResourceBackend::release_created_resource(
                                &mut self.backend,
                                buffer.handle,
                            );
                            continue;
                        }
                        if let Some(GpuResource::Buffer(actual)) =
                            self.backend.resources.get(&buffer.handle)
                        {
                            self.backend
                                .device
                                .memory
                                .stats
                                .mark_cached(actual.allocation_id(), &completed);
                        }
                        let token = buffer.handle;
                        self.pool.insert_buffer(buffer);
                        graph::ResourceBackend::park_pooled(&mut self.backend, token);
                    }
                    crate::rg::resource_registry::RegistryResource::OwnedTexture(texture)
                        if created.binary_search(&texture.handle).is_ok() =>
                    {
                        if graph::ResourceBackend::is_transient_alias(&self.backend, texture.handle)
                        {
                            graph::ResourceBackend::release_created_resource(
                                &mut self.backend,
                                texture.handle,
                            );
                            continue;
                        }
                        if let Some(GpuResource::Image(actual)) =
                            self.backend.resources.get(&texture.handle)
                        {
                            self.backend
                                .device
                                .memory
                                .stats
                                .mark_cached(actual.allocation_id(), &completed);
                        }
                        let token = texture.handle;
                        self.pool.insert_texture(texture);
                        graph::ResourceBackend::park_pooled(&mut self.backend, token);
                    }
                    _ => {}
                }
            }
        }
        keep_first_error(&mut first_error, run_retire_callbacks(pending.retire));
        self.collect_frame_imports();
        self.last_frame_duration_ns = pending.started_at.elapsed().as_nanos() as u64;
        first_error.map_or(Ok(()), Err)
    }

    /// Reclaim only frames whose actual queue timelines are already complete.
    pub fn collect_completed_frames(&mut self) -> crate::Result<usize> {
        let completed = self.backend.device.poll()?;
        let mut count = 0;
        while let Some(pending) = self.pending.front() {
            if !pending.execution.completion().is_reached(&completed) {
                break;
            }
            let frame = pending.frame;
            self.wait_frame(frame, 0)?;
            count += 1;
        }
        self.collect_cached_results(&completed)?;
        self.collect_frame_imports();
        Ok(count)
    }

    pub fn wait_frame(&mut self, frame: u64, timeout: u64) -> crate::Result<()> {
        let pending = self
            .pending
            .front()
            .ok_or_else(|| crate::Error::invalid_usage("没有待完成帧"))?;
        if pending.frame != frame {
            return Err(crate::Error::invalid_usage("帧回执不属于当前待完成帧"));
        }
        // 等待失败（含超时）时帧仍在队头，调用方可重试。
        self.backend
            .device
            .queues
            .timelines
            .wait(&pending.execution.completion(), timeout)?;
        let pending = self.pending.pop_front().unwrap();
        self.finish_frame(pending)
    }

    /// 显式淘汰缓存，必须在当前帧完成后调用；保持 counter/timeline，避免 resize 残留无限增长。
    pub fn collect_after_completion(
        &mut self,
        retained_tokens: &[u64],
    ) -> Result<usize, crate::Error> {
        if self.active_prepared.is_some() {
            return Err(crate::Error::invalid_usage(
                "a compiled frame still owns its physical resources",
            ));
        }
        if !self.pending.is_empty() {
            return Err(crate::Error::invalid_usage(
                "待完成帧仍拥有资源，不能清理缓存",
            ));
        }
        self.backend.device.wait_idle()?;
        self.result_cache.abandon_pending();
        self.result_cache.clear();
        self.collect_cached_results(&crate::sync::CompletedValues::all())?;
        // 缩放/收尾级等待：idle 后推进并回收提交引擎。
        self.backend.clear_pass_resources();
        for command in self.backend.commands.values_mut() {
            unsafe { command.release_completed_owners() };
        }
        let removed = self
            .backend
            .resources
            .keys()
            .copied()
            .filter(|token| {
                !retained_tokens.contains(token)
                    && !self
                        .named
                        .as_ref()
                        .is_some_and(|named| named.state.owns_token(*token))
            })
            .collect::<Vec<_>>();
        self.pool.clear();
        let parked = self.backend.caches.clear_idle();
        for token in &removed {
            self.backend.resources.remove(token);
            self.backend.import_families.remove(token);
        }
        self.backend.device.try_collect_all_completed()?;
        Ok(removed.len() + parked)
    }

    pub fn clear_after_completion(&mut self) -> Result<usize, crate::Error> {
        self.collect_after_completion(&[])
    }

    /// 有界等候队头；即使完成也不弹出，退役只在 begin_frame/wait_frame 发生。
    pub fn wait_gpu_queue_trim(&self, timeout: u64) -> Result<bool, crate::Error> {
        let Some(pending) = self.pending.front() else {
            return Ok(true);
        };
        match self
            .backend
            .device
            .queues
            .timelines
            .wait(&pending.execution.completion(), timeout)
        {
            Ok(()) => Ok(true),
            Err(crate::Error::Timeout) => Ok(false),
            Err(error) => Err(error),
        }
    }

    pub fn pending_frame_count(&self) -> usize {
        self.pending.in_flight()
    }
}
