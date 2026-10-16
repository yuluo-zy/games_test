//! Texture/ImageView 完整资源路径候选；描述符字段不声称恢复原 ABI。
use super::{
    allocator::MemoryBlock,
    device::VulkanDevice,
    error::{Error, Result},
    format::vk_format_valid_aspect_flags,
};
use ash::vk;
use std::sync::Arc;

/// 原 `image_aspect_mask_from_format`（0x140bd6630）：`format - 0x7c`（无符号）`> 6` 返回 COLOR(1)，
/// 否则查 7 项表 `0x142b0ffb8 = {2,2,2,4,6,6,6}`（D16 / X8_D24 / D32 / S8 / D16S8 / D24S8 / D32S8）。
pub fn image_aspect_mask_from_format(format: vk::Format) -> vk::ImageAspectFlags {
    const TABLE: [u32; 7] = [2, 2, 2, 4, 6, 6, 6];
    let index = (format.as_raw() as u32).wrapping_sub(0x7c);
    vk::ImageAspectFlags::from_raw(TABLE.get(index as usize).copied().unwrap_or(1))
}

/// 1D 尺寸的全 mip 数量：32 - leading_zeros(size)。
pub fn mip_count_1d(size: u32) -> u32 {
    32 - size.leading_zeros()
}

/// `create_texture`（0x140c07950）里各 Vulkan 调用失败时的 `.expect(msg)` 文案（原版 panic，
/// 位置 crates/rhapsody/src/raw/texture.rs 0x244 / 0x24d / 0x26a 行附近）。
/// 这里的创建路径返回 `Err(Error::Vulkan(..))`，调用方需要原文案时用 [`create_failure_text`] 拼出。
pub mod create_failure {
    /// 0x142b18af8（12B）：`vkCreateImage` 失败。
    pub const CREATE_IMAGE: &str = "create_image";
    /// 0x142b18b70（17B）：`vkBindImageMemory` 失败。
    pub const BIND_IMAGE_MEMORY: &str = "bind_image_memory";
    /// 0x142b18b38（27B）：`vkSetDebugUtilsObjectNameEXT` 失败（缓冲创建同名文案 0x142b15850）。
    pub const SET_DEBUG_UTILS_OBJECT_NAME: &str = "set_debug_utils_object_name";
}

/// `Result::expect(msg)` 的 panic 文本形态：`"{msg}: {Debug(err)}"`（0x1428d9760 的 `expect_failed` 调用形）。
pub fn create_failure_text(stage: &str, result: vk::Result) -> String {
    format!("{stage}: {result:?}")
}

/// 纹理采样数候选枚举；`to_vk` 映射到 Vulkan `SampleCountFlags`。
/// 枚举名来自 PDB，具体排序与语义未逐证明，保留显式映射。
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

/// 线性初始数据的逐层迭代器；每次 `next` 返回一层的数据切片。
/// 原 `LinearInitialDataIter` 从连续内存按 mip 偏移切分；候选按分配步进。
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

/// 构建 `ImageCreateInfo`（原 `get_image_create_info` 0x140bd5ed0，`has_data = false`）。
pub fn get_image_create_info(desc: &TextureDesc) -> vk::ImageCreateInfo<'_> {
    get_image_create_info_with_data(desc, false)
}

/// 原 `get_image_create_info(info, desc, has_data)`（0x140bd5ed0）。机器码逐字段：
/// `sType=14`、`flags = desc+4`（原样，不额外加 CUBE_COMPATIBLE）、`imageType`（1D 两种=0，3D=2，其余=1）、
/// `format = desc+8`、extent 按 kind：1D 类 `(w,1,1)`，2D/2DArray/Cube/CubeArray `(w,h,1)`，3D `(w,h,d)`；
/// `mipLevels = desc+0x1c`、`arrayLayers`（Cube=6、CubeArray=n*6）、`samples = desc+0x1d`（原始数值）、
/// `tiling=OPTIMAL(0)`、`usage = desc+0`、`sharingMode=EXCLUSIVE`、
/// **`initialLayout = has_data << 3`（= PREINITIALIZED(8)，无初始数据为 UNDEFINED）**。
///
/// 有意偏差：cube 种类仍自动补 `CUBE_COMPATIBLE`（原版不补，flags 由调用方给；Vulkan 创建 cube 视图
/// 需要它，原版调用方如何提供未证实）。
pub fn get_image_create_info_with_data(
    desc: &TextureDesc,
    has_data: bool,
) -> vk::ImageCreateInfo<'_> {
    let mut flags = desc.flags;
    if desc.kind.is_cube() {
        flags |= vk::ImageCreateFlags::CUBE_COMPATIBLE;
    }
    vk::ImageCreateInfo::default()
        .flags(flags)
        .image_type(desc.kind.image_type())
        .format(desc.format)
        .extent(desc.kind.image_extent(desc))
        .mip_levels(u32::from(desc.mip_levels))
        .array_layers(desc.array_layers())
        .samples(desc.samples())
        .tiling(vk::ImageTiling::OPTIMAL)
        .usage(desc.usage)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .initial_layout(if has_data {
            vk::ImageLayout::PREINITIALIZED
        } else {
            vk::ImageLayout::UNDEFINED
        })
}

/// 原图像种类枚举（`TextureDesc+0x1e`，0..=6）。
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
    /// `get_image_create_info` 的 extent 选择（0x140bd5ed0 跳表各 case）。
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
    /// 原 `array_elements` 的合法性掩码 `0x4a`：只有 1D 数组、2D 数组、cube 数组可以有多层。
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

/// 原 32 字节 `TextureDesc`（n2 REPORT）：`usage@0 flags@4 format@8 width@0xc height@0x10 depth@0x14
/// array_elements@0x18 mip_levels@0x1c sample_count@0x1d kind@0x1e`。
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
    fn base(width: u32, height: u32, depth: u32, format: vk::Format, kind: TextureKind) -> Self {
        Self {
            // 原版构造器把 usage/flags 整个 qword 清零（0x140bd5270/52a0）；由 `.sampled()` 等 builder 追加。
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
    /// 原 `new_2d`（0x140bd52a0）：深度 1、层 1。
    pub fn new_2d(width: u32, height: u32, format: vk::Format) -> Self {
        Self::base(width, height, 1, format, TextureKind::D2)
    }
    /// 原 `new_2d_array`（0x140bd52e0）。
    pub fn new_2d_array(width: u32, height: u32, layers: u32, format: vk::Format) -> Self {
        Self {
            array_elements: layers,
            ..Self::base(width, height, 1, format, TextureKind::D2Array)
        }
    }
    /// 原 `new_3d`（0x140bd5310）。
    pub fn new_3d(width: u32, height: u32, depth: u32, format: vk::Format) -> Self {
        Self::base(width, height, depth, format, TextureKind::D3)
    }
    /// 立方体贴图：6 层固定。
    pub fn new_cube(size: u32, format: vk::Format) -> Self {
        Self::base(size, size, 1, format, TextureKind::Cube)
    }
    /// 原 `TextureDesc::new(extent, kind, format)`（0x140bd5270：`r8b → +0x1e`、`r9d → +8`）。
    /// 压缩纹理文件尾的 `−3` 字节是 kind、`−4` 字节是 format。
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
    /// 原 `sample_count(self, n)`（0x140bd55a0）：`n → +0x1d`（原始数值，1/2/4/…）。
    pub fn sample_count(mut self, count: u8) -> Self {
        self.sample_count = count;
        self
    }
    /// 实际的 Vulkan 层数：cube 为 6，cube 数组为 `n * 6`。
    pub fn array_layers(&self) -> u32 {
        match self.kind {
            TextureKind::Cube => 6,
            TextureKind::CubeArray => self.array_elements * 6,
            _ => self.array_elements,
        }
    }
    /// 原 `array_elements(self, n)` builder：只有数组种类允许 `n > 1`。
    pub fn array_elements(mut self, count: u32) -> Self {
        assert!(
            count <= 1 || self.kind.is_array(),
            "array_elements > 1 requires an array texture kind"
        );
        self.array_elements = count;
        self
    }
    /// 原 `format(f)` builder（0x140bd5430）。
    pub fn format(mut self, format: vk::Format) -> Self {
        self.format = format;
        self
    }
    /// 整除 extent 的三分量（向上取整）；除数 0 保留原值（防御）。
    pub fn div_up_extent(&self, divisor: u32) -> vk::Extent3D {
        let f = |v: u32| if divisor == 0 { v } else { v.div_ceil(divisor) };
        vk::Extent3D {
            width: f(self.width),
            height: f(self.height),
            depth: f(self.depth),
        }
    }
    pub fn div_extent(&self, divisor: u32) -> vk::Extent3D {
        let f = |v: u32| v.checked_div(divisor).unwrap_or(v);
        vk::Extent3D {
            width: f(self.width),
            height: f(self.height),
            depth: f(self.depth),
        }
    }
    /// 原 `div_up_extent(self, divisors)`（0x140bd55e0）：三个维度各 `ceil(x / d)`，结果至少为 1，
    /// 任一除数为 0 则 panic（0x142b0fab0 `1428d99e0`）。返回整份描述（原函数整拷 0x20B）。
    pub fn div_up_extent_by(mut self, divisors: [u32; 3]) -> Self {
        assert!(
            divisors.iter().all(|&d| d != 0),
            "attempt to divide by zero"
        );
        let f = |x: u32, d: u32| (x / d + u32::from(!x.is_multiple_of(d))).max(1);
        self.width = f(self.width, divisors[0]);
        self.height = f(self.height, divisors[1]);
        self.depth = f(self.depth, divisors[2]);
        self
    }
    /// 原 `div_extent(self, divisors)`（0x140bd56a0）：三个维度各 `floor(x / d)`，结果至少为 1，
    /// 任一除数为 0 则 panic。
    pub fn div_extent_by(mut self, divisors: [u32; 3]) -> Self {
        assert!(
            divisors.iter().all(|&d| d != 0),
            "attempt to divide by zero"
        );
        self.width = (self.width / divisors[0]).max(1);
        self.height = (self.height / divisors[1]).max(1);
        self.depth = (self.depth / divisors[2]).max(1);
        self
    }
    /// 全 mip 链长度；`all_mip_levels` 的只读投影。
    pub fn full_mip_chain_length(&self) -> u32 {
        32 - self.width.max(self.height).max(self.depth).leading_zeros()
    }
    /// 旧候选谓词（非原版）：纯色格式且用途含 `SAMPLED`。
    pub fn is_sampled_color(&self) -> bool {
        self.usage.contains(vk::ImageUsageFlags::SAMPLED)
            && vk_format_valid_aspect_flags(self.format) == vk::ImageAspectFlags::COLOR
    }
    /// 原 `sampled_with_different_format`（0x140bd5410，`or qword [rdx], 0x108000004`）：
    /// `usage |= SAMPLED`，`flags |= 0x108`（`MUTABLE_FORMAT(0x8) | BLOCK_TEXEL_VIEW_COMPATIBLE(0x100)`）。
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
    /// 原 `all_mip_levels`（0x140bd54a0）：`max(bitlen(w), bitlen(h), bitlen(depth))` 截成 u8
    /// （三个 `bsr` 取大；旧文档写成 `max(w,h)` 不准确，机器码含 depth）。
    pub fn all_mip_levels(mut self) -> Self {
        self.mip_levels = self.full_mip_chain_length() as u8;
        self
    }
    /// 原 `mip_levels(n)` builder。
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
    /// 原 `half_res`（0x140bd5730）：宽/高/深各 `x - (x >> 1)`（= `ceil(x/2)`），结果为 0 时取 1。
    /// （旧实现漏了 depth；3D 纹理的 depth 同样减半。）
    pub fn half_res(mut self) -> Self {
        let f = |x: u32| (x - (x >> 1)).max(1);
        self.width = f(self.width);
        self.height = f(self.height);
        self.depth = f(self.depth);
        self
    }
}
/// 原版 `Texture` 句柄 = **0x38 字节**（不是 40B；40B 是 `Buffer`）。`create_texture`（0x140c07950）
/// 的 80B Result 里 Ok 载荷在 `out+0x08..0x40`：`{ctx@0, slot@8, offset u32@0x10, device_id u16@0x14,
/// flag u8@0x16 = 0, pad@0x17, desc 32B@0x18}`。`flag` 写 0（Buffer 写 1，DescriptorSet 6）。
/// 第二批读码：**消费方都用它做“种类”断言**——`Buffer::mapped_slice_mut`（0x140bd4cef）`cmp [h+0x16],1`、
/// `update_texture` 的校验包装（0x140bfe9c0 / 0x140bfe650）`cmp byte [h+0x16],0`，不符则
/// `assert_eq!(flag, 期望值)`（0x1428ca902，位置 managed/resources.rs）。所以它是托管句柄的
/// “资源种类”判别（纹理 0 / 缓冲 1 / 描述符集 6），不是就绪标记。第三批：该字节就是 `ResourceType`（`Debug` 0x140bd6bd0 的
/// `movzx ecx, byte [rcx]` 查长度表 0x142b10168 = {7,6,9,11,10,7,23,11}、偏移表 0x142b101a8），名字依次为
/// `Texture(0) Buffer(1) Swapchain(2) TextureView(3) RenderPass(4) Sampler(5) PersistentDescriptorSet(6) Framebuffer(7)`；
/// 与已证的 纹理 0 / 缓冲 1 / 描述符集 6 三个值逐一吻合。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextureRecord {
    pub ctx: u64,
    pub slot: u64,
    pub offset: u32,
    pub device_id: u16,
    pub flag: u8,
    pub _pad: u8,
    pub desc: TextureDesc,
}
const _: () = assert!(std::mem::size_of::<TextureRecord>() == 0x38);

/// `Option<u32>` 的原始布局 `{tag u32, value u32}`（`TextureViewDesc` 构造器写 `{1, r8d}`）。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OptionU32 {
    pub tag: u32,
    pub value: u32,
}
impl OptionU32 {
    pub const fn some(value: u32) -> Self {
        Self { tag: 1, value }
    }
    pub const fn get(self) -> Option<u32> {
        if self.tag == 1 {
            Some(self.value)
        } else {
            None
        }
    }
}

/// 原 `TextureViewDesc`（0x3c 字节；0x140bd57a0..0x140bd59f0 十个微函数逐字节核对）：
/// `view_type@0 usage@8 format@0x10 aspect_mask@0x18 level_count@0x20 layer_count@0x28`（均 `Option<u32>`）、
/// `base_mip_level@0x30 base_layer@0x34 bind_individual_mips@0x38`。
/// 下面的 `resolve` 才是“缺省值如何落到具体视图”的候选实现（机器码未覆盖该解析逻辑）。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TextureViewDescRaw {
    pub view_type: OptionU32,
    pub usage: OptionU32,
    pub format: OptionU32,
    pub aspect_mask: OptionU32,
    pub level_count: OptionU32,
    pub layer_count: OptionU32,
    pub base_mip_level: u32,
    pub base_layer: u32,
    pub bind_individual_mips: u8,
}
const _: () = assert!(std::mem::size_of::<TextureViewDescRaw>() == 0x3c);
impl TextureViewDescRaw {
    /// 0x140bd57a0：六个 Option 清零，`base_mip_level/base_layer/bind_individual_mips` 为 0。
    pub const fn new() -> Self {
        Self {
            view_type: OptionU32 { tag: 0, value: 0 },
            usage: OptionU32 { tag: 0, value: 0 },
            format: OptionU32 { tag: 0, value: 0 },
            aspect_mask: OptionU32 { tag: 0, value: 0 },
            level_count: OptionU32 { tag: 0, value: 0 },
            layer_count: OptionU32 { tag: 0, value: 0 },
            base_mip_level: 0,
            base_layer: 0,
            bind_individual_mips: 0,
        }
    }
    pub fn view_type(mut self, v: u32) -> Self {
        self.view_type = OptionU32::some(v);
        self
    }
    pub fn usage(mut self, v: u32) -> Self {
        self.usage = OptionU32::some(v);
        self
    }
    pub fn format(mut self, v: u32) -> Self {
        self.format = OptionU32::some(v);
        self
    }
    pub fn aspect_mask(mut self, v: u32) -> Self {
        self.aspect_mask = OptionU32::some(v);
        self
    }
    /// 0x140bd5920：参数是 `Option<u32>`（`r8d` 标签、`r9d` 值），这里按 `Some(v)` 写。
    pub fn level_count(mut self, v: u32) -> Self {
        self.level_count = OptionU32::some(v);
        self
    }
    pub fn layer_count(mut self, v: u32) -> Self {
        self.layer_count = OptionU32::some(v);
        self
    }
    pub fn base_mip_level(mut self, v: u32) -> Self {
        self.base_mip_level = v;
        self
    }
    pub fn base_layer(mut self, v: u32) -> Self {
        self.base_layer = v;
        self
    }
    pub fn bind_individual_mips(mut self, v: bool) -> Self {
        self.bind_individual_mips = u8::from(v);
        self
    }
    /// 候选解析：未给出的项取纹理默认（view_type/aspect 由 `TextureViewDesc::new` 推出，usage 取纹理用途，
    /// level/layer 取 `base` 之后的剩余数量）。**非原版证实**。
    pub fn resolve(&self, texture: &TextureDesc) -> TextureViewDesc {
        let mut out = TextureViewDesc::new(texture);
        if let Some(v) = self.view_type.get() {
            out.view_type = vk::ImageViewType::from_raw(v as i32);
        }
        if let Some(v) = self.usage.get() {
            out.usage = vk::ImageUsageFlags::from_raw(v);
        }
        if let Some(v) = self.format.get() {
            out.format = vk::Format::from_raw(v as i32);
        }
        if let Some(v) = self.aspect_mask.get() {
            out.aspect = vk::ImageAspectFlags::from_raw(v);
        }
        out.base_mip_level = self.base_mip_level;
        out.base_layer = self.base_layer;
        out.level_count = self
            .level_count
            .get()
            .unwrap_or_else(|| u32::from(texture.mip_levels).saturating_sub(self.base_mip_level));
        out.layer_count = self
            .layer_count
            .get()
            .unwrap_or_else(|| texture.array_layers().saturating_sub(self.base_layer));
        out
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
    pub fn new(texture: &TextureDesc) -> Self {
        Self {
            view_type: if texture.image_type() == vk::ImageType::TYPE_3D {
                vk::ImageViewType::TYPE_3D
            } else if texture.array_layers() > 1 {
                vk::ImageViewType::TYPE_2D_ARRAY
            } else {
                vk::ImageViewType::TYPE_2D
            },
            format: texture.format,
            aspect: vk_format_valid_aspect_flags(texture.format),
            base_mip_level: 0,
            level_count: u32::from(texture.mip_levels),
            base_layer: 0,
            layer_count: texture.array_layers(),
            usage: texture.usage,
        }
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
    pub device: Arc<VulkanDevice>,
    pub handle: vk::Image,
    pub desc: TextureDesc,
    allocation: std::mem::ManuallyDrop<MemoryBlock>,
}
impl Texture {
    pub fn new(device: Arc<VulkanDevice>, desc: &TextureDesc) -> Result<Self> {
        Self::new_with_initial_data(device, desc, false)
    }
    /// 带初始数据时 `initialLayout = PREINITIALIZED`（`get_image_create_info` 第三参，见该函数注释）。
    pub fn new_with_initial_data(
        device: Arc<VulkanDevice>,
        desc: &TextureDesc,
        has_data: bool,
    ) -> Result<Self> {
        if desc.extent().width == 0
            || desc.extent().height == 0
            || desc.extent().depth == 0
            || desc.array_layers() == 0
            || desc.mip_levels == 0
        {
            return Err(Error::invalid_usage("纹理范围和层数必须非零"));
        }
        let info = get_image_create_info_with_data(desc, has_data);
        let handle = unsafe { device.device.create_image(&info, None) }?;
        let requirements = unsafe { device.device.get_image_memory_requirements(handle) };
        let allocation = match MemoryBlock::allocate(
            Arc::clone(&device),
            requirements,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        ) {
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
            return Err(error.into());
        }
        Ok(Self {
            device,
            handle,
            desc: *desc,
            allocation: std::mem::ManuallyDrop::new(allocation),
        })
    }
    pub fn image_view(self: &Arc<Self>, desc: &TextureViewDesc) -> Result<TextureView> {
        if desc
            .base_mip_level
            .checked_add(desc.level_count)
            .is_none_or(|n| n > u32::from(self.desc.mip_levels))
            || desc
                .base_layer
                .checked_add(desc.layer_count)
                .is_none_or(|n| n > self.desc.array_layers())
            || desc.level_count == 0
            || desc.layer_count == 0
        {
            return Err(Error::invalid_usage("图片视图子资源范围无效"));
        }
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
        let handle = unsafe { self.device.device.create_image_view(&info, None) }?;
        Ok(TextureView {
            texture: Arc::clone(self),
            handle,
            desc: *desc,
        })
    }
    /// 转成原版 0x38 字节记录（`ctx/slot/offset/device_id` 由托管槽表提供）。
    pub fn record(&self, ctx: u64, slot: u64, offset: u32, device_id: u16) -> TextureRecord {
        TextureRecord {
            ctx,
            slot,
            offset,
            device_id,
            flag: 0,
            _pad: 0,
            desc: self.desc,
        }
    }
    pub fn allocation_bytes(&self) -> u64 {
        self.allocation.size
    }
}
impl Drop for Texture {
    fn drop(&mut self) {
        // SAFETY: allocation 只在此处取出一次，之后不会再被访问。
        let memory = unsafe { std::mem::ManuallyDrop::take(&mut self.allocation) }.into_deferred();
        self.device
            .bury(super::graveyard::Doomed::Image(self.handle, memory));
    }
}
pub struct TextureView {
    pub texture: Arc<Texture>,
    pub handle: vk::ImageView,
    pub desc: TextureViewDesc,
}
impl Drop for TextureView {
    fn drop(&mut self) {
        self.texture
            .device
            .bury(super::graveyard::Doomed::ImageView(self.handle));
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
