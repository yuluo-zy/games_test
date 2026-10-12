//! shader 资产选择、反射布局生产、管线与缓存的可运行候选。
//! SPIR-V 解析复用原锁 rspirv；反射 schema 复用原 spirq fork 已保存目录。
//! 原资产键到 country-core pass 的关联、完整原缓存策略与 ABI 仍未全部对齐。
use crate::raw::binding::ResourceOwner;
use crate::raw::descriptor_set::VulkanDescriptorSetLayout;
use crate::raw::descriptor_set::{Binding, DescriptorSetState, LayoutPlan, descriptor_set_layout};
use crate::raw::device::VulkanDevice;
use ash::vk;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    ffi::{CStr, CString},
    io::Cursor,
    sync::{Arc, Mutex},
};

pub struct ShaderModule {
    pub device: Arc<VulkanDevice>,
    pub handle: vk::ShaderModule,
}
impl ShaderModule {
    pub fn from_spirv(device: Arc<VulkanDevice>, bytes: &[u8]) -> Result<Arc<Self>, vk::Result> {
        let words = ash::util::read_spv(&mut Cursor::new(bytes))
            .map_err(|_| vk::Result::ERROR_INVALID_SHADER_NV)?;
        if words.first().copied() != Some(0x07230203) {
            return Err(vk::Result::ERROR_INVALID_SHADER_NV);
        }
        let info = vk::ShaderModuleCreateInfo::default().code(&words);
        let handle = unsafe { device.device.create_shader_module(&info, None) }?;
        Ok(Arc::new(Self { device, handle }))
    }
}
impl Drop for ShaderModule {
    fn drop(&mut self) {
        unsafe {
            self.device.device.destroy_shader_module(self.handle, None);
        }
    }
}

pub struct PipelineLayout {
    pub device: Arc<VulkanDevice>,
    pub handle: vk::PipelineLayout,
    pub sets: Vec<Arc<VulkanDescriptorSetLayout>>,
    pub push_constants: Vec<vk::PushConstantRange>,
}
impl PipelineLayout {
    pub fn new(
        device: Arc<VulkanDevice>,
        sets: Vec<Arc<VulkanDescriptorSetLayout>>,
        push_constants: Vec<vk::PushConstantRange>,
    ) -> Result<Arc<Self>, vk::Result> {
        if sets.iter().any(|s| !Arc::ptr_eq(&device, &s.device)) {
            return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
        }
        let limits = unsafe {
            device
                .instance
                .instance
                .get_physical_device_properties(device.physical)
        }
        .limits;
        if sets.len() > limits.max_bound_descriptor_sets as usize
            || push_constants.iter().any(|range| {
                range.size == 0
                    || range.size % 4 != 0
                    || range.offset % 4 != 0
                    || range.stage_flags.is_empty()
                    || range
                        .offset
                        .checked_add(range.size)
                        .is_none_or(|end| end > limits.max_push_constants_size)
            })
        {
            return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
        }
        for (index, range) in push_constants.iter().enumerate() {
            if push_constants[..index]
                .iter()
                .any(|prior| prior.stage_flags.intersects(range.stage_flags))
            {
                return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
            }
        }
        let raw: Vec<_> = sets.iter().map(|s| s.handle).collect();
        let info = vk::PipelineLayoutCreateInfo::default()
            .set_layouts(&raw)
            .push_constant_ranges(&push_constants);
        let handle = unsafe { device.device.create_pipeline_layout(&info, None) }?;
        Ok(Arc::new(Self {
            device,
            handle,
            sets,
            push_constants,
        }))
    }
}
impl Drop for PipelineLayout {
    fn drop(&mut self) {
        unsafe {
            let _ = self.device.wait_idle();
            self.device
                .device
                .destroy_pipeline_layout(self.handle, None);
        }
    }
}

/// Vulkan 驱动缓存；序列化其读写及 pipeline 创建，满足缓存宿主同步要求。
type ComputePipelineKey = (u64, u64, Vec<u8>);
pub struct PipelineCache {
    pub device: Arc<VulkanDevice>,
    handle: vk::PipelineCache,
    lock: Mutex<()>,
    compute: Mutex<BTreeMap<ComputePipelineKey, Arc<VulkanPipeline>>>,
    raster: Mutex<BTreeMap<Vec<u64>, Arc<VulkanPipeline>>>,
}
impl PipelineCache {
    pub fn new(device: Arc<VulkanDevice>, initial_data: &[u8]) -> Result<Self, vk::Result> {
        let info = vk::PipelineCacheCreateInfo::default().initial_data(initial_data);
        let handle = unsafe { device.device.create_pipeline_cache(&info, None) }?;
        Ok(Self {
            device,
            handle,
            lock: Mutex::new(()),
            compute: Mutex::new(BTreeMap::new()),
            raster: Mutex::new(BTreeMap::new()),
        })
    }
    pub fn data(&self) -> Result<Vec<u8>, vk::Result> {
        let _lock = self.lock.lock().unwrap_or_else(|p| p.into_inner());
        unsafe { self.device.device.get_pipeline_cache_data(self.handle) }
    }
    /// 管线缓存持有布局与 shader；显式失效后已有提交仍由外部 Arc 保持有效。
    pub fn invalidate(&self) {
        self.compute
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .clear();
        self.raster
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .clear();
    }
    pub fn cached_compute_count(&self) -> usize {
        self.compute.lock().unwrap_or_else(|p| p.into_inner()).len()
    }
}
impl Drop for PipelineCache {
    fn drop(&mut self) {
        unsafe { self.device.device.destroy_pipeline_cache(self.handle, None) }
    }
}

pub struct ComputePipelineDesc {
    pub shader: Arc<ShaderModule>,
    pub entry: CString,
    pub layout: Arc<PipelineLayout>,
}
impl ComputePipelineDesc {
    pub fn new(shader: Arc<ShaderModule>, layout: Arc<PipelineLayout>) -> Self {
        Self {
            shader,
            entry: CString::new("main").unwrap(),
            layout,
        }
    }
}

/// 图形状态显式交给调用者；不猜原作各材质的默认深度、混合或剔除规则。
pub struct RasterPipelineDesc {
    pub vertex: Arc<ShaderModule>,
    pub fragment: Option<Arc<ShaderModule>>,
    pub entry: CString,
    /// 原发行 vertex/fragment 入口通常分别为 vs_main/ps_main，不能误用同一名称。
    pub fragment_entry: Option<CString>,
    pub layout: Arc<PipelineLayout>,
    pub render_pass: vk::RenderPass,
    pub render_pass_owner: ResourceOwner,
    pub subpass: u32,
    pub vertex_bindings: Vec<vk::VertexInputBindingDescription>,
    pub vertex_attributes: Vec<vk::VertexInputAttributeDescription>,
    pub topology: vk::PrimitiveTopology,
    pub polygon_mode: vk::PolygonMode,
    pub cull_mode: vk::CullModeFlags,
    pub front_face: vk::FrontFace,
    pub samples: vk::SampleCountFlags,
    pub depth_test: bool,
    pub depth_write: bool,
    pub depth_compare: vk::CompareOp,
    pub blend_attachments: Vec<vk::PipelineColorBlendAttachmentState>,
}
impl RasterPipelineDesc {
    /// 构造带必需 vertex/layout 的默认图形管线描述；其余固定状态由调用方填写。
    pub fn new(vertex: Arc<ShaderModule>, layout: Arc<PipelineLayout>) -> Self {
        Self {
            vertex,
            fragment: None,
            entry: CString::new("vs_main").unwrap_or_default(),
            fragment_entry: None,
            layout,
            render_pass: vk::RenderPass::null(),
            render_pass_owner: Arc::from(Box::new(()) as Box<dyn Send + Sync>),
            subpass: 0,
            vertex_bindings: Vec::new(),
            vertex_attributes: Vec::new(),
            topology: vk::PrimitiveTopology::TRIANGLE_LIST,
            polygon_mode: vk::PolygonMode::FILL,
            cull_mode: vk::CullModeFlags::BACK,
            front_face: vk::FrontFace::COUNTER_CLOCKWISE,
            samples: vk::SampleCountFlags::TYPE_1,
            depth_test: false,
            depth_write: false,
            depth_compare: vk::CompareOp::LESS_OR_EQUAL,
            blend_attachments: Vec::new(),
        }
    }
}

/// 固定状态选项构建器。getter 返回 `Option`（未设置），`or_err` 在缺失时报错，
/// `set_*` 直接覆盖，`set_*_checked` 先验证取值再写入。
#[derive(Clone, Debug, Default)]
pub struct RasterPipelineOpts {
    pub topology: Option<vk::PrimitiveTopology>,
    pub polygon_mode: Option<vk::PolygonMode>,
    pub cull_mode: Option<vk::CullModeFlags>,
    pub front_face: Option<vk::FrontFace>,
    pub samples: Option<vk::SampleCountFlags>,
    pub depth_test: Option<bool>,
    pub depth_write: Option<bool>,
    pub depth_compare: Option<vk::CompareOp>,
    pub stencil_test: Option<bool>,
    pub stencil_pass_op: Option<vk::StencilOp>,
    pub stencil_compare_op: Option<vk::CompareOp>,
    pub stencil_compare_mask: Option<u32>,
    pub blend_mode: Option<u8>,
    pub line_width: Option<f32>,
}
impl RasterPipelineOpts {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn topology(&self) -> Option<vk::PrimitiveTopology> {
        self.topology
    }
    pub fn topology_or_err(&self) -> Result<vk::PrimitiveTopology, vk::Result> {
        self.topology.ok_or(vk::Result::ERROR_INITIALIZATION_FAILED)
    }
    pub fn set_topology(&mut self, value: vk::PrimitiveTopology) {
        self.topology = Some(value);
    }
    pub fn set_topology_checked(&mut self, value: vk::PrimitiveTopology) -> Result<(), vk::Result> {
        match value {
            vk::PrimitiveTopology::POINT_LIST
            | vk::PrimitiveTopology::LINE_LIST
            | vk::PrimitiveTopology::TRIANGLE_LIST
            | vk::PrimitiveTopology::TRIANGLE_STRIP => {
                self.set_topology(value);
                Ok(())
            }
            _ => Err(vk::Result::ERROR_FEATURE_NOT_PRESENT),
        }
    }
    pub fn polygon_mode(&self) -> Option<vk::PolygonMode> {
        self.polygon_mode
    }
    pub fn set_polygon_mode(&mut self, value: vk::PolygonMode) {
        self.polygon_mode = Some(value);
    }
    pub fn set_polygon_mode_checked(&mut self, value: vk::PolygonMode) -> Result<(), vk::Result> {
        match value {
            vk::PolygonMode::FILL | vk::PolygonMode::LINE => self.set_polygon_mode(value),
            _ => return Err(vk::Result::ERROR_FEATURE_NOT_PRESENT),
        }
        Ok(())
    }
    pub fn cull_mode(&self) -> Option<vk::CullModeFlags> {
        self.cull_mode
    }
    pub fn cull_mode_or_err(&self) -> Result<vk::CullModeFlags, vk::Result> {
        self.cull_mode.ok_or(vk::Result::ERROR_INITIALIZATION_FAILED)
    }
    pub fn set_cull_mode(&mut self, value: vk::CullModeFlags) {
        self.cull_mode = Some(value);
    }
    pub fn set_cull_mode_checked(&mut self, value: vk::CullModeFlags) -> Result<(), vk::Result> {
        if value.contains(vk::CullModeFlags::NONE)
            || value.contains(vk::CullModeFlags::FRONT)
            || value.contains(vk::CullModeFlags::BACK)
            || value.contains(vk::CullModeFlags::FRONT_AND_BACK)
        {
            self.set_cull_mode(value);
            Ok(())
        } else {
            Err(vk::Result::ERROR_INITIALIZATION_FAILED)
        }
    }
    pub fn depth_compare_mode(&self) -> Option<vk::CompareOp> {
        self.depth_compare
    }
    pub fn depth_compare_mode_or_err(&self) -> Result<vk::CompareOp, vk::Result> {
        self.depth_compare.ok_or(vk::Result::ERROR_INITIALIZATION_FAILED)
    }
    pub fn set_depth_compare_mode(&mut self, value: vk::CompareOp) {
        self.depth_compare = Some(value);
    }
    pub fn set_depth_compare_mode_checked(&mut self, value: vk::CompareOp) -> Result<(), vk::Result> {
        self.set_depth_compare_mode(value);
        Ok(())
    }
    pub fn set_depth_write(&mut self, value: bool) {
        self.depth_write = Some(value);
    }
    pub fn set_depth_write_checked(&mut self, value: bool) -> Result<(), vk::Result> {
        self.set_depth_write(value);
        Ok(())
    }
    pub fn set_depth_test_enable(&mut self, value: bool) {
        self.depth_test = Some(value);
    }
    pub fn set_depth_test_enable_checked(&mut self, value: bool) -> Result<(), vk::Result> {
        self.set_depth_test_enable(value);
        Ok(())
    }
    pub fn set_stencil_test_enable(&mut self, value: bool) {
        self.stencil_test = Some(value);
    }
    pub fn set_stencil_test_enable_checked(&mut self, value: bool) -> Result<(), vk::Result> {
        self.set_stencil_test_enable(value);
        Ok(())
    }
    pub fn set_stencil_pass_op(&mut self, value: vk::StencilOp) {
        self.stencil_pass_op = Some(value);
    }
    pub fn set_stencil_pass_op_checked(&mut self, value: vk::StencilOp) -> Result<(), vk::Result> {
        match value {
            vk::StencilOp::KEEP
            | vk::StencilOp::REPLACE
            | vk::StencilOp::INCREMENT_AND_CLAMP
            | vk::StencilOp::DECREMENT_AND_CLAMP => self.set_stencil_pass_op(value),
            _ => return Err(vk::Result::ERROR_FEATURE_NOT_PRESENT),
        }
        Ok(())
    }
    pub fn set_stencil_compare_op(&mut self, value: vk::CompareOp) {
        self.stencil_compare_op = Some(value);
    }
    pub fn set_stencil_compare_op_checked(&mut self, value: vk::CompareOp) -> Result<(), vk::Result> {
        self.set_stencil_compare_op(value);
        Ok(())
    }
    pub fn set_stencil_compare_mask(&mut self, value: u32) {
        self.stencil_compare_mask = Some(value);
    }
    pub fn set_stencil_compare_mask_checked(&mut self, value: u32) -> Result<(), vk::Result> {
        self.set_stencil_compare_mask(value);
        Ok(())
    }
    /// blend_mode 编码与原 raster 状态 +0x1b 高两位一致；`get_vk_blend_mode` 展开成完整附件。
    pub fn set_blend_mode(&mut self, value: u8) {
        self.blend_mode = Some(value);
    }
    pub fn set_blend_mode_checked(&mut self, value: u8) -> Result<(), vk::Result> {
        self.set_blend_mode(value);
        Ok(())
    }
    /// 合并到 `RasterPipelineDesc`；缺失字段保持 desc 原值。
    pub fn apply_to(&self, desc: &mut RasterPipelineDesc) {
        if let Some(v) = self.topology {
            desc.topology = v;
        }
        if let Some(v) = self.polygon_mode {
            desc.polygon_mode = v;
        }
        if let Some(v) = self.cull_mode {
            desc.cull_mode = v;
        }
        if let Some(v) = self.front_face {
            desc.front_face = v;
        }
        if let Some(v) = self.samples {
            desc.samples = v;
        }
        if let Some(v) = self.depth_test {
            desc.depth_test = v;
        }
        if let Some(v) = self.depth_write {
            desc.depth_write = v;
        }
        if let Some(v) = self.depth_compare {
            desc.depth_compare = v;
        }
        if let Some(blend) = self.blend_mode {
            desc.blend_attachments = vec![get_vk_blend_mode(blend)];
        }
    }
}
pub struct VulkanPipeline {
    pub device: Arc<VulkanDevice>,
    pub handle: vk::Pipeline,
    pub layout: Arc<PipelineLayout>,
    pub point: vk::PipelineBindPoint,
    shaders: Vec<Arc<ShaderModule>>,
    render_pass_owner: Option<ResourceOwner>,
}
impl VulkanPipeline {
    pub fn create_compute_pipeline(
        cache: &PipelineCache,
        desc: ComputePipelineDesc,
    ) -> Result<Arc<Self>, vk::Result> {
        if !Arc::ptr_eq(&cache.device, &desc.layout.device)
            || !Arc::ptr_eq(&cache.device, &desc.shader.device)
        {
            return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
        }
        use ash::vk::Handle;
        let key = (
            desc.shader.handle.as_raw(),
            desc.layout.handle.as_raw(),
            desc.entry.as_bytes().to_vec(),
        );
        let mut candidates = cache.compute.lock().unwrap_or_else(|p| p.into_inner());
        if let Some(pipeline) = candidates.get(&key) {
            return Ok(pipeline.clone());
        }
        let _lock = cache.lock.lock().unwrap_or_else(|p| p.into_inner());
        let stage = vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::COMPUTE)
            .module(desc.shader.handle)
            .name(&desc.entry);
        let info = vk::ComputePipelineCreateInfo::default()
            .stage(stage)
            .layout(desc.layout.handle);
        let result = unsafe {
            cache
                .device
                .device
                .create_compute_pipelines(cache.handle, &[info], None)
        };
        let handle = finish_pipeline(&cache.device, result)?;
        let pipeline = Arc::new(Self {
            device: cache.device.clone(),
            handle,
            layout: desc.layout,
            point: vk::PipelineBindPoint::COMPUTE,
            shaders: vec![desc.shader],
            render_pass_owner: None,
        });
        candidates.insert(key, pipeline.clone());
        Ok(pipeline)
    }
    pub fn create_raster_pipeline(
        cache: &PipelineCache,
        desc: RasterPipelineDesc,
    ) -> Result<Arc<Self>, vk::Result> {
        if !Arc::ptr_eq(&cache.device, &desc.layout.device)
            || !Arc::ptr_eq(&cache.device, &desc.vertex.device)
            || desc
                .fragment
                .as_ref()
                .is_some_and(|s| !Arc::ptr_eq(&cache.device, &s.device))
        {
            return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
        }
        let key = raster_pipeline_key(&desc);
        let mut candidates = cache.raster.lock().unwrap_or_else(|p| p.into_inner());
        if let Some(pipeline) = candidates.get(&key) {
            return Ok(pipeline.clone());
        }
        let _lock = cache.lock.lock().unwrap_or_else(|p| p.into_inner());
        let mut stages = vec![stage(
            desc.vertex.handle,
            vk::ShaderStageFlags::VERTEX,
            &desc.entry,
        )];
        let mut shaders = vec![desc.vertex.clone()];
        if let Some(fragment) = desc.fragment {
            stages.push(stage(
                fragment.handle,
                vk::ShaderStageFlags::FRAGMENT,
                desc.fragment_entry.as_deref().unwrap_or(&desc.entry),
            ));
            shaders.push(fragment);
        }
        let vertex = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(&desc.vertex_bindings)
            .vertex_attribute_descriptions(&desc.vertex_attributes);
        let assembly = vk::PipelineInputAssemblyStateCreateInfo::default().topology(desc.topology);
        let viewport = vk::PipelineViewportStateCreateInfo::default()
            .viewport_count(1)
            .scissor_count(1);
        let raster = vk::PipelineRasterizationStateCreateInfo::default()
            .polygon_mode(desc.polygon_mode)
            .cull_mode(desc.cull_mode)
            .front_face(desc.front_face)
            .line_width(1.0);
        let multisample =
            vk::PipelineMultisampleStateCreateInfo::default().rasterization_samples(desc.samples);
        let depth = vk::PipelineDepthStencilStateCreateInfo::default()
            .depth_test_enable(desc.depth_test)
            .depth_write_enable(desc.depth_write)
            .depth_compare_op(desc.depth_compare);
        let blend =
            vk::PipelineColorBlendStateCreateInfo::default().attachments(&desc.blend_attachments);
        let dynamic_values = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
        let dynamic = vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic_values);
        let info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&stages)
            .vertex_input_state(&vertex)
            .input_assembly_state(&assembly)
            .viewport_state(&viewport)
            .rasterization_state(&raster)
            .multisample_state(&multisample)
            .depth_stencil_state(&depth)
            .color_blend_state(&blend)
            .dynamic_state(&dynamic)
            .layout(desc.layout.handle)
            .render_pass(desc.render_pass)
            .subpass(desc.subpass);
        let result = unsafe {
            cache
                .device
                .device
                .create_graphics_pipelines(cache.handle, &[info], None)
        };
        let handle = finish_pipeline(&cache.device, result)?;
        let pipeline = Arc::new(Self {
            device: cache.device.clone(),
            handle,
            layout: desc.layout,
            point: vk::PipelineBindPoint::GRAPHICS,
            shaders,
            render_pass_owner: Some(desc.render_pass_owner),
        });
        candidates.insert(key, pipeline.clone());
        Ok(pipeline)
    }
    /// # Safety
    /// command 必须处于正确录制状态，上层提交链应保持本对象至 GPU 完成。
    pub unsafe fn bind_pipeline(&self, command: vk::CommandBuffer) {
        unsafe {
            self.device
                .device
                .cmd_bind_pipeline(command, self.point, self.handle)
        }
    }
    pub fn shader_count(&self) -> usize {
        self.shaders.len()
    }
    pub fn retains_render_pass(&self) -> bool {
        self.render_pass_owner.is_some()
    }
}
fn stage<'a>(
    module: vk::ShaderModule,
    kind: vk::ShaderStageFlags,
    entry: &'a CStr,
) -> vk::PipelineShaderStageCreateInfo<'a> {
    vk::PipelineShaderStageCreateInfo::default()
        .module(module)
        .stage(kind)
        .name(entry)
}
fn finish_pipeline(
    device: &VulkanDevice,
    result: Result<Vec<vk::Pipeline>, (Vec<vk::Pipeline>, vk::Result)>,
) -> Result<vk::Pipeline, vk::Result> {
    match result {
        Ok(values) => values
            .first()
            .copied()
            .ok_or(vk::Result::ERROR_INITIALIZATION_FAILED),
        Err((values, error)) => {
            for handle in values {
                unsafe { device.device.destroy_pipeline(handle, None) }
            }
            Err(error)
        }
    }
}
/// 键覆盖本候选 RasterPipelineDesc 所有会影响 Vulkan 管线的字段；不使用不稳定 Debug 字串或哈希碰撞替代相等性。
fn raster_pipeline_key(desc: &RasterPipelineDesc) -> Vec<u64> {
    use ash::vk::Handle;
    let mut key = vec![
        desc.vertex.handle.as_raw(),
        desc.fragment
            .as_ref()
            .map_or(0, |shader| shader.handle.as_raw()),
        desc.layout.handle.as_raw(),
        desc.render_pass.as_raw(),
        u64::from(desc.subpass),
        desc.topology.as_raw() as u64,
        desc.polygon_mode.as_raw() as u64,
        u64::from(desc.cull_mode.as_raw()),
        desc.front_face.as_raw() as u64,
        u64::from(desc.samples.as_raw()),
        u64::from(desc.depth_test),
        u64::from(desc.depth_write),
        desc.depth_compare.as_raw() as u64,
        desc.entry.as_bytes().len() as u64,
    ];
    key.extend(desc.entry.as_bytes().iter().map(|byte| u64::from(*byte)));
    let fragment_entry = desc.fragment_entry.as_deref().unwrap_or(&desc.entry);
    key.push(fragment_entry.to_bytes().len() as u64);
    key.extend(
        fragment_entry
            .to_bytes()
            .iter()
            .map(|byte| u64::from(*byte)),
    );
    key.push(desc.vertex_bindings.len() as u64);
    for binding in &desc.vertex_bindings {
        key.extend([
            u64::from(binding.binding),
            u64::from(binding.stride),
            binding.input_rate.as_raw() as u64,
        ]);
    }
    key.push(desc.vertex_attributes.len() as u64);
    for attribute in &desc.vertex_attributes {
        key.extend([
            u64::from(attribute.location),
            u64::from(attribute.binding),
            attribute.format.as_raw() as u64,
            u64::from(attribute.offset),
        ]);
    }
    key.push(desc.blend_attachments.len() as u64);
    for blend in &desc.blend_attachments {
        key.extend([
            u64::from(blend.blend_enable),
            blend.src_color_blend_factor.as_raw() as u64,
            blend.dst_color_blend_factor.as_raw() as u64,
            blend.color_blend_op.as_raw() as u64,
            blend.src_alpha_blend_factor.as_raw() as u64,
            blend.dst_alpha_blend_factor.as_raw() as u64,
            blend.alpha_blend_op.as_raw() as u64,
            u64::from(blend.color_write_mask.as_raw()),
        ]);
    }
    key
}
impl Drop for VulkanPipeline {
    fn drop(&mut self) {
        unsafe {
            let _ = self.device.wait_idle();
            self.device.device.destroy_pipeline(self.handle, None)
        }
    }
}

/// 原 spirq 分叉生成的已保存反射条目；通用 SPIR-V 解析不在本模块重写。
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReflectedEntry {
    pub entry_point: String,
    pub stage: String,
    pub descriptors: Vec<ReflectedDescriptor>,
    #[serde(default)]
    pub push_constants: Vec<ReflectedPushConstant>,
    #[serde(default)]
    pub inputs: Vec<ReflectedLocation>,
    #[serde(default)]
    pub outputs: Vec<ReflectedLocation>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReflectedLocation {
    pub location: u32,
    pub component: u32,
    #[serde(rename = "type")]
    pub reflected_type: serde_json::Value,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReflectedDescriptor {
    pub set: u32,
    pub binding: u32,
    pub name: String,
    pub count: u32,
    pub descriptor_type: String,
    #[serde(default, rename = "type")]
    pub reflected_type: serde_json::Value,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReflectedPushConstant {
    #[serde(rename = "type")]
    pub reflected_type: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InterfaceError {
    Stage(String),
    Descriptor(String),
    IncompatibleBinding(u32, u32),
    MissingSampler(String),
    InvalidPushConstant,
    RuntimeBindingNotLast(u32, u32),
    InvalidLayout,
    IncompatibleLocation(u32, u32),
}
impl std::fmt::Display for InterfaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "着色器接口无效：{self:?}")
    }
}
impl std::error::Error for InterfaceError {}

pub struct ShaderInterfacePlan {
    /// 保留原反射 set/binding 编号；缺失的 set 用空布局填充，绝不重新枚举 sparse binding。
    pub sets: Vec<LayoutPlan>,
    pub push_constants: Vec<vk::PushConstantRange>,
}
fn reflected_stage(stage: &str) -> Result<vk::ShaderStageFlags, InterfaceError> {
    match stage {
        "GLCompute" => Ok(vk::ShaderStageFlags::COMPUTE),
        "Vertex" => Ok(vk::ShaderStageFlags::VERTEX),
        "Fragment" => Ok(vk::ShaderStageFlags::FRAGMENT),
        other => Err(InterfaceError::Stage(other.to_owned())),
    }
}
impl ShaderInterfacePlan {
    pub fn from_reflection(
        entries: &[ReflectedEntry],
        immutable_samplers: &BTreeMap<String, u64>,
        runtime_device_scalar: u32,
    ) -> Result<Self, InterfaceError> {
        if let (Some(vertex), Some(fragment)) = (
            entries.iter().find(|entry| entry.stage == "Vertex"),
            entries.iter().find(|entry| entry.stage == "Fragment"),
        ) {
            for input in &fragment.inputs {
                let output = vertex
                    .outputs
                    .iter()
                    .find(|output| {
                        output.location == input.location && output.component == input.component
                    })
                    .ok_or(InterfaceError::IncompatibleLocation(
                        input.location,
                        input.component,
                    ))?;
                if canonical_type(&input.reflected_type) != canonical_type(&output.reflected_type) {
                    return Err(InterfaceError::IncompatibleLocation(
                        input.location,
                        input.component,
                    ));
                }
            }
        }
        let mut merged = BTreeMap::<(u32, u32), (Binding, vk::ShaderStageFlags)>::new();
        let mut pushes = BTreeMap::<(u32, u32), vk::ShaderStageFlags>::new();
        for entry in entries {
            let stage = reflected_stage(&entry.stage)?;
            for descriptor in &entry.descriptors {
                let tag = match descriptor.descriptor_type.as_str() {
                    "StorageBuffer(ReadWrite)" | "StorageBuffer(ReadOnly)" => 0,
                    "UniformBuffer" => 1,
                    "CombinedImageSampler" => 2,
                    "StorageImage(ReadWrite)" | "StorageImage(ReadOnly)" => 3,
                    "SampledImage" => 4,
                    "Sampler" => 5,
                    other => return Err(InterfaceError::Descriptor(other.to_owned())),
                };
                let sampler = if tag == 5 {
                    *immutable_samplers
                        .get(&descriptor.name)
                        .filter(|v| **v != 0)
                        .ok_or_else(|| InterfaceError::MissingSampler(descriptor.name.clone()))?
                } else {
                    0
                };
                let value = Binding {
                    tag,
                    name: descriptor.name.clone(),
                    count: descriptor.count,
                    runtime: descriptor.count == 0,
                    sampler,
                };
                match merged.entry((descriptor.set, descriptor.binding)) {
                    std::collections::btree_map::Entry::Vacant(slot) => {
                        slot.insert((value, stage));
                    }
                    std::collections::btree_map::Entry::Occupied(mut slot) => {
                        let (prior, stages) = slot.get_mut();
                        // 名称仅在 _dyn 分类与 immutable sampler 策略中影响布局；跨 stage 别名允许合并。
                        if prior.tag != value.tag
                            || prior.count != value.count
                            || prior.sampler != value.sampler
                            || prior.name.ends_with("_dyn") != value.name.ends_with("_dyn")
                        {
                            return Err(InterfaceError::IncompatibleBinding(
                                descriptor.set,
                                descriptor.binding,
                            ));
                        }
                        *stages |= stage;
                    }
                }
            }
            for push in &entry.push_constants {
                let size = push.reflected_type["minimum_bytes"]
                    .as_u64()
                    .and_then(|size| u32::try_from(size).ok())
                    .filter(|size| *size > 0 && *size % 4 == 0)
                    .ok_or(InterfaceError::InvalidPushConstant)?;
                // 当前原 catalog 的 push block 始于 0；大小来自原 fork 反射，而非猜测成员数量。
                *pushes.entry((0, size)).or_default() |= stage;
            }
        }
        let set_count = merged
            .keys()
            .map(|(set, _)| *set)
            .max()
            .map_or(0, |set| set as usize + 1);
        if set_count > 32 {
            return Err(InterfaceError::InvalidLayout);
        }
        let mut sets = (0..set_count)
            .map(|_| LayoutPlan {
                flags: 0,
                bindings: Vec::new(),
                binding_flags: Vec::new(),
            })
            .collect::<Vec<_>>();
        for ((set, binding), (value, stages)) in merged {
            let plan =
                descriptor_set_layout(&[value], u64::from(binding), stages, runtime_device_scalar)
                    .map_err(|_| InterfaceError::InvalidLayout)?;
            let target = &mut sets[set as usize];
            target.flags |= plan.flags;
            target.bindings.extend(plan.bindings);
            target.binding_flags.extend(plan.binding_flags);
        }
        for (set, plan) in sets.iter().enumerate() {
            for (index, flags) in plan.binding_flags.iter().enumerate() {
                if flags & vk::DescriptorBindingFlags::VARIABLE_DESCRIPTOR_COUNT.as_raw() != 0
                    && index + 1 != plan.bindings.len()
                {
                    return Err(InterfaceError::RuntimeBindingNotLast(
                        set as u32,
                        plan.bindings[index].binding,
                    ));
                }
            }
        }
        // Vulkan 不允许同一 stage 在多个 push 范围中出现；合并该 stage 的最大范围。
        let mut by_stage = BTreeMap::<u32, u32>::new();
        for ((_, size), flags) in pushes {
            for stage in [
                vk::ShaderStageFlags::VERTEX,
                vk::ShaderStageFlags::FRAGMENT,
                vk::ShaderStageFlags::COMPUTE,
            ] {
                if flags.contains(stage) {
                    by_stage
                        .entry(stage.as_raw())
                        .and_modify(|s| *s = (*s).max(size))
                        .or_insert(size);
                }
            }
        }
        let mut grouped = BTreeMap::<u32, vk::ShaderStageFlags>::new();
        for (stage, size) in by_stage {
            *grouped.entry(size).or_default() |= vk::ShaderStageFlags::from_raw(stage);
        }
        Ok(Self {
            sets,
            push_constants: grouped
                .into_iter()
                .map(|(size, stage_flags)| vk::PushConstantRange {
                    stage_flags,
                    offset: 0,
                    size,
                })
                .collect(),
        })
    }
    pub fn create_descriptor_set_layouts(
        &self,
        device: Arc<VulkanDevice>,
        samplers: &[Arc<crate::raw::sampler::Sampler>],
    ) -> Result<Vec<Arc<VulkanDescriptorSetLayout>>, vk::Result> {
        self.sets
            .iter()
            .map(|plan| {
                VulkanDescriptorSetLayout::new_with_samplers(
                    device.clone(),
                    plan,
                    samplers.to_vec(),
                )
            })
            .collect()
    }
    pub fn create_descriptor_set_states(
        layouts: &[Arc<VulkanDescriptorSetLayout>],
        sets_per_pool: u32,
    ) -> Result<Vec<DescriptorSetState>, vk::Result> {
        layouts
            .iter()
            .map(|layout| DescriptorSetState::new(layout.clone(), sets_per_pool))
            .collect()
    }
    pub fn create_pipeline_layout(
        &self,
        device: Arc<VulkanDevice>,
        samplers: &[Arc<crate::raw::sampler::Sampler>],
    ) -> Result<Arc<PipelineLayout>, vk::Result> {
        let layouts = self.create_descriptor_set_layouts(device.clone(), samplers)?;
        PipelineLayout::new(device, layouts, self.push_constants.clone())
    }
}
/// 去掉反射调试名字后比较结构；仅处理已有 schema，不解析 SPIR-V 或重写公开反射库。
fn canonical_type(value: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(fields) => serde_json::Value::Object(
            fields
                .iter()
                .filter(|(key, _)| key.as_str() != "name")
                .map(|(key, child)| (key.clone(), canonical_type(child)))
                .collect(),
        ),
        serde_json::Value::Array(values) => {
            serde_json::Value::Array(values.iter().map(canonical_type).collect())
        }
        other => other.clone(),
    }
}

/// 原函数读取 raster 状态偏移 0x1b 的高两位；枚举名称未知，因此保留编码接口。
/// 表值与写入的完整 VkPipelineColorBlendAttachmentState 已按原机器码确认。
pub fn get_vk_blend_mode(encoded_raster_flags: u8) -> vk::PipelineColorBlendAttachmentState {
    let mode = usize::from(encoded_raster_flags >> 5 & 3);
    let enabled = [0, 1, 1, 1][mode];
    let dst_color = [0, 1, 7, 16][mode];
    let dst_alpha = [0, 1, 7, 18][mode];
    vk::PipelineColorBlendAttachmentState::default()
        .blend_enable(enabled != 0)
        .src_color_blend_factor(vk::BlendFactor::ONE)
        .dst_color_blend_factor(vk::BlendFactor::from_raw(dst_color))
        .color_blend_op(vk::BlendOp::ADD)
        .src_alpha_blend_factor(vk::BlendFactor::ONE)
        .dst_alpha_blend_factor(vk::BlendFactor::from_raw(dst_alpha))
        .alpha_blend_op(vk::BlendOp::ADD)
        .color_write_mask(vk::ColorComponentFlags::RGBA)
}
/// 原类型 tag 2/3/4 对应 32 位有符号/无符号/浮点，字节宽度仅允许 4/8/12/16。
pub fn map_vertex_attribute_format(
    type_tag: u32,
    bytes: u64,
) -> Result<vk::Format, InterfaceError> {
    let base = match type_tag {
        2 => 99,
        3 => 98,
        4 => 100,
        _ => return Err(InterfaceError::InvalidLayout),
    };
    if !(4..=16).contains(&bytes) || bytes % 4 != 0 {
        return Err(InterfaceError::InvalidLayout);
    }
    Ok(vk::Format::from_raw(base + 3 * (bytes as i32 / 4 - 1)))
}

/// 已解析发行容器中的一个模块。容器 offset/word_count 来自原 spirq catalog。
#[derive(Clone)]
pub struct ReflectedShader {
    pub module: Arc<ShaderModule>,
    pub entry: ReflectedEntry,
}
impl ReflectedShader {
    pub fn from_compiled_container(
        device: Arc<VulkanDevice>,
        container: &[u8],
        metadata: &serde_json::Value,
        entry_name: &str,
    ) -> Result<Self, vk::Result> {
        let offset = metadata["offset"]
            .as_u64()
            .and_then(|v| usize::try_from(v).ok())
            .ok_or(vk::Result::ERROR_INVALID_SHADER_NV)?;
        let len = metadata["word_count"]
            .as_u64()
            .and_then(|v| usize::try_from(v).ok())
            .and_then(|v| v.checked_mul(4))
            .ok_or(vk::Result::ERROR_INVALID_SHADER_NV)?;
        let payload = container
            .get(
                offset
                    ..offset
                        .checked_add(len)
                        .ok_or(vk::Result::ERROR_INVALID_SHADER_NV)?,
            )
            .ok_or(vk::Result::ERROR_INVALID_SHADER_NV)?;
        let entry: ReflectedEntry = serde_json::from_value(
            metadata["entries"]
                .as_array()
                .and_then(|entries| {
                    entries
                        .iter()
                        .find(|entry| entry["entry_point"] == entry_name)
                })
                .cloned()
                .ok_or(vk::Result::ERROR_INVALID_SHADER_NV)?,
        )
        .map_err(|_| vk::Result::ERROR_INVALID_SHADER_NV)?;
        let parsed =
            rspirv::dr::load_bytes(payload).map_err(|_| vk::Result::ERROR_INVALID_SHADER_NV)?;
        let expected = match entry.stage.as_str() {
            "GLCompute" => rspirv::spirv::ExecutionModel::GLCompute,
            "Vertex" => rspirv::spirv::ExecutionModel::Vertex,
            "Fragment" => rspirv::spirv::ExecutionModel::Fragment,
            _ => return Err(vk::Result::ERROR_INVALID_SHADER_NV),
        };
        let present = parsed.entry_points.iter().any(|point| matches!((&point.operands[0], &point.operands[2]), (rspirv::dr::Operand::ExecutionModel(stage), rspirv::dr::Operand::LiteralString(name)) if *stage == expected && name == entry_name));
        if !present {
            return Err(vk::Result::ERROR_INVALID_SHADER_NV);
        }
        Ok(Self {
            module: ShaderModule::from_spirv(device, payload)?,
            entry,
        })
    }
}

/// 按资产键选择真实 shader，自动生产布局与计算管线；不假定文件名就是 country-core pass 键。
pub struct ShaderLibrary {
    pub cache: PipelineCache,
    shaders: BTreeMap<String, ReflectedShader>,
    layouts: BTreeMap<String, Arc<PipelineLayout>>,
}
pub struct RasterProgram {
    pub vertex: ReflectedShader,
    pub fragment: ReflectedShader,
    pub layout: Arc<PipelineLayout>,
}
impl RasterProgram {
    /// 图形固定状态仍由材质/附件消费者提供；shader/entry/layout 则由资产选择生产。
    pub fn pipeline_desc(
        &self,
        mut state: RasterPipelineDesc,
    ) -> Result<RasterPipelineDesc, vk::Result> {
        state.vertex = self.vertex.module.clone();
        state.fragment = Some(self.fragment.module.clone());
        state.layout = self.layout.clone();
        state.entry = CString::new(self.vertex.entry.entry_point.as_str())
            .map_err(|_| vk::Result::ERROR_INVALID_SHADER_NV)?;
        state.fragment_entry = Some(
            CString::new(self.fragment.entry.entry_point.as_str())
                .map_err(|_| vk::Result::ERROR_INVALID_SHADER_NV)?,
        );
        Ok(state)
    }
}
impl ShaderLibrary {
    pub fn new(device: Arc<VulkanDevice>) -> Result<Self, vk::Result> {
        Ok(Self {
            cache: PipelineCache::new(device, &[])?,
            shaders: BTreeMap::new(),
            layouts: BTreeMap::new(),
        })
    }
    pub fn register(&mut self, key: String, shader: ReflectedShader) -> Result<(), vk::Result> {
        if !Arc::ptr_eq(&self.cache.device, &shader.module.device) {
            return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
        }
        self.layouts.clear();
        self.cache.invalidate();
        self.shaders.insert(key, shader);
        Ok(())
    }
    pub fn create_compute_pipeline(
        &mut self,
        key: &str,
        samplers: &BTreeMap<String, Arc<crate::raw::sampler::Sampler>>,
    ) -> Result<Arc<VulkanPipeline>, vk::Result> {
        let runtime_device_scalar = self
            .cache
            .device
            .max_descriptor_set_update_after_bind_sampled_images();
        use ash::vk::Handle;
        if samplers
            .values()
            .any(|sampler| !Arc::ptr_eq(&self.cache.device, &sampler.device))
        {
            return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
        }
        let shader = self
            .shaders
            .get(key)
            .ok_or(vk::Result::ERROR_INVALID_SHADER_NV)?;
        if shader.entry.stage != "GLCompute" {
            return Err(vk::Result::ERROR_INVALID_SHADER_NV);
        }
        // sampler 句柄属于布局键；修改 sampler 配置不能复用旧 layout。
        let sampler_map = samplers
            .iter()
            .map(|(name, sampler)| (name.clone(), sampler.handle.as_raw()))
            .collect::<BTreeMap<_, _>>();
        let mut by_name = BTreeMap::new();
        for descriptor in &shader.entry.descriptors {
            if descriptor.descriptor_type == "Sampler" {
                let handle = sampler_map
                    .get(&descriptor.name)
                    .copied()
                    .ok_or(vk::Result::ERROR_INITIALIZATION_FAILED)?;
                by_name.insert(descriptor.name.clone(), handle);
            }
        }
        let plan = ShaderInterfacePlan::from_reflection(
            std::slice::from_ref(&shader.entry),
            &by_name,
            runtime_device_scalar,
        )
        .map_err(|_| vk::Result::ERROR_INITIALIZATION_FAILED)?;
        let layout_key = serde_json::to_string(&(key, &plan.sets, runtime_device_scalar)).unwrap();
        let layout = match self.layouts.get(&layout_key) {
            Some(layout) => layout.clone(),
            None => {
                let retained = samplers.values().cloned().collect::<Vec<_>>();
                let layout = plan.create_pipeline_layout(self.cache.device.clone(), &retained)?;
                self.layouts.insert(layout_key, layout.clone());
                layout
            }
        };
        VulkanPipeline::create_compute_pipeline(
            &self.cache,
            ComputePipelineDesc {
                shader: shader.module.clone(),
                entry: CString::new(shader.entry.entry_point.as_str())
                    .map_err(|_| vk::Result::ERROR_INVALID_SHADER_NV)?,
                layout,
            },
        )
    }
    pub fn invalidate(&mut self, key: &str) {
        self.shaders.remove(key);
        self.layouts.clear();
        self.cache.invalidate();
    }
    pub fn prepare_raster_program(
        &mut self,
        vertex_key: &str,
        fragment_key: &str,
        samplers: &BTreeMap<String, Arc<crate::raw::sampler::Sampler>>,
    ) -> Result<RasterProgram, vk::Result> {
        let runtime_device_scalar = self
            .cache
            .device
            .max_descriptor_set_update_after_bind_sampled_images();
        use ash::vk::Handle;
        if samplers
            .values()
            .any(|sampler| !Arc::ptr_eq(&self.cache.device, &sampler.device))
        {
            return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
        }
        let vertex = self
            .shaders
            .get(vertex_key)
            .cloned()
            .ok_or(vk::Result::ERROR_INVALID_SHADER_NV)?;
        let fragment = self
            .shaders
            .get(fragment_key)
            .cloned()
            .ok_or(vk::Result::ERROR_INVALID_SHADER_NV)?;
        if vertex.entry.stage != "Vertex" || fragment.entry.stage != "Fragment" {
            return Err(vk::Result::ERROR_INVALID_SHADER_NV);
        }
        let sampler_map = samplers
            .iter()
            .map(|(name, sampler)| (name.clone(), sampler.handle.as_raw()))
            .collect();
        let plan = ShaderInterfacePlan::from_reflection(
            &[vertex.entry.clone(), fragment.entry.clone()],
            &sampler_map,
            runtime_device_scalar,
        )
        .map_err(|_| vk::Result::ERROR_INITIALIZATION_FAILED)?;
        let key =
            serde_json::to_string(&(vertex_key, fragment_key, &plan.sets, runtime_device_scalar))
                .unwrap();
        let layout = match self.layouts.get(&key) {
            Some(layout) => layout.clone(),
            None => {
                let retained = samplers.values().cloned().collect::<Vec<_>>();
                let layout = plan.create_pipeline_layout(self.cache.device.clone(), &retained)?;
                self.layouts.insert(key, layout.clone());
                layout
            }
        };
        Ok(RasterProgram {
            vertex,
            fragment,
            layout,
        })
    }
}
