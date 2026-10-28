# country-core ↔ rhapsody shader、纹理与 bindless 接口调查

日期：2026-10-18。以 `games_test/tiny-glade/crates/` 当前生产源码为准。复用已有发行 shader 反射/ABI 证据，没有新增原码查询、全量 shader 扫描或原版行为验收。

## 实际调用关系

`country-core` 保留游戏自己的 shader 来源、程序库、pass 声明和绑定规则；真正的设备、shader ABI 改写、管线异步编译、bindless heap 和图执行由 `rhapsody` 提供。两个库通过 `tiny-glade/src/render_host.rs` 与 `raster_host.rs` 相接。不能根据 core 内旧 `GraphicsDevice` 包装器的 Unsupported 推断正式 shader producer 不存在。

| 游戏侧输入/责任 | 正式桥接入口 | rhapsody 输出/责任 |
|---|---|---|
| `ShaderUri` / `ComputePipelineDesc` / raster 两个 depth-only 变体 | `ShaderHost::load_compiled_compute` / `load_compiled_raster` / `raster_shader_variants` | 按真实 `compiled_file_path()` 读 `.bin`；`CompiledShaderPipeline::decode` 解码 |
| `ComputeShaderProgram::try_new` | `ShaderHost::create_compute_pipeline` | `BindlessShaderPackage::new` 改写 SPIR-V 与布局，`AsyncPipelineCompiler::request_compute_spirv` 返回有界异步 ticket |
| `RasterShaderProgramVariant::get_or_generate`，key=render-pass id+state | `ShaderHost::create_raster_pipeline` | 动态渲染附件格式、反射 vertex input、`RasterPipelineOpts::apply_to`、异步 raster ticket |
| 原反射的名字哈希、uniform 大小/偏移 | `registry_from_reflection`，`DeviceHost::pipeline_registry` | core `ResourceRegistry`；保留原反射 set/binding，和 V9 改写后的实际 heap ABI 分开使用 |
| `RecordedComputePass` 的 RG/external 绑定、uniform、push、dispatch | `GraphPasses::add_compute` | 声明访问、从同帧 `ManagedPassResources` 解析真实资源，`BindlessParameters::write/finish/bind_pass`，再发 dispatch |
| `RasterPassSubmission` / `DrawPacket` | `RasterPassHost::submit_raster_pass` | 解析 shader variant、附件/外部访问；执行时 `bindless_parameters`、`begin_attachments`、支持的 draw 命令 |
| core `TextureDesc`、磁盘载荷、`Texture.id` | `TextureHost::convert_for_graph/create_texture/get_by_id/default_view` | rhapsody texture/managed owner、真实 VkImageView；代数校验和 view owner 保留 |
| 原 sampler 反射名字 | `ShaderHost::bindings` | 宿主覆盖优先，随后查 rhapsody 已恢复的内置 sampler 表，转换为 heap sampler lease |

关键定位：

- core：`src/render/shader.rs` 的 `ShaderBackend` / `ComputeShaderProgram::try_new` / `RasterShaderProgramVariant::get_or_generate`；`src/render/pass_builder/compute_pass_builder.rs:410` 的程序创建与缓存命中；同文件 `RecordedComputePass`。
- 宿主：`tiny-glade/src/render_host.rs:492` 的 `registry_from_reflection`；`:887` 的 `ShaderBackend for ShaderHost`；`GraphPasses::add_compute`；`tiny-glade/src/raster_host.rs:656` 附近的 `submit_raster_pass`。
- rhapsody：`src/bindless_parameters.rs:52` 的 `BindlessShaderPackage::new`；`:230` 的参数 ABI 校验；`:363` 的 `finish`；`:435` 的 `PreparedBindlessParameters::bind_pass`。

V9 的生产链是共享 heap + 参数缓冲切片 + 4 字节参数索引 push。原 shader 的资源 set/binding 和原 push payload 由 `BindlessShaderAbi` 映射到该参数缓冲。重新建立旧每 pass descriptor-set API 会绕开当前生产模型。

## 本轮实际修复

### 1. 光栅 shader 句柄来自同一个权威程序库

旧 `RasterShared::register_program` 对独立 Vec 从 1 分配 id，core `AssetShaderLibrary::add_raster` 却使用全局 `Handle::new()`。来自 core 的真实 draw handle 不能据此查独立 Vec；数字偶然相同还可能选中另一程序。

当前新增：

- `AssetShaderLibrary::get_raster_by_id(raw)` / `get_compute_by_id(raw)`，直接查原 FrozenMap，不分配新 handle。位置：`country-core/src/asset_libraries/shader_library.rs:217` / `:233`。
- `RasterShared::new_with_shader_library(..., Arc<AssetShaderLibrary>)`，位置：`tiny-glade/src/raster_host.rs:358`。正式程序来源固定为该库；反射读取和管线 preparation 都借用库内原程序及其原缓存，没有 clone 程序或重新建一套 shader identity。
- `RasterPrograms::{Core, Standalone}` 是互斥来源；Core 模式不回退到 standalone Vec。`try_register_program` 在 Core 模式返回 InvalidUsage。原 `new/register_program` 保留 standalone 测试兼容路径，不作为正式 core 集成入口。

### 2. 内置 sampler 自动调用真实设备实现

旧 `ShaderHost::bindings` 只接受 `with_sampler` 注入；而 `rhapsody/src/raw/sampler.rs:58` 起已恢复发行 shader 的 9 个名字及完整 13 项内置参数，设备已提供 `get_built_in_sampler_by_name`（`raw/device.rs:586`）。

当前 `render_host.rs:853` 先查 `with_sampler` 覆盖，再调用真实设备名字查找。未知名字仍报设备错误，DeviceLost/OOM 不被转换成成功；没有补造 sampler 参数。

### 验证范围

在 sampler 补线前执行了：

```text
cargo test -p tiny-glade --lib program_identity_tests --locked --offline
2 passed; 0 failed; 0 ignored
```

两项 CPU 测试确认：真实全局 id 可以读到库内相同程序对象，后插入程序立即可见；无效 id 返回 None；Core 模式不接纳 standalone 注册、不回退到 standalone 同值 id。sampler 补线后的完整编译/测试由本轮整合 owner 统一执行，结果见同目录总报告。上述测试不构成原版 GPU/整帧等价验收。

## 仍阻碍真实游戏渲染的缺口

### P1：frame_constants 的 set 1 资源尚未进入 pass 参数

当前 compute 绑定写入是 `parameters.write(BindingLocation { set: 0, binding }, ...)`；raster 的 `bindless_parameters` 同样固定 set 0（`raster_host.rs:1221`）。`ShaderBindings::parameters` 只自动填 sampler，没有注入帧 UBO/SSBO。

原反射表明确要求 `frame_constants` set1/b0，部分 pass 还有 `frame_constants_storage_buffer` set1/b1：`reverse-engineering/tiny-glade/evidence/render-passes/INDEX.md:23`。现有 V9 `rhapsody/reports/refactor/2026-10-17-v9/bindless-domain/shader-assets/_dof_fullres.cs.hlsl_cs_main_6f1dd5dd8a66e104/abi.txt` 最后一项也直接保留 set1/b0、UniformBuffer、Fixed(1)。

`BindlessParameters::finish`（`bindless_parameters.rs:369`）对未写入的固定槽明确报 `required shader resource element is unbound`。因此只通过 set0 合成 shader 的局部测试，不能证明实际 DoF/世界 shader 可以执行。应把真实帧常量上传结果作为强类型 frame binding 注入 compute/raster 共用的参数构建；图访问与资源租约也必须覆盖它，而不能填默认零缓冲骗过校验。

### P1：AssetTextureLibrary 的材质索引没有桥接到 V9 heap

`country-core/src/asset_libraries/texture_library.rs:475` 确实实现了 CPU 索引分配；`:500` 会收集 pending 纹理更新。但唯一非测试 `TextureLibraryDevice` 实现仍是同文件 `:159` 的 core `GraphicsDevice` 包装：

- `create_persistent_descriptor_set` 返回零字段 `DescriptorSet` 占位；
- `update_persistent_descriptor_set`（`:165`）只转发日志入口；
- `get_vk_image_view`（`:174`）把 `texture.id` 当视图返回；
- `update_texture`（`:177`）直接 `Ok(())`。

正式 `TextureHost/DeviceHost` 尚无 `TextureLibraryDevice` 实现。pass 级别的 `BindlessParameters` 已连接，不代表材质/实例数据内的 bindless texture index 已连接：`nani/instance_updates.rs` 把这些索引写入 GPU instance words。后续需要保留库的逻辑索引与 V9 lease 的真实对应、索引生命周期、变更上传和 shader runtime-array 映射，不能把 CPU 索引直接当 heap 槽位。

### P1：常用 GPU-driven draw kind 在宿主被拒绝

`tiny-glade/src/raster_host.rs:958` 的 `reject_unverified` 只接受 NoVao、NoIndexBuffer、NoVaoIndexed。MultiInstanced、InstancedGlDrawFlag、MultiDrawIndirect、IndexedIndirect、IndexedIndirectCount 仍返回 Unsupported。

生产者已经存在：`country-core/src/render/pass_builder/draw_builder.rs:740` 起，尤其 `multi_draw_indirect`（`:766`）、`multi_draw_indexed_indirect`（`:777`）、`draw_indexed_indirect_raw`（`:791`）、`draw_indexed_indirect_count`（`:813`）。这是游戏 nani/实例化 draw 的真实阻塞，不能以全屏三角形可画升级为建筑世界已可渲染。需按已有 packet 参数与真实调用证据恢复命令映射、VAO/索引来源与精确访问范围。

### 本轮追加修复：callback 首次 push 与 dispatch 时序

`roof_tiles_gravity_compute` 在 callback 内逐项提供实际 20 字节 push；旧宿主在回调前以空初值建立 V9 参数块，会因原 ABI 长度要求提前失败。当前 `ComputeParameterState` 对“非零 ABI、空初始 payload”延迟绑定，首次实际 push 才创建参数块；后续 `with_push` 复用同一资源租约。direct/indirect dispatch 共用 ready/error 检查；尚无合法参数时不录制命令，首个错误不能被后续 push 覆盖。没有用补零载荷代替真实参数。

四项 CPU 状态反例已通过，覆盖 20 字节首次 push、无 push 先 dispatch、错误长度及错误保持、零字节 ABI/显式初始载荷。真实 GPU callback 未执行，本机 loader 不可用；不能升级成整屋顶 pass 运行验收。

**仍未修复的独立边界**：`ShaderHost::create_raster_pipeline` 对 `original_push_bytes != 0` 仍返回 Unsupported。这不是 compute 首次 push 的问题，需单独确认光栅 producer 的载荷后接线。

## 后续接线的边界

本轮负责程序 identity、sampler 与 compute 首次 push；同一 session 的 ComputeFrame/PassFrame、世界资源视图及 CoreRenderBridge 由整合任务交付。历史 `ResourceLookup for bevy World` 恒 None（`pass_resources.rs:114`）不能作为新桥已失败的结论，但直接将 `&World` 当 PassEnv world 仍不可用，必须经过显式类型映射和有效生命周期的 adapter。

下一批优先闭合：真实帧常量 → 两类 pass 参数；材质纹理逻辑索引 → V9 heap；GPU-driven draw → 命令提交。先选择一个真实游戏 pass 做有资源、有 uniform/push 的闭包验证，然后扩大到相邻 pass。原版整帧画面、行为差分与 A/C 范围仍未由本报告验收。
