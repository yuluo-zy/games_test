//! Batch execution: recording, submission and result collection.
use ash::vk;
use ash::vk::Handle;
use crate::command::command_buffer::VulkanCommandBuffer;
use crate::command::recording;
use crate::device::semaphore::TimelineSemaphore;
use crate::native::submit;
use crate::rg::executor::ExecutionBackend;
use crate::rg::executor::GraphBatchSync;
use crate::rg::executor::GraphTimelineWait;
use crate::rg::graph;
use std::sync::Arc;
use super::FrameCommandPool;
use super::FramePoolKey;
use super::VulkanGraphBackend;
use super::chunk_recording::VulkanChunkView;
use super::chunk_recording::add_transition;
use super::super::capture::GpuSubmissionCapture;
use super::super::capture::SUBMISSION_CAPTURE_LIMIT;
use super::super::resource_spec::GpuResource;

impl ExecutionBackend for VulkanGraphBackend {
    fn reserve_pass_timestamp(
        &mut self,
        command: u64,
        pass: usize,
        name: &crate::resource::ResourceName,
    ) -> Option<crate::rg::profiling::GpuPassTimestamp> {
        self.gpu_timing.as_mut()?.reserve_pass(command, pass, name)
    }
    unsafe fn write_pass_timestamp(
        &self,
        command: u64,
        pair: crate::rg::profiling::GpuPassTimestamp,
        end: bool,
    ) {
        pair.write(
            &self.device.core.device,
            vk::CommandBuffer::from_raw(command),
            end,
        );
    }
    fn owner_storage_pool(
        &self,
    ) -> Option<&crate::collections::temp_list_pool::TempListPool<crate::command::binding::RetainedResource>>
    {
        Some(&self.owner_pool)
    }

    fn requires_graph_completion(&self, batch: usize, total: usize) -> bool {
        batch + 1 == total && !self.graph_signals.is_empty()
    }
    fn select_queue_domain(
        &mut self,
        domain: crate::rg::queue_schedule::QueueDomain,
    ) -> crate::Result<()> {
        let family = if domain.queue_index == 0 {
            self.device.queues.primary_family
        } else {
            self.device
                .queues
                .additional
                .get(domain.queue_index as usize - 1)
                .ok_or_else(|| crate::Error::invalid_usage("scheduled queue does not exist"))?
                .family
        };
        if domain.family != family {
            return Err(crate::Error::invalid_usage(
                "scheduled queue family differs from actual queue",
            ));
        }
        self.recording_family = family;
        self.recording_queue =
            crate::sync::QueueId::new(domain.queue_index as usize).ok_or_else(|| {
                crate::Error::invalid_usage("scheduled queue index exceeds device limits")
            })?;
        Ok(())
    }
    fn next_queue_prelude(
        &mut self,
        compiled: &graph::FrameInstance,
        batch: usize,
    ) -> crate::Result<Option<crate::rg::executor::QueuePrelude>> {
        let prelude = self.next_prelude(compiled, batch)?;
        if let Some(prelude) = &prelude {
            self.select_presentation_batch(prelude.tag as usize);
        }
        Ok(prelude)
    }
    fn record_queue_prelude(
        &mut self,
        command: u64,
        prelude: crate::rg::executor::QueuePrelude,
    ) -> crate::Result<()> {
        self.record_prelude(command, prelude)
    }
    fn select_main_batch(
        &mut self,
        _compiled: &graph::FrameInstance,
        batch: usize,
    ) -> crate::Result<()> {
        self.queue_runtime.select_main_batch(batch)?;
        self.select_presentation_batch(batch);
        Ok(())
    }
    fn timeline_wait(&self, point: crate::sync::SyncPoint) -> crate::Result<GraphTimelineWait> {
        self.queue_timeline_wait(point)
    }
    fn cpu_arena_pool(&self) -> Option<&crate::command::CpuArenaPool> {
        Some(&self.cpu_arenas)
    }
    fn plan_context(&self) -> crate::rg::plan::PlanContext {
        crate::rg::plan::PlanContext {
            queue_domains: Some(
                crate::device::QueueClass::ALL.map(|class| self.domain_for_class(class)),
            ),
            device_generation: self.device.generation().value(),
            queue_family: self.family,
            queue_index: match self.queue {
                recording::Queue::Primary => 0,
                recording::Queue::Auxiliary(i) => i + 1,
            },
            policy_signature: u64::from(self.transient_aliasing),
            // Only the actually enabled feature lets depth and stencil be
            // tracked (and transitioned) independently; otherwise the planner
            // merges partial D/S ranges and uses combined layouts.
            separate_depth_stencil_layouts: self
                .device
                .caps()
                .is_enabled(crate::device::Feature::SeparateDepthStencilLayouts),
        }
    }
    fn prepare_declaration(
        &mut self,
        declaration: &graph::GraphDeclaration,
    ) -> crate::Result<()> {
        let result = self.validate_graph(declaration)
            .and_then(|()| self.begin_command_frame());
        if result.as_ref().is_err_and(|error| error.is_device_lost()) {
            self.device.queues.timelines.mark_device_lost();
        }
        result
    }
    fn synchronization_plan(&self) -> Arc<crate::rg::sync_plan::SynchronizationPlanner> {
        self.queue_runtime.planner()
    }
    fn prepare_batch_submission(&mut self, commands: &[u64], batch: usize) -> crate::Result<()> {
        self.attach_range_patches(commands, batch)
    }
    fn prepare_compiled_resources(&mut self, compiled: &graph::FrameInstance) -> crate::Result<()> {
        self.bind_queue_resources(compiled)?;
        self.resolve_presentation(compiled)?;
        if let Some(slot) = &mut self.active_receipt_slot {
            slot.reserve_points(self.queue_runtime.planner().bound().batches().len())?;
        }
        // Commands own GPU resources; external Arc/Weak owners pin the snapshot.
        let mut projection = self.caches.projection();
        projection.snapshot.take();
        projection.snapshot = Some(projection.pool.checkout(
            &self.device,
            &compiled.registry,
            &self.stores,
            Arc::as_ptr(&compiled.plan) as usize,
            |token| self.resources.get(&token).cloned(),
        )?);
        Ok(())
    }

    fn managed_pass_resources(&self) -> Option<Arc<crate::rg::pass_bindings::ManagedPassResources>> {
        self.caches.projection().snapshot.clone()
    }
    fn retain_command_owners(
        &mut self,
        handle: u64,
        owners: &mut crate::rg::owner_storage::ResourceOwners,
    ) -> crate::Result<()> {
        self.commands.get_mut(&handle)
            .ok_or_else(|| crate::Error::invalid_usage("retained command is missing"))?
            .retain_resources(owners.drain())
    }

    fn graph_queue(&self) -> (recording::Queue, u32) {
        (self.queue, self.family)
    }
    fn batch_synchronization(&self, index: usize, total: usize) -> GraphBatchSync {
        let mut synchronization = GraphBatchSync::default();
        let explicit = self.synchronization.get(&index);
        let waits = explicit.into_iter().flat_map(|s| s.waits.iter()).chain(
            self.graph_waits
                .iter()
                .filter(|_| index == 0 || self.queue_runtime.multi_queue),
        );
        let signals = explicit
            .into_iter()
            .flat_map(|s| s.signals.iter())
            .chain(self.graph_signals.iter().filter(|_| index + 1 == total));
        synchronization.waits = waits
            .map(|(semaphore, value, stage)| GraphTimelineWait {
                semaphore: semaphore.handle,
                value: *value,
                stage: *stage,
            })
            .collect();
        synchronization.signals = signals
            .map(|(semaphore, value)| crate::sync::timeline::SubmitSignal {
                semaphore: semaphore.handle,
                value: *value,
            })
            .collect();
        synchronization
    }
    fn command_handle(&mut self) -> crate::Result<u64> {
        self.command_for_worker(0)
    }
    fn chunk_command_handle(&mut self, chunk: usize) -> crate::Result<u64> {
        // Chunk 0 reuses the initial command from pool 0. Remaining chunks
        // lease distinct pools; the owner never records while they are lent.
        self.command_for_worker(chunk + 1)
    }
    fn drain_pending_acquires(&mut self, handle: u64) -> crate::Result<u32> {
        let Some(command) = self.commands.get_mut(&handle) else {
            return Err(crate::Error::invalid_usage("batch command is missing"));
        };
        let (resources, tokens) = (&self.resources, self.queue_runtime.frame_tokens());
        // Resources of this frame are acquired by the planner itself.
        let drained = self
            .device
            .record_pending_acquires_except(command, |cell| {
                tokens.iter().flatten().any(|token| {
                    resources
                        .get(token)
                        .is_some_and(|resource| Arc::ptr_eq(&resource.owner_cell(), cell))
                })
            })?;
        if drained.is_empty() {
            return Ok(0);
        }
        let count = drained.len() as u32;
        self.drained_acquires
            .try_reserve(1)
            .map_err(|_| crate::Error::OutOfHostMemory)?; // drained Drop restores its prepaid slots.
        self.drained_acquires.push((handle, drained));
        Ok(count)
    }
    fn abandon_command(&mut self, handle: u64) {
        if let Some(position) = self
            .drained_acquires
            .iter()
            .position(|(command, _)| *command == handle)
        {
            let (_, drained) = self.drained_acquires.swap_remove(position);
            self.device.requeue_pending_acquires(drained);
        }
        if let Some(ring) = &mut self.gpu_timing {
            ring.abandon(handle);
        }
        self.barriers.remove(handle);
        let Some(command) = self.commands.get_mut(&handle) else {
            return;
        };
        // Unsubmitted recording state is discarded by the next whole-pool
        // reset. Keeping its owners until then also preserves reset failure state.
        if !command.has_submissions() && !self.recycled_commands.contains(&handle) {
            self.recycled_commands.push(handle);
        }
    }
    fn chunk_recording(
        &mut self,
        commands: &[u64],
    ) -> crate::Result<crate::rg::executor::ChunkRecording<'_>> {
        // Each lease exclusively borrows a distinct frame pool. Chunk zero
        // owns the initial command; its owner performs no recording while the
        // worker scope runs. Other chunks own independent pools.
        let present_patch = self.present_patch();
        let mut leases = smallvec::SmallVec::<[crate::rg::executor::ChunkLease<'_>; 8]>::new();
        let mut entries = smallvec::SmallVec::<[(&FramePoolKey, &mut FrameCommandPool); 8]>::new();
        let frame_slot = self.active_frame_slot;
        let family = self.recording_family;
        for (key, entry) in self.frame_pools.iter_mut() {
            if key.frame_slot == frame_slot && key.family == family {
                entries.push((key, entry));
            }
        }
        for &command in commands {
            let key = self
                .command_slot
                .get(&command)
                .ok_or_else(|| crate::Error::invalid_usage("parallel command has no frame pool"))?;
            let position = entries
                .iter()
                .position(|(candidate, _)| *candidate == key)
                .ok_or_else(|| {
                    crate::Error::invalid_usage(
                        "parallel commands must each own a distinct command pool",
                    )
                })?;
            let (_, entry) = entries.swap_remove(position);
            if !self
                .commands
                .get(&command)
                .is_some_and(|recorded| Arc::ptr_eq(&recorded.pool, &entry.pool))
            {
                return Err(crate::Error::invalid_usage(
                    "parallel command was not allocated from its leased pool",
                ));
            }
            // SAFETY: `command` belongs to this exclusively borrowed pool
            // entry, which no other lease or recording command shares.
            leases.push(unsafe { crate::rg::executor::ChunkLease::new(entry, command) });
        }
        let view: Box<dyn crate::rg::executor::ChunkRecorder + '_> = Box::new(VulkanChunkView {
            present_patch,
            barrier_device: &self.device,
            barrier_dispatch: self.barrier_dispatch,
            cpu_arenas: &self.cpu_arenas,
            owner_pool: &self.owner_pool,
            device: &self.device.core.device,
            resources: &self.resources,
        });
        Ok((view, leases))
    }
    fn begin_command(&mut self, handle: u64) -> crate::Result<()> {
        self.retained_tokens.clear();
        self.retained_command = handle;
        self.commands.get_mut(&handle).expect("owned command").begin()?;
        if let Some(ring) = &mut self.gpu_timing {
            ring.begin(
                self.commands.get_mut(&handle).expect("owned command"),
                self.recording_queue,
                self.recording_epoch,
            )?;
        }
        Ok(())
    }

    fn end_command(&mut self, handle: vk::CommandBuffer) -> crate::Result<()> {
        self.retained_tokens.clear();
        self.flush_barriers(handle.as_raw())?;
        if let Some(ring) = &mut self.gpu_timing {
            ring.end(handle);
        }
        self.commands.get_mut(&handle.as_raw())
            .ok_or_else(|| crate::Error::invalid_usage("ending command is missing"))?
            .end()
    }

    fn flush_barriers(&mut self, handle: u64) -> crate::Result<()> {
        let Some(scope) = self.barriers.remove(handle) else {
            return Ok(());
        };
        if !self.commands.contains_key(&handle) {
            return Err(crate::Error::invalid_usage("barrier command is missing"));
        }
        if !scope.is_empty() {
            unsafe {
                self.barrier_dispatch.issue(
                    &self.device,
                    vk::CommandBuffer::from_raw(handle),
                    &scope,
                )
            }?;
        }
        Ok(())
    }

    fn transition(
        &mut self,
        command: u64,
        token: u64,
        transition: &crate::rg::sync_plan::BoundTransition,
    ) -> crate::Result<()> {
        let Some(recording) = self.commands.get_mut(&command) else {
            return Err(crate::Error::invalid_usage("transition command is missing"));
        };
        if self.retained_command != command {
            self.retained_tokens.clear();
            self.retained_command = command;
        }
        let first_in_command = match self.retained_tokens.binary_search(&token) {
            Ok(_) => false,
            Err(at) => {
                self.retained_tokens.insert(at, token);
                true
            }
        };
        // SAFETY: this operand is exactly covered by the graph's bound native
        // transition and by the batch state patch attached before submission.
        let retained = if first_in_command {
            unsafe {
                match self.resources.get(&token) {
                    Some(GpuResource::Buffer(buffer)) => recording.retain_recorded(buffer.clone()),
                    Some(GpuResource::Image(image)) => recording.retain_recorded(image.clone()),
                    None => return Err(crate::Error::invalid_usage("transition resource is missing")),
                }
            }
        } else {
            Ok(())
        };
        if let Err(error) = retained {
            // Not retained: the next transition for this token must retry.
            if let Ok(at) = self.retained_tokens.binary_search(&token) {
                self.retained_tokens.remove(at);
            }
            return Err(error);
        }
        let present = self.present_patch();
        add_transition(
            self.barriers.entry(command),
            &self.resources,
            token,
            transition,
            present,
        )
    }
    fn submit(
        &mut self,
        queue: vk::Queue,
        prepared: &[submit::PreparedSubmit],
    ) -> crate::Result<crate::sync::SyncPoint> {
        if self.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        let mut trace = crate::trace::SubmitTrace::new("rg_submit");
        trace.count("batches", prepared.len());
        let mut capture = GpuSubmissionCapture::try_new(
            queue,
            prepared,
            self.device.queues.timelines.queue_count().saturating_sub(1),
            &self.device.memory.stats,
        )?;
        let (present_role, fence) = self.amend_presentation_submit(&mut capture.batches)?;
        if present_role.waits_acquire || present_role.signals_rendered {
            capture.refresh_budget(&self.device.memory.stats)?;
        }
        if self.submissions.len() < SUBMISSION_CAPTURE_LIMIT {
            self.submissions
                .try_reserve(1)
                .map_err(|_| crate::Error::OutOfHostMemory)?;
        }
        let command_count = prepared.iter().try_fold(0usize, |count, batch| {
            count
                .checked_add(batch.commands.len())
                .ok_or_else(|| crate::Error::invalid_usage("submission command count overflows"))
        })?;
        let command_refs = submit::SubmitScratch::<[&VulkanCommandBuffer; 8]>::try_collect(
            command_count,
            &self.device.memory.stats,
            prepared
                .iter()
                .flat_map(|batch| &batch.commands)
                .map(|command| {
                    self.commands.get(&command.as_raw()).ok_or_else(|| {
                        crate::Error::invalid_usage("submitted command is absent from backend")
                    })
                }),
        )?;
        trace.mark("capture_and_command_refs");
        crate::command::command_buffer::validate_submission_commands(&command_refs)?;
        trace.mark("validate1");
        let semaphore_refs = self
            .graph_waits
            .iter()
            .map(|(s, _, _)| s)
            .chain(self.graph_signals.iter().map(|(s, _)| s))
            .chain(self.synchronization.values().flat_map(|sync| {
                sync.waits
                    .iter()
                    .map(|(s, _, _)| s)
                    .chain(sync.signals.iter().map(|(s, _)| s))
            }))
            .filter(|s| {
                prepared.iter().any(|batch| {
                    batch.wait_semaphores.contains(&s.handle)
                        || batch.signal_semaphores.contains(&s.handle)
                })
            });
        let semaphores = submit::SubmitScratch::<[Arc<TimelineSemaphore>; 8]>::try_collect(
            semaphore_refs.clone().count(),
            &self.device.memory.stats,
            semaphore_refs.map(|semaphore| Ok(semaphore.clone())),
        )?;
        trace.mark("semaphore_refs");
        let use_sources =
            submit::SubmitScratch::<[crate::tracking::tracked::OrderedSubmissionUses<'_>; 8]>::try_collect(
                command_refs.len(),
                &self.device.memory.stats,
                command_refs.iter().map(|command| command.submission_uses()),
            )?;
        let uses = crate::tracking::tracked::collect_submission_uses(
            &use_sources,
            &self.device.memory.stats,
        )?;
        drop(use_sources);
        trace.mark("uses_merge");
        trace.count("commands", command_refs.len());
        trace.count("unique_uses", uses.len());
        let _reservations =
            crate::tracking::tracked::lock_submission_uses(uses.ordered(), &self.device.memory.stats)?;
        trace.mark("state_locks");
        let family = self.recording_family;
        let prepared_states = unsafe {
            crate::command::command_buffer::prepare_submission_states(
                &command_refs,
                &uses,
                self.device.queue_id(queue)?,
                family,
                &self.device.memory.stats,
            )?
        };
        trace.mark("prepare_states");
        drop(command_refs);
        // Declared (planner-bound) accesses carry their first-use stages;
        // opaque uses inside the commands keep ALL_COMMANDS.
        let mut dependencies = prepared_states.staged_dependencies();
        dependencies.merge(&self.queue_submission_dependencies()?);
        self.device
            .add_staged_timeline_waits(queue, &mut capture.batches, &dependencies)?;
        trace.mark("dependencies");
        let result = self.device.submit_prepared(queue, &capture.batches, fence);
        trace.mark("device_submit_prepared");
        if let Ok(point) = result {
            // Infallible after the irreversible queue call: the owner only
            // records the receipt it must now protect.
            if let Err(error) = self.presentation_submitted(present_role, point) {
                log::error!(target: "rhapsody::rg::presentation", "presentation bookkeeping failed after submit: {error}");
            }
            prepared_states.publish(point);
            self.queue_submitted(point);
            // Coalesced acquires in these commands are now committed.
            self.drained_acquires.retain_mut(|(command, drained)| {
                let submitted = prepared.iter()
                    .any(|batch| batch.commands.iter().any(|c| c.as_raw() == *command));
                if submitted { drained.submitted(); }
                !submitted
            });
            for semaphore in semaphores.iter() {
                semaphore.last_use.insert_overall(point);
            }
            for batch in prepared {
                for command in &batch.commands {
                    // All entries were validated before the irreversible queue call.
                    if let Some(ring) = &mut self.gpu_timing {
                        ring.submitted(command.as_raw(), point);
                    }
                    self.commands
                        .get_mut(&command.as_raw())
                        .expect("validated command")
                        .record_submission(point);
                }
            }
        }
        capture.result = match &result {
            Ok(_) => 0,
            Err(crate::Error::DeviceLost) => vk::Result::ERROR_DEVICE_LOST.as_raw(),
            Err(_) => vk::Result::ERROR_UNKNOWN.as_raw(),
        };
        if self.submissions.len() >= SUBMISSION_CAPTURE_LIMIT {
            self.submissions.pop_front();
        }
        self.submissions.push_back(capture);
        result
    }
}
