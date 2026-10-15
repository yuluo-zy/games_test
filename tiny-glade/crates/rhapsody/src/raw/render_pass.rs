//! 创建原生 Vulkan render pass；附件和依赖由调用者明确声明。
use super::{device::VulkanDevice, error::Result};
use ash::vk;
use std::sync::Arc;

/// 精确创建键：load/store 和布局会改变执行语义，不能仅按格式查缓存。
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AttachmentKey {
    pub format: i32,
    pub samples: u32,
    pub load: i32,
    pub store: i32,
    pub stencil_load: i32,
    pub stencil_store: i32,
    pub initial_layout: i32,
    pub final_layout: i32,
    pub subpass_layout: i32,
    pub depth_stencil: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RenderPassKey {
    /// 顺序同时是附件索引；颜色在前，可选深度模板在末尾。
    pub attachments: Vec<AttachmentKey>,
}

impl RenderPassKey {
    /// 管线兼容性忽略 load/store 和首尾布局，但保留子通道中的附件顺序与类型。
    /// 当前候选只生产单个图形子通道，没有 input/resolve 附件。
    pub fn compatibility_key(&self) -> Vec<(i32, u32, bool)> {
        self.attachments
            .iter()
            .map(|a| (a.format, a.samples, a.depth_stencil))
            .collect()
    }
}
/// 附件声明构建器（原版 0x10B，机器码 0x140be6780/67a0/67c0）：
/// `{format@0, clear_input@4, store_op@8, sample_count@0xc, 3B@0xd..0xf}`。
/// `+0x08` 经 `create_render_pass`（0x1418b8c50）原值写入 `AttachmentDescription.storeOp`（0=STORE），
/// `+0x04` 原值写入 `loadOp`（0=LOAD,1=CLEAR）；`+0x0d..0x0f` 是 `{u32,u32,u32,u8}` 的**尾部填充**：`new`（0x140be6780）
/// 只写 `[+0]=format`、`[+4..+0xc)=0`（qword）、`[+0xc]=1`，**并不清零** +0x0d..0x0f（原版此处是未初始化字节）；
/// 按值传递的 builder（0x140be67a0/0x140be67c0）按字段拷贝（`movzx word [+0xd]` 等），`create_render_pass`（0x1418b8c50）
/// 与已读到的消费端都不读它（缓存键哈希是否按字节含填充未证实）。这里固定为 0 且不影响语义（原“被 new 清零”的说法作废）。原版也没有
/// “深度模板”方法，是否深度模板由格式的 aspect 决定（`image_aspect_mask_from_format`）。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RenderPassAttachmentDesc {
    pub format: vk::Format,
    pub clear_input: u32,
    pub store_op: u32,
    pub sample_count: u8,
    pub reserved_0d: [u8; 3],
}
const _: () = assert!(std::mem::size_of::<RenderPassAttachmentDesc>() == 0x10);
impl RenderPassAttachmentDesc {
    pub fn new(format: vk::Format) -> Self {
        Self {
            format,
            clear_input: 0,
            store_op: 0,
            sample_count: 1,
            reserved_0d: [0; 3],
        }
    }
    /// `+0x04 = 1`。
    pub fn clear_input(mut self) -> Self {
        self.clear_input = 1;
        self
    }
    pub fn sample_count(mut self, samples: u8) -> Self {
        self.sample_count = samples;
        self
    }
    /// 7 个深度/模板格式 D16..D32S8（`format - 0x7c <= 6`）。
    pub fn is_depth_stencil(&self) -> bool {
        (124..=130).contains(&self.format.as_raw())
    }
    /// 转成旧的平铺 `AttachmentKey`，语义与 `RenderPass::from_desc` 逐项一致：
    /// load/store 取字段原值，stencil 恒 LOAD/STORE，initial=final=子通道布局（颜色
    /// `COLOR_ATTACHMENT_OPTIMAL`；深度按格式视为 `DepthAccess::ReadWrite`）。
    pub fn to_attachment_key(&self) -> AttachmentKey {
        let depth_stencil = self.is_depth_stencil();
        let layout = if depth_stencil {
            DepthAccess::ReadWrite.layout()
        } else {
            vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL
        };
        AttachmentKey {
            format: self.format.as_raw(),
            samples: u32::from(self.sample_count),
            load: self.clear_input as i32,
            store: self.store_op as i32,
            stencil_load: vk::AttachmentLoadOp::LOAD.as_raw(),
            stencil_store: vk::AttachmentStoreOp::STORE.as_raw(),
            initial_layout: layout.as_raw(),
            final_layout: layout.as_raw(),
            subpass_layout: layout.as_raw(),
            depth_stencil,
        }
    }
}


/// 深度附件的访问模式（原 `RasterPass` `+0x12*0x10+0xc` 字节；`create_render_pass` 跳表 0x142b174cc）：
/// 0→`DEPTH_STENCIL_READ_ONLY_OPTIMAL(4)`、1→`DEPTH_STENCIL_ATTACHMENT_OPTIMAL(3)`、
/// 2→`1000117001`(`DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL)`；值 3 在 `Option` 里是 `None`（无深度）。
/// 变体名是候选（按布局取名），数值/布局由跳表机器码证实。
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DepthAccess {
    ReadOnly = 0,
    ReadWrite = 1,
    DepthWriteStencilRead = 2,
}
impl DepthAccess {
    pub fn layout(self) -> vk::ImageLayout {
        match self {
            Self::ReadOnly => vk::ImageLayout::DEPTH_STENCIL_READ_ONLY_OPTIMAL,
            Self::ReadWrite => vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            Self::DepthWriteStencilRead => vk::ImageLayout::DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL,
        }
    }
    pub fn from_tag(tag: u8) -> Option<Option<Self>> {
        match tag {
            0 => Some(Some(Self::ReadOnly)),
            1 => Some(Some(Self::ReadWrite)),
            2 => Some(Some(Self::DepthWriteStencilRead)),
            3 => Some(None),
            _ => None,
        }
    }
}

/// `ArrayVec<RenderPassAttachmentDesc, 8>`（arrayvec 0.7.4 布局：`[T;8]@0` + `len:u32@0x80`，0x84B）。
/// rhapsody 不依赖 arrayvec，这里按同形自带最小实现；哈希/相等只看有效切片（与 ArrayVec 一致）。
#[derive(Clone, Copy)]
pub struct AttachmentArray {
    items: [RenderPassAttachmentDesc; 8],
    len: u32,
}
impl AttachmentArray {
    pub const CAPACITY: usize = 8;
    pub fn new() -> Self {
        Self {
            items: [RenderPassAttachmentDesc::new(vk::Format::UNDEFINED); 8],
            len: 0,
        }
    }
    /// 满了 panic（`FUN_1428fa090(&UNK_142c77358)`，draw_stream::render 的 `lVar14 == 0x80` 分支）。
    pub fn push(&mut self, item: RenderPassAttachmentDesc) {
        assert!(self.len < 8, "ArrayVec: capacity exceeded in push");
        self.items[self.len as usize] = item;
        self.len += 1;
    }
    pub fn len(&self) -> u32 {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn as_slice(&self) -> &[RenderPassAttachmentDesc] {
        &self.items[..self.len as usize]
    }
}
impl Default for AttachmentArray {
    fn default() -> Self {
        Self::new()
    }
}
impl PartialEq for AttachmentArray {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}
impl Eq for AttachmentArray {}
impl std::hash::Hash for AttachmentArray {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state)
    }
}
impl std::fmt::Debug for AttachmentArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.as_slice()).finish()
    }
}
impl FromIterator<RenderPassAttachmentDesc> for AttachmentArray {
    fn from_iter<I: IntoIterator<Item = RenderPassAttachmentDesc>>(iter: I) -> Self {
        let mut array = Self::new();
        for item in iter {
            array.push(item);
        }
        array
    }
}

/// 原 `RenderPassDesc`（0x98B，`HashMap<RenderPassDesc, RenderPass>` 的键，`device+0x188`，桶 0xb0 =
/// 键 0x98 + 值 `RenderPass`(ManagedHandle 0x18)）：
/// `{colors: ArrayVec<_,8>@0 (0x84B), depth: Option<(RenderPassAttachmentDesc@0x84, DepthAccess tag@0x94)>}`，
/// `Option` 用 tag 值 3 做 niche。证据：`create_render_pass` 0x140bfc0d0 读 `[desc+0x80]`=颜色数、
/// `[desc+0x84..0x94)`=深度附件 16B、`[desc+0x94]`=tag（跳表）；`RasterPass::render` 0x140868370
/// 构造该键（`iStack_a8`@0x80、`pauStack_a4/puStack_9c`@0x84/0x8c、`uStack_94`@0x94）、桶 stride 0xb0。
/// 注意：旧文档的 “ManagedHandle@0 + u32@0x18” 是 `RasterPipelineKey`（渲染通道句柄 + 选项 u32），不是本键。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct RenderPassDesc {
    pub colors: AttachmentArray,
    pub depth: Option<(RenderPassAttachmentDesc, DepthAccess)>,
}
impl RenderPassDesc {
    /// 从平铺附件序列（颜色在前、按格式识别出的深度附件）构造；深度访问模式取
    /// `ReadWrite`（`DEPTH_STENCIL_ATTACHMENT_OPTIMAL`）。多于一个深度附件时 panic
    /// （原版结构里只有一个 `Option` 深度槽）。
    pub fn from_flat(attachments: &[RenderPassAttachmentDesc]) -> Self {
        let mut desc = Self::default();
        for attachment in attachments {
            if attachment.is_depth_stencil() {
                assert!(desc.depth.is_none(), "RenderPassDesc 只有一个深度槽");
                desc.depth = Some((*attachment, DepthAccess::ReadWrite));
            } else {
                desc.colors.push(*attachment);
            }
        }
        desc
    }
    /// 通道采样数：有深度取深度附件，否则取首个颜色附件；两者皆无时原版越界 panic
    /// （0x140bfc47f `index out of bounds: len 0, index 0`）。
    pub fn pass_samples(&self) -> Result<u32> {
        if let Some((depth, _)) = &self.depth {
            return Ok(u32::from(depth.sample_count));
        }
        self.colors
            .as_slice()
            .first()
            .map(|a| u32::from(a.sample_count))
            .ok_or(super::error::Error::Invalid(
                "index out of bounds: the len is 0 but the index is 0",
            ))
    }
}

fn attachment_description(
    a: &RenderPassAttachmentDesc,
    layout: vk::ImageLayout,
) -> vk::AttachmentDescription {
    // `0x1418b8c50`：loadOp=desc[4]、storeOp=desc[8] 按原值写入；stencil 恒 LOAD/STORE(0/0)。
    vk::AttachmentDescription::default()
        .format(a.format)
        .samples(vk::SampleCountFlags::from_raw(u32::from(a.sample_count)))
        .load_op(vk::AttachmentLoadOp::from_raw(a.clear_input as i32))
        .store_op(vk::AttachmentStoreOp::from_raw(a.store_op as i32))
        .stencil_load_op(vk::AttachmentLoadOp::LOAD)
        .stencil_store_op(vk::AttachmentStoreOp::STORE)
        .initial_layout(layout)
        .final_layout(layout)
}

/// 原版 `RenderPass` 槽负载（`ManagedResourceStore` 槽 0x28B，`+0x10` 起 24B）：
/// `{vk::RenderPass@0, color_count:u64@8, samples:u32@0x10}`，对应槽内 `+0x10/+0x18/+0x20`
/// （机器码 0x140bfc371..0x140bfc3b9：`[rbp+0x58]=handle, [rbp+0x60]=ecx(颜色附件数), [rbp+0x68]=edx(采样数)`）。
pub struct RenderPass {
    pub device: Arc<VulkanDevice>,
    pub handle: vk::RenderPass,
    /// 槽 `+0x18`：颜色附件数（不含深度）。
    pub color_count: u32,
    /// 槽 `+0x20`：采样数；有深度取深度附件的 `sample_count`，否则取第 0 个颜色附件的。
    pub samples: u32,
}
impl RenderPass {
    pub fn from_key(device: Arc<VulkanDevice>, key: &RenderPassKey) -> Result<Self> {
        let descriptions: Vec<_> = key
            .attachments
            .iter()
            .map(|a| {
                vk::AttachmentDescription::default()
                    .format(vk::Format::from_raw(a.format))
                    .samples(vk::SampleCountFlags::from_raw(a.samples))
                    .load_op(vk::AttachmentLoadOp::from_raw(a.load))
                    .store_op(vk::AttachmentStoreOp::from_raw(a.store))
                    .stencil_load_op(vk::AttachmentLoadOp::from_raw(a.stencil_load))
                    .stencil_store_op(vk::AttachmentStoreOp::from_raw(a.stencil_store))
                    .initial_layout(vk::ImageLayout::from_raw(a.initial_layout))
                    .final_layout(vk::ImageLayout::from_raw(a.final_layout))
            })
            .collect();
        let colors: Vec<_> = key
            .attachments
            .iter()
            .enumerate()
            .filter(|(_, a)| !a.depth_stencil)
            .map(|(i, a)| vk::AttachmentReference {
                attachment: i as u32,
                layout: vk::ImageLayout::from_raw(a.subpass_layout),
            })
            .collect();
        let depth = key
            .attachments
            .iter()
            .enumerate()
            .find(|(_, a)| a.depth_stencil)
            .map(|(i, a)| vk::AttachmentReference {
                attachment: i as u32,
                layout: vk::ImageLayout::from_raw(a.subpass_layout),
            });
        let mut subpass = vk::SubpassDescription::default()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&colors);
        if let Some(ref reference) = depth {
            subpass = subpass.depth_stencil_attachment(reference);
        }
        let subpasses = [subpass];
        // 外部资源图负责进入子通道前后的资源屏障，此处不猜测原作依赖掩码。
        let info = vk::RenderPassCreateInfo::default()
            .attachments(&descriptions)
            .subpasses(&subpasses);
        let mut pass = Self::new(device, &info)?;
        // 与原版一致：有深度时采样数取深度附件的（`new` 只能看到首个附件）。
        if let Some(depth) = key.attachments.iter().find(|a| a.depth_stencil) {
            pass.samples = depth.samples;
        }
        Ok(pass)
    }
    pub fn new(device: Arc<VulkanDevice>, info: &vk::RenderPassCreateInfo<'_>) -> Result<Self> {
        let handle = unsafe { device.device.create_render_pass(info, None) }?;
        // 非 `create_render_pass(desc)` 路径没有 desc：按首个子通道/附件推出同样两项。
        let color_count = if info.subpass_count > 0 && !info.p_subpasses.is_null() {
            // SAFETY: 调用者给出的 create info 指针在本调用期间有效。
            unsafe { (*info.p_subpasses).color_attachment_count }
        } else {
            0
        };
        let samples = if info.attachment_count > 0 {
            // SAFETY: 同上，`attachment_count > 0` 时 `p_attachments` 非空。
            unsafe { (*info.p_attachments).samples.as_raw() }
        } else {
            0
        };
        Ok(Self {
            device,
            handle,
            color_count,
            samples,
        })
    }
    /// 原 `GraphicsDevice::create_render_pass` 本体（0x140bfc0d0，缓存未命中分支）：
    /// - 颜色附件逐个 `AttachmentDescription{flags:0, format, samples:=sample_count,
    ///   loadOp:=clear_input(原值), storeOp:=u32@8(原值), stencil:LOAD/STORE(0/0),
    ///   initial=final=COLOR_ATTACHMENT_OPTIMAL}`（`0x1418b8c50` 的 Chain 展开，常量 @0x142d032c0）；
    /// - 深度（`tag != 3`）追加在末尾，stencil LOAD/STORE，initial=final=按 tag 的布局；
    /// - 单个 GRAPHICS 子通道：颜色引用 `(i, COLOR_ATTACHMENT_OPTIMAL)`，深度引用 `(color_count, layout)`；
    /// - **没有子通道依赖**（`dependencyCount=0, pDependencies=null`，0x140bfc31c..0x140bfc335）。
    pub fn from_desc(device: Arc<VulkanDevice>, desc: &RenderPassDesc) -> Result<Self> {
        let samples = desc.pass_samples()?;
        let mut descriptions: Vec<vk::AttachmentDescription> = desc
            .colors
            .as_slice()
            .iter()
            .map(|a| attachment_description(a, vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL))
            .collect();
        if let Some((depth, access)) = desc.depth {
            descriptions.push(attachment_description(&depth, access.layout()));
        }
        let color_count = desc.colors.len();
        let colors: Vec<_> = (0..color_count)
            .map(|i| vk::AttachmentReference {
                attachment: i,
                layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            })
            .collect();
        let depth_ref = desc.depth.map(|(_, access)| vk::AttachmentReference {
            attachment: color_count,
            layout: access.layout(),
        });
        let mut subpass = vk::SubpassDescription::default()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&colors);
        if let Some(ref r) = depth_ref {
            subpass = subpass.depth_stencil_attachment(r);
        }
        let subpasses = [subpass];
        let info = vk::RenderPassCreateInfo::default()
            .attachments(&descriptions)
            .subpasses(&subpasses);
        let handle = unsafe { device.device.create_render_pass(&info, None) }?;
        Ok(Self {
            device,
            handle,
            color_count,
            samples,
        })
    }
}
/// 原 `GraphicsDevice::create_render_pass`（0x140bfc0d0）的 `vkCreateRenderPass` 失败路径（O）：
/// 0x140bfc435 起 `Result::unwrap()` → `called `Result::unwrap()` on an `Err` value: {vk::Result:?}`，
/// 源位置 `crates/rhapsody/src/raw/render_pass.rs:124:18`（Location 0x142b16b08，消息 0x142b15188 len 0x2b）。
/// 即原版**不返回错误而是 panic**；Rust 版仍以 `Result` 传出（与 `Framebuffer` 的 Err 变体相反，见下）。
pub const CREATE_RENDER_PASS_FAILURE_LOCATION: (&str, u32, u32) =
    ("crates/rhapsody/src/raw/render_pass.rs", 124, 18);
impl Drop for RenderPass {
    fn drop(&mut self) {
        self.device
            .bury(super::graveyard::Doomed::RenderPass(self.handle));
    }
}

#[cfg(test)]
mod attachment_desc_tests {
    use super::*;
    use std::mem::offset_of;

    #[test]
    fn layout_matches_the_exe_machine_code() {
        assert_eq!(std::mem::size_of::<RenderPassAttachmentDesc>(), 0x10);
        assert_eq!(offset_of!(RenderPassAttachmentDesc, format), 0x00);
        assert_eq!(offset_of!(RenderPassAttachmentDesc, clear_input), 0x04);
        assert_eq!(offset_of!(RenderPassAttachmentDesc, sample_count), 0x0c);
        let d = RenderPassAttachmentDesc::new(vk::Format::D32_SFLOAT);
        assert_eq!((d.clear_input, d.sample_count), (0, 1));
        assert!(d.is_depth_stencil());
        assert!(!RenderPassAttachmentDesc::new(vk::Format::R8G8B8A8_UNORM).is_depth_stencil());
        let d = d.clear_input().sample_count(4);
        assert_eq!((d.clear_input, d.sample_count), (1, 4));
        assert_eq!(d.to_attachment_key().samples, 4);
    }
}
