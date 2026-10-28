//! Texture and texture-view descriptions.
use ash::vk;
use crate::Error;
use crate::Result;
use crate::format::vk_format_valid_aspect_flags;
use super::info::image_create_flags;
use super::kind::TextureKind;

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
        if u32::from(self.mip_levels) > self.full_mip_chain_length()
            || matches!(self.kind, TextureKind::D1 | TextureKind::D1Array)
                && (self.height != 1 || self.depth != 1)
            || !matches!(
                self.kind,
                TextureKind::D1 | TextureKind::D1Array | TextureKind::D3
            ) && self.depth != 1
        {
            return Err(Error::invalid_usage(
                "texture extent or mip chain differs from its kind",
            ));
        }
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
    /// `usage |= SAMPLED`, `flags |= MUTABLE_FORMAT | EXTENDED_USAGE` (native 0x108).
    pub fn sampled_with_different_format(mut self) -> Self {
        self.usage |= vk::ImageUsageFlags::SAMPLED;
        self.flags |= vk::ImageCreateFlags::MUTABLE_FORMAT | vk::ImageCreateFlags::EXTENDED_USAGE;
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
        if self.usage.is_empty() || !texture.usage.contains(self.usage) {
            return Err(Error::invalid_usage("image view usage must be a nonempty subset of its image usage"));
        }
        let layers = texture.array_layers()?;
        let valid_kind = match texture.kind {
            TextureKind::D1 | TextureKind::D1Array => matches!(
                self.view_type,
                vk::ImageViewType::TYPE_1D | vk::ImageViewType::TYPE_1D_ARRAY
            ),
            TextureKind::D3 => {
                self.view_type == vk::ImageViewType::TYPE_3D && self.layer_count == 1
            }
            _ => {
                matches!(
                    self.view_type,
                    vk::ImageViewType::TYPE_2D | vk::ImageViewType::TYPE_2D_ARRAY
                ) || image_create_flags(texture).contains(vk::ImageCreateFlags::CUBE_COMPATIBLE)
                    && matches!(
                        self.view_type,
                        vk::ImageViewType::CUBE | vk::ImageViewType::CUBE_ARRAY
                    )
            }
        };
        let aspects = crate::format::vk_format_all_aspects(texture.format);
        if !valid_kind
            || self.aspect.is_empty()
            || !aspects.contains(self.aspect)
            || self.format != texture.format
                && !texture.flags.contains(vk::ImageCreateFlags::MUTABLE_FORMAT)
            || matches!(
                self.view_type,
                vk::ImageViewType::TYPE_1D | vk::ImageViewType::TYPE_2D
            ) && self.layer_count != 1
            || self.view_type == vk::ImageViewType::CUBE && self.layer_count != 6
            || self.view_type == vk::ImageViewType::CUBE_ARRAY && self.layer_count % 6 != 0
        {
            return Err(Error::invalid_usage(
                "image view is incompatible with its texture",
            ));
        }
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
