//! Host mapping and host-range access.
use ash::vk;
use crate::Error;
use crate::Result;
use super::Buffer;

/// Default bound for synchronous host reads/writes waiting on GPU work.
pub const HOST_ACCESS_TIMEOUT_NS: u64 = 10_000_000_000;

pub(super) struct HostRangeState<'a> {
    cell: &'a crate::tracking::SyncCell,
    accessed: bool,
}

impl Drop for HostRangeState<'_> {
    fn drop(&mut self) {
        if self.accessed {
            self.cell.publish_buffer_host_range();
        } else {
            self.cell.cancel_range_state();
        }
    }
}

impl Buffer {
    /// Whole-buffer mapped slice (`desc.size` bytes), or `None` for memory that
    /// is not host visible. Aliasing is the caller's responsibility; see
    /// [`Self::mapped_slice_mut`] for a sub-range.
    /// # Safety
    /// The caller must exclude all host/GPU accesses for the returned borrow,
    /// wait for previous uses, and flush non-coherent writes before submission.
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn mapped_full_slice_mut(&self) -> Option<&mut [u8]> {
        if self.device.is_device_lost() {
            return None;
        }
        if self.allocation.is_shared() {
            return None;
        }
        let len = usize::try_from(self.size).ok()?;
        if self.size > self.allocation.size {
            return None;
        }
        let ptr = self.persistent_host_ptr()?;
        // SAFETY: The mapping covers this live allocation; the caller provides
        // exclusive host/GPU access and non-coherent visibility.
        Some(unsafe { std::slice::from_raw_parts_mut(ptr, len) })
    }

    /// 请求持久映射；首次调用建立映射并保留到 `drop`（或下一次 `unmap`）。
    /// 返回的可变切片在下次映射/释放前保持有效，调用者负责同步边界。
    /// # Safety
    /// The caller must establish GPU completion and non-coherent visibility,
    /// and prevent submissions from using the mapped region during this borrow.
    pub unsafe fn mapped_slice_mut(&mut self, offset: u64, size: u64) -> Result<&mut [u8]> {
        if self.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        if self.allocation.is_shared() {
            return Err(Error::unsupported(
                "graph transient memory cannot be mapped",
            ));
        }
        let end = offset
            .checked_add(size)
            .ok_or(Error::invalid_usage("映射范围上溢"))?;
        if end > self.size {
            return Err(Error::invalid_usage("映射访问越过缓冲区边界"));
        }
        if !self
            .allocation
            .properties
            .contains(vk::MemoryPropertyFlags::HOST_VISIBLE)
        {
            return Err(Error::unsupported("内存不可映射"));
        }
        let address = match self.mapped_ptr {
            Some(address) => address,
            // 子分配：gpu-allocator 已持久映射整块，指针已含偏移；不能再 vkMapMemory 同一块内存。
            None if self.allocation.host_ptr().is_some() => {
                self.allocation.host_ptr().map_or(0, |p| p as usize)
            }
            None => {
                self.device.wait(&self.last_use.snapshot(), u64::MAX)?;
                let pointer = crate::device::observe_vulkan(&self.device, unsafe {
                    self.device.core.device.map_memory(
                        self.allocation.handle,
                        0,
                        vk::WHOLE_SIZE,
                        vk::MemoryMapFlags::empty(),
                    )
                })?;
                let address = pointer as usize;
                self.mapped_ptr = Some(address);
                address
            }
        };
        // SAFETY: 上面已建立/复用本块持续映射；映射至少覆盖整个内存对象，
        // 返回的借用只在调用方持有时有效，期间不允许再 unmap 或 drop。
        let start = address as *mut u8;
        let start = unsafe { start.add(offset as usize) };
        Ok(unsafe { std::slice::from_raw_parts_mut(start, size as usize) })
    }

    /// Persistent host pointer to the buffer start (suballocated or mapped
    /// dedicated memory); `None` for memory that is not host visible.
    /// 调用者负责同步，并保证指针只在本缓冲存活期间使用。
    pub fn persistent_host_ptr(&self) -> Option<*mut u8> {
        if self.device.is_device_lost() {
            return None;
        }
        self.allocation
            .host_ptr()
            .map(|p| p.cast())
            .or(self.mapped_ptr.map(|address| address as *mut u8))
    }

    /// 释放持久映射；不撤销借用（调用者必须先结束对切片的借用）。
    pub fn unmap(&mut self) {
        if self.mapped_ptr.take().is_some() {
            unsafe {
                self.device.core.device.unmap_memory(self.allocation.handle);
            }
        }
    }

    /// 宿主刷新/失效范围 `(偏移, 大小)`，供测试核对子分配的对齐前提。
    #[doc(hidden)]
    pub fn host_range_for_test(&self) -> (u64, u64) {
        self.allocation.host_range()
    }

    /// Map-and-copy core shared by all host reads and writes.
    ///
    /// The overlapping GPU uses are checked against the cached completion
    /// snapshot first and the device is only polled when that snapshot is
    /// insufficient. Waiting never holds `host_access` or the submission
    /// gate: the locks are released, the wait is bounded by `wait_ns`
    /// (`None` = do not wait, return `Timeout`), and the range is re-checked
    /// under the gate before copying. Non-coherent memory is invalidated or
    /// flushed once for the whole `[offset, offset + len)` range.
    pub(super) fn host_access(
        &self,
        offset: u64,
        len: u64,
        write: bool,
        wait_ns: Option<u64>,
        copy: impl FnOnce(*mut u8),
    ) -> Result<()> {
        self.host_access_with(offset, len, write, false, wait_ns, copy)
    }

    /// [`Self::host_access`]; `defer_flush` queues non-coherent written atoms
    /// on the device instead of flushing them immediately.
    pub(super) fn host_access_with(
        &self,
        offset: u64,
        len: u64,
        write: bool,
        defer_flush: bool,
        wait_ns: Option<u64>,
        copy: impl FnOnce(*mut u8),
    ) -> Result<()> {
        if self.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        if self.allocation.is_shared() {
            return Err(Error::unsupported(
                "graph transient memory cannot be mapped",
            ));
        }
        if offset.checked_add(len).is_none_or(|end| end > self.size) {
            return Err(Error::invalid_usage("映射访问越过缓冲区边界"));
        }
        if !self
            .allocation
            .properties
            .contains(vk::MemoryPropertyFlags::HOST_VISIBLE)
        {
            return Err(Error::unsupported("内存不可映射"));
        }
        if len == 0 {
            return Ok(());
        }
        let coherent = self
            .allocation
            .properties
            .contains(vk::MemoryPropertyFlags::HOST_COHERENT);
        let host_range = self.allocation.host_subrange(
            self.size,
            offset,
            len,
            if coherent {
                1
            } else {
                self.device.limits().non_coherent_atom_size.max(1)
            },
        )?;
        let wait_started = wait_ns.map(|ns| (std::time::Instant::now(), ns));
        let mut completed = self.device.completed();
        let mut polled = false;
        loop {
            let host = self.host_access.lock();
            let reservation = self.last_use.lock_submission();
            let pending = self.last_use.buffer_range_snapshot(
                host_range.buffer_offset,
                host_range.buffer_size,
                write,
                &completed,
            )?;
            if pending.is_empty() {
                let flush = if write {
                    crate::device::host_flush::host_write_flush(
                        self.allocation.properties,
                        defer_flush,
                    )
                } else {
                    crate::device::host_flush::HostFlush::None
                };
                let result = self.copy_locked(&host_range, coherent, write, flush, copy);
                drop(reservation);
                drop(host);
                return result;
            }
            drop(reservation);
            drop(host);
            if !polled {
                // The cached snapshot is stale-conservative; observe progress once.
                completed = self.device.poll()?;
                polled = true;
                continue;
            }
            let Some((started, budget_ns)) = wait_started else {
                return Err(Error::Timeout);
            };
            let remaining = budget_ns
                .saturating_sub(started.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64);
            self.device.wait(&pending, remaining)?;
            completed = self.device.completed();
        }
    }

    /// Copy with `host_access` and the submission gate held by the caller.
    fn copy_locked(
        &self,
        host_range: &crate::device::allocator::HostMemoryRange,
        coherent: bool,
        write: bool,
        flush: crate::device::host_flush::HostFlush,
        copy: impl FnOnce(*mut u8),
    ) -> Result<()> {
        self.last_use.prepare_buffer_host_range(
            host_range.buffer_offset,
            host_range.buffer_size,
            write,
        )?;
        let mut host_state = HostRangeState {
            cell: &self.last_use,
            accessed: false,
        };
        // Suballocations and dedicated host-visible buffers are persistently
        // mapped; map temporarily only when neither exists.
        let shared_pointer = self
            .allocation
            .host_ptr()
            .or(self.mapped_ptr.map(|a| a as *mut u8));
        let pointer = match shared_pointer {
            Some(pointer) => pointer.cast(),
            None => crate::device::observe_vulkan(&self.device, unsafe {
                self.device.core.device.map_memory(
                    self.allocation.handle,
                    0,
                    vk::WHOLE_SIZE,
                    vk::MemoryMapFlags::empty(),
                )
            })?,
        };
        let range = [vk::MappedMemoryRange::default()
            .memory(self.allocation.handle)
            .offset(host_range.memory_offset)
            .size(host_range.memory_size)];
        let unmap = || {
            if shared_pointer.is_none() {
                unsafe { self.device.core.device.unmap_memory(self.allocation.handle) };
            }
        };
        let result = if !write && !coherent {
            unsafe {
                self.device
                    .core
                    .device
                    .invalidate_mapped_memory_ranges(&range)
            }
        } else {
            Ok(())
        };
        if let Err(error) = crate::device::observe_vulkan(&self.device, result) {
            unmap();
            return Err(error);
        }
        copy(pointer.cast::<u8>());
        // A failing flush does not undo the CPU copy. Publish the conservative
        // host revision even on that error, so bound native programs go stale.
        host_state.accessed = true;
        match flush {
            crate::device::host_flush::HostFlush::None => {
                unmap();
                Ok(())
            }
            crate::device::host_flush::HostFlush::Now => {
                let result = unsafe { self.device.core.device.flush_mapped_memory_ranges(&range) };
                unmap();
                crate::device::observe_vulkan(&self.device, result)
            }
            // Deferred flushes require a persistent mapping (constants pages).
            crate::device::host_flush::HostFlush::Deferred if shared_pointer.is_some() => {
                self.device.queues.host_flush.record(
                    self.allocation.accounting_id(),
                    self.allocation.handle,
                    host_range.memory_offset,
                    host_range.memory_size,
                )
            }
            crate::device::host_flush::HostFlush::Deferred => {
                let result = unsafe { self.device.core.device.flush_mapped_memory_ranges(&range) };
                unmap();
                crate::device::observe_vulkan(&self.device, result)
            }
        }
    }
}
