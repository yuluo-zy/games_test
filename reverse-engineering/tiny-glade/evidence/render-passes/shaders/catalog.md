# Tiny Glade 原作 GPU 侧 pass 清单（来自 404 个发行编译 shader）

研究日期：2026-10-13。数据源：`/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/compiled-shaders/`（404 个 `.bin` SPIR-V）。
本文件只做 shader 静态证据盘点，不运行游戏、不恢复 HLSL 算法。机读对应物为同目录 `catalog.json`；
解析方法见 `method.md`；〈500 词总结见 `summary.md`。

图例：**Observation** = 直接来自文件名/SPIR-V 声明；**Inference** = 由命名、绑定、能力组合出的工作模型；
**Unknown** = 需要 PDB/机器码/运行证据确认。除注明外，本节数字全部是 Observation。

## 总体（Observation）

- 404 个文件 = 234 raster + 170 compute。唯一 pass 数 260（117 raster + 143 compute），35 个分析族。
- 每个 raster `.bin` 恰好含 **2 个 SPIR-V 模块**（`Vertex:vs_main` + `Fragment:ps_main`）；每个 compute `.bin` 含 **1 个模块**（`GLCompute:cs_main`，少数如 `sample1/sample4/r16f/rgba16f/cs_pass1/cs_pass2/cs_halfres_all`）。638 个模块。
- 命名编码：`<pass>.<cs|raster>.hlsl_<entry>[_<variant>].bin`。raster 的 `entry` 槽实际是 16 位 hex HLSL 源哈希（**全部** raster 恰有两个值 `b903e43ffb3da915` 和 `f0adff76827e73ff`）；compute 的 variant 是 `0`（单变体）或 16 位 hex（多变体）。
- 描述符约定（Observation）：`frame_constants` UBO 固定 `set1/binding0`（`_wall_generate_draw_lists`/`_roof_tiles_gravity` 额外有 `set1/binding1` 的 `frame_constants_storage_buffer` SSBO）；sampler 位于 `set0/binding0..12`；`$Globals` UBO 多在 `set0/binding13`；texture/SSBO 从 `binding14` 起。
- 638 个模块的 capabilities **没有任何 RayTracing/RayQuery**。带 `PhysicalStorageBufferAddresses` 的是 15 个模块：nani cull/bucket（8 类）×2 变体、`_nani_plaster/_nani_stairs_floor` 顶点、`_terrain_editing_rasterize_terrain_stroke`。Inference：GI 与水的“RT”走软件 BVH 与 GPU 指针，非硬件光追（与 `findings.md` 的 `rt_bl_bvh/rt_triangles` 推断一致）。

## 变体结构（Observation，重要）

- 全部 117 个 raster pass 恰有 2 个变体文件；138 个两变体 pass 中，**97 个**两变体绑定集是严格子集关系、**35 个**绑定完全一致、**0 个**混合交叉。其中 87 个为 `b903e43ffb3da915` ⊃ `f0adff76827e73ff`（b903 是“全”变体），10 个反向（全部是 nani cull 的 `8a7d2f81e72c8eeb` 带 `hzb_tex` ⊃ `d8520a92ddeb65ab`，以及 `_light_binning_bin_lights_to_cells`）。
- 例证：`_nani_uber` b903 片段含 `bindless_textures_dyn@2.0`，f0ad 没有；`_final_post` b903 含 `grading_lut_tex/icc_lut/tony_mc_mapface_lut/blur_pyramid_tex/shadow_mask_mean`，f0ad 只剩 `input_tex`；`_billboard_forest`/`_grass`/`_ui_2d` 等 b903 多出 `color_library/frame_constants/sampler_llc/ui_texture` 等。
- Inference：这些 16 位 hex 变体对对应 `shader-variants` 的**编译期特性变体**（如 cull 是否读 HZB、是否用 bindless 纹理、后处理是否启用色调映射/LUT），不是两条并列渲染路径。**Unknown**：每个变体对应的具体特性 flag 与绑定选择规则。

## 可选 / 条件 pass 族（Inference，勿过度声明）

shader 集合同时含 `_taa`、`_tgsr`（含 `_tgsr_reproject/_tgsr_blur_lum`）与 `_fxaa`；`findings.md` 帧分支显示 TAA 与 TGSR 是选择关系。`_blit_resolve_msaa_to_swapchain` 存在意味着某档设置下主目标可能 MSAA。DoF 有 `_dof_fused` 与 `_dof_halfres/_dof_fullres` 两套实现并存。多个 16 位变体只说明存在多份变体编译，不等于每帧都运行。

---

# 逐族清单

## 1. nani_cull_bucket — GPU-driven 剔除/分桶（compute，24 passes）
观察到的完整链：`_nani_clear_draw_lists/_nani_clear_histogram/_nani_clear_subtotals` → `_nani_cull_and_bucket{,_bush,_curved_halftimber,_flowers,_plaster,_roof_tile,_smoke,_stairs_floor,_wooden_plank}`（LocalSize 64×1×1）→ `_nani_histogram_prefix_sum/_nani_subtotal_prefix_sum/_nani_group_draw_cmd_prefix_sum` → `_nani_fill_unsorted_instance_list/_nani_fill_unsorted_instance_indirection/_nani_prefix_sum_unsorted_instances_per_atlas_entry/_nani_fill_sorted_instance_list` → `_nani_prepare_unsorted_draw_lists` → `_nani_generate_draw_lists`（LocalSize 32）→ `_nani_allocate_draw_commands`/`_nani_generate_dispatch_args` → `_nani_draw_indexed_indirect_to_dispatch`（LocalSize 1×1×1，`draw_buffer`+`args_buffer`）。
强证据：cull 绑定 `atlas_part_info/filtered_instance_{buffer,allocator,meta}/histogram/prev+next_visibility_bitmask/draw_list_info/hzb_tex`，capability `PhysicalStorageBufferAddresses`。`8a7d2f81e72c8eeb` 变体带 `hzb_tex`、`d8520a92ddeb65ab` 不带（对应 HZB 前/后两轮剔除，见 `findings.md`）。**Unknown**：LOD 选择规则与分桶算法。

## 2. wall_draw_lists（compute，2）+ wall_raster（raster，2）
`_wall_generate_draw_lists.cs.hlsl_cs_pass1/cs_pass2`：LocalSize 64；pass1 无 `hzb_tex`、pass2 有 `hzb_tex@0.19`，绑定 `brick_instance_data/prev_visibility_bitmask/next_visibility_bitmask/brick_draw_list`，另有 `frame_constants_storage_buffer@1.1`。`_wall_reset_pass2_inst_counts` 清 pass2 计数。`_wall_wall_brick_lod0/_wall_wall_brick_lod1` raster（LOD0/1 墙砖，片段读 `brick_colors/roof_colors/color_library`）。
Inference：墙走独立两轮 draw-list（HZB 前/后）再间接绘制；`lod0/lod1` 是墙砖 LOD 变体。

## 3. nani_raster_main — 主场景栅格（raster，23 passes）
包含 `_nani_uber`、`_nani_solid_albedo_map{,_emissive}`、`_nani_solid_vertex_color{,_bricks,_window,_window_glass}`、`_nani_velocity_only`、`_nani_instanced_roof/_nani_roof_floor/_nani_wooden_plank/_nani_stairs_floor/_nani_curved_halftimber/_nani_plaster`、`_nani_plant/_nani_flower/_nani_preplaced_flower/_nani_bush/_nani_smoke/_nani_cloth/_nani_balcony_door/_nani_gothic_window_bricks/_nani_lantern_emissive`。
强证据：`_nani_uber` 顶点读 `instance_indirection_buffer@0.14` + `nani_instance_data@0.15`（SSBO,BufferBlock），片段 b903 变体读 `bindless_textures_dyn@2.0`（UniformConstant）。Inference：GPU-driven 实例化 + bindless 纹理的主不透明/透明材质 pass；`_nani_velocity_only` 是纯运动矢量输出。`_nani_curved_halftimber` 顶点也带 `PhysicalStorageBufferAddresses`。

## 4. main_raster_simple — 独立单体栅格（raster，7）
`_solid_vertex_color{,_albedo_map,_boat,_garden_stone,_waterwheel,_wooden_gate}`、`_vertex_color`。简单材质（`$Globals` + `frame_constants`，albedo_map 变体加 `sampler_llr+albedo_map`）。Inference：非 nani 路径的通用 mesh（如小船、水车等道具）。

## 5. deferred_raster — 延迟光照（raster，4）
`_deferred_canopy_light_sun_sky/_deferred_grass_light_sun_sky/_deferred_plant_light_sun_sky/_deferred_solid_light_sun_sky`。强证据（canopy）：读 `gbuffer_tex/depth_tex/half_depth_tex/blue_noise/shadow_map/color_library/spatial_resolve_offsets/lantern_buf/indirect_radiance_tex/rdi_reservoir_tex/light_indirection_grid@0.36/grid_light_indices@0.37`；grass 版还读 `ssgi_tex/screen_shadows_tex/brdf_fg_lut`。Inference：按几何类别区分的太阳+天空延迟光照，叠加 ReSTIR/间接辐射、SSGI、屏幕阴影、灯光 binning 网格。配合 `_extract_half_res_depth_normals` 读 `gbuffer_input_tex+stencil_tex` → 存在 G-buffer 与 stencil。

## 6. shadow（compute 2 + raster 1）
`_apply_screen_shadows`（8×8，读 `gbuffer_tex/depth_tex/history_tex/reprojection_tex/shadow_map`，写 `output_tex`+`shadow_mask_histogram`）、`_shadow_mask_mean`（16×16，生成均值 shadow mask）、`_mei_shadow` raster（全屏，读 `blue_noise_texture+input_tex`）。
Inference：屏幕空间阴影（历史/重投影复用）+ shadow mask 后处理 + 一个“mei”阴影全屏 pass。**Unknown**：`_mei_shadow` 具体语义（命名可能与植被类型有关）。

## 7. hzb（compute，1）
`_generate_hzb`：LocalSize 256×1×1，`spdConstants`(PushConstant)+`imgSrc@0.0/imgDst@0.1/imgDst6@0.2/spdGlobalAtomic@0.3`，caps `GroupNonUniform+GroupNonUniformShuffle`。**这是 FidelityFX SPD 单趟降采样签名（Observation）**。`_generate_mips`（blur_mips 族）签名相同（`imgSrc/imgDst/imgDst6/spdGlobalAtomic`）→ Inference：同一 SPD 内核用于 HZB 与通用 mip。

## 8. depth_normals + reprojection（compute，2+2）
`_extract_half_res_depth_normals`（8×8）：读 `stencil_tex/depth_input_tex/gbuffer_input_tex`，写 `depth_output_tex/normals_output_tex`。`_copy_depth_to_r`：depth→R 拷贝。`_calculate_reprojection_map`（8×8，见 shader-reflection.json 逐字复用）：`depth_tex/prev_depth_tex/velocity_tex→output_tex`。`_rdi_temporal`：读 `lantern_buf/reprojection_tex/half_depth/half_view_normal/light_indirection_grid/grid_light_indices`，写 `rdi_temporal_history/out`。Inference：半分辨率 depth/normal 缓冲 + 速度重投影图是 GI/TAA/TGSR/屏幕阴影共用的跨帧输入。

## 9. gi — 全局光照（compute，5）+ ssgi（compute，3）
GI 链：`_gi_trace_gi_candidates`（软件 BVH：`rt_bl_bvh/rt_triangles/rt_triangle_meta`，读 terrain 各 mask、`shadow_map/blue_noise/taa_history/reprojection/half_depth/half_view_normal`，写 `candidate_radiance/hit`）→ `_gi_update_gi_radiance`（写 `restir_radiance/ray_dir/reservoir_out`）→ `_gi_eval_screen_radiance`（读 3 张 restir + `ssgi_tex`，写 `output_tex`）→ `_gi_reproject_screen_radiance`（`history_tex+reprojection_tex+temporal_stats`）→ `_gi_blur_screen_radiance`（`half_depth/half_view_normal/ssgi_tex/temporal_stats_out`）。
SSGI：`_ssgi_ssgi`（8×8，`depth_tex/half_view_normal→output`）、`_ssgi_temporal_filter`（`input/history/reprojection→final_output/history_output`）、`_ssgi_upsample`（`ssgi_tex/depth/half_depth/half_view_normal/gbuffer_tex→output`）。
全部 8×8×1，无光追能力。**Unknown**：ReSTIR 变体细节、权重/偏差修正（见 findings.md）。

## 10. light_binning（compute，4）
`_light_binning_bin_lights_to_cells`（64，读 `lantern_buf`，写 `indirection_grid+light_indices`）、`_cell_prefix_sum/_cell_subtotal_prefix_sum/_cell_prefix_merge`（256/512）。Inference：灯光（灯笼）→ 3D 网格 binning，供延迟 pass 的 `light_indirection_grid/grid_light_indices` 使用。

## 11. taa_tgsr + fsr_fxaa（compute，4+3）
`_taa`（8×8，`color/history/reprojection→history_output/output`）。`_tgsr` 4 变体（8×8 与 8×4），caps `GroupNonUniform+GroupNonUniformArithmetic+GroupNonUniformShuffle`，YCoCg 式 `reprojected_history_tex_{lum,chroma}/color_tex/blurred_luminance_tex/aux_tex`→`history_output/output/aux_output`。`_tgsr_reproject`（`history/reprojection/depth→output_lum/output_chrom`）、`_tgsr_blur_lum`（`color→output`）。`_fsr_copy_history`、`_fsr_init_reactive`（8×8），`_fxaa`。
Observation：TAA 与 TGSR 是两套独立时域 AA；FSR 仅有 history/reactive 两 compute（无完整 FSR 上采样链，可能与 TGSR 共用）。**Unknown**：TAA/TGSR/FSR 的组合与触发条件。

## 12. dof（compute，11）
`_dof_analyze`（CoC tile + `dispatch_args/tile_coords`）、`_dof_autofocus`（`autofocus_buf/focus_readback_buf`）、`_dof_prepare`（`depth+autofocus_buf→coc_out+tile_output`）、`_dof_clear_halfres`、`_dof_init_buffers`、`_dof_halfres`（`cs_halfres`/`cs_halfres_all` 各 ×2 变体）、`_dof_fullres{,_all_lod0,_all_lod1,_upsample}`、`_dof_fused`。多 16 位变体（`6f1dd5dd8a66e104`/`72a2a95d73e37edc` 贯穿 DoF）。Inference：tile-based 半分辨率 CoC 扩散 + 全分辨率应用 + 上采样 + “fused”合并版本；LOD0/1 为不同品质档。

## 13. volumetrics（compute，1）
`_apply_volumetrics`（8×8）：读 `shadow_map/depth_tex/blue_noise`，写 `color_tex`。Inference：体积光/雾。

## 14. water（compute 13 + raster 8）
生成/状态：`_water_draw_water_mask`/`_water_read_water_mask`、`_water_spawn_boats/_spawn_lilypads/_spawn_reeds/_spawn_waterwheels`、`_lut_water_noise`。
水表面：`_water_plane` raster（b903 变体读 `shadow_map/water_mask/refraction_tex/taa_history/reprojection/autofocus_buf`）、`_water_plane_refraction_copy`、`_water_snow_plane`、`_water_x_paths`、`_waterfall`、`_waterfall_refraction_copy`、`_ice_plane`、`_water_ice_cracks`。
“延迟 RT”链：`_water_calc_deferred_rt_args`（1×1）→ `_water_deferred_rt`/`_water_ice_deferred_rt`（64×1）读 **`rt_bl_bvh`/`rt_triangles`**（软件 BVH SSBO，与 GI 同名资源）→ `_water_ice_refl_march`/`_water_ice_refl_resolve`/`_water_ice_refr_resolve`。
强证据：水/冰的“deferred RT”是**软件 BVH 光线行进**（无 RayTracing 能力），对折射/反射（`refl_hit/refraction_tex`）做屏幕空间采样。**Unknown**：BVH 构建端与采样算法。

## 15. terrain + path + rocky/stairs（compute 11 / raster 3 / …）
terrain 编辑：`_terrain_editing_rasterize_terrain_stroke`（写 `img_output`，读 `stroke_indices/rocky_terrain/terrain_sdg/water_mask`）、`_terrain_editing_cache_stroke`、`_terrain_editing_calc_highlight_hill_mask`、`_read_terrain_height`、`_read_terrain_rock_mask`、`_read_terrain_stroke`；`_terrain_editing_floor` raster。terrain 派生：`_terrain_derivative`（`rocky_terrain+terrain_heightmap`）、`_terrain_lighting_texture`（读 `shadow_map+color_library+terrain_heightmap`，写 `img_output`）、`_upload_cpu_raster`（r16f/rgba16f 入口）。raycast：`_mouse_terrain_raycast`、`_read_mouse_depth_value`。overlay：`_demo_terrain_overlay`、`_timed_terrain_gizmo` raster。
path：`_paths_combine_path_exclusions`、`_paths_directional_contract`（读 `terrain_sdg/terrain_heightmap/path_mask`）、`_paths_path_mask_visual`、`_path_pebbles`（读 `terrain_heightmap/path_mask/rocky_terrain/water_mask`，写 `pebble_placement`）、`_path_pebble` raster。
rocky/stairs：`_rocky_terrain_displace_rocky_terrain`（64，读 `terrain_heightmap/rocky_terrain/path_mask/water_mask`）、`_rocky_terrain_rocky_terrain` raster（读 `vertices/triangles` SSBO + `rocky_terrain_texture`）、`_rocky_terrain_stairs_stairs{,_counter}`、`_stairs_stairs_indirect` raster。
building_rasterize：`_rasterize_buildings`（8×8，写 `out_mask/out_mask2`，读 `buildings_buffer`）、`_rasterize_buildings_animate`。
Inference：terrain 高度/岩石/路径/水 mask 是大量程序化 pass 的共享输入；构建体被光栅化为 mask 供植被排斥等使用。

## 16. vegetation_grass（compute 18 + raster 20）— 最大族
grass 生成链：`_generate_grass`（64，读 `terrain_heightmap/path_mask/rocky_terrain/water_mask/buildings_mask/garden_mask/tile_buffer/bundle_buffer/grasslet_bundle_ranges`，写 `grass_blade_buffer/lod1_commands/atomics`）→ `_grass_calc_tiles`（读 `hzb_tex`）→ `_grass_tile_bundle_count/_grass_tile_bundle_gen/_grass_bundles_prefix/_grass_tiles_prefix/_grass_calc_grasslet_bundle_ranges/_grass_calc_dispatch_args`。
garden：`_garden_marching_squares`、`_garden_spawn_*`（读 `garden_mask/path_mask/water_mask/rocky_terrain/foliage_exclusion_mask/terrain_heightmap`）、`_garden_read_*`（CPU 回读）、`_garden_exclude_other_masks`、`_garden_lowpoly_flower` raster。
billboard/leaf/ivy：`_billboard_forest/_billboard_plant`、`_clearing_{distant_billboard,instanced_background_tree,meadow_flower,meadow_plant,tree_canopy,tree_trunk}`、`_leaf_particle` raster + `_leaf_particles` compute（读 `water_noise_texture/grass_object_push`，写 `leaf_particle_buffer/emitter_buffer`）、`_ivy_*`、`_lilypad*/_reed/_stairs_foliage_exclusion`。
grass raster：`_grass/_grass_lod1/_grass_object_push`。Inference：草地/植被完全 GPU 生成与剔除（分 tile/bundle 的间接绘制），billboard 与 instanced background tree 承担远景。

## 17. roof（compute 1 + raster 6）
`_roof_tiles_gravity`（64，`instanced_data` SSBO + `frame_constants_storage_buffer@1.1`，与 `roof_tiles_gravity_compute` 帧调用对应）、`_roof_snow` raster、`_instanced_roof_beams/_instanced_roof_spire`、`_flag_roof_cloth/_flag_wall_cloth/_flag_small`（读 `border_mask/flag_palettes/flag_patterns`）。Inference：屋顶瓦片重力模拟 compute + 屋檐/旗织物。

## 18. critters_particles + sky_cloud
`_critters_firefly_vis_query` compute；`_critters_{butterfly,duck_animated,firefly,firefly_glow,nani_wren}` raster；`_rain_particles/_snow_particles/_sparkles/_splash_particle/_sheep`。`_skydome`（读 `moon_tex`）、`_cloud`（读 `cloud1..4`）、`_star`。

## 19. ui（raster，8）
`_egui`、`_ui_2d`、`_ui_2d_color_button`、`_ui_2d_ws`（world-space）、`_text`、`_custom_cursor`（`cursor_tex`）、`_dashed_line`、`_timer_circle`。Inference：egui + 自建 2D/WS UI 走同一渲染图执行层（见 rendering-framework-map）。

## 20. blit_resolve_swapchain（compute 1 + raster 4）
`_final_post` raster（b903：读 `input_tex/blur_pyramid_tex/grading_lut_tex(3D)/icc_lut(3D)/tony_mc_mapface_lut(3D)/blue_noise/shadow_mask_mean(SSBO)`；f0ad：仅 `input_tex`）→ `_blit_resolve_to_swapchain` / `_blit_resolve_msaa_to_swapchain`（MSAA！） / `_blit_blit_stretch_decode_srgb` / `_resolve_to_swapchain` compute（8×8，`input_tex→output_tex`）。Inference：色调映射/色彩分级/ICC/柔光合并 → 解码 sRGB 并 resolve（含 MSAA 档）到交换链。

## 21. painterly_vermeer（compute 6 + raster 5）
`_painterly_acquire_target`/`_vermeer_acquire_target`（读 `coc/color/depth`）、`_painterly_generate_strokes`/`_vermeer_generate_strokes`（32×1，写 `strokes_buf`；vermeer 两变体差异为有无 `background_tex`）、`_painterly_convolve_target`/`_vermeer_convolve_target`、`_painterly_raster_strokes`/`_vermeer_raster_strokes`/`_vermeer_combine`/`_painterly_debug_target`、`_lut_gallery_thumbnail`（读 `icon_tex+grading_lut_tex`）。Inference：程序化“painterly/vermeer”风格化绘制链（笔触生成→光栅→合并），用于缩略图/画廊。

## 22. session_ui（raster，8）
`_loading_screen`、`_logo_animation/_logo_branches_animation/_logo_overlay`、`_session_selection_{background,new_glade_thumbnail,thumbnail,thumbnail_painterly}`。Inference：启动/会话选择/Logo 全屏界面。

## 23. blur_mips / prefix_scan / lut / debug / screenshot（compute）
- blur_mips（9）：`_blur/_blur_downsample`（`cs_main` 与 `cs_main_multisampled` 两入口）、`_blur_pyramid_blur/_blur_pyramid_rev_blur`、`_gauss_blur{,_building_mask,_mask,_r16f}`、`_generate_mips`（SPD 签名）。
- prefix_scan（3）：`_prefix_scan_inclusive_prefix_scan{,_merge,_subtotals}`（512×1）——通用 GPU 前缀和原语，供 nani/grass/light binning 复用。
- lut：`_lut_brdf_fg`（BRDF F0/FG 查找表生成）。
- debug_dev：`_fill_buffer`（`__alias_output`）、`_normal_debug`、`_texture_test`、`_shader_refactoring.cs.hlsl_{capture,compare}`（开发期差分）、`_primary_rt_debug`（软件 BVH 调试）。
- screenshot：`_screenshot_capture_screenshot_{buffer,texture}.cs.hlsl_{sample1,sample4}`、`_capture_panic_screenshot`。

---

# 各族的“最强证据”速览（Observation）

| 族 | 最强证据 |
|---|---|
| nani cull/bucket | 64×1×1、`PhysicalStorageBufferAddresses`、visibility bitmask + histogram + `hzb_tex` 有/无两变体 |
| nani raster | VS 读 `instance_indirection_buffer/nani_instance_data` SSBO；uber b903 读 `bindless_textures_dyn@2.0` |
| deferred | 读 G-buffer + `light_indirection_grid/grid_light_indices/rdi_reservoir_tex/ssgi_tex/screen_shadows_tex` |
| shadow | `_apply_screen_shadows` 读 `shadow_map/history/reprojection` 写 histogram |
| hzb | SPD 签名（`spdConstants/imgSrc/imgDst/imgDst6/spdGlobalAtomic`，256×1） |
| GI/水 | 无 RayTracing 能力；`rt_bl_bvh/rt_triangles` SSBO 软件 BVH |
| TAA/TGSR | TGSR 用 `GroupNonUniform*` + YCoCg `lum/chroma`，4 变体；TAA 普通 8×8 |
| final_post | `grading_lut/icc_lut/tony_mc_mapface_lut`(3D) + `shadow_mask_mean` + blur pyramid |
| grass | 完整 tile/bundle/prefix 生成链，读 `hzb_tex`，写 `grass_blade_buffer/lod1_commands` |
| roof | `_roof_tiles_gravity`（64）+ `frame_constants_storage_buffer@1.1` |

# 主要 Unknown

1. 两 16 位 hex 变体对（`b903…`/`f0ad…`）对应的确切特性 flag 与选择规则。
2. TAA/TGSR/FSR/FXAA 的组合方式与每帧实际运行哪个。
3. 软件 BVH（`rt_bl_bvh/rt_triangles`）的构建端、遍历/采样算法及与 GI/水的共用方式。
4. `_mei_shadow`、`_final_post` 两种变体的语义差异。
5. 每个 pass 实际绑定的 descriptor 句柄、format/extent 与执行顺序（需 CPU 侧 PDB/机器码或运行时证据）。
