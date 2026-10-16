# D 组（rhapsody 接入）接线 SPEC — rhapsody-WIRING

日期：2026-10-16。组：D rhapsody 接入，历史分析产出。范围：country-core 渲染层占位/桩 → `crates/rhapsody` 真实公开 API 的接线契约。

**只列 rhapsody 已存在公开 API**；缺接口登记「登记缺口」，不往 rhapsody 补造函数。签名以 `crates/rhapsody/src/**` 实际源码为准；原入口以 rhapsody_types.rs/rhapsody_contract.rs/render/* 注释的 O 地址与 `evidence/render-recovery/`、`country-core-deep/rhapsody/RHAPSODY-DEEP-SUMMARY.md` 为证据。找不到原入口的标「待取证」。

## 0. 关键事实与前提

- `crates/rhapsody` 公开面 = `rhapsody::{raw, rg, leek, log_ring, shader_pipeline, shader_compiler}`（lib.rs）。`shader_pipeline`/`shader_compiler` 是 rhapsody 嵌套包（`crates/rhapsody/crates/`），经 `pub use` 重导出。
- **设备句柄适配层在 `rhapsody::raw::device_api.rs`**（`impl GraphicsDevice`），不是 `raw::device.rs` 本体。country-core 想要的 `create_buffer/create_texture/get_vk_image_view/update_buffer_bytes/buffer_mapped_slice_mut/buffer_device_address/create_render_pass/create_framebuffer` 都在 device_api.rs。
- 图构图入口在 `rhapsody::rg::graph::{RenderGraph, ResourceBackend, GraphDeclaration, BuiltGraph, compile_graph, Pool, Descriptor, OriginSource, ResourceBackend, NO_ACCESS}`；录制回调类型 `RenderCallback = Box<dyn FnMut(PassContext) -> bool + Send>`（graph.rs）。
- 图执行入口在 `rhapsody::rg::executor::{ExecutionChain, ExecutionBackend, execute_graph, execute_graph_with_context, begin, record, submit_recorded, complete, PassContext, GraphExecution, ExportedRenderGraphResources, PassPgoFrames}`。**country-core 全 crate 当前无 ExecutionChain 引用**（协调简报 §D 现状）——这是本轮新增的接线点。
- **三种 32B TextureDesc 并存**，务必区分：
  1. `rhapsody::raw::texture::TextureDesc`（设备/纹理创建；命名布局：usage@0 flags@4 format@8 width/height/depth array_elements mip_levels sample_count kind）；
  2. `rhapsody::rg::graph::TextureDesc`（`resource_registry.rs`，`[u32;7]+[u8;3]` 裸 ABI，图 Descriptor::Image 载荷；`rg::resource::descriptor_from_texture` 逐字节拷贝转换）；
  3. country-core `rhapsody_types.rs::TextureDesc`（占位，builder 风格，待替换为 1）。
  country-core `render_world.rs` 自带 32B `TextureDesc` 与 1 的字段布局一致（usage@0 reserved@4 format@8 extent@0xc …），可直接替换为 1。

## 1. 映射表

约定：`CC := country-core 占位/桩`，`RH := rhapsody 真实公开类型/API`。「签名差异」= 桩与真实形态不同，需桥接/改造；「缺」= rhapsody 无此接口，见 §4。

| CC（文件） | CC 现状 | RH（rhapsody 真实） | 差异/说明 |
|---|---|---|---|
| `rhapsody_contract.rs::GraphicsDevice(pub u64)` | 5 处 panic 桩 | `rhapsody::raw::device::GraphicsDevice`（Arc 化；`create`/`from_device` 返回 `Arc<Self>`） | **类型**：u64 占位 → `Arc<GraphicsDevice>`。RHAPSODY-DEEP-SUMMARY §6.1 明确：原资源是 `Arc<真实结构>` |
| `GraphicsDevice::create_persistent_descriptor_set(&self, _name:&str)` | panic 桩 | `GraphicsDevice::create_persistent_descriptor_set(&self, state:&DescriptorSetState, writes:&[DescriptorWrite]) -> Result<Arc<VulkanDescriptorSet>, vk::Result>`（device.rs:941） | **签名差异**：桩是 `(name)`，真是 `(state, writes)`；`DescriptorSetState`/`DescriptorWrite` 见 `raw::descriptor_set.rs`。消费端 `texture_library.rs:161` 需改造 |
| `GraphicsDevice::update_persistent_descriptor_set(&self)` | panic 桩 | （无此方法；`raw::device_api::bind_persistent_descriptor_set` 存在，但语义是 bind 不是 update） | **缺**（见 §4.2） |
| `GraphicsDevice::full_barrier_required(&self) -> bool` | panic 桩 | （无此方法；原版 `[device+0xaba]` 全屏障开关，device.rs 结构注释已记录布局但无公开 getter） | **缺**（见 §4.3） |
| `GraphicsDevice::issue_full_pipeline_barrier(&self, _command:u64)` | panic 桩 | `GraphicsDevice::issue_full_pipeline_barrier(&self, command: vk::CommandBuffer)`（device.rs:911） | **直改**：u64 → `vk::CommandBuffer` |
| `GraphicsDevice::dispatch(&self, _command:u64, _x,_y,_z)` | panic 桩 | 设备**无 dispatch 方法**；真实命令面 = `rg::pass_api::RenderPassApi::dispatch_groups_3d(&mut self, x,y,z)`（pass_api.rs:595）或 `Command::DispatchGroups{x,y,z}` 经 `CommandBackend::dispatch` | **删桩**：真实调用经 `RenderPassApi`/`CommandBackend`（RHAPSODY-DEEP-SUMMARY §6.1「dispatch 方法删除，原经 RenderPassApi::dispatch_groups_3d」） |
| `rhapsody_contract.rs::DescriptorSet`（unit） | 空结构 | `raw::descriptor_set::VulkanDescriptorSet`（`Arc`） | 真实持久 set = `Arc<VulkanDescriptorSet>` |
| `rhapsody_contract.rs::ManagedHandle(pub u64)` | u64 占位 | `raw::managed::handle::ManagedHandle<T: Send+Sync+'static>`（包装 `Managed<T>`）；`Managed<T>` Deref→T | **泛型化**；`ManagedTexture=Managed<Texture>`（managed/texture.rs:12）、`ManagedBuffer=Managed<Buffer>`（managed/buffer.rs:10） |
| `rhapsody_contract.rs::Handle<T>`（`get()`, `handle()`) | `get` panic | `rg::resource::Handle`（图内逻辑句柄 `{resource:ResourceHandle, descriptor}`）**与** `Managed<T>`（物理托管）是两个不同句柄 | **语义拆分**：`Handle<Texture>` 游戏侧 = `ManagedTexture`；`get()`→`Managed::resource()`/`ManagedRef::get_untracked`；`handle()`→`Managed::key()` |
| `rhapsody_contract.rs::ReadOnlyHandle<T>` | `get` panic | 原版**无 ReadOnlyHandle**（RHAPSODY-DEEP-SUMMARY §6.2）；真实跨帧只读面 = `rg::temporal::TemporalLookup`/`TemporalResource` | **缺/待取证**（见 §4.5） |
| `rhapsody_contract.rs::ResourceHandle(pub u64)` | u64 | `rg::resource::ResourceHandle(u32)`，`raw()` 取逻辑索引 | 值域收窄 u64→u32；`NONE` 哨兵对应图内未声明（原版用 u32 逻辑索引） |
| `rhapsody_contract.rs::Extent2D{width,height}` | 自建 | `ash::vk::Extent2D`（同形） | 别名/转发即可 |
| `rhapsody_contract.rs::RenderPassDesc{selector:u32}` | 占位 | `raw::render_pass::RenderPassDesc{colors: AttachmentArray, depth: Option<(RenderPassAttachmentDesc, DepthAccess)>}`（render_pass.rs:204，0x98B 缓存键） | **语义不同**：`{selector}` 是臆造；真实键 = 附件表。`RasterPipelineKey{ManagedHandle,u32}` 才是「渲染通道句柄 + 选项 u32」（render_pass.rs:202 注释已纠正旧文档） |
| `rhapsody_contract.rs::RenderPass`（unit） | 占位 | `raw::managed::render_pass::ManagedRenderPass`（`create_render_pass` 返回） | 见 device_api.rs:224 |
| `rhapsody_contract.rs::RenderPassAttachmentDesc`（unit+new） | 占位 | `raw::render_pass::RenderPassAttachmentDesc`（0x10B：format@0 clear_input@4 store_op@8 sample_count@0xc） | 直接替换；`new(vk::Format)` |
| `rhapsody_contract.rs::PassResourceSlot`（4 变体，resource:u32） | 占位 | `rg::binding::PassResourceSlot`（4 变体，`ResourceHandle` + buffer 带 offset/size，image 带 layout） | **签名差异**：resource:u32→`ResourceHandle`；缓冲槽加 `offset:u64,size:u64`；图像槽加 `layout: vk::ImageLayout` |
| `rhapsody_contract.rs::RenderPassBinding{resource_index, access_type, view}` + `bind(resource_index, access_type)` | 占位；`rg::pass_api::bind` 是注释里的函数名 | `rg::pass_bindings::PassBinding{slot: PassResourceSlot, view: Option<TextureViewDesc>, expected_key}`（pass_bindings.rs:64） | **`rg::pass_api::bind` 自由函数不存在**；真实构造 = `PassBinding::{buffer,image}`（pass_bindings.rs:70/89）。0x44B 布局 = 视图 0x3cB + 资源号 + 写标志的证据在 `rg/resource.rs::original_ref_view_binding`/`original_ref_buffer_binding` |
| `rhapsody_contract.rs::PassBuilder<'a,UD>`（`add_resource_access(u32,u8,u32)`、`resolve_resource(hash)`、`render(FnOnce(u64)->bool+'static)`） | 4 处 panic | `rg::pass_builder::PassBuilder<'graph>`（pass_builder.rs）：`add_resource_access(ResourceHandle,u8,u32)->Result<&mut Self,BuildError>`、`render(FnOnce(u64)->bool+Send+'static)`、`render_with_context(FnOnce(PassContext)->bool+Send)`、`rg_mut()`、`buffer_ref`/`texture_ref` | **签名差异**：resource u32→`ResourceHandle`；`render` 加 `Send`；`resolve_resource` 在真实里**不存在**（缺，见 §4.7） |
| `rhapsody_contract.rs::RenderGraph`（unit） | 占位 | `rg::graph::RenderGraph`（graph.rs:146） | 直接替换 |
| `rhapsody_types.rs::TextureDesc`（builder，kind:u8，format:u32） | 占位 | `raw::texture::TextureDesc`（`new(Extent3D, TextureKind, vk::Format)`；builder `sampled/storage/color_attachment/depth_stencil_attachment/mip_levels/array_elements/format/all_mip_levels/half_res`） | **替换 + 适配**：`kind:u8→TextureKind::from_raw`、`format:u8→vk::Format`、`extent:[u32;3]→vk::Extent3D`；`new_2d/new_2d_array/new_3d/extent_2d` 全存在。原 O 地址 0x140bd5270/5390/53d0/53f0/52a0/52e0/5310/5430/5790 与 raw::texture 注释逐一吻合 |
| `rhapsody_types.rs::TextureViewDesc`（0x3cB，Option<u32> 字段） | 占位 | `raw::texture::TextureViewDescRaw`（0x3cB，`OptionU32` 字段，`new`/`view_type/usage/format/aspect_mask/level_count/layer_count/base_mip_level/base_layer/bind_individual_mips`）+ `resolve(&TextureDesc)->TextureViewDesc` | **替换**：占位字段与 TextureViewDescRaw 逐字段同形；绑定/`get_vk_image_view` 用 `resolve()` 出的 `TextureViewDesc`（resolved，`vk::ImageViewType/Format/AspectFlags`） |
| `rhapsody_types.rs::Texture{id,width,height,format}`（`.handle()`） | 占位 | `raw::managed::texture::ManagedTexture`（= `Managed<Texture>`，`.image_view(desc)`/`.view(None)`，`Managed::key()`） | 游戏侧句柄 → `ManagedTexture` |
| `rhapsody_types.rs::BackendError(String)` | 占位 | `raw::error::Error`（枚举）+ `raw::error::Result<T>` | 替换 |
| `rhapsody_types.rs::CompiledShaderPipeline(Vec<u8>)` | 占位 | `rhapsody::shader_pipeline::compiled::CompiledShaderPipeline`（`decode(bytes)`/`load_compiled(path)`/`encode()`，`stages`/`reflection`） | 替换；原 O 0x130B 载荷/0x150B Arc 块见 compiled.rs |
| `rhapsody_types.rs::RasterPipelineDesc{path,entry,defines}`（0x48B） | 占位 | `rhapsody::shader_pipeline::pipeline_desc::RasterPipelineDesc`（`new_vertex_fragment`、`compiled_file_path`）+ `ComputePipelineDesc`（`new`、`with_entry`、`compiled_file_path`） | 替换；注意与 `raw::shader::RasterPipelineDesc`（Vulkan 固定状态）是**两个不同类型** |
| `rhapsody_types.rs::RasterPipelineKey{handle:u64, selector:u32}` | 占位 | 无此结构；selector = `raw::shader::RasterPipelineOpts(pub u32).0`（0x1416a3720 还原的 4B 位域，README 明示「同时是 RasterPipelineKey.selector」） | **游戏侧类型保留**；字段 2 映射到 RasterPipelineOpts 位域（topology/cull/depth/stencil/blend/polygon getter/setter 全存在） |
| `rhapsody_types.rs::RasterPipeline{id,compiled}` | 占位 | `raw::shader::VulkanPipeline`（`Arc`；compute/raster 两管线，`.bind_pipeline`） | 替换；缓存（shader.rs `HashMap<RasterPipelineKey, Box<RasterPipeline>>`）→ 缓存 `Arc<VulkanPipeline>` |
| `rhapsody_types.rs::ComputePipeline{id}` | 占位 | `raw::shader::VulkanPipeline`（`Arc`，compute bind point）+ `ShaderLibrary::create_compute_pipeline` | 替换 |
| `rhapsody_types.rs::ImportedBuffer{resource,generation,access}`（20B） | 占位 | `rg::graph::RenderGraph::import_unowned/import_owned -> Result<ResourceHandle, BuildError>`；`rg::import_export::ImportedBuffer`（20B `{size,desc_tail,resource_index}`）是**导入记录**不是句柄 | **重构**：游戏侧 `get_rg_buffer` 应存 `ResourceHandle`；`resource as u32` = `ResourceHandle::raw()`（= 图内逻辑索引）。旧注释 `+0xe=5 未导入判别` 已被 import_export.rs 证伪（`5` 是 `RegistryResource::Empty` 后备 tag，与 ImportedBuffer 无关） |
| `rhapsody_types.rs::BufferGraphBackend` trait + `RenderGraph`（buffer-graph recorder） | 占位 | `rg::graph::RenderGraph`（`create_raw_resource/add_pass/export/import_unowned/import_owned/batch_per_pass/build`）；无「BufferGraphBackend」trait | 重构：真实 import 需 `(name, Descriptor, token, initial_access)`，token = 物理托管句柄 |
| `rhapsody_types.rs::StorageBufferDescriptor` trait | 游戏侧 | 无 RH 对应（SSBO 访问经 `AssetSsboLibrary::get_rg_buffer` → 图导入；trait 仅注册名） | 保留为游戏侧接口 |
| `render/shader.rs::boundary::ShaderBackend`（5 方法） | 占位实现（`[cfg(not(test))]` 全 Err/panic） | 见 §2.2 逐方法映射 | 设备 trait 改为对真实 `GraphicsDevice` 实现 |
| `render/load_texture.rs::boundary::TextureDevice::create_texture(name,desc,data)->Texture` | 占位 | `GraphicsDevice::create_texture::<&[u8]>(&TextureDesc, name, data) -> Result<ManagedTexture>`（device_api.rs:151） | 返回 `ManagedTexture` |
| `render/ssbo.rs::rhapsody_boundary::GraphicsDevice`（create_buffer/update_buffer_bytes/mapped_slice_mut/device_address） | 占位 | `device_api.rs` 同名方法，参数为 `ManagedBuffer`；`Buffer`(40B)→`BufferRecord`/`ManagedBuffer`；`BufferDesc{size,usage,memory,alignment}`→`raw::buffer::BufferDesc{size:u64, usage:BufferUsageFlags, alignment:u16, memory_location:MemoryLocation}` | **签名差异**：见 §2.3；usage u32→`vk::BufferUsageFlags`、memory u8→`MemoryLocation` |
| `render/pass_builder/pass_resource_access.rs::RecordingPassBuilder` + `RenderGraph`（占位） | 占位 | `rg::pass_builder::PassBuilder` + `rg::graph::RenderGraph` | 重构（见 §2.4） |
| `render/rg_helpers.rs::rg_boundary::{RenderPassApi,RenderGraph}` | 占位 trait | `rg::pass_api::RenderPassApi`（`fill_buffer(Option<Range>,u32)`、`clear_color_image(image,mips,layers)`、`copy_buffer`）+ `rg::graph::RenderGraph::add_pass` | 见 §2.5；`rg_helpers` 的 `u32` 资源号→`ResourceHandle` |
| `render_world.rs::FramePasses::{get_or_create_temporal,get_or_create_temporal_pair,create_raw_resource}` | trait 方法 | `rg::temporal::{TemporalRenderGraphState, TemporalRenderGraphRecorder, GetOrCreateTemporal}`（键 `Ustr`）+ `rg::graph::RenderGraph::create_raw_resource` | 见 §2.6 |
| `render_frame.rs::RenderHost::{execute_main_graph, begin_frame, end_frame, clear_resource_pool, reset_temporal_tables, wait_idle_and_recycle_resources, …}` | trait 方法（注释标注原调用点） | `rg::executor::ExecutionChain` + `rg::graph::Pool` + `TemporalRenderGraphState` + `GraphicsDevice::wait_idle_and_recycle_resources` | **新增执行链**（见 §2.7） |

## 2. 调用契约

### 2.1 设备直连（device_api.rs / device.rs）

以下参数类型/数量/顺序为 rhapsody 实际签名，替换桩时按此接线：

- `GraphicsDevice::create_buffer<D: BufferInitialData>(&self, desc: &raw::buffer::BufferDesc, name: &str, data: D) -> raw::error::Result<ManagedBuffer>`（device_api.rs:71）。`D = () | &[u8] | Option<&[u8]>`。有数据时自动 `usage |= TRANSFER_DST`，断言 `data.len() <= desc.size`。
- `GraphicsDevice::create_texture<D: TextureInitialBytes>(&self, desc: &raw::texture::TextureDesc, name: &str, data: D) -> Result<ManagedTexture>`（device_api.rs:151）。断言 `desc.usage != empty`、extent 三维 > 0；有数据加 `TRANSFER_DST`。
- `GraphicsDevice::create_texture_with_data(&self, desc, name, data: setup::TextureInitialData) -> Result<ManagedTexture>`（device_api.rs:181）；`create_texture_zeroed(&self, desc, name)`（device_api.rs:205）。
- `GraphicsDevice::create_persistent_descriptor_set(&self, state: &DescriptorSetState, writes: &[DescriptorWrite]) -> Result<Arc<VulkanDescriptorSet>, vk::Result>`（device.rs:941）。
- `GraphicsDevice::get_vk_buffer(&self, buffer: &ManagedBuffer) -> Result<vk::Buffer>`（device_api.rs:325）。
- `GraphicsDevice::get_vk_image_view(&self, texture: &ManagedTexture, desc: &TextureViewDesc) -> Result<ManagedTextureView>`（device_api.rs:331）。默认视图用 `ManagedTexture::view(None)`（managed/texture.rs:26）。
- `GraphicsDevice::update_buffer_bytes(&self, buffer: &ManagedBuffer, data: &[u8], ranges: &[Range<usize>], offset: usize) -> Result<()>`（device_api.rs:348）。原版判定顺序 = `plan_buffer_update`（句柄三查 → 逐区间 → TRANSFER_DST → total>0），一律暂存 + setup CB 拷贝。
- `GraphicsDevice::buffer_mapped_slice_mut<'a>(&self, buffer: &'a ManagedBuffer) -> Option<&'a mut [u8]>`（device_api.rs:137）；`buffer_device_address(&self, buffer: &ManagedBuffer) -> Result<vk::DeviceAddress>`（device_api.rs:144）。
- `GraphicsDevice::create_render_pass(&self, attachments: &[RenderPassAttachmentDesc]) -> Result<ManagedRenderPass>`（device_api.rs:224）；`create_framebuffer(&self, pass: &ManagedRenderPass, views: Vec<Arc<TextureView>>, extent: vk::Extent2D, layers: u32) -> Result<ManagedFramebuffer>`（device_api.rs:247）。
- `GraphicsDevice::issue_full_pipeline_barrier(&self, command: vk::CommandBuffer)`（device.rs:911）；`issue_pipeline_barrier`/`issue_and_reset_pipeline_barrier(&self, command, scope: &SynchronizationScope)`（device.rs:882/900）。
- `GraphicsDevice::wait_idle_and_recycle_resources(&self) -> Result<(), vk::Result>`（device.rs:699）；`wait_idle`（device.rs:680）；`create_command_buffer(&self, family: u32, name: &str) -> Result<VulkanCommandBuffer>`（device.rs:719）。
- `GraphicsDevice::init_texture_data/update_texture(&self, texture: &raw::texture::Texture, bytes: &[u8]) -> Result<()>`（device.rs:785/792）。
- 纹理创建工厂：`raw::texture::Texture::new(device, &desc)`/`new_with_initial_data`；视图 `Texture::image_view(&Arc<Self>, &TextureViewDesc) -> Result<TextureView>`。

### 2.2 `render/shader.rs::ShaderBackend` 5 方法 → 真实调用

| ShaderBackend 方法（CC） | 真实接线（RH） |
|---|---|
| `load_compiled_compute(desc: &ComputePipelineDesc) -> CompiledShaderPipeline` | `shader_pipeline::compiled::CompiledShaderPipeline::load_compiled(desc.compiled_file_path())`（`ComputePipelineDesc::compiled_file_path()` 由 shader_uri 的 path 得出）。失败 → Err |
| `load_compiled_raster(desc: &RasterPipelineDesc)` | 同上，`RasterPipelineDesc::compiled_file_path()` |
| `raster_shader_variants(desc) -> [RasterPipelineDesc;2]` | **缺**：shader-pipeline 无 `shader_variants` 模块；原 0x1417388a0 `shader_variants::get_raster_shader_variants` 待取证（§4.9） |
| `create_compute_pipeline(name, compiled: Arc<CompiledShaderPipeline>) -> ComputePipeline` | `ShaderLibrary::register_compiled_compute(key, bytes)`（bytes = `compiled.encode()` 或原始容器）→ `ShaderLibrary::create_compute_pipeline(key, samplers: &BTreeMap<String, Arc<Sampler>>) -> Arc<VulkanPipeline>`（raw/shader.rs:1220）。注意原版「结果不检查」，真实返回 `Result`，调用点保持不检查语义需 `let _ =` |
| `create_raster_pipeline(compiled, key: &RasterPipelineKey) -> RasterPipeline` | `ShaderLibrary::prepare_raster_program(vertex_key, fragment_key, samplers) -> RasterProgram`（raw/shader.rs:1299）→ `RasterProgram::pipeline_desc(RasterPipelineDesc) -> RasterPipelineDesc`（raw/shader.rs:1182，填 vertex/fragment/layout/entry）→ 固定状态由 `key.selector`（=`RasterPipelineOpts`）的 `apply_to(&mut desc, color_attachments)`（raw/shader.rs:392）→ `VulkanPipeline::create_raster_pipeline(&cache, desc) -> Arc<VulkanPipeline>` |

### 2.3 `render/ssbo.rs::rhapsody_boundary` → 真实

- `create_buffer(&BufferDesc, name, data) -> Buffer`：CC `BufferDesc{size:usize, usage:u32, memory:u8, alignment:usize, gpu_only:bool}` → RH `raw::buffer::BufferDesc{size:u64, usage: vk::BufferUsageFlags, alignment:u16, memory_location: MemoryLocation}`；memory 1/2/4 → `MemoryLocation::{GpuOnly, CpuToGpu, GpuToCpu}`（raw/buffer.rs:13）。CC `usage::{TRANSFER_DST,UNIFORM_BUFFER,STORAGE_BUFFER,INDEX_BUFFER,VERTEX_BUFFER}` = `vk::BufferUsageFlags` 同名位（0x02/0x10/0x20/0x40/0x80，ssbo.rs 注释已证）。
- 返回：CC `Buffer`(40B) → `ManagedBuffer`；`byte_size` 读 `ManagedBuffer.size`（Deref→`raw::buffer::Buffer` `.size`），或 `Buffer::record(ctx,slot,offset,device_id) -> BufferRecord`（40B，raw/buffer.rs:104）。
- `update_buffer_bytes(&buffer, data, ranges, offset)`：直连 `device_api::update_buffer_bytes`（§2.1）。`gpu_write_ranges_nonoverlapping` 保持「字节区间换算」逻辑不变，仅 trait 实现换成真实设备。
- `mapped_slice_mut(&buffer) -> Option<&mut [u8]>` → `GraphicsDevice::buffer_mapped_slice_mut(&ManagedBuffer)`（device_api.rs:137）。
- `device_address(&buffer) -> u64` → `GraphicsDevice::buffer_device_address(&ManagedBuffer)`（device_api.rs:144）。
- `MappedStorageBuffer` 的映射区重解释逻辑保留（game 侧），仅 trait 设备参数换 `&rhapsody::raw::device::GraphicsDevice`。

### 2.4 `render/pass_builder/pass_resource_access.rs`

- `RecordingPassBuilder` 是 A1 层接缝，机器码显示 SSBO 访问直接走 `PassBuilder::add_resource_access`（O 0x1409dca90/0x140533b10），**不经注册表**（模块注释已纠正旧 `append_resource_binding`）。接线：`RecordingPassBuilder` 持有 `rg::pass_builder::PassBuilder`（或直接替换），`add_resource_access(resource: ResourceHandle, access, group)`；`render_graph` 字段 → `rg::graph::RenderGraph`。
- `PassBinding`（CC，4 变体 `BufferRead/Buffer/ImageView`）→ `rg::pass_bindings::PassBinding`（`PassResourceSlot` + `Option<TextureViewDesc>`）。标签证据：`original_ref_buffer_binding(logical, uniform)` 返回 `[3|4, logical]`（rg/resource.rs:224）对应 `BufferRead tag 3 / Buffer tag 4`；视图绑定 `original_ref_view_binding`（rg/resource.rs:213）对应 0x44B 布局。
- `TextureAccess::as_texture` 的 `get_vk_image_view(&ManagedHandle) -> u64` → `GraphicsDevice::get_vk_image_view(&ManagedTexture, &TextureViewDesc)`（§2.1），`managed_texture_desc` → `ManagedTexture.desc`（Deref 后读 `Texture.desc: raw::texture::TextureDesc`）。`TextureBinding::{Rg,Managed,ByName}` 枚举保留（游戏侧），其 `desc: TextureDesc` 换 `raw::texture::TextureDesc`。

### 2.5 `render/rg_helpers.rs`

- `rg_boundary::RenderGraph::add_pass(name, resource:u32, access, render)` → 真实：`graph.add_pass(name).add_resource_access(ResourceHandle, access, group)` + `.render_with_context(closure)`；闭包内 `RenderPassApi::fill_buffer(buffer: vk::Buffer, range: Option<(u64,u64)>, data: u32)`（pass_api.rs:455）、`clear_color_image(image: vk::Image, mip_levels: u8, array_layers: u32)`（pass_api.rs:480）、`copy_buffer(source: vk::Buffer, destination: vk::Buffer, regions: &[vk::BufferCopy])`（pass_api.rs:440）。
- `ACCESS_TRANSFER_WRITE = 0x34`、`DynamicConstants::allocate(size)` 对应 `raw::dynamic_constants`（rg_helpers 的 `StagingAlloc{buffer,offset,bytes}` 是游戏侧暂存视图；真实动态常量 `raw::dynamic_constants::DynamicConstantsChunk`）。

### 2.6 `render_world.rs::FramePasses` 的 rhapsody 三入口

- `get_or_create_temporal(name, &TextureDesc) -> TemporalTexture` → `rg::temporal::TemporalRenderGraphRecorder::get_or_create_temporal(&mut self, name: &str, descriptor: Descriptor, create: impl FnOnce() -> Result<u64, BuildError>) -> Result<TemporalLookup, TemporalError>`（temporal.rs:426，trait `GetOrCreateTemporal`）。`Descriptor` = `rg::graph::Descriptor::Image(rg::graph::TextureDesc)`（经 `rg::resource::descriptor_from_texture` 从 `raw::texture::TextureDesc` 转）；`create` 闭包 = `GraphicsDevice::create_texture` 返回 `ManagedTexture` 的 token。原 O 0x1404f6e50/0x1404f7450 与 temporal.rs 注释吻合。
- `create_raw_resource(name, &TextureDesc) -> u32` → `rg::graph::RenderGraph::create_raw_resource(name, ResourceOrigin{descriptor: Descriptor::Image(..), source}) -> ResourceHandle`（graph.rs:163；`create_resource_handle` 同实现）。原 O 0x140be94d0。
- `render_world` 里 `TextureDesc`（render_world.rs 自带 32B）→ `raw::texture::TextureDesc`（字段布局已对齐）。

### 2.7 `render_frame.rs::RenderHost` → ExecutionChain（本轮核心新增）

`render_frame.rs::render` 的 O 调用点（0x1404e8010，模块头注释逐一标注）映射：

| RenderHost 方法 | 真实接线 |
|---|---|
| `compile_world_graph` / `compile_loading_graph` | 构图：`rg::graph::RenderGraph::new()` → `create_raw_resource`/`import_unowned`/`add_pass(name).add_resource_access(..).render_with_context(..)`/`export(handle, access)`/`batch_per_pass(enabled)` → `build() -> BuiltGraph` → `into_parts() -> (GraphDeclaration, Vec<RenderCallback>, Vec<RetireCallback>)`。OOM 降分辨率语义在 graph.rs `compile_graph`（population/批次）与 `ExecutionChain::execute_graph_inner`（OOM 返回 `Error::ResourceUnavailable`） |
| `execute_main_graph(compiled)` | `ExecutionChain::execute_graph_with_context(&mut self, graph: GraphDeclaration, pool: &mut Pool, dependency: &mut impl AccessDependency, backend: &mut impl ExecutionBackend, callbacks: &mut [impl FnMut(PassContext<'_>) -> bool + Send]) -> Result<GraphExecution, Error>`（executor.rs:277）。`callbacks.len() == graph.passes.len()`（否则 `Error::CallbackCount`）。`PassContext{pass_index, command, compiled, logical_resources, managed, owners}`；`command_state()`/`keep_alive()`（executor.rs:239）供 pass 回调取真实 `CommandState` 与托管注册表 |
| `begin_frame` | `ExecutionChain::new(auxiliary_count, family_count, device: submit::DeviceState)`（executor.rs:614）；每次执行 `execute_graph_with_context` 即完成 begin/record/submit 链 |
| `end_frame` | `PassPgoFrames::end_frame()`（executor.rs:1147；`execute_graph_with_context` 末尾已自动调用）；`complete(queue, timeline)`（executor.rs:1002）推进完成回收 |
| `execute_presentation_graph` | 另一 `GraphDeclaration`（展示图）+ `execute_graph_with_context`；`ExportedRenderGraphResources::take_resource(logical)` 取回导出 swapchain 资源 |
| `clear_resource_pool` | `rg::graph::Pool`/`rg::resource_pool::ResourcePool::clear`（原 O 0x140be76f0） |
| `reset_temporal_tables` | `TemporalRenderGraphState::clear()`（temporal.rs:178）——换带新随机种子的空表 |
| `wait_idle_and_recycle_resources` | `GraphicsDevice::wait_idle_and_recycle_resources()`（device.rs:699） |
| `last_gpu_frame_ns` | `GraphExecution::stats`（compile/record/submit_ns）或 `ExecutionStats` 投影（原版 `GraphExecutor::last_known_frame_duration_ns` 实返 bool+ns，RHAPSODY-DEEP-SUMMARY §3.6 已纠偏） |
| `wait_gpu_queue_trim(wait)` | `GraphicsDevice::wait_submit_done(fence, timeout)`（device.rs:925）或完成侧等待；**待取证**（GPU 后置） |
| `acquire_swapchain_image`/`present`/`swapchain_info` | `raw::swapchain`（见 §4.10 缺口） |

真实后端：`rg::vulkan::VulkanGraphBackend`（vulkan.rs:166 `GpuResource`、`VulkanGraphBackend` 实现 `ExecutionBackend`+`ResourceBackend`；`pass_bindings::ManagedPassResources::from_registry` 用 `resolve(token) -> Option<GpuResource>` 接物理对象）。这是「主图 provider 桥」候选边界（README 已登记）。

## 3. 桩替换计划（rhapsody_contract.rs 5 处 + rhapsody_types.rs + render/*）

按协调简报 §D 规则 3：先建编译基线的等价桥，保持 nani/pass_builder/render_loop 上层签名不破坏，再逐层替换。

### 3.1 `rhapsody_contract.rs` 5 处 panic 桩

| # | 桩 | 替换动作 |
|---|---|---|
| 1 | `create_persistent_descriptor_set(&self, name)` | 设备类型改 `Arc<rhapsody::raw::device::GraphicsDevice>` 后，方法签名改真实 `(&state: &DescriptorSetState, writes: &[DescriptorWrite]) -> Result<Arc<VulkanDescriptorSet>, vk::Result>`；游戏侧需持有 bindless `DescriptorSetState`（§4.1） |
| 2 | `update_persistent_descriptor_set(&self)` | **登记缺口**，不补造；临时在桥内保留 no-op/记录（标注 `#[cfg(test)]` 语义保留，非 test 路径登记缺口，不 panic 不假成功——按验收纪律「禁止伪造成功」，改返回未接日志） |
| 3 | `full_barrier_required() -> bool` | **登记缺口**（§4.3）；桥内返回 `false` 并登记，不得当真实完成 |
| 4 | `issue_full_pipeline_barrier(&self, command: u64)` | 直改 `issue_full_pipeline_barrier(&self, command: vk::CommandBuffer)`（device.rs:911）；`compute_pass_builder::ComputeDispatchApi::issue_full_pipeline_barrier` 由此接真实设备 |
| 5 | `dispatch(&self, command, x, y, z)` | **删桩**：`ComputeDispatchApi::cmd_dispatch(x,y,z)`（compute_pass_builder.rs:211）接到 `rg::pass_api::RenderPassApi::dispatch_groups_3d(x,y,z)`；真实命令缓冲经 `PassContext.command.handle()`（`vk::CommandBuffer::from_raw`）构造 `AshCommandBackend`（pass_api.rs:175）。`full_barrier_enabled`（`[device+0xaba]`）见 §4.3 |

### 3.2 `rhapsody_types.rs` 本地类型 → rhapsody 别名/转发

- 删除本地定义，改为 `pub use rhapsody::…` 转发 + 必要适配函数：
  - `TextureDesc` → `pub use rhapsody::raw::texture::TextureDesc;`。**适配**：全 crate 里 `TextureDesc::new([w,h,d], kind_u8, format_u8)` 调用点 → `TextureDesc::new(vk::Extent3D{..}, TextureKind::from_raw(kind).unwrap(), vk::Format::from_raw(format))`；`.with_format(u32)` → `.format(vk::Format::from_raw(u32))`；`new_2d([w,h], format_u32)` → `new_2d(w, h, vk::Format)`；`new_2d_array([w,h], layers, format)` → `new_2d_array(w, h, layers, vk::Format)`；`new_3d([w,h,d], format)` → `new_3d(w,h,d,vk::Format)`。`extent_2d() -> [u32;2]` → `vk::Extent2D{..}`（CC 的 `Extent2D` 保留为转发 `vk::Extent2D`）。
  - `TextureViewDesc` → `pub use rhapsody::raw::texture::TextureViewDescRaw;`（builder 同名，字段 `Option<u32>` 同形；`level_count/layer_count` 参数为 `Option<u32>`）。绑定时用 `resolve(&texture_desc) -> TextureViewDesc`。
  - `Texture` → `pub use rhapsody::raw::managed::texture::ManagedTexture;`（`.handle()` 调用点改 `Managed::key()`；`id/width/height/format` 访问点改 Deref 后 `Texture.desc` 字段）。
  - `CompiledShaderPipeline` → `pub use rhapsody::shader_pipeline::compiled::CompiledShaderPipeline;`。
  - `RasterPipelineDesc` → `pub use rhapsody::shader_pipeline::pipeline_desc::RasterPipelineDesc;`（path/entry 语义同占位；`defines` 字段在 pipeline_desc 版没有——原 0x48B `{path,entry,defines}` 的 defines 是**编译期特性**，真实在 `shader_compiler`/`CompiledShaderPipeline` 内，**登记差异**：占位 `defines` 不能直接透传，需经 `ComputePipelineDesc` 的编译变体机制——见 §4.11）。
  - `RasterPipelineKey` 保留（游戏侧缓存键）；`selector` 与 `RasterPipelineOpts(pub u32)` 互转（`RasterPipelineOpts::apply_to` 已还原全部位域）。
  - `RasterPipeline`/`ComputePipeline` → 桥内 `type RasterPipeline = Arc<rhapsody::raw::shader::VulkanPipeline>`（游戏侧 `.id`/`.compiled` 访问点改写）。
  - `BackendError`/`Error` → `pub use rhapsody::raw::error::Error;`。
  - `ImportedBuffer`/`BufferGraphBackend`/`RenderGraph`（rhapsody_types.rs 尾部）→ `rg::graph::{RenderGraph, ResourceHandle, Descriptor}`；`get_rg_buffer` 返回 `ResourceHandle`（§3.4）。
- `StorageBufferDescriptor` trait 保留（游戏侧 SSBO 注册名接口）。

### 3.3 `render/*` 上层签名保持

- `render/shader.rs`：`ShaderBackend` trait 的**5 方法签名本身不动**（它是接缝），只把 `[cfg(not(test))] impl ShaderBackend for GraphicsDevice` 换成真实实现（§2.2）。`RasterShaderProgramVariant::cache: HashMap<RasterPipelineKey, Box<RasterPipeline>>` 的 key/值类型随转发后类型自动收窄；`Arc::clone(&self.compiled)` 继续有效（CompiledShaderPipeline 仍是 Arc 载荷）。
- `render/load_texture.rs`：`TextureDevice::create_texture` 签名保持，`[cfg(not(test))]` 实现换真实 `GraphicsDevice::create_texture::<&[u8]>`，返回 `ManagedTexture`；`parse_compiled_image` 构造的 `TextureDesc` 换真实类型后调用 `create_texture`。
- `render/ssbo.rs`：`rhapsody_boundary::GraphicsDevice` trait 签名**保持**（它是接缝），把 `[cfg(not(test))] impl` 换成对 `Arc<rhapsody::raw::device::GraphicsDevice>` 的适配（§2.3）；`Buffer`(40B)/`BufferDesc` 就地替换为 `ManagedBuffer`/`raw::buffer::BufferDesc` 的别名或适配结构，`TypedStorageBuffer<T>`/`MappedStorageBuffer`/`InstanceBuffer` 的字段与 `byte_size()` 读法改 `ManagedBuffer.size`。
- `render/pass_builder/*`：`RecordingPassBuilder` 内部替换为 `rg::pass_builder::PassBuilder` + `rg::graph::RenderGraph`；`PassBinding`/`ResourceAccess` 记录的构造与 `to_contract`/`tag()` 测试适配（§2.4）。`ComputePipelineDesc`（shader_uri.rs 占位 `{path,entry,defines}`）→ `shader_pipeline::pipeline_desc::ComputePipelineDesc`（`new`/`with_entry` 存在；`defines` 差异见 §4.11）。`ComputeDispatchApi`（compute_pass_builder.rs:202-211）→ 接 `RenderPassApi` + `GraphicsDevice`（§3.1 #5）。
- `render/rg_helpers.rs`：`rg_boundary::RenderGraph`/`RenderPassApi` trait 保持形状，`[cfg(not(test))]` 接线换真实（§2.5）。
- `render_world.rs`：`FramePasses` trait 保持（它是缝），真实实现接 §2.6；`TextureDesc`/`RgTexture` 换 `raw::texture::TextureDesc`（`RgTexture{id:u32, desc}` 的 id 即 `ResourceHandle::raw()`）。
- `render_frame.rs`：`RenderHost` trait 保持（它是缝），真实实现接 §2.7 ExecutionChain。
- `render/nani/*`、`render/mesh_atlas.rs`、`render/sparse_instance_buffer.rs`、`render/vao.rs`：消费 `ssbo::rhapsody_boundary::GraphicsDevice`/`rhapsody_contract::GraphicsDevice` 的地方随 §3.3 自动生效；`nani::trimesh` 的 `device_address` 由 §2.3 覆盖。

### 3.4 `asset_libraries/*` 渲染侧

- `ssbo_library.rs`：`get_rg_buffer<T>(&self, rg: &mut RenderGraph)` 改真实构图：构造 `rg::graph::Descriptor::Buffer(rg::graph::BufferDesc)`（16B ABI，经 `rg::resource::descriptor_from_buffer`）→ `rg.import_unowned(name, descriptor, token, 0x37)`（O：`mov r9b, 0x37`）→ `Result<ResourceHandle>`。`export_from_rg` → `rg.export(handle, 0x37)`。`ImportedBuffer`/`BufferState` 的 `+0x28` 句柄槽改存 `ResourceHandle`。
- `texture_library.rs`：`TextureLibraryDevice`（`update_texture`/`get_vk_image_view`/`update_persistent_descriptor_set`/`create_persistent_descriptor_set`）→ 按 §2.1/§4.1/§4.2 实现；`TextureRenderGraph::import_unowned_texture` → `rg.import_unowned(name, Descriptor::Image(..), token, access)`（O 0x140bd60a0）；`get_rg_texture` 返回 36B `{id:u32, desc}` 的 desc 换 `raw::texture::TextureDesc`。
- `shader_library.rs`（asset_libraries）：`RasterPipelineDesc`/`RasterPipelineKey` 随转发类型；`create_raster_pipeline` 调用点接 §2.2。
- `vao_library.rs`/`color_library.rs`/`placement_data_library.rs`：走 `ssbo::rhapsody_boundary::GraphicsDevice`，随 §3.3 自动生效（`vao` 的 `BufferDesc::new_gpu_only` 对应 `raw::buffer::BufferDesc::new_gpu_only`）。

## 4. 登记缺口（country-core 需要但 rhapsody **不存在**）

按规则不往 rhapsody 补造，登记待取证或后续工作：

1. **`GraphicsDevice::create_persistent_descriptor_set(name)`（一参）**：真实为 `(state: &DescriptorSetState, writes: &[DescriptorWrite])`。游戏侧缺 bindless `DescriptorSetState` 持有者（texture_library.rs:161 是唯一消费点）。待取证：原 bindless 池规划（`raw::descriptor_set::persistent_pool_plan` 存在，但游戏侧 state 构造点未恢复）。
2. **`GraphicsDevice::update_persistent_descriptor_set`**：rhapsody 只有 `bind_persistent_descriptor_set`（device_api.rs:302），无 update 方法身份。原 O 0x1416a26aa（`descriptor_update_count` 写入点，device.rs:42 已记录）。待取证。
3. **`full_barrier_required` / `[device+0xaba]` 全屏障开关**：无公开 getter；原字段布局已记录（device.rs 结构注释）。rhapsody 本体不可改；游戏侧要么登记为「保留 false 桥 + 待取证」，要么等 rhapsody-rebuild 链补 getter 后再接。
4. **`GraphicsDevice::dispatch`**：不存在；真实路径 `RenderPassApi::dispatch_groups_3d`（已映射，非缺口，但 `ComputeDispatchApi` 桩需删改）。
5. **`ReadOnlyHandle<T>`**：原版无此类型（RHAPSODY-DEEP-SUMMARY §6.2）；`TextureAccess for &ReadOnlyHandle<Texture>` 的语义待取证（原跨帧只读句柄形态 = `TemporalLookup`/`ResourceHandle`）。
6. **`rg::pass_api::bind(resource_index, access_type)` 自由函数**：不存在；用 `PassBinding::{buffer,image}` + `PassBuilder::add_resource_access`（已映射）。合同注释里的函数名是 PDB 名，非真实自由函数。
7. **`PassBuilder::resolve_resource(hash) -> (u32,u32)`**：真实 PassBuilder 无此方法（哈希→(set,resource) 的 typed gap 原版存在但 rhapsody 未恢复）。待取证。
8. **`Buffer` 40B 契约（D1，RHAPSODY-DEEP-SUMMARY §2.2）**：游戏侧 `TypedStorageBuffer`/`MappedStorageBuffer` 用 `BufferRecord`(40B)；rhapsody 内部是 `Managed<Buffer>`，两者经 `Buffer::record(ctx,slot,offset,device_id)` 转换（已存在），但游戏侧槽表/ctx/slot 语义未恢复。待取证。
9. **`shader_variants::get_raster_shader_variants`**：shader-pipeline 无此模块；原 O 0x1417388a0（两份 desc）。待取证：两份变体（如 photomode / 常规）的生成规则。
10. **swapchain 获取/展示**（`validate_swapchain_and_acquire_image`、`TrackedSwapchainTextureLease::into_present_source`、`present`）：`raw::swapchain` 存在但游戏侧租约/展示入口未接；GPU 真机执行后置（协调简报 §D 规则 4），登记后续。
11. **`RasterPipelineDesc.defines`（0x48B 编译特性 Vec）**：shader-pipeline `pipeline_desc::RasterPipelineDesc` 无 defines 字段；原 defines 哈希算法（404 .bin 无复现，RHAPSODY-DEEP-SUMMARY §4 crates）待取证；`ComputeShaderProgram::new` 的 `PERM_*` 变体需经 `shader_compiler`（preprocess/pack）路径，占位 `defines` 不能直接透传。
12. **`GraphExecutor::last_known_frame_duration_ns` 返回形态**：`ExecutionStats` 已覆盖 compile/record/submit_ns，但原版返回 `(bool, u64)`（RHAPSODY-DEEP-SUMMARY §3.6），游戏侧 `last_gpu_frame_ns` 的接线映射待取证。
13. **`ImportedBuffer` 旧注释 `+0xe=5 未导入判别`**：已由 import_export.rs 证伪（`5` = 后备资源 `Empty` tag），不引入该判别（登记防误接）。
14. **`temporal` 键**：真实用 `Ustr`（与 country-core `ustr_contract` 意图一致），`get_or_create_temporal` 的 `create` 闭包需提供真实 token 生产者（ManagedTexture 的 key/token 映射），游戏侧未恢复。
15. **`GraphicsDevice::get_vk_image_view(handle)`（一参，视图=None）**：真实二参 `(&ManagedTexture, &TextureViewDesc)`；`ManagedTexture::view(None)`（managed/texture.rs:26）是默认视图形态，`pass_resource_access.rs` 的 `TextureDevice::get_vk_image_view(&ManagedHandle)` trait 需改二参并保留默认视图路径。
16. **`RenderPassDesc{selector}` 桩语义**：真实 `RenderPassDesc`（附件表键）与 `RasterPipelineKey`（句柄+selector）是两个不同键；`rhapsody_contract.rs` 的 `RenderPassDesc{selector:u32}` 是臆造，删改时无对应真实单字段（§1 已映射）。

## 5. Cargo 依赖方案

`crates/country-core/Cargo.toml`：

```toml
[dependencies]
rhapsody.workspace = true
```

### 5.1 编译影响预判

- **新增传递依赖**：rhapsody 依赖 `spirq, ustr, rspirv, shader-pipeline, shader-compiler, arc-swap, append-only-vec, bevy_tasks, gpu-allocator`（`suballocation` 默认开启）＋已有 `ash/serde/serde_json/ron/parking_lot/log`。country-core 已有 `ash/serde/parking_lot/log`（workspace 同版本），不冲突。
- **feature 冲突**：`rhapsody` 默认 `suballocation` 拉 `gpu-allocator = 0.27`（vulkan）。country-core 无 gpu-allocator；无其它 crate 依赖 gpu-allocator，无版本冲突。若需要 country-core 不启用子分配，用 `rhapsody = { workspace = true, default-features = false }`（但会偏离原版「默认子分配」语义，不建议）。
- **ash 版本**：country-core 已 `ash.workspace = true`，rhapsody 同 workspace ash，二者共享一份，`vk::CommandBuffer`/`vk::Extent2D` 等类型同一来源，跨 crate 互操作无需转换。
- **bevy 版本**：country-core `bevy_ecs/bevy_time = 0.16`，rhapsody 用 `bevy_tasks`（同 0.16 family，Bevy 分叉约束），无版本冲突。
- **编译量**：rhapsody 是编译期较重 crate（ash + spirq + rspirv + gpu-allocator），`cargo check -p country-core` 首次会显著变慢；不引入 GPU 头文件/链接（纯 Rust）。
- **命名冲突**：`rhapsody` crate 名 vs country-core `rhapsody_contract`/`rhapsody_types` 模块名——Rust 2018 下 `use rhapsody::…` 指向 extern crate，与 `crate::rhapsody_types` 路径不冲突（country-core 无顶层 `rhapsody` 模块）。
- **旧差分报告**：接入后 `rhapsody_types.rs`/`rhapsody_contract.rs`/`render/*` 改动会标记旧差分报告过期，需按 AGENTS.md 重新校验后再更新进度。

### 5.2 建议实施顺序（实时复合 agent）

1. 先加 `rhapsody.workspace = true`，保持现有桩编译通过（基线）。
2. 建等价桥：`rhapsody_types.rs` 改为转发别名（§3.2），一次性过 `cargo check`。
3. 逐层替换设备 trait 实现（§3.3 顺序：load_texture → ssbo → shader → pass_resource_access → rg_helpers → asset_libraries）。
4. 最后接 ExecutionChain（§2.7：render_frame `RenderHost` 真实实现 + `rg::vulkan::VulkanGraphBackend`）。GPU 真机不验收（§D 规则 4），验收 = 真实调用接线 + `cargo check` + 原侧入口核对。

## 6. 证据入口汇总

- 原调用关系：`evidence/render-recovery/{RHAPSODY_PROGRESS.md, BATCHED_RHAPSODY_RECOVERY_PLAN.md, WORKFLOW_RESET.md}`、`country-core-deep/rhapsody/RHAPSODY-DEEP-SUMMARY.md`（§6 接线建议、§2.2 D1-D10 偏差表）、`render-wp 工作包`参考。
- rhapsody 公开面：`crates/rhapsody/src/{raw/device_api.rs, raw/device.rs, raw/texture.rs, raw/buffer.rs, raw/shader.rs, raw/descriptor_set.rs, raw/render_pass.rs, raw/managed/{handle,texture,buffer,resources}.rs, rg/{graph,executor,pass_builder,pass_api,pass_bindings,binding,import_export,temporal,resource,resource_registry,vulkan}.rs}`、`crates/rhapsody/crates/shader-pipeline/src/{compiled.rs,pipeline_desc.rs}`。
- 占位现状：`crates/country-core/src/{rhapsody_contract.rs, rhapsody_types.rs, render/*, systems/render_loop/*, asset_libraries/*}`。
