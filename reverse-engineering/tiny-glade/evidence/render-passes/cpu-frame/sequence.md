# CPU 侧帧 Pass 有序序列（render_world / render）

研究日期：2026-10-13。载体：patched B EXE 反编译（`tiny-glade.exe`）。
来源：`evidence/render-architecture/0x1404ec110.c`（render_world）、`0x1404e8010.c`（render）、`0x140161cd0.c`（compile 实例）、`0x140987dd0.c`（gpu_generate_draw_lists）、`0x1400fa190.c`（execute_compiled），地址→PDB 名映射来自 `evidence/render-architecture/resolved-calls.json`。
标注：O=Observation（伪代码直接可见/PDB 名），I=Inference，U=Unknown。

> 注：本文件末尾 F 节为任务要求的三件交付中的"summary"内容（summary.md 被工具写入护栏拒绝，内容并入本文件，同时在本子代理最终报告中给出）。



两个层面的顺序分开记录：(1) `render` 帧外层（begin/execute/acquire/present）；(2) `render_world` 世界图闭包内部（编译时按顺序注册的 pass）。

---

## A. 帧外层 `tiny_glade::render_loop::render`（0x1404e8010）

按调用顺序（O）：

| # | 步骤 | 地址 | PDB/动作 | 条件 |
|---|---|---|---|---|
| R1 | ECS 资源获取 | 0x1400dd6f0 | `bevy_ecs::component::Components::get_resource_id`（哈希 0x2ed189581b92decf、0x3d7b56e787d47c40），Arc 计数；settings 结构读入模式字节 +0xac | 存在性检查 |
| R2 | DrawLists::new / build_draw_lists / sort | 0x1409ba340 / 0x1409ba540 / 0x1409ba4b0 | `country_core::systems::render_loop::draw_passes::draw_lists`（CPU 侧绘制列表构造+排序） | 无条件 |
| R3 | Swapchain::format / extent | 0x140c0e7e0 / 0x140c0ea90 | `rhapsody::raw::swapchain::Swapchain` | 仅当 swapchain 存在 |
| R4 | RenderGraphState::compile | 0x1401647b0 | `rg::state::RenderGraphState::compile`（另一个单态实例） | 模式字节 == 0 分支 |
| R4' | RenderGraphState::compile | 0x140161cd0 | 含 render_world 闭包的主图 compile：render_world → 后处理/UI → ResourceInfo::calculate → registry → CompiledRenderGraph::new | 模式字节 != 0 分支（O：render_world 调用在 0x140161cd0.c:835，ResourceInfo::calculate 在 :1736） |
| R5 | GraphExecutor::begin_frame | 0x140beabd0 | `rg::executor::GraphExecutor::begin_frame` | 无条件 |
| R6 | profiler retrieve | 0x140294cf0 | `gpu_profiler::backend::ash::ProfilerSubmittedFrame::retrieve_results`（循环）+ TimedFrame::new/send_to_puffin（0x140bd3cb0/0x140bd4030） | 无条件 |
| R7 | PassResources::new_from | 0x140909310 | `country_core::render::pass_builder::pass_resources::PassResources::new_from` | 无条件 |
| R8 | GraphExecutionRecorder::execute_compiled | 0x1400fa190 | BatchCompiler::new → flush_batch 循环 → submit_pending_cbs → 逐 pass ensure_active_cb + CompiledRenderGraph::record_batch → ResourceRegistry::into_exports → ExecutedRenderGraph::new（O，0x1400fa190.c） | 无条件 |
| R9 | retire / take_resource / from_owned_rg_resource | 0x140be9d00 / 0x140be9c80 / 0x140bd63f0 | `ExecutedRenderGraph::retire`、`ExportedRenderGraphResources::take_resource`、`ImportExportToRenderGraph::from_owned_rg_resource`（把导出资源重新导入） | 无条件 |
| R10 | validate_swapchain_and_acquire_image | 0x140bfd9d0 | `raw::swapchain::GraphicsDevice::validate_swapchain_and_acquire_image` | 编译成功（flag 0x113c==1）且 swapchain 存在；返回字节 ==7 时本帧跳过 present |
| R11 | TrackedSwapchainTextureLease::image | 0x140c0f120 | 取已 acquire 的交换链图像 | 同上 |
| R12 | PassResources::new_from | 0x140909310 | 为最终图构造资源 | 同上 |
| R13 | GraphExecutionRecorder::execute | 0x1400fb5b0 | 用 acquire 到的图像执行最终图（UI/后处理） | 同上 |
| R14 | into_present_source | 0x140c0f170 | `TrackedSwapchainTextureLease::into_present_source` | 同上 |
| R15 | GraphExecutionRecorder::end_frame | 0x140bec400 | `rg::executor::GraphExecutionRecorder::end_frame` | 无条件 |
| R16 | Swapchain::present | 0x140c0ef80 | `raw::swapchain::Swapchain::present` | 仅当本帧已 acquire |
| R17 | finish_screenshot_capture | 0x14031ea20 | `tiny_glade::screenshot::finish_screenshot_capture` | 无条件 |
| R18 | RenderStats::snapshot_and_reset | 0x140c02da0 | `raw::stats::RenderStats::snapshot_and_reset` | 无条件 |
| R19 | refresh_memory_budget | 0x140bf8f80 | `raw::device::GraphicsDevice::refresh_memory_budget` | 无条件 |
| R20 | wait_gpu_queue_trim | 0x140bea680 | 依据 last_known_frame_duration_ns（0x140beaae0） | 帧尾 |

要点：acquire 出现在主图 execute 之后，final 图 execute 之前（O）。与 findings.md 第 7 节一致。

---

## B. 世界图闭包 `tiny_glade::render_loop::render_world`（0x1404ec110）——编译期注册顺序

`render_world` 由 compile 实例 0x140161cd0 调用（0x140161cd0.c:835）。以下为伪代码调用顺序（O，地址→PDB 名全部来自 resolved-calls.json）。整段主体受判别值 `cStack_154 != '\x02'` 保护，==2 时走 unwrap_failed panic（I：判别为错误标签）。

### B1 上传 / 程序化生成（compute，少量 draw）

| # | Pass | 地址 | 类型 | 备注 |
|---|---|---|---|---|
| W1 | upload_world_raster ×6 | 0x1403d8bd0, 0x1403da050, 0x1403da570, 0x1403d90f0, 0x1403d9610, 0x1403d9b30 | compute | 同 PDB 名 `upload_raster_staging_to_texture` 的 6 个单态实例 |
| W2 | water_rasterize_from_cpu | 0x140a12680 | compute | |
| W3 | rasterize_terrain_stroke | 0x1408c7120 | compute | |
| W4 | displace_rocky_terrain | 0x140a3ee80 | compute | |
| W5 | read_terrain_data | 0x14098c680 | compute | |
| W6 | mouse_read_terrain_stroke_index | 0x1409768b0 | compute | |
| W7 | rasterize_foliage_exclusion_mask | 0x1409cb7b0 | draw | |
| W8 | highlight_active_terrain_stroke | 0x140a58450 | compute | |
| W9 | object_grass_push_pass | 0x1409daaa0 | draw | |
| W10 | compute_path_masks | 0x1409014e0 | compute | |
| W11 | rasterize_buildings | 0x140a32770 | compute | |
| W12 | garden_rasterize_from_cpu | 0x140a22140 | compute | |
| W13 | water_spawn_vegetation | 0x140a13970 | compute | |
| W14 | spawn_vegetation_around_buildings | 0x1408e4cc0 | compute | |
| W15 | read_bush_positions_for_cpu | 0x140866540 | compute | |
| W16 | garden_spawn_vegetation | 0x140a22b80 | compute | |
| W17 | path_pebbles_compute | 0x140958f70 | compute | |
| W18 | leaf_particles_compute | 0x1408f4d00 | compute | |
| W19 | roof_tiles_gravity_compute | 0x140a29b10 | compute | |
| W20 | stairs | 0x140a68a50 | compute | |
| W21 | generate_wall_draw_lists | 0x14098aaf0 | compute | 第一轮墙绘制列表 |
| W22 | gpu_generate_draw_lists（第 1 轮） | 0x140987dd0，第三参 0 | compute | 建 GPU-only 缓冲（0x6c000、实例数×4）、清绘制列表、cull_and_bucket（O，0x140987dd0.c） |

### B2 第一轮可见性与绘制

| # | Pass | 地址 | 类型 | 备注 |
|---|---|---|---|---|
| W23 | shadow_pass | 0x1409cc270 | draw | callees：render/bind_common/get_rg_texture/extent_2d（O）→ 绘制阴影图 |
| W24 | calc_terrain_lighting_texture | 0x140a51440 | compute | |
| — | create_raw_resource ×2 | 0x140be94d0 | build | main 深度纹理 + HZB 纹理资源声明（O） |
| W25 | main_pass | 0x140833bc0 | draw | 主深度/反照率/速度绘制 |
| W26 | generate_hzb | 0x1409286f0 | compute | callees：bind_individual_mips + write_rg_storage_buffer + get_or_create_temporal + all_mip_levels（O）→ HZB mip 链 |

### B3 HZB 之后的补充阶段

| # | Pass | 地址 | 类型 | 条件 |
|---|---|---|---|---|
| W27 | generate_wall_draw_lists_pass2 | 0x14098b2a0 | compute | **全局 \*0x14362c0d8 == 0**（O） |
| W28 | gpu_generate_draw_lists（第 2 轮） | 0x140987dd0，第三参非 0 | compute | 同上（O）；第三参选择第 2 轮的 clear buffer 路径（0x140987dd0.c） |
| W29 | grass_compute | 0x140a60280 | compute | dispatch_indirect_raw；写 DrawIndexedIndirectBuffer/DrawIndirectBuffer/AtomicCounterBuffer；inclusive_prefix_scan_u32_1m（O） |
| W30 | main_pass_oc2 | 0x140835a40 | draw | HZB 后的第二次主绘制（遮挡二次判定） |
| W31 | run_visibility_queries | 0x140a2f2e0 | compute | |
| W32 | calculate_dof_autofocus | 0x140a17880 | compute | |
| W33 | mouse_depth_compute | 0x140a01620 | compute | |
| W34 | late_forward_and_deferred_pass | 0x1408373b0 | draw | 光照/着色（bind_common+render） |
| W35 | extract_half_res_depth_normals | 0x1408f6180 | compute | |
| W36 | calculate_reprojection_map | 0x140a0d190 | compute | |
| W37 | calculate_ssgi | 0x1408743b0 | compute | |

### B4 GI 求解（gi 历史 temporal pair 创建成功后，无条件执行）

| # | Pass | 地址 | 类型 | 备注 |
|---|---|---|---|---|
| — | GI history temporal pair | 0x1404f7450 | build | get_or_create_temporal_pair；extent 来源按 param_6[0xab] 字节选择（O） |
| W38 | ray_trace_gi | 0x140a01cf0 | compute | callees：get_or_create_temporal_pair + bind_ray_tracing(ComputePass) + div_up_extent（O）；shader 反射显示为软件 BVH 追踪（无 RT capability，见 findings.md 第 6 节） |
| W39 | eval_gi_screen_radiance | 0x140a44cd0 | compute | |
| W40 | bin_lights | 0x1409e7770 | compute | |
| W41 | restir_sample_direct_lights | 0x140a6fec0 | compute | |
| W42 | apply_screen_shadows | 0x140a39e70 | compute | |
| W43 | deferred_raster | 0x1409d20b0 | draw | draw_no_vao + bind_color_library + bind_ray_tracing(DrawBuilder)；读 LanternSsbo / SpatialResolveOffsetsSsbo（O） |

### B5 透明与图像处理

| # | Pass | 地址 | 类型 | 条件 |
|---|---|---|---|---|
| W44 | refraction_copy_pass | 0x1409bf320 | draw | **全局 \*0x14362c098 == 0**（O） |
| W45 | water_surface_pass 或 ice_surface_pass | 0x1409bf800 / 0x1409c08d0 | draw+compute | **按 is_winter 标志（0x140ae8320，GladeType::is_winter）间接派发**：非冬季 water_surface_pass，冬季 ice_surface_pass（O）；两者都用 dispatch_indirect_raw + bind_ray_tracing |
| W46 | pre_taa_transparency_pass | 0x1409c3a80 | draw | |
| W47 | compute_shadow_mask_mean | 0x140a3a6f0 | compute | |
| W48 | apply_volumetrics | 0x1408a44c0 | compute | |
| W49 | dof_prepare | 0x140a5a020 | compute | 读 DofAutofocusSsbo |
| W50 | generate_mips | 0x1400fb890 | compute | **配置字节 param_6[0xa9 + (res_type==0x13?1:0)] != 0**（O） |
| W51 | calc_dof | 0x140a5a570 | compute | 同上；half_res（O） |
| W52 | calculate_taa 或 calculate_tgsr | 0x140a3b2f0 / 0x140a3ba60 | compute | **选择关系：param_6[0xab]==0 → TAA，!=0 → TGSR**（O）；TGSR 用 min_subgroup_size + get_or_create_temporal_pair（O） |
| W53 | post_taa_transparency_pass | 0x1409c40b0 | draw | |
| W54 | render_shader_refactoring | 0x14098f9f0 | U | callees 仅 RwLock（read_contended/wake_writer_or_readers）+ unwrap（O）；名称来自 PDB，实际作用 U |

（结束）render_world 把图结果写回调用方（0x1404ec110.c:487-502），compile 实例随后做 ResourceInfo::calculate 等。

---

## C. 条件汇总（全部为伪代码可确认的 O，语义 U）

| 分支 | 判定 | 效果 |
|---|---|---|
| 整帧主体 | `cStack_154 != '\x02'` | ==2 → unwrap_failed panic（I：错误标签） |
| 冬季水/冰 | `is_winter`（0x140ae8320） | water_surface_pass ↔ ice_surface_pass |
| 第二轮回合 | 全局 \*0x14362c0d8 == 0 | generate_wall_draw_lists_pass2 + gpu_generate_draw_lists 第 2 轮 |
| 折射/水面 | 全局 \*0x14362c098 == 0 | refraction_copy_pass + 水/冰 surface |
| DoF 主体 | param_6[0xa9(+type)] != 0 | generate_mips + calc_dof |
| TAA vs TGSR | param_6[0xab] == 0 / != 0 | calculate_taa ↔ calculate_tgsr（互斥选择） |
| compile 实例选择 | settings+0xac 模式字节 | 0x1401647b0（==0）↔ 0x140161cd0（!=0，含 render_world） |
| present 跳过 | acquire 结果字节 == 7 | 本帧不 present |

---

## D. 与 compiled-shaders 交叉核对

| CPU pass | compiled-shaders 文件名（存在性） |
|---|---|
| shadow_pass | `_mei_shadow.raster.*`（无字面 `_shadow_pass` 文件；I：mei_shadow 即阴影图光栅） |
| generate_hzb | `_generate_hzb.cs.hlsl_cs_main_0.bin` |
| gpu_generate_draw_lists | `_nani_generate_draw_lists.cs`、`_nani_clear_draw_lists.cs`、`_nani_cull_and_bucket*.cs`（8a7d2f81/d8520a92 两个变体）、`_nani_histogram_prefix_sum.cs`、`_nani_subtotal_prefix_sum.cs`、`_nani_allocate_draw_commands.cs`、`_nani_draw_indexed_indirect_to_dispatch.cs` |
| grass_compute | `_generate_grass.cs`、`_grass_calc_tiles.cs`、`_grass_tile_bundle_gen.cs`、`_grass_bundles_prefix.cs`、`_grass_calc_dispatch_args.cs` |
| extract_half_res_depth_normals | `_extract_half_res_depth_normals.cs` |
| calculate_reprojection_map | `_calculate_reprojection_map.cs` |
| calculate_ssgi | `_ssgi_ssgi.cs`、`_ssgi_temporal_filter.cs`、`_ssgi_upsample.cs` |
| ray_trace_gi | `_gi_trace_gi_candidates.cs`、`_gi_update_gi_radiance.cs`、`_gi_reproject_screen_radiance.cs`、`_gi_blur_screen_radiance.cs` |
| eval_gi_screen_radiance | `_gi_eval_screen_radiance.cs` |
| bin_lights | `_light_binning_bin_lights_to_cells.cs`（2 变体）、`_light_binning_cell_prefix_sum.cs` |
| apply_screen_shadows | `_apply_screen_shadows.cs` |
| deferred_raster | `_deferred_solid/grass/plant/canopy_light_sun_sky.raster.*` |
| refraction_copy_pass | `_water_plane_refraction_copy.raster.*`、`_waterfall_refraction_copy.raster.*` |
| water_surface_pass | `_water_plane.raster.*`、`_water_deferred_rt.cs`、`_water_calc_deferred_rt_args.cs`、`_water_draw_water_mask.cs` |
| ice_surface_pass | `_water_ice_refl_march.cs`、`_water_ice_refl_resolve.cs`、`_water_ice_refr_resolve.cs`、`_water_ice_deferred_rt.cs`、`_water_ice_cracks.raster.*` |
| compute_shadow_mask_mean | `_shadow_mask_mean.cs` |
| apply_volumetrics | `_apply_volumetrics.cs` |
| dof_prepare | `_dof_prepare.cs`、`_dof_clear_halfres.cs`、`_dof_init_buffers.cs` |
| generate_mips | `_generate_mips.cs` |
| calc_dof | `_dof_fused.cs`、`_dof_halfres.cs`、`_dof_fullres.cs`（多变体）、`_dof_fullres_upsample.cs` |
| calculate_taa | `_taa.cs` |
| calculate_tgsr | `_tgsr.cs`（4 变体）、`_tgsr_blur_lum.cs`、`_tgsr_reproject.cs` |
| mouse_depth_compute | `_read_mouse_depth_value.cs` |
| calc_terrain_lighting_texture | `_terrain_lighting_texture.cs` |
| stairs | `_rocky_terrain_stairs_stairs.cs`、`_rocky_terrain_stairs_stairs_indirect.raster.*` |
| roof_tiles_gravity | `_roof_tiles_gravity.cs` |
| path_pebbles | `_path_pebbles.cs` |
| leaf_particles | `_leaf_particles.cs` |
| rasterize_buildings | `_rasterize_buildings.cs`、`_rasterize_buildings_animate.cs` |
| garden_spawn_vegetation | `_garden_spawn_bushes_garden.cs`、`_garden_spawn_flowers_garden.cs`、`_garden_spawn_small_flowers_garden.cs` |
| UI（最终图） | `_egui.raster.*`、`_ui_2d.raster.*`（execute 最终图，I） |

注意区分：上表"文件名存在"是 shader 资产级观察；"CPU 帧内已调用"以 B/A 两节为准。同一 CPU pass 实际绑定哪个 shader 变体未逐项核实（U）。

---

## E. 本文件已确认 vs findings.md

- findings.md 第 4 节所列构图段落全部在本文件中逐调用确认（O）。
- "TAA 与 TGSR 选择关系"：确认，判定字节 param_6[0xab]（O）。
- "DoF 主体开关分支"：确认，判定字节 param_6[0xa9(+type)]（O）。
- "受条件控制的 refraction copy 和水/冰绘制"：确认，全局 \*0x14362c098 与 is_winter（O）。
- "受条件控制的 generate_wall_draw_lists_pass2/gpu_generate_draw_lists"：确认，全局 \*0x14362c0d8（O）。

---

## F. Summary（任务交付三：summary）

### 已确认的帧顺序

**帧外层 `render`（0x1404e8010）**：ECS 资源获取 → DrawLists build/sort → swapchain format/extent → RenderGraphState::compile（模式字节选 0x1401647b0 或 0x140161cd0；后者含 render_world 闭包）→ GraphExecutor::begin_frame → profiler retrieve → PassResources::new_from → **execute_compiled（0x1400fa190）** → retire/take_resource/from_owned_rg_resource → 条件性 **validate_swapchain_and_acquire_image** → 用 acquired 图像 **GraphExecutionRecorder::execute**（最终图/UI）→ into_present_source → end_frame → **Swapchain::present**（仅本帧已 acquire）→ screenshot/stats/refresh_memory_budget → wait_gpu_queue_trim。acquire 在主图执行之后、最终图执行之前（与 findings.md 一致）。

**世界图 `render_world`（0x1404ec110）**（编译期注册顺序）：
1. 上传/程序化：upload_world_raster ×6、water_rasterize_from_cpu、rasterize_terrain_stroke、displace_rocky_terrain、read/mouse_read、foliage exclusion、highlight stroke、object_grass_push、path masks、rasterize_buildings、garden/water/spawn vegetation、path_pebbles、leaf_particles、roof_tiles_gravity、stairs
2. generate_wall_draw_lists → **gpu_generate_draw_lists 第 1 轮** → **shadow_pass** → calc_terrain_lighting_texture → **main_pass** → **generate_hzb**
3. 条件第 2 轮：generate_wall_draw_lists_pass2 + gpu_generate_draw_lists 第 2 轮 → grass_compute（indirect）→ main_pass_oc2 → visibility_queries / dof_autofocus / mouse_depth → late_forward_and_deferred_pass
4. extract_half_res_depth_normals → calculate_reprojection_map → calculate_ssgi
5. GI：ray_trace_gi（软件 BVH 追踪，bind_ray_tracing(ComputePass)）→ eval_gi_screen_radiance → bin_lights → restir_sample_direct_lights → apply_screen_shadows → deferred_raster（draw）
6. 透明/图像：条件 refraction_copy_pass + water_surface_pass/ice_surface_pass（按 is_winter）→ pre_taa_transparency_pass → shadow_mask_mean → apply_volumetrics → dof_prepare → 条件 generate_mips+calc_dof → **calculate_taa 或 calculate_tgsr（互斥）** → post_taa_transparency_pass → render_shader_refactoring

### 条件分支（伪代码可确认）
- 第 2 轮 draw lists：全局 \*0x14362c0d8 == 0
- 折射+水/冰：全局 \*0x14362c098 == 0；水/冰由 is_winter（0x140ae8320）选择
- DoF 主体：param_6[0xa9(+res_type==0x13)] != 0
- TAA vs TGSR：param_6[0xab] 选择（==0→TAA，!=0→TGSR）
- compile 实例：settings+0xac 模式字节
- present 跳过：acquire 返回字节 ==7

### 证据文件
- `evidence/render-passes/cpu-frame/sequence.md`
- `evidence/render-passes/cpu-frame/evidence.json`（59 个 render_world pass、24 个帧步骤、19 个 Ghidra callee 校验、shader 交叉表、分支表）
- `evidence/render-passes/cpu-frame/ghidra-callees-result.json`、`ghidra-water-result.json`（原始 Ghidra 返回）
- 复用 `evidence/render-architecture/*.c` 与 `resolved-calls.json`

### 主要未知
1. 全局 \*0x14362c0d8 / \*0x14362c098 的语义（U）
2. param_6[0xa9/+0xaa]、param_6[0xab] 字节语义（U；影响 DoF 与 TAA/TGSR、GI pair extent）
3. settings+0xac 模式字节选择两个 compile 实例的语义（U）
4. render_shader_refactoring（0x14098f9f0）实际作用（callees 仅 RwLock；U）
5. 每个 CPU pass 绑定的确切 shader 变体（文件名家族已核对，逐 pass 绑定未核；I/U）
6. ray_trace_gi 的 bind_ray_tracing 是软件 raymarch 绑定还是硬件 RT（shader 无 RT capability → I：软件；未验收）
7. water/ice 的 `_mei_shadow` 与 shadow_pass 的一一对应（I）
