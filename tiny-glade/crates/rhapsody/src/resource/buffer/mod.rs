//! Buffer creation, memory binding and mapped host reads/writes.
use crate::device::allocator::MemoryBlock;
use crate::device::Device;
use ash::vk;
use std::sync::Arc;
mod memory;
mod desc;
mod update;
mod create;
mod dynamic;
mod host;
mod write;
mod read;
pub use memory::{MemoryLocation, USAGE_ALIGN_64_BIT_RAW, buffer_alignment, create_failure};
pub use desc::BufferDesc;
pub use update::{BufferUpdateError, BufferRegion, BufferUpdatePlan, plan_buffer_update};
pub use host::HOST_ACCESS_TIMEOUT_NS;
const _: () = assert!(std::mem::size_of::<BufferDesc>() == 16);

pub struct Buffer {
    retirement: Option<crate::device::RetireTicket>,
    dynamic_constants_page: bool,
    pub(crate) sharing: crate::tracking::ResourceSharing,
    allocation_use: Option<Arc<crate::tracking::SyncCell>>,
    pub(crate) last_use: Arc<crate::tracking::SyncCell>,
    pub(crate) device: Arc<Device>,
    pub(crate) handle: vk::Buffer,
    pub size: u64,
    pub usage: vk::BufferUsageFlags,
    /// Creation description (graph imports need alignment/memory_location).
    pub desc: BufferDesc,
    allocation: std::mem::ManuallyDrop<MemoryBlock>,
    mapped_ptr: Option<usize>,
    host_access: parking_lot::Mutex<()>,
    _memory: crate::accounting::CpuMemoryReservation,
}

impl Buffer {
    /// The device that owns this object.
    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }
    /// Interop escape hatch: the native handle. It stays owned by `self`; never destroy it or use it after `self` is dropped.
    pub fn raw_handle(&self) -> vk::Buffer {
        self.handle
    }
}

