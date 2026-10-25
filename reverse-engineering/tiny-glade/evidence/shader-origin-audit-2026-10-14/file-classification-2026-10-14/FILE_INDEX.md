# Shader 逐文件来源清单

这里的“游戏专属”指原作游戏的数据、规则、参数和接入实现；不是每行代码/数学算法都从零原创的证明。确认上游的条目也只对说明的核心段负责，不把整个消费者 shader 判成上游源码。

原作 421 个记录路径合并为 417 个规范化路径（去除 eval/../ 等别名）；当前恢复目录 415 个 HLSL。原始证据来自发行包，完整性验证和详细分析见 [README.md](README.md)。

分类是来源证据等级与实现范围，不是开源代码行数占比。

## 已确认包含上游核心（6 个路径）

| 原作源路径 | 来源/范围 | 置信度 |
|---|---|---|
| `shaders/blur_downsample.cs.hlsl` | Kajiya blur.hlsl 同源实现，游戏资源/入口及MSAA适配 | high |
| `shaders/inc/fidelityfx/ffx_a.h` | AMD FidelityFX SPD include source family | high_for_spd_family |
| `shaders/inc/fidelityfx/ffx_spd.h` | AMD FidelityFX SPD include source family | high_for_spd_family |
| `shaders/inc/reservoir.hlsl` | Kajiya reservoir helper source lineage | high |
| `shaders/inc/rt.hlsl` | GPU-Raytracer BVH8 traversal port plus game integration | high_for_bvh8_core |
| `shaders/ssgi/XeGTAO.hlsl.inc` | Intel XeGTAO 主算法源码核心；游戏 SSGI 调用与资源绑定另计 | high |

## 上游同源并经游戏改写（4 个路径）

| 原作源路径 | 来源/范围 | 置信度 |
|---|---|---|
| `shaders/blur_pyramid/blur.cs.hlsl` | Kajiya共享内存两遍blur派生 | high |
| `shaders/calculate_reprojection_map.cs.hlsl` | Kajiya reprojection-map lineage with changed validity handling | high_for_adapted_core |
| `shaders/inc/frame_constants.hlsl` | Kajiya shared ViewConstants/FrameConstants lineage plus Tiny Glade extensions | high_for_layout_lineage |
| `shaders/ssgi/temporal_filter.cs.hlsl` | Kajiya SSGI temporal-filter design adapted for Tiny Glade | high_for_adapted_core |

## 疑似上游派生（4 个路径）

| 原作源路径 | 来源/范围 | 置信度 |
|---|---|---|
| `shaders/fxaa.inc.hlsl` | NVIDIA Timothy Lottes FXAA 3.11 PC Quality 源码家族 | high |
| `shaders/inc/color/helmholtz_kohlrausch.hlsl` | Likely Kajiya color helper source family | medium |
| `shaders/inc/unjitter_taa.hlsl` | Likely Kajiya unjitter helper with changes | medium_high |
| `shaders/ssgi/upsample.cs.hlsl` | Kajiya ssgi/upsample.hlsl 候选改写 | medium |

## 游戏专属数据与规则（174 个路径）

| 原作源路径 | 来源/范围 | 置信度 |
|---|---|---|
| `shaders/billboard_forest.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/billboard_plant.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/clearing/canopy_vert.inc.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/clearing/inc/forest_floor.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/clearing/inc/instanced_placement.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/clearing/instanced_background_tree.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/clearing/meadow_flower.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/clearing/meadow_plant.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/clearing/tree_canopy.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/clearing/tree_trunk.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/critters/butterfly.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/critters/duck_animated.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/critters/firefly.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/critters/firefly_glow.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/critters/firefly_vis_query.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/critters/nani_wren.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/dof/analyze.cs.hlsl` | 游戏多分辨率DOF/自动对焦方案，非Kajiya公开gather原样拷贝 | medium |
| `shaders/dof/autofocus.cs.hlsl` | 游戏多分辨率DOF/自动对焦方案，非Kajiya公开gather原样拷贝 | medium |
| `shaders/dof/clear_halfres.cs.hlsl` | 游戏多分辨率DOF/自动对焦方案，非Kajiya公开gather原样拷贝 | medium |
| `shaders/dof/fullres.cs.hlsl` | 游戏多分辨率DOF/自动对焦方案，非Kajiya公开gather原样拷贝 | medium |
| `shaders/dof/fullres_all_lod0.cs.hlsl` | 游戏多分辨率DOF/自动对焦方案，非Kajiya公开gather原样拷贝 | medium |
| `shaders/dof/fullres_all_lod1.cs.hlsl` | 游戏多分辨率DOF/自动对焦方案，非Kajiya公开gather原样拷贝 | medium |
| `shaders/dof/fullres_upsample.cs.hlsl` | 游戏多分辨率DOF/自动对焦方案，非Kajiya公开gather原样拷贝 | medium |
| `shaders/dof/fused.cs.hlsl` | 游戏多分辨率DOF/自动对焦方案，非Kajiya公开gather原样拷贝 | medium |
| `shaders/dof/halfres.cs.hlsl` | 游戏多分辨率DOF/自动对焦方案，非Kajiya公开gather原样拷贝 | medium |
| `shaders/dof/init_buffers.cs.hlsl` | 游戏多分辨率DOF/自动对焦方案，非Kajiya公开gather原样拷贝 | medium |
| `shaders/dof/multires.inc.hlsl` | 游戏多分辨率DOF/自动对焦方案，非Kajiya公开gather原样拷贝 | medium |
| `shaders/dof/prepare.cs.hlsl` | 游戏多分辨率DOF/自动对焦方案，非Kajiya公开gather原样拷贝 | medium |
| `shaders/dof/tiles.inc.hlsl` | 游戏多分辨率DOF/自动对焦方案，非Kajiya公开gather原样拷贝 | medium |
| `shaders/dof/upsample_halfres.inc.hlsl` | 游戏多分辨率DOF/自动对焦方案，非Kajiya公开gather原样拷贝 | medium |
| `shaders/flag_roof_cloth.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/flag_small.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/flag_wall_cloth.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/garden/exclude_other_masks.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/garden/marching_squares.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/garden/placement_utils.inc.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/garden/read_garden_mask.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/garden/spawn_bushes_buildings.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/garden/spawn_bushes_garden.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/garden/spawn_flowers_buildings.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/garden/spawn_flowers_garden.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/garden/spawn_small_flowers_garden.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/garden_lowpoly_flower.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/generate_grass.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/grass.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/grass/calc_dispatch_args.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/grass/calc_grasslet_bundle_ranges.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/grass/calc_tiles.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/grass/tile_bundle_count.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/grass/tile_bundle_gen.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/inc/foliage_exclusion.hlsl` | Tiny Glade game domain data or integration | high_for_named_data_layout_or_parameters |
| `shaders/inc/grass_blade_data.hlsl` | Tiny Glade game domain data or integration | high_for_named_data_layout_or_parameters |
| `shaders/inc/highlight_color.hlsl` | Tiny Glade game domain data or integration | medium_for_domain_scope |
| `shaders/inc/lanterns.hlsl` | Tiny Glade game domain data or integration | high_for_named_data_layout_or_parameters |
| `shaders/inc/material/canopy_material.inc.hlsl` | Tiny Glade game domain data or integration | medium_for_domain_scope |
| `shaders/inc/material/eval/canopy_material.inc.hlsl` | Tiny Glade game domain data or integration | medium_for_domain_scope |
| `shaders/inc/material/eval/grass_material.inc.hlsl` | Tiny Glade game domain data or integration | medium_for_domain_scope |
| `shaders/inc/material/eval/plant_material.inc.hlsl` | Tiny Glade game domain data or integration | medium_for_domain_scope |
| `shaders/inc/material/eval/solid_material.inc.hlsl` | Tiny Glade game domain data or integration | medium_for_domain_scope |
| `shaders/inc/material/grass_material.inc.hlsl` | Tiny Glade game domain data or integration | medium_for_domain_scope |
| `shaders/inc/material/plant_material.inc.hlsl` | Tiny Glade game domain data or integration | medium_for_domain_scope |
| `shaders/inc/material/solid_material.inc.hlsl` | Tiny Glade game domain data or integration | medium_for_domain_scope |
| `shaders/inc/material_eval.inc.hlsl` | Tiny Glade game domain data or integration | medium_for_domain_scope |
| `shaders/inc/rocky_terrain.inc.hlsl` | Tiny Glade game domain data or integration | medium_for_domain_scope |
| `shaders/inc/roof_colors_include.hlsl` | Tiny Glade game domain data or integration | high_for_named_data_layout_or_parameters |
| `shaders/inc/terrain_heightmap.inc.hlsl` | Tiny Glade game domain data or integration | high_for_named_data_layout_or_parameters |
| `shaders/inc/wall_color_customization.hlsl` | Tiny Glade game domain data or integration | high_for_named_data_layout_or_parameters |
| `shaders/inc/wall_color_ids.hlsl` | Tiny Glade game domain data or integration | high_for_named_data_layout_or_parameters |
| `shaders/inc/wall_distortion.hlsl` | Tiny Glade game domain data or integration | medium_for_domain_scope |
| `shaders/inc/wind.hlsl` | Tiny Glade game domain data or integration | high_for_named_data_layout_or_parameters |
| `shaders/inc/window_color_customization.hlsl` | Tiny Glade game domain data or integration | high_for_named_data_layout_or_parameters |
| `shaders/instanced_roof_beams.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/instanced_roof_spire.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/ivy/instanced_ivy_branch.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/ivy/instanced_ivy_leaf.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/ivy/ivy.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/leaf_particle.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/leaf_particles.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/lilypad.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/lilypad_flower.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/mouse_terrain_raycast.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani/allocate_draw_commands.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani/cull_and_bucket.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani/cull_and_bucket.inc.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani/cull_and_bucket_bush.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani/cull_and_bucket_flowers.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani/cull_and_bucket_roof_tile.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani/cull_and_bucket_wooden_plank.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani/fill_sorted_instance_list.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani/fill_unsorted_instance_indirection.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani/fill_unsorted_instance_list.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani/generate_dispatch_args.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani/generate_draw_lists.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani/inc/cull_and_bucket_owned_custom.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani/prepare_unsorted_draw_lists.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani_balcony_door.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani_bush.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani_cloth.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani_curved_halftimber.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani_flower.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani_gothic_window_bricks.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani_instanced_roof.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani_lantern_emissive.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani_plant.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani_plaster.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani_preplaced_flower.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani_roof_floor.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani_smoke.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani_solid_albedo_map.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani_solid_albedo_map_emissive.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani_solid_vertex_color.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani_solid_vertex_color_bricks.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani_solid_vertex_color_window.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani_solid_vertex_color_window_glass.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani_stairs_floor.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani_uber.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani_velocity_only.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani_wooden_plank.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/painterly/generate_strokes.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/painterly/stroke.inc.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/path_pebble.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/path_pebbles.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/paths/combine_path_exclusions.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/paths/directional_contract.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/paths/path_mask_sampler.inc.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/paths/path_mask_visual.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/rasterize_buildings.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/read_terrain_stroke.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/reed.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/rocky_terrain/displace_rocky_terrain.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/rocky_terrain/rocky_terrain.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/rocky_terrain/stairs/place_stairs.inc.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/rocky_terrain/stairs/stairs.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/rocky_terrain/stairs/stairs_counter.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/rocky_terrain/stairs/stairs_indirect.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/roof/dormer_hole.inc.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/roof/roof_tile.inc.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/roof/stone_floor.inc.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/roof/wooden_floor.inc.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/roof_snow.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/roof_tiles_gravity.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/sheep.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/snow_particles.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/solid_vertex_color_garden_stone.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/solid_vertex_color_wooden_gate.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/sparkles.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/taa.cs.hlsl` | 游戏TAA实现；共享Kajiya工具并不证明TAA主体复制 | medium |
| `shaders/terrain_derivative.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/terrain_editing/cache_stroke.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/terrain_editing/calc_highlight_hill_mask.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/terrain_editing/floor.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/terrain_editing/inc_highlight_active_terrain_chunk.inc.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/terrain_editing/path_cobblestone_surface.inc.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/terrain_editing/rasterize_terrain_stroke.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/terrain_editing/read_terrain_height.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/terrain_lighting_texture.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/tgsr.cs.hlsl` | Tiny Glade TGSR；官方称为自有超分方案 | high |
| `shaders/tgsr_blur_lum.cs.hlsl` | Tiny Glade TGSR；官方称为自有超分方案 | high |
| `shaders/tgsr_common.inc.hlsl` | Tiny Glade TGSR；官方称为自有超分方案 | high |
| `shaders/tgsr_reproject.cs.hlsl` | Tiny Glade TGSR；官方称为自有超分方案 | high |
| `shaders/timed_terrain_gizmo.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/vermeer/generate_strokes.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/vermeer/raster_strokes.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/vermeer/stroke.inc.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/wall/generate_draw_lists.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/wall/inc/brick_frag.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/wall/inc/brick_vert.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/water/draw_water_mask.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/water/spawn_boats.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/water/spawn_lilypads.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/water/spawn_reeds.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/water/water_mask_sampler.inc.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/water/waves.inc.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/wooden_plank.inc.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |

## 游戏接入或组合层（125 个路径）

| 原作源路径 | 来源/范围 | 置信度 |
|---|---|---|
| `shaders/apply_screen_shadows.cs.hlsl` | 游戏美术/光照/天气/后处理组合 | medium |
| `shaders/apply_volumetrics.cs.hlsl` | 游戏美术/光照/天气/后处理组合 | medium |
| `shaders/buildings/building_mask.inc.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | medium |
| `shaders/buildings/curved_halftimber.inc.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | low |
| `shaders/clearing/distant_billboard.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | medium |
| `shaders/clearing/inc/canopy_common.inc.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | medium |
| `shaders/cloud.raster.hlsl` | 游戏美术/光照/天气/后处理组合 | medium |
| `shaders/custom_cursor.raster.hlsl` | 游戏UI/选择界面/字形/颜色按钮资源接线 | medium |
| `shaders/deferred/canopy_light_sun_sky.raster.hlsl` | 游戏GI/材质/水冰pass组合；内含Kajiya reservoir或GPU-Raytracer BVH8等共享代码；消费者另含 Kajiya Reservoir helper | medium |
| `shaders/deferred/grass_light_sun_sky.raster.hlsl` | 游戏GI/材质/水冰pass组合；内含Kajiya reservoir或GPU-Raytracer BVH8等共享代码；消费者另含 Kajiya Reservoir helper | medium |
| `shaders/deferred/light_sun_sky.inc.hlsl` | 游戏GI/材质/水冰pass组合；内含Kajiya reservoir或GPU-Raytracer BVH8等共享代码 | medium |
| `shaders/deferred/light_sun_sky_prelude.inc.hlsl` | 游戏GI/材质/水冰pass组合；内含Kajiya reservoir或GPU-Raytracer BVH8等共享代码 | medium |
| `shaders/deferred/plant_light_sun_sky.raster.hlsl` | 游戏GI/材质/水冰pass组合；内含Kajiya reservoir或GPU-Raytracer BVH8等共享代码；消费者另含 Kajiya Reservoir helper | medium |
| `shaders/deferred/solid_light_sun_sky.raster.hlsl` | 游戏GI/材质/水冰pass组合；内含Kajiya reservoir或GPU-Raytracer BVH8等共享代码；消费者另含 Kajiya Reservoir helper | medium |
| `shaders/demo_terrain_overlay.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | medium |
| `shaders/egui.raster.hlsl` | 游戏UI/选择界面/字形/颜色按钮资源接线 | medium |
| `shaders/final_post.raster.hlsl` | 游戏美术/光照/天气/后处理组合 | medium |
| `shaders/fxaa.cs.hlsl` | 游戏FXAA compute入口接线；核心include很可能NVIDIA3.11 | high |
| `shaders/garden/defs.inc.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | low |
| `shaders/garden/read_bush_positions_for_cpu.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | medium |
| `shaders/gauss_blur_building_mask.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/generate_hzb.cs.hlsl` | FidelityFX SPD原源码+游戏归约/load/store回调；消费者另含 AMD FidelityFX SPD downsampler, AMD FidelityFX SPD portability header | high |
| `shaders/generate_mips.cs.hlsl` | FidelityFX SPD原源码+游戏归约/load/store回调；消费者另含 AMD FidelityFX SPD downsampler, AMD FidelityFX SPD portability header | high |
| `shaders/gi/blur_screen_radiance.cs.hlsl` | 游戏GI/材质/水冰pass组合；内含Kajiya reservoir或GPU-Raytracer BVH8等共享代码 | medium |
| `shaders/gi/eval_screen_radiance.cs.hlsl` | 游戏GI/材质/水冰pass组合；内含Kajiya reservoir或GPU-Raytracer BVH8等共享代码；消费者另含 Kajiya Reservoir helper | medium |
| `shaders/gi/gi_settings.inc.hlsl` | 游戏GI/材质/水冰pass组合；内含Kajiya reservoir或GPU-Raytracer BVH8等共享代码 | medium |
| `shaders/gi/reproject_screen_radiance.cs.hlsl` | 游戏GI/材质/水冰pass组合；内含Kajiya reservoir或GPU-Raytracer BVH8等共享代码 | medium |
| `shaders/gi/restir.inc.hlsl` | 游戏GI/材质/水冰pass组合；内含Kajiya reservoir或GPU-Raytracer BVH8等共享代码 | medium |
| `shaders/gi/trace_gi_candidates.cs.hlsl` | 游戏GI/材质/水冰pass组合；内含Kajiya reservoir或GPU-Raytracer BVH8等共享代码；消费者另含 GPU-Raytracer BVH8 traversal fragment | medium |
| `shaders/gi/update_gi_radiance.cs.hlsl` | 游戏GI/材质/水冰pass组合；内含Kajiya reservoir或GPU-Raytracer BVH8等共享代码；消费者另含 Kajiya Reservoir helper | medium |
| `shaders/grass/bundles_prefix.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/grass/grass_common.inc.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | medium |
| `shaders/grass/tiles_prefix.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/grass_lod1.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | low |
| `shaders/grass_object_push.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | medium |
| `shaders/ice_plane.raster.hlsl` | Tiny Glade游戏水冰/基础材质接线 | medium |
| `shaders/light_binning/bin_lights_to_cells.cs.hlsl` | 游戏灯光网格/桶/前缀合并接线 | medium |
| `shaders/light_binning/cell_prefix_merge.cs.hlsl` | 游戏灯光网格/桶/前缀合并接线 | medium |
| `shaders/light_binning/cell_prefix_sum.cs.hlsl` | 游戏灯光网格/桶/前缀合并接线 | medium |
| `shaders/light_binning/cell_subtotal_prefix_sum.cs.hlsl` | 游戏灯光网格/桶/前缀合并接线 | medium |
| `shaders/light_binning/grid.inc.hlsl` | 游戏灯光网格/桶/前缀合并接线 | medium |
| `shaders/loading_screen.raster.hlsl` | 游戏UI/选择界面/字形/颜色按钮资源接线 | medium |
| `shaders/logo_animation.raster.hlsl` | 游戏UI/选择界面/字形/颜色按钮资源接线 | medium |
| `shaders/logo_branches_animation.raster.hlsl` | 游戏UI/选择界面/字形/颜色按钮资源接线 | medium |
| `shaders/logo_overlay.raster.hlsl` | 游戏UI/选择界面/字形/颜色按钮资源接线 | medium |
| `shaders/lut/water_noise.cs.hlsl` | 游戏美术/光照/天气/后处理组合 | medium |
| `shaders/lut_gallery/thumbnail.raster.hlsl` | 游戏UI/选择界面/字形/颜色按钮资源接线 | medium |
| `shaders/nani/atlas_part_info.inc.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | low |
| `shaders/nani/clear_draw_lists.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani/clear_histogram.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani/clear_subtotals.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani/cull_and_bucket_curved_halftimber.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | medium |
| `shaders/nani/cull_and_bucket_plaster.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | medium |
| `shaders/nani/cull_and_bucket_smoke.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | medium |
| `shaders/nani/cull_and_bucket_stairs_floor.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | medium |
| `shaders/nani/draw_indexed_indirect_to_dispatch.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani/draw_list_offsets.inc.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | medium |
| `shaders/nani/draw_lists.inc.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | medium |
| `shaders/nani/group_draw_cmd_prefix_sum.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani/histogram_prefix_sum.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani/inc/user.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | low |
| `shaders/nani/prefix_sum_unsorted_instances_per_atlas_entry.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/nani/subtotal_prefix_sum.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/painterly/acquire_target.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/painterly/convolve_target.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/painterly/debug_target.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | medium |
| `shaders/painterly/raster_strokes.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | medium |
| `shaders/plaster.inc.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | low |
| `shaders/rain_particles.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | medium |
| `shaders/rasterize_buildings_animate.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | medium |
| `shaders/rdi/temporal.cs.hlsl` | 游戏美术/光照/天气/后处理组合；消费者另含 Kajiya Reservoir helper | medium |
| `shaders/read_mouse_depth_value.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | medium |
| `shaders/rocky_terrain/inc_mesh.inc.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | low |
| `shaders/roof_material_id.inc.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | medium |
| `shaders/session_selection/background.raster.hlsl` | 游戏UI/选择界面/字形/颜色按钮资源接线 | medium |
| `shaders/session_selection/new_glade_thumbnail.raster.hlsl` | 游戏UI/选择界面/字形/颜色按钮资源接线 | medium |
| `shaders/session_selection/thumbnail.raster.hlsl` | 游戏UI/选择界面/字形/颜色按钮资源接线 | medium |
| `shaders/session_selection/thumbnail_painterly.raster.hlsl` | 游戏UI/选择界面/字形/颜色按钮资源接线 | medium |
| `shaders/skydome.raster.hlsl` | 游戏美术/光照/天气/后处理组合 | medium |
| `shaders/smoke.inc.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | low |
| `shaders/solid_vertex_color.raster.hlsl` | Tiny Glade游戏水冰/基础材质接线 | medium |
| `shaders/solid_vertex_color_albedo_map.raster.hlsl` | Tiny Glade游戏水冰/基础材质接线 | medium |
| `shaders/solid_vertex_color_boat.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | medium |
| `shaders/solid_vertex_color_waterwheel.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | medium |
| `shaders/splash_particle.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | medium |
| `shaders/ssgi/ssgi.cs.hlsl` | XeGTAO主算法include+游戏SSGI调度和绑定；消费者另含 Intel XeGTAO algorithm include | high |
| `shaders/stairs_floor.inc.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | low |
| `shaders/stairs_foliage_exclusion.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | medium |
| `shaders/terrain_editing/inc_terrain_stroke.inc.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | low |
| `shaders/terrain_editing/read_terrain_rock_mask.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | medium |
| `shaders/text.raster.hlsl` | 游戏UI/选择界面/字形/颜色按钮资源接线 | medium |
| `shaders/timer_circle.raster.hlsl` | 游戏UI/选择界面/字形/颜色按钮资源接线 | medium |
| `shaders/ui_2d.raster.hlsl` | 游戏UI/选择界面/字形/颜色按钮资源接线 | medium |
| `shaders/ui_2d_color_button.raster.hlsl` | 游戏UI/选择界面/字形/颜色按钮资源接线 | medium |
| `shaders/ui_2d_ws.raster.hlsl` | 游戏UI/选择界面/字形/颜色按钮资源接线 | medium |
| `shaders/vermeer/acquire_target.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/vermeer/combine.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/vermeer/convolve_target.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | high |
| `shaders/wall/inc/brick_instance.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | low |
| `shaders/wall/inc/brick_instance_flags.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | low |
| `shaders/wall/inc/brick_vs_out.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | medium |
| `shaders/wall/inc/draw_lists.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | medium |
| `shaders/wall/inc/highlight_mode.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | low |
| `shaders/wall/reset_pass2_inst_counts.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | medium |
| `shaders/wall/wall_brick_lod0.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | low |
| `shaders/wall/wall_brick_lod1.raster.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | low |
| `shaders/water/calc_deferred_rt_args.cs.hlsl` | 游戏GI/材质/水冰pass组合；内含Kajiya reservoir或GPU-Raytracer BVH8等共享代码 | medium |
| `shaders/water/deferred_rt.cs.hlsl` | 游戏GI/材质/水冰pass组合；内含Kajiya reservoir或GPU-Raytracer BVH8等共享代码；消费者另含 GPU-Raytracer BVH8 traversal fragment | medium |
| `shaders/water/ice_cracks.raster.hlsl` | Tiny Glade游戏水冰/基础材质接线 | medium |
| `shaders/water/ice_deferred_rt.cs.hlsl` | 游戏GI/材质/水冰pass组合；内含Kajiya reservoir或GPU-Raytracer BVH8等共享代码；消费者另含 GPU-Raytracer BVH8 traversal fragment | medium |
| `shaders/water/ice_refl_march.cs.hlsl` | 游戏GI/材质/水冰pass组合；内含Kajiya reservoir或GPU-Raytracer BVH8等共享代码 | medium |
| `shaders/water/ice_refl_resolve.cs.hlsl` | 游戏GI/材质/水冰pass组合；内含Kajiya reservoir或GPU-Raytracer BVH8等共享代码 | medium |
| `shaders/water/ice_refr_resolve.cs.hlsl` | 游戏GI/材质/水冰pass组合；内含Kajiya reservoir或GPU-Raytracer BVH8等共享代码 | medium |
| `shaders/water/ice_shared.inc.hlsl` | Tiny Glade游戏水冰/基础材质接线 | medium |
| `shaders/water/read_water_mask.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | medium |
| `shaders/water/spawn_waterwheels.cs.hlsl` | Tiny Glade game integration; individual formula/source ancestry unspecified | medium |
| `shaders/water/water_object_push.inc.hlsl` | Tiny Glade游戏水冰/基础材质接线 | medium |
| `shaders/water/water_surface_frag.inc.hlsl` | Tiny Glade游戏水冰/基础材质接线 | medium |
| `shaders/water/waterfall_vert.inc.hlsl` | Tiny Glade游戏水冰/基础材质接线 | medium |
| `shaders/water_plane.raster.hlsl` | Tiny Glade游戏水冰/基础材质接线 | medium |
| `shaders/water_plane_refraction_copy.raster.hlsl` | Tiny Glade游戏水冰/基础材质接线 | medium |
| `shaders/water_snow_plane.raster.hlsl` | Tiny Glade游戏水冰/基础材质接线 | medium |
| `shaders/water_x_paths.raster.hlsl` | Tiny Glade游戏水冰/基础材质接线 | medium |
| `shaders/waterfall.raster.hlsl` | Tiny Glade游戏水冰/基础材质接线 | medium |
| `shaders/waterfall_refraction_copy.raster.hlsl` | Tiny Glade游戏水冰/基础材质接线 | medium |

## 通用代码，具体来源未定（104 个路径）

| 原作源路径 | 来源/范围 | 置信度 |
|---|---|---|
| `shaders/blit/blit_stretch_decode_srgb.raster.hlsl` | 未确定 | low |
| `shaders/blit/resolve_msaa_to_swapchain.raster.hlsl` | 未确定 | low |
| `shaders/blit/resolve_to_swapchain.raster.hlsl` | 未确定 | low |
| `shaders/blur.cs.hlsl` | 未确定 | low |
| `shaders/blur_pyramid/rev_blur.cs.hlsl` | 未确定 | low |
| `shaders/capture_panic_screenshot.cs.hlsl` | 未确定 | low |
| `shaders/copy_depth_to_r.cs.hlsl` | 未确定 | low |
| `shaders/dashed_line.raster.hlsl` | 未确定 | low |
| `shaders/extract_half_res_depth_normals.cs.hlsl` | 未确定 | low |
| `shaders/fill_buffer.cs.hlsl` | 未确定 | low |
| `shaders/fsr_copy_history.cs.hlsl` | 未确定 | low |
| `shaders/fsr_init_reactive.cs.hlsl` | 未确定 | low |
| `shaders/fullscreen_triangle_vert.inc.hlsl` | 未确定 | low |
| `shaders/gauss_blur.cs.hlsl` | 未确定 | low |
| `shaders/gauss_blur_mask.cs.hlsl` | 未确定 | low |
| `shaders/gauss_blur_r16f.cs.hlsl` | 未确定 | low |
| `shaders/inc/atomics.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/bicubic.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/bilinear.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/bitonic_sort.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/blue_noise.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/brdf.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/brdf_lut.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/cascades.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/coc.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/color/luv.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/color/math.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/color/srgb.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/color/xyz.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/color/ycbcr.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/color_library.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/common.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/consts.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/current_view.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/curve.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/debug_utils.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/draw_indirect.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/draw_indirect_readonly.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/draw_indirect_types.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/fbm_noise.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/find_root.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/frustum.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/gaussian_rand.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/grading_lut.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/halfres.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/hammersley.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/hash.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/history_valid.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/hsv.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/image.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/lighting.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/lighting_indirect.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/lod.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/math.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/math_const.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/morton.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/pack_unpack.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/parallax_mapping.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/perlin.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/prefix_scan.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/projection.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/quad_mesh.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/ray_cone.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/raymarch.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/samplers.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/scatter.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/sh.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/shadow.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/simplex_noise.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/smooth_max.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/stochastic_tiling.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/sun.inc.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/texture_blend.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/texture_utils.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/tracked_buffer.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/upsample/easu.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/upsample/lanczos2.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/utils.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/inc/uv.hlsl` | Unresolved common graphics helper or pass | insufficient_for_code_origin |
| `shaders/ivy/arch_noise_discard.inc.hlsl` | Unresolved generic helper ancestry | medium |
| `shaders/lut/brdf_fg.cs.hlsl` | 未确定 | low |
| `shaders/mei_shadow.raster.hlsl` | Unresolved generic helper ancestry | medium |
| `shaders/nani/inc/trimesh.hlsl` | Unresolved generic helper ancestry | medium |
| `shaders/normal_debug.raster.hlsl` | 未确定 | low |
| `shaders/prefix_scan/inclusive_prefix_scan.cs.hlsl` | 未确定 | low |
| `shaders/prefix_scan/inclusive_prefix_scan_merge.cs.hlsl` | 未确定 | low |
| `shaders/prefix_scan/inclusive_prefix_scan_subtotals.cs.hlsl` | 未确定 | low |
| `shaders/primary_rt_debug.cs.hlsl` | 未确定；消费者另含 GPU-Raytracer BVH8 traversal fragment | low |
| `shaders/resolve_to_swapchain.cs.hlsl` | 未确定 | low |
| `shaders/resolve_to_swapchain.inc.hlsl` | 未确定 | low |
| `shaders/rocky_terrain/stairs/isolines.inc.hlsl` | Unresolved generic helper ancestry | medium |
| `shaders/rocky_terrain/stairs/marching_squares.inc.hlsl` | Unresolved generic helper ancestry | medium |
| `shaders/rocky_terrain/triangle_adjacency.inc.hlsl` | Unresolved generic helper ancestry | medium |
| `shaders/screenshot/capture_screenshot.inc.hlsl` | 未确定 | low |
| `shaders/screenshot/capture_screenshot_buffer.cs.hlsl` | 未确定 | low |
| `shaders/screenshot/capture_screenshot_texture.cs.hlsl` | 未确定 | low |
| `shaders/shader_refactoring.cs.hlsl` | 未确定 | low |
| `shaders/shadow_mask_mean.cs.hlsl` | 未确定 | low |
| `shaders/ssgi/vaShared.hlsl.inc` | XeGTAO 配套 vaShared 支持头候选；原作只保留依赖，具体实现出处与改写范围未定 | low |
| `shaders/star.raster.hlsl` | Unresolved generic helper ancestry | medium |
| `shaders/terrain_editing/sdf_utils.inc.hlsl` | Unresolved generic helper ancestry | medium |
| `shaders/texture_test.raster.hlsl` | 未确定 | low |
| `shaders/upload_cpu_raster.cs.hlsl` | 未确定 | low |
| `shaders/vertex_color.raster.hlsl` | 通用顶点颜色shader，源码来源待定 | low |

