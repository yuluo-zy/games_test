# D 组（rhapsody 接入）交付 — rhapsody-DELIVERY

日期：2026-10-16。实现 agent：D 组实时复合。契约：`D-rhapsody/rhapsody-WIRING.md`。范围：country-core 渲染层占位/桩 → `crates/rhapsody` 真实公开 API 接线。

**验收基准**：`cargo check -p country-core`。本文档逐项分离「真实接线 / candidate 链测试 / 原侧验证 / 仍缺项」，不以编译通过冒充原侧验证。

---

## 0. 前提与并发现象（重要）

- 本轮期间工作树被 A/B/C 组并行改动（`system-roof` Cargo.toml 曾临时加入 `country-core.workspace = true` 造成 package 循环依赖；roof 重构把 `tiles/surface/ridge/edit_rules` 移出 country-core；save/audio/sheep/ivy 各有在制改动）。
- 最终 `cargo check -p country-core` 的 13 个错误**全部落在并发 agent 在制的文件**（`resources/roofs/geometry/*`、`systems/save/*`、`systems/audio/music/*`、`systems/debug/sheep_controls.rs`、`systems/ivy/ivy_grower.rs` 等），**无一个在 D 组文件内**。D 组 8 个改动文件自身编译干净。
- 循环依赖已由并发 agent 移除 `country-core -> system-roof` 依赖解决（country-core 源码只注释引用 `system_roof`，无代码引用，见 `resources/roofs/*` 注释）。

## 1. 真实接线（已落地，编译通过）

### 1.1 Cargo
- `crates/country-core/Cargo.toml` 加 `rhapsody.workspace = true`。rhapsody 传递依赖（spirq/ustr/rspirv/shader-pipeline/shader-compiler/gpu-allocator/bevy_tasks 等）与 country-core 已有 `ash/serde/parking_lot/log` 无版本冲突；ash 同 workspace 版本，`vk::CommandBuffer`/`vk::Extent2D` 同一来源。**基线 + 接入后均无 feature 冲突**。

### 1.2 `rhapsody_contract.rs`（WIRING §3.1 五处桩）
- `GraphicsDevice`：`pub u64` 占位 → **真实包装** `struct GraphicsDevice { inner: Arc<rhapsody::raw::device::GraphicsDevice> }`，`new(inner)` / `inner()`。
- **`create_persistent_descriptor_set`**：签名改真实 `(&DescriptorSetState, &[DescriptorWrite]) -> Result<Arc<VulkanDescriptorSet>, vk::Result>`（device.rs:941），内部 `self.inner.create_persistent_descriptor_set(state, writes)`。游戏侧 bindless 状态未恢复 → 桥 `bindless_create_persistent_descriptor_set(name)` 登记缺口 §4.1（日志 + 占位，不 panic 不假成功）。
- **`issue_full_pipeline_barrier`**：`u64` → `vk::CommandBuffer`，内部 `self.inner.issue_full_pipeline_barrier(command)`（device.rs:911）。
- **`dispatch`**：**删桩**（全 crate 无调用点）。真实命令面 = `rg::pass_api::RenderPassApi::dispatch_groups_3d(x,y,z)`（pass_api.rs:595），接线见 §1.5。
- **`update_persistent_descriptor_set`**：**登记缺口 §4.2**（rhapsody 无此方法身份，只有 bind 语义的 `bind_persistent_descriptor_set`）；桥内只记日志。
- **`full_barrier_required`**：**登记缺口 §4.3**（`[device+0xaba]` 全屏障开关无公开 getter）；桥内返回 `false` 并记日志。
- 另接真实设备方法（WIRING §2.1 全部存在者）：`issue_pipeline_barrier`、`issue_and_reset_pipeline_barrier`、`wait_idle`、`wait_idle_and_recycle_resources`、`create_command_buffer`、`get_built_in_sampler_by_name`、`max_bindless_descriptor_count`、`refresh_memory_budget`、`create_buffer`、`create_texture`、`get_vk_image_view`、`get_vk_buffer`、`update_buffer_bytes`、`buffer_mapped_slice_mut`、`buffer_device_address`、`create_render_pass`、`create_framebuffer`（device_api.rs / device.rs 同名真实实现）。

### 1.3 `rhapsody_types.rs`（WIRING §3.2 转发层）
- 新增「rhapsody 真实面」转发：`RhapsodyTextureDesc`/`RhapsodyTextureViewDescRaw`/`ManagedTexture`/`ManagedBuffer`/`RhapsodyGraphicsDevice`/`RhapsodyVulkanDevice`/`RhapsodyError`/`RhapsodyBufferDesc`/`RhapsodyShaderLibrary`/`RhapsodyVulkanPipeline`/`RhapsodyVulkanDescriptorSet`/`RhapsodyCompiledShaderPipeline`/`RhapsodyComputePipelineDesc`/`RhapsodyRasterPipelineDesc`/`ShaderDefines`/`RgDescriptor`/`RgResourceHandle`/`RhapsodyRenderGraph`/`ExecutionChain`/`ExecutionBackend`/`PassContext`/`GraphExecution`/`RhapsodyPassBuilder`/`RenderPassApi`/`AshCommandBackend`/`RhapsodyVulkanGraphBackend`/`AccessTable`/`GpuResource`/`RgPool`/`RhapsodyDeviceState`/`RhapsodyQueueState` 等。
- 适配函数：`convert_texture_desc`（游戏侧 builder → 真实 32B，`TextureKind::from_raw` + `vk::Format::from_raw`）、`texture_desc_to_rg_descriptor`（`RgResourceSpec::Image(desc).descriptor()`）、`convert_texture_view_desc`（`Option<u32>` → `OptionU32`）、`decode_compiled_pipeline`（speedy `decode`）。
- 游戏侧占位类型保留（编译桥），逐项映射登记在 WIRING §1。
- `ImportedBuffer` 注释按 **WIRING §4.13** 更新：`+0xe=5` 判别证伪（`5` = `RegistryResource::Empty` tag，graph.rs:749）。

### 1.4 `render/shader.rs`（WIRING §2.2/§3.3）
- `ShaderBackend for GraphicsDevice` 的 `#[cfg(not(test))]` 实现：
  - `load_compiled_compute` / `load_compiled_raster` → **真实加载**：游戏侧 desc → 真实 `ComputePipelineDesc`/`RasterPipelineDesc`（`ShaderDefines` 承载 `defines`）→ `RhapsodyCompiledShaderPipeline::load_compiled(real.compiled_file_path())`（读发行 .bin，speedy 解码）→ `encode()` → 游戏侧字节容器。
  - `raster_shader_variants` → 登记缺口 §4.9（shader-pipeline 无该模块；O 0x1417388a0 待取证）。
  - `create_compute_pipeline`/`create_raster_pipeline` → 登记缺口 §2.2（真实路径 `ShaderLibrary::register_compiled_compute`→`create_compute_pipeline`/`prepare_raster_program`→`pipeline_desc`→`apply_to`；游戏侧 `ComputePipeline`/`RasterPipeline` 占位装不下 `Arc<VulkanPipeline>`，§4.8 类型缝）。

### 1.5 `render/execution_host.rs`（WIRING §2.7 ExecutionChain，本轮核心新增）
- 新模块 `render/execution_host.rs`（`render/mod.rs` 已登记）：
  - `RhapsodyExecutionHost::new(&GraphicsDevice)`：`VulkanGraphBackend::new(device, AccessTable::original())` + `ExecutionChain::new(auxiliary_count, family_count, DeviceState{primary queue + timeline})`。
  - `execute_graph(GraphDeclaration, callbacks)`：`chain.execute_graph_with_context(graph, &mut pool, &mut access, &mut backend, callbacks)`（executor.rs:277；`callbacks.len()==graph.passes.len()`，否则 `Error::CallbackCount`）。
  - `complete_primary()`：`timeline.completed()` → `chain.complete(Queue::Primary, Timeline(v))`。
  - `wait_idle_and_recycle_resources()`：`device.wait_idle_and_recycle_resources()`。
  - `new_render_graph()`：`rg::graph::RenderGraph::new()`（构图入口：`create_raw_resource`/`add_pass(..).render_with_context(..)`/`export(..)`/`batch_per_pass(..)`/`build()`→`into_parts()`）。
  - `pass_api_from_context(backend, ctx)`：`vk::CommandBuffer::from_raw(ctx.command.handle())` + `AshCommandBackend` 构造真实 `RenderPassApi`（可调 `dispatch_groups_3d`，WIRING §3.1 #5）。
  - `last_frame_ns(&GraphExecution)`：`stats` 投影（WIRING §2.7 `last_gpu_frame_ns`）。

### 1.6 `render/ssbo.rs` / `render/load_texture.rs`（WIRING §2.3/§3.3）
- `ssbo`：`#[cfg(not(test))] impl rhapsody_boundary::GraphicsDevice for GraphicsDevice` 改为**设备感知缺口桥**：`convert_buffer_desc`（游戏侧 `{size,usage,memory,alignment}` → 真实 `raw::buffer::BufferDesc`，memory 1/2/3/4 → `MemoryLocation`）真实换算；四个方法记录日志并返回未接（登记缺口 §4.8：游戏侧 40B `Buffer` 装不下 `ManagedBuffer`）。
- `load_texture`：`#[cfg(not(test))] impl TextureDevice for GraphicsDevice` 改为缺口桥：`convert_texture_desc` 真实换算 desc，记录日志并返回未接（登记缺口 §4.8：游戏侧 0x38B `Texture` 装不下 `ManagedTexture`）。

### 1.7 `asset_libraries/texture_library.rs`（WIRING §3.1 #1 / §4.1 / §4.2）
- `impl TextureLibraryDevice for GraphicsDevice` 的 `create_persistent_descriptor_set(name)` 改经 `bindless_create_persistent_descriptor_set`（登记缺口 §4.1）；`update_persistent_descriptor_set` 改经 `GraphicsDevice::update_persistent_descriptor_set`（登记缺口 §4.2）。不再调用已改签名的一参占位方法。

## 2. candidate 链测试

- `cargo check -p country-core`：**D 组 8 个文件无错误无新增 warning**。全 crate 当前 13 个错误均来自并发 agent 在制文件（见 §0），非本组引入。
- 基线（加 rhapsody 依赖、未改源码时）：`cargo check` exit 0，9 warning 与接入前一致 → **无 feature 冲突**（WIRING §5.1 预判成立：gpu-allocator/spirq 传递依赖不冲突，ash 同版本）。
- 未跑 `cargo test`：`#[cfg(test)]` 下构造 `GraphicsDevice(u64)` 的旧测试（`shader.rs`/`ssbo.rs`/`load_texture.rs`/`garden`/`terrain`/`pass_resource_access` 等）需 mock 设备重构，登记为后续工作（非 cargo check 验收面）。

## 3. 原侧验证

- `import_unowned`（O 0x140bd4fc0）：`rne-read.py --hex` 读出函数入口 prologue（`push r15; push r14; push rsi; push rbx; sub rsp,0x88`），入口用 rcx/rdx/r8 三个寄存器参数 → 与原 `import_export::impl$1::import_unowned(buffer, rg, access)` 调用形态一致；真实 API = `rg::graph::RenderGraph::import_unowned(name, descriptor, token, access)`（rg/import_export.rs:7，`impl RenderGraph` 扩展）。`mov r9b, 0x37`（WIRING §3.4 的 access 字节证据）位于调用方（ssbo_library），非函数体内。
- `issue_full_pipeline_barrier`：`rhapsody::raw::device::GraphicsDevice` 的 device.rs:911 实现（`vkCmdPipelineBarrier` ALL_COMMANDS）本身就是 O 证据重建（注释链：0x140bf8e60 族）。
- `render`（0x1404e8010）：`render_frame.rs::render` 的 O 调用点映射在 WIRING §2.7 表中逐一登记（`execute_main_graph`/`begin_frame`/`end_frame`），本组接线按其签名完成。
- **未做**：GPU 真机执行（协调简报 §D 规则 4 后置）；RE-MCP 全量反编译核对本轮新增宿主代码（在制并行期间不稳，登记待集成复核）。

## 4. 仍缺项（登记，不补造）

| # | 缺项 | 状态 |
|---|---|---|
| 1 | 游戏侧 bindless `DescriptorSetState` 构造（§4.1）→ `create_persistent_descriptor_set(state, writes)` 真实调用点 | 桥已接真实签名，等 bindless 状态恢复 |
| 2 | `update_persistent_descriptor_set`（§4.2） | 登记缺口，日志桥 |
| 3 | `full_barrier_required` / `[device+0xaba]`（§4.3） | 登记缺口，false 桥 |
| 4 | 游戏侧 40B `Buffer`/0x38B `Texture` 装不下 `ManagedBuffer`/`ManagedTexture`（§4.8） | 登记缺口；`convert_buffer_desc`/`convert_texture_desc` 已真实落地，create 调用点待槽表恢复 |
| 5 | `ShaderLibrary` 实例 + sampler 表未持有 → `create_compute_pipeline`/`create_raster_pipeline`（§2.2） | 登记缺口 |
| 6 | `shader_variants::get_raster_shader_variants`（§4.9，O 0x1417388a0） | 登记缺口 |
| 7 | `rg::pass_api::bind` 自由函数（§4.6）/ `resolve_resource`（§4.7） | PDB 名非真实自由函数；真实路径 `PassBinding::{buffer,image}` 已映射 |
| 8 | ExecutionChain 宿主附加队列 timeline 与后端 `queue_timelines` 对齐（私有字段） | 已用占位句柄，GPU 核验时对齐 |
| 9 | swapchain 获取/展示租约（§4.10）、`GraphExecutor::last_known_frame_duration_ns` 返回形态（§4.12） | 登记待取证（GPU 后置） |
| 10 | `#[cfg(test)]` 旧设备测试（`GraphicsDevice(u64)` 构造） | 需 mock 设备重构，后续工作 |

## 5. 与原 WIRING 契约的一致性

- 只调用 rhapsody **已存在**的公开 API；`crates/rhapsody` 本体零改动。
- 签名全部来自 `crates/rhapsody/src/**` 实际源码（`rg/executor.rs`、`rg/import_export.rs`、`rg/pass_api.rs`、`rg/vulkan.rs`、`raw/device.rs`、`raw/device_api.rs`、`raw/texture.rs`、`raw/descriptor_set.rs`、`raw/submit.rs`、`shader-pipeline/{compiled,pipeline_desc}.rs`）。
- 无虚构函数；缺接口一律登记缺口。
