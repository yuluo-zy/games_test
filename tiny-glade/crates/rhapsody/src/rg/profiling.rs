//! Optional bounded GPU command-span measurements. Each pair is written by one
//! command on one physical queue. Completion comes from that submission's actual
//! timeline point; polling never waits or resets queries still used by the GPU.
use crate::{
    Error, Result,
    raw::{command_buffer::VulkanCommandBuffer, device::Device},
    resource::{GpuUse, SyncCell},
    sync::{QueueId, SyncPoint},
};
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
}
impl Default for GpuTimingConfig {
    fn default() -> Self {
        Self {
            pass_spans: false,
            pending_commands: 256,
            retained_samples: 10000,
        }
    }
}
#[derive(Clone, Copy, Debug, serde::Serialize)]
pub struct GpuCommandTiming {
    pub recording_epoch: u64,
    pub queue: usize,
    pub completion_value: u64,
    pub duration_ns: u64,
    /// None is the whole command span. Pass spans overlap command spans and
    /// must never be added to them to estimate a frame's GPU latency.
    pub pass_index: Option<usize>,
}
/// A CPU-only reserved query pair. Its query-pool owner is retained by the
/// command's outer timing scope, through that command's real completion point.
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct GpuPassTimestamp {
    pool: vk::QueryPool,
    first: u32,
}
impl GpuPassTimestamp {
    pub(crate) fn write(self, device: &ash::Device, command: vk::CommandBuffer, end: bool) {
        unsafe {
            if !end {
                device.cmd_reset_query_pool(command, self.pool, self.first, 2);
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
    device: Arc<Device>,
    handle: vk::QueryPool,
    last_use: Arc<SyncCell>,
    // The command may retain this owner after the ring itself is dropped.
    _host_memory: crate::raw::allocator::CpuMemoryReservation,
}
impl GpuUse for Queries {
    fn visit_uses(&self, visit: &mut crate::resource::UseVisitor<'_>) -> Result<()> {
        visit(&self.last_use)
    }
}
impl Drop for Queries {
    fn drop(&mut self) {
        self.device.retire(
            self.last_use.snapshot(),
            crate::raw::graveyard::Doomed::QueryPool(self.handle),
        );
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
}
pub(crate) struct GpuTimingRing {
    // Slots and completed samples drop before their reservation.
    slots: Vec<Option<Tracked>>,
    active: Vec<(u64, usize)>,
    free: VecDeque<usize>,
    samples: VecDeque<GpuCommandTiming>,
    sample_limit: usize,
    pass_spans: bool,
    _host_memory: crate::raw::allocator::CpuMemoryReservation,
    queries: Arc<Queries>,
    valid_bits: [u32; crate::sync::MAX_QUEUES],
    period_ns: f32,
    stats: GpuTimingStats,
}
impl GpuTimingRing {
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
        let properties = unsafe {
            device
                .instance
                .instance
                .get_physical_device_properties(device.physical)
        };
        let families = unsafe {
            device
                .instance
                .instance
                .get_physical_device_queue_family_properties(device.physical)
        };
        let mut valid_bits = [0; crate::sync::MAX_QUEUES];
        for (index, queue) in device.queue_plan.queues.iter().enumerate() {
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
        let bytes = config.pending_commands
            * (std::mem::size_of::<Option<Tracked>>()
                + std::mem::size_of::<(u64, usize)>()
                + std::mem::size_of::<usize>())
            + config.retained_samples * std::mem::size_of::<GpuCommandTiming>()
            + std::mem::size_of::<Self>();
        let mut host_memory = device
            .allocator_stats
            .reserve_cpu_bytes(bytes as u64, crate::device::BudgetCategory::Live)?;
        let mut slots = Vec::new();
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
        slots.resize(config.pending_commands, None);
        free.extend(0..config.pending_commands);
        let actual_bytes = slots.capacity() * std::mem::size_of::<Option<Tracked>>()
            + active.capacity() * std::mem::size_of::<(u64, usize)>()
            + free.capacity() * std::mem::size_of::<usize>()
            + samples.capacity() * std::mem::size_of::<GpuCommandTiming>()
            + std::mem::size_of::<Self>();
        host_memory.resize(actual_bytes as u64)?;
        let query_memory = device.allocator_stats.reserve_cpu_bytes(
            crate::raw::allocator::shared_cpu_bytes::<Queries>()?,
            crate::device::BudgetCategory::Live,
        )?;
        let last_use = SyncCell::for_device_budgeted(device.generation(), &device.allocator_stats)?;
        let info = vk::QueryPoolCreateInfo::default()
            .query_type(vk::QueryType::TIMESTAMP)
            .query_count(config.pending_commands as u32 * 2);
        let handle = unsafe { device.device.create_query_pool(&info, None) }.map_err(|error| {
            if error == vk::Result::ERROR_DEVICE_LOST {
                device.timelines.mark_device_lost();
            }
            Error::from(error)
        })?;
        Ok(Self {
            slots,
            active,
            free,
            samples,
            sample_limit: config.retained_samples,
            pass_spans: config.pass_spans,
            _host_memory: host_memory,
            queries: Arc::new(Queries {
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
                .queue_plan
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
            self.queries.device.device.cmd_reset_query_pool(
                command.handle,
                self.queries.handle,
                slot as u32 * 2,
                2,
            );
            self.queries.device.device.cmd_write_timestamp(
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
        self.slots[slot] = Some(Tracked {
            command,
            pass_index: Some(pass_index),
            // Successful recording writes both timestamps. Failed/panicking
            // callbacks abandon the unsubmitted command and all its pairs.
            ended: true,
            ..outer
        });
        self.active.push((command, slot));
        self.stats.pending += 1;
        self.stats.peak_pending = self.stats.peak_pending.max(self.stats.pending);
        Some(GpuPassTimestamp {
            pool: self.queries.handle,
            first: slot as u32 * 2,
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
            self.queries.device.device.cmd_write_timestamp(
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
                self.queries.device.device.get_query_pool_results(
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
                        self.queries.device.timelines.mark_device_lost();
                    }
                    return Err(error.into());
                }
            }
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
mod tests {
    use super::*;
    #[test]
    fn timestamp_wrap_and_period_are_applied_without_treating_missing_as_zero() {
        assert_eq!(duration_ns([250, 4], 8, 2.0), Some(20));
        assert_eq!(duration_ns([10, 10], 64, 1.0), Some(0));
        assert_eq!(duration_ns([1, 3], 0, 1.0), None);
        assert_eq!(duration_ns([1, 3], 64, f32::NAN), None);
        assert_eq!(duration_ns([1, 3], 64, 0.0), None);
    }
}
