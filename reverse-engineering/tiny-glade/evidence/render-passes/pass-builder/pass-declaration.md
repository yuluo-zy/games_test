# country-core pass declaration API and shader URI usage

研究日期：2026-10-13。只读 Ghidra（`--worker shared`）反编译 B patched EXE。本文件回答“游戏层 country-core 如何声明/绑定 pass、实际使用哪些 shader URI”。所有地址来自 `evidence/render-architecture/symbol-seeds.json` / `public-symbols.json`，PDB 名称来自种子；解包反编译正文见本目录 `dec1_/dec2_/dec3_0x<addr>.c`。

标注：**Observation** = 机器码/反编译/种子直接支持；**Inference** = 由名称、调用关系组合；**Unknown** = 未恢复。

## 1. pass 声明 API 形状

country-core 的游戏 pass 构造层（`crates/country-core/src/render/pass_builder/*`）声明资源访问和绘制/dispatch，实际执行闭包交给 `rhapsody::rg::pass_builder::PassBuilder::render`。

### 1.1 ComputePass（compute 侧声明）

**Observation** — 结构体 `ComputePass` 约 400 字节，构造与各声明方法按值返回（`memcpy(in_RCX, in_RDX, 400)` 模式）。

- `ComputePass::new<alloc::string::String>` `0x1405876a0`
  - clone 2 个 `DescriptorSetBinding` 句柄；对 ShaderUri 算 hash；`PassResources::new_from()`（`0x140909310` 从 ECS 世界按 hash 取 7 个资源）；`get_resource<GraphicsDeviceResource>`；`AssetShaderLibrary::get_compute_by_name()` → 命中则复用缓存，未命中则 `shader_uri::impl$1::as_compute_pipeline` + `shader::ComputeShaderProgram::new` 并 `add_compute()`；`UniformBinder::new()`；`bind_resources::impl$0::bind_color_library<ComputePass>()`。
  - 推断：compute shader 程序按名称在 `AssetShaderLibrary` 中缓存。
- `read_rg_storage_buffer` `0x14096dac0` / `write_rg_storage_buffer` `0x14096dd60` / `write_rg_storage_buffer_concurrent` `0x14096e000`：在资源注册表（hashbrown，key = handle hash，字段 +0x98/+0xa0 为桶/掩码）中解析 RG 资源 → `assert_failed<shader_pipeline::reflection::SetIndex>` 校验 set 索引 → `PassBuilder::add_resource_access<PassResources>()` → `rhapsody::rg::pass_api::impl$5::bind()` → 追加 0x44 字节 `RenderPassBinding` 到 Vec（+0/0x8/0x10 容量字段）→ 追加 8 字节 `Range<ResourceAccessIndex>` 到访问范围 Vec。
- `dispatch_1d/2d/3d/groups` `0x14096eaf0/0x14096ec80/0x14096efa0/0x14096ee10`：薄封装，设置 dispatch 组数后调用 `PassBuilder::render<PassResources, compute_pass_builder::impl_0::dispatch_impl::closure_env_0>`。
- `dispatch_with<closure>` `0x14160a2e0`：dispatch 模式置 3，复制 400 字节 closure env，调 `PassBuilder::render<PassResources, dispatch_impl::closure_env_0>`。PDB 实例：`<...gpu_generate_draw_lists::cull_and_bucket::closure_env$1>`。
- `dispatch_indirect_raw` `0x14096e880`：模式置 2；从 `in_RDX+0x10` 读 indirect args buffer 的资源索引；`PassBuilder::add_resource_access<PassResources>()`；调 render。
- `dispatch_indirect_with<closure>` `0x14160b850`：模式置 4；PDB 实例：`<...gpu_generate_draw_lists::group_draw_cmd_prefix_sum::closure_env$0<RenderSublistsIter>>`。
- `bind_external_raw` `0x14096e540` / `bind_external_storage_buffer` `0x14096e2a0`：绑定外部原始/SSBO 句柄。
- `write_to_cpu_mapped_storage_buffer<T>` `0x141609770`（实例 `MouseDepthValueSsbo`）：先 `pass_resource_access::write_access_buffer<T>()`，再登记访问范围。

### 1.2 DrawBuilder（raster/draw 侧声明）

**Observation** — `DrawBuilder` 0xf8 字节。

- `DrawBuilder::new` `0x140905be0`：`FrozenMap::get<...RasterShaderProgram>` 取 raster shader；`rhapsody::raw::dynamic_constants::DynamicConstants::allocate` + `into_uniform_buffer_binding_and_data` 分配动态常量；`HashMap<shader_pipeline::reflection::BindingIndex, DescriptorSetBinding>` 插入绑定。
- `read_rg_storage_buffer` `0x140906150` / `write_rg_storage_buffer` `0x1409067e0`。
- `draw` `0x140906f40`、`draw_no_vao` `0x140906a90`、`draw_no_vao_indexed` `0x140906cb0`、`draw_no_index_buffer` `0x1409083f0`、`draw_multi_instanced` `0x140907050`、`draw_instanced_from_placement_data` `0x140907280`、`draw_instanced_w_gl_draw_flag` `0x140907490`：向 draw command Vec 追加 0xb8 字节命令。
- `draw_indexed_indirect_raw` `0x140907cc0` / `draw_indexed_indirect_count` `0x140907f90` / `multi_draw_indirect` `0x1409076b0` / `multi_draw_indexed_indirect` `0x140907990`：`AssetSsboLibrary::get_rg_buffer<DrawIndexedIndirectBuffer>()` 取间接缓冲；`PassBuilder::add_resource_access<PassResources>()`；在 `+0x40` 置 draw 模式 7；命令步长 0x14（`VkDrawIndexedIndirectCommand`=20 字节）；`DAT_142aaaa78 + in_EDX*0x14` 处读 4 字节常量表。
- `bind_external_storage_buffer` `0x140906400`、`discard` `0x1409082c0`。
- `bind_resources::BindReadOnlyResources` trait 为 DrawBuilder/ComputePass 提供 `read_storage_buffer<T>` 与 `optional_uniform<T>`（public-symbols 中 DrawBuilder 43 个 read_storage_buffer、ComputePass 25 个 read_storage_buffer 实例）。

### 1.3 pass 资源与绑定

- `pass_resources::PassResources::new_from` `0x140909310`：按 hash 从 ECS 世界资源表取 7 个指针 → 7 字段结构（56 字节）。
- `pass_resource_access::read_access_buffer<T>` `0x140970620` / `write_access_buffer<T>` `0x140975920` / `write_access_buffer_concurrent<T>` `0x1409764a0`：`AssetSsboLibrary::get_rg_buffer<T>()` → `PassBuilder::add_resource_access<PassResources>()` → `rhapsody::rg::pass_api::impl$5::bind()` → 追加 0x44 字节 `RenderPassBinding`（上限 0x100）。类型实例（游戏 SSBO）：`AtomicCounterBuffer`、`NaniDrawListLengthSsbo`、`NaniNextVisibilityBitmaskSsbo`、`ColorLibrarySsbo`、`Cobblestone2RasterStagingSsbo`、`Cobblestone3RasterStagingSsbo`、`MouseDepthValueSsbo`、`RtTrianglesSsbo`、`RtTriangleMetaSsbo`、`RtBvhSsbo`、`WallColorIdSsbo`。
- `uniform_binder::UniformBinder::write_uniform<Vec3>` `0x14096b660` / `write_optional_uniform` `0x14096be40`：按 hash 查 UniformBinder 的绑定表 → 把值写入 uniform 数据缓冲（偏移为 binding 记录的 offset）。
- `bind_resources::BindGameResources`：`bind_color_library<DrawBuilder>` `0x1416286f0`（`HashMap<&str, rhapsody::rg::binding::PassResourceSlot>`，键为资源名字符串；绑定 `ColorLibrarySsbo` 读访问）；`bind_ray_tracing<DrawBuilder>` `0x1416289d0`（绑定 `RtTrianglesSsbo`、`RtTriangleMetaSsbo`、`RtBvhSsbo`、`WallColorIdSsbo` 读访问，经 `SsboId` frozen map + 字符串断言）。

### 1.4 draw_stream（高层 raster pass 封装）

**Observation** — `RasterPass`、`DrawPacket`、`DrawStream`、`InstancedDraw`、`TextureBindingKey`、`DrawPassRenderCheck`、`ToTextureDrawStreamBindingKey`（为 `&Texture` 实现 `to_texture_binding_key`）。

- `RasterPass::new_impl` `0x140867970`：`arrayvec::from_iter<RasterPassColorAttachment,5, Map<IterMut<ColorAttachment>, closure>>` 把至多 5 个 `ColorAttachment` 转成 raster 附件；`PassResources::new_from()`；clone 2 个 descriptor binding；建两个 RandomState hash map；写 `+0x7c` 附件计数、`+0x1c` 深度标志。
- `RasterPass::render` `0x140868370`：由 color attachments（每个 0xc 字节）构造 `RenderPassAttachmentDesc` 列表（arrayvec，上限 0x80）；按 `+0x1c != 2` 条件加深度附件并可选 `clear_input`；`HashMap::rustc_entry<RenderPassDesc, RenderPass>` 去重；`GraphicsDevice::create_render_pass()`；最后 `PassBuilder::render<PassResources, draw_stream::impl_11::render::closure_env_0>`。
- `DrawPacket::new` `0x140867850`：默认 `rhapsody::raw::shader::impl$7::default()` shader + 两个 hash map + `+0xac = 0x3f800000`（blend 1.0）。

### 1.5 rhapsody PassBuilder 层

**Observation**（`crates/rhapsody/src/rg/pass_builder.rs`）：
- `PassBuilder::render<UserData=PassResources, closure>` `0x140533d90`：置“已渲染”标志与 pass 函数指针（`anon_..._118_llvm...`），向图的 pass 列表追加 0x40 字节记录；已渲染则 panic。实例：`<PassResources, tiny_glade::screenshot::handle_screenshot_opportunity::closure_env$0>`。
- `PassBuilder::add_resource_access<PassResources>` `0x1409dca90`：按 `(resource_index, access_type, sync_group)` 12 字节记录；`recorder::ResourceSyncGroup::is_equivalent` 判断可合并，兼容则升级访问类型（观测到 0x02/0x17/0x1a/0x1d/0x1e 等访问类型值），不兼容则 `panic_fmt("Previous access of resource already...")`。

### 1.6 ShaderUri

- `shader_uri::ShaderUri::parse` `0x140968960`：对 URI 依次按最后 `:`、最后 `/`、最后 `.` 切分，产出 3 个 `&str` 切片（48 字节）：整串、最后 `/` 后到最后一个 `.` 的名字部分、`:` 后的 entry。URI 格式为 `/<path>/<name>.(cs|raster).hlsl:<entry>`。解析失败 `unwrap_failed`。
- **Observation** — `shader_uri::AsComputeShaderSource` trait（方法 `as_compute_pipeline`、`source_hash`）。其 19 个实现类型（含通用 `T`）即游戏声明的 compute pass 结构体（见 §2）。

## 2. 游戏接线的 pass 家族（shader URI 证据）

### 2.1 AsComputeShaderSource 实现 = compute pass 清单

**Observation** — public-symbols.json 中 `AsComputeShaderSource` 实现（地址见 `symbol-locations.json`）：

| pass 结构体 | 对应 URI（编译产物匹配） |
|---|---|
| `render::nani::cull_shaders::NaniCullAndBucketMain` | `/nani/cull_and_bucket.cs.hlsl:cs_main` |
| `NaniCullAndBucketBush` | `/nani/cull_and_bucket_bush.cs.hlsl:cs_main` |
| `NaniCullAndBucketCurvedHalftimber` | `/nani/cull_and_bucket_curved_halftimber.cs.hlsl:cs_main` |
| `NaniCullAndBucketFlowers` | `/nani/cull_and_bucket_flowers.cs.hlsl:cs_main` |
| `NaniCullAndBucketPlaster` | `/nani/cull_and_bucket_plaster.cs.hlsl:cs_main` |
| `NaniCullAndBucketRoofTile` | `/nani/cull_and_bucket_roof_tile.cs.hlsl:cs_main` |
| `NaniCullAndBucketSmoke` | `/nani/cull_and_bucket_smoke.cs.hlsl:cs_main` |
| `NaniCullAndBucketStairsFloor` | `/nani/cull_and_bucket_stairs_floor.cs.hlsl:cs_main` |
| `NaniCullAndBucketWoodenPlank` | `/nani/cull_and_bucket_wooden_plank.cs.hlsl:cs_main` |
| `systems::render_loop::compute_passes::bin_lights::BinLights` | `/light_binning/bin_lights_to_cells.cs.hlsl:cs_main` |
| `compute_passes::dof::FullResDof` | `/dof/fullres.cs.hlsl:cs_main` |
| `compute_passes::dof::FullResAllLod0Dof` | `/dof/fullres_all_lod0.cs.hlsl:cs_main` |
| `compute_passes::dof::FullResAllLod1Dof` | `/dof/fullres_all_lod1.cs.hlsl:cs_main` |
| `compute_passes::dof::HalfResDof` | `/dof/halfres.cs.hlsl:cs_halfres` |
| `compute_passes::dof::HalfResAllDof` | `/dof/halfres.cs.hlsl:cs_halfres_all` |
| `compute_passes::rasterize_terrain_stroke::GaussBlur` | `/gauss_blur.cs.hlsl:cs_main` |
| `compute_passes::taa::TgsrMain` | `/tgsr.cs.hlsl:cs_main` |
| `vermeer::GenerateStrokes` | `/vermeer/generate_strokes.cs.hlsl:cs_main` |

### 2.2 ComputePass dispatch 闭包实例（接线证据）

**Observation** — PDB 单态化名称：
- `ComputePass::dispatch_with<...gpu_generate_draw_lists::cull_and_bucket::closure_env$1>` `0x14160a2e0`
- `ComputePass::dispatch_indirect_with<...gpu_generate_draw_lists::group_draw_cmd_prefix_sum::closure_env$0<RenderSublistsIter>>` `0x14160b850`

即 `gpu_generate_draw_lists`（Nani GPU-driven 第一轮）由 cull_and_bucket compute 与 group_draw_cmd_prefix_sum 间接 compute 组成。

### 2.3 二进制中的 shader URI 全集

**Observation** — 在 `tiny-glade.exe` 中按字节正则扫描得到 255 条游戏 shader URI（`/<path>/<name>.(cs|raster).hlsl[:entry]`），其中 **254 条与 `compiled-shaders/` 404 个 .bin 文件名匹配**（`shader-uri-list.json`）。计算类 default entry 为 `cs_main`。代表：

- Nani GPU cull/bucket + draw-list 家族：`/nani/cull_and_bucket*.cs.hlsl:cs_main`、`/nani/{histogram_prefix_sum,group_draw_cmd_prefix_sum,subtotal_prefix_sum,allocate_draw_commands,generate_draw_lists,draw_indexed_indirect_to_dispatch,...}.cs.hlsl`；raster 侧 `/nani_*.raster.hlsl`（uber/albedo_map/vertex_color/plaster/smoke/window 等 25 个）。
- Wall：`/wall/generate_draw_lists.cs.hlsl:cs_pass1`、`:cs_pass2`、`/wall/reset_pass2_inst_counts.cs.hlsl`。
- 光照/GI：`/light_binning/bin_lights_to_cells.cs.hlsl:cs_main`、`/light_binning/{cell_prefix_sum,cell_subtotal_prefix_sum,cell_prefix_merge}.cs.hlsl`；`/gi/{trace_gi_candidates,update_gi_radiance,reproject_screen_radiance,blur_screen_radiance,eval_screen_radiance}.cs.hlsl`；`/ssgi/{ssgi,temporal_filter,upsample}.cs.hlsl`。
- 阴影/体积：`/shadow_mask_mean.cs.hlsl`、`/apply_screen_shadows.cs.hlsl`、`/apply_volumetrics.cs.hlsl`。
- HZB/深度：`/generate_hzb.cs.hlsl`、`/extract_half_res_depth_normals.cs.hlsl`、`/copy_depth_to_r.cs.hlsl`。
- TAA/TGSR：`/taa.cs.hlsl`、`/tgsr.cs.hlsl:cs_main`、`/tgsr_reproject.cs.hlsl`、`/tgsr_blur_lum.cs.hlsl`、`/calculate_reprojection_map.cs.hlsl`。
- DoF：`/dof/{prepare,analyze,init_buffers,clear_halfres,halfres,fused,fullres,fullres_upsample,fullres_all_lod0,fullres_all_lod1,autofocus}.cs.hlsl`。
- 地形/建筑/路径/草/水/屋顶/粒子：`/upload_cpu_raster.cs.hlsl`、`/terrain_editing/*.cs.hlsl`、`/rasterize_buildings.cs.hlsl`、`/paths/*.cs.hlsl`、`/path_pebbles.cs.hlsl`、`/grass/*.cs.hlsl`、`/water/*.cs.hlsl`、`/roof_tiles_gravity.cs.hlsl`、`/leaf_particles.cs.hlsl` 等。
- 主绘制/后处理/UI：`/deferred/{solid,grass,plant,canopy}_light_sun_sky.raster.hlsl`、`/final_post.raster.hlsl`、`/blit/*.raster.hlsl`、`/skydome.raster.hlsl`、`/cloud.raster.hlsl`、`/ui_2d.raster.hlsl`、`/egui.raster.hlsl`、`/text.raster.hlsl`、`/loading_screen.raster.hlsl`、`/custom_cursor.raster.hlsl` 等。

**Inference** — findings.md 中的帧构图 pass 名（`shadow_pass`/`main_pass`/`late_forward_and_deferred_pass`/`deferred_raster`）在 `compiled-shaders/` 中没有同名 shader 文件；它们是对上述 URI 家族的组合（shadow 复用 nani_* 光栅 shader；main/late 由 nani_* 绘制 + deferred/*_light_sun_sky 光照 + final_post 组合）。FSR 相关编译产物（`fsr_copy_history`、`fsr_init_reactive` 等）在二进制中无 `/<name>.cs.hlsl` 形式 URI（见 gaps）。

## 3. 直接证据文件

- 反编译：`dec1_/dec2_/dec3_0x<addr>.c`（本目录，55 个函数）
- 种子→地址→已反编译映射：`symbol-locations.json`
- URI↔compiled-shaders 匹配：`shader-uri-list.json`（含 gaps）
- 完整 404 文件名清单：`compiled-shaders-full-list.txt`

## 4. Unknown / 未覆盖

- `ColorAttachment::desc` `0x1409092f0` 反编译为空（void() 占位；真实逻辑被内联），未核实。
- `draw_list.rs`、`render/indirect.rs`、`render/nani/cull_shaders.rs` 在 skeleton 中无独立 procedure（多为泛型/内联），只从 trait 实现名确认。
- 每个 draw/dispatch 模式的枚举判别值（2/3/4/7）未逐一映射到原始 Rust 枚举名。
- 运行时动态拼接的 URI（如 `/clearing/<name>.raster.hlsl`）只观测到静态前缀。
- 具体的 RenderPassBinding（0x44 字节）字段布局与 descriptor set 的实际句柄未恢复。
