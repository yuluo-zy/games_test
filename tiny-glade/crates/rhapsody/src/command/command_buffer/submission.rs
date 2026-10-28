//! Submission validation and queue submit.
use ash::vk;
use crate::trace::SubmitTrace;
use std::sync::Arc;
use super::VulkanCommandBuffer;
use super::recorded_state::OpaqueUseRange;
use super::recorded_state::order_recorded_patches;

/// All capacity and preconditions are prepared while the caller holds the
/// resource gates. Failure cancels every staged cell; success publishes once
/// per physical object with the same real timeline point as command retirement.
pub(crate) struct PreparedResourceStates {
    pub(super) cells: crate::native::submit::SubmitScratch<[Arc<crate::tracking::SyncCell>; 32]>,
    pub(super) queue: crate::sync::QueueId,
    pub(super) dependencies: crate::sync::StagedDependencies,
    pub(super) published: bool,
}

impl PreparedResourceStates {
    /// Prerequisites without stage information (conservative consumers).
    pub(crate) fn dependencies(&self) -> crate::sync::SyncSet {
        self.dependencies.points()
    }
    /// Prerequisites with the destination stages of the declared accesses
    /// that need them; opaque uses carry `ALL_COMMANDS`.
    pub(crate) fn staged_dependencies(&self) -> crate::sync::StagedDependencies {
        self.dependencies
    }
    pub(super) fn prepare_cell(
        &mut self,
        index: usize,
        patches: &[crate::tracking::range_state::StatePatch],
        family: u32,
        stats: &Arc<crate::accounting::AllocatorStats>,
    ) -> crate::Result<()> {
        let cell = &self.cells[index];
        cell.prepare_range_state(patches, self.queue, family, stats)?;
        self.dependencies
            .merge(&cell.prepared_range_staged_dependencies());
        Ok(())
    }
    pub(crate) fn publish(mut self, point: crate::sync::SyncPoint) {
        assert_eq!(
            point.queue, self.queue,
            "native state and queue completion must match"
        );
        for cell in self.cells.iter() {
            cell.publish_range_state(point);
        }
        self.published = true;
    }
}

impl Drop for PreparedResourceStates {
    fn drop(&mut self) {
        if !self.published {
            for cell in self.cells.iter() {
                cell.cancel_range_state();
            }
        }
    }
}

/// # Safety
/// The submission gates for all `uses` must remain held until this guard is
/// published or dropped. `commands` are in actual native execution order.
pub(crate) unsafe fn prepare_submission_states(
    commands: &[&VulkanCommandBuffer],
    uses: &[Arc<crate::tracking::SyncCell>],
    queue: crate::sync::QueueId,
    family: u32,
    stats: &Arc<crate::accounting::AllocatorStats>,
) -> crate::Result<PreparedResourceStates> {
    use crate::tracking::range_state::{PatchAction, StatePatch};
    let mut trace = SubmitTrace::new("native_state_prepare");
    trace.count("commands", commands.len());
    trace.count("uses", uses.len());
    if trace.enabled() {
        trace.count(
            "state_patches",
            commands
                .iter()
                .map(|command| command.state_patches.as_slice().len())
                .sum(),
        );
        trace.count(
            "opaque_ranges",
            commands
                .iter()
                .map(|command| command.range_uses.as_slice().len())
                .sum(),
        );
    }
    // Plan-private graph resources carry no native state (P3): their
    // completion is still published through `uses`, their barriers came from
    // the bound plan.
    let physical = uses
        .iter()
        .filter(|cell| cell.ownership_range().is_some() && !cell.is_plan_private());
    let cells = crate::native::submit::SubmitScratch::try_collect(
        physical.clone().count(),
        stats,
        physical.map(|cell| Ok(cell.clone())),
    )?;
    let mut prepared = PreparedResourceStates {
        cells,
        queue,
        dependencies: crate::sync::StagedDependencies::EMPTY,
        published: false,
    };
    let Some(first) = commands.first() else {
        if !prepared.cells.is_empty() {
            return Err(crate::Error::invalid_usage(
                "native memory state requires recorded commands",
            ));
        }
        return Ok(prepared);
    };
    let mut scratch = crate::collections::temp_list::TempList::checkout(&first.pool.cpu_state_patches);
    // One pass buckets every recorded patch by physical cell (command order,
    // explicit before opaque inside a command), instead of re-filtering all
    // patches once per cell.
    let mut buckets = crate::collections::temp_list::TempList::checkout(&first.pool.cpu_state_buckets);
    let count = commands.iter().try_fold(0usize, |count, command| {
        count
            .checked_add(command.state_patches.as_slice().len())
            .and_then(|count| count.checked_add(command.range_uses.as_slice().len()))
            .ok_or(crate::Error::OutOfHostMemory)
    })?;
    // Submission uses are already in pointer order. Binary lookup avoids a
    // second unbudgeted HashMap; the bucket storage follows the pool's CPU
    // reservation and can be trimmed while GPU work is in flight.
    buckets.with_reserved(count, |recorded| -> crate::Result<()> {
        for (command_index, command) in commands.iter().enumerate() {
            for (entry_index, entry) in command.state_patches.as_slice().iter().enumerate() {
                if let Ok(slot) = prepared
                    .cells
                    .binary_search_by_key(&(Arc::as_ptr(&entry.cell) as usize), |cell| {
                        Arc::as_ptr(cell) as usize
                    })
                {
                    recorded.push((slot, 0, command_index, entry_index));
                }
            }
            for (entry_index, entry) in command.range_uses.as_slice().iter().enumerate() {
                if let Ok(slot) = prepared
                    .cells
                    .binary_search_by_key(&(Arc::as_ptr(&entry.cell) as usize), |cell| {
                        Arc::as_ptr(cell) as usize
                    })
                {
                    recorded.push((slot, 1, command_index, entry_index));
                }
            }
        }
        order_recorded_patches(recorded);
        for index in 0..prepared.cells.len() {
            let cell = prepared.cells[index].clone();
            let domain = cell.ownership_range().expect("filtered native domain");
            let start = recorded.partition_point(|item| item.0 < index);
            let end = recorded.partition_point(|item| item.0 <= index);
            let own = &recorded[start..end];
            let count = own.len();
            let fallback = count == 0;
            scratch.with_entries(|patches| -> crate::Result<()> {
                patches.reserve(count.max(usize::from(fallback)))?;
                for &(_, kind, command_index, entry_index) in own {
                    if kind == 0 {
                        patches.add(
                            commands[command_index].state_patches.as_slice()[entry_index].patch,
                        )?;
                    } else {
                        let entry = &commands[command_index].range_uses.as_slice()[entry_index];
                        let range = match entry.range {
                            OpaqueUseRange::Whole => domain,
                            OpaqueUseRange::Bytes { offset, size } => {
                                crate::tracking::OwnershipRange::Buffer { offset, size }
                            }
                        };
                        patches.add(StatePatch {
                            range,
                            expected_revision: u64::MAX,
                            action: PatchAction::Unknown { queue, family },
                        })?;
                    }
                }
                if fallback {
                    patches.add(StatePatch {
                        range: domain,
                        expected_revision: u64::MAX,
                        action: PatchAction::Unknown { queue, family },
                    })?;
                }
                prepared.prepare_cell(index, patches.as_slice(), family, stats)
            })?;
        }
        Ok(())
    })??;
    trace.mark("prepare_all_cells");
    trace.count("physical_cells", prepared.cells.len());
    Ok(prepared)
}

pub(crate) fn validate_submission_commands(commands: &[&VulkanCommandBuffer]) -> crate::Result<()> {
    let Some(first) = commands.first() else {
        return Ok(());
    };
    // Each command's `uses` is already sorted and unique. The previous
    // implementation also de-duplicated across commands into a scratch list
    // that nothing read, which was O(U^2) pointer comparisons plus an Arc clone
    // per cell, twice per submission.
    for command in commands {
        if let Some(error) = command.range_failure {
            return Err(error.error());
        }
        if !command.uses.is_sealed() {
            return Err(crate::Error::invalid_usage("提交命令尚未结束录制"));
        }
        if first.pool.family != command.pool.family
            || !Arc::ptr_eq(&first.pool.device, &command.pool.device)
        {
            return Err(crate::Error::invalid_usage(
                "submitted commands use different devices or queue families",
            ));
        }
        for cell in command.uses.as_slice() {
            cell.validate_device(command.pool.device.generation())?;
            cell.validate_sharing_family(command.pool.family)?;
        }
    }
    Ok(())
}

pub(super) fn record_command_uses(uses: &[Arc<crate::tracking::SyncCell>], point: crate::sync::SyncPoint) {
    for cell in uses {
        cell.insert_overall(point);
    }
}

/// A submission receipt. Dropping it transfers command and resource destruction
/// to the device timeline; it never blocks on a fence or device idle.
pub struct PendingCommand {
    command: VulkanCommandBuffer,
    pub point: crate::sync::SyncPoint,
    _semaphores: crate::native::submit::SubmitScratch<[Arc<crate::device::semaphore::TimelineSemaphore>; 8]>,
}

impl PendingCommand {
    pub fn wait(&self, timeout: u64) -> crate::Result<()> {
        self.command
            .pool
            .device
            .queues
            .timelines
            .wait(&crate::sync::SyncSet::from_point(self.point), timeout)
    }
    pub fn is_complete(&self) -> crate::Result<bool> {
        self.command
            .pool
            .device
            .queues
            .timelines
            .is_reached(&crate::sync::SyncSet::from_point(self.point))
    }
    /// The submitted command, for recycling after `point` completes.
    pub(crate) fn into_command(self) -> VulkanCommandBuffer {
        self.command
    }
}

impl VulkanCommandBuffer {
    pub(crate) fn validate_submission(&self) -> crate::Result<()> {
        validate_submission_commands(&[self])
    }

    pub(crate) fn submission_uses(
        &self,
    ) -> crate::Result<crate::tracking::tracked::OrderedSubmissionUses<'_>> {
        self.uses.ordered()
    }

    pub(crate) fn record_submission(&mut self, point: crate::sync::SyncPoint) {
        self.last_use.insert(point);
        record_command_uses(self.uses.as_slice(), point);
    }

    pub fn submit(self) -> crate::Result<PendingCommand> {
        let queue = crate::device::DeviceQueue {
            handle: self.pool.device.queues.primary,
            family: self.pool.device.queues.primary_family,
            index: 0,
        };
        self.submit_to(queue, &[], &[])
    }

    /// 显式生产时间线 wait/signal 数组，并将依赖持有到提交点登记。
    ///
    /// 提交与 PendingCommand 等待共用设备时间线。
    pub fn submit_to(
        mut self,
        queue: crate::device::DeviceQueue,
        waits: &[(
            Arc<crate::device::semaphore::TimelineSemaphore>,
            u64,
            vk::PipelineStageFlags,
        )],
        signals: &[(Arc<crate::device::semaphore::TimelineSemaphore>, u64)],
    ) -> crate::Result<PendingCommand> {
        if !self.uses.is_sealed() {
            return Err(crate::Error::invalid_usage("提交命令、队列或同步对象无效"));
        }
        self.validate_submission()?;
        let device = &self.pool.device;
        let known = (queue.handle == device.queues.primary
            && queue.family == device.queues.primary_family
            && queue.index == 0)
            || device.queues.additional.iter().any(|q| {
                q.handle == queue.handle && q.family == queue.family && q.index == queue.index
            });
        if !known
            || queue.family != self.pool.family
            || waits
                .iter()
                .any(|(s, _, stage)| !Arc::ptr_eq(&s.device, device) || stage.is_empty())
            || signals.iter().any(|(s, _)| !Arc::ptr_eq(&s.device, device))
        {
            return Err(crate::Error::invalid_usage("提交命令、队列或同步对象无效"));
        }
        let semaphore_count = waits
            .len()
            .checked_add(signals.len())
            .ok_or_else(|| crate::Error::invalid_usage("submission owner count overflows"))?;
        // The pending receipt owns the actual semaphore Arcs and their spill
        // credit together; dropping the caller's owners cannot release them.
        let semaphores = crate::native::submit::SubmitScratch::<
            [Arc<crate::device::semaphore::TimelineSemaphore>; 8],
        >::try_collect(
            semaphore_count,
            &device.memory.stats,
            waits
                .iter()
                .map(|(s, _, _)| Ok(s.clone()))
                .chain(signals.iter().map(|(s, _)| Ok(s.clone()))),
        )?;
        let wait_capacity = waits
            .len()
            .checked_add(device.queues.timelines.queue_count().saturating_sub(1))
            .ok_or_else(|| crate::Error::invalid_usage("submission wait count overflows"))?;
        let nested_bytes =
            crate::native::submit::PreparedSubmit::heap_bytes_for_counts(wait_capacity, signals.len(), 1)?;
        // Declare the ticket first: all real nested arrays drop before their
        // credit on every path. Reserve foreign waits before taking any gates.
        let mut nested_memory = if nested_bytes == 0 {
            None
        } else {
            Some(
                device
                    .memory
                    .stats
                    .reserve_cpu_bytes(nested_bytes as u64, crate::accounting::BudgetCategory::Live)?,
            )
        };
        let mut prepared = [crate::native::submit::PreparedSubmit::default()];
        {
            fn reserve<T>(
                values: &mut crate::native::submit::SubmitArray<T>,
                count: usize,
            ) -> crate::Result<()> {
                values
                    .try_reserve_exact(count)
                    .map_err(|_| crate::Error::OutOfHostMemory)
            }
            let batch = &mut prepared[0];
            reserve(&mut batch.wait_semaphores, wait_capacity)?;
            reserve(&mut batch.wait_values, wait_capacity)?;
            reserve(&mut batch.wait_stages, wait_capacity)?;
            reserve(&mut batch.signal_semaphores, signals.len())?;
            reserve(&mut batch.signal_values, signals.len())?;
            for (semaphore, value, stage) in waits {
                batch.wait_semaphores.push(semaphore.handle);
                batch.wait_values.push(*value);
                batch.wait_stages.push(*stage);
            }
            for (semaphore, value) in signals {
                batch.signal_semaphores.push(semaphore.handle);
                batch.signal_values.push(*value);
            }
            batch.commands.push(self.handle);
        }
        if let Some(memory) = &mut nested_memory {
            memory.resize(prepared[0].heap_capacity_bytes() as u64)?;
        }
        let uses = crate::tracking::tracked::collect_submission_uses(
            &[self.submission_uses()?],
            &device.memory.stats,
        )?;
        let _reservations =
            crate::tracking::tracked::lock_submission_uses(uses.ordered(), &device.memory.stats)?;
        self.validate_submission()?;
        let native_states = unsafe {
            prepare_submission_states(
                &[&self],
                &uses,
                device.queue_id(queue.handle)?,
                queue.family,
                &device.memory.stats,
            )?
        };
        device.add_timeline_waits(queue.handle, &mut prepared, &native_states.dependencies())?;
        let result = device.submit_prepared(queue.handle, &prepared, vk::Fence::null());
        let point = result?;
        native_states.publish(point);
        self.record_submission(point);
        for semaphore in semaphores.iter() {
            semaphore.last_use.insert_overall(point);
        }
        Ok(PendingCommand {
            command: self,
            point,
            _semaphores: semaphores,
        })
    }
}
