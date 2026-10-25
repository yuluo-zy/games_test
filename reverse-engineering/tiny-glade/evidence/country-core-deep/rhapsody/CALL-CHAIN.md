# country-core ↔ rhapsody 渲染调用链调查

- 日期：2026-10-15。基线：B patched EXE（SHA cf048e27...）；原始指令 oracle 为发行 RNE。
- 范围：游戏侧 API 表面（`rhapsody_contract`、`rg_helpers`、pass 声明层）→ 原 rhapsody 真实入口
  （render graph 编译/执行/资源）的完整调用链；reconstruction 现状 vs 原机器码差距。
- 数据来源：`reconstruction/country-core/src`（代码面）、`reconstruction/rhapsody-rebuild`（候选实现面）、
  原 PDB 骨架 `skeleton/B/crates/rhapsody`（原函数符号面）、RE-MCP 反编译（`batch-report.json`，dbid `tg_rl2`）、
  `evidence/render-passes/cpu-frame/evidence.json`（已解析帧序）、rne-read 原字节直读。
- 标注：**O**=机器码/PDB 实测，**I**=推断，**U**=未知。

## 0. 结论速览

1. **调用链是三层**：`tiny_glade::render_loop`（帧编排，render/render_world）→ `country_core`（pass 声明与
   资源接线，54 个 pass 入口 + PassResources）→ `rhapsody::rg`（图编译/录制/执行）→ `rhapsody::raw`（Vulkan）。
   `country_core` 与 `rhapsody` 之间**唯一的共享数据契约是 `PassResources`**（整张图泛型 `RenderGraphRecord<PassResources>` 的 UserData）。
2. **原机器码边界**（O）：
   - 编译链：`render`(0x1404e8010) → `RenderGraphState::compile`(rhapsody，0x140161cd0 世界图 / 0x1401647b0 加载图 /
     0x140166510 展示图) → 闭包内 `render_world`(0x1404ec110，tiny_glade) + country_core 各 pass → 闭包尾部
     **最后一个 country_core 调用 `GpuWorld::export_from_rg`(0x140992220)** → 第一个 rhapsody 函数
     `TemporalRenderGraphRecorder::export_temporal`(0x1404f7ca0) → `ResourceInfo::calculate`(0x140530430) →
     `ResourceRegistry::populate_resources`(0x140be7ba0) → `CompiledRenderGraph::new`(0x1402c4280)。
   - 录制/提交链：render → **最后一个 country_core 函数 `PassResources::new_from`(0x140909310)** →
     `GraphExecutionRecorder::execute_compiled`(0x1400fa190, rhapsody) → `BatchCompiler::flush_batch`(0x140be9600) →
     `submit_pending_cbs`(0x140bebdf0) → `ensure_active_cb`(0x140bebc90) → `CompiledRenderGraph::record_batch`(0x1402c1d40，
     经 `bevy_tasks::ComputeTaskPool` 并行) → `ResourceRegistry::into_exports`(0x140be9040) →
     `ExecutedRenderGraph::new`(0x140bea180)。
3. **reconstruction 现状**：country-core 从未 `use rhapsody::` 调用 rhapsody-rebuild（Cargo.toml 声明了 path dep
   但源码零引用，51 处 `rhapsody::` 全在注释里）。所有 rhapsody 表面都是 crate 内 stub：
   `rhapsody_contract`（lib.rs:82–330）、`rhapsody_types.rs`、`render/ssbo.rs::rhapsody_boundary`、
   `render/shader.rs::boundary`、`render/rg_helpers.rs::rg_boundary`、`render/pass_builder/rg_resources.rs::RgResources`。
   帧级 trait（`FramePasses`/`RenderFrameSurface`/`PassHost`/`RgResources`）**只有测试 Mock 实现，无生产接线**。
4. **rhapsody-rebuild 对得上/对不上**：以原 PDB 853 个 rhapsody 符号为基准，游戏侧实际调用的 rhapsody 入口约
   **30 个**；rhapsody-rebuild 已恢复其中 **约 19 个**（rg::graph/compiled/executor/pass_api/pass_builder/temporal 主体、
   raw::device 一半、raw::buffer/swapchain/submit/barrier），**约 11 个对不上或缺失**（见 §5 差距表）。

## 1. 期望调用链（reconstruction 代码级）

### 1.1 country-core 内所有碰 rhapsody 的文件与所用表面

| 文件 | 用的 rhapsody 表面 | 签名/状态 | stub/todo 位置 |
|---|---|---|---|
| `src/lib.rs` `rhapsody_contract`（82–330） | `GraphicsDevice(pub u64)` + `create_persistent_descriptor_set`/`update_buffer_bytes`/`update_persistent_descriptor_set`/`full_barrier_required`/`issue_full_pipeline_barrier`/`dispatch`；`DescriptorSet`；`ManagedHandle(pub u64)`；`Handle<T>`（`gap_new`/`handle()`/`get()`）；`ReadOnlyHandle<T>`；`ResourceHandle(u64)`；`Extent2D`；`RenderPassDesc{selector:u32}`；`RenderPass`；`RenderPassAttachmentDesc`；`PassResourceSlot`；`RenderPassBinding{resource_index,access_type}`；`bind()`；`PassBuilder<'a,UD>`（`add_resource_access`/`resolve_resource`/`render`/`detached`）；`RenderGraph`；`PipelineError`/`TextureError`；`StorageBufferDescriptor`/`StorageBufferDescriptorImpl<T>` | 全部占位实现（不触 GPU） | `Handle::get`/`ReadOnlyHandle::get` 是 `todo!`（155–157、176–178 行）；`resolve_resource` 返回 (0,0)；`render` 空 |
| `src/rhapsody_types.rs` | `TextureDesc`（32 B，O 布局）、`TextureViewDesc`（0x3c B，O）、`Texture{id,width,height,format}`、`CompiledShaderPipeline(Vec<u8>)`、`RasterPipelineDesc{path,entry,defines}`、`RasterPipelineKey{handle,selector}`、`RasterPipeline`、`ComputePipeline{id}`、`ImportedBuffer`、`RenderGraph{imports,exports}`（占位，只记录导入导出） | 占位，字段以机器码核对 | `Texture::handle()` 返回 id 低 32 位（U）；`RenderGraph` 无编译/录制 |
| `render/ssbo.rs` `rhapsody_boundary`（13–129） | `usage` 常量（O 0x22/0x12/0x40/0x80）；`Buffer`(40 B)；`BufferDesc::new/new_gpu_only/alignment`；`BackendError`；trait `GraphicsDevice{create_buffer,update_buffer_bytes,mapped_slice_mut,device_address}`；`slice_as_bytes`；`ResourceHandle(u64,u64)`；`buffer_cpu_ptr` | 占位；`impl GraphicsDevice for rhapsody_contract::GraphicsDevice`（134–164）只回填 byte_size、映射用泄漏零页 | `mapped_slice_mut` 默认 None（真实 = rhapsody 持久映射）；`device_address` 默认 0 |
| `render/shader.rs` `boundary`（33–61） | `CompiledShaderPipeline`/`ComputePipeline`/`RasterPipeline*`（从 rhapsody_types 再导出）；trait `ShaderBackend{load_compiled_compute,load_compiled_raster,raster_shader_variants,create_compute_pipeline,create_raster_pipeline}`；`impl ShaderBackend for rhapsody_contract::GraphicsDevice`（161–181）返回空壳产物 | 占位（编译器库与设备压在同一 trait 上，原版是两个库） | — |
| `render/rg_helpers.rs` `rg_boundary` | `ACCESS_TRANSFER_WRITE=0x34`；trait `RenderPassApi{fill_buffer,clear_color_image,copy_buffer}`；`StagingAlloc`；`CopyRegion`；后续 `PassBuilder::new`/`add_resource_access`/`render` 模板 | 占位（只复刻调用形状） | 渲染图本身 rhapsody::rg 无实现 |
| `render/pass_builder/compute_pass_builder.rs` | `ComputePass`（`new`/`read_rg_storage_buffer`/`write_rg_storage_buffer[_concurrent]`/`bind_external_storage_buffer`/`bind_external_raw`/`uniform`/`push_constants`/`bind_texture[_with_view]`/`dispatch_1d`…）；`RecordedComputePass`；`ComputeExec{full_barrier_enabled,issue_full_pipeline_barrier,count_dispatch,cmd_dispatch,cmd_dispatch_indirect,local_size,push_constants}`；`ImmediateDispatch`（0x14096efa0 逻辑）；`DispatchMode` 判别 0/1/2/3/4；`ResourceRegistry::get(hash)->Option<(set,res)>`；`RgBuffer`；`ComputePassDevice`（`get_vk_buffer`/`pipeline_registry`） | 声明/记录层完整（O），执行层经 `ComputeExec` trait 接 rhapsody | `get_vk_buffer` 等由宿主实现（缺生产宿主） |
| `render/pass_builder/pass_resource_access.rs` | `RecordingPassBuilder::add_resource_access(resource,access,group)`；`PassBinding::to_contract()->rhapsody_contract::RenderPassBinding`（98 行）；`TextureDevice::extent_2d`→`Extent2D`；`ssbo_to_rg_handle` | O | — |
| `render/pass_builder/draw_builder.rs` | `DrawBuilder`（`draw`/`draw_no_vao[_indexed]`/`draw_no_index_buffer`/`draw_instanced_w_gl_draw_flag`/`draw_multi_instanced`/`draw_indexed_indirect_raw[_count]`/`draw_instanced_from_placement_data`，0x140906f40 起各入口）；`ManagedHandle`/`RenderPassBinding` 用于 draw 种类载荷 | 声明层 O；执行经 `PassHost` | — |
| `render/pass_builder/draw_stream.rs` | `PassHost` trait（398–460）：`raster_shader`/`allocate_dynamic_constants`/`rg_buffer_resource`/`placement_buffer`/`vk_buffer`(`get_vk_buffer`)/`vk_image_view`(`get_vk_image_view`)/`texture_by_name`/`rg_texture`/`with_suffix_dims`/`default_raster_state`(`rhapsody::raw::shader::impl$7::default`)/`create_render_pass`/`render_pass_cache`(+0x188 缓存)/`submit_raster_pass`(`PassBuilder::render`)；`RasterPassSubmission`；`RasterState` | 调用面 O；实现者是 rhapsody + 游戏资源库 | `ssbo_buffer_word`/`rg_resources` 带缺省（离线测试） |
| `render/pass_builder/rg_resources.rs` | `RgResources` trait（51–66）：`create_raw_texture`/`create_raw_buffer`（0x140be94d0）/`get_or_create_temporal_texture[_pair]`（0x1404f6e50/0x1404f7450）/`get_or_create_temporal_buffer`；`TemporalTexture`/`TemporalTexturePair`/`TemporalBuffer`/`RgError`；`RecordingRg` | trait O；`RecordingRg` 为离线记录实现 | 真实实现应由 rhapsody RenderGraph 提供（未接） |
| `systems/render_loop/render_world.rs` | `FramePasses` trait（225–404）：54 个 pass 入口 + `is_winter`/`display_dims`/`dof_mode`/`flag_c0d8`/`flag_c098`/`get_or_create_temporal`/`get_or_create_temporal_pair`/`create_raw_resource`；`TextureDesc`（O 布局）/`RgTexture`/`TemporalTexture`/`TemporalPair`/`PassHandle` 等 sret 类型；`render_world<P:FramePasses>`（418 起，0x1404ec110 逐调用还原） | 编排 O 完整 | `FramePasses` 只有测试 `Rec` 实现（640 行） |
| `systems/render_loop/render_frame.rs` | `RenderFrameSurface` trait（约 95–200 行）：`compile_loading_graph`(0x1401647b0)/`compile_world_graph`(0x140161cd0)/`begin_frame`(`GraphExecutor::begin_frame`)/`drain_finished_frames`/`execute_main_graph`(`PassResources::new_from`+`execute_compiled`+`retire`)/`acquire_swapchain_image`/`execute_presentation_graph`/`end_frame`/`present`/`snapshot_render_stats`/`refresh_memory_budget`/`last_gpu_frame_ns`/`wait_gpu_queue_trim`；`render`(0x1404e8010) 逐步骤还原（R1–R20） | 编排 O 完整（16279 B 全吃透） | trait 无生产实现 |

### 1.2 期望调用链图（代码级，两侧各自现状）

```
tiny_glade::render_loop::render (render_frame.rs::render, 0x1404e8010)
  ├─ build_draw_lists / DrawLists::sort                    (country_core draw_passes)
  ├─ Swapchain::format/extent                              → rhapsody::raw::swapchain
  ├─ RenderFrameSurface::compile_world_graph               → (期望) rhapsody::rg::state::RenderGraphState::compile
  │    闭包 = render_world (render_world.rs::render_world, 0x1404ec110)
  │      ├─ FramePasses::get_or_create_temporal[_pair]     → (期望) rhapsody::rg::temporal::GetOrCreateTemporal(trait)
  │      ├─ FramePasses::create_raw_resource               → (期望) rhapsody::rg::graph::RenderGraph::create_raw_resource
  │      ├─ W1–W54 pass 入口 (compute_passes::* / draw_passes::*)
  │      │    ├─ ComputePass::new → ComputeShaderProgram::new → ShaderBackend::load_compiled/create_pipeline → (期望) shader_pipeline / GraphicsDevice::create_compute_pipeline
  │      │    ├─ ComputePass::read/write_* → RecordingPassBuilder::add_resource_access → (期望) rhapsody::rg::pass_builder::PassBuilder::add_resource_access
  │      │    ├─ RecordedComputePass::execute → ComputeExec → (期望) rhapsody RenderPassApi (dispatch_groups_3d / issue_full_pipeline_barrier / bind_descriptor_set)
  │      │    └─ DrawBuilder::draw* → (期望) rhapsody::rg::pass_builder::PassBuilder::render<PassResources, draw_stream::impl$11::render::closure>
  │      └─ PassResources::new_from(world) 产生的 PassResources = 图泛型 UserData
  │    闭包尾部：RenderContext::finish / export_* → (期望) rhapsody::rg::import_export + temporal::export_temporal
  │    → (期望) ResourceInfo::calculate → ResourceRegistry::populate_resources → CompiledRenderGraph::new
  ├─ GraphExecutor::begin_frame / profiler
  ├─ PassResources::new_from (country_core, 最后一个 game 函数)   ← 真实边界
  ├─ GraphExecutionRecorder::execute_compiled                → rhapsody::rg::executor
  │    └─ CompiledRenderGraph::record_batch (bevy_tasks 并行) → 逐 pass 回调: game pass 闭包 (FnMut(PassContext))
  ├─ ExecutedRenderGraph::retire / take_resource / from_owned_rg_resource
  ├─ validate_swapchain_and_acquire_image
  ├─ PassResources::new_from → GraphExecutionRecorder::execute (展示图: 内部 RenderGraphState::compile 0x140166510)
  ├─ TrackedSwapchainTextureLease::into_present_source
  ├─ end_frame (submit_pending_cbs) / Swapchain::present
  └─ last_known_frame_duration_ns / wait_gpu_queue_trim
```

**当前 reconstruction 断点**：上面所有"（期望）"处，country-core 里都是 crate 内 stub（§1.1 表）；
`FramePasses`/`RenderFrameSurface`/`PassHost` 无生产实现；rhapsody-rebuild 虽声明为 path dep，从未被引用。

## 2. 原机器码调用链（反编译级）

反编译证据落盘 `decomp/`（RE-MCP dbid `tg_rl2` 重反编译）+ `resolved-calls.json`（调用目标→PDB 名解析，S_PUB32）。

### 2.1 帧外层 `render`（0x1404e8010）→ rhapsody 调用点（按函数内偏移）

| 偏移 | 目标 | 真名（S_PUB32） | 帧步 |
|---|---|---|---|
| 0x3eaa/0x3fd0 | 0x1409ba540/0x1409ba4b0 | country_core `draw_lists::build_draw_lists`/`DrawLists::sort` | R2 |
| 0x4230 | 0x140c0ea90 | rhapsody `raw::swapchain::Swapchain::extent` | R3 |
| 0x4974 | 0x1401647b0 | rhapsody `rg::state::RenderGraphState::compile<ResourceExportHandle<Texture>,PassResources,compile_loading_screen_render_graph::closure_env$0>` | R4（mode==0 加载图） |
| 0x5455 / 0xc622 | 0x140161cd0 | 同上 `…compile_main_render_graph::closure_env$0>`（世界图；0xc622 为降分辨率重试） | R4' |
| 0x6d23 | 0x140be76f0 | rhapsody `rg::resource_pool::ResourcePool::clear` | — |
| 0xa35b | 0x140bf8e60 | rhapsody `raw::device::GraphicsDevice::wait_idle_and_recycle_resources` | — |
| 0xd9df | 0x140bff4a0 | rhapsody `raw::profiler::…GraphicsDevice::create_gpu_profiler_output_buffer` | R5 |
| 0xee52 | 0x140beabd0 | rhapsody `rg::executor::GraphExecutor::begin_frame` | R5 |
| 0xfc02 | 0x140294cf0 | gpu_profiler `ProfilerSubmittedFrame::retrieve_results` | R6 |
| 0x1115b/0x1123f | 0x140bd3cb0/0x140bd4030 | rhapsody `raw::profiler::TimedFrame::new/send_to_puffin` | R6 |
| 0x118f0 / 0x13593 | 0x140909310 | **country_core** `render::pass_builder::pass_resources::PassResources::new_from`（两处：主图/展示图前） | R7 / R12 |
| 0x11a10 | 0x1400fa190 | rhapsody `rg::executor::GraphExecutionRecorder::execute_compiled` | R8 |
| 0x11b6c/0x11fde | 0x140be9d00/0x140be9c80 | rhapsody `ExecutedRenderGraph::retire`/`ExportedRenderGraphResources::take_resource` | R9 |
| 0x12129 | 0x140bd63f0 | rhapsody `raw::texture::Texture as import_export::ImportExportToRenderGraph::from_owned_rg_resource` | R9 |
| 0x1254b | 0x140bfd9d0 | rhapsody `raw::swapchain::…GraphicsDevice::validate_swapchain_and_acquire_image` | R10 |
| 0x13f4b | 0x1400fb5b0 | rhapsody `rg::executor::GraphExecutionRecorder::execute`（展示图；内部 0x140166510 compile） | R13 |
| 0x14e66 | 0x140c0f170 | rhapsody `TrackedSwapchainTextureLease::into_present_source` | R14 |
| 0x15256 | 0x140bec400 | rhapsody `GraphExecutionRecorder::end_frame` | R15 |
| 0x154bf | 0x140c0ef80 | rhapsody `raw::swapchain::Swapchain::present` | R16 |
| 0x15610 | 0x14031ea20 | tiny_glade `screenshot::finish_screenshot_capture` | R17 |
| 0x15f25/0x1810c | 0x140beaae0/0x140bea680 | rhapsody `GraphExecutor::last_known_frame_duration_ns`/`wait_gpu_queue_trim` | R19/R20 |

### 2.2 `RenderGraphState::compile`（0x140161cd0，世界图实例）内部链

函数体 = 泛型单态化：rhapsody `compile` + **内联的 tiny_glade 闭包** `compile_main_render_graph::closure_env$0`。

```
0x140161cd0（rhapsody::rg::state::RenderGraphState::compile<ResourceExportHandle<Texture>, PassResources, closure_env>）
  ├─ [内联闭包前段] 0x14093cd30 import_texture_presets_to_rg、0x1409921c0 GpuWorld::import_to_rg、0x14090e280 render_luts、
  │   0x14090a870 reset_indirect_args、0x140c02740 DynamicConstants::allocate、0x140c02370 …into_uniform_and_storage_buffer_binding
  ├─ 0x10767 0x140be94d0 RenderGraph::create_raw_resource ×4
  ├─ 0x1117c.. 0x1404ec110 render_world ← ← ← 闭包主体（54 pass，见 §2.3）
  ├─ [闭包尾部 country_core] 0x13349 ColorGradingState::flattened → 0x13ab3 bidirectional_blur_pyramid → 0x148a8 final_post →
  │   render_menu_bg / painterly_pass / vermeer_pass / logo_overlay / session_selection / lut_gallery / feedback_report_bg /
  │   wireframe_pass / ui_pass / ui_2d_pass / mei_pass / custom_cursor_pass / loading_screen_pass / EguiBackend::draw /
  │   capture_image → 0x19dd2 Texture::export(ImportExportToRenderGraph) → 0x1a147 RenderContext::finish →
  │   0x1a994 export_preset_textures_from_rg → 0x1b194 AssetSsboLibrary::export_from_rg → 0x1ba0f GpuWorld::export_from_rg
  │   ← ← ← **最后一个 country_core 调用**
  ├─ 0x1c2ff 0x1404f7ca0 rhapsody TemporalRenderGraphRecorder::export_temporal ← ← ← **第一个 rhapsody 函数**
  ├─ 0x2116a 0x140530430 rhapsody ResourceInfo::calculate
  ├─ 0x…    0x140be7ba0 ResourceRegistry::populate_resources
  └─ 0x…    0x1402c4280 CompiledRenderGraph::new
```

加载图实例 0x1401647b0 同构（闭包 = `loading_background_path` → image::open/to_rgba8 → create_texture →
`vermeer_render_from_target`/`mei_pass`/`loading_screen_pass` → 同样导出序列 → `ResourceInfo::calculate` → populate →
`CompiledRenderGraph::new`），**不调用 render_world**。
展示图实例 0x140166510 由 `GraphExecutionRecorder::execute`（0x1400fb5b0）内部调用，闭包 = `execute_presentation_render_graph`。

### 2.3 `render_world`（0x1404ec110）→ rhapsody 调用点

| 偏移 | 目标 | 真名 |
|---|---|---|
| 0x1816 | 0x1404f6e50 | rhapsody `TemporalRenderGraphRecorder<UD> as GetOrCreateTemporal<TextureDesc>::get_or_create_temporal`（"reprojection.prev_depth"） |
| 0x1996–0x1d80 | 0x1403d8bd0…0x140a68a50 | country_core W1–W21 pass 入口（54 个中前段） |
| 0x209b/0x23a2 | 0x140be94d0 | rhapsody `RenderGraph::create_raw_resource`（gbuffer / velocity，0x1404ec404/0x1404ec48e） |
| 0x2649–0x2ae2… | 0x140833bc0… | country_core 主 pass / HZB / GI 入口 |
| （中部）0x1404f7450 | 同上 pair 实例 | `GetOrCreateTemporalPair<TextureDesc>::get_or_create_temporal_pair`（taa/tgsr_history） |

（完整 68 个调用目标 + 54 pass 逐一解析见 `resolved-calls.json` 的 `0x1404ec110` 表与
`evidence/render-passes/cpu-frame/evidence.json`。）

### 2.4 录制/提交链（`execute_compiled` 0x1400fa190 内部）

```
0x1400fa190 GraphExecutionRecorder::execute_compiled<PassResources>
  ├─ 0x140be9600 BatchCompiler::flush_batch ×2（批量编译边界）
  ├─ 0x140bebdf0 submit_pending_cbs ×2（待提交命令缓冲）
  ├─ 0x140bebc90 ensure_active_cb（逐 pass 活动命令缓冲）
  ├─ 0x1402c1d40 CompiledRenderGraph::record_batch ← 并行录制入口
  │    内部（0x1402c1d40.c）：0x140becfd0 CommandBufferSeqTracker::track ×4、0x140be78b0 ResourceRegistry::resource ×3、
  │      0x140bd6c80 RuntimeResource::borrow ×3、0x140bd66a0/0x140bd6810 SynchronizationScope1 image/buffer_barrier、
  │      0x140bee030 issue_pipeline_barrier、0x140bed3d0 CommandBuffer::end、0x140bf09b0/0x140bf0b40 begin/end_label、
  │      0x140bea1e0 PassPgoStats::duration_ns、**0x140d45730 bevy_tasks::ComputeTaskPool::get +
  │      0x1401058a0 TaskPool::scope_with_executor_inner**（并行块）、0x140c110f0 gpu_profiler create_marker
  ├─ 0x140be9040 ResourceRegistry::into_exports
  └─ 0x140bea180 ExecutedRenderGraph::new
```

`end_frame`（0x140bec400）= `submit_pending_cbs` + `ensure_active_cb` + profiler 收尾（`CommandBufferState::fmt` 是 panic 调试
路径）+ RawManagedHandle 回收，与 rhapsody-rebuild `ARCHITECTURE.md` 的 PGO 帧切换描述一致。
`get_or_create_temporal`（0x1404f6e50）内部：`Texture::import_unowned`(0x140bd60a0) + **`ustr::Ustr::fmt`**（键是 intern 字符串）
+ `BuildHasher::hash_one` + HashMap insert + `create_texture` + anyhow `with_context`。
`populate_resources`（0x140be7ba0）内部：`ResourcePool::take_texture`(0x140be6b90) → `create_texture`(0x140c07950) →
`set_texture_debug_name`(0x140bfe650)；`ResourcePool::take_buffer`(0x140be71e0) → `create_buffer`(0x140c03460) →
`set_buffer_debug_name`(0x140beec30)。

## 3. 两侧 API 对照表

### 3.1 rg 层（原 PDB 853 符号 → 游戏实际调用面 → rhapsody-rebuild）

| 原 PDB 真名（游戏实际调用入口） | 地址 | 参数面（O） | rhapsody-rebuild 现状 |
|---|---|---|---|
| `rg::state::RenderGraphState::compile<ResourceExportHandle<Texture>,PassResources,closure_env>` ×3 实例 | 0x1401647b0 / 0x140161cd0 / 0x140166510 | 前两实例返回 `ResourceExportHandle<Texture>`；展示图实例返回 `()` | **半对上**：有 `RenderGraphState{compile_frame}` + `PersistentCompileState::take_for_compile`（注释引用 0x140166510 段 ✓）；但**无 `ResourceExportHandle` 返回类型、无 compile 的三个游戏闭包实例、编译失败语义为 Result（原 OOM 降分辨率路径未建模）** |
| `rg::resource_info::ResourceInfo::calculate<PassResources>` / `calculate_lifetime_events` / `extract_creation_flags` / `extend_usage_flags_for_exports` / `clear` | 0x140530430 等 | `(&mut self, graph, registry…)` | **对得上**（`rg/resource_info.rs` `calculate` + 5 方法） |
| `rg::resource_registry::ResourceRegistry::{populate_resources, into_exports, buffer, image, image_view, image_from_raw_handle, resource}` | 0x140be7ba0 等 | — | **对得上**（`rg/resource_registry.rs`） |
| `rg::compiled::CompiledRenderGraph<UD>::new/record_batch/record_postamble/add_pre_pass_barriers/compute_batches/gather_batch_preamble_barriers` + `BatchCompiler::{new,compile,flush_batch}` | 0x1402c4280 / 0x1402c1d40 / 0x140be9600 | `record_batch` 并行（bevy_tasks） | **对得上**（`rg/compiled.rs` Batch/`record_batch`，含并行 `rg/parallel.rs`） |
| `rg::executor::GraphExecutor::{new,begin_frame,wait_submits_done,wait_gpu_queue_trim,last_known_frame_duration_ns,begin/end_top_level_timestamp_query}` | 0x140beabd0/0x140bea680/0x140beaae0 | — | **对得上**（`rg/executor.rs`；`wait_gpu_queue_trim` 为 0x140bea680 ✓） |
| `rg::executor::GraphExecutionRecorder::{execute_compiled,execute,submit_pending_cbs,end_frame,ensure_active_cb}` | 0x1400fa190/0x1400fb5b0/0x140bec400 | 展示图 `execute` 内调 compile 0x140166510 | **对得上**（`rg/executor.rs` execute_graph/execute_compiled/submit_recorded + `rg/recorder.rs`） |
| `rg::executor::{ExecutedRenderGraph::new/retire, ExportedRenderGraphResources::take_exported/take_resource}` | 0x140bea180/0x140be9d00/0x140be9c80 | — | **对得上** |
| `rg::graph::RenderGraph::{create_raw_resource, create_resource_handle, on_retire}` + `ExportableResource::{raw,index}` | 0x140be94d0/0x140be9420 | `create_raw_resource(&mut self, name:&str, desc: impl Into<resource::Desc>)`（O） | **对得上**（`rg/graph.rs`） |
| `rg::pass_builder::PassBuilder::{add_resource_access<PassResources>, render<PassResources,closure>, rg_mut, write_impl}` | — | 6 个 render 实例（dispatch_impl/draw_stream/clear_rg_buffer/clear_rg_color_image/reset_indirect_args/rg_buffer_copy_from_slice/handle_screenshot_opportunity） | **对得上**（`rg/pass_builder.rs`；回调 `FnMut(PassContext)`；原回调签名 `impl FnOnce(u64)->bool`，rebuild 为 `FnMut(PassContext)->bool`） |
| `rg::pass_api::RenderPassApi`（21 方法：draw/draw_indexed/draw_indirect/draw_indirect_count/draw_indexed_indirect/draw_indexed_indirect_count/dispatch_groups_3d/dispatch_threads_3d/dispatch_indirect/push_constants/fill_buffer/clear_color_image/copy_buffer/bind_vertex_buffer_by_binding_point/bind_index_buffer/set_default_view_and_scissor/set_line_width/set_stencil_ref/begin/end/find_resource_access_type） | — | 0x44 B RenderPassBinding 载荷（INDEX §5.9 U） | **对得上**（`rg/pass_api.rs` RenderPassApi<'a,B> 全部 21 个） |
| `rg::temporal::{GetOrCreateTemporal<T>, GetOrCreateTemporalPair<T>} for TemporalRenderGraphRecorder<UD>` + `TemporalRenderGraphRecorder::export_temporal` | 0x1404f6e50/0x1404f7450/0x1404f7ca0 | 键为 **`ustr::Ustr`**（O：0x1404f6e50 内 Ustr::fmt）；`Result<_,E>` 调用点 unwrap | **半对上**：有 `TemporalRenderGraphState::get_or_create_temporal[_pair]`（键 `impl AsRef<str>`/&str，非 Ustr）；**无 `TemporalRenderGraphRecorder` 类型、无 trait 化 GetOrCreateTemporal*、无 `export_temporal` 在 recorder 上** |
| `rg::binding::{PassResourceSlot::new, rg_binding_to_descriptor_binding}` | — | — | **对得上**（`rg/binding.rs` + 枚举构造器） |
| `rg::resource::{Handle{raw,desc,clone_unchecked}, Ref{raw,bind_uniform_buffer,bind_view,into_sampled_rg_texture}, ResourceHandle, ResourceOrigin::ty, RuntimeResource::borrow}` | 0x140bd6c80 等 | `bind_view` 拷 0x3c B TextureViewDesc 进 RenderPassBinding（O） | **对得上**（`rg/resource.rs`；rebuild 的 Ref 绑 view 时写 0x3c B ✓） |
| `rg::recorder::{RenderGraphRecorder::{new,create,add_pass,record_pass,import_unowned,import_owned,export}, PassResourceAccessType, ResourceSyncGroup, PassResourceRef}` | — | 原录制器与图录制对象 `RenderGraphRecord<PassResources>`（O：drop_in_place 名） | **半对上**：有 `rg/recorder.rs::Recorder`（同名不同物）；**无 `RenderGraphRecorder`/`RenderGraphRecord<PassResources>` 命名** |
| `rg::resource_pool::ResourcePool::{clear,take_texture,take_buffer,insert_texture,insert_buffer}` | 0x140be76f0/0x140be6b90/0x140be71e0 | — | **对得上**（`rg/resource_registry.rs::ResourcePool` 或 `rg/resource_pool.rs`） |
| `rg::import_export::{impl$0/impl$1::export,import_unowned,import_owned,from_owned_rg_resource, into_sampled_rg_texture}` + `raw::texture::Texture as ImportExportToRenderGraph` | 0x140bd4fc0/0x140bd60a0/0x140bd6380/0x140bd63f0 | 0x140bd4fc0（Buffer）0x28B 返回 | **半对上**：有 `RenderGraph::import_unowned/import_owned`（参数面不同：原 impl$1 为 `(&mut RG, token, access)`，rebuild 为 `(name, Descriptor, token, access)`）；**无 `from_owned_rg_resource`、无 `ImportExportToRenderGraph` trait、无 `export`（impl$0/impl$1 两套）** |

### 3.2 raw 层（游戏调用面 → rhapsody-rebuild）

| 原 PDB 真名 | 地址（有则列） | rhapsody-rebuild 现状 |
|---|---|---|
| `raw::device::GraphicsDevice::{issue_full_pipeline_barrier, issue_pipeline_barrier, issue_and_reset_pipeline_barrier, wait_idle_and_recycle_resources, create_persistent_descriptor_set, update_persistent_descriptor_set, bind_descriptor_set(4 游戏闭包实例), bind_pipeline, bind_persistent_descriptor_set, create_command_buffer, create_framebuffer, create_render_pass, create_samplers, get_built_in_sampler, get_vk_buffer, get_vk_image_view, get_vk_descriptor_set, update_descriptor_set, update_texture, init_texture_data, create_gpu_profiler_output_buffer, refresh_memory_budget, max_bindless_descriptor_count, …}` | 0x140bf8e60/0x140bff4a0/0x140bf8f80 等 | **半对上**：issue_* 屏障 3 个 ✓、wait_idle_and_recycle_resources ✓、create_persistent_descriptor_set ✓、create_gpu_profiler_output_buffer ✓、get_built_in_sampler ✓；**缺 get_vk_buffer / get_vk_image_view / create_render_pass / create_framebuffer / create_command_buffer / bind_*（rebuild 只有 raw/binding.rs 与 rg/binding.rs 的 free fn、raw/shader.rs RasterPipeline::bind_pipeline） / update_descriptor_set / update_persistent_descriptor_set / refresh_memory_budget(rebuild 是 physical_device.rs 的 static fn)** |
| `raw::buffer::{BufferDesc::new/new_gpu_only/new_cpu_to_gpu/new_cpu_to_gpu_low_prio/new_gpu_to_cpu/alignment, Buffer::{device_address,mapped_slice_mut}, impl$6::{create_buffer, create_buffer_impl, update_buffer_bytes}(+closures)}` | 0x140bd4c00/0x140bd4cd0/0x140bd4e20/0x140bd4f20 | **半对上**：BufferDesc 形状 ✓（rebuild `new(size,usage,MemoryLocation)` vs 原 ABI `(edx=size, r8d=usage, r9b=memory 1..=4)`；rebuild MemoryLocation 枚举 `from_raw` ✓）；**`Buffer` 是 rebuild 的自有实现（`Buffer::new(device,desc)`、`mapped_slice_mut(&mut self,offset,size)->Result`），游戏面 `create_buffer(device,&desc,name,Option<&[u8]>) -> Buffer` / `update_buffer_bytes(device,&buf,data,ranges,offset)` / `mapped_slice_mut(&self)` 方法形状与调用形状不一致；无 `update_buffer_bytes`** |
| `raw::command_buffer::{CommandBuffer::{new,begin,end,raw}, CommandBufferPool, CbGraphicsState::record_default, CommandBufferSeqTracker::{current,track}}` | 0x140becfd0/0x140bed3d0 | **对得上**（`raw/command_buffer.rs`） |
| `raw::shader::{create_compute_pipeline<Arc<Compiled>>, create_raster_pipeline<Arc<Compiled>>, RasterPipeline, impl$7::default}` | 0x140bd4030 区 | **对得上**（`raw/shader.rs` create_compute_pipeline 0x140... 418/1082 行） |
| `raw::texture::{create_texture<TextureInitialData>/<&[u8]>, set_texture_debug_name, Texture, TextureDesc, TextureViewDesc}` | 0x140bd52a0 等 | **对得上**（`raw/texture.rs`；TextureDesc 32B 布局 ✓） |
| `raw::swapchain::{Swapchain::{format,extent,present,create_swapchain}, TrackedSwapchainTextureLease::{image,into_present_source}, validate_swapchain_and_acquire_image}` | 0x140c0ea90/0x140c0ef80/0x140c0f170/0x140bfd9d0 | **对得上**（`raw/swapchain.rs` + `raw/managed/swapchain.rs`） |
| `raw::submit::{SubmissionEngine::{queue,queue_mut,update_queue_pending_submissions,reclaim_command_buffer_resources_all_queues}, SubmitInfo, carve_off_front}` | — | **对得上**（`raw/submit.rs`） |
| `raw::profiler::{TimedFrame::{new,send_to_puffin}, ProfilerBuffer, create_gpu_profiler_output_buffer}` | 0x140bd3cb0/0x140bd4030 | **对得上**（`raw/profiler.rs`） |
| `raw::dynamic_constants::{DynamicConstants::allocate, DynamicConstantsChunk::into_uniform_and_storage_buffer_binding}` | 0x140c02740/0x140c02370 | **对得上**（`raw/dynamic_constants.rs`） |
| `raw::barrier::{SynchronizationScope1::{image_barrier,buffer_barrier}}` | 0x140bd66a0/0x140bd6810 | **对得上**（`raw/barrier.rs`） |
| `raw::managed::handle::{ManagedHandle{into_gpu_owned,into_raw_gpu_owned}, ManagedRef{…}, GpuOwned{…}, RawManagedHandle, DisposedRawResourceHandle, ManagedResourceChecksum}` | 0x140be24b0 等 | **对得上**（`raw/managed/handle.rs`；注意原 `ManagedHandle` 是**泛型** `ManagedHandle<T:Send+Sync>`，country-core 合约是非泛型 u64 包装 —— 见 §4） |
| `raw::debug::{begin_label,end_label}`（device impl） | 0x140bf09b0/0x140bf0b40 | **对得上**（`raw/debug.rs`） |
| `raw::bytes::slice_as_bytes` | — | **对得上** |
| `raw::disposal`/`raw::graveyard`（候选差异：原按序列号销毁桶，rebuild `raw/graveyard.rs` 按 epoch） | — | 行为候选差异（MODULE_STATUS 已登记） |
| `gpu_profiler::backend::ash::{ProfilerSubmittedFrame::retrieve_results, ActiveProfiler::create_marker}` | 0x140294cf0/0x140c110f0 | 外部本地 crate（`rhapsody/crates/gpu-profiler`），rebuild 用真实 gpu-profiler 依赖 ✓ |
| `ustr::Ustr`（temporal 键） | 0x140c0f770 | **对不上**：country-core 有 `ustr_contract::intern` stub；rebuild temporal 用 `impl AsRef<str>` 键 |

### 3.3 对得上 / 对不上统计（游戏侧实际调用的 rhapsody 入口）

以 §2 机器码调用点为分母（去重后 **30 个 rhapsody 入口** + 3 个外部库入口）：

- **对得上（rebuild 已有同名/同职责实现，19）**：RenderGraphState::compile_frame（半）、ResourceInfo::calculate、
  ResourceRegistry（populate/into_exports/buffer/image/image_view）、CompiledRenderGraph::new/record_batch、BatchCompiler、
  GraphExecutor 5 方法、GraphExecutionRecorder（execute/execute_compiled/submit_pending_cbs/end_frame/ensure_active_cb）、
  ExecutedRenderGraph/ExportedRenderGraphResources、RenderGraph::create_raw_resource/on_retire、PassBuilder（add_resource_access/render）、
  RenderPassApi 21 方法、ResourcePool、RenderGraph::import_unowned/import_owned（半）、PassResourceSlot、Handle/Ref/ResourceHandle、
  ResourcePool::clear、Swapchain 4 方法、SubmitInfo/SubmissionEngine、TimedFrame、DynamicConstants、barrier Scope、managed 全族、
  debug begin/end_label、command_buffer 全族、raw::shader 管线、raw::texture。
- **对不上 / 缺失（11）**：`RenderGraphState::compile` 三实例与 `ResourceExportHandle<Texture>` 返回、`TemporalRenderGraphRecorder`
  + `GetOrCreateTemporal(Pair)` trait（键应为 Ustr）、`ImportExportToRenderGraph` trait 与 `from_owned_rg_resource`/`export`、
  `RenderGraphRecorder`/`RenderGraphRecord<PassResources>` 命名、`GraphicsDevice::{get_vk_buffer, get_vk_image_view, create_render_pass,
  create_framebuffer, create_command_buffer, bind_descriptor_set, bind_pipeline, bind_persistent_descriptor_set, update_descriptor_set,
  update_persistent_descriptor_set}`、`raw::buffer::impl$6::{create_buffer, update_buffer_bytes}`（rebuild 形状不同）、
  `Buffer::mapped_slice_mut(&self)`/`device_address(&self)` 方法形状、`ManagedHandle<T>` 泛型身份、`ustr::Ustr` 键、
  OOM 降分辨率编译语义、原回调 `FnOnce(u64)->bool` 签名（rebuild 为 `FnMut(PassContext)->bool`）。

## 4. 差距清单（rhapsody_contract 占位 vs 原 ABI）

| # | 游戏侧调用点 | rhapsody 目标（真名+地址） | rhapsody-rebuild 现状 | 需补的适配 |
|---|---|---|---|---|
| 1 | `render` R4 模式字节分支 → `compile_loading_graph`/`compile_world_graph`（render_frame.rs） | `RenderGraphState::compile<ResourceExportHandle<Texture>,PassResources,closure_env>`（0x1401647b0/0x140161cd0；展示图 0x140166510 在 `execute` 内部） | `compile_frame` 半对上；无游戏闭包实例、无 `ResourceExportHandle`、无 32x32 兜底/降分辨率路径（`min_extent`、`[32,32]`、写 2 到 TypeId 0x3335386a… 资源、0x1404e9376 常量） | 补 `compile` 游戏面：`(state, swapchain, closure) -> Result<ResourceExportHandle<Texture>, CompileError>`；闭包 = `&mut RenderGraph` + `&World`；补 OOM 降级语义 |
| 2 | `PassResources::new_from` → `execute_compiled`（render R7/R8） | `GraphExecutionRecorder::execute_compiled<PassResources>`（0x1400fa190） | ✓ | `PassResources`（country_core 类型）需满足 rebuild 的 `ChunkRecorder`/`ResourceBackend` 约束；rebuild 回调已改 `FnMut(PassContext)->bool`，原 PDB 是 `FnOnce(u64)->bool`（INDEX §5.9：0x44 B RenderPassBinding 未恢复） |
| 3 | `ComputePass` 全部 `read/write_*`（compute_pass_builder.rs） | `PassBuilder::add_resource_access<PassResources>(resource_index, access, group)`（rg::pass_builder） | ✓ | `RecordingPassBuilder` 是记录侧、`PassBuilder` 是图侧；需把记录结果灌进图声明 |
| 4 | `RecordedComputePass::execute`（ComputeExec 实现） | `RenderPassApi::{dispatch_groups_3d,dispatch_threads_3d,dispatch_indirect,push_constants}` + `GraphicsDevice::{issue_full_pipeline_barrier, bind_descriptor_set}`（`[device+0xaba]` 全屏障开关、`[device+0xaa0]` dispatch 计数） | ✓ 大部分；`bind_descriptor_set` 缺 | 在 rebuild 补 `GraphicsDevice::bind_descriptor_set`（原 4 个游戏闭包实例的签名：Chain<Map<IntoIter<(BindingIndex,PassResourceSlot)>,…>,Map<Iter<(BindingIndex,GpuOwned<DescriptorSetBinding>)>,…>> 与 Enumerate<Iter<DescriptorSetBinding>> 两种） |
| 5 | `DrawBuilder::draw*`（draw_builder.rs 0x140906f40 起 9 种 draw 种类） | `PassBuilder::render<PassResources, draw_stream::impl$11::render::closure_env$0>` → `RenderPassApi::{draw,draw_indexed,draw_indirect[_count],draw_indexed_indirect[_count]}` | ✓ | draw 种类载荷（0x44 B RenderPassBinding、draw 模式枚举 2/3/4/7）需与 rebuild pass_api Command 枚举对齐 |
| 6 | `PassHost::{vk_buffer, vk_image_view, create_render_pass, render_pass_cache}`（draw_stream.rs） | `GraphicsDevice::{get_vk_buffer(0x140bef470), get_vk_image_view, create_render_pass, +0x188 HashMap<RenderPassDesc,RenderPass>}` | **缺** | rebuild 补这 4 个方法 + 缓存字段；`get_vk_image_view` 调用点 `Result::unwrap`（原 Err 载荷） |
| 7 | `SsboType::new_storage_buffer` 21+38 个实例（ssbo.rs） | `raw::buffer::impl$6::{create_buffer, update_buffer_bytes}`（0x140bd4e20 区）；`Buffer::mapped_slice_mut(0x140bd4cd0)`/`device_address(0x140bd4c00)` | 半对上 | 原 `create_buffer(device,&desc,name,Option<&[u8]>)` 的 Ok 载荷 40 B 整块进 `TypedStorageBuffer`；rebuild `Buffer` 是 16 B desc + 句柄，**需提供游戏面 `Buffer`(40B) 转换或改 game 侧布局**；`mapped_slice_mut(&self)`/`device_address(&self)` 从 device trait 移到 Buffer 方法（原方法在 Buffer 上） |
| 8 | `MappedStorageBuffer::new_storage_buffer`（mapped_slice_mut unwrap → panic） | `Buffer::mapped_slice_mut` 持久映射 | 半对上 | rebuild 签名 `(&mut self, offset, size)->Result<&mut [u8]>`；游戏面为 `(&self)->Option<&mut [u8]>` 全长切片；内存种类 2/4（`MAPPED_MEMORY_UPLOAD/READBACK`）语义 U，rebuild `MemoryLocation` 需对上 1..=4 编码 |
| 9 | `nani::trimesh::Trimesh::build`（块地址加法） | `Buffer::device_address`（vkGetBufferDeviceAddress） | 半对上 | 同上方法形状 |
| 10 | `render_world` temporal 两处（0x1404ec110: 0x1816/中部） | `GetOrCreateTemporal(Pair)<TextureDesc> for TemporalRenderGraphRecorder<UD>`（0x1404f6e50/0x1404f7450）；键 = **Ustr** | 半对上 | rebuild 需补 `TemporalRenderGraphRecorder`（图录制时态）并把键换 Ustr（country-core `ustr_contract::intern` 现在返回 0）；`TemporalTexture{tex,flag}`/`TemporalPair{cur,prev,fresh}` 布局（40 B/73 B）需对上 |
| 11 | `RgResources::{create_raw_texture, get_or_create_temporal_*}`（rg_resources.rs） | `RenderGraph::create_raw_resource(0x140be94d0)`（`name:&str, desc: impl Into<resource::Desc>`）、temporal（0x1404f6e50/0x1404f7450） | ✓ 前一半；temporal 见 #10 | `RgResources` 应由真实 RenderGraph 实现（现在只有 `RecordingRg`） |
| 12 | `AssetTextureLibrary::{import_texture_presets_to_rg, export_preset_textures_from_rg, bindless_textures_dset}` / `AssetSsboLibrary::export_from_rg` / `GpuWorld::{import_to_rg,export_from_rg}` / `RenderContext::finish`（compile 闭包内） | `Texture as ImportExportToRenderGraph::{import_unowned(0x140bd60a0), export(0x140bd6380), from_owned_rg_resource(0x140bd63f0)}`、`AssetSsboLibrary` 侧 `Buffer import_unowned(0x140bd4fc0)`、`TemporalRenderGraphRecorder::export_temporal(0x1404f7ca0)` | 半对上 | **补 `ImportExportToRenderGraph` trait**（impl$0=Texture? impl$1=RenderGraph? 两个 impl 各含 export/import_unowned/import_owned/from_owned_rg_resource）；`ImportedBuffer` 20 B（+0xe 处 5 = "未导入"判别） |
| 13 | `Handle<T>`/`ManagedHandle`/`ReadOnlyHandle<T>`（lib.rs 合约，draw_stream/draw_builder 载荷） | 原 = `rg::resource::Handle{ResourceHandle,Descriptor}` 与 `managed::handle::ManagedHandle<T:Send+Sync>`、`ManagedRef`、`GpuOwned`；**无 ReadOnlyHandle** | 半对上 | 合约 `Handle<T>`（PhantomData 包装）+ `ReadOnlyHandle<T>` 是**自造类型**：原图内句柄是 `rg::resource::Handle`（非泛型），原 managed 句柄是泛型 `ManagedHandle<T>`；需改合约：图侧句柄换 `rg::resource::Handle`，设备侧换 `ManagedHandle<T>`；`get()` 两个 todo! |
| 14 | `RenderPassDesc{selector:u32}` / `RenderPass` / `RenderPassAttachmentDesc`（lib.rs 合约） | `raw::shader::RenderPassAttachmentDesc`（0x10 B）、`raw::render_pass::RenderPassDesc`（缓存键：+0 托管句柄 +0x18 u32，二者相等才算同键） | 半对上 | 合约把 RenderPassDesc 当 dedupe 键但只有 selector：原键是 `(ManagedHandle, u32)` 对（`RasterPipelineKey` 同款）；`RenderPassAttachmentDesc` 0x10 B 缺字段 |
| 15 | `PassResourceSlot` / `RenderPassBinding`（lib.rs 合约 + pass_resource_access.rs） | `rg::binding::PassResourceSlot`（rebuild：`uniform_buffer/storage_buffer/sampled_image/storage_image` 带 offset/size/layout）、`RenderPassBinding`（0x44 B，含 TextureViewDesc 0x3c B + 资源号 + 写标志） | 半对上 | 合约的 4 变体只有 `resource: u32`，rebuild 带 offset/size/layout；0x44 B RenderPassBinding 布局未恢复（INDEX §5.9），合约 `{resource_index,access_type}` 是占位 |
| 16 | `GraphicsDevice::dispatch`（lib.rs 合约 112 行） | 原**没有** `GraphicsDevice::dispatch`；dispatch 经 `RenderPassApi::dispatch_groups_3d`（rg）或 `[device+0x360]` 的 vkCmdDispatch | 对不上 | 删合约方法或改指 RenderPassApi；`full_barrier_required` 原为 `[device+0xaba]` 字节读（`ComputeExec::full_barrier_enabled` 已正确建模） |
| 17 | `StorageBufferDescriptor`（lib.rs 294–328） | 原 PDB 无此名（vtable +0x18/0x20/0x30 的 SSBO 描述在 ssbo_library 侧，属 country-core） | — | 归属存疑：更可能是 country-core 自身类型，不是 rhapsody 合约 |
| 18 | `GraphicsDeviceResource(Arc<GraphicsDevice>)`（render/mod.rs，Deref +0x10 = ArcInner） | `raw::device::GraphicsDevice`（真实结构体，非 u64） | 半对上 | 合约 `GraphicsDevice(pub u64)` 是占位：原资源是 `Arc<GraphicsDevice>`（+0x10 数据偏移 ✓ 重建）；rebuild 的 `GraphicsDevice` 是 `Arc<Self>` 创建（`from_device(device)->Arc<Self>`）——接时用 rebuild 类型替换合约 |
| 19 | `frozen_map`/`hotwatch`/`puffin`/`shader-pipeline`/`shader-variants` 合约 | 原 rhapsody workspace 内 crate（`crates/{shader-pipeline,gpu-profiler,vk-sync-rs,aftermath-rs,dxgi-enumerate-gpus}`）；puffin 0.14.1 外部 | 半对上 | `ShaderCompilerArtifacts` 现返回空壳；rebuild `shader_pipeline/` 已有 compiled/reflection 实现可接 |
| 20 | 帧级 trait 无生产实现（`FramePasses`/`RenderFrameSurface`/`PassHost`/`RgResources` 只有测试 Mock） | 原接线在 `tiny_glade::render_loop`（compile 闭包内联进 rhapsody 泛型，见 §2.2） | — | 需写集成层：把 54 个 pass 入口接到 compile 闭包；这是 RENDER_GAP_ANALYSIS #2/#6/#8 的核心 |

## 5. panic / 日志串（rne-read 原字节直读）

原仓库路径证据：`/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/…`（多串内嵌）。

| VA | 内容 | 归属/语义 |
|---|---|---|
| 0x14294c478 | `crates/rhapsody/src/rg/executor.rs` + **`Execute render graph`**（line 192 col 0x4a）+ **`Submit swapchain dep`**（line 302 col 0x53） | `GraphExecutionRecorder::execute` 的 puffin scope 名与 swapchain 提交标签 |
| 0x14294c3a0 | `crates/rhapsody/src/rg/compiled.rs` + **`Record pass`**（line 82 col 0x26；同文件另 5 处：207/215/225/231/189 行） | `CompiledRenderGraph::record_batch` 逐 pass 标签 |
| 0x14294c430 | **`not yet implemented`**（rg/compiled.rs 区，line 82 后的 todo! 路径） | 原 rhapsody 自身也有未实现分支 |
| 0x14294c228 | `ResourceAccess`+错误名串 + `rg executor cb`（compiled.rs） | rg 错误类型 Debug 名 + 命令缓冲标签 |
| 0x142d06098 | `raw/managed/resources.rs` + **`not yet implemented`** | managed 层未实现分支 |
| 0x142d069b0 | **`capacity overflow`**、**`bucket removed prematurely or command buffer begin() never called?`**（raw/disposal.rs） | 销毁桶/命令缓冲断言 |
| 0x142d06f88 | **`assertion failed: seq > self.last_retired`**（raw/disposal.rs） | 退役序号断言 |
| 0x142d08400 | **`Swapchain creation failed: `**（raw/managed/swapchain.rs） | swapchain 创建错误日志 |
| 0x142fd9448 | **`Must have at least one framebuffer attachment`**（raw/framebuffer.rs） | framebuffer 断言 |
| 0x142c88860 | **`assertion failed: initial_data.byte_count() <= desc.size`**（raw/descriptor_set.rs 区） | 纹理初始数据大小断言 |
| 0x142c88978 | `setup cb`（raw/shader.rs） | setup 命令缓冲标签 |
| 0x1429ebc17…0x1429ebe17 | `render graph` / **`render graph resources; reducing resolution scale -> `** / **`render graph for the reduced resolution scale`** / **`render graph compile failed`** / **`render graph compiled`** / **`render graph: `** | tiny_glade `render`（0x1404e8010 区）OOM 降分辨率与编译日志（对应 render_frame.rs 的 0x1404e8a3d..0x1404e9516 重试路径） |
| 0x142b1022d | **`render graph is not currently supported`** | 不支持的图操作 panic（rg） |
| 0x142b13f41 / 0x142b13fd9 | `render graph texture: ` / `render graph buffer: ` | 资源调试名前缀（rg 内 set_*_debug_name 调用） |
| （bevy 侧）0x1429259b0 等 | `Resource does not exist` + `… requested by  does not exist: ` | bevy `get_resource`/`get_resource_id` panic（游戏侧；compute_pass_builder 289 行注释已引用） |

任务提示的 `"Can't find … in render graph"` 类串在 B 二进制中以**上述形态**出现（`render graph texture:/buffer:` 调试名前缀、
`not yet implemented`、registry 的 `panic_cold_explicit`——`ResourceRegistry::{buffer,image,image_view,image_from_raw_handle}::panic_cold_explicit`
在 graph-compiler 契约里，文本未在 .rdata 独立成串，推测经 `format_args` 现场拼接）。

## 6. 交付文件

- 本文 `evidence/country-core-deep/rhapsody/CALL-CHAIN.md`
- `decomp/`：13 个关键函数反编译（RE-MCP dbid `tg_rl2`，2026-10-15）+ 2 个字符串 xref 结果（空）
- `resolved-calls.json`：5 个旧 .c + 8 个新 .c 的全部调用目标→PDB 名解析
- `batch-job.json` / `batch-report.json`：可复现 job 与原始报告
- `resolve_calls.py`：解析脚本
- `rhapsody-panic-strings.json`：原 rhapsody panic/日志串（rne-read 直读）

不改任何 reconstruction 源码。
