//! Optional bounded GPU command-span measurements. Each pair is written by one
//! command on one physical queue. Completion comes from that submission's actual
//! timeline point; polling never waits or resets queries still used by the GPU.
use crate::{Error, Result, sync::{QueueId, SyncPoint}};
use crate::tracking::{GpuUse, SyncCell};
use crate::command::command_buffer::VulkanCommandBuffer;
use crate::device::Device;
use ash::vk;
use std::{collections::VecDeque, sync::Arc};

#[derive(Clone, Copy, Debug)]
pub struct GpuTimingConfig {
    /// Additional per-pass pairs; disabled measurements incur no per-pass GPU
    /// commands. Increase pending_commands to include both kinds of pairs.
    pub pass_spans: bool,
    /// Maximum pending timestamp pairs, including optional pass scopes.
    pub pending_commands: usize,
    pub retained_samples: usize,
    /// Diagnostic only (requires `pass_spans`): drain the GPU with an
    /// `ALL_COMMANDS` execution barrier before each pass's first timestamp, so a
    /// span holds that pass's own work instead of the tail of earlier passes.
    /// Spans then sum to a serialised frame, which is slower than production.
    pub isolate_passes: bool,
    /// Diagnostic only (requires `pass_spans`): record a pipeline-statistics
    /// query around each pass. Needs `pipelineStatisticsQuery`; passes on a
    /// queue family without graphics and compute carry no statistics.
    pub pass_statistics: bool,
}
impl Default for GpuTimingConfig {
    fn default() -> Self {
        Self {
            pass_spans: false,
            pending_commands: 256,
            retained_samples: 10000,
            isolate_passes: false,
            pass_statistics: false,
        }
    }
}
/// Pipeline-statistics counters of one pass (query results in ascending bit
/// order of [`STATISTIC_FLAGS`]). Counts are over everything the pass recorded,
/// so they separate geometry-bound passes (vertices, primitives) from
/// pixel-bound ones (fragment invocations) and compute-bound ones.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub struct GpuPassStatistics {
    pub input_vertices: u64,
    pub input_primitives: u64,
    pub vertex_invocations: u64,
    pub clipping_invocations: u64,
    pub clipping_primitives: u64,
    pub fragment_invocations: u64,
    pub compute_invocations: u64,
}
const STATISTIC_COUNT: usize = 7;
const STATISTIC_FLAGS: vk::QueryPipelineStatisticFlags = vk::QueryPipelineStatisticFlags::from_raw(
    vk::QueryPipelineStatisticFlags::INPUT_ASSEMBLY_VERTICES.as_raw()
        | vk::QueryPipelineStatisticFlags::INPUT_ASSEMBLY_PRIMITIVES.as_raw()
        | vk::QueryPipelineStatisticFlags::VERTEX_SHADER_INVOCATIONS.as_raw()
        | vk::QueryPipelineStatisticFlags::CLIPPING_INVOCATIONS.as_raw()
        | vk::QueryPipelineStatisticFlags::CLIPPING_PRIMITIVES.as_raw()
        | vk::QueryPipelineStatisticFlags::FRAGMENT_SHADER_INVOCATIONS.as_raw()
        | vk::QueryPipelineStatisticFlags::COMPUTE_SHADER_INVOCATIONS.as_raw(),
);
impl From<[u64; STATISTIC_COUNT]> for GpuPassStatistics {
    fn from(v: [u64; STATISTIC_COUNT]) -> Self {
        Self {
            input_vertices: v[0],
            input_primitives: v[1],
            vertex_invocations: v[2],
            clipping_invocations: v[3],
            clipping_primitives: v[4],
            fragment_invocations: v[5],
            compute_invocations: v[6],
        }
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct GpuCommandTiming {
    pub recording_epoch: u64,
    pub queue: usize,
    pub completion_value: u64,
    pub duration_ns: u64,
    /// None is the whole command span. Pass spans overlap command spans and
    /// must never be added to them to estimate a frame's GPU latency.
    pub pass_index: Option<usize>,
    /// Declared pass name for pass spans; always None for whole-command spans.
    /// Filled only when pass spans are enabled (diagnostic builds of the log).
    pub pass_name: Option<&'static str>,
    /// Pass spans only, when `pass_statistics` is enabled and the queue supports it.
    pub statistics: Option<GpuPassStatistics>,
}
/// A CPU-only reserved query pair. Its query-pool owner is retained by the
/// command's outer timing scope, through that command's real completion point.
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct GpuPassTimestamp {
    pool: vk::QueryPool,
    first: u32,
    isolate: bool,
    /// Pipeline-statistics pool and query index; None when not measured.
    statistics: Option<(vk::QueryPool, u32)>,
}
impl GpuPassTimestamp {
    /// Both calls happen outside any render pass instance: the pass callback
    /// opens and closes its own rendering between them, so statistics queries
    /// begin and end at the same (outermost) scope.
    pub(crate) fn write(self, device: &ash::Device, command: vk::CommandBuffer, end: bool) {
        unsafe {
            if end {
                if let Some((pool, query)) = self.statistics {
                    device.cmd_end_query(command, pool, query);
                }
            } else {
                device.cmd_reset_query_pool(command, self.pool, self.first, 2);
                if let Some((pool, query)) = self.statistics {
                    device.cmd_reset_query_pool(command, pool, query, 1);
                }
                if self.isolate {
                    device.cmd_pipeline_barrier(
                        command,
                        vk::PipelineStageFlags::ALL_COMMANDS,
                        vk::PipelineStageFlags::ALL_COMMANDS,
                        vk::DependencyFlags::empty(),
                        &[],
                        &[],
                        &[],
                    );
                }
            }
            device.cmd_write_timestamp(
                command,
                if end {
                    vk::PipelineStageFlags::BOTTOM_OF_PIPE
                } else {
                    vk::PipelineStageFlags::TOP_OF_PIPE
                },
                self.pool,
                self.first + u32::from(end),
            );
            if !end && let Some((pool, query)) = self.statistics {
                device.cmd_begin_query(command, pool, query, vk::QueryControlFlags::empty());
            }
        }
    }
}
#[derive(Clone, Copy, Debug, Default, serde::Serialize)]
pub struct GpuTimingStats {
    pub pending: usize,
    pub peak_pending: usize,
    pub completed: u64,
    pub capacity_skips: u64,
    pub retained_samples: usize,
}
struct Queries {
    retirement: Option<crate::device::RetireTicket>,
    device: Arc<Device>,
    handle: vk::QueryPool,
    /// Pipeline-statistics pool (one query per slot) and its own retirement node.
    statistics: Option<(vk::QueryPool, crate::device::RetireTicket)>,
    last_use: Arc<SyncCell>,
    // The command may retain this owner after the ring itself is dropped.
    _host_memory: crate::accounting::CpuMemoryReservation,
}
impl GpuUse for Queries {
    fn visit_uses(&self, visit: &mut crate::tracking::UseVisitor<'_>) -> Result<()> {
        visit(&self.last_use)
    }
}
impl Drop for Queries {
    fn drop(&mut self) {
        self.device.retire_reserved(
            self.last_use.snapshot(),
            crate::device::graveyard::Doomed::QueryPool(self.handle),
            self.retirement.take().expect("query retirement node"),
        );
        if let Some((pool, ticket)) = self.statistics.take() {
            self.device.retire_reserved(
                self.last_use.snapshot(),
                crate::device::graveyard::Doomed::QueryPool(pool),
                ticket,
            );
        }
    }
}
#[derive(Clone, Copy)]
struct Tracked {
    command: u64,
    epoch: u64,
    queue: QueueId,
    ended: bool,
    point: Option<SyncPoint>,
    pass_index: Option<usize>,
    /// A pipeline-statistics query was recorded for this pass slot.
    statistics: bool,
}
pub(crate) struct GpuTimingRing {
    // Slots and completed samples drop before their reservation.
    slots: Vec<Option<Tracked>>,
    /// Pass names of reserved pass-span slots, removed when the slot is released.
    slot_names: Vec<Option<crate::resource::ResourceName>>,
    active: Vec<(u64, usize)>,
    free: VecDeque<usize>,
    samples: VecDeque<GpuCommandTiming>,
    sample_limit: usize,
    pass_spans: bool,
    isolate_passes: bool,
    pass_statistics: bool,
    /// Queues whose family can run both graphics and compute statistics.
    statistics_queue: [bool; crate::sync::MAX_QUEUES],
    _host_memory: crate::accounting::CpuMemoryReservation,
    queries: Arc<Queries>,
    valid_bits: [u32; crate::sync::MAX_QUEUES],
    period_ns: f32,
    stats: GpuTimingStats,
}
impl GpuTimingRing {
    pub(crate) fn matches_config(&self, config: GpuTimingConfig) -> bool {
        self.pass_spans == config.pass_spans
            && self.isolate_passes == config.isolate_passes
            && self.pass_statistics == config.pass_statistics
            && self.slots.len() == config.pending_commands
            && self.sample_limit == config.retained_samples
    }
    pub(crate) fn new(device: Arc<Device>, config: GpuTimingConfig) -> Result<Self> {
        if device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        if config.pending_commands == 0
            || config.pending_commands > 65536
            || config.retained_samples == 0
            || config.retained_samples > 100000
        {
            return Err(Error::invalid_usage(
                "invalid bounded GPU timing configuration",
            ));
        }
        if (config.isolate_passes || config.pass_statistics) && !config.pass_spans {
            return Err(Error::invalid_usage(
                "pass isolation and pass statistics need pass spans",
            ));
        }
        let properties = unsafe {
            device
                .core
                .instance
                .instance
                .get_physical_device_properties(device.core.physical)
        };
        let families = unsafe {
            device
                .core
                .instance
                .instance
                .get_physical_device_queue_family_properties(device.core.physical)
        };
        let mut valid_bits = [0; crate::sync::MAX_QUEUES];
        for (index, queue) in device.queues.plan.queues.iter().enumerate() {
            valid_bits[index] = families[queue.family as usize].timestamp_valid_bits;
            if !(1..=64).contains(&valid_bits[index]) {
                return Err(Error::unsupported(
                    "an enabled queue has no usable GPU timestamp clock",
                ));
            }
        }
        let period_ns = properties.limits.timestamp_period;
        if !period_ns.is_finite() || period_ns <= 0.0 {
            return Err(Error::unsupported("invalid GPU timestamp period"));
        }
        let mut statistics_queue = [false; crate::sync::MAX_QUEUES];
        if config.pass_statistics {
            // Device creation enables the feature whenever the driver has it.
            let features = unsafe {
                device
                    .core
                    .instance
                    .instance
                    .get_physical_device_features(device.core.physical)
            };
            if features.pipeline_statistics_query == 0 {
                return Err(Error::unsupported("pipelineStatisticsQuery is unavailable"));
            }
            let both = vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE;
            for (index, queue) in device.queues.plan.queues.iter().enumerate() {
                statistics_queue[index] = families[queue.family as usize].queue_flags.contains(both);
            }
        }
        let bytes = config.pending_commands
            * (std::mem::size_of::<Option<Tracked>>()
                + std::mem::size_of::<Option<crate::resource::ResourceName>>()
                + std::mem::size_of::<(u64, usize)>()
                + std::mem::size_of::<usize>())
            + config.retained_samples * std::mem::size_of::<GpuCommandTiming>()
            + std::mem::size_of::<Self>();
        let mut host_memory = device
            .memory
            .stats
            .reserve_cpu_bytes(bytes as u64, crate::accounting::BudgetCategory::Live)?;
        let mut slots = Vec::new();
        let mut slot_names = Vec::new();
        let mut active = Vec::new();
        let mut free = VecDeque::new();
        let mut samples = VecDeque::new();
        slots
            .try_reserve_exact(config.pending_commands)
            .map_err(|_| Error::OutOfHostMemory)?;
        active
            .try_reserve_exact(config.pending_commands)
            .map_err(|_| Error::OutOfHostMemory)?;
        free.try_reserve_exact(config.pending_commands)
            .map_err(|_| Error::OutOfHostMemory)?;
        samples
            .try_reserve_exact(config.retained_samples)
            .map_err(|_| Error::OutOfHostMemory)?;
        slot_names
            .try_reserve_exact(config.pending_commands)
            .map_err(|_| Error::OutOfHostMemory)?;
        slot_names.resize(config.pending_commands, None);
        slots.resize(config.pending_commands, None);
        free.extend(0..config.pending_commands);
        let actual_bytes = slots.capacity() * std::mem::size_of::<Option<Tracked>>()
            + slot_names.capacity() * std::mem::size_of::<Option<crate::resource::ResourceName>>()
            + active.capacity() * std::mem::size_of::<(u64, usize)>()
            + free.capacity() * std::mem::size_of::<usize>()
            + samples.capacity() * std::mem::size_of::<GpuCommandTiming>()
            + std::mem::size_of::<Self>();
        host_memory.resize(actual_bytes as u64)?;
        let query_memory = device.memory.stats.reserve_cpu_bytes(
            crate::accounting::shared_cpu_bytes::<Queries>()?,
            crate::accounting::BudgetCategory::Live,
        )?;
        let last_use = SyncCell::for_device_budgeted(device.generation(), &device.memory.stats)?;
        let retirement = device.reserve_retirement()?;
        let statistics_retirement = config
            .pass_statistics
            .then(|| device.reserve_retirement())
            .transpose()?;
        let info = vk::QueryPoolCreateInfo::default()
            .query_type(vk::QueryType::TIMESTAMP)
            .query_count(config.pending_commands as u32 * 2);
        let handle =
            unsafe { device.core.device.create_query_pool(&info, None) }.map_err(|error| {
                if error == vk::Result::ERROR_DEVICE_LOST {
                    device.queues.timelines.mark_device_lost();
                }
                Error::from(error)
            })?;
        let statistics = match statistics_retirement {
            Some(ticket) => {
                let info = vk::QueryPoolCreateInfo::default()
                    .query_type(vk::QueryType::PIPELINE_STATISTICS)
                    .query_count(config.pending_commands as u32)
                    .pipeline_statistics(STATISTIC_FLAGS);
                match unsafe { device.core.device.create_query_pool(&info, None) } {
                    Ok(pool) => Some((pool, ticket)),
                    Err(error) => {
                        // The timestamp pool is owned by nothing yet.
                        unsafe { device.core.device.destroy_query_pool(handle, None) };
                        if error == vk::Result::ERROR_DEVICE_LOST {
                            device.queues.timelines.mark_device_lost();
                        }
                        return Err(Error::from(error));
                    }
                }
            }
            None => None,
        };
        Ok(Self {
            slots,
            slot_names,
            active,
            free,
            samples,
            sample_limit: config.retained_samples,
            pass_spans: config.pass_spans,
            isolate_passes: config.isolate_passes,
            pass_statistics: config.pass_statistics,
            statistics_queue,
            _host_memory: host_memory,
            queries: Arc::new(Queries {
                retirement: Some(retirement),
                statistics,
                last_use,
                device,
                handle,
                _host_memory: query_memory,
            }),
            valid_bits,
            period_ns,
            stats: Default::default(),
        })
    }
    pub(crate) fn begin(
        &mut self,
        command: &mut VulkanCommandBuffer,
        queue: QueueId,
        epoch: u64,
    ) -> Result<()> {
        if !Arc::ptr_eq(&command.pool.device, &self.queries.device)
            || !command.is_recording()
            || self
                .queries
                .device
                .queues
                .plan
                .queues
                .get(queue.index())
                .is_none_or(|q| q.family != command.pool.family)
        {
            return Err(Error::invalid_usage(
                "GPU timing command/queue/device mismatch",
            ));
        }
        use ash::vk::Handle;
        let handle = command.handle.as_raw();
        if self.active.iter().any(|(old, _)| *old == handle) {
            return Err(Error::invalid_usage("GPU timing command already tracked"));
        }
        let Some(slot) = self.free.front().copied() else {
            self.stats.capacity_skips += 1;
            return Ok(());
        };
        command.retain_object(self.queries.clone())?;
        self.free.pop_front();
        unsafe {
            self.queries.device.core.device.cmd_reset_query_pool(
                command.handle,
                self.queries.handle,
                slot as u32 * 2,
                2,
            );
            self.queries.device.core.device.cmd_write_timestamp(
                command.handle,
                vk::PipelineStageFlags::TOP_OF_PIPE,
                self.queries.handle,
                slot as u32 * 2,
            );
        }
        self.slots[slot] = Some(Tracked {
            command: handle,
            epoch,
            queue,
            ended: false,
            point: None,
            pass_index: None,
            statistics: false,
        });
        self.active.push((handle, slot));
        self.stats.pending += 1;
        self.stats.peak_pending = self.stats.peak_pending.max(self.stats.pending);
        Ok(())
    }
    /// Reserve on the coordinator before workers record. The value contains no
    /// owner or heap allocation and each worker writes its own disjoint pair.
    pub(crate) fn reserve_pass(
        &mut self,
        command: u64,
        pass_index: usize,
        name: &crate::resource::ResourceName,
    ) -> Option<GpuPassTimestamp> {
        if !self.pass_spans {
            return None;
        }
        let outer = self.active.iter().find_map(|(handle, slot)| {
            let tracked = self.slots[*slot]?;
            (*handle == command
                && tracked.pass_index.is_none()
                && tracked.point.is_none()
                && !tracked.ended)
                .then_some(tracked)
        })?;
        let Some(slot) = self.free.pop_front() else {
            self.stats.capacity_skips += 1;
            return None;
        };
        let statistics = self.queries.statistics.as_ref().filter(|_| {
            self.pass_statistics && self.statistics_queue[outer.queue.index()]
        });
        self.slots[slot] = Some(Tracked {
            command,
            pass_index: Some(pass_index),
            // Successful recording writes both timestamps. Failed/panicking
            // callbacks abandon the unsubmitted command and all its pairs.
            ended: true,
            statistics: statistics.is_some(),
            ..outer
        });
        self.slot_names[slot] = Some(*name);
        self.active.push((command, slot));
        self.stats.pending += 1;
        self.stats.peak_pending = self.stats.peak_pending.max(self.stats.pending);
        Some(GpuPassTimestamp {
            pool: self.queries.handle,
            first: slot as u32 * 2,
            isolate: self.isolate_passes,
            statistics: statistics.map(|(pool, _)| (*pool, slot as u32)),
        })
    }
    pub(crate) fn end(&mut self, command: vk::CommandBuffer) {
        use ash::vk::Handle;
        let Some((_, slot)) = self.active.iter().find(|(old, slot)| {
            *old == command.as_raw()
                && self.slots[*slot].is_some_and(|tracked| tracked.pass_index.is_none())
        }) else {
            return;
        };
        let tracked = self.slots[*slot].as_mut().expect("active query slot");
        debug_assert!(!tracked.ended && tracked.point.is_none());
        unsafe {
            self.queries.device.core.device.cmd_write_timestamp(
                command,
                vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                self.queries.handle,
                *slot as u32 * 2 + 1,
            );
        }
        tracked.ended = true;
    }
    pub(crate) fn submitted(&mut self, command: u64, point: SyncPoint) {
        for (_, slot) in self.active.iter().filter(|(old, _)| *old == command) {
            let tracked = self.slots[*slot].as_mut().expect("active query slot");
            debug_assert!(tracked.ended && tracked.point.is_none() && point.queue == tracked.queue);
            tracked.point = Some(point);
        }
    }
    /// Only for an unsubmitted command whose recording is being abandoned.
    pub(crate) fn abandon(&mut self, command: u64) {
        let mut index = 0;
        while index < self.active.len() {
            let slot = self.active[index].1;
            if self.active[index].0 == command
                && self.slots[slot].is_some_and(|tracked| tracked.point.is_none())
            {
                self.release(index, slot);
            } else {
                index += 1;
            }
        }
    }
    fn release(&mut self, index: usize, slot: usize) {
        self.active.swap_remove(index);
        self.slots[slot] = None;
        self.slot_names[slot] = None;
        self.free.push_back(slot);
        self.stats.pending -= 1;
    }
    pub(crate) fn poll(&mut self) -> Result<()> {
        let completed = self.queries.device.poll()?;
        let mut index = 0;
        while index < self.active.len() {
            let slot = self.active[index].1;
            let tracked = self.slots[slot].expect("active query slot");
            let Some(point) = tracked
                .point
                .filter(|point| completed.get(point.queue) >= point.value)
            else {
                index += 1;
                continue;
            };
            debug_assert_eq!(tracked.command, self.active[index].0);
            let mut values = [0u64; 2];
            match unsafe {
                self.queries.device.core.device.get_query_pool_results(
                    self.queries.handle,
                    slot as u32 * 2,
                    &mut values,
                    vk::QueryResultFlags::TYPE_64,
                )
            } {
                Ok(()) => {}
                Err(vk::Result::NOT_READY) => {
                    index += 1;
                    continue;
                }
                Err(error) => {
                    if error == vk::Result::ERROR_DEVICE_LOST {
                        self.queries.device.queues.timelines.mark_device_lost();
                    }
                    return Err(error.into());
                }
            }
            let statistics = match (tracked.statistics, self.queries.statistics.as_ref()) {
                (true, Some((pool, _))) => {
                    // One query: the slice length is the query count and the
                    // element size is the stride, so read one `[u64; N]` element.
                    let mut counters = [[0u64; STATISTIC_COUNT]; 1];
                    match unsafe {
                        self.queries.device.core.device.get_query_pool_results(
                            *pool,
                            slot as u32,
                            &mut counters,
                            vk::QueryResultFlags::TYPE_64,
                        )
                    } {
                        Ok(()) => Some(GpuPassStatistics::from(counters[0])),
                        Err(vk::Result::NOT_READY) => None,
                        Err(error) => {
                            if error == vk::Result::ERROR_DEVICE_LOST {
                                self.queries.device.queues.timelines.mark_device_lost();
                            }
                            return Err(error.into());
                        }
                    }
                }
                _ => None,
            };
            let duration_ns =
                duration_ns(values, self.valid_bits[point.queue.index()], self.period_ns);
            if let Some(duration_ns) = duration_ns {
                if self.samples.len() == self.sample_limit {
                    self.samples.pop_front();
                }
                self.samples.push_back(GpuCommandTiming {
                    recording_epoch: tracked.epoch,
                    queue: point.queue.index(),
                    completion_value: point.value,
                    duration_ns,
                    pass_index: tracked.pass_index,
                    pass_name: self.slot_names[slot].map(|name| name.as_str()),
                    statistics,
                });
                self.stats.completed += 1;
            }
            self.release(index, slot);
        }
        Ok(())
    }
    pub(crate) fn samples(&self) -> impl Iterator<Item = &GpuCommandTiming> {
        self.samples.iter()
    }
    /// A command whose query pair has not been harvested yet must not be reset
    /// or recorded again, even if its submission point is already complete.
    pub(crate) fn is_tracking_handle(&self, handle: u64) -> bool {
        self.active.iter().any(|(command, _)| *command == handle)
    }
    pub(crate) fn drain_samples(&mut self) -> impl Iterator<Item = GpuCommandTiming> + '_ {
        self.samples.drain(..)
    }
    pub(crate) fn stats(&self) -> GpuTimingStats {
        GpuTimingStats {
            retained_samples: self.samples.len(),
            ..self.stats
        }
    }
}
fn duration_ns(values: [u64; 2], bits: u32, period: f32) -> Option<u64> {
    if !(1..=64).contains(&bits) || !period.is_finite() || period <= 0.0 {
        return None;
    }
    let mask = u64::MAX >> (64 - bits);
    let ticks = values[1].wrapping_sub(values[0]) & mask;
    let ns = ticks as f64 * f64::from(period);
    (ns.is_finite() && ns < u64::MAX as f64).then(|| ns.round() as u64)
}
#[cfg(test)]
mod tests;
