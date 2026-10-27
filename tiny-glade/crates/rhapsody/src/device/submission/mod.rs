//! 所有生产队列调用的共同入口：每队列锁、成功提交后确认时间线。
use crate::device::Device;
use crate::native::submit::{PreparedSubmit, SubmitScratch};
use crate::sync::{QueueId, SyncPoint};
use crate::{Error, Result};
use ash::vk;

#[derive(Clone, Copy, Debug)]
pub enum Wait {
    Binary {
        semaphore: vk::Semaphore,
        stage: vk::PipelineStageFlags,
    },
    Timeline {
        point: SyncPoint,
        stage: vk::PipelineStageFlags,
    },
}

#[derive(Clone, Copy, Debug)]
pub enum Signal {
    Binary(vk::Semaphore),
}

pub struct SubmitBatch<'a> {
    pub waits: &'a [Wait],
    pub commands: &'a [vk::CommandBuffer],
    pub signals: &'a [Signal],
}

impl Device {
    pub fn queue(&self, class: crate::device::QueueClass) -> Result<QueueId> {
        Ok(self.queues.plan.queue(class))
    }

    /// Fresh completion values (queries queues with outstanding work). The
    /// same snapshot drives budget classification; that step is skipped when
    /// no queue advanced.
    pub fn poll(&self) -> Result<crate::sync::CompletedValues> {
        let completed = self.queues.timelines.poll()?;
        self.memory.stats.observe_completed(&completed);
        Ok(completed)
    }

    /// `(poll calls, driver queries)` issued so far; diagnostics for how often
    /// callers ask the driver about completion.
    pub fn timeline_poll_stats(&self) -> (u64, u64) {
        self.queues.timelines.poll_stats()
    }

    /// Last observed completion values without a device query. Stale values
    /// are conservative (work may look pending, never complete early); use
    /// [`Self::poll`] when progress must be observed.
    pub fn completed(&self) -> crate::sync::CompletedValues {
        self.queues.timelines.completed()
    }

    pub fn memory_budget_usage(&self) -> Result<crate::accounting::BudgetUsage> {
        let _ = self.poll()?;
        Ok(self.memory.stats.budget_usage())
    }

    pub fn wait(&self, set: &crate::sync::SyncSet, timeout_ns: u64) -> Result<()> {
        self.queues.timelines.wait(set, timeout_ns)
    }

    pub fn is_device_lost(&self) -> bool {
        self.queues.timelines.is_device_lost()
    }

    /// Conservative variant for prerequisites without a proven consumer
    /// stage: every cross-queue wait blocks `ALL_COMMANDS`.
    pub(crate) fn add_timeline_waits(
        &self,
        queue: vk::Queue,
        prepared: &mut [PreparedSubmit],
        dependencies: &crate::sync::SyncSet,
    ) -> Result<()> {
        self.add_staged_timeline_waits(
            queue,
            prepared,
            &crate::sync::StagedDependencies::opaque(*dependencies),
        )
    }

    /// Add one timeline wait per foreign queue to the first batch. Each wait
    /// blocks exactly the stages recorded for that queue in `dependencies`
    /// (merged with any wait the caller already placed on that semaphore).
    pub(crate) fn add_staged_timeline_waits(
        &self,
        queue: vk::Queue,
        prepared: &mut [PreparedSubmit],
        dependencies: &crate::sync::StagedDependencies,
    ) -> Result<()> {
        let queue_id = self.queue_id(queue)?;
        let Some(first) = prepared.first_mut() else {
            return if dependencies.is_empty() {
                Ok(())
            } else {
                Err(Error::invalid_usage(
                    "timeline dependencies require a submission batch",
                ))
            };
        };
        apply_staged_waits(first, queue_id, dependencies, |queue| {
            self.queues.timelines.semaphore(queue)
        })
    }

    /// 提交前检查设备时间线等待值；成功后返回同一设备完成域中的真实完成点。
    ///
    /// # Safety
    /// 命令必须已录制结束且来自此队列族；二值信号量必须属于此设备并满足 Vulkan
    /// 的信号/等待状态约束。命令、信号量及引用资源须保留至返回的完成点达成。
    pub unsafe fn submit(&self, queue: QueueId, batches: &[SubmitBatch<'_>]) -> Result<SyncPoint> {
        if self.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        let raw = if queue.index() == 0 {
            self.queues.primary
        } else {
            self.queues
                .additional
                .get(queue.index() - 1)
                .ok_or_else(|| Error::invalid_usage("提交队列不属于设备"))?
                .handle
        };
        let nested_bytes = batches.iter().try_fold(0usize, |sum, batch| {
            sum.checked_add(PreparedSubmit::heap_bytes_for_counts(
                batch.waits.len(),
                batch.signals.len(),
                batch.commands.len(),
            )?)
            .ok_or_else(|| Error::invalid_usage("submission adapter storage overflows"))
        })?;
        // Both outer and nested storage are admitted before input collection.
        // This ticket precedes `prepared`, so actual arrays are freed first on
        // success and every validation/allocation/driver error.
        let mut nested_memory = if nested_bytes == 0 {
            None
        } else {
            Some(
                self.memory
                    .stats
                    .reserve_cpu_bytes(nested_bytes as u64, crate::accounting::BudgetCategory::Live)?,
            )
        };
        let prepared = SubmitScratch::<[PreparedSubmit; 1]>::try_collect(
            batches.len(),
            &self.memory.stats,
            batches.iter().map(|batch| {
                fn reserve<T>(
                    values: &mut crate::native::submit::SubmitArray<T>,
                    count: usize,
                ) -> Result<()> {
                    values
                        .try_reserve_exact(count)
                        .map_err(|_| Error::OutOfHostMemory)
                }
                let mut output = PreparedSubmit::default();
                reserve(&mut output.commands, batch.commands.len())?;
                reserve(&mut output.wait_semaphores, batch.waits.len())?;
                reserve(&mut output.wait_values, batch.waits.len())?;
                reserve(&mut output.wait_stages, batch.waits.len())?;
                reserve(&mut output.signal_semaphores, batch.signals.len())?;
                reserve(&mut output.signal_values, batch.signals.len())?;
                output.commands.extend_from_slice(batch.commands);
                for wait in batch.waits {
                    let (semaphore, value, stage) = match *wait {
                        Wait::Binary { semaphore, stage } => (semaphore, 0, stage),
                        Wait::Timeline { point, stage } => {
                            if point.value > self.queues.timelines.submitted(point.queue)? {
                                return Err(Error::invalid_usage("等待的时间线值尚未提交"));
                            }
                            (
                                self.queues.timelines.semaphore(point.queue)?,
                                point.value,
                                stage,
                            )
                        }
                    };
                    if semaphore == vk::Semaphore::null()
                        || stage.is_empty()
                        || output.wait_semaphores.contains(&semaphore)
                    {
                        return Err(Error::invalid_usage(
                            "提交等待信号量为空、重复或等待阶段为空",
                        ));
                    }
                    output.wait_semaphores.push(semaphore);
                    output.wait_values.push(value);
                    output.wait_stages.push(stage);
                }
                for &Signal::Binary(semaphore) in batch.signals {
                    if semaphore == vk::Semaphore::null()
                        || output.signal_semaphores.contains(&semaphore)
                    {
                        return Err(Error::invalid_usage("提交信号量为空或重复"));
                    }
                    output.signal_semaphores.push(semaphore);
                    output.signal_values.push(0);
                }
                Ok(output)
            }),
        )?;
        if let Some(memory) = &mut nested_memory {
            let observed = prepared.iter().try_fold(0usize, |sum, batch| {
                sum.checked_add(batch.heap_capacity_bytes())
                    .ok_or_else(|| Error::invalid_usage("submission adapter capacity overflows"))
            })?;
            memory.resize(observed as u64)?;
        }
        self.submit_prepared(raw, &prepared, vk::Fence::null())
    }
    pub fn queue_id(&self, queue: vk::Queue) -> Result<QueueId> {
        let index = if queue == self.queues.primary {
            0
        } else {
            self.queues
                .additional
                .iter()
                .position(|q| q.handle == queue)
                .map(|i| i + 1)
                .ok_or_else(|| Error::invalid_usage("队列不属于设备"))?
        };
        QueueId::new(index).ok_or_else(|| Error::invalid_usage("队列数超过支持上限"))
    }

    pub(crate) fn submit_prepared(
        &self,
        queue: vk::Queue,
        batches: &[PreparedSubmit],
        fence: vk::Fence,
    ) -> Result<SyncPoint> {
        if self.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        let mut trace = crate::trace::SubmitTrace::new("device_submit_prepared");
        trace.count("batches", batches.len());
        let id = self.queue_id(queue)?;
        for batch in batches {
            batch.validate(&self.queues.timelines)?;
        }
        trace.mark("validate_batches");
        self.flush_deferred_host_writes()?;
        trace.mark("flush_host_writes");
        self.submit_prepared2(queue, id, batches, fence)
    }

    fn submit_prepared2(
        &self,
        queue: vk::Queue,
        id: QueueId,
        batches: &[PreparedSubmit],
        fence: vk::Fence,
    ) -> Result<SyncPoint> {
        // All owned arrays and views are prepared before acquiring the queue
        // lock. Flat arrays avoid an allocation for every batch's small lists.
        let count = batches.len().max(1);
        let counts = batches.iter().try_fold(
            (0usize, 0usize, 1usize),
            |(commands, waits, signals), batch| {
                Ok::<_, Error>((
                    commands.checked_add(batch.commands.len()).ok_or_else(|| {
                        Error::invalid_usage("submission command count overflows")
                    })?,
                    waits
                        .checked_add(batch.wait_semaphores.len())
                        .ok_or_else(|| Error::invalid_usage("submission wait count overflows"))?,
                    signals
                        .checked_add(batch.signal_semaphores.len())
                        .ok_or_else(|| Error::invalid_usage("submission signal count overflows"))?,
                ))
            },
        )?;
        let commands = SubmitScratch::<[vk::CommandBufferSubmitInfo<'static>; 8]>::try_collect(
            counts.0,
            &self.memory.stats,
            batches
                .iter()
                .flat_map(|batch| &batch.commands)
                .map(|&command| Ok(vk::CommandBufferSubmitInfo::default().command_buffer(command))),
        )?;
        let waits = SubmitScratch::<[vk::SemaphoreSubmitInfo<'static>; 8]>::try_collect(
            counts.1,
            &self.memory.stats,
            batches
                .iter()
                .flat_map(|batch| {
                    batch
                        .wait_semaphores
                        .iter()
                        .zip(&batch.wait_values)
                        .zip(&batch.wait_stages)
                })
                .map(|((&semaphore, &value), &stage)| {
                    Ok(vk::SemaphoreSubmitInfo::default()
                        .semaphore(semaphore)
                        .value(value)
                        .stage_mask(vk::PipelineStageFlags2::from_raw(u64::from(stage.as_raw()))))
                }),
        )?;
        let mut signals = SubmitScratch::<[vk::SemaphoreSubmitInfo<'static>; 8]>::try_collect(
            counts.2,
            &self.memory.stats,
            batches
                .iter()
                .flat_map(|batch| batch.signal_semaphores.iter().zip(&batch.signal_values))
                .map(|(&semaphore, &value)| {
                    Ok(vk::SemaphoreSubmitInfo::default()
                        .semaphore(semaphore)
                        .value(value)
                        .stage_mask(vk::PipelineStageFlags2::ALL_COMMANDS))
                })
                .chain(std::iter::once(Ok(vk::SemaphoreSubmitInfo::default()
                    .stage_mask(vk::PipelineStageFlags2::ALL_COMMANDS)))),
        )?;
        let signals_ptr = signals.as_mut_ptr();
        let (mut command_start, mut wait_start, mut signal_start) = (0usize, 0usize, 0usize);
        // These raw pointer views are consumed only by queue_submit2 below. The
        // three owning scratch arrays have their final capacities and locations.
        let infos = SubmitScratch::<[vk::SubmitInfo2<'static>; 1]>::try_collect(
            count,
            &self.memory.stats,
            (0..count).map(|index| {
                let batch = batches.get(index);
                let command_len = batch.map_or(0, |batch| batch.commands.len());
                let wait_len = batch.map_or(0, |batch| batch.wait_semaphores.len());
                let signal_len = batch.map_or(0, |batch| batch.signal_semaphores.len())
                    + usize::from(index + 1 == count);
                let view = vk::SubmitInfo2 {
                    command_buffer_info_count: native_count(command_len)?,
                    p_command_buffer_infos: commands.as_ptr().wrapping_add(command_start),
                    wait_semaphore_info_count: native_count(wait_len)?,
                    p_wait_semaphore_infos: waits.as_ptr().wrapping_add(wait_start),
                    signal_semaphore_info_count: native_count(signal_len)?,
                    p_signal_semaphore_infos: signals_ptr.wrapping_add(signal_start),
                    ..Default::default()
                };
                command_start += command_len;
                wait_start += wait_len;
                signal_start += signal_len;
                Ok(view)
            }),
        )?;
        native_count(infos.len())?;
        let extension = self.core.synchronization2.as_ref();
        let (point, ()) = self.queues.timelines.submit(id, |signal| {
            let mut driver_trace = crate::trace::SubmitTrace::new("vkQueueSubmit2");
            driver_trace.count("submit_infos", infos.len());
            // SAFETY: all arrays are owned by this call, all counts are checked,
            // and no backing can grow or move while the driver reads the views.
            let result = unsafe {
                let last = signals_ptr.add(counts.2 - 1);
                std::ptr::addr_of_mut!((*last).semaphore).write(signal.semaphore);
                std::ptr::addr_of_mut!((*last).value).write(signal.value);
                if let Some(extension) = extension {
                    extension.queue_submit2(queue, &infos, fence)
                } else {
                    self.core.device.queue_submit2(queue, &infos, fence)
                }
            };
            driver_trace.mark("driver");
            result.map_err(Into::into)
        })?;
        Ok(point)
    }
}

/// Waits for every foreign queue in `dependencies` at its recorded stages.
pub(crate) fn apply_staged_waits(
    batch: &mut PreparedSubmit,
    queue: QueueId,
    dependencies: &crate::sync::StagedDependencies,
    semaphore: impl Fn(QueueId) -> Result<vk::Semaphore>,
) -> Result<()> {
    for (point, stages) in dependencies.waits() {
        if point.queue != queue {
            merge_timeline_wait(batch, semaphore(point.queue)?, point.value, stages)?;
        }
    }
    Ok(())
}

/// Merge one timeline wait into a prepared batch: an existing wait on the
/// same semaphore keeps the larger value and the union of both stage masks.
pub(crate) fn merge_timeline_wait(
    batch: &mut PreparedSubmit,
    semaphore: vk::Semaphore,
    value: u64,
    stages: vk::PipelineStageFlags,
) -> Result<()> {
    let stages = if stages.is_empty() {
        vk::PipelineStageFlags::ALL_COMMANDS
    } else {
        stages
    };
    if let Some(index) = batch
        .wait_semaphores
        .iter()
        .position(|&wait| wait == semaphore)
    {
        let old = batch
            .wait_values
            .get_mut(index)
            .ok_or_else(|| Error::invalid_usage("invalid wait value array"))?;
        *old = (*old).max(value);
        let stage = batch
            .wait_stages
            .get_mut(index)
            .ok_or_else(|| Error::invalid_usage("invalid wait stage array"))?;
        *stage |= stages;
        return Ok(());
    }
    batch
        .wait_semaphores
        .try_reserve(1)
        .map_err(|_| Error::OutOfHostMemory)?;
    batch
        .wait_values
        .try_reserve(1)
        .map_err(|_| Error::OutOfHostMemory)?;
    batch
        .wait_stages
        .try_reserve(1)
        .map_err(|_| Error::OutOfHostMemory)?;
    batch.wait_semaphores.push(semaphore);
    batch.wait_values.push(value);
    batch.wait_stages.push(stages);
    Ok(())
}

fn native_count(count: usize) -> Result<u32> {
    u32::try_from(count)
        .map_err(|_| Error::invalid_usage("Vulkan submission array exceeds u32 count"))
}

#[cfg(test)]
mod tests;
