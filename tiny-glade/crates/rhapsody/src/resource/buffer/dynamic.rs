//! Dynamic-constants pages backed by buffers.
use ash::vk;
use crate::Result;
use super::Buffer;

impl Buffer {
    /// Mark a new constants page and map it for its whole lifetime, so its
    /// non-coherent writes can defer their flush to the next submit.
    pub(crate) fn into_dynamic_constants_page(mut self) -> Result<Self> {
        // SAFETY: the buffer is new; no GPU or host access can overlap.
        unsafe {
            self.mapped_slice_mut(0, self.size)?;
        }
        self.dynamic_constants_page = true;
        Ok(self)
    }

    pub(crate) fn is_dynamic_constants_page(&self) -> bool {
        self.dynamic_constants_page
    }

    /// Fast path for a dynamic-constants page with no unfinished GPU use:
    /// the bytes a chunk covers were handed out by the page cursor after that
    /// was established, so no GPU access can overlap them and
    /// `vkQueueSubmit` makes the host write visible. Skips the per-write range
    /// table transaction (three locks, an O(cells) copy and a sort), which made
    /// a page of K chunks cost O(K^2). Returns `false` to use the exact path.
    pub(super) fn write_idle_constants_page(&self, offset: u64, bytes: &[u8]) -> Result<bool> {
        if bytes.is_empty() || self.device.is_device_lost() {
            return Ok(false);
        }
        if offset
            .checked_add(bytes.len() as u64)
            .is_none_or(|end| end > self.size)
        {
            return Ok(false);
        }
        let Some(pointer) = self
            .allocation
            .host_ptr()
            .or(self.mapped_ptr.map(|a| a as *mut u8))
        else {
            return Ok(false);
        };
        // Match host_access_with's lock order. A page cursor gives disjoint
        // allocations, but public chunk/buffer owners can revisit a range or
        // submit it concurrently. Hold both gates through the idle proof and
        // copy without rebuilding the native range table for every chunk.
        let _host = self.host_access.lock();
        let _submission = self.last_use.lock_submission();
        if !self.last_use.is_reached(&self.device.completed()) {
            return Ok(false);
        }
        let coherent = self
            .allocation
            .properties
            .contains(vk::MemoryPropertyFlags::HOST_COHERENT);
        let host_range = self.allocation.host_subrange(
            self.size,
            offset,
            bytes.len() as u64,
            if coherent {
                1
            } else {
                self.device.limits().non_coherent_atom_size.max(1)
            },
        )?;
        // SAFETY: the range is inside the persistent mapping (checked above)
        // and, per the page invariant, covered by no GPU access.
        unsafe {
            std::ptr::copy_nonoverlapping(
                bytes.as_ptr(),
                pointer.cast::<u8>().add(offset as usize),
                bytes.len(),
            )
        };
        if !coherent {
            self.device.queues.host_flush.record(
                self.allocation.accounting_id(),
                self.allocation.handle,
                host_range.memory_offset,
                host_range.memory_size,
            )?;
        }
        Ok(true)
    }
}
