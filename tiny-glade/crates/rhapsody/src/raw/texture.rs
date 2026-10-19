//! Textures, texture descriptions and image views.
use super::{
    allocator::MemoryBlock,
    device::Device,
    error::{Error, Result},
    format::vk_format_valid_aspect_flags,
};
use ash::vk;
use std::sync::Arc;

/// Aspect mask of a format: depth/stencil aspects for D16 / X8_D24 / D32 / S8 /
/// D16S8 / D24S8 / D32S8 (`{2,2,2,4,6,6,6}`), COLOR otherwise.
pub fn image_aspect_mask_from_format(format: vk::Format) -> vk::ImageAspectFlags {
    const TABLE: [u32; 7] = [2, 2, 2, 4, 6, 6, 6];
    let index = (format.as_raw() as u32).wrapping_sub(0x7c);
    vk::ImageAspectFlags::from_raw(TABLE.get(index as usize).copied().unwrap_or(1))
}

/// 1D 尺寸的全 mip 数量：32 - leading_zeros(size)。
pub fn mip_count_1d(size: u32) -> u32 {
    32 - size.leading_zeros()
}

/// Stage names of texture creation failures, for diagnostics built with
/// [`create_failure_text`]; creation itself returns `Err(Error::Vulkan(..))`.
pub mod create_failure {
    /// `vkCreateImage` failed.
    pub const CREATE_IMAGE: &str = "create_image";
    /// `vkBindImageMemory` failed.
    pub const BIND_IMAGE_MEMORY: &str = "bind_image_memory";
    /// `vkSetDebugUtilsObjectNameEXT` failed.
    pub const SET_DEBUG_UTILS_OBJECT_NAME: &str = "set_debug_utils_object_name";
}

/// `"{stage}: {result:?}"` diagnostic text.
pub fn create_failure_text(stage: &str, result: vk::Result) -> String {
    format!("{stage}: {result:?}")
}

/// Sample count; `to_vk` maps to Vulkan `SampleCountFlags`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextureSampleCount {
    One,
    Two,
    Four,
    Eight,
    Sixteen,
    ThirtyTwo,
    SixtyFour,
}
impl TextureSampleCount {
    pub fn to_vk(self) -> vk::SampleCountFlags {
        match self {
            Self::One => vk::SampleCountFlags::TYPE_1,
            Self::Two => vk::SampleCountFlags::TYPE_2,
            Self::Four => vk::SampleCountFlags::TYPE_4,
            Self::Eight => vk::SampleCountFlags::TYPE_8,
            Self::Sixteen => vk::SampleCountFlags::TYPE_16,
            Self::ThirtyTwo => vk::SampleCountFlags::TYPE_32,
            Self::SixtyFour => vk::SampleCountFlags::TYPE_64,
        }
    }
}

/// Splits linear data into consecutive slices of `step` bytes.
pub struct LinearInitialDataIter<'a> {
    data: &'a [u8],
    step: usize,
}
impl<'a> LinearInitialDataIter<'a> {
    pub fn new(data: &'a [u8], step: usize) -> Self {
        Self {
            data,
            step: step.max(1),
        }
    }
}
impl<'a> Iterator for LinearInitialDataIter<'a> {
    type Item = &'a [u8];
    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            return None;
        }
        let take = self.step.min(self.data.len());
        let (head, rest) = self.data.split_at(take);
        self.data = rest;
        Some(head)
    }
}

/// `ImageCreateInfo` for `desc`.
pub fn get_image_create_info(desc: &TextureDesc) -> Result<vk::ImageCreateInfo<'_>> {
    get_image_create_info_with_data(desc, false)
}

/// Build an optimally tiled device-local image. Initial pixels are staged after
/// creation, so both initialized and empty images start in UNDEFINED. Vulkan
/// only permits PREINITIALIZED with LINEAR tiling; it is never used by this path.
/// Cube kinds add CUBE_COMPATIBLE so subsequent cube views remain valid.
pub fn get_image_create_info_with_data(
    desc: &TextureDesc,
    _has_data: bool,
) -> Result<vk::ImageCreateInfo<'_>> {
    let array_layers = desc.array_layers()?;
    let mut flags = desc.flags;
    if desc.kind.is_cube() {
        flags |= vk::ImageCreateFlags::CUBE_COMPATIBLE;
    }
    Ok(vk::ImageCreateInfo::default()
        .flags(flags)
        .image_type(desc.kind.image_type())
        .format(desc.format)
        .extent(desc.kind.image_extent(desc))
        .mip_levels(u32::from(desc.mip_levels))
        .array_layers(array_layers)
        .samples(desc.samples())
        .tiling(vk::ImageTiling::OPTIMAL)
        .usage(desc.usage)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .initial_layout(vk::ImageLayout::UNDEFINED))
}

/// Image kind.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TextureKind {
    D1 = 0,
    D1Array = 1,
    D2 = 2,
    D2Array = 3,
    D3 = 4,
    Cube = 5,
    CubeArray = 6,
}
impl TextureKind {
    pub fn image_type(self) -> vk::ImageType {
        match self {
            Self::D1 | Self::D1Array => vk::ImageType::TYPE_1D,
            Self::D3 => vk::ImageType::TYPE_3D,
            _ => vk::ImageType::TYPE_2D,
        }
    }
    /// Vulkan image extent of this kind.
    pub fn image_extent(self, desc: &TextureDesc) -> vk::Extent3D {
        match self {
            Self::D1 | Self::D1Array => vk::Extent3D {
                width: desc.width,
                height: 1,
                depth: 1,
            },
            Self::D3 => desc.extent(),
            _ => vk::Extent3D {
                width: desc.width,
                height: desc.height,
                depth: 1,
            },
        }
    }
    pub fn is_cube(self) -> bool {
        matches!(self, Self::Cube | Self::CubeArray)
    }
    /// Only 1D arrays, 2D arrays and cube arrays may have several elements.
    pub fn is_array(self) -> bool {
        (0x4au32 >> self as u32) & 1 != 0
    }
    pub fn from_raw(raw: u8) -> Option<Self> {
        Some(match raw {
            0 => Self::D1,
            1 => Self::D1Array,
            2 => Self::D2,
            3 => Self::D2Array,
            4 => Self::D3,
            5 => Self::Cube,
            6 => Self::CubeArray,
            _ => return None,
        })
    }
}

/// Texture description (32 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextureDesc {
    pub usage: vk::ImageUsageFlags,
    pub flags: vk::ImageCreateFlags,
    pub format: vk::Format,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub array_elements: u32,
    pub mip_levels: u8,
    pub sample_count: u8,
    pub kind: TextureKind,
}
const _: () = assert!(std::mem::size_of::<TextureDesc>() == 32);
impl TextureDesc {
    /// Validate caller-controlled fields before any Vulkan call or size arithmetic.
    pub fn validate(&self) -> Result<()> {
        if self.usage.is_empty()
            || self.width == 0
            || self.height == 0
            || self.depth == 0
            || self.array_elements == 0
            || self.mip_levels == 0
        {
            return Err(Error::invalid_usage(
                "纹理 usage、范围、层数和 mip 数必须非零",
            ));
        }
        self.array_layers()?;
        if !matches!(self.sample_count, 1 | 2 | 4 | 8 | 16 | 32 | 64) {
            return Err(Error::invalid_usage("纹理采样数无效"));
        }
        Ok(())
    }
    fn base(width: u32, height: u32, depth: u32, format: vk::Format, kind: TextureKind) -> Self {
        Self {
            // Usage and flags start empty; builders such as `.sampled()` add them.
            usage: vk::ImageUsageFlags::empty(),
            flags: vk::ImageCreateFlags::empty(),
            format,
            width,
            height,
            depth,
            array_elements: 1,
            mip_levels: 1,
            sample_count: 1,
            kind,
        }
    }
    /// 2D texture with one layer.
    pub fn new_2d(width: u32, height: u32, format: vk::Format) -> Self {
        Self::base(width, height, 1, format, TextureKind::D2)
    }
    /// 2D array texture.
    pub fn new_2d_array(width: u32, height: u32, layers: u32, format: vk::Format) -> Self {
        Self {
            array_elements: layers,
            ..Self::base(width, height, 1, format, TextureKind::D2Array)
        }
    }
    /// 3D texture.
    pub fn new_3d(width: u32, height: u32, depth: u32, format: vk::Format) -> Self {
        Self::base(width, height, depth, format, TextureKind::D3)
    }
    /// 立方体贴图：6 层固定。
    pub fn new_cube(size: u32, format: vk::Format) -> Self {
        Self::base(size, size, 1, format, TextureKind::Cube)
    }
    /// Texture of `kind` with `extent` and `format`.
    pub fn new(extent: vk::Extent3D, kind: TextureKind, format: vk::Format) -> Self {
        Self::base(extent.width, extent.height, extent.depth, format, kind)
    }
    pub fn extent(&self) -> vk::Extent3D {
        vk::Extent3D {
            width: self.width,
            height: self.height,
            depth: self.depth,
        }
    }
    pub fn image_type(&self) -> vk::ImageType {
        self.kind.image_type()
    }
    pub fn samples(&self) -> vk::SampleCountFlags {
        vk::SampleCountFlags::from_raw(u32::from(self.sample_count))
    }
    /// Set the raw sample count (1/2/4/…).
    pub fn sample_count(mut self, count: u8) -> Self {
        self.sample_count = count;
        self
    }
    /// 实际的 Vulkan 层数：cube 为 6，cube 数组为 `n * 6`。
    pub fn array_layers(&self) -> Result<u32> {
        if self.array_elements == 0 || (self.array_elements > 1 && !self.kind.is_array()) {
            return Err(Error::invalid_usage("纹理数组元素数量与种类不匹配"));
        }
        match self.kind {
            TextureKind::Cube => Ok(6),
            TextureKind::CubeArray => self
                .array_elements
                .checked_mul(6)
                .ok_or_else(|| Error::invalid_usage("立方体数组层数溢出")),
            _ => Ok(self.array_elements),
        }
    }
    /// Set the element count; only array kinds allow `n > 1`.
    pub fn array_elements(mut self, count: u32) -> Result<Self> {
        if count == 0 || (count > 1 && !self.kind.is_array()) {
            return Err(Error::invalid_usage("纹理数组元素数量与种类不匹配"));
        }
        if self.kind == TextureKind::CubeArray && count.checked_mul(6).is_none() {
            return Err(Error::invalid_usage("立方体数组层数溢出"));
        }
        self.array_elements = count;
        Ok(self)
    }
    /// Set the format.
    pub fn format(mut self, format: vk::Format) -> Self {
        self.format = format;
        self
    }
    /// 整除 extent 的三分量（向上取整）。
    pub fn div_up_extent(&self, divisor: u32) -> Result<vk::Extent3D> {
        if divisor == 0 {
            return Err(Error::invalid_usage("纹理尺寸除数必须非零"));
        }
        let f = |v: u32| v.div_ceil(divisor);
        Ok(vk::Extent3D {
            width: f(self.width),
            height: f(self.height),
            depth: f(self.depth),
        })
    }
    pub fn div_extent(&self, divisor: u32) -> Result<vk::Extent3D> {
        if divisor == 0 {
            return Err(Error::invalid_usage("纹理尺寸除数必须非零"));
        }
        let f = |v: u32| v / divisor;
        Ok(vk::Extent3D {
            width: f(self.width),
            height: f(self.height),
            depth: f(self.depth),
        })
    }
    /// Each dimension becomes `ceil(x / d)`, at least 1;
    /// 任一除数为 0 则返回 InvalidUsage。
    pub fn div_up_extent_by(mut self, divisors: [u32; 3]) -> Result<Self> {
        if divisors.contains(&0) {
            return Err(Error::invalid_usage("纹理尺寸除数必须非零"));
        }
        let f = |x: u32, d: u32| (x / d + u32::from(!x.is_multiple_of(d))).max(1);
        self.width = f(self.width, divisors[0]);
        self.height = f(self.height, divisors[1]);
        self.depth = f(self.depth, divisors[2]);
        Ok(self)
    }
    /// Each dimension becomes `floor(x / d)`, at least 1;
    /// 任一除数为 0 则返回 InvalidUsage。
    pub fn div_extent_by(mut self, divisors: [u32; 3]) -> Result<Self> {
        if divisors.contains(&0) {
            return Err(Error::invalid_usage("纹理尺寸除数必须非零"));
        }
        self.width = (self.width / divisors[0]).max(1);
        self.height = (self.height / divisors[1]).max(1);
        self.depth = (self.depth / divisors[2]).max(1);
        Ok(self)
    }
    /// 全 mip 链长度；`all_mip_levels` 的只读投影。
    pub fn full_mip_chain_length(&self) -> u32 {
        32 - self.width.max(self.height).max(self.depth).leading_zeros()
    }
    /// Color format with `SAMPLED` usage.
    pub fn is_sampled_color(&self) -> bool {
        self.usage.contains(vk::ImageUsageFlags::SAMPLED)
            && vk_format_valid_aspect_flags(self.format) == vk::ImageAspectFlags::COLOR
    }
    /// `usage |= SAMPLED`, `flags |= MUTABLE_FORMAT | BLOCK_TEXEL_VIEW_COMPATIBLE`.
    pub fn sampled_with_different_format(mut self) -> Self {
        self.usage |= vk::ImageUsageFlags::SAMPLED;
        self.flags |= vk::ImageCreateFlags::from_raw(0x108);
        self
    }
    pub fn usage(mut self, usage: vk::ImageUsageFlags) -> Self {
        self.usage = usage;
        self
    }
    pub fn sampled(mut self) -> Self {
        self.usage |= vk::ImageUsageFlags::SAMPLED;
        self
    }
    pub fn storage(mut self) -> Self {
        self.usage |= vk::ImageUsageFlags::STORAGE;
        self
    }
    pub fn transfer_dst(mut self) -> Self {
        self.usage |= vk::ImageUsageFlags::TRANSFER_DST;
        self
    }
    pub fn color_attachment(mut self) -> Self {
        self.usage |= vk::ImageUsageFlags::COLOR_ATTACHMENT;
        self
    }
    pub fn depth_stencil_attachment(mut self) -> Self {
        self.usage |= vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT;
        self
    }
    /// Full mip chain: `max(bitlen(w), bitlen(h), bitlen(depth))` levels.
    pub fn all_mip_levels(mut self) -> Self {
        self.mip_levels = self.full_mip_chain_length() as u8;
        self
    }
    /// Set the mip level count.
    pub fn mip_levels(mut self, levels: u8) -> Self {
        self.mip_levels = levels;
        self
    }
    pub fn extent_2d(&self) -> vk::Extent2D {
        vk::Extent2D {
            width: self.width,
            height: self.height,
        }
    }
    /// Halve width, height and depth (`ceil(x / 2)`, at least 1).
    pub fn half_res(mut self) -> Self {
        let f = |x: u32| (x - (x >> 1)).max(1);
        self.width = f(self.width);
        self.height = f(self.height);
        self.depth = f(self.depth);
        self
    }
}
#[derive(Clone, Copy, Debug)]
pub struct TextureViewDesc {
    pub view_type: vk::ImageViewType,
    pub format: vk::Format,
    pub aspect: vk::ImageAspectFlags,
    pub base_mip_level: u32,
    pub level_count: u32,
    pub base_layer: u32,
    pub layer_count: u32,
    pub usage: vk::ImageUsageFlags,
}
impl TextureViewDesc {
    pub fn new(texture: &TextureDesc) -> Result<Self> {
        let array_layers = texture.array_layers()?;
        Ok(Self {
            view_type: match texture.kind {
                TextureKind::D1 => vk::ImageViewType::TYPE_1D,
                TextureKind::D1Array => vk::ImageViewType::TYPE_1D_ARRAY,
                TextureKind::D2 => vk::ImageViewType::TYPE_2D,
                TextureKind::D2Array => vk::ImageViewType::TYPE_2D_ARRAY,
                TextureKind::D3 => vk::ImageViewType::TYPE_3D,
                TextureKind::Cube => vk::ImageViewType::CUBE,
                TextureKind::CubeArray => vk::ImageViewType::CUBE_ARRAY,
            },
            format: texture.format,
            aspect: vk_format_valid_aspect_flags(texture.format),
            base_mip_level: 0,
            level_count: u32::from(texture.mip_levels),
            base_layer: 0,
            layer_count: array_layers,
            usage: texture.usage,
        })
    }
    /// Validate a nonempty subresource range without truncation or overflow.
    pub fn validate(&self, texture: &TextureDesc) -> Result<()> {
        let layers = texture.array_layers()?;
        if self.level_count == 0
            || self.layer_count == 0
            || self
                .base_mip_level
                .checked_add(self.level_count)
                .is_none_or(|end| end > u32::from(texture.mip_levels))
            || self
                .base_layer
                .checked_add(self.layer_count)
                .is_none_or(|end| end > layers)
        {
            return Err(Error::invalid_usage("图片视图子资源范围无效"));
        }
        Ok(())
    }
    /// 是否逐 mip 单独绑定：视图只覆盖单个 mip 层。
    pub fn bind_individual_mips(&self) -> bool {
        self.level_count == 1
    }
    pub fn usage(&self) -> vk::ImageUsageFlags {
        self.usage
    }
}
pub struct Texture {
    pub(crate) sharing: crate::resource::ResourceSharing,
    allocation_use: Option<Arc<crate::resource::SyncCell>>,
    pub(crate) last_use: Arc<crate::resource::SyncCell>,
    pub device: Arc<Device>,
    pub handle: vk::Image,
    pub desc: TextureDesc,
    storage: TextureStorage,
    _memory: super::allocator::CpuMemoryReservation,
}
enum TextureStorage {
    Allocated(std::mem::ManuallyDrop<MemoryBlock>),
    // The swapchain, not this texture wrapper, destroys the VkImage. A strong
    // owner also protects the image while graph imports and views retain it.
    Swapchain(
        #[allow(dead_code, reason = "keeps the swapchain image alive")]
        Arc<super::swapchain::Swapchain>,
    ),
}
impl Texture {
    pub(crate) fn ownership_range(desc: &TextureDesc) -> Result<crate::resource::OwnershipRange> {
        Ok(crate::resource::OwnershipRange::Image {
            aspects: super::format::vk_format_valid_aspect_flags(desc.format).as_raw(),
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
        sharing: crate::resource::ResourceSharing,
        range: crate::resource::OwnershipRange,
        storage: crate::resource::tracked::ResourceStorage,
    ) -> Self {
        let allocation_use = allocation.shared_use();
        let last_use = (if sharing.is_concurrent() {
            crate::resource::SyncCell::for_device(device.generation())
                .with_concurrent_families(match sharing {
                    crate::resource::ResourceSharing::Concurrent(families) => families,
                    _ => unreachable!("concurrent branch"),
                })
                .with_ownership_range(range)
        } else {
            crate::resource::SyncCell::for_device(device.generation()).with_ownership_range(range)
        })
        .into_reserved_arc(storage.synchronization);
        Self {
            _memory: storage.object,
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
        owner: Arc<super::swapchain::Swapchain>,
        handle: vk::Image,
        desc: TextureDesc,
    ) -> Result<Self> {
        if handle == vk::Image::null() {
            return Err(Error::invalid_usage("cannot borrow a null swapchain image"));
        }
        let device = owner.device.clone();
        let range = Self::ownership_range(&desc)?;
        let synchronization =
            crate::resource::SyncCell::reserve_arc_storage(&device.allocator_stats)?;
        let memory =
            super::allocator::reserve_shared_cpu_metadata::<Self>(&device.allocator_stats, 0)?;
        Ok(Self {
            sharing: crate::resource::ResourceSharing::Exclusive,
            allocation_use: None,
            last_use: crate::resource::SyncCell::for_device(device.generation())
                .with_ownership_range(range)
                .into_reserved_arc(synchronization),
            device,
            handle,
            desc,
            storage: TextureStorage::Swapchain(owner),
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
            crate::resource::ResourceSharing::Exclusive,
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
            crate::resource::ResourceSharing::Concurrent(crate::resource::QueueFamilies::new(
                families,
            )?),
        )
    }
    pub fn sharing(&self) -> crate::resource::ResourceSharing {
        self.sharing
    }
    fn new_with_sharing(
        device: Arc<Device>,
        desc: &TextureDesc,
        has_data: bool,
        sharing: crate::resource::ResourceSharing,
    ) -> Result<Self> {
        if device.is_device_lost() {
            return Err(Error::DeviceLost);
        }
        sharing.validate_device(&device)?;
        desc.validate()?;
        let range = Self::ownership_range(desc)?;
        let sync_storage = crate::resource::SyncCell::reserve_arc_storage(&device.allocator_stats)?;
        let memory = device.allocator_stats.reserve_cpu_bytes(
            super::allocator::shared_cpu_bytes::<Self>()?,
            crate::device::BudgetCategory::Live,
        )?;
        let info = get_image_create_info_with_data(desc, has_data)?
            .sharing_mode(if sharing.is_concurrent() {
                vk::SharingMode::CONCURRENT
            } else {
                vk::SharingMode::EXCLUSIVE
            })
            .queue_family_indices(sharing.families());
        let handle = super::error::observe_vulkan(&device, unsafe {
            device.device.create_image(&info, None)
        })?;
        let mut dedicated = vk::MemoryDedicatedRequirements::default();
        let mut requirements2 = vk::MemoryRequirements2::default().push_next(&mut dedicated);
        unsafe {
            device.device.get_image_memory_requirements2(
                &vk::ImageMemoryRequirementsInfo2::default().image(handle),
                &mut requirements2,
            );
        }
        let requirements = requirements2.memory_requirements;
        let allocated = if dedicated.requires_dedicated_allocation != 0 {
            MemoryBlock::allocate_for_resource(
                device.clone(),
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
                    device.device.destroy_image(handle, None);
                }
                return Err(error);
            }
        };
        if let Err(error) = unsafe {
            device
                .device
                .bind_image_memory(handle, allocation.handle, allocation.offset)
        } {
            unsafe {
                device.device.destroy_image(handle, None);
            }
            return super::error::observe_vulkan(&device, Err(error));
        }
        Ok(Self {
            _memory: memory,
            sharing,
            allocation_use: None,
            last_use: (if sharing.is_concurrent() {
                crate::resource::SyncCell::for_allocation(
                    device.generation(),
                    device.allocator_stats.clone(),
                    allocation.accounting_id(),
                )
                .with_concurrent_families(match sharing {
                    crate::resource::ResourceSharing::Concurrent(families) => families,
                    _ => unreachable!("concurrent branch"),
                })
                .with_ownership_range(range)
            } else {
                crate::resource::SyncCell::for_allocation(
                    device.generation(),
                    device.allocator_stats.clone(),
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
        let memory = self.device.allocator_stats.reserve_cpu_bytes(
            super::allocator::shared_cpu_bytes::<TextureView>()?,
            crate::device::BudgetCategory::Live,
        )?;
        let range = vk::ImageSubresourceRange::default()
            .aspect_mask(desc.aspect)
            .base_mip_level(desc.base_mip_level)
            .level_count(desc.level_count)
            .base_array_layer(desc.base_layer)
            .layer_count(desc.layer_count);
        let info = vk::ImageViewCreateInfo::default()
            .image(self.handle)
            .view_type(desc.view_type)
            .format(desc.format)
            .subresource_range(range);
        let sync_storage =
            crate::resource::SyncCell::reserve_arc_storage(&self.device.allocator_stats)?;
        let handle = super::error::observe_vulkan(&self.device, unsafe {
            self.device.device.create_image_view(&info, None)
        })?;
        Ok(TextureView {
            _memory: memory,
            last_use: crate::resource::SyncCell::for_device(self.device.generation())
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
impl Drop for Texture {
    fn drop(&mut self) {
        if let TextureStorage::Allocated(allocation) = &mut self.storage {
            // SAFETY: this owned allocation is taken exactly once.
            let memory = unsafe { std::mem::ManuallyDrop::take(allocation) }.into_deferred();
            self.device.retire(
                self.last_use.snapshot(),
                super::graveyard::Doomed::Image(self.handle, memory),
            );
        }
    }
}
pub struct TextureView {
    pub(crate) last_use: Arc<crate::resource::SyncCell>,
    pub texture: Arc<Texture>,
    pub handle: vk::ImageView,
    pub desc: TextureViewDesc,
    _memory: super::allocator::CpuMemoryReservation,
}
impl Drop for TextureView {
    fn drop(&mut self) {
        self.texture.device.retire(
            self.last_use.snapshot(),
            super::graveyard::Doomed::ImageView(self.handle),
        );
    }
}
pub struct MipChainIter {
    extent: vk::Extent3D,
    remaining: u32,
}
impl MipChainIter {
    pub fn new(extent: vk::Extent3D, levels: u32) -> Self {
        Self {
            extent,
            remaining: levels,
        }
    }
}
impl Iterator for MipChainIter {
    type Item = vk::Extent3D;
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let current = self.extent;
        self.remaining -= 1;
        self.extent = vk::Extent3D {
            width: (current.width / 2).max(1),
            height: (current.height / 2).max(1),
            depth: (current.depth / 2).max(1),
        };
        Some(current)
    }
}

impl crate::resource::GpuUse for Texture {
    fn visit_uses(&self, visitor: &mut crate::resource::UseVisitor<'_>) -> crate::Result<()> {
        visitor(&self.last_use)?;
        if let Some(cell) = &self.allocation_use {
            visitor(cell)?;
        }

        Ok(())
    }
}

impl crate::resource::GpuUse for TextureView {
    fn visit_uses(&self, visitor: &mut crate::resource::UseVisitor<'_>) -> crate::Result<()> {
        visitor(&self.last_use)?;
        self.texture.visit_uses(visitor)?;

        Ok(())
    }
}
