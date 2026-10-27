//! Command pool slots and per-worker command buffers.
use ash::vk;
use ash::vk::Handle;
use crate::command::command_buffer::VulkanCommandBuffer;
use crate::command::command_buffer::VulkanCommandPool;
use std::sync::Arc;
use super::FrameCommandPool;
use super::FramePoolKey;
use super::VulkanGraphBackend;
use super::super::session::COMMAND_SLOTS_PER_DISPLAY_FRAME;

impl VulkanGraphBackend {
    /// Command pool slots: one per in-flight graph. Once display frames are in
    /// use each of the `frames_in_flight` display frames may hold two graphs.
    pub(in crate::rg::vulkan) fn command_slot_count(&self) -> usize {
        self.device.frames_in_flight() as usize
            * if self.display_frames {
                COMMAND_SLOTS_PER_DISPLAY_FRAME as usize
            } else {
                1
            }
    }

    /// A new graph records into an available frame slot. CPU leases, rather than
    /// guessed frame counters, prevent resetting a pool still retained by a frame.
    pub(super) fn begin_command_frame(&mut self) -> crate::Result<()> {
        self.poll_gpu_timings()?;
        if !matches!(
            self.device.memory.stats.budget_action(),
            crate::accounting::BudgetAction::None
        ) {
            self.cpu_arenas.trim_idle(0);
            self.owner_pool.trim_idle(0);
            self.frame_storage.trim_idle(0);
            self.import_validation.get_mut().trim_idle();
            self.prepared_transients.trim_unaliased();
            self.caches.alias().trim();
        }
        // One slot per graph, not per display frame: a display frame holds the
        // main and the presentation graph, and `frames_in_flight` display frames
        // may be in flight at once.
        let slots = self.command_slot_count() as u32;
        let selected = (1..=slots)
            .map(|offset| (self.active_frame_slot + offset) % slots)
            .find(|slot| {
                !self.command_slot.iter().any(|(handle, key)| {
                    key.frame_slot == *slot && !self.recycled_commands.contains(handle)
                })
            })
            .ok_or_else(|| crate::Error::from(vk::Result::NOT_READY))?;
        self.recording_epoch = self
            .recording_epoch
            .checked_add(1)
            .ok_or_else(|| crate::Error::unsupported("command frame epoch exhausted"))?;
        self.active_frame_slot = selected;
        Ok(())
    }

    pub(super) fn command_for_worker(&mut self, worker: usize) -> crate::Result<u64> {
        let key = FramePoolKey {
            frame_slot: self.active_frame_slot,
            worker,
            family: self.recording_family,
        };
        if !self.frame_pools.contains_key(&key) {
            let pool =
                VulkanCommandPool::new_for_family(self.device.clone(), self.recording_family)?;
            self.frame_pools.insert(
                key,
                FrameCommandPool {
                    pool,
                    epoch: self.recording_epoch,
                },
            );
        }
        let entry = self
            .frame_pools
            .get(&key)
            .expect("inserted frame command pool");
        let pool = entry.pool.clone();
        if entry.epoch != self.recording_epoch {
            if self
                .command_slot
                .iter()
                .any(|(handle, slot)| *slot == key && !self.recycled_commands.contains(handle))
            {
                return Err(vk::Result::NOT_READY.into());
            }
            let mut commands = self
                .commands
                .values_mut()
                .filter(|command| Arc::ptr_eq(&command.pool, &pool))
                .collect::<smallvec::SmallVec<[&mut VulkanCommandBuffer; 4]>>();
            pool.reset_completed(&mut commands)?;
            self.frame_pools
                .get_mut(&key)
                .expect("existing frame command pool")
                .epoch = self.recording_epoch;
        }
        let position = self.recycled_commands.iter().rposition(|handle| {
            self.command_slot.get(handle) == Some(&key)
                && self
                    .commands
                    .get(handle)
                    .is_some_and(|command| !command.is_recording() && !command.is_executable())
                && self
                    .gpu_timing
                    .as_ref()
                    .is_none_or(|ring| !ring.is_tracking_handle(*handle))
        });
        if let Some(position) = position {
            return Ok(self.recycled_commands.remove(position));
        }
        let command = pool.allocate()?;
        let handle = command.handle.as_raw();
        self.commands.insert(handle, command);
        self.command_slot.insert(handle, key);
        Ok(handle)
    }
}
