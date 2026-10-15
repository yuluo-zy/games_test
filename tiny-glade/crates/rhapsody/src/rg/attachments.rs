//! 从附件声明生产视图、渲染通道与帧缓冲；缓存拥有依赖，命令持有最终产物。
//! 原 PDB/汇编约束描述符与 begin 的顺序；公开签名、缓存容器仍是候选实现。
use crate::raw::{
    command_buffer::VulkanCommandBuffer,
    device::VulkanDevice,
    error::{Error, Result},
    framebuffer::Framebuffer,
    render_pass::{AttachmentKey, RenderPass, RenderPassKey},
    texture::{Texture, TextureView, TextureViewDesc},
};
use ash::vk;
use ash::vk::Handle;
use std::{collections::HashMap, sync::Arc};

/// 原 16 字节描述符：new 写 format、load/store=0、samples=1，尾部填充保持未定义。
/// clear_input 改 +4 为 1；sample_count 只改 +12。原二进制地址在验证报告中。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RenderPassAttachmentDesc {
    pub format: vk::Format,
    pub load: vk::AttachmentLoadOp,
    pub store: vk::AttachmentStoreOp,
    pub samples: vk::SampleCountFlags,
}

/// 原 create_render_pass 的深度状态 0/1/2 分别映射到以下布局；3 表示无深度。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DepthAccess {
    ReadOnly,
    ReadWrite,
    DepthWriteStencilRead,
}
impl DepthAccess {
    pub fn layout(self) -> vk::ImageLayout {
        match self {
            Self::ReadOnly => vk::ImageLayout::DEPTH_STENCIL_READ_ONLY_OPTIMAL,
            Self::ReadWrite => vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            Self::DepthWriteStencilRead => {
                vk::ImageLayout::DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL
            }
        }
    }
}
impl RenderPassAttachmentDesc {
    pub fn new(format: vk::Format) -> Self {
        Self {
            format,
            load: vk::AttachmentLoadOp::LOAD,
            store: vk::AttachmentStoreOp::STORE,
            samples: vk::SampleCountFlags::TYPE_1,
        }
    }
    pub fn clear_input(mut self) -> Self {
        self.load = vk::AttachmentLoadOp::CLEAR;
        self
    }
    pub fn sample_count(mut self, samples: vk::SampleCountFlags) -> Self {
        self.samples = samples;
        self
    }
}

#[derive(Clone)]
pub struct Attachment {
    pub texture: Arc<Texture>,
    /// 同一 managed 槽位重建时递增；同时使用实际拥有者身份避免裸句柄重用。
    pub generation: u64,
    pub view: TextureViewDesc,
    pub desc: RenderPassAttachmentDesc,
    pub stencil_load: vk::AttachmentLoadOp,
    pub stencil_store: vk::AttachmentStoreOp,
    pub initial_layout: vk::ImageLayout,
    pub final_layout: vk::ImageLayout,
    pub subpass_layout: vk::ImageLayout,
    pub clear: Option<vk::ClearValue>,
    pub depth_stencil: bool,
}
impl Attachment {
    pub fn color(texture: Arc<Texture>, generation: u64, clear: Option<[f32; 4]>) -> Self {
        let mut view = TextureViewDesc::new(&texture.desc);
        view.level_count = 1;
        let mut desc =
            RenderPassAttachmentDesc::new(view.format).sample_count(texture.desc.samples());
        if clear.is_some() {
            desc = desc.clear_input();
        }
        Self {
            texture,
            generation,
            view,
            desc,
            // 原 VkAttachmentDescription producer 固定 stencilLoad/Store 为 LOAD/STORE。
            stencil_load: vk::AttachmentLoadOp::LOAD,
            stencil_store: vk::AttachmentStoreOp::STORE,
            initial_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            final_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            subpass_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            clear: clear.map(|float32| vk::ClearValue {
                color: vk::ClearColorValue { float32 },
            }),
            depth_stencil: false,
        }
    }
    pub fn depth(texture: Arc<Texture>, generation: u64, clear: Option<f32>) -> Self {
        let mut value = Self::color(texture, generation, None);
        value.depth_stencil = true;
        value.initial_layout = vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL;
        value.final_layout = value.initial_layout;
        value.subpass_layout = value.initial_layout;
        value.clear = clear.map(|depth| vk::ClearValue {
            depth_stencil: vk::ClearDepthStencilValue { depth, stencil: 0 },
        });
        if clear.is_some() {
            value.desc = value.desc.clear_input();
        }
        value
    }
    pub fn depth_access(mut self, access: DepthAccess) -> Self {
        self.initial_layout = access.layout();
        self.final_layout = access.layout();
        self.subpass_layout = access.layout();
        self
    }
    fn render_key(&self) -> AttachmentKey {
        AttachmentKey {
            format: self.desc.format.as_raw(),
            samples: self.desc.samples.as_raw(),
            load: self.desc.load.as_raw(),
            store: self.desc.store.as_raw(),
            stencil_load: self.stencil_load.as_raw(),
            stencil_store: self.stencil_store.as_raw(),
            initial_layout: self.initial_layout.as_raw(),
            final_layout: self.final_layout.as_raw(),
            subpass_layout: self.subpass_layout.as_raw(),
            depth_stencil: self.depth_stencil,
        }
    }
    fn view_key(&self) -> ViewKey {
        ViewKey {
            image: self.texture.handle.as_raw(),
            owner: Arc::as_ptr(&self.texture) as usize,
            generation: self.generation,
            view_type: self.view.view_type.as_raw(),
            format: self.view.format.as_raw(),
            aspect: self.view.aspect.as_raw(),
            mip: self.view.base_mip_level,
            levels: self.view.level_count,
            layer: self.view.base_layer,
            layers: self.view.layer_count,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct ViewKey {
    image: u64,
    owner: usize,
    generation: u64,
    view_type: i32,
    format: i32,
    aspect: u32,
    mip: u32,
    levels: u32,
    layer: u32,
    layers: u32,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct FramebufferKey {
    render_pass: RenderPassKey,
    views: Vec<ViewKey>,
    width: u32,
    height: u32,
    layers: u32,
}

/// 不把 clear 数值放入兼容键：它只影响本次 begin，而不是 Vulkan 对象身份。
pub struct PreparedRenderPass {
    pub framebuffer: Arc<Framebuffer>,
    pub clears: Vec<vk::ClearValue>,
}
impl PreparedRenderPass {
    pub fn begin_info(&self) -> vk::RenderPassBeginInfo<'_> {
        vk::RenderPassBeginInfo::default()
            .render_pass(self.framebuffer.render_pass.handle)
            .framebuffer(self.framebuffer.handle)
            .render_area(vk::Rect2D {
                offset: vk::Offset2D::default(),
                extent: self.framebuffer.extent,
            })
            .clear_values(&self.clears)
    }
    /// 在命令资源列表登记 framebuffer；它间接持有 renderpass、view、texture。
    pub fn keep_alive(&self, command: &mut VulkanCommandBuffer) {
        command.keep_alive(self.framebuffer.clone());
    }
}

pub struct AttachmentCache {
    device: Arc<VulkanDevice>,
    limits: vk::PhysicalDeviceLimits,
    views: HashMap<ViewKey, Arc<TextureView>>,
    render_passes: HashMap<RenderPassKey, Arc<RenderPass>>,
    framebuffers: HashMap<FramebufferKey, Arc<Framebuffer>>,
}
impl AttachmentCache {
    pub fn new(device: Arc<VulkanDevice>) -> Self {
        let limits = unsafe {
            device
                .instance
                .instance
                .get_physical_device_properties(device.physical)
        }
        .limits;
        Self {
            device,
            limits,
            views: HashMap::new(),
            render_passes: HashMap::new(),
            framebuffers: HashMap::new(),
        }
    }
    pub fn counts(&self) -> (usize, usize, usize) {
        (
            self.views.len(),
            self.render_passes.len(),
            self.framebuffers.len(),
        )
    }
    /// 整次生产成功后才发布缓存项，失败不会留下半完成对象。
    pub fn prepare(
        &mut self,
        attachments: &[Attachment],
        extent: vk::Extent2D,
        layers: u32,
    ) -> Result<PreparedRenderPass> {
        self.validate(attachments, extent, layers)?;
        let render_key = RenderPassKey {
            attachments: attachments.iter().map(Attachment::render_key).collect(),
        };
        let view_keys: Vec<_> = attachments.iter().map(Attachment::view_key).collect();
        let frame_key = FramebufferKey {
            render_pass: render_key.clone(),
            views: view_keys.clone(),
            width: extent.width,
            height: extent.height,
            layers,
        };
        let clears = attachments
            .iter()
            .map(|a| {
                a.clear.unwrap_or(vk::ClearValue {
                    color: vk::ClearColorValue { uint32: [0; 4] },
                })
            })
            .collect();
        if let Some(framebuffer) = self.framebuffers.get(&frame_key) {
            return Ok(PreparedRenderPass {
                framebuffer: framebuffer.clone(),
                clears,
            });
        }
        let render_pass = match self.render_passes.get(&render_key) {
            Some(value) => value.clone(),
            None => Arc::new(RenderPass::from_key(self.device.clone(), &render_key)?),
        };
        let mut new_views = Vec::new();
        let mut views = Vec::new();
        for (attachment, key) in attachments.iter().zip(&view_keys) {
            let view = match self
                .views
                .get(key)
                .or_else(|| new_views.iter().find(|(k, _)| k == key).map(|(_, v)| v))
            {
                Some(value) => Arc::clone(value),
                None => {
                    let value = Arc::new(attachment.texture.image_view(&attachment.view)?);
                    new_views.push((key.clone(), value.clone()));
                    value
                }
            };
            views.push(view);
        }
        let framebuffer = Arc::new(Framebuffer::new(
            render_pass.clone(),
            views,
            extent,
            layers,
        )?);
        self.views.extend(new_views);
        self.render_passes.insert(render_key, render_pass);
        self.framebuffers.insert(frame_key, framebuffer.clone());
        Ok(PreparedRenderPass {
            framebuffer,
            clears,
        })
    }
    fn validate(
        &self,
        attachments: &[Attachment],
        extent: vk::Extent2D,
        layers: u32,
    ) -> Result<()> {
        if attachments.is_empty()
            || attachments.len() > 9
            || extent.width == 0
            || extent.height == 0
            || layers == 0
        {
            return Err(Error::Invalid("附件数应为 1..9，帧缓冲范围及层数必须非零"));
        }
        let colors = attachments.iter().filter(|a| !a.depth_stencil).count();
        if colors > 8
            || colors > self.limits.max_color_attachments as usize
            || extent.width > self.limits.max_framebuffer_width
            || extent.height > self.limits.max_framebuffer_height
            || layers > self.limits.max_framebuffer_layers
        {
            return Err(Error::Invalid("附件数或帧缓冲范围超出设备能力"));
        }
        let mut depth_seen = false;
        let samples = attachments[0].desc.samples;
        for a in attachments {
            if !Arc::ptr_eq(&self.device, &a.texture.device) {
                return Err(Error::Invalid("附件属于其他设备"));
            }
            if depth_seen {
                return Err(Error::Invalid("只能有一个深度附件并且必须位于最后"));
            }
            depth_seen = a.depth_stencil;
            let desc = &a.texture.desc;
            if a.view.view_type != vk::ImageViewType::TYPE_2D
                && a.view.view_type != vk::ImageViewType::TYPE_2D_ARRAY
            {
                return Err(Error::Invalid("帧缓冲附件必须使用二维或二维数组视图"));
            }
            let valid_subpass = if a.depth_stencil {
                [
                    vk::ImageLayout::GENERAL,
                    vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                    vk::ImageLayout::DEPTH_STENCIL_READ_ONLY_OPTIMAL,
                    vk::ImageLayout::DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL,
                    vk::ImageLayout::DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL,
                ]
                .contains(&a.subpass_layout)
            } else {
                [
                    vk::ImageLayout::GENERAL,
                    vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                ]
                .contains(&a.subpass_layout)
            };
            if !valid_subpass {
                return Err(Error::Invalid("附件子通道布局与类型不兼容"));
            }
            if a.desc.format != a.view.format
                || a.desc.samples != desc.samples()
                || a.desc.samples != samples
            {
                return Err(Error::Invalid("附件格式或采样数不兼容"));
            }
            let expected_aspect = if a.depth_stencil {
                vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL
            } else {
                vk::ImageAspectFlags::COLOR
            };
            let usage = if a.depth_stencil {
                vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT
            } else {
                vk::ImageUsageFlags::COLOR_ATTACHMENT
            };
            if a.view.aspect.is_empty()
                || !expected_aspect.contains(a.view.aspect)
                || !desc.usage.contains(usage)
            {
                return Err(Error::Invalid("附件 aspect 或 usage 不兼容"));
            }
            let mip = a.view.base_mip_level;
            if a.view.level_count != 1
                || mip >= u32::from(desc.mip_levels)
                || mip >= 32
                || a.view.layer_count < layers
                || a.view
                    .base_layer
                    .checked_add(a.view.layer_count)
                    .is_none_or(|n| n > desc.array_layers())
                || extent.width > (desc.extent().width >> mip).max(1)
                || extent.height > (desc.extent().height >> mip).max(1)
            {
                return Err(Error::Invalid("帧缓冲范围越过附件子资源"));
            }
            if (a.desc.load == vk::AttachmentLoadOp::CLEAR
                || a.stencil_load == vk::AttachmentLoadOp::CLEAR)
                && a.clear.is_none()
            {
                return Err(Error::Invalid("CLEAR 附件未提供清除值"));
            }
            if (a.desc.load == vk::AttachmentLoadOp::LOAD
                || (a.view.aspect.contains(vk::ImageAspectFlags::STENCIL)
                    && a.stencil_load == vk::AttachmentLoadOp::LOAD))
                && a.initial_layout == vk::ImageLayout::UNDEFINED
            {
                return Err(Error::Invalid("LOAD 附件不能来自未定义布局"));
            }
            if a.final_layout == vk::ImageLayout::UNDEFINED
                || a.final_layout == vk::ImageLayout::PREINITIALIZED
            {
                return Err(Error::Invalid("附件最终布局无效"));
            }
        }
        Ok(())
    }
    /// 清理不撤销已经录制命令的拥有权，正在使用的对象由 Prepared/Command 持有。
    pub fn invalidate_image(&mut self, image: vk::Image, generation: u64) {
        self.framebuffers.retain(|k, _| {
            !k.views
                .iter()
                .any(|v| v.image == image.as_raw() && v.generation == generation)
        });
        self.views
            .retain(|k, _| k.image != image.as_raw() || k.generation != generation);
    }
    pub fn clear(&mut self) {
        self.framebuffers.clear();
        self.views.clear();
        self.render_passes.clear();
    }
}
