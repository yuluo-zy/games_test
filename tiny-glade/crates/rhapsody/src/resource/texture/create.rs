//! Texture creation, sharing, swapchain import and lifetime.
use ash::vk;
use crate::Error;
use crate::Result;
use crate::device::Device;
use crate::device::allocator::MemoryBlock;
use std::sync::Arc;
use super::Texture;
use super::desc::TextureDesc;
use super::desc::TextureViewDesc;
use super::info::get_image_create_info_with_data;
use super::view::TextureView;

pub(super) enum TextureStorage {
    Allocated(std::mem::ManuallyDrop<MemoryBlock>),
    // The swapchain, not this texture wrapper, destroys the VkImage. A strong
    // owner also protects the image while graph imports and views retain it.
    Swapchain(
        #[allow(dead_code, reason = "keeps the swapchain image alive")]
        Arc<dyn std::any::Any + Send + Sync>,
    ),
}

impl Drop for Texture {
    fn drop(&mut self) {
        if let TextureStorage::Allocated(allocation) = &mut self.storage {
            // SAFETY: this owned allocation is taken exactly once.
            let memory = unsafe { std::mem::ManuallyDrop::take(allocation) }.into_deferred();
            self.device.retire_reserved(
                self.last_use.snapshot(),
                crate::device::graveyard::Doomed::Image(self.handle, memory),
                self.retirement.take().expect("owned image retirement node"),
            );
        }
    }
}

impl crate::tracking::GpuUse for Texture {
    fn visit_uses(&self, visitor: &mut crate::tracking::UseVisitor<'_>) -> crate::Result<()> {
        crate::tracking::tracked::visit_cells(
            visitor,
            std::iter::once(&self.last_use).chain(self.allocation_use.iter()),
        )
    }
}

impl Texture {
    pub(crate) fn ownership_range(desc: &TextureDesc) -> Result<crate::tracking::OwnershipRange> {
        Ok(crate::tracking::OwnershipRange::Image {
            aspects: crate::format::vk_format_valid_aspect_flags(desc.format).as_raw(),
            base_mip: 0,
            mip_count: u32::from(desc.mip_levels),
            base_layer: 0,
            layer_count: desc.array_layers()?,
        })
    }

    pub(crate) fn allocation_id(&self) -> u64 {
        match &self.storage {
            TextureStorage::Allocated(allocation) => allocation.accounting_id(),
            TextureStorage::Swapchain(_) => panic!(
                "swapchain images have no allocator allocation; they must never enter the transient/result resource pool"
            ),
        }
    }

    /// The caller owns an already-bound handle and transfers its sole raw-object ownership.
    pub(crate) unsafe fn from_bound_transient(
        device: Arc<Device>,
        desc: TextureDesc,
        handle: vk::Image,
        allocation: MemoryBlock,
        sharing: crate::tracking::ResourceSharing,
        range: crate::tracking::OwnershipRange,
        storage: crate::device::ResourceStorage,
    ) -> Self {
        let allocation_use = allocation.shared_use();
        let last_use = (if sharing.is_concurrent() {
            crate::tracking::SyncCell::for_device(device.generation())
                .with_concurrent_families(match sharing {
                    crate::tracking::ResourceSharing::Concurrent(families) => families,
                    _ => unreachable!("concurrent branch"),
                })
                .with_ownership_range(range)
        } else {
            crate::tracking::SyncCell::for_device(device.generation()).with_ownership_range(range)
        })
        .into_reserved_arc(storage.synchronization);
        Self {
            _memory: storage.object,
            retirement: Some(storage.retirement),
            sharing,
            allocation_use,
            last_use,
            device,
            handle,
            desc,
            storage: TextureStorage::Allocated(std::mem::ManuallyDrop::new(allocation)),
        }
    }

    /// The WSI lease is the only producer of this non-owning image wrapper.
    /// It retains the actual swapchain generation, never allocates image memory
    /// and never calls vkDestroyImage.
    pub(crate) fn from_swapchain_image(
        owner: Arc<dyn std::any::Any + Send + Sync>,
        device: Arc<Device>,
        handle: vk::Image,
        desc: TextureDesc,
    ) -> Result<Self> {
        if handle == vk::Image::null() {
            return Err(Error::invalid_usage("cannot borrow a null swapchain image"));
        }
        let range = Self::ownership_range(&desc)?;
        let synchronization = crate::tracking::SyncCell::reserve_arc_storage(&device.memory.stats)?;
        let memory =
            crate::accounting::reserve_shared_cpu_metadata::<Self>(&device.memory.stats, 0)?;
        Ok(Self {
            sharing: crate::tracking::ResourceSharing::Exclusive,
            allocation_use: None,
            last_use: crate::tracking::SyncCell::for_device(device.generation())
                .with_ownership_range(range)
                .into_reserved_arc(synchronization),
            device,
            handle,
            desc,
            storage: TextureStorage::Swapchain(owner),
            retirement: None,
            _memory: memory,
        })
    }

    pub fn new(device: Arc<Device>, desc: &TextureDesc) -> Result<Self> {
        Self::new_with_initial_data(device, desc, false)
    }

    /// Create storage for a subsequent staging upload; creation always starts
    /// in UNDEFINED, and the caller records the upload and final layout.
    pub fn new_with_initial_data(
        device: Arc<Device>,
        desc: &TextureDesc,
        has_data: bool,
    ) -> Result<Self> {
        Self::new_with_sharing(
            device,
            desc,
            has_data,
            crate::tracking::ResourceSharing::Exclusive,
        )
    }

    pub fn new_concurrent(
        device: Arc<Device>,
        desc: &TextureDesc,
        families: &[u32],
    ) -> Result<Self> {
        Self::new_with_sharing(
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

    fn new_with_sharing(
        device: Arc<Device>,
        desc: &TextureDesc,
        has_data: bool,
        sharing: crate::tracking::ResourceSharing,
    ) -> Result<Self> {
        if device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        device.validate_sharing(sharing)?;
        desc.validate()?;
        let range = Self::ownership_range(desc)?;
        let sync_storage = crate::tracking::SyncCell::reserve_arc_storage(&device.memory.stats)?;
        let memory = device.memory.stats.reserve_cpu_bytes(
            crate::accounting::shared_cpu_bytes::<Self>()?,
            crate::accounting::BudgetCategory::Live,
        )?;
        let retirement = device.reserve_retirement()?;
        let info = get_image_create_info_with_data(desc, has_data)?
            .sharing_mode(if sharing.is_concurrent() {
                vk::SharingMode::CONCURRENT
            } else {
                vk::SharingMode::EXCLUSIVE
            })
            .queue_family_indices(sharing.families());
        let handle = crate::device::observe_vulkan(&device, unsafe {
            device.core.device.create_image(&info, None)
        })?;
        let mut dedicated = vk::MemoryDedicatedRequirements::default();
        let mut requirements2 = vk::MemoryRequirements2::default().push_next(&mut dedicated);
        unsafe {
            device.core.device.get_image_memory_requirements2(
                &vk::ImageMemoryRequirementsInfo2::default().image(handle),
                &mut requirements2,
            );
        }
        let requirements = requirements2.memory_requirements;
        let allocated = if dedicated.requires_dedicated_allocation != 0 {
            MemoryBlock::allocate_for_resource(
                &device,
                requirements,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
                vk::MemoryAllocateFlags::empty(),
                vk::Buffer::null(),
                handle,
            )
        } else {
            MemoryBlock::allocate(
                Arc::clone(&device),
                requirements,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
            )
        };
        let allocation = match allocated {
            Ok(value) => value,
            Err(error) => {
                unsafe {
                    device.core.device.destroy_image(handle, None);
                }
                return Err(error);
            }
        };
        if let Err(error) = unsafe {
            device
                .core
                .device
                .bind_image_memory(handle, allocation.handle, allocation.offset)
        } {
            unsafe {
                device.core.device.destroy_image(handle, None);
            }
            return crate::device::observe_vulkan(&device, Err(error));
        }
        Ok(Self {
            retirement: Some(retirement),
            _memory: memory,
            sharing,
            allocation_use: None,
            last_use: (if sharing.is_concurrent() {
                crate::tracking::SyncCell::for_allocation(
                    device.generation(),
                    device.memory.stats.clone(),
                    allocation.accounting_id(),
                )
                .with_concurrent_families(match sharing {
                    crate::tracking::ResourceSharing::Concurrent(families) => families,
                    _ => unreachable!("concurrent branch"),
                })
                .with_ownership_range(range)
            } else {
                crate::tracking::SyncCell::for_allocation(
                    device.generation(),
                    device.memory.stats.clone(),
                    allocation.accounting_id(),
                )
                .with_ownership_range(range)
            })
            .into_reserved_arc(sync_storage),
            device,
            handle,
            desc: *desc,
            storage: TextureStorage::Allocated(std::mem::ManuallyDrop::new(allocation)),
        })
    }

    pub fn image_view(self: &Arc<Self>, desc: &TextureViewDesc) -> Result<TextureView> {
        if self.device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        desc.validate(&self.desc)?;
        if desc.view_type == vk::ImageViewType::CUBE_ARRAY {
            self.device
                .caps()
                .require_enabled(crate::device::Feature::ImageCubeArray)?;
        }
        let memory = self.device.memory.stats.reserve_cpu_bytes(
            crate::accounting::shared_cpu_bytes::<TextureView>()?,
            crate::accounting::BudgetCategory::Live,
        )?;
        let retirement = self.device.reserve_retirement()?;
        let range = vk::ImageSubresourceRange::default()
            .aspect_mask(desc.aspect)
            .base_mip_level(desc.base_mip_level)
            .level_count(desc.level_count)
            .base_array_layer(desc.base_layer)
            .layer_count(desc.layer_count);
        // A mutable-format sampled view need not support the image's real
        // storage/attachment purposes. Honor the caller's existing view usage;
        // never silently intersect away an explicitly requested purpose.
        let mut usage = vk::ImageViewUsageCreateInfo::default().usage(desc.usage);
        let info = vk::ImageViewCreateInfo::default()
            .push_next(&mut usage)
            .image(self.handle)
            .view_type(desc.view_type)
            .format(desc.format)
            .subresource_range(range);
        let sync_storage =
            crate::tracking::SyncCell::reserve_arc_storage(&self.device.memory.stats)?;
        let handle = crate::device::observe_vulkan(&self.device, unsafe {
            self.device.core.device.create_image_view(&info, None)
        })?;
        Ok(TextureView {
            retirement: Some(retirement),
            _memory: memory,
            last_use: crate::tracking::SyncCell::for_device(self.device.generation())
                .into_reserved_arc(sync_storage),
            texture: Arc::clone(self),
            handle,
            desc: *desc,
        })
    }

    pub fn allocation_bytes(&self) -> u64 {
        match &self.storage {
            TextureStorage::Allocated(allocation) => allocation.size,
            TextureStorage::Swapchain(_) => 0,
        }
    }
}
