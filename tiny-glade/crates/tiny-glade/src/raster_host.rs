//! 光栅 pass 的宿主接线：core 的 `PassHost`（`RasterPass`/`DrawBuilder` 链的外部调用面）+ `RgResources`
//! 落到 rhapsody 的渲染通道缓存、帧缓冲缓存、管线、描述符集与 `vkCmdDraw*`。
//!
//! 调用链（一个 `RasterPass::render()`）：
//! 1. `RasterPass::render` 构造 `RasterRenderPassDesc` → [`PassHost::render_pass_cache`] 未命中 →
//!    [`PassHost::create_render_pass`] = [`RenderPassHost::create_from_desc`]（`RenderPass::from_desc`，0x98B 键，按键去重），
//!    返回的 `ManagedHandle` 就是 `RasterPipelineKey.handle`；
//! 2. [`PassHost::submit_raster_pass`]：每个 `DrawPacket` 取 `RasterShaderProgram.variants[variant].get_or_generate`
//!    （键 = 通道句柄 + `packet.state` 4 字节位域）得到 [`RasterEntry`]；pass 的资源访问（颜色/深度/RG 缓冲与纹理）
//!    进 rhapsody 图声明；
//! 3. 图执行时的回调：`AttachmentCache::prepare`（视图 + 兼容通道 + 帧缓冲）→ `begin_attachments` →
//!    默认视口（负高度翻转 Y）+ stencil ref → 逐 draw：`vkCmdBindPipeline`（原版属于 `GraphicsDevice::bind_pipeline`，不是
//!    `RenderPassApi`）→ set 0 描述符集（`DrawPacket.bindings` + RG 访问）→ `RenderPassApi::draw*`。
//!
//! 已核对（O）与未证实（U）的取舍都在对应函数注释里；不猜的 draw 种类在声明期直接 panic，不会静默画错。
use crate::render_host::{
    BufferHost, RasterEntry, ShaderHost, TextureHost, layout_binding_type, rh_view_desc,
};
use ash::vk;
use country_core::{
    render::{
        pass_builder::{
            draw_stream::{
                AttachmentDesc, DEPTH_STATE_NONE, DescriptorSetBinding as CoreBinding, DrawKind,
                DrawPacket, DynamicConstantsBinding, PassHost, RasterPassSubmission,
                RasterRenderPassDesc, RasterState, ShaderReflection as CoreReflection,
            },
            framebuffer_binding::{AttachmentTexture, DepthClear},
            pass_resource_access::RgTexture,
            rg_resources::{
                RgError, RgResources, TemporalBuffer, TemporalTexture, TemporalTexturePair,
            },
        },
        shader::{
            RasterShaderProgram,
            boundary::{RasterPipelineKey, ShaderBackend},
        },
        ssbo::rhapsody_boundary::BufferDesc as CoreBufferDesc,
    },
    rhapsody_contract::{Handle, ManagedHandle, RenderPassBinding},
    rhapsody_types::TextureDesc as CoreTextureDesc,
};
use hashbrown::HashMap as HbMap;
use parking_lot::Mutex;
use rhapsody::{
    raw::{
        binding::DescriptorSetBinding,
        descriptor_set::DescriptorWrite,
        dynamic_constants::{DynamicConstants, DynamicConstantsChunk},
        render_pass::{
            AttachmentArray, DepthAccess, RenderPassAttachmentDesc as RhAttachmentDesc,
            RenderPassDesc as RhRenderPassDesc,
        },
        shader::RasterPipelineOpts,
    },
    rg::{
        attachments::{Attachment, AttachmentCache},
        graph::{BuildError, RenderGraph as RhGraph},
        pass_api::{AshCommandBackend, RenderPassApi as RhRenderPassApi},
        pass_bindings::ManagedPassResources,
        resource::ResourceHandle,
        vulkan::{ResourceSpec, VulkanGraphSession},
    },
    shader_pipeline::{compiled::CompiledShaderPipeline as RhCompiled, reflection::BindingNameHash},
};
use std::{cell::RefCell, collections::HashMap, marker::PhantomData, rc::Rc, sync::Arc};

// ---------------------------------------------------------------- 图内资源声明

/// 在图里“创建”资源需要后端登记物理规格（`VulkanGraphBackend::declare`），而后端归会话所有。
/// 宿主不持有会话，由帧驱动方提供这个窄接口（对 `Rc<RefCell<VulkanGraphSession>>` 有现成实现）。
pub trait GraphDeclarer {
    fn declare(
        &self,
        graph: &mut RhGraph,
        name: &str,
        spec: ResourceSpec,
    ) -> Result<ResourceHandle, BuildError>;
}

impl GraphDeclarer for RefCell<VulkanGraphSession> {
    fn declare(
        &self,
        graph: &mut RhGraph,
        name: &str,
        spec: ResourceSpec,
    ) -> Result<ResourceHandle, BuildError> {
        self.borrow_mut().backend.declare(graph, name, spec)
    }
}

/// core `RgResources`（`RenderGraph::create_raw_resource` / `get_or_create_temporal*`）的宿主实现。
///
/// `create_raw_*` 真实落到 rhapsody 图（`create_raw_resource` + 后端规格登记）。temporal 三个入口**未接**：
/// rhapsody 的 `TemporalRenderGraphState` 以 `&str` 为键、需要按帧导入物理纹理，而 core 侧键应为 `Ustr`
/// （`ustr_contract::intern` 仍返回 0），键空间对不上之前接上只会得到“看起来能用”的错误行为。
pub struct GraphRgResources {
    graph: Rc<RefCell<RhGraph>>,
    declarer: Rc<dyn GraphDeclarer>,
    textures: Arc<TextureHost>,
}

impl RgResources for GraphRgResources {
    fn create_raw_texture(&mut self, name: &str, desc: &CoreTextureDesc) -> RgTexture {
        let rh = self
            .textures
            .convert_for_graph(desc)
            .unwrap_or_else(|e| panic!("create_raw_texture({name}): {e:?}"));
        let handle = self
            .declarer
            .declare(&mut self.graph.borrow_mut(), name, ResourceSpec::Image(rh))
            .unwrap_or_else(|e| panic!("RenderGraph::create_raw_resource({name}): {e:?}"));
        RgTexture { resource: handle.raw(), desc: *desc }
    }

    fn create_raw_buffer(&mut self, name: &str, desc: &CoreBufferDesc) -> country_core::render::pass_builder::compute_pass_builder::RgBuffer {
        let handle = self
            .declarer
            .declare(
                &mut self.graph.borrow_mut(),
                name,
                ResourceSpec::Buffer(BufferHost::convert(desc)),
            )
            .unwrap_or_else(|e| panic!("RenderGraph::create_raw_resource({name}): {e:?}"));
        country_core::render::pass_builder::compute_pass_builder::RgBuffer { resource: handle.raw() }
    }

    fn get_or_create_temporal_texture(
        &mut self,
        name: &str,
        _desc: &CoreTextureDesc,
    ) -> Result<TemporalTexture, RgError> {
        Err(RgError(format!("temporal texture `{name}`: host temporal resources are not connected")))
    }

    fn get_or_create_temporal_texture_pair(
        &mut self,
        name: &str,
        _desc: &CoreTextureDesc,
    ) -> Result<TemporalTexturePair, RgError> {
        Err(RgError(format!("temporal texture pair `{name}`: host temporal resources are not connected")))
    }

    fn get_or_create_temporal_buffer(
        &mut self,
        name: &str,
        _desc: &CoreBufferDesc,
    ) -> Result<TemporalBuffer, RgError> {
        Err(RgError(format!("temporal buffer `{name}`: host temporal resources are not connected")))
    }
}

// ---------------------------------------------------------------- 共享状态

/// 动态常量页里的一块（`DrawBuilder::new` 分配，`flush_dynamic_constants` 写入，描述符集绑定它）。
struct ConstantsSlot {
    chunk: DynamicConstantsChunk,
}

/// 资产库在 `PassHost` 上的几个只读查询（`AssetSsboLibrary` / `AssetTextureLibrary` / `PlacementDataSsboLibrary`）。
/// 这些库本身是 core 的世界资源，宿主在这里按需登记；没登记就按原版“库里没有该项”的路径失败。
#[derive(Default)]
pub struct AssetTables {
    /// `SsboId`（`xxh3_64(NAME)`）→ 图里的缓冲资源下标。
    pub rg_buffers: HashMap<u64, u32>,
    /// `GlobalId` → (缓冲句柄, flag)。
    pub placement: HashMap<u64, (ManagedHandle, u32)>,
    /// `TextureFromDisk` 文件键 → (纹理句柄, 描述)。
    pub textures_by_name: HashMap<u64, (ManagedHandle, AttachmentTexture)>,
    /// `TexturePreset` → RG 纹理。
    pub presets: HashMap<u8, AttachmentTexture>,
}

/// 全部光栅 pass 共享的宿主状态。
pub struct RasterShared {
    pub buffers: Arc<BufferHost>,
    pub textures: Arc<TextureHost>,
    pub shaders: Arc<ShaderHost>,
    device: Arc<rhapsody::raw::device::GraphicsDevice>,
    attachments: Mutex<AttachmentCache>,
    programs: Mutex<Vec<Arc<RasterShaderProgram>>>,
    reflections: Mutex<HashMap<(u64, u8), Arc<CoreReflection>>>,
    dynamic: Mutex<DynamicConstants>,
    constants: Mutex<Vec<ConstantsSlot>>,
    pub assets: Mutex<AssetTables>,
}

impl RasterShared {
    pub fn new(
        device: Arc<rhapsody::raw::device::GraphicsDevice>,
        buffers: Arc<BufferHost>,
        textures: Arc<TextureHost>,
        shaders: Arc<ShaderHost>,
    ) -> Result<Arc<Self>, rhapsody::raw::error::Error> {
        Ok(Arc::new(Self {
            attachments: Mutex::new(AttachmentCache::new(device.inner.clone())),
            dynamic: Mutex::new(DynamicConstants::new(device.inner.clone())?),
            device,
            buffers,
            textures,
            shaders,
            programs: Mutex::new(Vec::new()),
            reflections: Mutex::new(HashMap::new()),
            constants: Mutex::new(Vec::new()),
            assets: Mutex::new(AssetTables::default()),
        }))
    }

    /// 登记一个 core 的 `RasterShaderProgram`，返回 `DrawPacket.shader` 用的句柄（id 从 1 起）。
    pub fn register_program(&self, program: Arc<RasterShaderProgram>) -> Handle<RasterShaderProgram> {
        let mut table = self.programs.lock();
        table.push(program);
        Handle(ManagedHandle(table.len() as u64), PhantomData)
    }

    fn program(&self, id: u64) -> Option<Arc<RasterShaderProgram>> {
        self.programs.lock().get((id as usize).checked_sub(1)?).cloned()
    }

    /// 清掉帧缓冲/视图缓存（附件纹理被销毁/缩放后调用；对应会话的 `collect_after_completion` 之前）。
    pub fn clear_attachment_cache(&self) {
        self.attachments.lock().clear();
    }

    /// 一个 pass 一个宿主句柄（`RasterPass` 按 `Box<dyn PassHost>` 拥有它）。
    pub fn pass_host(
        self: &Arc<Self>,
        graph: Rc<RefCell<RhGraph>>,
        declarer: Option<Rc<dyn GraphDeclarer>>,
    ) -> RasterPassHost {
        RasterPassHost {
            shared: self.clone(),
            rg: declarer.map(|declarer| GraphRgResources {
                graph: graph.clone(),
                declarer,
                textures: self.textures.clone(),
            }),
            graph,
            cache: HbMap::new(),
        }
    }
}

// ---------------------------------------------------------------- 描述转换

/// core 的 `RasterRenderPassDesc`（`RasterPass::render` 构造）→ rhapsody 的 0x98B `RenderPassDesc` 键。
///
/// 颜色附件的 `+4`（原 `clear_input` 字，即 `loadOp`）在 core 里被覆盖为附件 tag：tag0 → LOAD、tag1 → CLEAR、
/// tag2 → DONT_CARE（O：`render` 里的覆盖写；`vkCreateRenderPass` 原样用该字作 loadOp，0x1418b8c50）。
/// 深度：`depth_state` 字节 3 = 无深度，其余按 `DepthAccess` 跳表（0 只读 / 1 读写 / 2 深度写+模板读）。
pub fn rh_render_pass_desc(desc: &RasterRenderPassDesc) -> RhRenderPassDesc {
    let attachment = |a: &AttachmentDesc, load: u32| RhAttachmentDesc {
        format: vk::Format::from_raw(a.format as i32),
        clear_input: load,
        store_op: 0,
        sample_count: a.sample_count,
        reserved_0d: [0; 3],
    };
    let mut colors = AttachmentArray::new();
    for a in &desc.colors[..desc.color_count as usize] {
        colors.push(attachment(a, a.kind_tag));
    }
    let depth = (desc.depth_state != DEPTH_STATE_NONE).then(|| {
        let access = DepthAccess::from_tag(desc.depth_state)
            .flatten()
            .expect("RasterRenderPassDesc.depth_state is not a DepthAccess tag");
        (attachment(&desc.depth, u32::from(desc.depth.clear_input)), access)
    });
    RhRenderPassDesc { colors, depth }
}

fn core_reflection(
    program: &RasterShaderProgram,
    variant: u8,
) -> Option<Arc<CoreReflection>> {
    let compiled = &program.variants.get(variant as usize)?.compiled;
    let decoded = RhCompiled::decode(&compiled.0).ok()?;
    let reflection = &decoded.reflection;
    Some(Arc::new(CoreReflection {
        program_name: program.name,
        bindings: reflection
            .binding_name_hashes
            .iter()
            .map(|(hash, location)| (*hash, (location.set, location.binding)))
            .collect(),
        has_uniform_block: reflection.uniform_block.is_some(),
        uniform_offsets: reflection
            .uniform_block
            .iter()
            .flat_map(|block| block.name_hash_offsets.iter().copied())
            .collect(),
        uniform_block_size: reflection.uniform_block.as_ref().map_or(0, |b| b.byte_size),
        uniform_block_binding: reflection.uniform_block.as_ref().map_or(0, |b| b.binding.binding),
    }))
}

// ---------------------------------------------------------------- PassHost

/// 一个 `RasterPass` 的宿主句柄。
pub struct RasterPassHost {
    shared: Arc<RasterShared>,
    graph: Rc<RefCell<RhGraph>>,
    rg: Option<GraphRgResources>,
    /// 设备级 `HashMap<RenderPassDesc, RenderPass>` 缓存（`+0x188`）的每 pass 视图：去重由
    /// [`RenderPassHost::create_from_desc`] 的键表保证，所以不同 pass 拿到的是同一个通道句柄。
    cache: HbMap<RasterRenderPassDesc, ManagedHandle>,
}

/// 一个 draw 在声明期解析好的东西：包 + 管线条目。
struct PreparedDraw {
    packet: DrawPacket,
    entry: Arc<RasterEntry>,
}

impl PassHost for RasterPassHost {
    fn raster_shader(
        &self,
        shader: &Handle<RasterShaderProgram>,
        variant: u8,
    ) -> Option<Arc<CoreReflection>> {
        let key = (shader.0.0, variant);
        if let Some(hit) = self.shared.reflections.lock().get(&key) {
            return Some(hit.clone());
        }
        let program = self.shared.program(shader.0.0)?;
        let reflection = core_reflection(&program, variant)?;
        self.shared.reflections.lock().insert(key, reflection.clone());
        Some(reflection)
    }

    /// 在常量页里为该管线的 uniform 块分配一块；字节在 [`PassHost::flush_dynamic_constants`] 写入。
    fn allocate_dynamic_constants(
        &mut self,
        reflection: &CoreReflection,
    ) -> Option<DynamicConstantsBinding> {
        let size = u64::from(reflection.uniform_block_size);
        let chunk = self.shared.dynamic.lock().allocate(size).ok()?;
        let mut slots = self.shared.constants.lock();
        slots.push(ConstantsSlot { chunk });
        Some(DynamicConstantsBinding {
            data: vec![0; size as usize],
            binding_index: reflection.uniform_block_binding,
            keepalive: ManagedHandle(slots.len() as u64),
        })
    }

    fn flush_dynamic_constants(&mut self, constants: &DynamicConstantsBinding) {
        let slots = self.shared.constants.lock();
        let slot = (constants.keepalive.0 as usize)
            .checked_sub(1)
            .and_then(|i| slots.get(i))
            .expect("flush_dynamic_constants: keepalive handle was not allocated by this host");
        // 声明期写入：GPU 尚未访问这块（页只增不复用），提交后可见。
        slot.chunk
            .write_bytes(&constants.data)
            .expect("flush_dynamic_constants: chunk write failed");
    }

    fn rg_buffer_resource(&self, rg_buffer_hash: u64) -> Option<u32> {
        self.shared.assets.lock().rg_buffers.get(&rg_buffer_hash).copied()
    }

    fn placement_buffer(&self, id: u64) -> Option<(ManagedHandle, u32)> {
        self.shared.assets.lock().placement.get(&id).cloned()
    }

    fn vk_buffer(&self, buffer: &ManagedHandle) -> Option<u64> {
        let managed = self.shared.buffers.get_by_id(buffer.0)?;
        Some(ash::vk::Handle::as_raw(managed.raw()))
    }

    fn vk_image_view(&self, texture: &ManagedHandle) -> Result<u64, String> {
        self.shared
            .textures
            .default_view(texture.0)
            .ok_or_else(|| "get_vk_image_view: texture handle is not live".to_owned())
    }

    fn texture_by_name(&self, name_hash: u64) -> (ManagedHandle, AttachmentTexture) {
        self.shared
            .assets
            .lock()
            .textures_by_name
            .get(&name_hash)
            .cloned()
            .expect("No such texture")
    }

    fn rg_texture(&self, preset: u8) -> AttachmentTexture {
        self.shared
            .assets
            .lock()
            .presets
            .get(&preset)
            .copied()
            .expect("no entry found for key")
    }

    /// `BindingNameHash::with_suffix_dims`（O：`wrapping_add(1)`，RHAPSODY-DEEP-SUMMARY §3 #9）。
    fn with_suffix_dims(&self, name_hash: u64) -> u64 {
        BindingNameHash(name_hash).with_suffix_dims().0
    }

    /// `rhapsody::raw::shader::impl$7::default()`：`RasterPipelineOpts` 的 4 字节位域缺省值。
    fn default_raster_state(&self) -> RasterState {
        RasterState(RasterPipelineOpts::new().0)
    }

    fn create_render_pass(&mut self, desc: &RasterRenderPassDesc) -> ManagedHandle {
        let id = self
            .shared
            .shaders
            .passes()
            .create_from_desc(&self.shared.device, &rh_render_pass_desc(desc))
            .unwrap_or_else(|e| panic!("GraphicsDevice::create_render_pass: {e:?}"));
        ManagedHandle(id)
    }

    fn render_pass_cache(&mut self) -> &mut HbMap<RasterRenderPassDesc, ManagedHandle> {
        &mut self.cache
    }

    fn submit_raster_pass(&mut self, submission: RasterPassSubmission) {
        let shared = self.shared.clone();
        let RasterPassSubmission {
            name,
            draws,
            bindings,
            render_pass,
            accesses,
            colors,
            depth,
            ..
        } = submission;

        // 声明期解析每个 draw 的管线（失败 = 原版 `Result::unwrap` panic，见 `get_or_generate`）。
        let prepared: Vec<PreparedDraw> = draws
            .into_iter()
            .map(|packet| {
                reject_unverified(&packet.kind);
                let program = shared
                    .program(packet.shader.0.0)
                    .expect("draw packet references a shader program that was not registered");
                let variant = program
                    .variants
                    .get(usize::from(packet.variant))
                    .expect("draw packet variant out of range");
                let key = RasterPipelineKey { handle: render_pass.0, selector: packet.state.0 };
                let pipeline = variant.get_or_generate(&*shared.shaders as &dyn ShaderBackend, key);
                let entry = shared
                    .shaders
                    .raster_entry(pipeline)
                    .expect("raster pipeline was not created through this host");
                PreparedDraw { packet, entry }
            })
            .collect();

        let mut graph = self.graph.borrow_mut();
        let mut declared = graph.add_pass(name);
        for a in &accesses {
            declared
                .add_resource_access(ResourceHandle::from_raw(a.resource), a.access, a.group)
                .expect("add_resource_access: resource does not belong to this graph");
        }
        declared.render_with_context(move |ctx| {
            let managed = ctx.managed.expect("pass callback without managed resources");
            let device = shared.device.inner.clone();
            let command = <vk::CommandBuffer as ash::vk::Handle>::from_raw(ctx.command.handle());

            // 附件：tag 决定 loadOp（0 LOAD / 1 CLEAR / 2 DONT_CARE，clear 值取缓存默认的 0）。
            let mut attachments = Vec::new();
            let mut extent = None;
            for c in &colors {
                let texture = managed.texture_for(c.id).expect("color attachment: unresolved image");
                extent.get_or_insert(vk::Extent2D {
                    width: texture.desc.width,
                    height: texture.desc.height,
                });
                // CLEAR（tag1）需要清除值；core 的附件描述里没有颜色清除值，取零（I：`AttachmentCache` 的缺省）。
                let clear = (c.tag == 1).then_some([0.0f32; 4]);
                let mut a = Attachment::color(texture.shared(), 0, clear);
                a.desc.load = vk::AttachmentLoadOp::from_raw(c.tag as i32);
                attachments.push(a);
            }
            if let Some(d) = &depth {
                let texture = managed.texture_for(d.texture_id).expect("depth attachment: unresolved image");
                extent.get_or_insert(vk::Extent2D {
                    width: texture.desc.width,
                    height: texture.desc.height,
                });
                let clear = match d.clear {
                    DepthClear::Clear(v) => Some(v),
                    DepthClear::Load => None,
                };
                let mut a = Attachment::depth(texture.shared(), 0, clear).depth_access(if d.writable {
                    rhapsody::rg::attachments::DepthAccess::ReadWrite
                } else {
                    rhapsody::rg::attachments::DepthAccess::ReadOnly
                });
                a.view.base_layer = d.layer;
                a.view.layer_count = 1;
                attachments.push(a);
            }
            let extent = extent.expect("raster pass without attachments");
            let prepared_pass = shared
                .attachments
                .lock()
                .prepare(&attachments, extent, 1)
                .expect("AttachmentCache::prepare failed");
            ctx.keep_alive(prepared_pass.framebuffer.clone());

            // SAFETY: 命令缓冲处于录制状态；资源与描述符集经 `keep_alive` 交给命令拥有者保活。
            let mut backend = unsafe { AshCommandBackend::new(&device.device) };
            // 初始 stencil 缓存取 1，使 `set_stencil_ref(0)` 必发（管线的 stencil 引用是动态状态）。
            let mut api = RhRenderPassApi::new(&mut backend, command, 1, false);
            api.begin_attachments(&prepared_pass);
            api.set_default_view_and_scissor(extent);
            api.set_stencil_ref(0);

            for draw in &prepared {
                ctx.keep_alive(draw.entry.clone());
                // SAFETY: 管线属于本设备，命令缓冲在录制中。
                unsafe {
                    device.device.cmd_bind_pipeline(
                        command,
                        vk::PipelineBindPoint::GRAPHICS,
                        draw.entry.pipeline.handle,
                    )
                };
                if let Some(state) = &draw.entry.set0 {
                    let writes = descriptor_writes(&shared, draw, &bindings, managed);
                    // SAFETY: 写入的句柄属于本设备，资源由 `owner` 持有并随集合交给命令拥有者保活。
                    let set = unsafe { state.allocate_descriptor_set(&writes) }
                        .expect("raster pass: descriptor set allocation failed");
                    // SAFETY: 同上。
                    unsafe {
                        device.device.cmd_bind_descriptor_sets(
                            command,
                            vk::PipelineBindPoint::GRAPHICS,
                            draw.entry.pipeline.layout.handle,
                            0,
                            &[set.handle],
                            &[],
                        )
                    };
                    ctx.keep_alive(set);
                }
                record_draw(&mut api, &shared, &draw.packet.kind, |owner| ctx.keep_alive(owner));
            }
            api.end_render_pass();
            true
        });
    }

    fn rg_resources(&mut self) -> Option<&mut dyn RgResources> {
        self.rg.as_mut().map(|r| r as &mut dyn RgResources)
    }
}

/// 只接机器码/调用点能确定语义的 draw 种类。其余在声明期 panic（“未证实”不画错）：
/// - `InstancedGlDrawFlag`：`flag` 的含义（实例数？绘制标志？）与顶点数来源未知；
/// - `MultiInstanced`：`(first_instance, instance_count)` 已证实（`sparse_instance_buffer`），但每实例顶点数来源未知；
/// - `MultiDrawIndirect` / `IndexedIndirect` / `IndexedIndirectCount`：`arg_48/50/58` 与 vkCmdDraw*Indirect 参数
///   （偏移/次数/步长）的对应没有证据。
fn reject_unverified(kind: &DrawKind) {
    match kind {
        DrawKind::NoVao { .. } | DrawKind::NoIndexBuffer { .. } | DrawKind::NoVaoIndexed { .. } => {}
        other => panic!(
            "raster host: draw kind {other:?} (discriminant {}) is not verified against the original binary (未证实), refusing to guess its vkCmdDraw mapping",
            other.discriminant()
        ),
    }
}

fn record_draw<B: rhapsody::rg::pass_api::CommandBackend>(
    api: &mut RhRenderPassApi<'_, B>,
    shared: &RasterShared,
    kind: &DrawKind,
    keep_alive: impl Fn(Arc<rhapsody::raw::managed::buffer::ManagedBuffer>),
) {
    match kind {
        // 全屏三角形/SSBO 拉取顶点：无顶点缓冲，`vkCmdDraw(count, 1, 0, 0)`（O：`draw_no_vao(3)`）。
        DrawKind::NoVao { vertex_count } | DrawKind::NoIndexBuffer { vertex_count } => {
            api.draw(*vertex_count, 1, 0, 0);
        }
        // 索引缓冲 `vao.rs`：`indices.len() * 4` 字节 → UINT32（O）。
        DrawKind::NoVaoIndexed { index_count, index_buffer } => {
            let buffer = shared
                .buffers
                .get_by_id(index_buffer.0)
                .expect("draw_no_vao_indexed: index buffer is not live");
            api.bind_index_buffer(buffer.raw(), vk::IndexType::UINT32);
            keep_alive(Arc::new(buffer));
            api.draw_indexed(*index_count, 1, 0, 0, 0);
        }
        other => unreachable!("{other:?} was rejected at declaration time"),
    }
}

fn descriptor_writes(
    shared: &RasterShared,
    draw: &PreparedDraw,
    pass_bindings: &[RenderPassBinding],
    managed: &ManagedPassResources,
) -> Vec<DescriptorWrite> {
    let layout0 = draw.entry.pipeline.layout.sets.first();
    let ty_of = |binding: u32, default: vk::DescriptorType| {
        layout0.and_then(|l| layout_binding_type(l, binding)).unwrap_or(default)
    };
    let mut writes = Vec::new();

    // 包内的原生绑定（外部缓冲/原生纹理/动态常量）。
    for (binding, owned) in &draw.packet.bindings {
        if owned.state != country_core::render::pass_builder::draw_stream::GpuOwned::STATE_OWNED {
            continue;
        }
        let value = match &owned.binding {
            CoreBinding::StorageBuffer { buffer, offset, range, keepalive } => {
                let owner = shared
                    .buffers
                    .get_by_id(keepalive.0)
                    .expect("storage buffer binding: buffer is not live");
                DescriptorSetBinding::Buffer {
                    ty: ty_of(*binding, vk::DescriptorType::STORAGE_BUFFER),
                    info: vk::DescriptorBufferInfo {
                        buffer: <vk::Buffer as ash::vk::Handle>::from_raw(*buffer),
                        offset: *offset,
                        range: *range,
                    },
                    owner: Arc::new(owner),
                }
            }
            CoreBinding::SampledImage { image_view, layout_word, keepalive } => {
                let owner = shared
                    .textures
                    .get_by_id(keepalive.0)
                    .expect("sampled image binding: texture is not live");
                DescriptorSetBinding::Image {
                    ty: ty_of(*binding, vk::DescriptorType::SAMPLED_IMAGE),
                    info: vk::DescriptorImageInfo {
                        sampler: vk::Sampler::null(),
                        image_view: <vk::ImageView as ash::vk::Handle>::from_raw(*image_view),
                        // `{sampler 0, view, 5}`：5 = SHADER_READ_ONLY_OPTIMAL（O：raw 路径立即数）。
                        image_layout: vk::ImageLayout::from_raw(*layout_word as i32),
                    },
                    owners: vec![Arc::new(owner)],
                }
            }
            CoreBinding::UniformBuffer { size, keepalive } => {
                let slots = shared.constants.lock();
                let slot = slots
                    .get((keepalive.0 as usize) - 1)
                    .expect("uniform binding: constants chunk was not allocated by this host");
                DescriptorSetBinding::Buffer {
                    ty: ty_of(*binding, vk::DescriptorType::UNIFORM_BUFFER),
                    info: vk::DescriptorBufferInfo {
                        buffer: slot.chunk.buffer.handle,
                        offset: slot.chunk.offset,
                        range: u64::from(*size),
                    },
                    owner: slot.chunk.buffer.clone(),
                }
            }
        };
        writes.push(DescriptorWrite { binding: *binding, array_element: 0, value });
    }

    // RG 资源访问：`accesses[(binding, slot)]` → pass 绑定表 → 图里的物理资源。
    for access in &draw.packet.accesses {
        let pb = pass_bindings[usize::from(access.slot)];
        let value = if let Ok(texture) = managed.texture_for(pb.resource_index) {
            let desc = pb
                .view
                .as_ref()
                .map_or_else(|| rhapsody::raw::texture::TextureViewDesc::new(&texture.desc), |v| rh_view_desc(&texture, v));
            let view = texture.image_view(&desc).expect("raster binding: image view creation failed");
            let written = country_core::render::pass_builder::draw_stream::access_is_write(pb.access_type);
            let ty = ty_of(
                access.binding_index,
                if written { vk::DescriptorType::STORAGE_IMAGE } else { vk::DescriptorType::SAMPLED_IMAGE },
            );
            let image_layout = if ty == vk::DescriptorType::STORAGE_IMAGE {
                vk::ImageLayout::GENERAL
            } else {
                vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL
            };
            DescriptorSetBinding::Image {
                ty,
                info: vk::DescriptorImageInfo {
                    sampler: vk::Sampler::null(),
                    image_view: view.raw(),
                    image_layout,
                },
                owners: vec![Arc::new(view)],
            }
        } else {
            let buffer = managed
                .buffer_for(pb.resource_index)
                .expect("raster binding: resource is neither an image nor a buffer");
            DescriptorSetBinding::Buffer {
                ty: ty_of(access.binding_index, vk::DescriptorType::STORAGE_BUFFER),
                info: vk::DescriptorBufferInfo { buffer: buffer.handle, offset: 0, range: vk::WHOLE_SIZE },
                owner: Arc::new(buffer),
            }
        };
        writes.push(DescriptorWrite { binding: access.binding_index, array_element: 0, value });
    }
    writes
}
