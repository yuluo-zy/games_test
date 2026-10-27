//! Native pool state retained by deferred frees. These objects deliberately hold the raw
//! `ash::Device` and no `Arc<Device>`: the retirement queue that owns them is drained
//! before `vkDestroyDevice`, so they can sit inside [`Doomed`](crate::device::graveyard::Doomed)
//! without a reference cycle.
use ash::vk;
use parking_lot::Mutex;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

/// Pool storage retained by deferred command frees. It deliberately does not own
/// Arc<Device>; the retirement queue is drained before vkDestroyDevice.
///
/// Vulkan requires the `VkCommandPool` to be externally synchronized for every
/// pool-level call and for recording into any of its buffers. Recording
/// buffers are `&mut`-exclusive and the owning thread performs begin/end and
/// allocate/reset, all serialized by `lock`. The one call that can arrive from
/// an arbitrary thread is the deferred free issued by device retirement; while
/// any buffer of the pool is recording, such frees are queued and executed
/// when the last recording ends, so they never race the recording thread.
pub struct CommandPoolAllocation {
    pub(crate) device: ash::Device,
    pub(crate) handle: vk::CommandPool,
    pub(crate) lock: Mutex<PoolSync>,
    pub(crate) last_use: Arc<crate::tracking::SyncCell>,
    // Native commands still owned by a wrapper or detached cache. Retired
    // commands are protected by last_use and no longer need CPU exclusivity.
    pub(crate) live_owned_commands: AtomicUsize,
    pub(crate) pending_retirement_frees: AtomicUsize,
    pub(crate) _memory: crate::accounting::CpuMemoryReservation,
}
#[derive(Default)]
pub(crate) struct PoolSync {
    pub(crate) recording: usize,
}
impl CommandPoolAllocation {
    /// Maintenance never allocates a second queue while a pool is recording.
    /// The caller keeps the existing retirement node and retries next tick.
    pub(crate) fn try_free(&self, handle: vk::CommandBuffer, waiting: &mut bool) -> bool {
        let sync = self.lock.lock();
        if sync.recording != 0 {
            if !*waiting {
                self.pending_retirement_frees
                    .fetch_add(1, Ordering::Relaxed);
                *waiting = true;
            }
            return false;
        }
        if *waiting {
            self.pending_retirement_frees
                .fetch_sub(1, Ordering::Relaxed);
            *waiting = false;
        }
        unsafe {
            self.device.free_command_buffers(self.handle, &[handle]);
        }
        true
    }
    pub(crate) fn release_detached(&self) {
        self.live_owned_commands.fetch_sub(1, Ordering::Relaxed);
    }
    pub(crate) fn end_recording(&self, sync: &mut PoolSync) {
        sync.recording -= 1;
    }
}
impl Drop for CommandPoolAllocation {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_command_pool(self.handle, None);
        }
    }
}

/// Raw pool ownership without an Arc<Device> back-reference.
pub struct DescriptorPoolAllocation {
    pub(crate) _descriptor_quota: Option<crate::accounting::DescriptorReservation>,
    pub(crate) device: ash::Device,
    pub(crate) handle: vk::DescriptorPool,
    pub(crate) lock: Mutex<()>,
    pub(crate) _memory: crate::accounting::CpuMemoryReservation,
}
impl DescriptorPoolAllocation {
    pub(crate) fn free(&self, handle: vk::DescriptorSet) {
        let _guard = self.lock.lock();
        unsafe {
            let _ = self.device.free_descriptor_sets(self.handle, &[handle]);
        }
    }
}
impl Drop for DescriptorPoolAllocation {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_descriptor_pool(self.handle, None);
        }
    }
}
