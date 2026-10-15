# core 渲染接入（country-core ⇄ rhapsody，lavapipe 像素读回）

日期：2026-10-15。范围：`crates/country-core/**`、`crates/tiny-glade/**`（新增 `src/raster_host.rs`、`tests/core_raster_draw.rs`）。
**没有改动任何 rhapsody 现有文件**（也没有在 rhapsody 里新增文件），因此没有需要统一应用的 rhapsody 补丁（见 §5）。

## 1. core trait 方法 → 宿主实现状态

先说结论：core 的 `rg_boundary::RenderPassApi` 只有 3 个方法（fill/clear/copy），**没有** draw/bind/viewport 方法；光栅绘制的外部调用面是
`draw_stream::PassHost`（`RasterPass::render` → `submit_raster_pass(RasterPassSubmission)`）。接线前宿主对 `PassHost` **一个方法都没有实现**。

| core trait / 方法 | 状态（接线前 → 现在） | 说明 |
|---|---|---|
| `rg_boundary::RenderPassApi::{fill_buffer, clear_color_image, copy_buffer}` | 真实 → 真实 | `PassApiHost`（原有） |
| `rg_boundary::RenderGraph::{add_pass, allocate_dynamic}` | 真实 → 真实 | `GraphPasses`（原有） |
| `PassHost::raster_shader` | 缺 → **真实** | 由 `RasterShaderProgram.variants[v].compiled` 解码反射，转 core 的 `ShaderReflection`（名字表/uniform 偏移/块大小/块 binding），按 (程序,变体) 缓存 |
| `PassHost::allocate_dynamic_constants` | 缺 → **真实** | rhapsody `DynamicConstants::allocate`（256KiB 页，只增不复用）；`keepalive` = 宿主常量块表下标 |
| `PassHost::flush_dynamic_constants`（**新增，见 §3.1**） | 不存在 → **真实** | uniform 字节写入常量页 |
| `PassHost::rg_buffer_resource` / `placement_buffer` / `texture_by_name` / `rg_texture` | 缺 → **表驱动** | `RasterShared::assets`（宿主登记）；没登记走原版“库里没有该项”的 `expect` 路径。`AssetSsboLibrary`/`AssetTextureLibrary` 本身未接 |
| `PassHost::vk_buffer` / `vk_image_view` | 缺 → **真实** | `BufferHost::get_by_id` / `TextureHost::default_view` |
| `PassHost::with_suffix_dims` | 缺 → **真实** | `BindingNameHash::with_suffix_dims`（`wrapping_add(1)`，O） |
| `PassHost::default_raster_state` | 缺 → **真实** | `RasterPipelineOpts::new().0` |
| `PassHost::create_render_pass` / `render_pass_cache` | 缺 → **真实** | `RasterRenderPassDesc` → rhapsody 0x98B `RenderPassDesc` → `RenderPass::from_desc`；键表去重（`RenderPassHost::create_from_desc`）；返回的 `ManagedHandle` 就是 `RasterPipelineKey.handle` |
| `PassHost::submit_raster_pass` | 缺 → **真实（部分 draw 种类）** | 见 §2；`NoVao`/`NoIndexBuffer`/`NoVaoIndexed` 真画，其余 5 种在声明期 panic（未证实） |
| `PassHost::ssbo_buffer_word` | 缺 → 缺（缺省 `None`） | 需要 `AssetSsboLibrary` 的缓冲首字，库未接 |
| `PassHost::rg_resources` → `RgResources::create_raw_texture/create_raw_buffer` | 缺 → **真实** | `GraphRgResources`：图里 `create_raw_resource` + 后端 `declare`（经 `GraphDeclarer`，对 `Rc<RefCell<VulkanGraphSession>>` 有现成实现） |
| `RgResources::get_or_create_temporal_{texture,pair,buffer}` | 缺 → **明确报“未接”**（返回 `Err`） | 键空间不一致（core 应为 `Ustr`，`ustr_contract::intern` 仍返回 0；rhapsody `TemporalRenderGraphState` 以 `&str` 为键且要按帧导入物理纹理）。不伪造历史 |
| `ShaderBackend`（5 方法） | 真实 | 原有；raster 的 `vertex_entry` 恒空串仍是未证实项 |
| `ssbo::GraphicsDevice`、`TextureDevice`、`BufferGraphBackend`、`ComputePassDevice`、`pass_resource_access::TextureDevice`、`ComputeExec` | 真实 | 原有（`BufferGraphBackend` 的 token/命名见 §3.3） |
| `UniformAllocator`、`WorldAccess`/`ResourceLookup`（`PassResources::new_from`）、`BindGameResources`、`TextureAccess`、`painterly::PassHost`/`VermeerHost` | 缺 | 属于上层“帧级 trait”（RENDER_GAP_ANALYSIS #2/#6/#8），需要 tiny_glade 的 render_loop 层；本轮未做 |

rhapsody `RenderPassApi` 方法在宿主里的使用情况（宿主按 `RasterPassSubmission` 调用，不经 core trait）：
`begin_attachments`、`set_default_view_and_scissor`、`set_stencil_ref`、`draw`、`draw_indexed`、`bind_index_buffer` 已用；
`bind_vertex_buffer_by_binding_point`（`DrawPacket` 没有 VAO 绑定，core 的几何全部是 SSBO 拉取/索引）、`push_constants`（`DrawPacket.unknown_bytes` 语义未知）、
`draw_indirect*`/`draw_indexed_indirect*`（`arg_48/50/58` 与 vkCmdDraw*Indirect 参数的对应没有证据）、`set_line_width` **未用**。
管线绑定按原版属于 `GraphicsDevice::bind_pipeline`，宿主直接 `vkCmdBindPipeline`（与 triangle_animation_graph 一致）。

类型对应：`RenderPassDesc`(core `{selector}` 占位，未用) → 实际键是 `draw_stream::RasterRenderPassDesc` → `rhapsody::raw::render_pass::RenderPassDesc`（0x98B）；
颜色附件 `+4` 在 core 里被覆盖为附件 tag，宿主照原样当 loadOp（tag0 LOAD / tag1 CLEAR / tag2 DONT_CARE，`vkCreateRenderPass` 原样用该字，0x1418b8c50）；
深度 `depth_state` 0/1/3 ↔ `DepthAccess::ReadOnly/ReadWrite/None`。`RenderPassBinding`（core 只有 `{resource_index, access_type}`）→ 本轮补了 `view: Option<TextureViewDesc>`（见 §3.2）；
`PassResourceSlot` core 版 4 变体仍只给 compute 外部绑定用，光栅路径用 `DrawPacket.bindings`（`DescriptorSetBinding` 三变体）+ `RenderPassBinding`，不需要它。

## 2. 接通了什么、怎么接的（调用链）

```
raster_single_object / RasterPass::new + DrawBuilder::new(.., variant)          [core]
  ├ PassHost::raster_shader → 反射（名字表/uniform 偏移）                          [RasterPassHost]
  ├ PassHost::allocate_dynamic_constants → DynamicConstants::allocate
  ├ DrawBuilder::uniform / bind_* → packet.bindings / pass.bindings / accesses
  └ draw_no_vao(n) → finish(): flush_dynamic_constants → pass.draws.push         [core，本轮补 flush]
RasterPass::render()                                                              [core]
  ├ build_render_pass_desc → render_pass_cache 未命中 → create_render_pass
  │     → rh_render_pass_desc → RenderPassHost::create_from_desc → RenderPass::from_desc（按 0x98B 键去重）
  └ submit_raster_pass(RasterPassSubmission{draws,bindings,render_pass,accesses,colors,depth,..})   [RasterPassHost]
       ├ 声明期：每个 draw → program.variants[v].get_or_generate(ShaderHost, {通道句柄, packet.state}) → RasterEntry
       │         （→ ShaderHost::create_raster_pipeline，M7.4 已有）
       ├ 声明期：accesses → rhapsody 图 add_resource_access（颜色 0x36/0x2e、深度 0x2f/0x17、RG 缓冲/纹理…）
       └ render_with_context 回调（图执行时）：
            AttachmentCache::prepare（视图 + 兼容通道 + 帧缓冲，缓存）→ RenderPassApi::begin_attachments
            → set_default_view_and_scissor → set_stencil_ref(0)
            → 逐 draw：vkCmdBindPipeline → set 0 描述符集（DrawPacket.bindings + RG 访问→物理资源）
                       → RenderPassApi::draw / bind_index_buffer + draw_indexed
            → end_render_pass；管线/帧缓冲/描述符集/索引缓冲经 PassContext::keep_alive 交给命令拥有者
```
执行由 `VulkanGraphSession::execute_frame` 完成（2 帧在飞、帧账本、资源池均沿用 rhapsody 原有机制）。

## 3. 发现并修复的 core / 宿主 bug（均先写失败测试）

1. **uniform 字节丢失（core）**：`DynamicConstantsBinding.data` 是原版“可写映射切片”的拥有型替身，`DrawBuilder::finish/draw` 消费 builder 时随之丢弃，uniform 值永远到不了 GPU。
   - 失败测试 `finishing_a_draw_flushes_the_uniform_bytes_to_the_host`（先 RED：`flushed_constants.len()==0`）；修复：`PassHost::flush_dynamic_constants`（缺省空实现，不影响既有 mock）+ `DrawBuilder::flush_constants()` 在 `finish`/`draw` 调用。`discard` 不交回（测试覆盖）。
   - 像素测试里不修复则 `m = 0`，三角形退化、像素数 0，断言必败。
2. **`TextureDesc::new_2d/new_2d_array/new_3d` 没写 kind（core）**：原版 `+0x1e` 维度码（2D=2、2D 数组=3、3D=4，core 自己的文档标 O），重建版恒 0（=1D）；宿主据 kind 建图像时图内创建的 2D 纹理成了 1D。
   - 失败测试 `rhapsody_types::tests::dimension_constructors_set_the_kind_byte`（先 RED：`left: 0, right: 2`）；修复：三个构造器写 2/3/4。core 全量测试仍绿。
3. **导入 token 空间撞车（宿主）**：缓冲导入用槽位序号（小整数）当 rhapsody 后端 token，而后端 `allocate_token()` 给图内创建资源也从小整数发 → “token已经属于其他物理资源”（深度测试第二帧发现）。
   修复：`graph_token(id)`（缓冲 id 打 `1<<62` 标记，纹理 id 本就有 `1<<63`）；`GraphHost::import_unowned` 用它，导入名字改为 `imported-{id:x}`（同图内同名会串位）。已同步更新 `tests/render_host.rs` 里两处 `bind_import_named`（只改 token 取值，断言不变）。
4. 契约占位补口：`rhapsody_contract::RenderPassBinding` 增加 `view: Option<TextureViewDesc>`（原 0x44B 结构的 0x3c B 视图块在占位里丢了；`bind_rg_texture` 现在把请求的视图带到宿主，测试 `rg_texture_binding_keeps_the_requested_view_desc`）。
5. `RasterPassSubmission` 增加 `colors`/`depth`/`viewport`（宿主接缝；原版在 `PassBuilder::render` 的闭包环境里同样带着 `RasterPass +0x1e0..+0x3e0` 的附件，**I：未逐字节核对闭包环境布局**）。否则回调里不知道附件是哪几个纹理。

宿主侧新增（`render_host.rs`）：`RenderPassHost::create_from_desc`、`TextureHost::with_extra_usage/convert_for_graph`（core `TextureDesc` 没有 TRANSFER_SRC 位，读回需宿主附加；宿主策略，非原版行为）、
`BufferHost::record`（core `Buffer` 与 rhapsody `BufferRecord` 同为 40 B：`ctx=id`、`slot=generation`、`+0x10={offset,device_id,ready}`、`+0x18=size`、`+0x20=desc_tail`，`create_buffer` 现在把 `+0x10/+0x20` 填真值；`offset/device_id` 槽表不提供记 0，U）。

## 4. 新测试与实测数字（`crates/tiny-glade/tests/core_raster_draw.rs`，7 个）

着色器全由 rspirv 生成，装成发行 .bin（含 `SHADER_VARIANT_DEPTH_ONLY=0/1` 两个变体），经 `ShaderHost` + `RasterShaderProgram::new_draw` 读入。顶点着色器 `gl_Position = u.m * BASE[gl_VertexIndex]`，
`m` 由 core 的 `DrawBuilder::uniform(hash, Mat4)` 写入（已含 Y 翻转以抵消 rg 默认负高度视口），`draw_no_vao(3)`。读回缓冲/颜色纹理由 core 的 `BufferHost`/`TextureHost` 建（单帧测试）。

| 测试 | 内容 | 实测 |
|---|---|---|
| `core_raster_pass_draws_a_triangle_into_a_core_texture_and_reads_back` | core 纹理（导入图）→ `raster_single_object`（tag1 = CLEAR 附件）→ 读回，逐点：四角 `[0,0,0,0]`、质心像素 FILL、全图与 `reference_coverage` 对比；渲染通道 1 个、变体0管线 1 条、变体1 0 条 | 三角形像素 **509**（理论 512），与参考光栅化不一致 **0**，质心误差 **0.049 px** |
| `core_raster_pass_chain_animates_over_frames_with_two_in_flight` | 30 帧、每帧 15°、每帧不同清除色（脚手架 pass 写清除色，core 的 tag0 = LOAD 附件叠三角形，证明 LOAD 保留内容且上一帧不泄漏）、颜色目标由 core 的 `RgResources::create_raw_texture` 在图内创建（执行器资源池）、2 帧在飞 | 校验 **30** 帧；每帧像素数 **512**；与参考光栅化最多 **0** 个不一致；质心最大误差 **0.09 px**；在飞峰值 **2**；每帧 `begin_frame` 后在飞数 `[0,1,1,…]`；30 帧共 **1** 个渲染通道、**1** 条管线（缓存命中）；无帧账本告警 |
| `core_raster_pass_depth_attachment_resolves_overlap_independent_of_draw_order` | 深度附件（D32，`DepthClear::Clear(1.0)`，LESS+写）+ 同一 `RasterPass` 里 2 个 draw（`RasterPass::new` + 两个 `DrawBuilder`，颜色走 3 个 f32 uniform），先远后近 / 先近后远两种顺序 | 两种顺序都是：红 512、绿 332、重叠区红 **180/180**、与参考合成不一致 **0** |
| `core_buffer_handle_is_bit_identical_to_the_rhapsody_buffer_record` | `transmute(Buffer) == BufferRecord`、字段、过期句柄无记录 | 通过 |
| `render_pass_desc_maps_attachment_tags_to_load_ops_and_depth_access` | tag→loadOp、深度 tag、键去重 | 通过 |
| `draw_kinds_without_a_verified_vulkan_mapping_are_rejected_when_the_pass_is_declared` | `InstancedGlDrawFlag` 在声明期 panic 且文案含“未证实” | 通过 |
| `host_temporal_resources_report_not_connected_instead_of_faking_history` | temporal 入口返回 `Err` | 通过 |

没有 Vulkan 设备时各测试 `return` 跳过。core 新增 3 个单元测试（flush、视图保留、kind 字节）。

## 5. 需要在 rhapsody 现有文件里应用的补丁

**无必须项**（全部在宿主层绕过）。可选建议（不阻塞）：
- `rg/vulkan.rs`：`VulkanGraphBackend::declare` 需要 `&mut backend` 而后端归会话所有，宿主只能用 `GraphDeclarer`（`Rc<RefCell<VulkanGraphSession>>`）绕；若后端能提供“按 `&graph` + 内部可变规格表”的声明入口会更直接。
- token 空间：`allocate_token()` 与显式 `bind_import*` 共用小整数域，撞车时只有“token已经属于其他物理资源”一句错；建议让 `allocate_token` 从高位起或在冲突时给出资源名。宿主已用高位标记规避。
- `device_api::create_render_pass` 仍用平铺 `RenderPassKey` 做缓存键（前序报告已提）；宿主直接走 `RenderPass::from_desc` + 自己的 0x98B 键表，不依赖它。

## 6. 契约替换 DEEP-SUMMARY §6 的 9 条逐条结论

总体判断：**宿主槽表方案应当保留**。core 的 `Buffer`/`Texture` 是 `Copy` 的 40 B/`u64` 值，1300+ 测试和游戏代码大量依赖 `Copy`/`Default`；
rhapsody 的 `Managed<T>` 是带 Drop 的 Arc 句柄，直接替换会让 core 句柄变成非 Copy、不可 Default，且 core crate 目前**不依赖 rhapsody**（`Cargo.toml` 无该项）。
槽表（id+代数）还提供了“过期句柄被拒绝”的语义。唯一应补的是布局对齐（第 7 条，已做）。

1. **`GraphicsDevice(pub u64)` → `Arc<rhapsody GraphicsDevice>`**：**不做**。`GraphicsDevice(n)` 在 40 处（18 个文件）以字面量出现，大多是测试里 `GraphicsDevice(1)` 的无 GPU 占位；换成真设备会让 1300+ 个 CPU 测试都需要 Vulkan。且 core 要新增对 rhapsody 的依赖。最小路径：保持占位（它只是个不可用的标记，`dispatch` 等方法本就 panic），所有真实调用走宿主的 `BufferHost/TextureHost/ShaderHost/DeviceHost`——本轮已经这样用了。
2. **`Handle<T>`/`ReadOnlyHandle<T>` → `rg::resource::Handle` + `ManagedHandle<T>`**：**不做**。`Handle<T>` 在 31 个文件 166 处使用，`ManagedHandle(u64)` 29 个文件 152 处，宿主已把 `ManagedHandle.0` 约定为缓冲 id / 带标记的纹理 id（`get_vk_buffer`、`vk_image_view` 等都按它查槽表）。`Handle::get()`/`ReadOnlyHandle::get()` 的两个 panic 目前没有调用点触发。最小路径：等 core 有一个真正持有 `Arc<RenderGraph>` 的帧上下文后，再把 `get()` 接到 `ManagedPassResources`。
3. **`RenderPassDesc{selector}` → `RasterPipelineKey{ManagedHandle,u32}`**：**已具备/已落地**。core 的 `rhapsody_types::RasterPipelineKey{handle, selector}` 本来就是双字段；`contract::RenderPassDesc{selector}` 没有使用点（实际缓存键是 `RasterRenderPassDesc`）。本轮落地：`RasterRenderPassDesc` → rhapsody 0x98B `RenderPassDesc`（含 D9 的字段序：`clear_input@+4`、`sample_count@+0xc`）、`create_from_desc` 键表去重、tag→loadOp 映射（有测试）。
4. **`PassResourceSlot`/`RenderPassBinding` 对齐 0x44 B**：**部分**。`RenderPassBinding` 补了 `view`（原结构最前的 0x3c B 视图块），光栅路径已端到端用上（`bind_rg_texture` → 宿主 `rh_view_desc`）；0x44 B 的精确字节布局不引入（core 里它不被整块搬运，只有宿主读取）。`PassResourceSlot` 4 变体带 offset/size，原版 ≥6 个判别式且缓冲整块（wave2 报告），只在 compute 外部绑定用，需要 rhapsody `rg/binding.rs` 与 core 同时改，留给集成者。
5. **temporal 键改 `Ustr`**：**不做（理由明确）**。`ustr_contract::intern` 仍返回 0，若把键换成 `Ustr` 而 intern 不实现，所有 temporal 键都相同。宿主 `RgResources` 的 temporal 三个入口显式返回“未接”。最小路径：先让 core 引入真正的 `ustr`（workspace 已有 `ustr = 1.0.0`）并实现 `intern`，再让宿主把 `Ustr` 映射到 `TemporalRenderGraphState` 的 `&str` 键 + `TemporalGpuResources::import_into`（rhapsody 侧已有，`triangle_animation_graph.rs` 有完整用法）。
6. **`ImportExportToRenderGraph`/`IntoSampledRenderGraphTexture`**：**不做**。孤儿规则：trait 在 rhapsody、`Buffer`/`Texture` 在 country-core，宿主 crate 无法为二者实现；要么 core 依赖 rhapsody，要么用新类型包装。现有 `BufferGraphBackend`（`import_unowned`/`export`）+ `GraphHost` 已经覆盖 core 实际用到的两个调用点（缓冲/纹理导入导出），`ImportedBuffer` 的 20 B 形状维持占位。
7. **Buffer 40 B 契约**：**已做**。`BufferRecord` 与 core `Buffer` 逐字节一致（`transmute` 测试），`BufferHost::record` 提供转换，`create_buffer` 填入真实 `size`/`desc_tail`。`offset/device_id` 记 0（槽表无此值，U）。
8. **补 GraphicsDevice 方法身份（get_vk_buffer/get_vk_image_view/create_render_pass/create_framebuffer/bind_descriptor_set…）**：**宿主层已覆盖**（`PassHost::vk_buffer/vk_image_view/create_render_pass`、帧缓冲经 `AttachmentCache::prepare`、`bind_pipeline`/`bind_descriptor_set` 在回调里直接调 ash）。没有往 core 的占位 `GraphicsDevice` 上加方法（它不可用，加了只是“看起来接入了”）。
9. **帧级 trait 生产接线（`FramePasses`/`RenderFrameSurface`/`PassHost`/`RgResources`）**：**`PassHost` 与 `RgResources`（raw 创建部分）已接**，光栅路径可真实执行并有像素证明。`FramePasses`/`RenderFrameSurface`/painterly `PassHost`/`VermeerHost`/`WorldAccess`/`UniformAllocator` 仍缺（需要 tiny_glade 的 render_loop 层与世界资源，非本 crate 范围）。

## 7. 没做到的与原因（诚实清单）

- draw 种类：`InstancedGlDrawFlag`（flag 含义、顶点数来源）、`MultiInstanced`（`(first_instance, instance_count)` 已证实，每实例顶点数来源未知）、`MultiDrawIndirect`/`IndexedIndirect`/`IndexedIndirectCount`（`arg_48/50/58` ↔ vkCmdDraw*Indirect 参数无证据）：声明期 panic（文案含“未证实”），不猜映射。`NoVaoIndexed` 的索引类型取 UINT32（依据 `vao.rs`：`indices.len()*4`，O）。
- `DrawPacket.unknown_bytes`（push constants？）、`DrawPacket.blend`（`+0xac` f32，缺省 1.0）、`head`：语义未知，宿主不使用；tag1 附件的 16 B 视图字（`ColorViewWords`）宿主忽略（走默认视图）。
- 颜色 CLEAR 的清除值：core 描述里没有，取 0（I，`AttachmentCache` 缺省）。深度 `DepthWriteStencilRead` 在 core 里不可达。storeOp 恒 STORE。
- temporal `RgResources`、`ssbo_buffer_word`、资产库（`AssetSsboLibrary`/`AssetTextureLibrary`/placement）的真实对接：只有宿主登记表。
- 描述符：只处理 set 0（与 core `DrawBuilder` 的 `assert_eq!(set, 0)` 一致）；采样器名→内置采样器映射、多 set 未接；每个 draw 瞬时分配一个描述符集（原版持久集形态未对照）。
- 未做像素验证：MSAA/resolve、`SHADER_VARIANT_DEPTH_ONLY=1` 变体、RG 纹理/缓冲经 `packet.accesses` 的描述符写入（代码已写，宿主 compute 路径有同类验证，但光栅路径没有对应像素测试）、`bind_*` 纹理采样。
- `RasterPassSubmission` 新增字段与原版闭包环境的对应（I）。

## 8. 最终全量验证

`cargo test --offline -p rhapsody -p tiny-glade -p country-core -p shader-pipeline --all-features`（lavapipe）：58 个 `test result`，**1888 passed / 0 failed**（基线 1806；差额含本轮新增 core 3 个 + tiny-glade 7 个，其余为并行 agent 同期新增）。
