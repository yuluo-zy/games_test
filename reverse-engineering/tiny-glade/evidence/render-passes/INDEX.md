# Tiny Glade 原作渲染 Pass 总清单（三路证据合成）

日期：2026-10-13。研究目标：patched B EXE（`tiny-glade.exe`，SHA cf048e27...）+ 发行 `compiled-shaders/` 404 个 `.bin`。

合成自三条独立证据链（均为只读分析，非验收）：
- [shaders/](shaders/catalog.md)：404 compiled-shaders → **260 唯一 pass**（117 raster + 143 compute，638 SPIR-V modules，35 分析族）。
- [cpu-frame/](cpu-frame/sequence.md)：`render_world`（0x1404ec110）确认接线 **54 个 pass 调用**（W1–W54）+ 帧外层 `render`（0x1404e8010）**20 步**（R1–R20）+ 最终图/UI/present；**8 个条件分支**。
- [pass-builder/](pass-builder/pass-declaration.md)：country-core 声明 API（ComputePass/DrawBuilder/PassResources/ShaderUri）+ 二进制字节扫描 **255 条游戏 shader URI，254 条匹配 .bin 文件名**。

标注：**O**=Observation，**I**=Inference，**U**=Unknown。本文件不构成 GPU 动态验收；逐 pass descriptor 句柄、format/extent、执行时序与 shader 算法未恢复。

## 1. 总体结论（O）

| 结论 | 证据 |
|---|---|
| 自建渲染器：tiny-glade 编排帧 → country-core 造 pass → rhapsody::rg 编译图/资源/同步 → rhapsody::raw 出 Vulkan 命令/present | 帧外层 R1–R20 全链 |
| **全程无硬件光追**（638 模块无 RayTracingKHR/RayQuery）；GI 与水/冰的"deferred RT"读同一组软件 BVH SSBO（`rt_bl_bvh`/`rt_triangles`/`rt_triangle_meta`）→ 软件 ray marching | shaders §1 + findings.md §6 |
| HZB 与通用 mips 用 **FidelityFX SPD 单趟签名**（`spdConstants` push const + `imgSrc/imgDst/imgDst6/spdGlobalAtomic`，LocalSize 256） | shaders §7 |
| GPU-driven 实例化：nani cull/bucket 用 `PhysicalStorageBufferAddresses`，nani VS 读 `instance_indirection_buffer`+`nani_instance_data` | shaders §3 |
| **G-buffer 存在**：`extract_half_res_depth_normals` 读 `gbuffer_input_tex`+`stencil_tex`；deferred 读 `light_indirection_grid/grid_light_indices/rdi_reservoir_tex/ssgi_tex/screen_shadows_tex` | shaders §5/§8 |
| 每个 raster pass 恰好 2 个 16-hex 变体编译；97/138 一个变体绑定是另一个严格超集（0 混合）→ 编译期特性变体（I），如 cull 有/无 `hzb_tex`、nani_uber 有/无 `bindless_textures_dyn@2.0`、final_post 全 LUT vs 最小 | shaders §变体结构 |
| TAA 与 TGSR 是**互斥选择**（param_6[0xab]），FSR 仅 history/reactive 两 compute；DoF 主体有开关（param_6[0xa9(+type)]） | cpu-frame C 节 + shaders §11/§12 |
| descriptor 约定：`frame_constants` UBO set1/b0（部分加 `frame_constants_storage_buffer` set1/b1）；sampler set0/0..12；`$Globals` UBO set0/13；texture/SSBO 自 binding14 | shaders §总体 |

## 2. 主表：pass → 类型 → shader → 帧序/条件 → 模块

### 帧外层 `tiny_glade::render_loop::render`（0x1404e8010，O）

| 步骤 | 动作 | 模块 |
|---|---|---|
| R1 | ECS 资源获取、settings 模式字节(+0xac) | bevy_ecs / tiny_glade |
| R2 | `DrawLists::new/build/sort` | `country_core::systems::render_loop::draw_passes` |
| R3 | `Swapchain::format/extent` | `rhapsody::raw::swapchain` |
| R4/R4' | `RenderGraphState::compile`（按模式字节选 0x1401647b0 或 0x140161cd0 含 render_world 闭包）→ `ResourceInfo::calculate` | `rhapsody::rg::{state,compiled,resource_info}` |
| R5–R7 | `GraphExecutor::begin_frame` → profiler retrieve → `PassResources::new_from` | `rhapsody::rg::executor` / `gpu_profiler` / `country_core::render::pass_builder::pass_resources` |
| R8 | `execute_compiled`（BatchCompiler→flush_batch→submit_pending_cbs→逐 pass record_batch→into_exports→ExecutedRenderGraph） | `rhapsody::rg::recorder` / `rhapsody::raw::{command_buffer,submit}` |
| R9 | retire / take_resource / from_owned_rg_resource（导出资源重新导入） | `rhapsody::rg::import_export` |
| R10–R11 | `validate_swapchain_and_acquire_image`（acquire 在主图执行**之后**、最终图之前）；返回字节==7 本帧跳过 present | `rhapsody::raw::swapchain` |
| R12–R14 | 最终图 `execute`（用 acquired 图像，UI/后处理）→ `into_present_source` | `rhapsody::raw::swapchain` |
| R15–R16 | `end_frame` → `Swapchain::present` | `rhapsody::raw::swapchain` |
| R17–R20 | 截图收尾 / stats / refresh_memory_budget / wait_gpu_queue_trim | tiny_glade / `rhapsody::raw::{stats,device}` |

### 世界图 `render_world`（0x1404ec110，编译期注册顺序，O）

#### B1 上传与程序化生成（compute，少量 draw）

| Pass | 类型 | shader URI / 产物 | 模块 |
|---|---|---|---|
| W1 upload_world_raster ×6 | compute | `/upload_cpu_raster.cs.hlsl`（r16f/rgba16f 入口）→ `_upload_cpu_raster` | `country_core::render`（staging→texture） |
| W2 water_rasterize_from_cpu | compute | `/water/*` → `_water_draw_water_mask` 等 | `country_core::render`（水） |
| W3 rasterize_terrain_stroke | compute | `/terrain_editing/rasterize_terrain_stroke` → `_terrain_editing_rasterize_terrain_stroke`（PhysicalStorageBufferAddresses） | `compute_passes::rasterize_terrain_stroke` |
| W4 displace_rocky_terrain | compute | `/rocky_terrain/displace_rocky_terrain` → `_rocky_terrain_displace_rocky_terrain` | `country_core::render`（地形） |
| W5 read_terrain_data | compute | `/read_terrain_height`、`/read_terrain_rock_mask` | `country_core::render`（地形回读） |
| W6 mouse_read_terrain_stroke_index | compute | `/read_terrain_stroke` | `country_core::render` |
| W7 rasterize_foliage_exclusion_mask | draw | foliage exclusion 族 | `country_core::render`（植被排斥） |
| W8 highlight_active_terrain_stroke | compute | `/terrain_editing/calc_highlight_hill_mask` | `country_core::render` |
| W9 object_grass_push_pass | draw | `_grass_object_push.raster` | `country_core::render`（草） |
| W10 compute_path_masks | compute | `/paths/{combine_path_exclusions,directional_contract,path_mask_visual}` | `compute_passes`（路径） |
| W11 rasterize_buildings | compute | `/rasterize_buildings` → `_rasterize_buildings`/`_rasterize_buildings_animate` | `country_core::render`（建筑 mask） |
| W12 garden_rasterize_from_cpu | compute | garden 族 | `country_core::render`（花园） |
| W13 water_spawn_vegetation | compute | `/water/spawn_{boats,lilypads,reeds,waterwheels}` | `country_core::render`（水） |
| W14 spawn_vegetation_around_buildings | compute | `/garden/spawn_*` | `country_core::render` |
| W15 read_bush_positions_for_cpu | compute | `/garden/read_*`（CPU 回读） | `country_core::render` |
| W16 garden_spawn_vegetation | compute | `/garden/spawn_bushes/flowers/small_flowers_garden` | `country_core::render` |
| W17 path_pebbles_compute | compute | `/path_pebbles.cs.hlsl` → `_path_pebbles` | `compute_passes` |
| W18 leaf_particles_compute | compute | `/leaf_particles.cs.hlsl` → `_leaf_particles` | `compute_passes` |
| W19 roof_tiles_gravity_compute | compute | `/roof_tiles_gravity.cs.hlsl` → `_roof_tiles_gravity`（+frame_constants_storage_buffer@1.1） | `compute_passes`（屋顶） |
| W20 stairs | compute | `/rocky_terrain/stairs.cs.hlsl` → `_rocky_terrain_stairs_stairs`（+`_stairs_indirect.raster`） | `compute_passes`（楼梯） |
| W21 generate_wall_draw_lists | compute | `/wall/generate_draw_lists.cs.hlsl:cs_pass1`（无 hzb_tex） | `country_core::render`（墙 draw-list） |
| W22 gpu_generate_draw_lists 第 1 轮 | compute | nani clear/cull_and_bucket（无 hzb 变体 d8520a92…）/histogram/prefix/draw 列表 | `country_core::render::nani` |

#### B2 第一轮可见性与绘制

| Pass | 类型 | shader URI / 产物 | 条件/备注 |
|---|---|---|---|
| W23 shadow_pass | draw | `_mei_shadow.raster.*`（I：无同名文件） | callees render/bind_common |
| W24 calc_terrain_lighting_texture | compute | `/terrain_lighting_texture.cs.hlsl` | |
| — create_raw_resource ×2 | build | main 深度 + HZB 纹理声明 | |
| W25 main_pass | draw | nani_* raster（uber/albedo_map/vertex_color/velocity_only…） | 主深度/反照率/速度 |
| W26 generate_hzb | compute | `/generate_hzb.cs.hlsl` → `_generate_hzb`（SPD） | |

#### B3 HZB 后补充阶段

| Pass | 类型 | shader | 条件 |
|---|---|---|---|
| W27 generate_wall_draw_lists_pass2 | compute | `/wall/generate_draw_lists.cs.hlsl:cs_pass2`（带 hzb_tex@0.19）+ `_wall_reset_pass2_inst_counts` | 全局 \*0x14362c0d8==0 |
| W28 gpu_generate_draw_lists 第 2 轮 | compute | nani cull_and_bucket（带 hzb 变体 8a7d2f81…） | 同上（第三参非 0） |
| W29 grass_compute | compute | `_generate_grass`/`_grass_calc_tiles`(读hzb)/`_grass_tile_bundle_*`/`_grass_bundles_prefix`/`_grass_calc_dispatch_args` | dispatch_indirect_raw；写 DrawIndexedIndirect/DrawIndirect/AtomicCounter |
| W30 main_pass_oc2 | draw | nani raster（遮挡二次判定） | |
| W31 run_visibility_queries | compute | visibility query 族（U 具体 shader） | |
| W32 calculate_dof_autofocus | compute | `/dof/autofocus.cs.hlsl` → `_dof_autofocus`/`_dof_analyze` | |
| W33 mouse_depth_compute | compute | `/read_mouse_depth_value.cs.hlsl` | |
| W34 late_forward_and_deferred_pass | draw | nani_* + `/deferred/*_light_sun_sky.raster` 组合 | bind_common+render |
| W35 extract_half_res_depth_normals | compute | `/extract_half_res_depth_normals.cs.hlsl`（读 gbuffer+stencil） | |
| W36 calculate_reprojection_map | compute | `/calculate_reprojection_map.cs.hlsl` | |
| W37 calculate_ssgi | compute | `/ssgi/{ssgi,temporal_filter,upsample}.cs.hlsl` | |

#### B4 GI 求解（历史 temporal pair 创建后无条件执行）

| Pass | 类型 | shader | 备注 |
|---|---|---|---|
| — GI history temporal pair | build | `get_or_create_temporal_pair` | extent 按 param_6[0xab] |
| W38 ray_trace_gi | compute | `/gi/trace_gi_candidates.cs.hlsl`（软件 BVH）+ `_gi_update_gi_radiance` | bind_ray_tracing(ComputePass) |
| W39 eval_gi_screen_radiance | compute | `/gi/eval_screen_radiance.cs.hlsl` + `_gi_reproject_screen_radiance` + `_gi_blur_screen_radiance` | |
| W40 bin_lights | compute | `/light_binning/bin_lights_to_cells.cs.hlsl` + `_cell_prefix_sum/_subtotal/_merge` | |
| W41 restir_sample_direct_lights | compute | `_rdi_temporal` 候选（U） | ReSTIR 蓄水池（rdi_reservoir_tex） |
| W42 apply_screen_shadows | compute | `/apply_screen_shadows.cs.hlsl` | 写 shadow_mask_histogram |
| W43 deferred_raster | draw | `/deferred/{solid,grass,plant,canopy}_light_sun_sky.raster` | draw_no_vao + bind_color_library + bind_ray_tracing(DrawBuilder) |

#### B5 透明与图像处理

| Pass | 类型 | shader | 条件 |
|---|---|---|---|
| W44 refraction_copy_pass | draw | `_water_plane_refraction_copy`/`_waterfall_refraction_copy` | 全局 \*0x14362c098==0 |
| W45 water/ice_surface_pass | draw+compute | 水：`_water_plane`+`_water_deferred_rt`+`_water_calc_deferred_rt_args`；冰：`_water_ice_{deferred_rt,refl_march,refl_resolve,refr_resolve}`+`_ice_plane`+`_water_ice_cracks` | is_winter 二选一 |
| W46 pre_taa_transparency_pass | draw | 透明 nani raster（window_glass/cloth…） | |
| W47 compute_shadow_mask_mean | compute | `/shadow_mask_mean.cs.hlsl` | |
| W48 apply_volumetrics | compute | `/apply_volumetrics.cs.hlsl` | |
| W49 dof_prepare | compute | `/dof/{prepare,init_buffers,clear_halfres}.cs.hlsl` | 读 DofAutofocusSsbo |
| W50 generate_mips | compute | `/generate_mips.cs.hlsl`（SPD） | param_6[0xa9(+type)]!=0 |
| W51 calc_dof | compute | `/dof/{halfres,fused,fullres,fullres_upsample,fullres_all_lod0,fullres_all_lod1}` | 同上 |
| W52 calculate_taa / calculate_tgsr | compute | `/taa.cs.hlsl` ↔ `/tgsr.cs.hlsl`(+`_tgsr_reproject`+`_tgsr_blur_lum`) | param_6[0xab]==0→TAA，!=0→TGSR |
| W53 post_taa_transparency_pass | draw | 透明 nani raster | |
| W54 render_shader_refactoring | U | `_shader_refactoring.cs.hlsl_{capture,compare}`（dev 差分） | callees 仅 RwLock；作用 U |

### 最终图 / present（R12–R16，O）

| pass | shader | 模块 |
|---|---|---|
| 最终图 execute（UI/后处理） | `_final_post.raster`（色调映射/grading_lut/icc_lut/tony_mc_mapface_lut/shadow_mask_mean）+ `_blit_resolve_to_swapchain`/`_blit_resolve_msaa_to_swapchain`（MSAA 档）/`_blit_blit_stretch_decode_srgb` + `_ui_2d`/`_egui`/`_text`/`_custom_cursor` | `tiny_glade::render_loop` / `rhapsody::raw::swapchain` |
| Swapchain::present | — | `rhapsody::raw::swapchain` |

## 3. 不在 render_world 主帧的 shader 族（存在但接线/触发未知，I/U）

| 族 | 用途推断 | 备注 |
|---|---|---|
| painterly_vermeer（6C+5R） | 风格化笔触绘制链（`generate_strokes`→`raster_strokes`→`combine`） | 缩略图/画廊；`vermeer::GenerateStrokes` 有 URI `/vermeer/generate_strokes.cs.hlsl`（O） |
| session_ui（8R） | 启动/会话选择/Logo 全屏 | `_loading_screen`/`_session_selection_*`/`_logo_*` |
| fsr_fxaa（3C） | `_fsr_copy_history`/`_fsr_init_reactive`/`_fxaa` | 二进制中无 `/<name>.cs.hlsl` URI（U 接线） |
| sky_cloud（3R） | `_skydome`/`_cloud`/`_star` | |
| screenshot_capture（3C） | `_screenshot_capture_screenshot_{buffer,texture}`/`_capture_panic_screenshot` | 崩溃截图 |
| debug_dev（5） | `_fill_buffer`/`_normal_debug`/`_texture_test`/`_shader_refactoring`/`_primary_rt_debug` | 开发/调试 |
| critters（1C+10R） | 蝴蝶/萤火虫/鸭/鹪鹩/雨雪粒子/绵羊 | `_critters_firefly_vis_query` 是可见性查询 compute |
| blur_mips / prefix_scan / lut | 通用原语：`_blur*`、`_prefix_scan*`、`_gauss_blur*`、`_lut_brdf_fg` | 供 nani/grass/light_binning/后处理复用（O 家族存在，具体复用点 U） |

## 4. 模块归属总结

| 层 | 模块 | 承担 |
|---|---|---|
| L0 帧编排 | `tiny_glade::render_loop` | render / render_world / 最终图 / 截图收尾 |
| L1 游戏 pass | `country_core::render::nani` | GPU cull/分桶/主绘制（nani_* 全部） |
| L1 游戏 pass | `country_core::render::pass_builder`（compute_pass_builder/draw_builder/pass_resources/pass_resource_access/bind_resources/uniform_binder/draw_stream/shader_uri） | 全部 pass 声明/绑定/URI 解析 |
| L1 游戏 pass | `country_core::render`（各内容族） | 地形/水/植被/屋顶/草/路径/建筑/透明/后处理/GI/阴影/DoF/AA |
| L1 游戏 pass | `country_core::systems::render_loop::draw_passes` | CPU draw-list 构造排序 |
| L2 渲染图 | `rhapsody::rg::{graph,resource_info,resource_registry,compiled,state,pass_builder,executor,recorder,import_export,temporal,binding}` | 图编译/资源/同步/录制/执行/跨帧 |
| L3 Vulkan | `rhapsody::raw::{command_buffer,submit,swapchain,shader,descriptor_set,render_pass,stats,device,profiler}` | 命令/提交/呈现/shader 加载/管线 |
| L5 工具链 | `shader-compiler`/`shader-pipeline`/`shader-variants` | 变体编译产物（两个 16-hex 变体对） |
| 效果 | `fsr`/`fsr-sys` | FSR 接线（接线点 U） |

## 5. 主要 Unknown

1. 全局 \*0x14362c0d8（第 2 轮）与 \*0x14362c098（折射/水面）语义；param_6[0xa9/+0xaa/0xab] 与 settings+0xac 模式字节含义。
2. 两 16-hex 变体对（b903…/f0ad…）的确切特性 flag 与选择规则。
3. TAA/TGSR/FSR/FXAA 每帧实际组合；DoF fused vs halfres/fullres 档位触发。
4. 软件 BVH（rt_bl_bvh/rt_triangles）构建端与 GI/水共用方式。
5. render_shader_refactoring（0x14098f9f0）实际作用。
6. 每个 CPU pass 绑定的确切 shader 变体与 descriptor 句柄、format/extent（需 PDB/机器码/运行时证据）。
7. restir_sample_direct_lights 的确切 shader（`_rdi_temporal` 为候选）。
8. FSR 编译产物的静态 URI 缺失；`/clearing/` 家族为运行时动态拼接。
9. draw/dispatch 模式枚举值（2/3/4/7）未映射回 Rust 枚举名；0x44 字节 RenderPassBinding 字段布局未恢复。

## 6. 证据文件索引

- 合成：本文件 INDEX.md / INDEX.json
- shader 侧：`shaders/{catalog.json,catalog.md,method.md,summary.md}`；解析器 `scripts/render-passes/parse_shader_catalog.py`
- CPU 帧侧：`cpu-frame/{sequence.md,evidence.json,ghidra-callees-result.json,ghidra-water-result.json}`
- 声明侧：`pass-builder/{pass-declaration.md,symbol-locations.json,shader-uri-list.json,compiled-shaders-full-list.txt,dec{1,2,3}_0x*.c}`
- 复用：`evidence/render-architecture/{findings.md,rendering-framework-map.md,0x*.c,resolved-calls.json,shader-reflection.json,symbol-seeds.json}`
