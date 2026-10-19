//! Narrow ownership for graph-local aliased allocations. Deferred owners contain
//! an ash dispatch table and accounting only, never an `Arc<Device>` back edge.
use super::{
    allocator::{AllocatorStats, MemoryBlock, choose_memory_type},
    device::Device,
};
use ash::vk;
use std::sync::{
    Arc,
    atomic::{AtomicU64, AtomicUsize, Ordering},
};

#[derive(Default)]
pub struct TransientBackingCounters {
    allocations: AtomicU64,
    frees: AtomicU64,
    allocated_bytes: AtomicU64,
    freed_bytes: AtomicU64,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// Unique shared backing ranges, not the parent suballocator's native blocks.
pub struct TransientBackingStatistics {
    pub allocations: u64,
    pub frees: u64,
    pub allocated_bytes: u64,
    pub freed_bytes: u64,
}
impl TransientBackingCounters {
    pub fn snapshot(&self) -> TransientBackingStatistics {
        TransientBackingStatistics {
            allocations: self.allocations.load(Ordering::Relaxed),
            frees: self.frees.load(Ordering::Relaxed),
            allocated_bytes: self.allocated_bytes.load(Ordering::Relaxed),
            freed_bytes: self.freed_bytes.load(Ordering::Relaxed),
        }
    }
}

pub struct SharedBacking {
    device: ash::Device,
    stats: Arc<AllocatorStats>,
    pub(crate) handle: vk::DeviceMemory,
    pub(crate) size: u64,
    pub(crate) offset: u64,
    #[cfg(feature = "suballocation")]
    sub: Option<super::suballoc::SubAllocation>,
    pub(crate) properties: vk::MemoryPropertyFlags,
    pub(crate) accounting_id: u64,
    pub(crate) memory_type: u32,
    pub(crate) last_use: Arc<crate::resource::SyncCell>,
    live_members: AtomicUsize,
    counters: Arc<TransientBackingCounters>,
}
impl SharedBacking {
    pub(crate) fn allocate(
        device: &Arc<Device>,
        requirements: vk::MemoryRequirements,
        flags: vk::MemoryAllocateFlags,
        linear: bool,
        counters: Arc<TransientBackingCounters>,
    ) -> crate::Result<Arc<Self>> {
        if device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        if requirements.size == 0
            || requirements.alignment == 0
            || !requirements.alignment.is_power_of_two()
        {
            return Err(crate::Error::invalid_usage(
                "invalid transient allocation requirements",
            ));
        }
        if flags.contains(vk::MemoryAllocateFlags::DEVICE_ADDRESS)
            && !device.core.capabilities.buffer_device_address
        {
            return Err(crate::Error::unsupported(
                "transient device-address allocation is unavailable",
            ));
        }
        let sync_storage = crate::resource::SyncCell::reserve_arc_storage(&device.memory.stats)?;
        let memory_type = choose_memory_type(
            &device.core.memory_properties,
            requirements.memory_type_bits,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        )
        .ok_or_else(|| {
            crate::Error::unsupported("transient resources have no common device-local memory type")
        })?;
        let required = vk::MemoryPropertyFlags::DEVICE_LOCAL;
        #[cfg(feature = "suballocation")]
        let block = if device.memory.suballocator.is_some() {
            MemoryBlock::allocate_sub(
                device.clone(),
                requirements,
                required,
                required,
                flags,
                linear,
            )?
        } else {
            MemoryBlock::allocate_dedicated(
                device.clone(),
                requirements,
                required,
                required,
                flags,
            )?
        };
        #[cfg(not(feature = "suballocation"))]
        let block = {
            let _ = linear;
            MemoryBlock::allocate_dedicated(
                device.clone(),
                requirements,
                required,
                required,
                flags,
            )?
        };
        #[allow(unused_mut)] // Taking the suballocation owner is feature-gated.
        let mut block = std::mem::ManuallyDrop::new(block);
        let handle = block.handle;
        let offset = block.offset;
        let size = block.size;
        let properties = block.properties;
        let accounting_id = block.accounting_id;
        #[cfg(feature = "suballocation")]
        let sub = block.sub.take();
        // The backing owns the exact allocation, not the Device. Its surviving
        // member resources/retirement queue keep VkDevice alive until free.
        drop(unsafe { std::ptr::read(&block.device) });
        counters.allocations.fetch_add(1, Ordering::Relaxed);
        counters.allocated_bytes.fetch_add(size, Ordering::Relaxed);
        Ok(Arc::new(Self {
            device: device.core.device.clone(),
            stats: device.memory.stats.clone(),
            handle,
            size,
            offset,
            #[cfg(feature = "suballocation")]
            sub,
            properties,
            accounting_id,
            memory_type,
            last_use: crate::resource::SyncCell::for_allocation(
                device.generation(),
                device.memory.stats.clone(),
                accounting_id,
            )
            .into_reserved_arc(sync_storage),
            live_members: AtomicUsize::new(0),
            counters,
        }))
    }
    #[cfg(feature = "suballocation")]
    pub(crate) fn is_suballocated(&self) -> bool {
        self.sub.is_some()
    }
    #[cfg(feature = "suballocation")]
    pub(crate) fn host_ptr(&self) -> Option<*mut u8> {
        self.sub
            .as_ref()
            .and_then(|allocation| allocation.mapped_ptr())
            .map(|pointer| pointer.as_ptr().cast::<u8>())
    }
    pub(crate) fn add_member(&self) {
        self.live_members.fetch_add(1, Ordering::Relaxed);
    }
    pub(crate) fn retire_member(&self) {
        if self.live_members.fetch_sub(1, Ordering::AcqRel) == 1 {
            self.stats.mark_retiring(self.accounting_id);
        }
    }
}
impl Drop for SharedBacking {
    fn drop(&mut self) {
        debug_assert_eq!(self.live_members.load(Ordering::Relaxed), 0);
        #[cfg(feature = "suballocation")]
        let suballocated = if let Some(allocation) = self.sub.take() {
            drop(allocation);
            true
        } else {
            false
        };
        #[cfg(not(feature = "suballocation"))]
        let suballocated = false;
        if !suballocated {
            unsafe {
                self.device.free_memory(self.handle, None);
            }
            self.stats
                .release(self.accounting_id)
                .expect("shared backing accounting is released once");
        }
        self.counters.frees.fetch_add(1, Ordering::Relaxed);
        self.counters
            .freed_bytes
            .fetch_add(self.size, Ordering::Relaxed);
    }
}
