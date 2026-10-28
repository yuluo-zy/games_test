//! Buffer creation, sharing and lifetime.
use ash::vk;
use crate::Error;
use crate::Result;
use crate::device::Device;
use crate::device::allocator::MemoryBlock;
use crate::device::graveyard::Doomed;
use std::sync::Arc;
use super::Buffer;
use super::desc::BufferDesc;
use super::memory::buffer_alignment;

impl Drop for Buffer {
    fn drop(&mut self) {
        if self.dynamic_constants_page {
            // Unsubmitted deferred flushes of a freed page are meaningless.
            self.device
                .queues
                .host_flush
                .forget(self.allocation.accounting_id());
        }
        if self.mapped_ptr.take().is_some() {
            unsafe {
                self.device.core.device.unmap_memory(self.allocation.handle);
            }
        }
        // SAFETY: allocation 只在此处取出一次，之后不会再被访问。
        let memory = unsafe { std::mem::ManuallyDrop::take(&mut self.allocation) }.into_deferred();
        self.device.retire_reserved(
            self.last_use.snapshot(),
            Doomed::Buffer(self.handle, memory),
            self.retirement
                .take()
                .expect("owned buffer retirement node"),
        );
    }
}

impl crate::tracking::GpuUse for Buffer {
    fn visit_uses(&self, visitor: &mut crate::tracking::UseVisitor<'_>) -> crate::Result<()> {
        crate::tracking::tracked::visit_cells(
            visitor,
            std::iter::once(&self.last_use).chain(self.allocation_use.iter()),
        )
    }
}

impl Buffer {
    pub fn new(device: Arc<Device>, desc: &BufferDesc) -> Result<Self> {
        Self::new_with_memory(
            device,
            desc,
            false,
            crate::tracking::ResourceSharing::Exclusive,
        )
    }

    pub(crate) fn new_dedicated(device: Arc<Device>, desc: &BufferDesc) -> Result<Self> {
        Self::new_with_memory(
            device,
            desc,
            true,
            crate::tracking::ResourceSharing::Exclusive,
        )
    }

    pub fn new_concurrent(
        device: Arc<Device>,
        desc: &BufferDesc,
        families: &[u32],
    ) -> Result<Self> {
        Self::new_with_memory(
            device,
            desc,
            false,
            crate::tracking::ResourceSharing::Concurrent(crate::tracking::QueueFamilies::new(
                families,
            )?),
        )
    }

    pub fn sharing(&self) -> crate::tracking::ResourceSharing {
        self.sharing
    }

    fn new_with_memory(
        device: Arc<Device>,
        desc: &BufferDesc,
        dedicated: bool,
        sharing: crate::tracking::ResourceSharing,
    ) -> Result<Self> {
        if device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        device.validate_sharing(sharing)?;
        if desc.size == 0 {
            return Err(Error::invalid_usage("缓冲区大小不能为零"));
        }
        if !device.core.capabilities.buffer_device_address
            && desc
                .usage
                .contains(vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS)
        {
            return Err(Error::unsupported("此路径未启用 bufferDeviceAddress"));
        }
        let sync_storage = crate::tracking::SyncCell::reserve_arc_storage(&device.memory.stats)?;
        let memory = device.memory.stats.reserve_cpu_bytes(
            crate::accounting::shared_cpu_bytes::<Self>()?,
            crate::accounting::BudgetCategory::Live,
        )?;
        let retirement = device.reserve_retirement()?;
        let info = vk::BufferCreateInfo::default()
            .size(desc.size)
            .usage(desc.usage)
            .sharing_mode(if sharing.is_concurrent() {
                vk::SharingMode::CONCURRENT
            } else {
                vk::SharingMode::EXCLUSIVE
            })
            .queue_family_indices(sharing.families());
        let handle = crate::device::observe_vulkan(&device, unsafe {
            device.core.device.create_buffer(&info, None)
        })?;
        let mut dedicated_requirements = vk::MemoryDedicatedRequirements::default();
        let mut requirements2 =
            vk::MemoryRequirements2::default().push_next(&mut dedicated_requirements);
        unsafe {
            device.core.device.get_buffer_memory_requirements2(
                &vk::BufferMemoryRequirementsInfo2::default().buffer(handle),
                &mut requirements2,
            );
        }
        let mut requirements = requirements2.memory_requirements;
        requirements.alignment =
            buffer_alignment(requirements.alignment, desc.alignment, desc.usage.as_raw());
        let flags = if desc
            .usage
            .contains(vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS)
        {
            vk::MemoryAllocateFlags::DEVICE_ADDRESS
        } else {
            vk::MemoryAllocateFlags::empty()
        };
        let (required_memory, preferred_memory) = desc.memory_location.memory_properties();
        let allocated = if dedicated_requirements.requires_dedicated_allocation != 0 {
            MemoryBlock::allocate_for_resource(
                &device,
                requirements,
                required_memory,
                preferred_memory,
                flags,
                handle,
                vk::Image::null(),
            )
        } else if dedicated {
            MemoryBlock::allocate_dedicated(
                device.clone(),
                requirements,
                required_memory,
                preferred_memory,
                flags,
            )
        } else {
            MemoryBlock::allocate_with_flags(
                Arc::clone(&device),
                requirements,
                required_memory,
                preferred_memory,
                flags,
            )
        };
        let allocation = match allocated {
            Ok(value) => value,
            Err(error) => {
                unsafe {
                    device.core.device.destroy_buffer(handle, None);
                }
                return Err(error);
            }
        };
        let last_use = if sharing.is_concurrent() {
            crate::tracking::SyncCell::for_allocation(
                device.generation(),
                device.memory.stats.clone(),
                allocation.accounting_id(),
            )
            .with_concurrent_families(match sharing {
                crate::tracking::ResourceSharing::Concurrent(families) => families,
                _ => unreachable!("concurrent branch"),
            })
            .with_ownership_range(crate::tracking::OwnershipRange::Buffer {
                offset: 0,
                size: desc.size,
            })
        } else {
            crate::tracking::SyncCell::for_allocation(
                device.generation(),
                device.memory.stats.clone(),
                allocation.accounting_id(),
            )
            .with_ownership_range(crate::tracking::OwnershipRange::Buffer {
                offset: 0,
                size: desc.size,
            })
        };
        let last_use = if allocation
            .properties
            .contains(vk::MemoryPropertyFlags::HOST_VISIBLE)
        {
            match last_use.with_host_buffer_ranges(desc.size, &device.memory.stats) {
                Ok(cell) => cell,
                Err(error) => {
                    unsafe {
                        device.core.device.destroy_buffer(handle, None);
                    }
                    return Err(error);
                }
            }
        } else {
            last_use
        };
        if let Err(error) = unsafe {
            device
                .core
                .device
                .bind_buffer_memory(handle, allocation.handle, allocation.offset)
        } {
            unsafe {
                device.core.device.destroy_buffer(handle, None);
            }
            return crate::device::observe_vulkan(&device, Err(error));
        }
        // Host-visible dedicated memory is mapped once here and stays mapped;
        // suballocations use gpu-allocator's persistent block mapping.
        let mapped_ptr = if allocation
            .properties
            .contains(vk::MemoryPropertyFlags::HOST_VISIBLE)
            && allocation.host_ptr().is_none()
        {
            match unsafe {
                device.core.device.map_memory(
                    allocation.handle,
                    0,
                    vk::WHOLE_SIZE,
                    vk::MemoryMapFlags::empty(),
                )
            } {
                Ok(pointer) => Some(pointer as usize),
                Err(error) => {
                    unsafe { device.core.device.destroy_buffer(handle, None) };
                    return crate::device::observe_vulkan(&device, Err(error));
                }
            }
        } else {
            None
        };
        Ok(Self {
            dynamic_constants_page: false,
            _memory: memory,
            retirement: Some(retirement),
            sharing,
            allocation_use: None,
            last_use: last_use.into_reserved_arc(sync_storage),
            device,
            handle,
            size: desc.size,
            usage: desc.usage,
            desc: *desc,
            allocation: std::mem::ManuallyDrop::new(allocation),
            mapped_ptr,
            host_access: parking_lot::Mutex::new(()),
        })
    }

    /// The caller owns an already-bound handle and transfers its sole raw-object ownership.
    pub(crate) unsafe fn from_bound_transient(
        device: Arc<Device>,
        desc: BufferDesc,
        handle: vk::Buffer,
        allocation: MemoryBlock,
        sharing: crate::tracking::ResourceSharing,
        storage: crate::device::ResourceStorage,
    ) -> Self {
        let allocation_use = allocation.shared_use();
        let last_use = (if sharing.is_concurrent() {
            crate::tracking::SyncCell::for_device(device.generation())
                .with_concurrent_families(match sharing {
                    crate::tracking::ResourceSharing::Concurrent(families) => families,
                    _ => unreachable!("concurrent branch"),
                })
                .with_ownership_range(crate::tracking::OwnershipRange::Buffer {
                    offset: 0,
                    size: desc.size,
                })
        } else {
            crate::tracking::SyncCell::for_device(device.generation()).with_ownership_range(
                crate::tracking::OwnershipRange::Buffer {
                    offset: 0,
                    size: desc.size,
                },
            )
        })
        .into_reserved_arc(storage.synchronization);
        Self {
            dynamic_constants_page: false,
            _memory: storage.object,
            retirement: Some(storage.retirement),
            sharing,
            allocation_use,
            last_use,
            device,
            handle,
            size: desc.size,
            usage: desc.usage,
            desc,
            allocation: std::mem::ManuallyDrop::new(allocation),
            mapped_ptr: None,
            host_access: parking_lot::Mutex::new(()),
        }
    }

    pub(crate) fn allocation_bytes(&self) -> u64 {
        self.allocation.size
    }

    pub(crate) fn allocation_id(&self) -> u64 {
        self.allocation.accounting_id()
    }

    /// Property flags of the memory type backing this buffer.
    pub fn memory_properties(&self) -> vk::MemoryPropertyFlags {
        self.allocation.properties
    }

    /// 地址只能用于具有 DEVICE_ADDRESS 分配标记且设备已启用 BDA 的缓冲。
    pub fn device_address(&self) -> Result<vk::DeviceAddress> {
        if self.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        if !self.device.core.capabilities.buffer_device_address
            || !self
                .usage
                .contains(vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS)
        {
            return Err(Error::unsupported("缓冲区未启用设备地址用途"));
        }
        let info = vk::BufferDeviceAddressInfo::default().buffer(self.handle);
        Ok(unsafe { self.device.core.device.get_buffer_device_address(&info) })
    }
}
