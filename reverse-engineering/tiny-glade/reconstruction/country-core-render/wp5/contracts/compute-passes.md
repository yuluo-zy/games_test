# WP5 契约 2/3 — compute_passes（CPU 侧帧 compute pass 组合）

版本 **B**。数据源：skeleton bundle（`compute_passes/*.rs.{pdb,structures,enriched,dependencies}` 家族，见 evidence.json）、
`evidence/render-architecture/0x140987dd0.c`、`0x1404ec110.c`、`public-symbols.json`（地址→PDB 名）、
`pass-builder/pass-declaration.md`（ComputePass API 与 AsComputeShaderSource 表）、`shader-uri-list.json`（255 URI）。

标注：**O**=反编译/PDB/URI 直接支持；**I**=调用链组合；**U**=未恢复。`resources::` 游戏状态（水/地形/屋顶/草）为 typed gap，引用不臆造字段。

通用 wiring：每个 pass 以 `ComputePass::new<T: AsComputeShaderSource>(uri, world)` 建 400B builder；
`read_storage_buffer<T>/write_storage_buffer<T>/write_storage_buffer_concurrent<T>` 声明 SSBO 访问；
`read_texture/write_texture/read_write_texture` 声明纹理访问；`push_constants` 写 PUSH_CONSTANTS；
`dispatch_1d/2d/3d/groups/dispatch/dispatch_indirect_raw` 落到 `PassBuilder::render<PassResources, dispatch_impl::closure_env_0>`。
dispatch 模式判别 2/3/4（ComputePass）与 draw 模式（DrawBuilder）不同枚举，U 映射名。
descriptor 约定：`frame_constants` UBO set1/b0、sampler set0/0..12、`$Globals` set0/13、texture/SSBO 自 binding14。

## 1. `gpu_generate_draw_lists.rs`（W21/W22/W27/W28，最高价值）

模块内函数（O，PDB + enriched；行号来自 pdb.jsonl line 表）：

| 函数 | 行 | 语义 |
|---|---|---|
| `gpu_generate_draw_lists(world, round)` | 系统 | 第 1/2 轮 GPU draw-list 生成（0x140987dd0，第三参=round） |
| `clear_draw_lists` | 216 | `/nani/clear_draw_lists.cs.hlsl`；dispatch_1d |
| `clear_histogram` | 240 | `/nani/clear_histogram.cs.hlsl`；dispatch_1d |
| `clear_draw_cmd_histogram` | 871 | `/nani/histogram_prefix_sum.cs.hlsl`（清空路径，dispatch_1d） |
| `clear_draw_cmd_subtotals` | 891 | `/nani/clear_subtotals.cs.hlsl`（push_constants + dispatch_1d） |
| `prepare_empty_unsorted_draw_lists` | 1033 | `/nani/prepare_unsorted_draw_lists.cs.hlsl`；读 `GpuWorld::mesh_atlas_info_buf`（O） |
| `cull_and_bucket` | 260 | `/nani/cull_and_bucket.cs.hlsl`（`dispatch_with` 闭包，closure_env$1）；迭代 `enum_iterator::All<GpuWorldSubsetKind>`，filter_map 闭包，`itertools::multiunzip` 产出 `(Vec<cull_and_bucket::PushConstants>, Vec<u32>)`；按 subset 推 PushConstants 后逐 subset dispatch |
| `histogram_prefix_sum` | 645 | `/nani/histogram_prefix_sum.cs.hlsl`；dispatch_1d |
| `generate_draw_list_dispatch_args` | 687 | `/nani/generate_dispatch_args.cs.hlsl`；dispatch_1d |
| `draw_indexed_indirect_to_dispatch` | 506 | `/nani/draw_indexed_indirect_to_dispatch.cs.hlsl`；`RenderGraphRecorder::create`（间接 arg buffer）+ push_constants + dispatch_1d |
| `allocate_draw_lists` | 911 | `/nani/allocate_draw_commands.cs.hlsl`（I，dispatch 路径在闭包） |
| `fill_unsorted_instance_indirection` | 1165 | `/nani/fill_unsorted_instance_indirection.cs.hlsl` |
| `create_cull_and_bucket_pass<T>` | 584 | `ComputePass::new<T>` + `read_texture`（hzb 变体）+ `uniform`；第 2 轮带 hzb |
| `cull_and_bucket_external<SsboT, CullT>` | — | 外部实例缓冲剔除（WoodenPlank→`NaniCullAndBucketWoodenPlank`，RoofTile→`NaniCullAndBucketRoofTile`）；`utils::get_res` + `AssetSsboLibrary::get<SsboT>` + `create_cull_and_bucket_pass` |
| `generate_wall_draw_lists(world)` | — | W21；`/wall/generate_draw_lists.cs.hlsl:cs_pass1`；读 wall 实例（`UnsafeWorldCell::get_entity` 闭包），`ssbo_to_rg_handle` + `ComputePass::write_draw_indirect_cmd_buffer` |
| `generate_wall_draw_lists_pass2(world)` | — | W27；`:cs_pass2` + `/wall/reset_pass2_inst_counts.cs.hlsl`；`reset_wall_draw_indirect_args`（1287，`DrawBuilder::uniform` + dispatch_1d）+ `read_texture`（hzb_tex@0.19） |
| `IndirectArgsType` | 678 | 枚举（O 名；`buffer_offset()` 方法，语义 I） |
| `RenderSublistsIter` | 662 | 迭代器（`next`），`GpuWorldSubsetKind::use_fine_grained_sorting` 驱动；供 group_draw_cmd_prefix_sum `dispatch_indirect_with`（closure_env$0） |

反编译关键事实（0x140987dd0.c，已解码）：
1. 取 GpuWorld subset 槽（`param_2+0x1e0`、`+0x1450` 数组）；`BufferDesc::new_gpu_only(0x6c000, 0)` + `create_raw_resource` 建清空缓冲；第二块 `(subset_inst_count[0x60]<<2)`。
2. `ComputePass::new` ×2（clear_draw_lists、clear_histogram），`read/write_storage_buffer` 声明，`PassBuilder::render`。
3. 按 round 选 `AssetSsboLibrary::get_rg_buffer`（0x1412d1180 第 1 轮 / 0x1412ce810 第 2 轮）+ `render_loop::clear_rg_buffer`（0x1409ddbb0）。
4. `ComputePass::new`（cull_and_bucket）+ `bind_external_storage_buffer`（subset 信息@+0x1458 与 +0x1480）+ write_storage_buffer → `multiunzip`（0x140b2b7a0）→ `create_cull_and_bucket_pass`（0x140a28950）→ `dispatch_with`（0x14160a2e0，cull_and_bucket closure_env$1）。
5. `dispatch_indirect_with<group_draw_cmd_prefix_sum::closure_env$0<RenderSublistsIter>>`（0x14160b850）执行排序链。

帧序：W22（round 0，无 hzb）→ W27/W28（round 非 0，带 hzb，`\*0x14362c0d8==0` 时）。

Shader URI（O）：`/nani/{clear_draw_lists,clear_histogram,prepare_unsorted_draw_lists,cull_and_bucket*,histogram_prefix_sum,group_draw_cmd_prefix_sum,subtotal_prefix_sum,allocate_draw_commands,generate_draw_lists,draw_indexed_indirect_to_dispatch,fill_sorted_instance_list,fill_unsorted_instance_indirection,prefix_sum_unsorted_instances_per_atlas_entry,clear_subtotals,generate_dispatch_args}.cs.hlsl`；`/wall/generate_draw_lists.cs.hlsl:cs_pass1|cs_pass2`；`/wall/reset_pass2_inst_counts.cs.hlsl`。

资源读/写（O 句柄 hash，名映射 U）：read `NaniMeshAtlasPartInfo`、`NaniInstanceIndirectionSsbo`；write `NaniDrawListSsbo`、`NaniDrawListLengthSsbo`、histogram/counter SSBO；`frame_constants` uniform；`hzb_tex`（第 2 轮 read_texture）。

## 2. `dof.rs`（W49/W51）

| 函数 | 行 | 语义 |
|---|---|---|
| `dof_prepare` | W49 | `/dof/prepare.cs.hlsl`（+`/dof/init_buffers.cs.hlsl`、`/dof/clear_halfres.cs.hlsl`）；读 `DofAutofocusSsbo`；read_texture + write_texture + dispatch |
| `calc_dof` | W51 | 按 `calc_dof_multires`（162）或 `calc_dof_fullres`（79）分支；前者逐 mip 步进（Iterator::count + multiunzip），后者直接 dispatch |
| `calc_dof_multires` | 162 | `/dof/{halfres, fused, fullres_upsample, fullres_all_lod0, fullres_all_lod1}`；`ImageFormationConfigs::current`；DynamicConstants::push_slice + push_constants + dispatch_1d/2d |
| `calc_dof_fullres` | 79 | `/dof/fullres.cs.hlsl`；dispatch |

AsComputeShaderSource 实现（O）：`FullResDof`→`/dof/fullres.cs.hlsl:cs_main`、`FullResAllLod0Dof`、`FullResAllLod1Dof`、`HalfResDof`→`:cs_halfres`、`HalfResAllDof`→`:cs_halfres_all`。
结构体 `IndirectArgsType`（U 字段）。
分支：W50/W51 由 `param_6[0xa9+(res_type==0x13?1:0)] != 0` 门控（frame-wiring §3）。

## 3. `taa.rs`（W52）

| 函数 | 语义 |
|---|---|
| `calculate_taa` | `/taa.cs.hlsl`；`utils::get_res` + uniform + read_texture + write_texture + dispatch_2d |
| `calculate_tgsr` | `/tgsr.cs.hlsl:cs_main`（+`/tgsr_reproject.cs.hlsl`、`/tgsr_blur_lum.cs.hlsl`）；`Temporal::split_pair`/`into_inner` + `Option::is_some_and`；min_subgroup_size（O，sequence.md）；dispatch_2d |

`TgsrMain` 实现 `AsComputeShaderSource`（O）。选择：`param_6[0xab]==0 → TAA，!=0 → TGSR`。

## 4. `dof_autofocus.rs`（W32）

`calculate_dof_autofocus`：`/dof/autofocus.cs.hlsl`；`MainCamera::focus` + `WindowSize::aspect_ratio/as_vec2` 计算 focus 参数 → uniform + read_texture + dispatch_2d。
结构体：`DofAutofocusSsbo`（写，读于 dof_prepare）、`DofFocusReadbackSsbo`。

## 5. `grass.rs`（W29）

| 函数 | 语义 |
|---|---|
| `register_required_components` | bevy 组件注册（`Vec::push/pop`，行 107） |
| `grass_precompute` | 行 132；`bind_terrain_heightmap` + read_texture + uniform + dispatch_1d/groups |
| `grass_compute` | `grass_precompute` + `ComputePass::write_draw_indirect_cmd_buffer` + uniform + read_texture；dispatch_indirect（O：sequence.md） |
| `clone_behavior` | I |

Shader URI：`/grass/{calc_tiles, tile_bundle_gen, tile_bundle_count, tiles_prefix, bundles_prefix, calc_dispatch_args, calc_grasslet_bundle_ranges}.cs.hlsl` + `_generate_grass.cs`。写 DrawIndexedIndirect/DrawIndirect/AtomicCounter（sequence.md）。`inclusive_prefix_scan_u32_1m` 复用（见 §22）。
结构体：`GrassBladeDataSsbo`、`ObjectGrassPush`（draw_passes 共享）。

## 6. `apply_screen_shadows.rs`（W42/W47）

| 函数 | 语义 |
|---|---|
| `apply_screen_shadows` | `/apply_screen_shadows.cs.hlsl`；uniform + `bind_shadowmap` + read_texture + write_texture + dispatch_2d；写 shadow_mask_histogram |
| `compute_shadow_mask_mean` | `/shadow_mask_mean.cs.hlsl`；dispatch_1d；读 `ShadowMaskHistogramSsbo` → 写 `ShadowMaskMeanSsbo` |

结构体：`ShadowMaskHistogramSsbo`、`ShadowMaskMeanSsbo`（U 布局；final_post 消费）。

## 7. `apply_volumetrics.rs`（W48）

`apply_volumetrics`：`/apply_volumetrics.cs.hlsl`；`MainCamera::mode` + `bind_shadowmap` + read_texture + write_texture + uniform + dispatch。

## 8. `half_res.rs`（W35）

`extract_half_res_depth_normals`：`/extract_half_res_depth_normals.cs.hlsl`；`RenderGraphRecorder::create` + read_texture（`gbuffer_input_tex`）+ `read_texture_view`（`stencil_tex`）+ write_texture + dispatch_2d。

## 9. `reprojection.rs`（W36）

`calculate_reprojection_map`：`/calculate_reprojection_map.cs.hlsl`；uniform + read_texture + write_texture + dispatch_2d。

## 10. `ssgi.rs`（W37）

`calculate_ssgi`：`/ssgi/{ssgi, temporal_filter, upsample}.cs.hlsl`；read_texture + write_texture + dispatch_2d + uniform。

## 11. `ray_trace_gi.rs`（W38）

`ray_trace_gi`：`/gi/{trace_gi_candidates, update_gi_radiance}.cs.hlsl`；软件 BVH（`rt_bl_bvh`/`rt_triangles`/`rt_triangle_meta`，I：无 RT capability）；`bind_shadowmap` + `bind_terrain_heightmap` + `bind_terrain_lighting` + read_texture + write_texture + dispatch_2d + `Temporal::into_inner`。GI history pair（extent 按 param_6[0xab]）创建后无条件执行。

## 12. `eval_gi_screen_radiance.rs`（W39）

`eval_gi_screen_radiance`：`/gi/{eval_screen_radiance, reproject_screen_radiance, blur_screen_radiance}.cs.hlsl`；`ShaderRefactoringState::disabled`；read/write/read_write_texture + dispatch_2d。

## 13. `bin_lights.rs`（W40）

`bin_lights`：`/light_binning/bin_lights_to_cells.cs.hlsl:cs_main`（`BinLights` AsComputeShaderSource）+ `/light_binning/{cell_prefix_sum, cell_subtotal_prefix_sum, cell_prefix_merge}.cs.hlsl`；`AssetSsboLibrary::get` + `InstanceBuffer::instance_count` + uniform + push_constants + dispatch_1d。

## 14. `restir_di.rs`（W41）

`restir_sample_direct_lights`：`_rdi_temporal` 候选（U 确切 shader）；`AssetSsboLibrary::get` + `InstanceBuffer::instance_count` + read_texture + write_texture + dispatch_2d；ReSTIR reservoir（`rdi_reservoir_tex`）。

## 15. `mouse_depth_value.rs`（W33）

`mouse_depth_compute`：`/read_mouse_depth_value.cs.hlsl`；`WindowSize::as_vec2` + clamp + read_texture + uniform + dispatch；`MouseDepthValueSsbo`（write_to_cpu_mapped_storage_buffer，wp1 §1.1）。

## 16. `visibility_queries.rs`（W31）

`run_visibility_queries`：visibility query 族（U 具体 shader）；read_texture + `bind_terrain_heightmap` + dispatch_1d。

## 17. `generate_hzb.rs`（W26）

`generate_hzb`：`/generate_hzb.cs.hlsl`（SPD 单趟）；`ustr::ustr` 名字 + `next_power_of_two` + `div_ceil` + `Handle::desc`；`read_texture`（main 深度）+ `read_write_texture`（各 mip）+ `push_constants`（spdConstants）+ `dispatch_groups`；`bind_individual_mips` + `write_rg_storage_buffer` + `get_or_create_temporal`（sequence.md W26 备注）。

## 18. `generate_mips.rs`（W50）

`generate_mips`：`/generate_mips.cs.hlsl`（SPD）；`ComputePass::read_write_texture` + `push_constants` + `dispatch_groups`（`WriteTextureUnchecked`/`TexturePreset` 两单态，eq+expect 断言）。分支：`param_6[0xa9+(type==0x13)]!=0`。

## 19. `blur_pyramid.rs`（复用原语，非独立帧 pass）

`blur_pyramid`（dispatch_2d，overflowing_sub+max 迭代）、`rev_blur_pyramid`（dispatch_3d + read_texture_view）、`bidirectional_blur_pyramid`（组合）。供 GI/nani/后处理复用（具体复用点 U）。

## 20. 上传/程序化生成族（B1，O 统一模式）

| 模块 | pass（W#） | URI | 模式 |
|---|---|---|---|
| `upload_world_raster.rs` | upload_world_raster ×6（W1） | `/upload_cpu_raster.cs.hlsl`（r16f/rgba16f 入口） | `utils::get_res_mut` + `on_uploaded_to_texture` + write_texture + uniform + dispatch_2d；6 单态（PathRaster/Cobblestone1/2/3/WaterRaster/GardenRaster） |
| `compute_water.rs` | water_rasterize_from_cpu（W2） | `/water/*` → `_water_draw_water_mask` | `on_texture_processed` + write_texture + dispatch_2d |
| `compute_water.rs` | water_spawn_vegetation（W13） | `/water/spawn_{boats,lilypads,reeds,waterwheels}`（boats/wheels URI 动态拼接，U） | `ActiveSession::is_autumn` + `bind_terrain_heightmap` + write_texture + write_draw_indirect_cmd_buffer + dispatch |
| `compute_garden.rs` | garden_rasterize_from_cpu（W12） | garden 族 | `on_texture_processed` + write_texture + dispatch_2d |
| `compute_garden.rs` | garden_spawn_vegetation（W16） | `/garden/spawn_{bushes_garden,flowers_garden,small_flowers_garden}.cs.hlsl` | `bind_terrain_heightmap` + write_draw_indirect_cmd_buffer + dispatch/2d |
| `spawn_vegetation_around_buildings.rs` | W14 | `/garden/spawn_{bushes_buildings,flowers_buildings}.cs.hlsl` | uniform + bind_terrain_heightmap + write_draw_indirect_cmd_buffer + dispatch |
| `read_bush_positions_for_cpu.rs` | W15 | `/garden/read_bush_positions_for_cpu.cs.hlsl` | `ComputePass::read_draw_indirect_cmd_buffer` + dispatch_1d；`BushCpuSsbo` |
| `spawn_boats.rs` | — | `/water/spawn_boats.cs.hlsl`（静态 URI 缺失，U） | `BoatSsbo` 结构体存在；dispatch 并入 water_spawn_vegetation（I） |

结构体（U 布局，`resources::` 域）：`BushSsbo/FlowerSsbo/Flower03Ssbo/GardenStoneSsbo`（compute_garden）、`LilypadSsbo/LilypadFlowerSsbo/ReedSsbo`（compute_water）、`BoatSsbo`。

## 21. 地形/建筑/路径族

| 模块 | pass（W#） | URI | 模式 |
|---|---|---|---|
| `rasterize_terrain_stroke.rs` | rasterize_terrain_stroke（W3） | `/terrain_editing/rasterize_terrain_stroke.cs.hlsl` | `Events::is_empty` 守卫；`DynamicConstantsChunk::with_data` + `DynamicConstants::push_slice` + uniform + write_texture + dispatch_2d；`bind_terrain_heightmap`；`GaussBlur`（`/gauss_blur.cs.hlsl:cs_main`）+ `RecalculateHeightMapCmd`（U） |
| `displace_rocky_terrain.rs` | W4 | `/rocky_terrain/displace_rocky_terrain.cs.hlsl` | `bind_terrain_heightmap` + read_texture + push_constants + dispatch |
| `stairs.rs` | W20 | `/rocky_terrain/stairs/stairs.cs.hlsl` | read + write_draw_indirect_cmd_buffer + read_draw_indirect_cmd_buffer + dispatch_2d + dispatch；`StairsTransformSsbo`；raster `_stairs_indirect.raster` |
| `path_pebbles.rs` | W17 | `/path_pebbles.cs.hlsl` | write_draw_indirect_cmd_buffer + uniform + read_texture + bind_terrain_heightmap + dispatch；`PebblePlacementSsbo` |
| `leaf_particles.rs` | W18 | `/leaf_particles.cs.hlsl` | `utils::get_res` + min + uniform + dispatch；`LeafParticleSsbo/ParticleEmitterSsbo/LoadedParticleEmitterCount`（U） |
| `roof_tiles_gravity.rs` | W19 | `/roof_tiles_gravity.cs.hlsl` | `AssetSsboLibrary::get` + World::query（RoofTiles 组件）+ flat_map + uniform；`PushConstants`（闭包 closure$0/closure$1，dispatch_1d）；`+frame_constants_storage_buffer@1.1`（O） |
| `rasterize_buildings.rs` | W11 | `/rasterize_buildings.cs.hlsl` + `/rasterize_buildings_animate.cs.hlsl` | `HashMap::iter` + `AssetSsboLibrary::get` + `InstanceBuffer::gpu_write` + write_texture + dispatch_2d；`GroundedBuildingsSsbo` |
| `compute_path_masks.rs` | W10 | `/paths/{combine_path_exclusions,directional_contract,path_mask_visual}.cs.hlsl` | `on_texture_processed` + `blur_exclusion_mask`/`blur_path_mask`（`/gauss_blur_mask.cs.hlsl` 族，I）+ write_texture + dispatch_2d |
| `terrain_lighting_texture.rs` | W24 | `/terrain_lighting_texture.cs.hlsl` | `bind_terrain_heightmap` + `bind_shadowmap` + uniform + write_texture + dispatch |
| `read_terrain_data.rs` | W5 | `/terrain_editing/read_terrain_height.cs.hlsl`、`/terrain_editing/read_terrain_rock_mask.cs.hlsl`、`/water/read_water_mask.cs.hlsl`、`/garden/read_garden_mask.cs.hlsl` | 各 dispatch_1d 回读；`read_terrain_data` 组合（`ScheduledTerrainReadback::on_gpu_copy_issued`） |
| `mouse_read_terrain_stroke_index.rs` | W6 | `/read_terrain_stroke.cs.hlsl` | `CursorTerrainRaycast::position` + read_texture + push_constants + dispatch；`MouseTerrainRaycastSsbo` |
| `highlight_active_terrain_stroke.rs` | W8 | `/terrain_editing/calc_highlight_hill_mask.cs.hlsl` | `EventCursor::read` + last + uniform + write_texture + dispatch；`AddTerrainHighlightCmd` |

## 22. 原语/调试族

| 模块 | 函数 | URI | 模式 |
|---|---|---|---|
| `prefix_scan.rs` | `inclusive_prefix_scan_u32_1m` | `/nani/prefix_sum_unsorted_instances_per_atlas_entry.cs.hlsl`（I） | div_ceil + max + next_multiple_of + dispatch_1d；grass/nani 复用 |
| `reset_indirect_args.rs` | `reset_indirect_args` | `/clearing/`（U） | `ssbo_to_rg_handle` + `RenderGraphRecorder::add_pass` + `DynamicConstants::push_slice` + `impl$2::write`（GpuOwned 追踪） |
| `shader_refactoring.rs` | `render_shader_refactoring`（W54） | `_shader_refactoring.cs.hlsl_{capture,compare}` | read_texture + write_texture + dispatch_2d；callees 仅 RwLock（作用 U） |
| `primary_rt_debug.rs` | `primary_rt_debug` | debug 族 | write_texture + bind_terrain_heightmap/lighting + bind_shadowmap + dispatch |
| `mouse_read_terrain_stroke_index.rs` | W6 | `/read_terrain_stroke.cs.hlsl` | 见上 |

## 23. 共用 typed gap

- `resources::` 游戏状态（water/terrain/roof/grass/roof_tile_ssbo 等 SSBO 内容、`*Ssbo` 字段布局）：**未恢复，引用为 typed gap**（`BoatSsbo`、`PebblePlacementSsbo` 等均为名 + 尺寸 unknown）。
- dispatch 模式判别 2/3/4 → Rust 枚举名：U。
- `RenderGraphRecorder::create`/`add_pass` 与 `DynamicConstants` 属 rhapsody 复用面（rhapsody-rebuild / wp1 边界）。
- handle hash（`0x662978458699a5ef` 等）→ 资源名映射：U（`PassResources::new_from` 7 hash，wp1）。
