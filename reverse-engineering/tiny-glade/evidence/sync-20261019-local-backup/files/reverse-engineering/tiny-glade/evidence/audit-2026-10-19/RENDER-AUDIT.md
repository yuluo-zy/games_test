# country-core B `render/**` 逐文件还原度复核（只读审计）

日期：2026-10-19
范围：`D:/game/tiny-glade/crates/country-core/src/render/**`（47 个 `.rs`）
基线：
- B 骨架 `D:/game/reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src/render`（39 文件）
- 机械统计 `evidence/audit-2026-10-19/fn_audit.json`
- `country-core/recovery/STUBS-2026-10-19.md`（本文多处与其口径不一致，见 §2）
- 真实 rhapsody API `D:/game/tiny-glade/crates/rhapsody/src/rg/{graph,vulkan,executor}.rs`

方法与限度：
- 以源码文本 + `fn_audit.json` + 骨架 PDB 注解为准。**本次没有编译、没有运行任何测试**；STUBS 里的 2452 通过/5 失败是它的历史结果，未复验。
- “行数”为 `wc -l`；另给非测试行/测试行，便于判断“水分”。
- 类型/字段的 O 偏移与测试被当作“有证据”；没有反编译复核每条注释里的地址。
- GPU 真机行为全部后置（协调规则 §D），不计入下方完成度主值。

---

## 0. 结论摘要

- 走查的 47 个文件中，**没有发现 `todo!()` / `unimplemented!()`**。`unreachable!()` 4 处全部在 `#[cfg(test)]` 内。
- 真正的空函数体 6 处，全部是**有注释说明的接缝默认值 / 原作内联函数**（§2），不是遗漏业务。
- 机械统计把大量**泛型单态化**记为 `no_trace`：`render/` 507 个骨架函数中 285 个 `no_trace` + 60 个 `va_only`，其中 344/345 是 `<T>` 单态化，当前由泛型实现覆盖（§3）。
- 唯一一个“非泛型、全 crate 无名字无地址”的真缺失是 `AvailableGraphicsDevices::from_preference_order`（原 0x140987cb0）。
- 47 个文件里约 2/3 是真实行为（含 10 个最大文件中的 9 个）；数据合同、CPU 业务、声明层都很扎实。
- rhapsody 真接线**存在且被真实调用**，但落点在 `crates/tiny-glade/src/{render_bridge,render_host,raster_host}.rs`，不在 core 内。core 自带的 `render/execution_host.rs` 是**未被引用的重复接缝**；`ExecutionChain`/`ExecutionBackend` 在游戏侧只被 re-export，没有直接构造。
- 完成度（不含 GPU 真机）：**≈90%**，置信度中。函数/地址覆盖 ≈99.8%；剩余风险集中在 GPU 管线创建接缝、少量默认值占位、以及“CPU 测试充分但 GPU 未跑”。

---

## 1. 文件级状态表（47 文件）

性质分类：**行为**=有可执行业务逻辑；**合同**=布局/枚举/serde/数据容器；**混合**=两者兼有；**接缝**=为宿主/库交互定义的类型或转发；**别名**=re-export。
行数：总行 / 非测试行 / 测试行。

| 文件 | 行数 | 性质 | 判定与证据 |
|---|---:|---|---|
| `adaptive_render_state.rs` | 31/31/0 | 合同 | 仅证明 B 启动变体（tag 0 + enabled），作者明确声明“不是自适应 GPU 预算控制器”。`at_startup` 真行为，`native_tag` 恒 0（O）。 |
| `atmosphere.rs` | 127/106/21 | 合同+行为 | `SunSkyColors::lerp` 按机器码逐分量 `other*t+self*(1-t)`；serde DTO 还原 4 个 Vec3；`DerivedAtmosphere` 0x7c 字节布局有 const assert，但**仅有 Default（启动清零），运行时产出者在别处**。 |
| `device.rs` | 47/47/0 | 接缝 | `GraphicsDeviceResource` 包装真实 `rhapsody_contract::GraphicsDevice` 并 Deref；`+0x188` 管线缓存登记为 gap（`_pass_cache_gap`），不造字段。 |
| `directional_light.rs` | 116/68/48 | 行为 | `look_at`/`set_direction_towards_light` 与 glam `look_at_rh` 的列构造逐条对齐，0x80 布局断言。测试覆盖。 |
| `dirty_index_range.rs` | 97/72/25 | 行为 | 原子闭区间脏窗口；空哨兵 `(MAX,0)`；测试覆盖合并与边界。 |
| `execution_host.rs` | 90/90/0 | 接缝（**死代码**） | 真实构造 `VulkanGraphBackend::new(..., AccessTable::original())` + `VulkanGraphSession`（:28-34），但全仓只被 `render/mod.rs:27` 声明，**无任何调用方**（grep 只命中自身）。 |
| `frame_constants.rs` | 166/133/33 | 合同 | 0x1090 布局、5×ViewConstants、`should_reset_temporal=(frame_counter==0)` 均有 O 证据；仅启动零块 + Default，逐帧 producer 不在本文件。 |
| `indirect.rs` | 62/62/0 | 合同 | 3 个 Vulkan indirect 命令的 repr(C) 布局与 `empty()`；无独立过程（原版内联）。 |
| `load_texture.rs` | 293/52/241 | 混合 | 解析器/路径/加载**真实**（magic 0x48502183、version 8、0x1c footer、越界检查）；设备 `create_texture` 的 `rhapsody_contract` 实现恒 Err（:34-51），生产由 `TextureHost` 实现（见 §4）。测试 8 个。 |
| `mesh.rs` | 461/322/139 | 行为 | Mesh/VertexAttributeValues 全量恢复：判别式、stride、append/retain/retain_vertices/碰撞盒；测试覆盖 panic 文案。 |
| `mesh_atlas.rs` | 7/7/0 | 别名 | 仅 re-export `resources::render::mesh_atlas_library::AtlassedMeshName`。 |
| `mod.rs` | 45/40/5 | 声明 | 模块挂载；已删 `uniform.rs`（并入 uniform_binder）。 |
| `nani/buffers.rs` | 611/301/310 | 行为 | 27 项 draw-list 参数跳表、`NaniDrawListOffsetDefaults`（108 u32）、SSBO 载体注册，均有立即数核对与测试。 |
| `nani/cull_shaders.rs` | 269/190/79 | 行为 | 9 个 `AsComputeShaderSource` 家族的 uri/source_hash/pipeline desc；路径长度与 hash 立即数测试。 |
| `nani/draw_ext.rs` | 473/236/237 | 行为 | `nani_draw` 分支（count/raw/external）、27 项 max_draws 表、plan 偏移；测试覆盖 6 条路径。 |
| `nani/gpu_world.rs` | 1263/827/436 | 行为 | GpuWorld/子集/实例句柄/27 kind/分配、`extend_no_data`、`update_instance_data`、`import_to_rg`/`export_from_rg`。67 个函数，逻辑量最大。 |
| `nani/instance_data.rs` | 448/296/152 | 合同+行为 | 每类实例 0x30/0x40/0x48… 布局、`CurvedHalfTimber` 三个构造器、marker 组件注册；测试逐字节。 |
| `nani/instance_updates.rs` | 1774/285/1489 | 行为 | 20 余个 `build_*` 纯载荷构建器 + TRS/打包辅助；测试占比高（1489 行）。 |
| `nani/mesh_atlas.rs` | 317/195/122 | 行为 | `MeshAtlas::add` 的合并/半径/属性追加/错误顺序；测试 8 个。 |
| `nani/mod.rs` | 76/76/0 | 声明 | 模块根 + warn-once 展开（原版宏语义）；re-export。骨架 `nani.rs` 的对应文件。 |
| `nani/solid_vertex_color.rs` | 38/38/0 | 混合（**自述未验证**） | 文件头写 “Parallel implementation: build/native/World verification deferred”。`project_solid_vertex_color` 是 64B 记录投影，未与原侧对齐。 |
| `nani/systems.rs` | 886/634/252 | 行为 | `nani_maintain_instances` + 20 个 `nani_update_*` 的行视图实现；写明不依赖真实 Query。 |
| `nani/trimesh.rs` | 527/317/210 | 行为 | builder/attrib/对齐降序/64 三角形分块/包围球；`build` 走设备接口，测试含 device_address。 |
| `normals_debug.rs` | 159/55/104 | 合同 | NormalDebugSsbo = `[u32;8]`（32B，O），`BUFFER_LEN=10000`；**逐字段语义 U**。 |
| `pass_builder/bind_resources.rs` | 331/194/137 | 行为+接缝 | `bind_color_library`/`bind_ray_tracing` 的名字哈希、访问码、uniform 写入逐条 O；`bind_shadowmap` 为空默认（见 §2）。 |
| `pass_builder/compute_pass_builder.rs` | 2323/1136/1187 | 混合 | 最大文件。ComputePass 声明面（bindings/access_ranges/external、rg storage、texture、uniform、dispatch 5 模式）真实；管线创建依赖 `ShaderBackend` 接缝。`panic!("TODO?")`（:864）是原作行为。 |
| `pass_builder/draw_builder.rs` | 2397/1094/1303 | 行为 | DrawPacket/DrawBuilder 全 draw 家族、uniform 写入、texture/storage 绑定、`Drop` 未消费 panic；测试极多。 |
| `pass_builder/draw_list.rs` | 176/137/39 | 混合 | DrawListItem/EntityDrawParams/Topology + 稳定排序；数据合同为主。 |
| `pass_builder/draw_stream.rs` | 1195/716/479 | 混合 | DrawPacket/DrawKind/RasterPass/RasterPassSubmission + `PassHost` 本地 trait 接缝；render 调用宿主统一提交。`DrawStream` 是旧兼容占位（:711，注释自认）。 |
| `pass_builder/framebuffer_binding.rs` | 255/159/96 | 合同 | ColorAttachment 3 变体/DepthAttachment 访问码 0x36/0x2e/0x2f/0x17，均有 O 数值与测试；`From<&Texture>` 是 I 映射。 |
| `pass_builder/pass_resource_access.rs` | 651/409/242 | 行为 | 三个 `*_access_buffer<T>`、绑定上限 0x100、纹理 trait 族（RG/Managed/ByName）。76 个 no_trace 全是泛型单态化（§3）。 |
| `pass_builder/pass_resources.rs` | 285/181/104 | 混合 | 7 个 128 位 TypeId、56B 布局、查找顺序 A..G 均有 O；`ResourceLookup for World` 是登记 typed gap（:116-120）。 |
| `pass_builder/rg_resources.rs` | 263/90/173 | 接缝 | `RgResources` trait + `RecordingRg` 测试记录器；生产由 host 实现。新增文件（骨架无）。 |
| `pass_builder/shader_uri.rs` | 259/198/61 | 行为 | URI 解析、`ComputePipelineDesc`(0x48)、siphash13 哈希、`AsComputeShaderSource` 实现；测试覆盖。 |
| `pass_builder/uniform_binder.rs` | 417/228/189 | 行为 | `UniformBinder::new`/`write_uniform`/`write_optional_uniform` 的守卫顺序与静默失败逐条 O；承接骨架 `uniform.rs`。 |
| `raster_single.rs` | 178/57/121 | 行为 | 单颜色全屏三角形 pass 的 7 步顺序（附件 extent/viewport/DrawBuilder/draw_no_vao(3)/render），3 个泛型实例归并。 |
| `render_config.rs` | 528/400/128 | 行为 | FullscreenConfig/UpscalingPreset/ImageQuality/VideoSettings/upgrade；scale 表与默认值有 O 证据。 |
| `render_temporal_state.rs` | 48/48/0 | 合同 | 0x110 布局、启动/返回/重建三处生命周期（O 行号注释）。 |
| `rg_helpers.rs` | 405/188/217 | 混合 | clear buffer/image、`rg_buffer_copy_from_slice_u32` 的真实顺序；`rg_boundary` 是本地 trait 接缝（:13），不是 rhapsody 本身。 |
| `shader.rs` | 538/286/252 | 混合 | 程序对象/变体缓存/get_or_generate/编译加载真实；**管线创建与 `raster_shader_variants` 是显式 gap**（:252-284）。 |
| `shadowed_buffer.rs` | 162/95/67 | 行为 | CPU 影子 + 闭区间脏窗口 + gpu_flush 的字节范围/失败路径；21 个 no_trace 是单态化（§3）。 |
| `sparse_instance_buffer.rs` | 858/555/303 | 行为 | 32/chunk 稀疏分配、waste 回收、hide/show、dirty flush、draw run 合并；测试 11 个。 |
| `ssbo.rs` | 1135/250/885 | 混合 | 泛型构造/`new_storage_buffer`/`TypedStorageBuffer`/Mapped/InstanceBuffer 真实；设备方法是接缝（默认 None/panic，宿主覆盖）。测试 885 行。 |
| `test_device.rs` | 134/134/0 | 测试接缝 | 仅 `#[cfg(test)]` 模块使用的 CPU 设备。 |
| `utils.rs` | 12/12/0 | **占位** | `calculate_hash` 用 std `DefaultHasher`，注释自认 “Exact algorithm U”；影响 shader_uri 的 source_hash 语义。 |
| `vao.rs` | 155/114/41 | 行为 | `build_vao` 索引/顶点缓冲 flags/名称/布局，与机器码对齐；测试覆盖。 |
| `view_constants.rs` | 314/192/122 | 混合 | `ViewportConstantBuilder::build` 的矩阵链与 jitter 真实；**Default 是显式占位**（:168-190，“尚未逐项核对；暂取单位矩阵/0”）。 |

---

## 2. 空壳与水分核查（对照 STUBS）

### 2.1 `todo!()` / `unimplemented!()`

- 全 `render/**` 扫描：**0 处 `todo!`、0 处 `unimplemented!`**。
- STUBS 列的 5 个 `todo!` 中，只有 `render/pass_builder/compute_pass_builder.rs:814` 落在本范围；该行是 `bind_texture_with_view` 的函数声明，函数体在外部纹理带 view 时 `panic!("TODO?")`（:863-865）。这是**原作行为**（机器码对应的 panic 文案），不是未实现占位。
- 另外 4 个（`resources/world_raster.rs:179`、`systems/render_loop/draw_passes/draw_lists.rs:407/412`、`systems/render_loop/session_selection/mod.rs:491`）不在 `render/**` 范围。

### 2.2 空函数体（严格 `{}`，非测试代码）

| 文件:行 | 函数 | 判定 |
|---|---|---|
| `render/load_texture.rs:19` | `TextureDevice::release_texture` | 宿主默认：托管纹理延迟回收；`TextureHost` 覆盖（render_host.rs:431）。 |
| `render/ssbo.rs:102` | `GraphicsDevice::release_buffer` | 同上，CPU/记录设备无物理资源；`BufferHost` 覆盖（render_host.rs:120）。 |
| `render/pass_builder/bind_resources.rs:69` | `bind_shadowmap` | **残留空壳**：全仓无调用方、无覆盖；原版是 12 处内联、无独立过程。当前阴影绑定走纹理/预设路径。需人工确认是否有行为缺口。 |
| `render/pass_builder/compute_pass_builder.rs:310` | `ImmediateDispatch::track_gpu_lifetime` | 立即 dispatch 无生命周期追踪；真实 ComputeExec 另行实现。 |
| `render/pass_builder/compute_pass_builder.rs:1090` | `ComputePass::read_wall_color_ids` | bind_resources 文档已说明 ComputePass 变体不写 CPU 缓冲（O），属忠实省略。 |
| `render/pass_builder/draw_stream.rs:505` | `PassHost::flush_dynamic_constants` | 接缝默认；真实宿主 `raster_host.rs:707` 覆盖。 |

### 2.3 “只返回常量/Default/None”的平凡体（启发式 12 处）

`adaptive_render_state.rs:28 native_tag->0`、`indirect.rs:17/39/59 empty->Default`、`nani/instance_data.rs:89 marker->default`、`nani/mesh_atlas.rs:52 new->default`、`compute_pass_builder.rs:992 write_guard->None`、`draw_stream.rs:446/489/495 placement_buffer/ssbo_buffer_word/rg_resources->None`、`pass_resources.rs:113 get_resource_ptr->None`、`shader_uri.rs:149 pipeline_defines->Vec::new`。

判定：**都是合同性默认值或可覆盖接缝**。其中 `placement_buffer`/`rg_resources` 由 `raster_host.rs:774/1359` 真实覆盖；`get_resource_ptr` 的 World 实现是登记 typed gap，真实查询由 `tiny-glade/src/frame_world.rs::FrameWorldAccess` 承接（STUBS §4 已说明）。`write_guard->None` 表示“可写”，是正确语义。

### 2.4 注释标注“占位/待取证/未实现/已登记 gap”

- `view_constants.rs:168-190`：Default 单位矩阵，标注“占位，非 O”。
- `utils.rs:4-12`：hash 算法 U，用 std 替身。
- `shader.rs:252-284`：`raster_shader_variants` panic、`create_compute_pipeline`/`create_raster_pipeline` 返回 Err，标注 registered gap §4.9/§2.2。
- `ssbo.rs:116-126`：`mapped_slice_mut` 默认 None、`device_address` 默认 panic（宿主已覆盖）。
- `load_texture.rs:13/29-51`：设备占位 + registered gap §4.8（生产由 TextureHost 覆盖）。
- `pass_resources.rs:18-21`：World typed gap。
- `nani/solid_vertex_color.rs:1-3`：自述并行实现、验证后置。
- `execution_host.rs:1-8`：自述 GPU 真机后置。
- 另有 `draw_stream.rs:711 DrawStream` 为旧占位聚合体（兼容 re-export，无 PDB 过程）。

### 2.5 与 STUBS 数字的对账

| STUBS 项 | STUBS 数字 | 本次在 render/** 复核 |
|---|---:|---|
| `todo!()` | 5（全 crate） | render 内 **0**；唯一相关点是 `panic!("TODO?")`（原作行为）。 |
| 空/平凡函数体 | 14（全 crate） | render 内严格空体 **6**（§2.2）；若把“常量/Default/None”也算入则 12（§2.3）。STUBS 列表中属于 render 的 5 项均能在源码定位。 |
| 空壳（目录表 9+8+1） | 18（render 三目录） | 无法按同一口径复现 18；严格空体 6 + 平凡返回 12，去重后约 14，全部有注释/覆盖。**STUBS 的“空壳”是文本启发式，不能逐条当缺口。** |

结论：render/** 的缺口**不在函数空壳**，而在少量已知接缝（管线创建、设备地址/映射、默认值）与 GPU 真机验证。

---

## 3. 对照骨架：no_trace / 改名 / 搬移 / 真缺失

### 3.1 文件级差异

- 共同文件 35；骨架独有 4；当前独有 12。

骨架独有：
| 骨架文件 | 当前处置 | 判定 |
|---|---|---|
| `render/nani.rs` | `render/nani/mod.rs` + 子模块 | **改名/拆分**。 |
| `render/uniform.rs` | `render/pass_builder/uniform_binder.rs`（+ draw_builder `optional_uniform`） | **搬移/合并**。`write_uniform_to_buffer`→`UniformBinder::write_uniform/write_at`；`bind_lazy_uniform`→`write_optional_uniform`/`DrawBuilder::optional_uniform`。 |
| `render/fsr_plugin.rs` | 无文件、无 `FsrPlugin` 类型 | **缺失（低风险）**：骨架里 `plugin_impl::build` 地址 0x14003bb80 是 1B `ret`（空实现），因此缺的是插件类型/注册身份，不是业务体。 |
| `render/debug_info_buffers.rs` | 无文件 | **无代码缺口**：骨架 PDB 明确 0 个候选接口、结构 unknown。 |

当前独有（新增/迁移的接入与合同）：`device.rs`、`execution_host.rs`、`mesh_atlas.rs`、`nani/instance_updates.rs`、`nani/solid_vertex_color.rs`、`normals_debug.rs`、`pass_builder/rg_resources.rs`、`render_temporal_state.rs`、`rg_helpers.rs`、`test_device.rs`、`utils.rs`。这些在骨架其他路径或本 crate 其他目录有对应；其中 `execution_host.rs` 是重复接缝（§4）。

### 3.2 fn_audit 口径

`render/` 共 507 条骨架函数记录：

| 状态 | 数量 | 含义 | 本次判定 |
|---|---:|---|---|
| `in_file` | 156 | 名字在原文件出现 | 已覆盖（不等于语义等价） |
| `moved` | 5 | 仅在别处找到地址 | derive/空 build/内联，已吸收 |
| `name_only` | 1 | 仅名字命中（`ssbo::fmt`） | Debug derive，已覆盖 |
| `no_trace` | 285 | 名字与地址都无 | **284 个是泛型单态化**；1 个真缺失 |
| `va_only` | 60 | 仅地址被引用 | **全部是泛型单态化** |

唯一非泛型 `no_trace`：`render/mod.rs::AvailableGraphicsDevices::from_preference_order`（0x140987cb0）——**真缺失**。

### 3.3 重点文件

| 文件 | 骨架函数 | no_trace/va_only | 结论 |
|---|---:|---|---|
| `pass_builder/pass_resource_access.rs` | 82 | 76 no_trace + 3 va_only | 76 个形如 `BrickDrawListSsbo>`、`RoofTileSsbo>`… 是 **`read/write_access_buffer<T>` 的单态化**。当前泛型 `read_access_buffer<T>`/`write_access_buffer<T>`/`write_access_buffer_concurrent<T>`（:155-195）覆盖全部；类型参数由各族 `LibrarySsbo` 提供。**不是 76 个缺失函数。** |
| `shadowed_buffer.rs` | 22 | 21 no_trace | 21 个是 `ShadowedBuffer::new<GpuInstance>` / `gpu_flush<…InstanceData>` 等实例。当前泛型 `new`/`gpu_flush`（:34-93）覆盖。 |
| `ssbo.rs` | 107 | 76 no_trace + 20 va_only | 各 SSBO 类型的 `new`/`new_storage_buffer`/`gpu_write_*` 单态化。当前泛型构造器（:339-500）覆盖；每类型的 `BUFFER_LEN`/`default_value` 由生产者模块声明。 |
| `uniform.rs`（骨架） | 2 类内联（84+4+82 实例） | — | 搬移进 `uniform_binder.rs`；**`write_uniform<…, Texture>`（0x14048f750）在现有 `UniformBytes` 中没有对应 impl**，标“需人工/低影响”。 |
| `load_texture.rs` | 1 | 0 | 骨架唯一过程 `load_compiled_texture_image` 在文件内（in_file），真实实现。 |
| `mesh.rs` | 18 | 2 no_trace | `Global>`、`VertexAttributeValues>` = `From<Vec<T>>` 单态化；当前宏 `impl_from_vec!`（:102-113）覆盖。 |
| `compute_pass_builder.rs` | 85 | 64 no_trace + 4 va_only | `ComputePass::new<Source>`、`read_storage_buffer<T>` 的单态化；泛型实现覆盖。 |
| `draw_builder.rs` | 65 | 11 no_trace + 30 va_only | `read_storage_buffer<T>` 等单态化；覆盖。 |
| `nani/gpu_world.rs` | 31 | 19 no_trace + 1 va_only | `create_instances<T>`/`update_instance_data<T>` 单态化；泛型覆盖。 |
| `sparse_instance_buffer.rs` | 4 | 3 no_trace + 1 va_only | `new<BrickTransformSsbo>` 等单态化；覆盖。 |

### 3.4 真缺失 / 被吸收候选

1. **真缺失**：`AvailableGraphicsDevices::from_preference_order`（骨架 `render/mod.rs:88-96`，VA 0x5378702512 / RVA 0x987cb0）。当前无 `AvailableGraphicsDevices` 类型。函数体含 `slice::reverse`，属设备偏好排序业务；**GPU 设备选择后置，但仍是未复现代码**。
2. **被吸收（需人工）**：`InstanceHandle::subset`（骨架 `render/nani.rs:47-54`，inline 3 实例，宿主 `systems::prefab::{update_prefabs,hot_reload_prefabs}`）。当前无同名方法，但 `systems/prefab.rs:682-688` 直接按 `GpuInstanceRecord.kind` 做 subset 命中判断，语义疑似等价；**需人工确认**。
3. **derive/内联，不算缺口**：`impl$2::clone`、`impl$3::component_ids`、`impl$5::get_components`（Bevy `Component` derive）、`fsr_plugin::build`（空 `ret`）。

---

## 4. rhapsody 接线真实性

### 4.1 真实创建与调用（证据）

| rhapsody 面 | 游戏侧落点 | 性质 |
|---|---|---|
| `VulkanGraphBackend::new(device, AccessTable::original())` | `crates/tiny-glade/src/render_bridge.rs:82` 与 `:104` | **真实构造**（生产 `CoreRenderBridge` / `from_owners`）。 |
| 同上 | `country-core/src/render/execution_host.rs:29-33` | 真实构造，但该模块**全仓无调用方** → 死接缝。 |
| `VulkanGraphSession::new/begin_frame/execute_frame/compile_frame/execute_prepared/cancel_frame/collect_completed_frames/wait_submits_done` | `render_bridge.rs:218,246,253,271,292,359,363`；`execution_host.rs:31-50` | **真实调用**。 |
| 图声明 `RenderGraph::add_pass(..).add_resource_access(..).render_with_result_context(..)` | `crates/tiny-glade/src/render_host.rs:1665-1692`（光栅）、`:2576-2605`（compute） | **真实**：core 的 `RecordedComputePass`/`PassHost` 被翻译为 rhapsody 访问范围与回调。 |
| `ExecutionChain` / `ExecutionBackend` | `country-core/src/rhapsody_types.rs:388-391` 仅 re-export；`rhapsody_contract.rs:341` 仅注释 | **游戏侧不直接创建**。rhapsody 内部由 `VulkanGraphBackend::new_execution_chain` 构造（`rg/vulkan.rs:1249-1259`）。 |
| `PassFrame`（core compute 的落点） | `crates/tiny-glade/src/compute_frame.rs:106-113` 实现；内部 `GraphPasses` | 真实。 |
| `RgResources` / `PassHost` / `BufferGraphBackend` | 生产实现分别为 `render_host.rs`（GraphPasses）、`raster_host.rs:658+`（RasterPassHost）、`render_bridge.rs:733-800`（FrameBufferGraph） | 真实宿主实现。 |

### 4.2 本地占位/接缝（不是 rhapsody 真类型）

- `rg_helpers.rs:13 rg_boundary`：本地 trait（`RenderGraph`/`RenderPassApi`/`StagingAlloc`），轻量复刻调用形状。生产由 `CoreComputeFrame`（compute_frame.rs:161+）实现。
- `ssbo.rs:26 rhapsody_boundary`：40B `Buffer`/`BufferDesc`/`GraphicsDevice` 是**类型缝**。`GraphicsDevice` 的默认方法返回 Err/None/panic；生产 `BufferHost`/`TextureHost` 实现真实 rhapsody 调用（`render_host.rs:119-240,430-470`）。
- `pass_builder/draw_stream.rs:420 PassHost`：本地宿主 trait；`raster_host.rs:658` 用真实 rhapsody 图与设备实现。
- `pass_builder/rg_resources.rs:92 RecordingRg`：测试记录器；生产由 host 实现 `RgResources`。
- `rhapsody_types.rs:279 RenderGraph`：本地外观；`import_unowned/export` 在无后端时（release）直接 panic（:305-329），生产由 `FrameBufferGraph` 注入后端。

结论：**不是“全是本地占位/panic 桩”**。核心声明→宿主演译→rhapsody 真图→真 session 的链路存在且被 `production_renderer.rs`/`render_bridge.rs` 使用。问题只有两点：
1. `render/execution_host.rs` 是未被引用的重复接缝（应删除或接入，登记为死代码）。
2. `ExecutionChain`/`ExecutionBackend` 在游戏代码中未直接出现，走的是 `VulkanGraphSession` 封装；没有证据表明需要游戏侧直接构造。

---

## 5. 最大 10 文件抽读结论（真实实现 vs 签名壳）

按非测试行排序，抽读最大 10 个：

1. **`pass_builder/draw_builder.rs`（1094 非测试行）— 真实实现。**
   证据：`finish`/`draw`/`draw_no_vao`/`multi_draw_*` 都写 `DrawKind` 并把 packet 交给 `RasterPass`（:740-853）；`impl Drop` 真实复刻 panic（:1046-1061）；`bind_ray_tracing` 四槽 + 两个 uniform（:2288-2306 测试）。不是签名壳。
2. **`pass_builder/compute_pass_builder.rs`（1136）— 混合，主体真实。**
   `ComputePass::try_new` 走 shader 库缓存/管线注册/UniformBinder（:474-520）；三类 rg buffer 与 texture 绑定、5 种 dispatch 均实现。缺口在 `ShaderBackend` 的管线创建（`shader.rs:252-284`），不在声明逻辑。
3. **`nani/gpu_world.rs`（827）— 真实实现。**
   `allocate`/`extend_no_data`/`write_cull` 有 O 步骤（:443-500），`gpu_flush` 转发 `NaniInstanceData`（:438-441）。19 个 no_trace 是 `create_instances<T>` 单态化。
4. **`pass_builder/draw_stream.rs`（716）— 真实声明层 + 本地宿主接缝。**
   `RasterPass::new` 逐附件登记访问码与组（:568-623），`render` 走 render-pass 缓存并 `submit_raster_pass`（:683-708）。`DrawStream`（:711）是旧兼容聚合体，不是原版过程。
5. **`nani/systems.rs`（634）— 真实实现（行视图适配）。**
   `nani_maintain_instances` 删除→新建→变更的顺序写在模块注释；`nani_update_*` 写 subset/position/sort_key/载荷（:155-200）。
6. **`sparse_instance_buffer.rs`（555）— 真实实现。**
   32/chunk 分配、waste 回收、`replace_all`/`push`/`clear`、dirty flush、draw run 合并（:400-520），16 个测试。非测试行占比高。
7. **`pass_builder/pass_resource_access.rs`（409）— 真实实现。**
   三个泛型访问登记 + 0x100 上限 + 纹理 trait 族；测试覆盖注册先于上限检查、RG 仅导入一次（:587-650）。76 个 no_trace 是单态化。
8. **`nani/trimesh.rs`（317）— 真实实现。**
   对齐降序属性、64 三角形分块、包围球/布局（:151-300），测试含设备地址读取。
9. **`nani/buffers.rs`（301）— 真实实现。**
   27 项跳表、offset defaults 的 108 u32、SSBO 载体注册；测试断言启动长度/策略。
10. **`nani/instance_data.rs`（296）/`shader.rs`（286）— 前者真实，后者混合。**
    `instance_data` 的布局与构造器逐字节测试；`shader.rs` 的 `ComputeShaderProgram`/`RasterShaderProgramVariant::get_or_generate` 真实（:86-176），但 `raster_shader_variants`/两个 `create_*_pipeline` 是显式登记缺口。

---

## 6. 仍未复现清单（带证据）

| # | 项 | 证据 |
|---|---|---|
| 1 | `AvailableGraphicsDevices::from_preference_order` 完全缺失 | 骨架 `render/mod.rs:88-96`；`fn_audit` no_trace 0x140987cb0；当前 grep 全仓无该符号。 |
| 2 | `FsrPlugin` 插件类型/注册缺失 | 骨架 `render/fsr_plugin.rs:...`；全仓 grep 无 `FsrPlugin`。其 `build` 地址 0x14003bb80 为 1B `ret`，故只是类型/身份缺口。 |
| 3 | `render/debug_info_buffers.rs` 无实现 | 骨架文件存在但 “文件级候选接口: 0；结构 unknown”；本次判定无代码缺口，登记为**无信息项**。 |
| 4 | `ShaderBackend::raster_shader_variants` panic | `shader.rs:252-258`（registered gap §4.9，原 0x1417388a0 待取证）。 |
| 5 | `create_compute_pipeline` 未接真实 ShaderLibrary | `shader.rs:259-271` 返回 `Err(unsupported)`；真实路径需 renderer 持有 `ShaderHost`。 |
| 6 | `create_raster_pipeline` 未接真实 ShaderLibrary | `shader.rs:272-284` 返回 Err；游戏侧 `RasterPipeline` 装不下 `Arc<VulkanPipeline>`。 |
| 7 | `ViewConstants::default` 为占位单位矩阵 | `view_constants.rs:168-190` 注释明确“占位，非 O”；影响启动零帧/首帧数据。 |
| 8 | `utils::calculate_hash` 算法未复现 | `utils.rs:4-12` 注释 “Exact algorithm U”；影响 `source_hash` 的等价性（若原版不是 SipHash13-key0）。 |
| 9 | `uniform` 的 `Texture` 单态化无对应 impl | `fn_audit` `render/pass_builder/uniform_binder.rs` no_trace `Texture>` 0x14048f750；`uniform_binder.rs` 全文无 `Texture`。 |
| 10 | `mapped_slice_mut` / `device_address` 默认值不可用 | `ssbo.rs:117-126` 默认 None/panic；真实实现在 `tiny-glade/src/render_host.rs:222-234`（宿主侧，core 内不可用）。 |
| 11 | `load_texture` 的 core 设备实现恒失败 | `load_texture.rs:34-51` 返回 `Err(unsupported)`；生产依赖 `TextureHost`（`render_host.rs:430-470`），core 单独构造不可用。 |
| 12 | `PassResources` 的 `World` 查询恒 None | `pass_resources.rs:116-120` typed gap；真实查询在 `tiny-glade/src/frame_world.rs`（STUBS §4 已述）。 |
| 13 | `bind_texture_with_view` 外部纹理带 view 直接 panic | `compute_pass_builder.rs:863-865`；分支不可用（虽与原版 panic 文案一致）。 |
| 14 | `bind_shadowmap` 空实现且无调用方/覆盖 | `bind_resources.rs:69`；全仓 grep 仅定义本身。需人工确认阴影绑定是否已在别处完成。 |
| 15 | `nani/solid_vertex_color` 仅“并行实现”，未与原侧验证 | `nani/solid_vertex_color.rs:1-3` 自述 build/native/World verification deferred。 |
| 16 | `execution_host` 是死接缝 | `render/execution_host.rs:24-52`；仅 `render/mod.rs:27` 引用，无调用方；与 `render_bridge.rs:81-104` 重复。 |
| 17 | 游戏侧不直接构造 `ExecutionChain`/`ExecutionBackend` | `rhapsody_types.rs:388-391` 仅 re-export；rhapsody 内部 `rg/vulkan.rs:1249-1259`。是否有“必须由游戏侧驱动”的原侧行为，**需人工**。 |
| 18 | GPU 真机/集成路径未验证 | STUBS §0.1 明确未运行 GPU 测试；仓库中存在 `tiny-glade/tests/original_*_gpu.rs`，本次未运行。 |

---

## 7. 完成度评估（GPU 真机单列）

统计基准：47 文件、21,402 行（非测试 11,353 / 测试 10,049）、393 个 `#[test]`；骨架 render 507 条函数记录 + nani.rs 4 条（derive/inline）。

| 维度 | 完成度 | 依据 | 置信度 |
|---|---:|---|---|
| 函数名/地址覆盖 | **99.8%** | 507 条中 506 条可由同名/泛型/搬移解释；真缺失仅 `from_preference_order`（`InstanceHandle::subset` 疑似被吸收）。 | 高（机械统计 + 人工去单态化） |
| 数据合同/布局 | **≈95%** | 大量 `repr(C)` + const assert + O 偏移注释；缺口：`ViewConstants::default` 占位、`normals_debug` 字段 U、`DerivedAtmosphere` 仅启动零值。 | 高 |
| CPU 业务逻辑 | **≈92%** | 最大 10 文件 9 个为真实实现；393 测试且非测试行 11.3k；扣分：`solid_vertex_color` 未验证、`calculate_hash` 占位、`bind_shadowmap`、`Texture` uniform。 | 中高 |
| core→host→rhapsody 接线 | **≈85%** | 真实 `VulkanGraphBackend`/`AccessTable`/`VulkanGraphSession` 与图 add_pass 在 tiny-glade 侧；core `execution_host` 死代码；`ExecutionChain` 未直接使用。 | 中高 |
| **源码级综合（不含 GPU）** | **≈90%** | 以上加权；主要不确定性来自“CPU 测试 ≠ 原侧行为”与少量接缝。 | **中** |
| **GPU 真机运行** | **未验证（0% 证据）** | 本次未编译/未跑任何 GPU 测试；`STUBS` 亦声明 GPU 后置。 | — |

GPU 权重建议：**不计入源码完成度主值**；若必须合成单一数字，建议源码 90% × 0.85 + GPU 0% × 0.15 ≈ 76.5%，但在 GPU 真机通过前该合成值不应对外宣称。

---

## 8. 需人工确认

1. `bind_shadowmap` 的空实现是否造成阴影绑定缺口（原版 12 处内联，重建无调用方）。
2. `InstanceHandle::subset` 是否已被 `systems/prefab.rs:682-688` 完全吸收（语义是否含排序/编号差异）。
3. `uniform_binder` 的 `Texture` 单态化在原版写入的具体字节（dims? id? 8B?）。
4. `utils::calculate_hash` 是否确为 SipHash13-key0（若是，`DefaultHasher` 等价；否则 source_hash 全错）。
5. `nani/solid_vertex_color` 的 64B 记录与边界半径公式是否与原侧 writer 一致（文件自述未验证）。
6. 游戏侧是否需要在 `ExecutionChain` 层（而非 `VulkanGraphSession`）提供原侧的并行录制/PGO 调度。
7. `ViewConstants::default` 的原版 2159B 默认构造（可能不是单位矩阵）。
8. `fsr_plugin` 是否需要在 App 插件列表中出现（原 `build` 为空，可能仅类型身份）。
