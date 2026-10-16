//! shader 资产选择、原容器存储反射布局生产、管线与缓存的可运行候选。
//! SPIR-V 解析复用原锁 rspirv；反射 schema 复用原 spirq fork 已保存目录。
//! 原资产键到 country-core pass 的关联、完整原缓存策略与 ABI 仍未全部对齐。
use crate::raw::binding::ResourceOwner;
use crate::raw::descriptor_set::VulkanDescriptorSetLayout;
use crate::raw::descriptor_set::{Binding, DescriptorSetState, LayoutPlan, descriptor_set_layout};
use crate::raw::device::VulkanDevice;
use ash::vk;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    ffi::{CStr, CString},
    io::Cursor,
    sync::Arc,
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
        self.device
            .bury(super::graveyard::Doomed::PipelineLayout(self.handle));
    }
}

/// Vulkan 驱动缓存。`VkPipelineCache` 未带 EXTERNALLY_SYNCHRONIZED 标志创建，规范规定其内部同步，
/// 所以不同键的管线可以并行创建；`lock` 只保护 `get_pipeline_cache_data` 的读取。
///
/// 每个键一个槽位：map 只在查找/插入槽位时短暂持锁，创建在槽位锁内进行——同键的并发调用者
/// 排队等待同一次创建（去重），不同键互不阻塞；创建失败不缓存，下次调用重试。
type ComputePipelineKey = (u64, u64, Vec<u8>);
type PipelineSlot = Arc<Mutex<Option<Arc<VulkanPipeline>>>>;
pub struct PipelineCache {
    pub device: Arc<VulkanDevice>,
    handle: vk::PipelineCache,
    lock: Mutex<()>,
    compute: Mutex<BTreeMap<ComputePipelineKey, PipelineSlot>>,
    raster: Mutex<BTreeMap<Vec<u64>, PipelineSlot>>,
}
/// 取（或插入）键对应的槽位；map 锁在返回前释放。
fn slot_for<K: Ord>(map: &Mutex<BTreeMap<K, PipelineSlot>>, key: K) -> PipelineSlot {
    map.lock().entry(key).or_default().clone()
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
        let _lock = self.lock.lock();
        unsafe { self.device.device.get_pipeline_cache_data(self.handle) }
    }
    /// 管线缓存持有布局与 shader；显式失效后已有提交仍由外部 Arc 保持有效。
    pub fn invalidate(&self) {
        self.compute.lock().clear();
        self.raster.lock().clear();
    }
    pub fn cached_compute_count(&self) -> usize {
        self.compute
            .lock()
            .values()
            .filter(|slot| slot.lock().is_some())
            .count()
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
    /// 模板测试；front/back 共用 `stencil`（原 0x1416a3720 两份同值）。
    pub stencil_test: bool,
    pub stencil: vk::StencilOpState,
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
            stencil_test: false,
            stencil: vk::StencilOpState::default(),
            blend_attachments: Vec::new(),
        }
    }
}

/// 原 `RasterPipelineOpts`：4 字节位域，同时是 `RasterPipelineKey.selector`（+0x18）。
///
/// 位分配读自 `create_raster_pipeline`（O 0x1416a3720，0x1416a3d08..0x1416a3f2f）和混合表函数
/// 0x140be67f0（B patched）；本轮对照机器码逐位重核（跳转表 0x142c88ff0 / 0x142c89000 逐项解出）：
///
/// | 字节 | 位 | 含义（原机器码里的用途） |
/// |---|---|---|
/// | b0 | 0–1 | 图元拓扑：0 三角列表 / 1 线列表 / 2 线条带（跳转表 0x142c88ff0，3 不可达） |
/// | b0 | 2–3 | 剔除模式（直接作 `VkCullModeFlags`） |
/// | b0 | 4–5 | 深度比较：0 LESS / 1 GREATER / 2 EQUAL（跳转表 0x142c89000，3 不可达） |
/// | b0 | 6 | 深度写 |
/// | b0 | 7 | 深度测试 |
/// | b1 | 0 | 模板测试 |
/// | b1 | 1–3 | 模板 fail op |
/// | b1 | 4–6 | 模板 pass op |
/// | b1 bit7 + b2 bit0–1 | 3 位 | 模板 depth-fail op（`b1>>7 + (b2&3)*2`） |
/// | b2 | 2–4 | 模板 compare op |
/// | b2 bit5–7 + b3 bit0–4 | 8 位 | 模板 compare mask（`(b3<<3 | b2>>5) & 0xff`） |
/// | b3 | 5–6 | 混合模式（`get_vk_blend_mode`） |
/// | b3 | 7 | 多边形模式：1 = LINE |
///
/// 模板 write mask 恒 `0xffffffff`、reference 0（动态，`STENCIL_REFERENCE`），front/back 同值。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct RasterPipelineOpts(pub u32);
impl RasterPipelineOpts {
    pub fn new() -> Self {
        Self(0)
    }
    fn byte(self, index: u32) -> u32 {
        (self.0 >> (index * 8)) & 0xff
    }
    fn set_bits(&mut self, shift: u32, width: u32, value: u32) {
        let mask = ((1u32 << width) - 1) << shift;
        self.0 = (self.0 & !mask) | ((value << shift) & mask);
    }
    pub fn topology(self) -> vk::PrimitiveTopology {
        match self.byte(0) & 3 {
            0 => vk::PrimitiveTopology::TRIANGLE_LIST,
            1 => vk::PrimitiveTopology::LINE_LIST,
            2 => vk::PrimitiveTopology::LINE_STRIP,
            // 原跳转表 case 3 不可达（`b1 b7 a1 fe` 指向 unreachable 块）。
            _ => unreachable!("RasterPipelineOpts topology value 3"),
        }
    }
    pub fn set_topology(&mut self, value: vk::PrimitiveTopology) {
        let code = match value {
            vk::PrimitiveTopology::TRIANGLE_LIST => 0,
            vk::PrimitiveTopology::LINE_LIST => 1,
            vk::PrimitiveTopology::LINE_STRIP => 2,
            other => panic!("RasterPipelineOpts 不支持拓扑 {other:?}"),
        };
        self.set_bits(0, 2, code);
    }
    pub fn cull_mode(self) -> vk::CullModeFlags {
        vk::CullModeFlags::from_raw((self.byte(0) >> 2) & 3)
    }
    pub fn set_cull_mode(&mut self, value: vk::CullModeFlags) {
        self.set_bits(2, 2, value.as_raw());
    }
    pub fn depth_compare_op(self) -> vk::CompareOp {
        match (self.byte(0) >> 4) & 3 {
            0 => vk::CompareOp::LESS,
            1 => vk::CompareOp::GREATER,
            2 => vk::CompareOp::EQUAL,
            _ => unreachable!("RasterPipelineOpts depth compare value 3"),
        }
    }
    pub fn set_depth_compare_op(&mut self, value: vk::CompareOp) {
        let code = match value {
            vk::CompareOp::LESS => 0,
            vk::CompareOp::GREATER => 1,
            vk::CompareOp::EQUAL => 2,
            other => panic!("RasterPipelineOpts 不支持深度比较 {other:?}"),
        };
        self.set_bits(4, 2, code);
    }
    pub fn depth_write(self) -> bool {
        (self.byte(0) >> 6) & 1 != 0
    }
    pub fn set_depth_write(&mut self, value: bool) {
        self.set_bits(6, 1, u32::from(value));
    }
    pub fn depth_test(self) -> bool {
        (self.byte(0) >> 7) & 1 != 0
    }
    pub fn set_depth_test_enable(&mut self, value: bool) {
        self.set_bits(7, 1, u32::from(value));
    }
    pub fn stencil_test(self) -> bool {
        self.byte(1) & 1 != 0
    }
    pub fn set_stencil_test_enable(&mut self, value: bool) {
        self.set_bits(8, 1, u32::from(value));
    }
    pub fn stencil_fail_op(self) -> vk::StencilOp {
        vk::StencilOp::from_raw(((self.byte(1) >> 1) & 7) as i32)
    }
    pub fn set_stencil_fail_op(&mut self, value: vk::StencilOp) {
        self.set_bits(9, 3, value.as_raw() as u32);
    }
    pub fn stencil_pass_op(self) -> vk::StencilOp {
        vk::StencilOp::from_raw(((self.byte(1) >> 4) & 7) as i32)
    }
    pub fn set_stencil_pass_op(&mut self, value: vk::StencilOp) {
        self.set_bits(12, 3, value.as_raw() as u32);
    }
    /// 3 位横跨 b1 bit7（低位）与 b2 bit0–1（高两位）。
    pub fn stencil_depth_fail_op(self) -> vk::StencilOp {
        vk::StencilOp::from_raw(((self.byte(1) >> 7) + (self.byte(2) & 3) * 2) as i32)
    }
    pub fn set_stencil_depth_fail_op(&mut self, value: vk::StencilOp) {
        let raw = value.as_raw() as u32;
        self.set_bits(15, 1, raw & 1);
        self.set_bits(16, 2, raw >> 1);
    }
    pub fn stencil_compare_op(self) -> vk::CompareOp {
        vk::CompareOp::from_raw(((self.byte(2) >> 2) & 7) as i32)
    }
    pub fn set_stencil_compare_op(&mut self, value: vk::CompareOp) {
        self.set_bits(18, 3, value.as_raw() as u32);
    }
    /// 8 位横跨 b2 bit5–7（低 3 位）与 b3 bit0–4（高 5 位）。
    pub fn stencil_compare_mask(self) -> u32 {
        ((self.byte(3) << 3) | (self.byte(2) >> 5)) & 0xff
    }
    pub fn set_stencil_compare_mask(&mut self, value: u32) {
        self.set_bits(21, 3, value & 7);
        self.set_bits(24, 5, (value >> 3) & 0x1f);
    }
    pub fn blend_mode(self) -> u8 {
        ((self.byte(3) >> 5) & 3) as u8
    }
    pub fn set_blend_mode(&mut self, value: u8) {
        self.set_bits(29, 2, u32::from(value));
    }
    pub fn polygon_mode(self) -> vk::PolygonMode {
        if self.byte(3) >> 7 != 0 {
            vk::PolygonMode::LINE
        } else {
            vk::PolygonMode::FILL
        }
    }
    pub fn set_polygon_mode(&mut self, value: vk::PolygonMode) {
        let code = match value {
            vk::PolygonMode::FILL => 0,
            vk::PolygonMode::LINE => 1,
            other => panic!("RasterPipelineOpts 不支持多边形模式 {other:?}"),
        };
        self.set_bits(31, 1, code);
    }
    /// front/back 共用的模板状态；write mask 恒 `0xffffffff`，reference 0（动态）。
    pub fn stencil_state(self) -> vk::StencilOpState {
        vk::StencilOpState::default()
            .fail_op(self.stencil_fail_op())
            .pass_op(self.stencil_pass_op())
            .depth_fail_op(self.stencil_depth_fail_op())
            .compare_op(self.stencil_compare_op())
            .compare_mask(self.stencil_compare_mask())
            .write_mask(0xffff_ffff)
            .reference(0)
    }
    /// 把位域展开到管线描述（固定状态部分；`front_face` 恒 COUNTER_CLOCKWISE=0，线宽 1.0）。
    /// 混合附件状态对每个颜色附件复制同一份（原版把同一个 32 字节状态写满 `color_attachments` 份）。
    pub fn apply_to(self, desc: &mut RasterPipelineDesc, color_attachments: usize) {
        desc.topology = self.topology();
        desc.polygon_mode = self.polygon_mode();
        desc.cull_mode = self.cull_mode();
        desc.front_face = vk::FrontFace::COUNTER_CLOCKWISE;
        desc.depth_test = self.depth_test();
        desc.depth_write = self.depth_write();
        desc.depth_compare = self.depth_compare_op();
        desc.stencil_test = self.stencil_test();
        desc.stencil = self.stencil_state();
        desc.blend_attachments = vec![get_vk_blend_mode(self.blend_mode() << 5); color_attachments];
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
        let slot = slot_for(&cache.compute, key);
        let mut created = slot.lock();
        if let Some(pipeline) = created.as_ref() {
            return Ok(pipeline.clone());
        }
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
        *created = Some(pipeline.clone());
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
        let slot = slot_for(&cache.raster, key);
        let mut created = slot.lock();
        if let Some(pipeline) = created.as_ref() {
            return Ok(pipeline.clone());
        }
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
            .depth_compare_op(desc.depth_compare)
            .depth_bounds_test_enable(false)
            .stencil_test_enable(desc.stencil_test)
            .front(desc.stencil)
            .back(desc.stencil)
            .min_depth_bounds(0.0)
            .max_depth_bounds(1.0);
        let blend =
            vk::PipelineColorBlendStateCreateInfo::default().attachments(&desc.blend_attachments);
        // 原 0x142c88b20 / 0x142c88b2c：{VIEWPORT, SCISSOR, STENCIL_REFERENCE[, LINE_WIDTH]}，
        // 个数 `3 + [device+0xad9]`；该字节在 `GraphicsDevice::create`（0x140bf3a6d `setne`，
        // 源 `[rbp+0x22fc]` = 启用特性链里 wideLines 槽）里写入，所以由设备“实际启用 wideLines”
        // 决定，而不是 desc 字段。
        let line_width_dynamic = cache.device.capabilities.wide_lines;
        let mut dynamic_values = vec![
            vk::DynamicState::VIEWPORT,
            vk::DynamicState::SCISSOR,
            vk::DynamicState::STENCIL_REFERENCE,
        ];
        if line_width_dynamic {
            dynamic_values.push(vk::DynamicState::LINE_WIDTH);
        }
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
        *created = Some(pipeline.clone());
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
        u64::from(desc.stencil_test),
        desc.stencil.fail_op.as_raw() as u64,
        desc.stencil.pass_op.as_raw() as u64,
        desc.stencil.depth_fail_op.as_raw() as u64,
        desc.stencil.compare_op.as_raw() as u64,
        u64::from(desc.stencil.compare_mask),
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
        self.device
            .bury(super::graveyard::Doomed::Pipeline(self.handle));
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
    /// 原 `create_raster_pipeline` shader.rs:840:47 `Option::expect("vertex input attrib byte count")`：
    /// `Type::nbyte()` 为 None（opaque 资源类型当顶点输入）。
    VertexInputByteCount,
    /// 原 shader.rs:838:26 `panic!("Unsupported vertex attribute type: {:?}", ty)`，载荷是类型的 Debug 文本。
    UnsupportedVertexAttributeType(String),
    /// 原 compute 管线创建 shader.rs:397:9 `assert!(compiled.reflection.inputs.is_empty())`。
    ComputeHasVertexInputs,
    /// 原 compute 管线创建 shader.rs:460:60 `.expect("a valid group size")`（`local_size` 为 None）。
    MissingGroupSize,
}
/// 原 `rhapsody/src/raw/shader.rs` 管线创建路径的 panic 文案（`.rne` 0x1416a3720 / 0x1416a4f1a 冷尾段读出）。
pub mod original_panics {
    /// 0x1416a4f21（shader.rs:397:9，字串 0x142c88c30，55 字节）。
    pub const COMPUTE_INPUTS_NOT_EMPTY: &str =
        "assertion failed: compiled.reflection.inputs.is_empty()";
    /// 0x1416a50ed（shader.rs:460:60，字串 0x142c88cf8，18 字节）。
    pub const VALID_GROUP_SIZE: &str = "a valid group size";
    /// 0x1416a50d1（shader.rs:453:39，片段 0x142c88d50）：`"Failed creating shader pipeline {name}: {err}"`。
    pub const FAILED_CREATING_SHADER_PIPELINE_PREFIX: &str = "Failed creating shader pipeline ";
    pub const FAILED_CREATING_SHADER_PIPELINE_SEP: &str = ": ";
    /// 0x1416a46c4（shader.rs:972:18，字串 0x142c88b3c，34 字节）：raster 管线 `.expect(..)`，Err 为 ash 的
    /// `(Vec<Pipeline>, vk::Result)`。
    pub const UNABLE_TO_CREATE_GRAPHICS_PIPELINE: &str = "Unable to create graphics pipeline";
    /// 0x1416a451d（shader.rs:840:47，字串 0x142c88b78，30 字节）。
    pub const VERTEX_INPUT_ATTRIB_BYTE_COUNT: &str = "vertex input attrib byte count";
    /// 0x1416a458c（shader.rs:838:26，片段 0x142c88be0，35 字节）。
    pub const UNSUPPORTED_VERTEX_ATTRIBUTE_TYPE_PREFIX: &str =
        "Unsupported vertex attribute type: ";
    /// `Result::unwrap()` 失败的固定前缀（0x142c88708，43 字节），用于 shader.rs:796:18（vkCreatePipelineLayout，
    /// 调用 `[device+0x210]`）、426:18 / 437:18 / 428:78（compute 路径的 vkCreate*）。
    pub const UNWRAP_ON_ERR: &str = "called `Result::unwrap()` on an `Err` value";

    /// compute 管线创建失败（453:39）的完整 panic 文本。
    pub fn failed_creating_shader_pipeline(name: &str, err: &dyn std::fmt::Debug) -> String {
        format!(
            "{FAILED_CREATING_SHADER_PIPELINE_PREFIX}{name}{FAILED_CREATING_SHADER_PIPELINE_SEP}{err:?}"
        )
    }
}
/// 原 compute 管线创建在使用反射前的两个检查（shader.rs:397 assert + 460 expect，顺序按机器码：
/// 先 assert 再取 local_size）。
pub fn validate_compute_reflection(
    reflection: &crate::shader_pipeline::reflection::ShaderReflection,
) -> Result<[u32; 3], InterfaceError> {
    if !reflection.vertex_inputs.is_empty() {
        return Err(InterfaceError::ComputeHasVertexInputs);
    }
    reflection
        .local_size
        .ok_or(InterfaceError::MissingGroupSize)
}
impl std::fmt::Display for InterfaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VertexInputByteCount => {
                f.write_str(original_panics::VERTEX_INPUT_ATTRIB_BYTE_COUNT)
            }
            Self::UnsupportedVertexAttributeType(ty) => {
                write!(
                    f,
                    "{}{ty}",
                    original_panics::UNSUPPORTED_VERTEX_ATTRIBUTE_TYPE_PREFIX
                )
            }
            Self::ComputeHasVertexInputs => f.write_str(original_panics::COMPUTE_INPUTS_NOT_EMPTY),
            Self::MissingGroupSize => f.write_str(original_panics::VALID_GROUP_SIZE),
            // 原 `get_built_in_sampler`（0x140bfc5b0，源 sampler.rs:201）：`panic!("Sampler not found: {name}")`。
            Self::MissingSampler(name) => write!(f, "Sampler not found: {name}"),
            _ => write!(f, "着色器接口无效：{self:?}"),
        }
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
        // 原 0x140be60a0：`assert!(set_count <= MAX_DESCRIPTOR_SETS)`（panic）；这里返回错误。
        // 原版没有布局缓存：每个 set 每次调用都 `vkCreateDescriptorSetLayout`（+0x230），
        // 空 set 也建一个空布局（sType 32，bindingCount 0）。0x140d4d740 是 std `RandomState` 的
        // 线程局部 KEYS（为空 set 的 HashMap 记录取 seed），不是布局缓存。
        if self.sets.len() > super::descriptor_set::MAX_DESCRIPTOR_SETS {
            return Err(vk::Result::ERROR_TOO_MANY_OBJECTS);
        }
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
    /// 原 `create_descriptor_set_states`（0x140be66c0）：每个布局一个状态，池容量固定 100。
    pub fn create_descriptor_set_states(
        layouts: &[Arc<VulkanDescriptorSetLayout>],
    ) -> Result<Vec<DescriptorSetState>, vk::Result> {
        layouts
            .iter()
            .map(|layout| DescriptorSetState::for_layout(layout.clone()))
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
    if !(4..=16).contains(&bytes) || !bytes.is_multiple_of(4) {
        return Err(InterfaceError::InvalidLayout);
    }
    Ok(vk::Format::from_raw(base + 3 * (bytes as i32 / 4 - 1)))
}

/// 反射里的顶点输入 → 每个 location 一个顶点绑定（binding == location，offset 0，
/// stride = 类型字节数，输入率 VERTEX），读自 `create_raster_pipeline`（O 0x1416a3720，0x1416a3a60 起的循环）。
/// 只接受 32 位标量/向量（`map_vertex_attribute_format` 的 tag 2/3/4 = 有符号/无符号/浮点）。
/// 原版按 `hashbrown` 表顺序迭代（任意序），这里按 location 升序；Vulkan 语义等价。
pub fn vertex_input_from_reflection(
    reflection: &crate::shader_pipeline::reflection::ShaderReflection,
) -> Result<
    (
        Vec<vk::VertexInputBindingDescription>,
        Vec<vk::VertexInputAttributeDescription>,
    ),
    InterfaceError,
> {
    use crate::shader_pipeline::reflection_ty::{ScalarType, Type};
    let mut bindings = Vec::new();
    let mut attributes = Vec::new();
    for input in reflection.effective_vertex_inputs().into_values() {
        // 原版先 `ty.nbyte().expect("vertex input attrib byte count")`（0x1416a3ba2），
        // 再按类型分派；非标量/向量 → `panic!("Unsupported vertex attribute type: {:?}")`。
        input
            .ty
            .nbyte()
            .ok_or(InterfaceError::VertexInputByteCount)?;
        let (scalar, count) = match &input.ty {
            Type::Scalar(s) => (s.clone(), 1),
            Type::Vector(v) => (v.scalar_ty.clone(), v.nscalar),
            other => {
                return Err(InterfaceError::UnsupportedVertexAttributeType(format!(
                    "{other:?}"
                )));
            }
        };
        let tag = match scalar {
            ScalarType::Signed(32) => 2,
            ScalarType::Unsigned(32) => 3,
            ScalarType::Float(32) => 4,
            _ => return Err(InterfaceError::InvalidLayout),
        };
        let bytes = u64::from(count) * 4;
        let format = map_vertex_attribute_format(tag, bytes)?;
        bindings.push(
            vk::VertexInputBindingDescription::default()
                .binding(input.location)
                .stride(bytes as u32)
                .input_rate(vk::VertexInputRate::VERTEX),
        );
        attributes.push(
            vk::VertexInputAttributeDescription::default()
                .location(input.location)
                .binding(input.location)
                .format(format)
                .offset(0),
        );
    }
    Ok((bindings, attributes))
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
    compiled_rasters:
        BTreeMap<(String, String), Arc<crate::shader_pipeline::compiled::CompiledShaderPipeline>>,
    compiled_computes:
        BTreeMap<String, Arc<crate::shader_pipeline::compiled::CompiledShaderPipeline>>,
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
            compiled_rasters: BTreeMap::new(),
            compiled_computes: BTreeMap::new(),
        })
    }
    pub fn register(&mut self, key: String, shader: ReflectedShader) -> Result<(), vk::Result> {
        if !Arc::ptr_eq(&self.cache.device, &shader.module.device) {
            return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
        }
        self.layouts.clear();
        self.cache.invalidate();
        self.compiled_computes.remove(&key);
        self.compiled_rasters
            .retain(|(vertex, fragment), _| vertex != &key && fragment != &key);
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
        let plan = if let Some(compiled) = self.compiled_computes.get(key) {
            // 原 compute 管线创建的两个检查（shader.rs:397 / 460）。
            validate_compute_reflection(&compiled.reflection)
                .map_err(|_| vk::Result::ERROR_INITIALIZATION_FAILED)?;
            ShaderInterfacePlan::from_compiled_pipeline(compiled, &by_name, runtime_device_scalar)?
        } else {
            ShaderInterfacePlan::from_reflection(
                std::slice::from_ref(&shader.entry),
                &by_name,
                runtime_device_scalar,
            )
            .map_err(|_| vk::Result::ERROR_INITIALIZATION_FAILED)?
        };
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
        self.compiled_computes.remove(key);
        self.compiled_rasters
            .retain(|(vertex, fragment), _| vertex != key && fragment != key);
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
        let plan = if let Some(compiled) = self
            .compiled_rasters
            .get(&(vertex_key.to_owned(), fragment_key.to_owned()))
        {
            ShaderInterfacePlan::from_compiled_pipeline(
                compiled,
                &sampler_map,
                runtime_device_scalar,
            )?
        } else {
            ShaderInterfacePlan::from_reflection(
                &[vertex.entry.clone(), fragment.entry.clone()],
                &sampler_map,
                runtime_device_scalar,
            )
            .map_err(|_| vk::Result::ERROR_INITIALIZATION_FAILED)?
        };
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
/// 复用原作锁定版本的公开反射库。容器保留原序列化反射尾段；
/// 此处重新反射是已明确标记的候选接口，尚未证明原尾段schema与合并规则等价。
pub fn reflect_spirv_entries(words: &[u32]) -> Result<Vec<ReflectedEntry>, vk::Result> {
    use spirq::{ReflectConfig, SpirvBinary, Variable};
    let reflected = ReflectConfig::new()
        .spv(SpirvBinary::from(words))
        .ref_all_rscs(true)
        .combine_img_samplers(false)
        .gen_unique_names(false)
        .reflect()
        .map_err(|_| vk::Result::ERROR_INVALID_SHADER_NV)?;
    reflected
        .into_iter()
        .map(|entry| {
            let mut descriptors = Vec::new();
            let mut inputs = Vec::new();
            let mut outputs = Vec::new();
            let mut push_constants = Vec::new();
            for variable in entry.vars {
                match variable {
                    Variable::Descriptor {
                        name,
                        desc_bind,
                        desc_ty,
                        ty,
                        nbind,
                    } => {
                        descriptors.push(ReflectedDescriptor {
                            set: desc_bind.set(),
                            binding: desc_bind.bind(),
                            name: name.unwrap_or_default(),
                            count: nbind,
                            descriptor_type: format!("{desc_ty:?}"),
                            reflected_type: reflected_type_json(&ty),
                        });
                    }
                    Variable::Input { location, ty, .. } => inputs.push(ReflectedLocation {
                        location: location.loc(),
                        component: location.comp(),
                        reflected_type: reflected_type_json(&ty),
                    }),
                    Variable::Output { location, ty, .. } => outputs.push(ReflectedLocation {
                        location: location.loc(),
                        component: location.comp(),
                        reflected_type: reflected_type_json(&ty),
                    }),
                    Variable::PushConstant { ty, .. } => {
                        push_constants.push(ReflectedPushConstant {
                            reflected_type: reflected_type_json(&ty),
                        })
                    }
                    // 特殊化常量由调用者另行提供，当前运行时接口计划不生成默认值。
                    Variable::SpecConstant { .. } => {}
                }
            }
            descriptors.sort_by_key(|value| (value.set, value.binding));
            inputs.sort_by_key(|value| (value.location, value.component));
            outputs.sort_by_key(|value| (value.location, value.component));
            Ok(ReflectedEntry {
                entry_point: entry.name,
                stage: format!("{:?}", entry.exec_model),
                descriptors,
                inputs,
                outputs,
                push_constants,
            })
        })
        .collect()
}

/// 复用既有公开fork反射字段投影，类型解析仍由spirq执行。
fn reflected_type_json(ty: &spirq::ty::Type) -> serde_json::Value {
    use serde_json::json;
    use spirq::ty::Type;
    let detail = match ty {
        Type::Struct(value) => {
            json!({"kind":"Struct","members":value.members.iter().map(|member| json!({"name":member.name,"offset":member.offset,"type":reflected_type_json(&member.ty)})).collect::<Vec<_>>()})
        }
        Type::Array(value) => {
            json!({"kind":"Array","count":value.nrepeat,"stride":value.stride,"element":reflected_type_json(&value.proto_ty)})
        }
        Type::Matrix(value) => {
            json!({"kind":"Matrix","axis_order":value.major.map(|major|format!("{major:?}")),"vector":value.vec_ty.to_string(),"count":value.nvec,"stride":value.stride})
        }
        Type::DevicePointer(value) => {
            json!({"kind":"DevicePointer","target":reflected_type_json(&value.pointee_ty)})
        }
        _ => json!({"kind":ty.to_string()}),
    };
    json!({"minimum_bytes":ty.nbyte(),"detail":detail})
}

/// 从真实容器stage记录取得SPIR-V，不使用外部catalog偏移或magic扫描。
/// 序列化stage/入口名与payload不一致时，在Vulkan对象创建前拒绝。
pub fn compiled_pipeline_entries(
    compiled: &crate::shader_pipeline::compiled::CompiledShaderPipeline,
) -> Result<Vec<ReflectedEntry>, vk::Result> {
    use crate::shader_pipeline::compiled::ShaderStage;
    compiled
        .stages
        .iter()
        .map(|stage| {
            let expected = match stage.stage {
                ShaderStage::Compute => "GLCompute",
                ShaderStage::Vertex => "Vertex",
                ShaderStage::Fragment => "Fragment",
            };
            reflect_spirv_entries(&stage.spirv)?
                .into_iter()
                .find(|entry| entry.entry_point == stage.entry_point && entry.stage == expected)
                .ok_or(vk::Result::ERROR_INVALID_SHADER_NV)
        })
        .collect()
}

impl ShaderInterfacePlan {
    /// 直接消费发行容器的stored reflection；SPIR-Q重反射仅用于独立核对。
    pub fn from_compiled_pipeline(
        compiled: &crate::shader_pipeline::compiled::CompiledShaderPipeline,
        samplers: &BTreeMap<String, u64>,
        runtime_device_scalar: u32,
    ) -> Result<Self, vk::Result> {
        use crate::shader_pipeline::{
            compiled::ShaderStage,
            reflection::{BindingArray, BindingKind},
        };
        let compute =
            compiled.stages.len() == 1 && compiled.stages[0].stage == ShaderStage::Compute;
        let raster = compiled.stages.len() == 2
            && compiled
                .stages
                .iter()
                .any(|stage| stage.stage == ShaderStage::Vertex)
            && compiled
                .stages
                .iter()
                .any(|stage| stage.stage == ShaderStage::Fragment);
        if !compute && !raster {
            return Err(vk::Result::ERROR_INVALID_SHADER_NV);
        }
        for index in 0..compiled.stages.len() {
            stored_pipeline_entry(compiled, index)?;
        }
        let visibility = if compute {
            vk::ShaderStageFlags::COMPUTE
        } else {
            vk::ShaderStageFlags::ALL_GRAPHICS
        };
        let mut sets = Vec::new();
        for stored_set in compiled.reflection.effective_descriptor_sets() {
            let mut set = LayoutPlan {
                flags: 0,
                bindings: Vec::new(),
                binding_flags: Vec::new(),
            };
            for (index, binding) in stored_set {
                let tag = match binding.kind {
                    BindingKind::StorageBuffer => 0,
                    BindingKind::UniformBuffer => 1,
                    BindingKind::CombinedImageSampler => 2,
                    BindingKind::StorageImage => 3,
                    BindingKind::SampledImage => 4,
                    BindingKind::Sampler => 5,
                };
                let (count, runtime) = match binding.array {
                    BindingArray::Fixed(count) => (count, false),
                    BindingArray::Runtime => (0, true),
                };
                let sampler = if tag == 5 {
                    samplers
                        .get(&binding.name)
                        .copied()
                        .filter(|handle| *handle != 0)
                        .ok_or(vk::Result::ERROR_INITIALIZATION_FAILED)?
                } else {
                    0
                };
                let plan = descriptor_set_layout(
                    &[Binding {
                        tag,
                        name: binding.name.clone(),
                        count,
                        runtime,
                        sampler,
                    }],
                    u64::from(index),
                    visibility,
                    runtime_device_scalar,
                )
                .map_err(|_| vk::Result::ERROR_INITIALIZATION_FAILED)?;
                set.flags |= plan.flags;
                set.bindings.extend(plan.bindings);
                set.binding_flags.extend(plan.binding_flags);
            }
            for (index, flags) in set.binding_flags.iter().enumerate() {
                if flags & vk::DescriptorBindingFlags::VARIABLE_DESCRIPTOR_COUNT.as_raw() != 0
                    && index + 1 != set.bindings.len()
                {
                    return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
                }
            }
            sets.push(set);
        }
        let size = compiled.reflection.push_constant_size;
        if !size.is_multiple_of(4) {
            return Err(vk::Result::ERROR_INITIALIZATION_FAILED);
        }
        let push_constants = if size == 0 {
            Vec::new()
        } else {
            vec![vk::PushConstantRange {
                stage_flags: visibility,
                offset: 0,
                size,
            }]
        };
        Ok(Self {
            sets,
            push_constants,
        })
    }
}

fn serialized_stage_name(stage: crate::shader_pipeline::compiled::ShaderStage) -> &'static str {
    use crate::shader_pipeline::compiled::ShaderStage;
    match stage {
        ShaderStage::Compute => "GLCompute",
        ShaderStage::Vertex => "Vertex",
        ShaderStage::Fragment => "Fragment",
    }
}

/// 原存储反射直接生产资源名字/编号/数组和push范围；不重新解析公开库类型。
/// SPIR-V只用于入口名/执行模型的一致性检查，完整stage IO独立核对时才重新反射。
pub fn stored_pipeline_entry(
    compiled: &crate::shader_pipeline::compiled::CompiledShaderPipeline,
    index: usize,
) -> Result<ReflectedEntry, vk::Result> {
    use crate::shader_pipeline::{
        compiled::ShaderStage,
        reflection::{BindingArray, BindingKind},
    };
    let stage = compiled
        .stages
        .get(index)
        .ok_or(vk::Result::ERROR_INVALID_SHADER_NV)?;
    let expected = match stage.stage {
        ShaderStage::Compute => rspirv::spirv::ExecutionModel::GLCompute,
        ShaderStage::Vertex => rspirv::spirv::ExecutionModel::Vertex,
        ShaderStage::Fragment => rspirv::spirv::ExecutionModel::Fragment,
    };
    let module =
        rspirv::dr::load_words(&stage.spirv).map_err(|_| vk::Result::ERROR_INVALID_SHADER_NV)?;
    let present = module.entry_points.iter().any(|point| matches!(
        (point.operands.first(), point.operands.get(2)),
        (Some(rspirv::dr::Operand::ExecutionModel(model)), Some(rspirv::dr::Operand::LiteralString(name)))
        if *model == expected && name == &stage.entry_point
    ));
    if !present {
        return Err(vk::Result::ERROR_INVALID_SHADER_NV);
    }
    let mut descriptors = Vec::new();
    for (set, bindings) in compiled
        .reflection
        .effective_descriptor_sets()
        .into_iter()
        .enumerate()
    {
        for (binding, value) in bindings {
            let descriptor_type = match value.kind {
                BindingKind::StorageBuffer => "StorageBuffer(ReadWrite)",
                BindingKind::UniformBuffer => "UniformBuffer",
                BindingKind::CombinedImageSampler => "CombinedImageSampler",
                BindingKind::StorageImage => "StorageImage(ReadWrite)",
                BindingKind::SampledImage => "SampledImage",
                BindingKind::Sampler => "Sampler",
            };
            let count = match value.array {
                BindingArray::Fixed(count) => count,
                BindingArray::Runtime => 0,
            };
            let reflected_type = compiled
                .reflection
                .uniform_block
                .as_ref()
                .filter(|block| block.binding.set == set as u32 && block.binding.binding == binding)
                .map_or(
                    serde_json::Value::Null,
                    |block| serde_json::json!({"minimum_bytes":block.byte_size}),
                );
            descriptors.push(ReflectedDescriptor {
                set: set as u32,
                binding,
                name: value.name.clone(),
                count,
                descriptor_type: descriptor_type.into(),
                reflected_type,
            });
        }
    }
    let push_constants = if compiled.reflection.push_constant_size == 0 {
        Vec::new()
    } else {
        vec![ReflectedPushConstant {
            reflected_type: serde_json::json!({"minimum_bytes":compiled.reflection.push_constant_size}),
        }]
    };
    let inputs = if stage.stage == ShaderStage::Vertex {
        compiled
            .reflection
            .effective_vertex_inputs()
            .into_values()
            .map(|input| {
                let ty = input
                    .ty
                    .to_spirq()
                    .ok_or(vk::Result::ERROR_INVALID_SHADER_NV)?;
                Ok(ReflectedLocation {
                    location: input.location,
                    component: 0,
                    reflected_type: reflected_type_json(&ty),
                })
            })
            .collect::<Result<Vec<_>, vk::Result>>()?
    } else {
        Vec::new()
    };
    Ok(ReflectedEntry {
        entry_point: stage.entry_point.clone(),
        stage: serialized_stage_name(stage.stage).into(),
        descriptors,
        push_constants,
        inputs,
        outputs: Vec::new(),
    })
}

impl ReflectedShader {
    /// 运行时完整容器入口：消费真实stored reflection，不构造空尾段替身。
    pub fn from_compiled_pipeline_stage(
        device: Arc<VulkanDevice>,
        compiled: &crate::shader_pipeline::compiled::CompiledShaderPipeline,
        index: usize,
    ) -> Result<Self, vk::Result> {
        let entry = stored_pipeline_entry(compiled, index)?;
        let bytes = compiled.stages[index].spirv_bytes();
        Ok(Self {
            module: ShaderModule::from_spirv(device, &bytes)?,
            entry,
        })
    }
}

impl ShaderLibrary {
    /// 资产owner提供选择键；原存储字段保留至实际layout生产。
    pub fn register_compiled_compute(
        &mut self,
        key: String,
        bytes: &[u8],
    ) -> Result<(), vk::Result> {
        use crate::shader_pipeline::compiled::{CompiledShaderPipeline, ShaderStage};
        let compiled = CompiledShaderPipeline::decode(bytes)
            .map_err(|_| vk::Result::ERROR_INVALID_SHADER_NV)?;
        if compiled.stages.len() != 1 || compiled.stages[0].stage != ShaderStage::Compute {
            return Err(vk::Result::ERROR_INVALID_SHADER_NV);
        }
        let shader =
            ReflectedShader::from_compiled_pipeline_stage(self.cache.device.clone(), &compiled, 0)?;
        self.register(key.clone(), shader)?;
        self.compiled_computes.insert(key, Arc::new(compiled));
        Ok(())
    }
    /// 两个stage创建成功后一起注册；fragment失败不破坏旧资产/缓存。
    pub fn register_compiled_raster(
        &mut self,
        vertex_key: String,
        fragment_key: String,
        bytes: &[u8],
    ) -> Result<(), vk::Result> {
        use crate::shader_pipeline::compiled::{CompiledShaderPipeline, ShaderStage};
        let compiled = CompiledShaderPipeline::decode(bytes)
            .map_err(|_| vk::Result::ERROR_INVALID_SHADER_NV)?;
        if vertex_key == fragment_key || compiled.stages.len() != 2 {
            return Err(vk::Result::ERROR_INVALID_SHADER_NV);
        }
        let vertex_index = compiled
            .stages
            .iter()
            .position(|stage| stage.stage == ShaderStage::Vertex)
            .ok_or(vk::Result::ERROR_INVALID_SHADER_NV)?;
        let fragment_index = compiled
            .stages
            .iter()
            .position(|stage| stage.stage == ShaderStage::Fragment)
            .ok_or(vk::Result::ERROR_INVALID_SHADER_NV)?;
        let vertex = ReflectedShader::from_compiled_pipeline_stage(
            self.cache.device.clone(),
            &compiled,
            vertex_index,
        )?;
        let fragment = ReflectedShader::from_compiled_pipeline_stage(
            self.cache.device.clone(),
            &compiled,
            fragment_index,
        )?;
        self.register(vertex_key.clone(), vertex)?;
        self.register(fragment_key.clone(), fragment)?;
        self.compiled_rasters
            .insert((vertex_key, fragment_key), Arc::new(compiled));
        Ok(())
    }
}

#[cfg(test)]
mod raster_opts_tests {
    use super::*;

    /// 逐位核对：每个字段只落在原机器码解码的位上（0x1416a3d08..0x1416a3f2f）。
    #[test]
    fn bit_positions_match_the_exe_decode() {
        let mut o = RasterPipelineOpts::new();
        o.set_topology(vk::PrimitiveTopology::LINE_STRIP);
        assert_eq!(o.0, 0b10);
        let mut o = RasterPipelineOpts::new();
        o.set_cull_mode(vk::CullModeFlags::BACK);
        assert_eq!(o.0, 0b10 << 2);
        let mut o = RasterPipelineOpts::new();
        o.set_depth_compare_op(vk::CompareOp::EQUAL);
        o.set_depth_write(true);
        o.set_depth_test_enable(true);
        assert_eq!(o.0, (2 << 4) | (1 << 6) | (1 << 7));
        let mut o = RasterPipelineOpts::new();
        o.set_stencil_test_enable(true);
        o.set_stencil_fail_op(vk::StencilOp::INCREMENT_AND_CLAMP);
        o.set_stencil_pass_op(vk::StencilOp::REPLACE);
        assert_eq!(o.0, (1 << 8) | (3 << 9) | (2 << 12));
        // depth-fail op 3 位：b1 bit7 为低位，b2 bit0–1 为高两位。
        let mut o = RasterPipelineOpts::new();
        o.set_stencil_depth_fail_op(vk::StencilOp::INVERT); // 5 = 0b101
        assert_eq!(o.0, (1 << 15) | (2 << 16));
        assert_eq!(o.stencil_depth_fail_op(), vk::StencilOp::INVERT);
        let mut o = RasterPipelineOpts::new();
        o.set_stencil_compare_op(vk::CompareOp::ALWAYS); // 7
        assert_eq!(o.0, 7 << 18);
        // compare mask 8 位：b2 bit5–7 低 3 位，b3 bit0–4 高 5 位。
        let mut o = RasterPipelineOpts::new();
        o.set_stencil_compare_mask(0xff);
        assert_eq!(o.0, (7 << 21) | (0x1f << 24));
        assert_eq!(o.stencil_compare_mask(), 0xff);
        let mut o = RasterPipelineOpts::new();
        o.set_blend_mode(3);
        o.set_polygon_mode(vk::PolygonMode::LINE);
        assert_eq!(o.0, (3 << 29) | (1 << 31));
    }

    #[test]
    fn split_fields_round_trip() {
        let mut o = RasterPipelineOpts::new();
        for (op, mask) in [(0u32, 0u32), (7, 0xa5), (3, 0x5a), (5, 0xff)] {
            o.set_stencil_depth_fail_op(vk::StencilOp::from_raw(op as i32));
            o.set_stencil_compare_mask(mask);
            o.set_stencil_compare_op(vk::CompareOp::LESS_OR_EQUAL);
            o.set_blend_mode(2);
            assert_eq!(o.stencil_depth_fail_op().as_raw(), op as i32);
            assert_eq!(o.stencil_compare_mask(), mask);
            assert_eq!(o.stencil_compare_op(), vk::CompareOp::LESS_OR_EQUAL);
            assert_eq!(o.blend_mode(), 2);
        }
    }

    /// 跳转表 0x142c88ff0 / 0x142c89000 逐项解出的枚举次序（不是 Vulkan 原值次序）。
    #[test]
    fn jump_table_enums_use_the_exe_ordering() {
        for (code, topology) in [
            (0, vk::PrimitiveTopology::TRIANGLE_LIST),
            (1, vk::PrimitiveTopology::LINE_LIST),
            (2, vk::PrimitiveTopology::LINE_STRIP),
        ] {
            assert_eq!(RasterPipelineOpts(code).topology(), topology);
        }
        for (code, op) in [
            (0, vk::CompareOp::LESS),
            (1, vk::CompareOp::GREATER),
            (2, vk::CompareOp::EQUAL),
        ] {
            assert_eq!(RasterPipelineOpts(code << 4).depth_compare_op(), op);
        }
    }

    #[test]
    fn apply_to_replicates_one_blend_state_per_color_attachment() {
        let device = match crate::raw::init::create_headless_device() {
            Ok(device) => device,
            Err(_) => return,
        };
        let module = Arc::new(ShaderModule {
            device: device.clone(),
            handle: vk::ShaderModule::null(),
        });
        let layout = Arc::new(PipelineLayout {
            device,
            handle: vk::PipelineLayout::null(),
            sets: Vec::new(),
            push_constants: Vec::new(),
        });
        let mut desc = RasterPipelineDesc::new(module, layout);
        let mut o = RasterPipelineOpts::new();
        o.set_blend_mode(2);
        o.set_stencil_test_enable(true);
        o.set_stencil_compare_mask(0x81);
        o.apply_to(&mut desc, 3);
        assert_eq!(desc.blend_attachments.len(), 3);
        // 原 0x140be67f0：模式 2 → dstColor/dstAlpha = ONE_MINUS_SRC_ALPHA(7)。
        assert_eq!(desc.blend_attachments[0].dst_color_blend_factor.as_raw(), 7);
        assert!(desc.stencil_test);
        assert_eq!(desc.stencil.compare_mask, 0x81);
        assert_eq!(desc.stencil.write_mask, 0xffff_ffff);
        assert_eq!(desc.front_face, vk::FrontFace::COUNTER_CLOCKWISE);
    }

    #[test]
    fn map_vertex_attribute_format_follows_the_exe_table() {
        // 0x140be6850：`(bytes-4)/4*3 + base`，base：tag2→99(SINT) / tag3→98(UINT) / tag4→100(SFLOAT)。
        assert_eq!(
            map_vertex_attribute_format(4, 4).unwrap(),
            vk::Format::R32_SFLOAT
        );
        assert_eq!(
            map_vertex_attribute_format(4, 8).unwrap(),
            vk::Format::R32G32_SFLOAT
        );
        assert_eq!(
            map_vertex_attribute_format(4, 12).unwrap(),
            vk::Format::R32G32B32_SFLOAT
        );
        assert_eq!(
            map_vertex_attribute_format(4, 16).unwrap(),
            vk::Format::R32G32B32A32_SFLOAT
        );
        assert_eq!(
            map_vertex_attribute_format(3, 4).unwrap(),
            vk::Format::R32_UINT
        );
        assert_eq!(
            map_vertex_attribute_format(2, 16).unwrap(),
            vk::Format::R32G32B32A32_SINT
        );
        assert!(map_vertex_attribute_format(4, 20).is_err());
        assert!(map_vertex_attribute_format(5, 4).is_err());
    }
}
