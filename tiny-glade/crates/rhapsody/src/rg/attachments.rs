//! Dynamic raster attachments and bounded image-view caching.
use crate::raw::{
    device::Device,
    error::{Error, Result},
    shader::DynamicRenderingFormats,
    texture::{Texture, TextureView, TextureViewDesc},
};
#[path = "attachment_contract.rs"]
pub mod contract;
use ash::vk;
use ash::vk::Handle;
use std::sync::Arc;

/// 16 字节附件描述符：new 写 format、load/store=0、samples=1。
/// clear_input 改 +4 为 1；sample_count 只改 +12。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RenderPassAttachmentDesc {
    pub format: vk::Format,
    pub load: vk::AttachmentLoadOp,
    pub store: vk::AttachmentStoreOp,
    pub samples: vk::SampleCountFlags,
}

pub use contract::DepthAccess;
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
    pub fn color(texture: Arc<Texture>, generation: u64, clear: Option<[f32; 4]>) -> Result<Self> {
        let mut view = TextureViewDesc::new(&texture.desc)?;
        view.level_count = 1;
        let mut desc =
            RenderPassAttachmentDesc::new(view.format).sample_count(texture.desc.samples());
        if clear.is_some() {
            desc = desc.clear_input();
        }
        Ok(Self {
            texture,
            generation,
            view,
            desc,
            // 模板 load/store 固定为 LOAD/STORE。
            stencil_load: vk::AttachmentLoadOp::LOAD,
            stencil_store: vk::AttachmentStoreOp::STORE,
            initial_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            final_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            subpass_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            clear: clear.map(|float32| vk::ClearValue {
                color: vk::ClearColorValue { float32 },
            }),
            depth_stencil: false,
        })
    }
    pub fn depth(texture: Arc<Texture>, generation: u64, clear: Option<f32>) -> Result<Self> {
        let mut value = Self::color(texture, generation, None)?;
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
        Ok(value)
    }
    pub fn depth_access(mut self, access: DepthAccess) -> Self {
        self.initial_layout = access.layout();
        self.final_layout = access.layout();
        self.subpass_layout = access.layout();
        self
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RenderingPolicy {
    #[default]
    Auto,
    RequireDynamic,
}
impl RenderingPolicy {
    /// Supported hardware is insufficient: the feature must actually be enabled.
    pub fn select_dynamic(self, enabled: bool, layouts_compatible: bool) -> Result<bool> {
        match self {
            Self::Auto | Self::RequireDynamic if !enabled => Err(Error::unsupported(
                "dynamicRendering feature was not enabled",
            )),
            Self::Auto | Self::RequireDynamic if !layouts_compatible => Err(Error::unsupported(
                "dynamic rendering requires explicit graph layout transitions",
            )),
            Self::Auto | Self::RequireDynamic => Ok(true),
        }
    }
}

/// Only views are needed; there is no VkRenderPass or VkFramebuffer allocation.
pub struct PreparedDynamicRendering {
    pub views: Vec<Arc<TextureView>>,
    pub formats: DynamicRenderingFormats,
    pub samples: vk::SampleCountFlags,
    pub extent: vk::Extent2D,
    pub layers: u32,
    colors: Vec<vk::RenderingAttachmentInfo<'static>>,
    depth: Option<vk::RenderingAttachmentInfo<'static>>,
    stencil: Option<vk::RenderingAttachmentInfo<'static>>,
    // Actual Arc and Vec backing is freed before releasing its budget credit.
    _memory: crate::raw::allocator::CpuMemoryReservation,
}
impl PreparedDynamicRendering {
    pub fn begin_info(&self) -> vk::RenderingInfo<'_> {
        let mut info = vk::RenderingInfo::default()
            .render_area(vk::Rect2D {
                offset: vk::Offset2D::default(),
                extent: self.extent,
            })
            .layer_count(self.layers)
            .view_mask(self.formats.view_mask)
            .color_attachments(&self.colors);
        if let Some(depth) = &self.depth {
            info = info.depth_attachment(depth);
        }
        if let Some(stencil) = &self.stencil {
            info = info.stencil_attachment(stencil);
        }
        info
    }
}
pub enum PreparedAttachments {
    Dynamic(PreparedDynamicRendering),
}
impl PreparedAttachments {
    pub fn dynamic_formats(&self) -> Option<&DynamicRenderingFormats> {
        match self {
            Self::Dynamic(rendering) => Some(&rendering.formats),
        }
    }
    /// Attach native views to a raw recording only after its explicit image
    /// access patches cover every attachment range and native layout.
    pub fn retain_for_command(
        self: &Arc<Self>,
        command: &mut crate::raw::command_buffer::VulkanCommandBuffer,
    ) -> Result<()> {
        let Self::Dynamic(rendering) = &**self;
        for view in &rendering.views {
            let (layout, required) = self.native_access(view)?;
            let range = crate::resource::OwnershipRange::Image {
                aspects: view.desc.aspect.as_raw(),
                base_mip: view.desc.base_mip_level,
                mip_count: view.desc.level_count,
                base_layer: view.desc.base_layer,
                layer_count: view.desc.layer_count,
            };
            if !command.recorded_access_covers(
                &view.texture.last_use,
                range,
                |scope, actual_layout| actual_layout == layout && scope.access.contains(required),
            )? {
                return Err(Error::invalid_usage(
                    "attachment view or layout has no recorded native access",
                ));
            }
        }
        // SAFETY: every native image reference was checked against this
        // recording's explicit patches; view SyncCells carry the actual point.
        unsafe { command.retain_recorded(self.clone()) }
    }
    fn native_access(&self, view: &TextureView) -> Result<(vk::ImageLayout, vk::AccessFlags)> {
        let Self::Dynamic(rendering) = self;
        if let Some(info) = rendering
            .colors
            .iter()
            .find(|info| info.image_view == view.handle)
        {
            let mut required = vk::AccessFlags::COLOR_ATTACHMENT_WRITE;
            if info.load_op == vk::AttachmentLoadOp::LOAD {
                required |= vk::AccessFlags::COLOR_ATTACHMENT_READ;
            }
            Ok((info.image_layout, required))
        } else if let Some(info) = rendering
            .depth
            .iter()
            .chain(&rendering.stencil)
            .find(|info| info.image_view == view.handle)
        {
            let mut required = vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ;
            if info.image_layout != vk::ImageLayout::DEPTH_STENCIL_READ_ONLY_OPTIMAL {
                required |= vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE;
            }
            Ok((info.image_layout, required))
        } else {
            Err(Error::invalid_usage(
                "prepared view has no native attachment",
            ))
        }
    }
    /// Retain native views after proving their attachment ranges and layouts
    /// are covered by this pass. Physical graph state still comes exclusively
    /// from the bound synchronization program.
    pub fn retain_for_pass(
        self: &Arc<Self>,
        context: &super::executor::PassContext<'_>,
    ) -> Result<()> {
        use super::{resource::AccessRange, vulkan::ResourceSpec};
        let managed = context
            .managed
            .ok_or_else(|| Error::invalid_usage("attachments require managed graph resources"))?;
        let pass = context
            .compiled
            .declarations
            .passes
            .get(context.pass_index)
            .ok_or_else(|| Error::invalid_usage("attachment pass index is invalid"))?;
        let Self::Dynamic(rendering) = &**self;
        for view in &rendering.views {
            let (layout, required) = self.native_access(view)?;
            let range = AccessRange::Texture {
                aspect_mask: view.desc.aspect.as_raw(),
                base_mip_level: view.desc.base_mip_level,
                level_count: view.desc.level_count,
                base_array_layer: view.desc.base_layer,
                layer_count: view.desc.layer_count,
            };
            let ranges = pass
                .accesses
                .iter()
                .filter(|access| {
                    context
                        .compiled
                        .plan
                        .definition
                        .access_semantics
                        .iter()
                        .find(|entry| entry.code == access.access_code)
                        .is_some_and(|native| {
                            native.layout == layout && native.scope.access.contains(required)
                        })
                        && managed
                            .texture_for(access.resource_index)
                            .is_ok_and(|candidate| {
                                std::ptr::eq(candidate.resource(), &*view.texture)
                            })
                })
                .map(|access| access.range);
            if !super::resource_contracts::covers_iter(
                ranges,
                range,
                ResourceSpec::Image(view.texture.desc).descriptor(),
            )
            .map_err(|_| Error::invalid_usage("attachment view range is invalid"))?
            {
                return Err(Error::invalid_usage(
                    "attachment view or layout is not declared by this graph pass",
                ));
            }
        }
        // SAFETY: every native view was checked against the pass declaration.
        unsafe { context.retain_recorded(self.clone()) }
    }
}
impl crate::resource::GpuUse for PreparedAttachments {
    fn visit_uses(&self, visit: &mut crate::resource::UseVisitor<'_>) -> Result<()> {
        match self {
            Self::Dynamic(rendering) => {
                for view in &rendering.views {
                    view.visit_uses(visit)?;
                }
                Ok(())
            }
        }
    }
}
pub struct AttachmentCache {
    device: Arc<Device>,
    limits: vk::PhysicalDeviceLimits,
    capacity: usize,
    views: Vec<(ViewKey, Arc<TextureView>)>,
    _view_memory: crate::raw::allocator::CpuMemoryReservation,
}
impl AttachmentCache {
    pub fn new(device: Arc<Device>) -> Result<Self> {
        Self::with_capacity(device, 256)
    }
    /// Per-table object limit. A saturated cache retains all externally held or
    /// GPU-active entries and applies Timeout backpressure until they retire.
    pub fn with_capacity(device: Arc<Device>, capacity: usize) -> Result<Self> {
        if capacity == 0 || capacity > 65_536 {
            return Err(Error::invalid_usage(
                "attachment cache capacity must be within 1..=65536",
            ));
        }
        let limits = unsafe {
            device
                .core
                .instance
                .instance
                .get_physical_device_properties(device.core.physical)
        }
        .limits;
        let memory = device
            .memory
            .stats
            .reserve_cpu_bytes(0, crate::device::BudgetCategory::Cache)?;
        Ok(Self {
            device,
            limits,
            capacity,
            views: Vec::new(),
            _view_memory: memory,
        })
    }
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    pub fn view_count(&self) -> usize {
        self.views.len()
    }
    pub fn prepare_for_rendering(
        &mut self,
        attachments: &[Attachment],
        extent: vk::Extent2D,
        layers: u32,
        policy: RenderingPolicy,
    ) -> Result<Arc<PreparedAttachments>> {
        self.validate(attachments, extent, layers)?;
        let compatible = attachments.iter().all(|attachment| {
            attachment.initial_layout == attachment.subpass_layout
                && attachment.final_layout == attachment.subpass_layout
        });
        let enabled = self
            .device
            .caps()
            .is_enabled(crate::device::Feature::DynamicRendering);
        policy.select_dynamic(enabled, compatible)?;
        let view_keys = attachments
            .iter()
            .map(Attachment::view_key)
            .collect::<smallvec::SmallVec<[_; 9]>>();
        self.reserve_views(&view_keys)?;
        let color_count = attachments
            .iter()
            .filter(|attachment| !attachment.depth_stencil)
            .count();
        let mut memory = self.device.memory.stats.reserve_cpu_bytes(
            crate::raw::allocator::shared_cpu_bytes::<PreparedAttachments>()?
                + (attachments.len() * size_of::<Arc<TextureView>>()
                    + color_count
                        * (size_of::<vk::Format>()
                            + size_of::<vk::RenderingAttachmentInfo<'static>>()))
                    as u64,
            crate::device::BudgetCategory::Live,
        )?;
        let mut views = Vec::new();
        views
            .try_reserve_exact(attachments.len())
            .map_err(|_| Error::OutOfHostMemory)?;
        let mut new_views = smallvec::SmallVec::<[(ViewKey, Arc<TextureView>); 9]>::new();
        for attachment in attachments {
            let key = attachment.view_key();
            let view = match self
                .views
                .iter()
                .find(|(old, _)| old == &key)
                .map(|(_, view)| view)
                .or_else(|| {
                    new_views
                        .iter()
                        .find(|(old, _)| old == &key)
                        .map(|(_, view)| view)
                }) {
                Some(view) => view.clone(),
                None => {
                    let view = Arc::new(attachment.texture.image_view(&attachment.view)?);
                    new_views.push((key, view.clone()));
                    view
                }
            };
            views.push(view);
        }
        let mut formats = DynamicRenderingFormats {
            colors: Vec::new(),
            depth: vk::Format::UNDEFINED,
            stencil: vk::Format::UNDEFINED,
            view_mask: 0,
        };
        let mut colors = Vec::new();
        formats
            .colors
            .try_reserve_exact(color_count)
            .map_err(|_| Error::OutOfHostMemory)?;
        colors
            .try_reserve_exact(color_count)
            .map_err(|_| Error::OutOfHostMemory)?;
        let mut depth = None;
        let mut stencil = None;
        for (attachment, view) in attachments.iter().zip(&views) {
            let info = vk::RenderingAttachmentInfo::default()
                .image_view(view.handle)
                .image_layout(attachment.subpass_layout)
                .load_op(attachment.desc.load)
                .store_op(attachment.desc.store)
                .clear_value(attachment.clear.unwrap_or(vk::ClearValue {
                    color: vk::ClearColorValue { uint32: [0; 4] },
                }));
            if attachment.depth_stencil {
                let aspects =
                    crate::raw::format::vk_format_valid_aspect_flags(attachment.desc.format);
                if aspects.contains(vk::ImageAspectFlags::DEPTH) {
                    formats.depth = attachment.desc.format;
                    depth = Some(info);
                }
                if aspects.contains(vk::ImageAspectFlags::STENCIL) {
                    formats.stencil = attachment.desc.format;
                    stencil = Some(
                        info.load_op(attachment.stencil_load)
                            .store_op(attachment.stencil_store),
                    );
                }
            } else {
                formats.colors.push(attachment.desc.format);
                colors.push(info);
            }
        }
        formats.validate(self.limits.max_color_attachments)?;
        memory.resize(
            crate::raw::allocator::shared_cpu_bytes::<PreparedAttachments>()?
                + (views.capacity() * size_of::<Arc<TextureView>>()
                    + formats.colors.capacity() * size_of::<vk::Format>()
                    + colors.capacity() * size_of::<vk::RenderingAttachmentInfo<'static>>())
                    as u64,
        )?;
        self.views.extend(new_views);
        Ok(Arc::new(PreparedAttachments::Dynamic(
            PreparedDynamicRendering {
                views,
                formats,
                samples: attachments[0].desc.samples,
                extent,
                layers,
                colors,
                depth,
                stencil,
                _memory: memory,
            },
        )))
    }
    /// 整次生产成功后才发布缓存项，失败不会留下半完成对象。
    fn reserve_views(&mut self, views: &[ViewKey]) -> Result<()> {
        let missing = views
            .iter()
            .enumerate()
            .filter(|(index, key)| {
                !views[..*index].contains(key) && !self.views.iter().any(|(old, _)| old == *key)
            })
            .count();
        if self.views.len().saturating_add(missing) > self.capacity {
            let completed = self.device.poll()?;
            while self.views.len().saturating_add(missing) > self.capacity {
                let victim = self
                    .views
                    .iter()
                    .position(|(key, value)| {
                        !views.contains(key)
                            && Arc::strong_count(value) == 1
                            && value.last_use.is_reached(&completed)
                    })
                    .ok_or(Error::Timeout)?;
                self.views.swap_remove(victim);
            }
        }
        let needed = self.views.len() + missing;
        if needed > self.views.capacity() {
            let capacity = needed
                .max(self.views.capacity().saturating_mul(2))
                .min(self.capacity);
            let mut memory = self.device.memory.stats.reserve_cpu_bytes(
                (capacity * size_of::<(ViewKey, Arc<TextureView>)>()) as u64,
                crate::device::BudgetCategory::Cache,
            )?;
            let mut next = Vec::new();
            next.try_reserve_exact(capacity)
                .map_err(|_| Error::OutOfHostMemory)?;
            memory.resize((next.capacity() * size_of::<(ViewKey, Arc<TextureView>)>()) as u64)?;
            next.append(&mut self.views);
            std::mem::swap(&mut self.views, &mut next);
            std::mem::swap(&mut self._view_memory, &mut memory);
        }
        Ok(())
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
            return Err(Error::invalid_usage(
                "附件数应为 1..9，帧缓冲范围及层数必须非零",
            ));
        }
        let colors = attachments.iter().filter(|a| !a.depth_stencil).count();
        if colors > 8
            || colors > self.limits.max_color_attachments as usize
            || extent.width > self.limits.max_framebuffer_width
            || extent.height > self.limits.max_framebuffer_height
            || layers > self.limits.max_framebuffer_layers
        {
            return Err(Error::invalid_usage("附件数或帧缓冲范围超出设备能力"));
        }
        let mut depth_seen = false;
        let samples = attachments[0].desc.samples;
        for a in attachments {
            if !Arc::ptr_eq(&self.device, &a.texture.device) {
                return Err(Error::invalid_usage("附件属于其他设备"));
            }
            if depth_seen {
                return Err(Error::invalid_usage("只能有一个深度附件并且必须位于最后"));
            }
            depth_seen = a.depth_stencil;
            let desc = &a.texture.desc;
            if a.view.view_type != vk::ImageViewType::TYPE_2D
                && a.view.view_type != vk::ImageViewType::TYPE_2D_ARRAY
            {
                return Err(Error::invalid_usage("帧缓冲附件必须使用二维或二维数组视图"));
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
                return Err(Error::invalid_usage("附件子通道布局与类型不兼容"));
            }
            if a.desc.format != a.view.format
                || a.desc.samples != desc.samples()
                || a.desc.samples != samples
            {
                return Err(Error::invalid_usage("附件格式或采样数不兼容"));
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
                return Err(Error::invalid_usage("附件 aspect 或 usage 不兼容"));
            }
            let array_layers = desc.array_layers()?;
            let mip = a.view.base_mip_level;
            if a.view.level_count != 1
                || mip >= u32::from(desc.mip_levels)
                || mip >= 32
                || a.view.layer_count < layers
                || a.view
                    .base_layer
                    .checked_add(a.view.layer_count)
                    .is_none_or(|n| n > array_layers)
                || extent.width > (desc.extent().width >> mip).max(1)
                || extent.height > (desc.extent().height >> mip).max(1)
            {
                return Err(Error::invalid_usage("帧缓冲范围越过附件子资源"));
            }
            if (a.desc.load == vk::AttachmentLoadOp::CLEAR
                || a.stencil_load == vk::AttachmentLoadOp::CLEAR)
                && a.clear.is_none()
            {
                return Err(Error::invalid_usage("CLEAR 附件未提供清除值"));
            }
            if (a.desc.load == vk::AttachmentLoadOp::LOAD
                || (a.view.aspect.contains(vk::ImageAspectFlags::STENCIL)
                    && a.stencil_load == vk::AttachmentLoadOp::LOAD))
                && a.initial_layout == vk::ImageLayout::UNDEFINED
            {
                return Err(Error::invalid_usage("LOAD 附件不能来自未定义布局"));
            }
            if a.final_layout == vk::ImageLayout::UNDEFINED
                || a.final_layout == vk::ImageLayout::PREINITIALIZED
            {
                return Err(Error::invalid_usage("附件最终布局无效"));
            }
        }
        Ok(())
    }
    /// 清理不撤销已经录制命令的拥有权，正在使用的对象由 Prepared/Command 持有。
    pub fn invalidate_image(&mut self, image: vk::Image, generation: u64) {
        self.views
            .retain(|(k, _)| k.image != image.as_raw() || k.generation != generation);
    }
    pub fn clear(&mut self) {
        self.views.clear();
    }
}

#[cfg(test)]
mod rendering_policy_tests {
    use super::*;
    #[test]
    fn auto_uses_only_enabled_feature_with_explicit_layouts() {
        assert!(RenderingPolicy::Auto.select_dynamic(false, true).is_err());
        assert!(RenderingPolicy::Auto.select_dynamic(true, false).is_err());
        assert!(RenderingPolicy::Auto.select_dynamic(true, true).unwrap());
    }
    #[test]
    fn required_dynamic_rejects_unsupported_or_implicit_transition() {
        for (enabled, compatible) in [(false, true), (true, false), (false, false)] {
            assert_eq!(
                RenderingPolicy::RequireDynamic
                    .select_dynamic(enabled, compatible)
                    .unwrap_err()
                    .kind(),
                crate::ErrorKind::Unsupported
            );
        }
        assert!(
            RenderingPolicy::RequireDynamic
                .select_dynamic(true, true)
                .unwrap()
        );
    }
}
