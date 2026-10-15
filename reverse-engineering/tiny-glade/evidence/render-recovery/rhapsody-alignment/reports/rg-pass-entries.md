# rg 工作包 5：3 个 compile 实例里内联游戏闭包的 pass 入口清单

证据：`evidence/render-passes/cpu-frame/evidence.json`（`render_world_passes_ordered` / `compile_instance_order`）、
`render-passes/cpu-frame/sequence.md` §B、`producer-closure/g3-draw-passes/findings.md`、`g7-post-ui/findings.md`、
`country-core-deep/rhapsody/CALL-CHAIN.md` §2.3。rhapsody 内的接口占位：`crates/rhapsody/src/rg/pass_entries.rs`
（`CompileInstance`、`PassEntryInfo`、`WORLD_GRAPH_PASS_ENTRIES`、`trait PassEntry`），**不含业务**。

说明：
* 地址为原 EXE（B patched）VA；"RenderPassApi/构图方法"一列只写证据里直接出现过的，空白 = **未读**，不是"无"。
* 表里是 `render_world`（0x1404ec110）的 **59 个调用点**；"54 个入口"指 country-core `FramePasses` trait 的方法数
  （`upload_raster_staging_to_texture` 6 实例、`gpu_generate_draw_lists` 2 轮、水面/冰面与 TAA/TGSR 互斥分支折叠后）。
  59 行里共 60 个不同入口地址（两处互斥分支各含 2 个地址，`gpu_generate_draw_lists` 两轮同址）。
* pass 回调的原签名是 `FnOnce(u64) -> bool`；rhapsody 现入口 `PassBuilder::render(FnOnce(u64)->bool)` 与原版一致，
  `render_with_context(FnOnce(PassContext)->bool)` 为兼容扩展。

## 三个 compile 实例

| 实例 | 地址 | Export | 闭包内容 | 调用方 |
|---|---|---|---|---|
| 加载图 | 0x1401647b0 | `ResourceExportHandle<Texture>` | `loading_background_path` → `image::open`/`to_rgba8` → `create_texture` → `vermeer_render_from_target` / `mei_pass`(0x1409c6700) / `loading_screen_pass`(0x140837ff0，与 `ui_pass` 0x140837c40 同一 RasterPass 名 `UI`)；**不调 render_world** | `render` 0x1404e8010（模式字节 == 0） |
| 世界图 | 0x140161cd0 | `ResourceExportHandle<Texture>` | 前段 `import_texture_presets_to_rg`/`GpuWorld::import_to_rg`/…；`create_raw_resource`×4；`render_world`（下表）；尾段 `Texture::export`/`export_preset_textures_from_rg`/`GpuWorld::export_from_rg` 等 | `render`（模式字节 != 0；OOM 时降分辨率再编译一次） |
| 展示图 | 0x140166510 | `()` | `execute_presentation_render_graph`：`final_post`(0x14090d000)、`resolve_to_swapchain`(0x14090d850)、`ui_pass`、`ui_2d_pass`(0x1409c4470)、`mei_pass` 等（取证只到 findings 级，未逐入口编号） | `GraphExecutionRecorder::execute` 0x1400fb5b0 |

## render_world 的 59 个调用点（按调用顺序）

| # | pass | 地址 | 类型 | 来源（PDB 名） | 证据里出现的备注/API |
|---|---|---|---|---|---|
| 1 | upload_world_raster#0 | 0x1403d8bd0 | compute | country_core::systems::render_loop::compute_passes::upload_world_raster::upload_raster_staging_to_texture | upload CPU raster staging to texture |
| 2 | upload_world_raster#1 | 0x1403da050 | compute | upload_world_raster::upload_raster_staging_to_texture |  |
| 3 | upload_world_raster#2 | 0x1403da570 | compute | upload_world_raster::upload_raster_staging_to_texture |  |
| 4 | upload_world_raster#3 | 0x1403d90f0 | compute | upload_world_raster::upload_raster_staging_to_texture |  |
| 5 | upload_world_raster#4 | 0x1403d9610 | compute | upload_world_raster::upload_raster_staging_to_texture |  |
| 6 | upload_world_raster#5 | 0x1403d9b30 | compute | upload_world_raster::upload_raster_staging_to_texture |  |
| 7 | water_rasterize_from_cpu | 0x140a12680 | compute | compute_water::water_rasterize_from_cpu |  |
| 8 | rasterize_terrain_stroke | 0x1408c7120 | compute | rasterize_terrain_stroke::rasterize_terrain_stroke |  |
| 9 | displace_rocky_terrain | 0x140a3ee80 | compute | displace_rocky_terrain::displace_rocky_terrain |  |
| 10 | read_terrain_data | 0x14098c680 | compute | read_terrain_data::read_terrain_data |  |
| 11 | mouse_read_terrain_stroke_index | 0x1409768b0 | compute | mouse_read_terrain_stroke_index::mouse_read_terrain_stroke_index |  |
| 12 | rasterize_foliage_exclusion_mask | 0x1409cb7b0 | draw | draw_passes::foliage_exclusion_mask::rasterize_foliage_exclusion_mask |  |
| 13 | highlight_active_terrain_stroke | 0x140a58450 | compute | highlight_active_terrain_stroke::highlight_active_terrain_stroke |  |
| 14 | object_grass_push_pass | 0x1409daaa0 | draw | draw_passes::object_grass_push::object_grass_push_pass |  |
| 15 | compute_path_masks | 0x1409014e0 | compute | compute_path_masks::compute_path_masks |  |
| 16 | rasterize_buildings | 0x140a32770 | compute | rasterize_buildings::rasterize_buildings |  |
| 17 | garden_rasterize_from_cpu | 0x140a22140 | compute | compute_garden::garden_rasterize_from_cpu |  |
| 18 | water_spawn_vegetation | 0x140a13970 | compute | compute_water::water_spawn_vegetation |  |
| 19 | spawn_vegetation_around_buildings | 0x1408e4cc0 | compute | spawn_vegetation_around_buildings::spawn_vegetation_around_buildings |  |
| 20 | read_bush_positions_for_cpu | 0x140866540 | compute | read_bush_positions_for_cpu::read_bush_positions_for_cpu |  |
| 21 | garden_spawn_vegetation | 0x140a22b80 | compute | compute_garden::garden_spawn_vegetation |  |
| 22 | path_pebbles_compute | 0x140958f70 | compute | path_pebbles::path_pebbles_compute |  |
| 23 | leaf_particles_compute | 0x1408f4d00 | compute | leaf_particles::leaf_particles_compute |  |
| 24 | roof_tiles_gravity_compute | 0x140a29b10 | compute | roof_tiles_gravity::roof_tiles_gravity_compute |  |
| 25 | stairs | 0x140a68a50 | compute | stairs::stairs |  |
| 26 | generate_wall_draw_lists | 0x14098aaf0 | compute | gpu_generate_draw_lists::generate_wall_draw_lists | round1 wall draw lists |
| 27 | gpu_generate_draw_lists#1 | 0x140987dd0 | compute | gpu_generate_draw_lists::gpu_generate_draw_lists | third arg 0 = first round; creates gpu-only buffers, clears draw lists, cull_and_bucket |
| 28 | shadow_pass | 0x1409cc270 | draw | draw_passes::shadow_pass::shadow_pass | draws shadow map (callees: render/bind_common/get_rg_texture/extent_2d) |
| 29 | calc_terrain_lighting_texture | 0x140a51440 | compute | terrain_lighting_texture::calc_terrain_lighting_texture |  |
| 30 | main_pass | 0x140833bc0 | draw | draw_passes::main_pass::main_pass | main depth/albedo/velocity draw; two create_raw_resource (depth+hzb) precede it |
| 31 | generate_hzb | 0x1409286f0 | compute | generate_hzb::generate_hzb | bind_individual_mips + write_rg_storage_buffer + get_or_create_temporal; all_mip_levels |
| 32 | generate_wall_draw_lists_pass2 | 0x14098b2a0 | compute | gpu_generate_draw_lists::generate_wall_draw_lists_pass2 | CONDITIONAL: global *0x14362c0d8 == 0 |
| 33 | gpu_generate_draw_lists#2 | 0x140987dd0 | compute | gpu_generate_draw_lists::gpu_generate_draw_lists | CONDITIONAL: same global; third arg != 0 = second round (post-HZB) |
| 34 | grass_compute | 0x140a60280 | compute | grass::grass_compute | dispatch_indirect_raw; writes DrawIndexedIndirectBuffer/DrawIndirectBuffer/AtomicCounterBuffer; inclusive_prefix_scan_u32_1m |
| 35 | main_pass_oc2 | 0x140835a40 | draw | draw_passes::main_pass::main_pass_oc2 | second main pass after HZB |
| 36 | run_visibility_queries | 0x140a2f2e0 | compute | visibility_queries::run_visibility_queries |  |
| 37 | calculate_dof_autofocus | 0x140a17880 | compute | dof_autofocus::calculate_dof_autofocus |  |
| 38 | mouse_depth_compute | 0x140a01620 | compute | mouse_depth_value::mouse_depth_compute |  |
| 39 | late_forward_and_deferred_pass | 0x1408373b0 | draw | draw_passes::main_pass::late_forward_and_deferred_pass | lighting pass (bind_common+render) |
| 40 | extract_half_res_depth_normals | 0x1408f6180 | compute | half_res::extract_half_res_depth_normals |  |
| 41 | calculate_reprojection_map | 0x140a0d190 | compute | reprojection::calculate_reprojection_map |  |
| 42 | calculate_ssgi | 0x1408743b0 | compute | ssgi::calculate_ssgi |  |
| 43 | ray_trace_gi | 0x140a01cf0 | compute | ray_trace_gi::ray_trace_gi | get_or_create_temporal_pair; bind_ray_tracing(ComputePass); div_up_extent |
| 44 | eval_gi_screen_radiance | 0x140a44cd0 | compute | eval_gi_screen_radiance::eval_gi_screen_radiance |  |
| 45 | bin_lights | 0x1409e7770 | compute | bin_lights::bin_lights |  |
| 46 | restir_sample_direct_lights | 0x140a6fec0 | compute | restir_di::restir_sample_direct_lights |  |
| 47 | apply_screen_shadows | 0x140a39e70 | compute | apply_screen_shadows::apply_screen_shadows |  |
| 48 | deferred_raster | 0x1409d20b0 | draw | draw_passes::deferred_raster::deferred_raster | draw_no_vao; bind_color_library; reads LanternSsbo+SpatialResolveOffsetsSsbo; bind_ray_tracing(DrawBuilder) |
| 49 | refraction_copy_pass | 0x1409bf320 | draw | draw_passes::transparency_pass::refraction_copy_pass | CONDITIONAL: global *0x14362c098 == 0 |
| 50 | water_surface_pass OR ice_surface_pass | 0x1409bf800 / 0x1409c08d0 | draw+compute | water_surface_pass / ice_surface_pass | CONDITIONAL dispatch by is_winter flag (0x140ae8320): non-winter water_surface_pass, winter ice_surface_pass; both use dispatch_indirect_raw + bind_ray_tracing |
| 51 | pre_taa_transparency_pass | 0x1409c3a80 | draw | transparency_pass::pre_taa_transparency_pass |  |
| 52 | compute_shadow_mask_mean | 0x140a3a6f0 | compute | apply_screen_shadows::compute_shadow_mask_mean |  |
| 53 | apply_volumetrics | 0x1408a44c0 | compute | apply_volumetrics::apply_volumetrics |  |
| 54 | dof_prepare | 0x140a5a020 | compute | dof::dof_prepare | reads DofAutofocusSsbo |
| 55 | generate_mips | 0x1400fb890 | compute | generate_mips::generate_mips | CONDITIONAL: config byte param6[0xa9 + (res_type==0x13)] != 0 |
| 56 | calc_dof | 0x140a5a570 | compute | dof::calc_dof | CONDITIONAL: same byte; uses half_res |
| 57 | calculate_taa OR calculate_tgsr | 0x140a3b2f0 / 0x140a3ba60 | compute | taa::calculate_taa / taa::calculate_tgsr | TAA vs TGSR choice by byte param6[0xab]: 0->TAA else TGSR (tgsr uses min_subgroup_size + get_or_create_temporal_pair) |
| 58 | post_taa_transparency_pass | 0x1409c40b0 | draw | transparency_pass::post_taa_transparency_pass |  |
| 59 | render_shader_refactoring | 0x14098f9f0 | unknown | shader_refactoring::render_shader_refactoring | callees only RwLock read (read_contended/wake_writer_or_readers) + unwrap; role unknown |

## 与 RenderPassApi 的对应（已读部分，来自 g3/g7 取证）

* 所有 draw pass 走「`RasterPass` 登记（0x140867970）→ 逐条 0x80 字节绘制记录 → `DrawBuilder::bind_common`（0x14090b460）→ `RasterPass::render`（0x140868370）」，
  体内不写死访问码，访问码由绑定器第 4 参隐式声明（`0x1d`/`0x19`/`0x32`/`0x1e`/`0x24`/`0x37`）。
* compute pass 走 `ComputePass::new`（0x1416032a0）+ `read_rg_storage_buffer`（0x1a）/`write_rg_storage_buffer`（0x37）+ dispatch（`dispatch_indirect_raw` 等）。
* `main_pass` 登记 6 个 RasterPass、`main_pass_oc2` 5 个、`late_forward_and_deferred_pass` 2 个、`deferred_raster` 1 个；`shadow_pass` 3 级联。
* 逐 pass 具体调用了 `RenderPassApi` 的哪些命令（draw/dispatch/push_constants/fill/clear/copy）在这些回调体内，取证只覆盖到 pass 名/绑定层，
  **未逐回调读出**——需要对每个闭包 `FnOnce(u64)->bool` 的体做反编译（体量大，本轮未做）。
