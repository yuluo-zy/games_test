# HLSL 原源码文件进度

独立HLSL源码层：404/404原bin有对应重建main；260main/共享include及大算法族已重建。对应覆盖不等于算法验收。

本轮实际DXC构建404包/638stage全部生成：393官方1.9+11官方preview1.10；新完整metadata210/404、stage公共接口302/638、cap617/638、非debugopcode histogram100/638与原一致。原metadata再反射404零差异，不等于源码shader算法等价。

原入口/阶段来自bin嵌入编译记录；重建helper名字为词法扫描，不能证明原作者名字或内联算法等价。每个直接匹配报告均保留，没有自动选择最新PASS。

| 原嵌入源码路径 | 本机恢复文件 | 原入口/阶段 | 报告与缺口 |
|---|---|---|---|
| `shaders/apply_screen_shadows.cs.hlsl` | [reconstruction/shader-source/shaders/apply_screen_shadows.cs.hlsl](../../reconstruction/shader-source/shaders/apply_screen_shadows.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/common.hlsl` | [reconstruction/shader-source/shaders/inc/common.hlsl](../../reconstruction/shader-source/shaders/inc/common.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/samplers.hlsl` | [reconstruction/shader-source/shaders/inc/samplers.hlsl](../../reconstruction/shader-source/shaders/inc/samplers.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/debug_utils.hlsl` | [reconstruction/shader-source/shaders/inc/debug_utils.hlsl](../../reconstruction/shader-source/shaders/inc/debug_utils.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/tracked_buffer.hlsl` | [reconstruction/shader-source/shaders/inc/tracked_buffer.hlsl](../../reconstruction/shader-source/shaders/inc/tracked_buffer.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/pack_unpack.hlsl` | [reconstruction/shader-source/shaders/inc/pack_unpack.hlsl](../../reconstruction/shader-source/shaders/inc/pack_unpack.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/math.hlsl` | [reconstruction/shader-source/shaders/inc/math.hlsl](../../reconstruction/shader-source/shaders/inc/math.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/math_const.hlsl` | [reconstruction/shader-source/shaders/inc/math_const.hlsl](../../reconstruction/shader-source/shaders/inc/math_const.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/uv.hlsl` | [reconstruction/shader-source/shaders/inc/uv.hlsl](../../reconstruction/shader-source/shaders/inc/uv.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/frame_constants.hlsl` | [reconstruction/shader-source/shaders/inc/frame_constants.hlsl](../../reconstruction/shader-source/shaders/inc/frame_constants.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/ray_cone.hlsl` | [reconstruction/shader-source/shaders/inc/ray_cone.hlsl](../../reconstruction/shader-source/shaders/inc/ray_cone.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/sun.inc.hlsl` | [reconstruction/shader-source/shaders/inc/sun.inc.hlsl](../../reconstruction/shader-source/shaders/inc/sun.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/color_library.hlsl` | [reconstruction/shader-source/shaders/inc/color_library.hlsl](../../reconstruction/shader-source/shaders/inc/color_library.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/color/srgb.hlsl` | [reconstruction/shader-source/shaders/inc/color/srgb.hlsl](../../reconstruction/shader-source/shaders/inc/color/srgb.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/utils.hlsl` | [reconstruction/shader-source/shaders/inc/utils.hlsl](../../reconstruction/shader-source/shaders/inc/utils.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/consts.hlsl` | [reconstruction/shader-source/shaders/inc/consts.hlsl](../../reconstruction/shader-source/shaders/inc/consts.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/hash.hlsl` | [reconstruction/shader-source/shaders/inc/hash.hlsl](../../reconstruction/shader-source/shaders/inc/hash.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/gi/gi_settings.inc.hlsl` | [reconstruction/shader-source/shaders/gi/gi_settings.inc.hlsl](../../reconstruction/shader-source/shaders/gi/gi_settings.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/history_valid.hlsl` | [reconstruction/shader-source/shaders/inc/history_valid.hlsl](../../reconstruction/shader-source/shaders/inc/history_valid.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/shadow.hlsl` | [reconstruction/shader-source/shaders/inc/shadow.hlsl](../../reconstruction/shader-source/shaders/inc/shadow.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/hammersley.hlsl` | [reconstruction/shader-source/shaders/inc/hammersley.hlsl](../../reconstruction/shader-source/shaders/inc/hammersley.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/cascades.hlsl` | [reconstruction/shader-source/shaders/inc/cascades.hlsl](../../reconstruction/shader-source/shaders/inc/cascades.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/apply_volumetrics.cs.hlsl` | [reconstruction/shader-source/shaders/apply_volumetrics.cs.hlsl](../../reconstruction/shader-source/shaders/apply_volumetrics.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/current_view.hlsl` | [reconstruction/shader-source/shaders/inc/current_view.hlsl](../../reconstruction/shader-source/shaders/inc/current_view.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/perlin.hlsl` | [reconstruction/shader-source/shaders/inc/perlin.hlsl](../../reconstruction/shader-source/shaders/inc/perlin.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/blue_noise.hlsl` | [reconstruction/shader-source/shaders/inc/blue_noise.hlsl](../../reconstruction/shader-source/shaders/inc/blue_noise.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/billboard_forest.raster.hlsl` | [reconstruction/shader-source/shaders/billboard_forest.raster.hlsl](../../reconstruction/shader-source/shaders/billboard_forest.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/simplex_noise.hlsl` | [reconstruction/shader-source/shaders/inc/simplex_noise.hlsl](../../reconstruction/shader-source/shaders/inc/simplex_noise.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/material/solid_material.inc.hlsl` | [reconstruction/shader-source/shaders/inc/material/solid_material.inc.hlsl](../../reconstruction/shader-source/shaders/inc/material/solid_material.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/material_eval.inc.hlsl` | [reconstruction/shader-source/shaders/inc/material_eval.inc.hlsl](../../reconstruction/shader-source/shaders/inc/material_eval.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/brdf.hlsl` | [reconstruction/shader-source/shaders/inc/brdf.hlsl](../../reconstruction/shader-source/shaders/inc/brdf.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/lighting.hlsl` | [reconstruction/shader-source/shaders/inc/lighting.hlsl](../../reconstruction/shader-source/shaders/inc/lighting.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/sh.hlsl` | [reconstruction/shader-source/shaders/inc/sh.hlsl](../../reconstruction/shader-source/shaders/inc/sh.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/billboard_plant.raster.hlsl` | [reconstruction/shader-source/shaders/billboard_plant.raster.hlsl](../../reconstruction/shader-source/shaders/billboard_plant.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/blit/blit_stretch_decode_srgb.raster.hlsl` | [reconstruction/shader-source/shaders/blit/blit_stretch_decode_srgb.raster.hlsl](../../reconstruction/shader-source/shaders/blit/blit_stretch_decode_srgb.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/fullscreen_triangle_vert.inc.hlsl` | [reconstruction/shader-source/shaders/fullscreen_triangle_vert.inc.hlsl](../../reconstruction/shader-source/shaders/fullscreen_triangle_vert.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/resolve_to_swapchain.inc.hlsl` | [reconstruction/shader-source/shaders/resolve_to_swapchain.inc.hlsl](../../reconstruction/shader-source/shaders/resolve_to_swapchain.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/blit/resolve_msaa_to_swapchain.raster.hlsl` | [reconstruction/shader-source/shaders/blit/resolve_msaa_to_swapchain.raster.hlsl](../../reconstruction/shader-source/shaders/blit/resolve_msaa_to_swapchain.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/blit/resolve_to_swapchain.raster.hlsl` | [reconstruction/shader-source/shaders/blit/resolve_to_swapchain.raster.hlsl](../../reconstruction/shader-source/shaders/blit/resolve_to_swapchain.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/blur.cs.hlsl` | [reconstruction/shader-source/shaders/blur.cs.hlsl](../../reconstruction/shader-source/shaders/blur.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/blur_downsample.cs.hlsl` | [reconstruction/shader-source/shaders/blur_downsample.cs.hlsl](../../reconstruction/shader-source/shaders/blur_downsample.cs.hlsl) 存在 | cs_main/compute, cs_main_multisampled/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/blur_pyramid/blur.cs.hlsl` | [reconstruction/shader-source/shaders/blur_pyramid/blur.cs.hlsl](../../reconstruction/shader-source/shaders/blur_pyramid/blur.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/blur_pyramid/rev_blur.cs.hlsl` | [reconstruction/shader-source/shaders/blur_pyramid/rev_blur.cs.hlsl](../../reconstruction/shader-source/shaders/blur_pyramid/rev_blur.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/calculate_reprojection_map.cs.hlsl` | [reconstruction/shader-source/shaders/calculate_reprojection_map.cs.hlsl](../../reconstruction/shader-source/shaders/calculate_reprojection_map.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/bilinear.hlsl` | [reconstruction/shader-source/shaders/inc/bilinear.hlsl](../../reconstruction/shader-source/shaders/inc/bilinear.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/capture_panic_screenshot.cs.hlsl` | [reconstruction/shader-source/shaders/capture_panic_screenshot.cs.hlsl](../../reconstruction/shader-source/shaders/capture_panic_screenshot.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/color/ycbcr.hlsl` | [reconstruction/shader-source/shaders/inc/color/ycbcr.hlsl](../../reconstruction/shader-source/shaders/inc/color/ycbcr.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/clearing/distant_billboard.raster.hlsl` | [reconstruction/shader-source/shaders/clearing/distant_billboard.raster.hlsl](../../reconstruction/shader-source/shaders/clearing/distant_billboard.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/clearing/instanced_background_tree.raster.hlsl` | [reconstruction/shader-source/shaders/clearing/instanced_background_tree.raster.hlsl](../../reconstruction/shader-source/shaders/clearing/instanced_background_tree.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/clearing/inc/instanced_placement.hlsl` | [reconstruction/shader-source/shaders/clearing/inc/instanced_placement.hlsl](../../reconstruction/shader-source/shaders/clearing/inc/instanced_placement.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/clearing/meadow_flower.raster.hlsl` | [reconstruction/shader-source/shaders/clearing/meadow_flower.raster.hlsl](../../reconstruction/shader-source/shaders/clearing/meadow_flower.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/paths/path_mask_sampler.inc.hlsl` | [reconstruction/shader-source/shaders/paths/path_mask_sampler.inc.hlsl](../../reconstruction/shader-source/shaders/paths/path_mask_sampler.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/texture_utils.hlsl` | [reconstruction/shader-source/shaders/inc/texture_utils.hlsl](../../reconstruction/shader-source/shaders/inc/texture_utils.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/terrain_heightmap.inc.hlsl` | [reconstruction/shader-source/shaders/inc/terrain_heightmap.inc.hlsl](../../reconstruction/shader-source/shaders/inc/terrain_heightmap.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/rocky_terrain.inc.hlsl` | [reconstruction/shader-source/shaders/inc/rocky_terrain.inc.hlsl](../../reconstruction/shader-source/shaders/inc/rocky_terrain.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/water/water_mask_sampler.inc.hlsl` | [reconstruction/shader-source/shaders/water/water_mask_sampler.inc.hlsl](../../reconstruction/shader-source/shaders/water/water_mask_sampler.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/material/plant_material.inc.hlsl` | [reconstruction/shader-source/shaders/inc/material/plant_material.inc.hlsl](../../reconstruction/shader-source/shaders/inc/material/plant_material.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/clearing/meadow_plant.raster.hlsl` | [reconstruction/shader-source/shaders/clearing/meadow_plant.raster.hlsl](../../reconstruction/shader-source/shaders/clearing/meadow_plant.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/buildings/building_mask.inc.hlsl` | [reconstruction/shader-source/shaders/buildings/building_mask.inc.hlsl](../../reconstruction/shader-source/shaders/buildings/building_mask.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/foliage_exclusion.hlsl` | [reconstruction/shader-source/shaders/inc/foliage_exclusion.hlsl](../../reconstruction/shader-source/shaders/inc/foliage_exclusion.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/wind.hlsl` | [reconstruction/shader-source/shaders/inc/wind.hlsl](../../reconstruction/shader-source/shaders/inc/wind.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/clearing/tree_canopy.raster.hlsl` | [reconstruction/shader-source/shaders/clearing/tree_canopy.raster.hlsl](../../reconstruction/shader-source/shaders/clearing/tree_canopy.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/clearing/canopy_vert.inc.hlsl` | [reconstruction/shader-source/shaders/clearing/canopy_vert.inc.hlsl](../../reconstruction/shader-source/shaders/clearing/canopy_vert.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/clearing/inc/canopy_common.inc.hlsl` | [reconstruction/shader-source/shaders/clearing/inc/canopy_common.inc.hlsl](../../reconstruction/shader-source/shaders/clearing/inc/canopy_common.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/inc/user.hlsl` | [reconstruction/shader-source/shaders/nani/inc/user.hlsl](../../reconstruction/shader-source/shaders/nani/inc/user.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/material/canopy_material.inc.hlsl` | [reconstruction/shader-source/shaders/inc/material/canopy_material.inc.hlsl](../../reconstruction/shader-source/shaders/inc/material/canopy_material.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/clearing/tree_trunk.raster.hlsl` | [reconstruction/shader-source/shaders/clearing/tree_trunk.raster.hlsl](../../reconstruction/shader-source/shaders/clearing/tree_trunk.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/texture_blend.hlsl` | [reconstruction/shader-source/shaders/inc/texture_blend.hlsl](../../reconstruction/shader-source/shaders/inc/texture_blend.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/cloud.raster.hlsl` | [reconstruction/shader-source/shaders/cloud.raster.hlsl](../../reconstruction/shader-source/shaders/cloud.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/copy_depth_to_r.cs.hlsl` | [reconstruction/shader-source/shaders/copy_depth_to_r.cs.hlsl](../../reconstruction/shader-source/shaders/copy_depth_to_r.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/critters/butterfly.raster.hlsl` | [reconstruction/shader-source/shaders/critters/butterfly.raster.hlsl](../../reconstruction/shader-source/shaders/critters/butterfly.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/critters/duck_animated.raster.hlsl` | [reconstruction/shader-source/shaders/critters/duck_animated.raster.hlsl](../../reconstruction/shader-source/shaders/critters/duck_animated.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/critters/firefly.raster.hlsl` | [reconstruction/shader-source/shaders/critters/firefly.raster.hlsl](../../reconstruction/shader-source/shaders/critters/firefly.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/critters/firefly_glow.raster.hlsl` | [reconstruction/shader-source/shaders/critters/firefly_glow.raster.hlsl](../../reconstruction/shader-source/shaders/critters/firefly_glow.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/coc.hlsl` | [reconstruction/shader-source/shaders/inc/coc.hlsl](../../reconstruction/shader-source/shaders/inc/coc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/critters/firefly_vis_query.cs.hlsl` | [reconstruction/shader-source/shaders/critters/firefly_vis_query.cs.hlsl](../../reconstruction/shader-source/shaders/critters/firefly_vis_query.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/critters/nani_wren.raster.hlsl` | [reconstruction/shader-source/shaders/critters/nani_wren.raster.hlsl](../../reconstruction/shader-source/shaders/critters/nani_wren.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/custom_cursor.raster.hlsl` | [reconstruction/shader-source/shaders/custom_cursor.raster.hlsl](../../reconstruction/shader-source/shaders/custom_cursor.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/quad_mesh.hlsl` | [reconstruction/shader-source/shaders/inc/quad_mesh.hlsl](../../reconstruction/shader-source/shaders/inc/quad_mesh.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/dashed_line.raster.hlsl` | [reconstruction/shader-source/shaders/dashed_line.raster.hlsl](../../reconstruction/shader-source/shaders/dashed_line.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/deferred/light_sun_sky_prelude.inc.hlsl` | [reconstruction/shader-source/shaders/deferred/light_sun_sky_prelude.inc.hlsl](../../reconstruction/shader-source/shaders/deferred/light_sun_sky_prelude.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/halfres.hlsl` | [reconstruction/shader-source/shaders/inc/halfres.hlsl](../../reconstruction/shader-source/shaders/inc/halfres.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/terrain_editing/inc_highlight_active_terrain_chunk.inc.hlsl` | [reconstruction/shader-source/shaders/terrain_editing/inc_highlight_active_terrain_chunk.inc.hlsl](../../reconstruction/shader-source/shaders/terrain_editing/inc_highlight_active_terrain_chunk.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/lighting_indirect.hlsl` | [reconstruction/shader-source/shaders/inc/lighting_indirect.hlsl](../../reconstruction/shader-source/shaders/inc/lighting_indirect.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/lanterns.hlsl` | [reconstruction/shader-source/shaders/inc/lanterns.hlsl](../../reconstruction/shader-source/shaders/inc/lanterns.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/deferred/canopy_light_sun_sky.raster.hlsl` | [reconstruction/shader-source/shaders/deferred/canopy_light_sun_sky.raster.hlsl](../../reconstruction/shader-source/shaders/deferred/canopy_light_sun_sky.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | [evidence/render-passes/integration/wave2-gi-deferred-merge/verify-deferred_canopy_b903.json](../../evidence/render-passes/integration/wave2-gi-deferred-merge/verify-deferred_canopy_b903.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>[evidence/render-passes/integration/wave2-gi-deferred-merge/verify-deferred_canopy_f0ad.json](../../evidence/render-passes/integration/wave2-gi-deferred-merge/verify-deferred_canopy_f0ad.json)：None<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/material/eval/canopy_material.inc.hlsl` | [reconstruction/shader-source/shaders/inc/material/eval/canopy_material.inc.hlsl](../../reconstruction/shader-source/shaders/inc/material/eval/canopy_material.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/material/eval/../canopy_material.inc.hlsl` | [reconstruction/shader-source/shaders/inc/material/eval/../canopy_material.inc.hlsl](../../reconstruction/shader-source/shaders/inc/material/canopy_material.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/raymarch.hlsl` | [reconstruction/shader-source/shaders/inc/raymarch.hlsl](../../reconstruction/shader-source/shaders/inc/raymarch.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/deferred/light_sun_sky.inc.hlsl` | [reconstruction/shader-source/shaders/deferred/light_sun_sky.inc.hlsl](../../reconstruction/shader-source/shaders/deferred/light_sun_sky.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/rt.hlsl` | [reconstruction/shader-source/shaders/inc/rt.hlsl](../../reconstruction/shader-source/shaders/inc/rt.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/material/eval/solid_material.inc.hlsl` | [reconstruction/shader-source/shaders/inc/material/eval/solid_material.inc.hlsl](../../reconstruction/shader-source/shaders/inc/material/eval/solid_material.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/material/eval/../solid_material.inc.hlsl` | [reconstruction/shader-source/shaders/inc/material/eval/../solid_material.inc.hlsl](../../reconstruction/shader-source/shaders/inc/material/solid_material.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/brdf_lut.hlsl` | [reconstruction/shader-source/shaders/inc/brdf_lut.hlsl](../../reconstruction/shader-source/shaders/inc/brdf_lut.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/highlight_color.hlsl` | [reconstruction/shader-source/shaders/inc/highlight_color.hlsl](../../reconstruction/shader-source/shaders/inc/highlight_color.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/wall_color_ids.hlsl` | [reconstruction/shader-source/shaders/inc/wall_color_ids.hlsl](../../reconstruction/shader-source/shaders/inc/wall_color_ids.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/reservoir.hlsl` | [reconstruction/shader-source/shaders/inc/reservoir.hlsl](../../reconstruction/shader-source/shaders/inc/reservoir.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/light_binning/grid.inc.hlsl` | [reconstruction/shader-source/shaders/light_binning/grid.inc.hlsl](../../reconstruction/shader-source/shaders/light_binning/grid.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/deferred/grass_light_sun_sky.raster.hlsl` | [reconstruction/shader-source/shaders/deferred/grass_light_sun_sky.raster.hlsl](../../reconstruction/shader-source/shaders/deferred/grass_light_sun_sky.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | [evidence/render-passes/integration/wave2-gi-deferred-merge/verify-deferred_grass_b903.json](../../evidence/render-passes/integration/wave2-gi-deferred-merge/verify-deferred_grass_b903.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>[evidence/render-passes/integration/wave2-gi-deferred-merge/verify-deferred_grass_f0ad.json](../../evidence/render-passes/integration/wave2-gi-deferred-merge/verify-deferred_grass_f0ad.json)：None<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/material/eval/grass_material.inc.hlsl` | [reconstruction/shader-source/shaders/inc/material/eval/grass_material.inc.hlsl](../../reconstruction/shader-source/shaders/inc/material/eval/grass_material.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/material/eval/../grass_material.inc.hlsl` | [reconstruction/shader-source/shaders/inc/material/eval/../grass_material.inc.hlsl](../../reconstruction/shader-source/shaders/inc/material/grass_material.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/deferred/plant_light_sun_sky.raster.hlsl` | [reconstruction/shader-source/shaders/deferred/plant_light_sun_sky.raster.hlsl](../../reconstruction/shader-source/shaders/deferred/plant_light_sun_sky.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | [evidence/render-passes/integration/wave2-gi-deferred-merge/verify-deferred_plant_b903.json](../../evidence/render-passes/integration/wave2-gi-deferred-merge/verify-deferred_plant_b903.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>[evidence/render-passes/integration/wave2-gi-deferred-merge/verify-deferred_plant_f0ad.json](../../evidence/render-passes/integration/wave2-gi-deferred-merge/verify-deferred_plant_f0ad.json)：None<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/material/eval/plant_material.inc.hlsl` | [reconstruction/shader-source/shaders/inc/material/eval/plant_material.inc.hlsl](../../reconstruction/shader-source/shaders/inc/material/eval/plant_material.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/material/eval/../plant_material.inc.hlsl` | [reconstruction/shader-source/shaders/inc/material/eval/../plant_material.inc.hlsl](../../reconstruction/shader-source/shaders/inc/material/plant_material.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/deferred/solid_light_sun_sky.raster.hlsl` | [reconstruction/shader-source/shaders/deferred/solid_light_sun_sky.raster.hlsl](../../reconstruction/shader-source/shaders/deferred/solid_light_sun_sky.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | [evidence/render-passes/integration/wave2-gi-deferred-merge/verify-deferred_solid_b903.json](../../evidence/render-passes/integration/wave2-gi-deferred-merge/verify-deferred_solid_b903.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>[evidence/render-passes/integration/wave2-gi-deferred-merge/verify-deferred_solid_f0ad.json](../../evidence/render-passes/integration/wave2-gi-deferred-merge/verify-deferred_solid_f0ad.json)：None<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/demo_terrain_overlay.raster.hlsl` | [reconstruction/shader-source/shaders/demo_terrain_overlay.raster.hlsl](../../reconstruction/shader-source/shaders/demo_terrain_overlay.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | [evidence/render-passes/recovery/f-t/demo_terrain_overlay.raster.hlsl.report.json](../../evidence/render-passes/recovery/f-t/demo_terrain_overlay.raster.hlsl.report.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 1, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/dof/analyze.cs.hlsl` | [reconstruction/shader-source/shaders/dof/analyze.cs.hlsl](../../reconstruction/shader-source/shaders/dof/analyze.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_analyze.cs.hlsl_cs_main_0.bin.json](../../evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_analyze.cs.hlsl_cs_main_0.bin.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/morton.hlsl` | [reconstruction/shader-source/shaders/inc/morton.hlsl](../../reconstruction/shader-source/shaders/inc/morton.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/dof/tiles.inc.hlsl` | [reconstruction/shader-source/shaders/dof/tiles.inc.hlsl](../../reconstruction/shader-source/shaders/dof/tiles.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/dof/autofocus.cs.hlsl` | [reconstruction/shader-source/shaders/dof/autofocus.cs.hlsl](../../reconstruction/shader-source/shaders/dof/autofocus.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_autofocus.cs.hlsl_cs_main_0.bin.json](../../evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_autofocus.cs.hlsl_cs_main_0.bin.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/bitonic_sort.hlsl` | [reconstruction/shader-source/shaders/inc/bitonic_sort.hlsl](../../reconstruction/shader-source/shaders/inc/bitonic_sort.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/dof/clear_halfres.cs.hlsl` | [reconstruction/shader-source/shaders/dof/clear_halfres.cs.hlsl](../../reconstruction/shader-source/shaders/dof/clear_halfres.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_clear_halfres.cs.hlsl_cs_main_0.bin.json](../../evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_clear_halfres.cs.hlsl_cs_main_0.bin.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/dof/fullres.cs.hlsl` | [reconstruction/shader-source/shaders/dof/fullres.cs.hlsl](../../reconstruction/shader-source/shaders/dof/fullres.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_fullres.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin.json](../../evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_fullres.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>[evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_fullres.cs.hlsl_cs_main_72a2a95d73e37edc.bin.json](../../evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_fullres.cs.hlsl_cs_main_72a2a95d73e37edc.bin.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/dof/multires.inc.hlsl` | [reconstruction/shader-source/shaders/dof/multires.inc.hlsl](../../reconstruction/shader-source/shaders/dof/multires.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/dof/upsample_halfres.inc.hlsl` | [reconstruction/shader-source/shaders/dof/upsample_halfres.inc.hlsl](../../reconstruction/shader-source/shaders/dof/upsample_halfres.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/dof/fullres_all_lod0.cs.hlsl` | [reconstruction/shader-source/shaders/dof/fullres_all_lod0.cs.hlsl](../../reconstruction/shader-source/shaders/dof/fullres_all_lod0.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_fullres_all_lod0.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin.json](../../evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_fullres_all_lod0.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>[evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_fullres_all_lod0.cs.hlsl_cs_main_72a2a95d73e37edc.bin.json](../../evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_fullres_all_lod0.cs.hlsl_cs_main_72a2a95d73e37edc.bin.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/dof/fullres_all_lod1.cs.hlsl` | [reconstruction/shader-source/shaders/dof/fullres_all_lod1.cs.hlsl](../../reconstruction/shader-source/shaders/dof/fullres_all_lod1.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_fullres_all_lod1.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin.json](../../evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_fullres_all_lod1.cs.hlsl_cs_main_6f1dd5dd8a66e104.bin.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>[evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_fullres_all_lod1.cs.hlsl_cs_main_72a2a95d73e37edc.bin.json](../../evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_fullres_all_lod1.cs.hlsl_cs_main_72a2a95d73e37edc.bin.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/dof/fullres_upsample.cs.hlsl` | [reconstruction/shader-source/shaders/dof/fullres_upsample.cs.hlsl](../../reconstruction/shader-source/shaders/dof/fullres_upsample.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_fullres_upsample.cs.hlsl_cs_main_0.bin.json](../../evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_fullres_upsample.cs.hlsl_cs_main_0.bin.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/dof/fused.cs.hlsl` | [reconstruction/shader-source/shaders/dof/fused.cs.hlsl](../../reconstruction/shader-source/shaders/dof/fused.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_fused.cs.hlsl_cs_main_0.bin.json](../../evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_fused.cs.hlsl_cs_main_0.bin.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/dof/halfres.cs.hlsl` | [reconstruction/shader-source/shaders/dof/halfres.cs.hlsl](../../reconstruction/shader-source/shaders/dof/halfres.cs.hlsl) 存在 | cs_halfres/compute, cs_halfres_all/compute | [evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_halfres.cs.hlsl_cs_halfres_6f1dd5dd8a66e104.bin.json](../../evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_halfres.cs.hlsl_cs_halfres_6f1dd5dd8a66e104.bin.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>[evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_halfres.cs.hlsl_cs_halfres_72a2a95d73e37edc.bin.json](../../evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_halfres.cs.hlsl_cs_halfres_72a2a95d73e37edc.bin.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>[evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_halfres.cs.hlsl_cs_halfres_all_6f1dd5dd8a66e104.bin.json](../../evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_halfres.cs.hlsl_cs_halfres_all_6f1dd5dd8a66e104.bin.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>[evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_halfres.cs.hlsl_cs_halfres_all_72a2a95d73e37edc.bin.json](../../evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_halfres.cs.hlsl_cs_halfres_all_72a2a95d73e37edc.bin.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/dof/init_buffers.cs.hlsl` | [reconstruction/shader-source/shaders/dof/init_buffers.cs.hlsl](../../reconstruction/shader-source/shaders/dof/init_buffers.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_init_buffers.cs.hlsl_cs_main_0.bin.json](../../evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_init_buffers.cs.hlsl_cs_main_0.bin.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/dof/prepare.cs.hlsl` | [reconstruction/shader-source/shaders/dof/prepare.cs.hlsl](../../reconstruction/shader-source/shaders/dof/prepare.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_prepare.cs.hlsl_cs_main_0.bin.json](../../evidence/render-passes/integration/wave2-dof-aa-merge/check__dof_prepare.cs.hlsl_cs_main_0.bin.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/egui.raster.hlsl` | [reconstruction/shader-source/shaders/egui.raster.hlsl](../../reconstruction/shader-source/shaders/egui.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/extract_half_res_depth_normals.cs.hlsl` | [reconstruction/shader-source/shaders/extract_half_res_depth_normals.cs.hlsl](../../reconstruction/shader-source/shaders/extract_half_res_depth_normals.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/fill_buffer.cs.hlsl` | [reconstruction/shader-source/shaders/fill_buffer.cs.hlsl](../../reconstruction/shader-source/shaders/fill_buffer.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/final_post.raster.hlsl` | [reconstruction/shader-source/shaders/final_post.raster.hlsl](../../reconstruction/shader-source/shaders/final_post.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | [evidence/render-passes/recovery/final-post-painterly/_final_post.raster.hlsl_b903e43ffb3da915.json](../../evidence/render-passes/recovery/final-post-painterly/_final_post.raster.hlsl_b903e43ffb3da915.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/image.hlsl` | [reconstruction/shader-source/shaders/inc/image.hlsl](../../reconstruction/shader-source/shaders/inc/image.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/curve.hlsl` | [reconstruction/shader-source/shaders/inc/curve.hlsl](../../reconstruction/shader-source/shaders/inc/curve.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/bicubic.hlsl` | [reconstruction/shader-source/shaders/inc/bicubic.hlsl](../../reconstruction/shader-source/shaders/inc/bicubic.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/grading_lut.hlsl` | [reconstruction/shader-source/shaders/inc/grading_lut.hlsl](../../reconstruction/shader-source/shaders/inc/grading_lut.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/color/xyz.hlsl` | [reconstruction/shader-source/shaders/inc/color/xyz.hlsl](../../reconstruction/shader-source/shaders/inc/color/xyz.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/color/luv.hlsl` | [reconstruction/shader-source/shaders/inc/color/luv.hlsl](../../reconstruction/shader-source/shaders/inc/color/luv.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/color/helmholtz_kohlrausch.hlsl` | [reconstruction/shader-source/shaders/inc/color/helmholtz_kohlrausch.hlsl](../../reconstruction/shader-source/shaders/inc/color/helmholtz_kohlrausch.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/color/math.hlsl` | [reconstruction/shader-source/shaders/inc/color/math.hlsl](../../reconstruction/shader-source/shaders/inc/color/math.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/upsample/easu.hlsl` | [reconstruction/shader-source/shaders/inc/upsample/easu.hlsl](../../reconstruction/shader-source/shaders/inc/upsample/easu.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/upsample/lanczos2.hlsl` | [reconstruction/shader-source/shaders/inc/upsample/lanczos2.hlsl](../../reconstruction/shader-source/shaders/inc/upsample/lanczos2.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/flag_roof_cloth.raster.hlsl` | [reconstruction/shader-source/shaders/flag_roof_cloth.raster.hlsl](../../reconstruction/shader-source/shaders/flag_roof_cloth.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/flag_small.raster.hlsl` | [reconstruction/shader-source/shaders/flag_small.raster.hlsl](../../reconstruction/shader-source/shaders/flag_small.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/flag_wall_cloth.raster.hlsl` | [reconstruction/shader-source/shaders/flag_wall_cloth.raster.hlsl](../../reconstruction/shader-source/shaders/flag_wall_cloth.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/fsr_copy_history.cs.hlsl` | [reconstruction/shader-source/shaders/fsr_copy_history.cs.hlsl](../../reconstruction/shader-source/shaders/fsr_copy_history.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-dof-aa-merge/check__fsr_copy_history.cs.hlsl_cs_main_0.bin.json](../../evidence/render-passes/integration/wave2-dof-aa-merge/check__fsr_copy_history.cs.hlsl_cs_main_0.bin.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/fsr_init_reactive.cs.hlsl` | [reconstruction/shader-source/shaders/fsr_init_reactive.cs.hlsl](../../reconstruction/shader-source/shaders/fsr_init_reactive.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-dof-aa-merge/check__fsr_init_reactive.cs.hlsl_cs_main_0.bin.json](../../evidence/render-passes/integration/wave2-dof-aa-merge/check__fsr_init_reactive.cs.hlsl_cs_main_0.bin.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/fxaa.cs.hlsl` | [reconstruction/shader-source/shaders/fxaa.cs.hlsl](../../reconstruction/shader-source/shaders/fxaa.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/fxaa.inc.hlsl` | [reconstruction/shader-source/shaders/fxaa.inc.hlsl](../../reconstruction/shader-source/shaders/fxaa.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/garden/exclude_other_masks.cs.hlsl` | [reconstruction/shader-source/shaders/garden/exclude_other_masks.cs.hlsl](../../reconstruction/shader-source/shaders/garden/exclude_other_masks.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/garden_lowpoly_flower.raster.hlsl` | [reconstruction/shader-source/shaders/garden_lowpoly_flower.raster.hlsl](../../reconstruction/shader-source/shaders/garden_lowpoly_flower.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/garden/marching_squares.cs.hlsl` | [reconstruction/shader-source/shaders/garden/marching_squares.cs.hlsl](../../reconstruction/shader-source/shaders/garden/marching_squares.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/draw_indirect.hlsl` | [reconstruction/shader-source/shaders/inc/draw_indirect.hlsl](../../reconstruction/shader-source/shaders/inc/draw_indirect.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/draw_indirect_types.hlsl` | [reconstruction/shader-source/shaders/inc/draw_indirect_types.hlsl](../../reconstruction/shader-source/shaders/inc/draw_indirect_types.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/find_root.hlsl` | [reconstruction/shader-source/shaders/inc/find_root.hlsl](../../reconstruction/shader-source/shaders/inc/find_root.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/garden/read_bush_positions_for_cpu.cs.hlsl` | [reconstruction/shader-source/shaders/garden/read_bush_positions_for_cpu.cs.hlsl](../../reconstruction/shader-source/shaders/garden/read_bush_positions_for_cpu.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/draw_indirect_readonly.hlsl` | [reconstruction/shader-source/shaders/inc/draw_indirect_readonly.hlsl](../../reconstruction/shader-source/shaders/inc/draw_indirect_readonly.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/garden/defs.inc.hlsl` | [reconstruction/shader-source/shaders/garden/defs.inc.hlsl](../../reconstruction/shader-source/shaders/garden/defs.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/garden/read_garden_mask.cs.hlsl` | [reconstruction/shader-source/shaders/garden/read_garden_mask.cs.hlsl](../../reconstruction/shader-source/shaders/garden/read_garden_mask.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/garden/spawn_bushes_buildings.cs.hlsl` | [reconstruction/shader-source/shaders/garden/spawn_bushes_buildings.cs.hlsl](../../reconstruction/shader-source/shaders/garden/spawn_bushes_buildings.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/garden/placement_utils.inc.hlsl` | [reconstruction/shader-source/shaders/garden/placement_utils.inc.hlsl](../../reconstruction/shader-source/shaders/garden/placement_utils.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/garden/spawn_bushes_garden.cs.hlsl` | [reconstruction/shader-source/shaders/garden/spawn_bushes_garden.cs.hlsl](../../reconstruction/shader-source/shaders/garden/spawn_bushes_garden.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/garden/spawn_flowers_buildings.cs.hlsl` | [reconstruction/shader-source/shaders/garden/spawn_flowers_buildings.cs.hlsl](../../reconstruction/shader-source/shaders/garden/spawn_flowers_buildings.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/garden/spawn_flowers_garden.cs.hlsl` | [reconstruction/shader-source/shaders/garden/spawn_flowers_garden.cs.hlsl](../../reconstruction/shader-source/shaders/garden/spawn_flowers_garden.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/garden/spawn_small_flowers_garden.cs.hlsl` | [reconstruction/shader-source/shaders/garden/spawn_small_flowers_garden.cs.hlsl](../../reconstruction/shader-source/shaders/garden/spawn_small_flowers_garden.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/gauss_blur.cs.hlsl` | [reconstruction/shader-source/shaders/gauss_blur.cs.hlsl](../../reconstruction/shader-source/shaders/gauss_blur.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/gauss_blur_building_mask.cs.hlsl` | [reconstruction/shader-source/shaders/gauss_blur_building_mask.cs.hlsl](../../reconstruction/shader-source/shaders/gauss_blur_building_mask.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/gauss_blur_mask.cs.hlsl` | [reconstruction/shader-source/shaders/gauss_blur_mask.cs.hlsl](../../reconstruction/shader-source/shaders/gauss_blur_mask.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/gauss_blur_r16f.cs.hlsl` | [reconstruction/shader-source/shaders/gauss_blur_r16f.cs.hlsl](../../reconstruction/shader-source/shaders/gauss_blur_r16f.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/generate_grass.cs.hlsl` | [reconstruction/shader-source/shaders/generate_grass.cs.hlsl](../../reconstruction/shader-source/shaders/generate_grass.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/gaussian_rand.hlsl` | [reconstruction/shader-source/shaders/inc/gaussian_rand.hlsl](../../reconstruction/shader-source/shaders/inc/gaussian_rand.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/scatter.hlsl` | [reconstruction/shader-source/shaders/inc/scatter.hlsl](../../reconstruction/shader-source/shaders/inc/scatter.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/grass_blade_data.hlsl` | [reconstruction/shader-source/shaders/inc/grass_blade_data.hlsl](../../reconstruction/shader-source/shaders/inc/grass_blade_data.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/atomics.hlsl` | [reconstruction/shader-source/shaders/inc/atomics.hlsl](../../reconstruction/shader-source/shaders/inc/atomics.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/fbm_noise.hlsl` | [reconstruction/shader-source/shaders/inc/fbm_noise.hlsl](../../reconstruction/shader-source/shaders/inc/fbm_noise.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/frustum.hlsl` | [reconstruction/shader-source/shaders/inc/frustum.hlsl](../../reconstruction/shader-source/shaders/inc/frustum.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/lod.hlsl` | [reconstruction/shader-source/shaders/inc/lod.hlsl](../../reconstruction/shader-source/shaders/inc/lod.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/grass/grass_common.inc.hlsl` | [reconstruction/shader-source/shaders/grass/grass_common.inc.hlsl](../../reconstruction/shader-source/shaders/grass/grass_common.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/generate_hzb.cs.hlsl` | [reconstruction/shader-source/shaders/generate_hzb.cs.hlsl](../../reconstruction/shader-source/shaders/generate_hzb.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/generate_mips.cs.hlsl` | [reconstruction/shader-source/shaders/generate_mips.cs.hlsl](../../reconstruction/shader-source/shaders/generate_mips.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/gi/blur_screen_radiance.cs.hlsl` | [reconstruction/shader-source/shaders/gi/blur_screen_radiance.cs.hlsl](../../reconstruction/shader-source/shaders/gi/blur_screen_radiance.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-gi-deferred-merge/verify-blur_screen_radiance.json](../../evidence/render-passes/integration/wave2-gi-deferred-merge/verify-blur_screen_radiance.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/gi/restir.inc.hlsl` | [reconstruction/shader-source/shaders/gi/restir.inc.hlsl](../../reconstruction/shader-source/shaders/gi/restir.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/gi/eval_screen_radiance.cs.hlsl` | [reconstruction/shader-source/shaders/gi/eval_screen_radiance.cs.hlsl](../../reconstruction/shader-source/shaders/gi/eval_screen_radiance.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-gi-deferred-merge/verify-eval_screen_radiance.json](../../evidence/render-passes/integration/wave2-gi-deferred-merge/verify-eval_screen_radiance.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/gi/reproject_screen_radiance.cs.hlsl` | [reconstruction/shader-source/shaders/gi/reproject_screen_radiance.cs.hlsl](../../reconstruction/shader-source/shaders/gi/reproject_screen_radiance.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-gi-deferred-merge/verify-reproject_screen_radiance.json](../../evidence/render-passes/integration/wave2-gi-deferred-merge/verify-reproject_screen_radiance.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/gi/trace_gi_candidates.cs.hlsl` | [reconstruction/shader-source/shaders/gi/trace_gi_candidates.cs.hlsl](../../reconstruction/shader-source/shaders/gi/trace_gi_candidates.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-gi-deferred-merge/verify-trace.json](../../evidence/render-passes/integration/wave2-gi-deferred-merge/verify-trace.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/gi/update_gi_radiance.cs.hlsl` | [reconstruction/shader-source/shaders/gi/update_gi_radiance.cs.hlsl](../../reconstruction/shader-source/shaders/gi/update_gi_radiance.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-gi-deferred-merge/verify-update_gi_radiance.json](../../evidence/render-passes/integration/wave2-gi-deferred-merge/verify-update_gi_radiance.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/grass.raster.hlsl` | [reconstruction/shader-source/shaders/grass.raster.hlsl](../../reconstruction/shader-source/shaders/grass.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/material/grass_material.inc.hlsl` | [reconstruction/shader-source/shaders/inc/material/grass_material.inc.hlsl](../../reconstruction/shader-source/shaders/inc/material/grass_material.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/grass/bundles_prefix.cs.hlsl` | [reconstruction/shader-source/shaders/grass/bundles_prefix.cs.hlsl](../../reconstruction/shader-source/shaders/grass/bundles_prefix.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/prefix_scan.hlsl` | [reconstruction/shader-source/shaders/inc/prefix_scan.hlsl](../../reconstruction/shader-source/shaders/inc/prefix_scan.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/grass/calc_dispatch_args.cs.hlsl` | [reconstruction/shader-source/shaders/grass/calc_dispatch_args.cs.hlsl](../../reconstruction/shader-source/shaders/grass/calc_dispatch_args.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/grass/calc_grasslet_bundle_ranges.cs.hlsl` | [reconstruction/shader-source/shaders/grass/calc_grasslet_bundle_ranges.cs.hlsl](../../reconstruction/shader-source/shaders/grass/calc_grasslet_bundle_ranges.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/grass/calc_tiles.cs.hlsl` | [reconstruction/shader-source/shaders/grass/calc_tiles.cs.hlsl](../../reconstruction/shader-source/shaders/grass/calc_tiles.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/projection.hlsl` | [reconstruction/shader-source/shaders/inc/projection.hlsl](../../reconstruction/shader-source/shaders/inc/projection.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/grass_lod1.raster.hlsl` | [reconstruction/shader-source/shaders/grass_lod1.raster.hlsl](../../reconstruction/shader-source/shaders/grass_lod1.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/grass_object_push.raster.hlsl` | [reconstruction/shader-source/shaders/grass_object_push.raster.hlsl](../../reconstruction/shader-source/shaders/grass_object_push.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/grass/tile_bundle_count.cs.hlsl` | [reconstruction/shader-source/shaders/grass/tile_bundle_count.cs.hlsl](../../reconstruction/shader-source/shaders/grass/tile_bundle_count.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/grass/tile_bundle_gen.cs.hlsl` | [reconstruction/shader-source/shaders/grass/tile_bundle_gen.cs.hlsl](../../reconstruction/shader-source/shaders/grass/tile_bundle_gen.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/grass/tiles_prefix.cs.hlsl` | [reconstruction/shader-source/shaders/grass/tiles_prefix.cs.hlsl](../../reconstruction/shader-source/shaders/grass/tiles_prefix.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/ice_plane.raster.hlsl` | [reconstruction/shader-source/shaders/ice_plane.raster.hlsl](../../reconstruction/shader-source/shaders/ice_plane.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/stochastic_tiling.hlsl` | [reconstruction/shader-source/shaders/inc/stochastic_tiling.hlsl](../../reconstruction/shader-source/shaders/inc/stochastic_tiling.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/water/ice_shared.inc.hlsl` | [reconstruction/shader-source/shaders/water/ice_shared.inc.hlsl](../../reconstruction/shader-source/shaders/water/ice_shared.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/instanced_roof_beams.raster.hlsl` | [reconstruction/shader-source/shaders/instanced_roof_beams.raster.hlsl](../../reconstruction/shader-source/shaders/instanced_roof_beams.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/instanced_roof_spire.raster.hlsl` | [reconstruction/shader-source/shaders/instanced_roof_spire.raster.hlsl](../../reconstruction/shader-source/shaders/instanced_roof_spire.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/wall_distortion.hlsl` | [reconstruction/shader-source/shaders/inc/wall_distortion.hlsl](../../reconstruction/shader-source/shaders/inc/wall_distortion.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/ivy/instanced_ivy_branch.raster.hlsl` | [reconstruction/shader-source/shaders/ivy/instanced_ivy_branch.raster.hlsl](../../reconstruction/shader-source/shaders/ivy/instanced_ivy_branch.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/ivy/arch_noise_discard.inc.hlsl` | [reconstruction/shader-source/shaders/ivy/arch_noise_discard.inc.hlsl](../../reconstruction/shader-source/shaders/ivy/arch_noise_discard.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/ivy/instanced_ivy_leaf.raster.hlsl` | [reconstruction/shader-source/shaders/ivy/instanced_ivy_leaf.raster.hlsl](../../reconstruction/shader-source/shaders/ivy/instanced_ivy_leaf.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/ivy/ivy.raster.hlsl` | [reconstruction/shader-source/shaders/ivy/ivy.raster.hlsl](../../reconstruction/shader-source/shaders/ivy/ivy.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/leaf_particle.raster.hlsl` | [reconstruction/shader-source/shaders/leaf_particle.raster.hlsl](../../reconstruction/shader-source/shaders/leaf_particle.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/leaf_particles.cs.hlsl` | [reconstruction/shader-source/shaders/leaf_particles.cs.hlsl](../../reconstruction/shader-source/shaders/leaf_particles.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/recovery/f-v/leaf_particles.r1.json](../../evidence/render-passes/recovery/f-v/leaf_particles.r1.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>[evidence/render-passes/recovery/f-v/leaf_particles.r2.json](../../evidence/render-passes/recovery/f-v/leaf_particles.r2.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>[evidence/render-passes/recovery/f-v/leaf_particles.r3.json](../../evidence/render-passes/recovery/f-v/leaf_particles.r3.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/water/water_object_push.inc.hlsl` | [reconstruction/shader-source/shaders/water/water_object_push.inc.hlsl](../../reconstruction/shader-source/shaders/water/water_object_push.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/water/waves.inc.hlsl` | [reconstruction/shader-source/shaders/water/waves.inc.hlsl](../../reconstruction/shader-source/shaders/water/waves.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/light_binning/bin_lights_to_cells.cs.hlsl` | [reconstruction/shader-source/shaders/light_binning/bin_lights_to_cells.cs.hlsl](../../reconstruction/shader-source/shaders/light_binning/bin_lights_to_cells.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-gi-deferred-merge/verify-bin_lights_14dc.json](../../evidence/render-passes/integration/wave2-gi-deferred-merge/verify-bin_lights_14dc.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>[evidence/render-passes/integration/wave2-gi-deferred-merge/verify-bin_lights_8fae.json](../../evidence/render-passes/integration/wave2-gi-deferred-merge/verify-bin_lights_8fae.json)：None<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/light_binning/cell_prefix_merge.cs.hlsl` | [reconstruction/shader-source/shaders/light_binning/cell_prefix_merge.cs.hlsl](../../reconstruction/shader-source/shaders/light_binning/cell_prefix_merge.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-gi-deferred-merge/verify-cell_prefix_merge.json](../../evidence/render-passes/integration/wave2-gi-deferred-merge/verify-cell_prefix_merge.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/light_binning/cell_prefix_sum.cs.hlsl` | [reconstruction/shader-source/shaders/light_binning/cell_prefix_sum.cs.hlsl](../../reconstruction/shader-source/shaders/light_binning/cell_prefix_sum.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-gi-deferred-merge/verify-cell_prefix_sum.json](../../evidence/render-passes/integration/wave2-gi-deferred-merge/verify-cell_prefix_sum.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/light_binning/cell_subtotal_prefix_sum.cs.hlsl` | [reconstruction/shader-source/shaders/light_binning/cell_subtotal_prefix_sum.cs.hlsl](../../reconstruction/shader-source/shaders/light_binning/cell_subtotal_prefix_sum.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-gi-deferred-merge/verify-cell_subtotal_prefix_sum.json](../../evidence/render-passes/integration/wave2-gi-deferred-merge/verify-cell_subtotal_prefix_sum.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/lilypad.raster.hlsl` | [reconstruction/shader-source/shaders/lilypad.raster.hlsl](../../reconstruction/shader-source/shaders/lilypad.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/lilypad_flower.raster.hlsl` | [reconstruction/shader-source/shaders/lilypad_flower.raster.hlsl](../../reconstruction/shader-source/shaders/lilypad_flower.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/loading_screen.raster.hlsl` | [reconstruction/shader-source/shaders/loading_screen.raster.hlsl](../../reconstruction/shader-source/shaders/loading_screen.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/logo_animation.raster.hlsl` | [reconstruction/shader-source/shaders/logo_animation.raster.hlsl](../../reconstruction/shader-source/shaders/logo_animation.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/logo_branches_animation.raster.hlsl` | [reconstruction/shader-source/shaders/logo_branches_animation.raster.hlsl](../../reconstruction/shader-source/shaders/logo_branches_animation.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/logo_overlay.raster.hlsl` | [reconstruction/shader-source/shaders/logo_overlay.raster.hlsl](../../reconstruction/shader-source/shaders/logo_overlay.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/lut/brdf_fg.cs.hlsl` | [reconstruction/shader-source/shaders/lut/brdf_fg.cs.hlsl](../../reconstruction/shader-source/shaders/lut/brdf_fg.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/lut_gallery/thumbnail.raster.hlsl` | [reconstruction/shader-source/shaders/lut_gallery/thumbnail.raster.hlsl](../../reconstruction/shader-source/shaders/lut_gallery/thumbnail.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/lut/water_noise.cs.hlsl` | [reconstruction/shader-source/shaders/lut/water_noise.cs.hlsl](../../reconstruction/shader-source/shaders/lut/water_noise.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/mei_shadow.raster.hlsl` | [reconstruction/shader-source/shaders/mei_shadow.raster.hlsl](../../reconstruction/shader-source/shaders/mei_shadow.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/mouse_terrain_raycast.cs.hlsl` | [reconstruction/shader-source/shaders/mouse_terrain_raycast.cs.hlsl](../../reconstruction/shader-source/shaders/mouse_terrain_raycast.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/recovery/f-t/mouse_terrain_raycast.cs.hlsl.report.json](../../evidence/render-passes/recovery/f-t/mouse_terrain_raycast.cs.hlsl.report.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/allocate_draw_commands.cs.hlsl` | [reconstruction/shader-source/shaders/nani/allocate_draw_commands.cs.hlsl](../../reconstruction/shader-source/shaders/nani/allocate_draw_commands.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_allocate_draw_commands.json](../../evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_allocate_draw_commands.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/draw_lists.inc.hlsl` | [reconstruction/shader-source/shaders/nani/draw_lists.inc.hlsl](../../reconstruction/shader-source/shaders/nani/draw_lists.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/draw_list_offsets.inc.hlsl` | [reconstruction/shader-source/shaders/nani/draw_list_offsets.inc.hlsl](../../reconstruction/shader-source/shaders/nani/draw_list_offsets.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani_balcony_door.raster.hlsl` | [reconstruction/shader-source/shaders/nani_balcony_door.raster.hlsl](../../reconstruction/shader-source/shaders/nani_balcony_door.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/window_color_customization.hlsl` | [reconstruction/shader-source/shaders/inc/window_color_customization.hlsl](../../reconstruction/shader-source/shaders/inc/window_color_customization.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani_bush.raster.hlsl` | [reconstruction/shader-source/shaders/nani_bush.raster.hlsl](../../reconstruction/shader-source/shaders/nani_bush.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/clear_draw_lists.cs.hlsl` | [reconstruction/shader-source/shaders/nani/clear_draw_lists.cs.hlsl](../../reconstruction/shader-source/shaders/nani/clear_draw_lists.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_clear_draw_lists.json](../../evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_clear_draw_lists.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/clear_histogram.cs.hlsl` | [reconstruction/shader-source/shaders/nani/clear_histogram.cs.hlsl](../../reconstruction/shader-source/shaders/nani/clear_histogram.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_clear_histogram.json](../../evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_clear_histogram.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 1, 'fail': 0}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/clear_subtotals.cs.hlsl` | [reconstruction/shader-source/shaders/nani/clear_subtotals.cs.hlsl](../../reconstruction/shader-source/shaders/nani/clear_subtotals.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_clear_subtotals.json](../../evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_clear_subtotals.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 1, 'fail': 0}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani_cloth.raster.hlsl` | [reconstruction/shader-source/shaders/nani_cloth.raster.hlsl](../../reconstruction/shader-source/shaders/nani_cloth.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/hsv.hlsl` | [reconstruction/shader-source/shaders/inc/hsv.hlsl](../../reconstruction/shader-source/shaders/inc/hsv.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/cull_and_bucket.cs.hlsl` | [reconstruction/shader-source/shaders/nani/cull_and_bucket.cs.hlsl](../../reconstruction/shader-source/shaders/nani/cull_and_bucket.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_cull_and_bucket_8a7d.json](../../evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_cull_and_bucket_8a7d.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/cull_and_bucket.inc.hlsl` | [reconstruction/shader-source/shaders/nani/cull_and_bucket.inc.hlsl](../../reconstruction/shader-source/shaders/nani/cull_and_bucket.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/atlas_part_info.inc.hlsl` | [reconstruction/shader-source/shaders/nani/atlas_part_info.inc.hlsl](../../reconstruction/shader-source/shaders/nani/atlas_part_info.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/cull_and_bucket_bush.cs.hlsl` | [reconstruction/shader-source/shaders/nani/cull_and_bucket_bush.cs.hlsl](../../reconstruction/shader-source/shaders/nani/cull_and_bucket_bush.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_cull_and_bucket_bush_8a7d.json](../../evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_cull_and_bucket_bush_8a7d.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/inc/cull_and_bucket_owned_custom.hlsl` | [reconstruction/shader-source/shaders/nani/inc/cull_and_bucket_owned_custom.hlsl](../../reconstruction/shader-source/shaders/nani/inc/cull_and_bucket_owned_custom.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/cull_and_bucket_curved_halftimber.cs.hlsl` | [reconstruction/shader-source/shaders/nani/cull_and_bucket_curved_halftimber.cs.hlsl](../../reconstruction/shader-source/shaders/nani/cull_and_bucket_curved_halftimber.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_cull_and_bucket_curved_halftimber_8a7d.json](../../evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_cull_and_bucket_curved_halftimber_8a7d.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/buildings/curved_halftimber.inc.hlsl` | [reconstruction/shader-source/shaders/buildings/curved_halftimber.inc.hlsl](../../reconstruction/shader-source/shaders/buildings/curved_halftimber.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/cull_and_bucket_flowers.cs.hlsl` | [reconstruction/shader-source/shaders/nani/cull_and_bucket_flowers.cs.hlsl](../../reconstruction/shader-source/shaders/nani/cull_and_bucket_flowers.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_cull_and_bucket_flowers_8a7d.json](../../evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_cull_and_bucket_flowers_8a7d.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/cull_and_bucket_plaster.cs.hlsl` | [reconstruction/shader-source/shaders/nani/cull_and_bucket_plaster.cs.hlsl](../../reconstruction/shader-source/shaders/nani/cull_and_bucket_plaster.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_cull_and_bucket_plaster_8a7d.json](../../evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_cull_and_bucket_plaster_8a7d.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/plaster.inc.hlsl` | [reconstruction/shader-source/shaders/plaster.inc.hlsl](../../reconstruction/shader-source/shaders/plaster.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/cull_and_bucket_roof_tile.cs.hlsl` | [reconstruction/shader-source/shaders/nani/cull_and_bucket_roof_tile.cs.hlsl](../../reconstruction/shader-source/shaders/nani/cull_and_bucket_roof_tile.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_cull_and_bucket_roof_tile_8a7d.json](../../evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_cull_and_bucket_roof_tile_8a7d.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/roof/roof_tile.inc.hlsl` | [reconstruction/shader-source/shaders/roof/roof_tile.inc.hlsl](../../reconstruction/shader-source/shaders/roof/roof_tile.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/cull_and_bucket_smoke.cs.hlsl` | [reconstruction/shader-source/shaders/nani/cull_and_bucket_smoke.cs.hlsl](../../reconstruction/shader-source/shaders/nani/cull_and_bucket_smoke.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_cull_and_bucket_smoke_8a7d.json](../../evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_cull_and_bucket_smoke_8a7d.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/smoke.inc.hlsl` | [reconstruction/shader-source/shaders/smoke.inc.hlsl](../../reconstruction/shader-source/shaders/smoke.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/cull_and_bucket_stairs_floor.cs.hlsl` | [reconstruction/shader-source/shaders/nani/cull_and_bucket_stairs_floor.cs.hlsl](../../reconstruction/shader-source/shaders/nani/cull_and_bucket_stairs_floor.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_cull_and_bucket_stairs_floor_8a7d.json](../../evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_cull_and_bucket_stairs_floor_8a7d.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/stairs_floor.inc.hlsl` | [reconstruction/shader-source/shaders/stairs_floor.inc.hlsl](../../reconstruction/shader-source/shaders/stairs_floor.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/inc/trimesh.hlsl` | [reconstruction/shader-source/shaders/nani/inc/trimesh.hlsl](../../reconstruction/shader-source/shaders/nani/inc/trimesh.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/cull_and_bucket_wooden_plank.cs.hlsl` | [reconstruction/shader-source/shaders/nani/cull_and_bucket_wooden_plank.cs.hlsl](../../reconstruction/shader-source/shaders/nani/cull_and_bucket_wooden_plank.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_cull_and_bucket_wooden_plank_8a7d.json](../../evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_cull_and_bucket_wooden_plank_8a7d.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/wooden_plank.inc.hlsl` | [reconstruction/shader-source/shaders/wooden_plank.inc.hlsl](../../reconstruction/shader-source/shaders/wooden_plank.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani_curved_halftimber.raster.hlsl` | [reconstruction/shader-source/shaders/nani_curved_halftimber.raster.hlsl](../../reconstruction/shader-source/shaders/nani_curved_halftimber.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/parallax_mapping.hlsl` | [reconstruction/shader-source/shaders/inc/parallax_mapping.hlsl](../../reconstruction/shader-source/shaders/inc/parallax_mapping.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/draw_indexed_indirect_to_dispatch.cs.hlsl` | [reconstruction/shader-source/shaders/nani/draw_indexed_indirect_to_dispatch.cs.hlsl](../../reconstruction/shader-source/shaders/nani/draw_indexed_indirect_to_dispatch.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_draw_indexed_indirect_to_dispatch.json](../../evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_draw_indexed_indirect_to_dispatch.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 1, 'fail': 0}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/fill_sorted_instance_list.cs.hlsl` | [reconstruction/shader-source/shaders/nani/fill_sorted_instance_list.cs.hlsl](../../reconstruction/shader-source/shaders/nani/fill_sorted_instance_list.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_fill_sorted_instance_list.json](../../evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_fill_sorted_instance_list.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/fill_unsorted_instance_indirection.cs.hlsl` | [reconstruction/shader-source/shaders/nani/fill_unsorted_instance_indirection.cs.hlsl](../../reconstruction/shader-source/shaders/nani/fill_unsorted_instance_indirection.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_fill_unsorted_instance_indirection.json](../../evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_fill_unsorted_instance_indirection.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 1, 'fail': 0}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/fill_unsorted_instance_list.cs.hlsl` | [reconstruction/shader-source/shaders/nani/fill_unsorted_instance_list.cs.hlsl](../../reconstruction/shader-source/shaders/nani/fill_unsorted_instance_list.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_fill_unsorted_instance_list.json](../../evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_fill_unsorted_instance_list.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani_flower.raster.hlsl` | [reconstruction/shader-source/shaders/nani_flower.raster.hlsl](../../reconstruction/shader-source/shaders/nani_flower.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/generate_dispatch_args.cs.hlsl` | [reconstruction/shader-source/shaders/nani/generate_dispatch_args.cs.hlsl](../../reconstruction/shader-source/shaders/nani/generate_dispatch_args.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_generate_dispatch_args.json](../../evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_generate_dispatch_args.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/generate_draw_lists.cs.hlsl` | [reconstruction/shader-source/shaders/nani/generate_draw_lists.cs.hlsl](../../reconstruction/shader-source/shaders/nani/generate_draw_lists.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_generate_draw_lists.json](../../evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_generate_draw_lists.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani_gothic_window_bricks.raster.hlsl` | [reconstruction/shader-source/shaders/nani_gothic_window_bricks.raster.hlsl](../../reconstruction/shader-source/shaders/nani_gothic_window_bricks.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/wall/inc/brick_vs_out.hlsl` | [reconstruction/shader-source/shaders/wall/inc/brick_vs_out.hlsl](../../reconstruction/shader-source/shaders/wall/inc/brick_vs_out.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/wall/inc/brick_instance_flags.hlsl` | [reconstruction/shader-source/shaders/wall/inc/brick_instance_flags.hlsl](../../reconstruction/shader-source/shaders/wall/inc/brick_instance_flags.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/wall/inc/brick_frag.hlsl` | [reconstruction/shader-source/shaders/wall/inc/brick_frag.hlsl](../../reconstruction/shader-source/shaders/wall/inc/brick_frag.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/wall_color_customization.hlsl` | [reconstruction/shader-source/shaders/inc/wall_color_customization.hlsl](../../reconstruction/shader-source/shaders/inc/wall_color_customization.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/roof_colors_include.hlsl` | [reconstruction/shader-source/shaders/inc/roof_colors_include.hlsl](../../reconstruction/shader-source/shaders/inc/roof_colors_include.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/wall/inc/highlight_mode.hlsl` | [reconstruction/shader-source/shaders/wall/inc/highlight_mode.hlsl](../../reconstruction/shader-source/shaders/wall/inc/highlight_mode.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/group_draw_cmd_prefix_sum.cs.hlsl` | [reconstruction/shader-source/shaders/nani/group_draw_cmd_prefix_sum.cs.hlsl](../../reconstruction/shader-source/shaders/nani/group_draw_cmd_prefix_sum.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_group_draw_cmd_prefix_sum.json](../../evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_group_draw_cmd_prefix_sum.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/histogram_prefix_sum.cs.hlsl` | [reconstruction/shader-source/shaders/nani/histogram_prefix_sum.cs.hlsl](../../reconstruction/shader-source/shaders/nani/histogram_prefix_sum.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_histogram_prefix_sum.json](../../evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_histogram_prefix_sum.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani_instanced_roof.raster.hlsl` | [reconstruction/shader-source/shaders/nani_instanced_roof.raster.hlsl](../../reconstruction/shader-source/shaders/nani_instanced_roof.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/roof/dormer_hole.inc.hlsl` | [reconstruction/shader-source/shaders/roof/dormer_hole.inc.hlsl](../../reconstruction/shader-source/shaders/roof/dormer_hole.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani_lantern_emissive.raster.hlsl` | [reconstruction/shader-source/shaders/nani_lantern_emissive.raster.hlsl](../../reconstruction/shader-source/shaders/nani_lantern_emissive.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani_plant.raster.hlsl` | [reconstruction/shader-source/shaders/nani_plant.raster.hlsl](../../reconstruction/shader-source/shaders/nani_plant.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani_plaster.raster.hlsl` | [reconstruction/shader-source/shaders/nani_plaster.raster.hlsl](../../reconstruction/shader-source/shaders/nani_plaster.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/prefix_sum_unsorted_instances_per_atlas_entry.cs.hlsl` | [reconstruction/shader-source/shaders/nani/prefix_sum_unsorted_instances_per_atlas_entry.cs.hlsl](../../reconstruction/shader-source/shaders/nani/prefix_sum_unsorted_instances_per_atlas_entry.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_prefix_sum_unsorted_instances_per_atlas_entry.json](../../evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_prefix_sum_unsorted_instances_per_atlas_entry.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/prepare_unsorted_draw_lists.cs.hlsl` | [reconstruction/shader-source/shaders/nani/prepare_unsorted_draw_lists.cs.hlsl](../../reconstruction/shader-source/shaders/nani/prepare_unsorted_draw_lists.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_prepare_unsorted_draw_lists.json](../../evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_prepare_unsorted_draw_lists.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani_preplaced_flower.raster.hlsl` | [reconstruction/shader-source/shaders/nani_preplaced_flower.raster.hlsl](../../reconstruction/shader-source/shaders/nani_preplaced_flower.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani_roof_floor.raster.hlsl` | [reconstruction/shader-source/shaders/nani_roof_floor.raster.hlsl](../../reconstruction/shader-source/shaders/nani_roof_floor.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/roof_material_id.inc.hlsl` | [reconstruction/shader-source/shaders/roof_material_id.inc.hlsl](../../reconstruction/shader-source/shaders/roof_material_id.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/roof/wooden_floor.inc.hlsl` | [reconstruction/shader-source/shaders/roof/wooden_floor.inc.hlsl](../../reconstruction/shader-source/shaders/roof/wooden_floor.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/roof/stone_floor.inc.hlsl` | [reconstruction/shader-source/shaders/roof/stone_floor.inc.hlsl](../../reconstruction/shader-source/shaders/roof/stone_floor.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani_smoke.raster.hlsl` | [reconstruction/shader-source/shaders/nani_smoke.raster.hlsl](../../reconstruction/shader-source/shaders/nani_smoke.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani_solid_albedo_map.raster.hlsl` | [reconstruction/shader-source/shaders/nani_solid_albedo_map.raster.hlsl](../../reconstruction/shader-source/shaders/nani_solid_albedo_map.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani_solid_albedo_map_emissive.raster.hlsl` | [reconstruction/shader-source/shaders/nani_solid_albedo_map_emissive.raster.hlsl](../../reconstruction/shader-source/shaders/nani_solid_albedo_map_emissive.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani_solid_vertex_color.raster.hlsl` | [reconstruction/shader-source/shaders/nani_solid_vertex_color.raster.hlsl](../../reconstruction/shader-source/shaders/nani_solid_vertex_color.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani_solid_vertex_color_bricks.raster.hlsl` | [reconstruction/shader-source/shaders/nani_solid_vertex_color_bricks.raster.hlsl](../../reconstruction/shader-source/shaders/nani_solid_vertex_color_bricks.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani_solid_vertex_color_window.raster.hlsl` | [reconstruction/shader-source/shaders/nani_solid_vertex_color_window.raster.hlsl](../../reconstruction/shader-source/shaders/nani_solid_vertex_color_window.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani_solid_vertex_color_window_glass.raster.hlsl` | [reconstruction/shader-source/shaders/nani_solid_vertex_color_window_glass.raster.hlsl](../../reconstruction/shader-source/shaders/nani_solid_vertex_color_window_glass.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani_stairs_floor.raster.hlsl` | [reconstruction/shader-source/shaders/nani_stairs_floor.raster.hlsl](../../reconstruction/shader-source/shaders/nani_stairs_floor.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani/subtotal_prefix_sum.cs.hlsl` | [reconstruction/shader-source/shaders/nani/subtotal_prefix_sum.cs.hlsl](../../reconstruction/shader-source/shaders/nani/subtotal_prefix_sum.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_subtotal_prefix_sum.json](../../evidence/render-passes/integration/wave2-nani-cull-merge/reports/nani_subtotal_prefix_sum.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani_uber.raster.hlsl` | [reconstruction/shader-source/shaders/nani_uber.raster.hlsl](../../reconstruction/shader-source/shaders/nani_uber.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani_velocity_only.raster.hlsl` | [reconstruction/shader-source/shaders/nani_velocity_only.raster.hlsl](../../reconstruction/shader-source/shaders/nani_velocity_only.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/nani_wooden_plank.raster.hlsl` | [reconstruction/shader-source/shaders/nani_wooden_plank.raster.hlsl](../../reconstruction/shader-source/shaders/nani_wooden_plank.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/normal_debug.raster.hlsl` | [reconstruction/shader-source/shaders/normal_debug.raster.hlsl](../../reconstruction/shader-source/shaders/normal_debug.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/painterly/acquire_target.cs.hlsl` | [reconstruction/shader-source/shaders/painterly/acquire_target.cs.hlsl](../../reconstruction/shader-source/shaders/painterly/acquire_target.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/recovery/final-post-painterly/_painterly_acquire_target.cs.hlsl_cs_main_0.json](../../evidence/render-passes/recovery/final-post-painterly/_painterly_acquire_target.cs.hlsl_cs_main_0.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/painterly/convolve_target.cs.hlsl` | [reconstruction/shader-source/shaders/painterly/convolve_target.cs.hlsl](../../reconstruction/shader-source/shaders/painterly/convolve_target.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/recovery/final-post-painterly/_painterly_convolve_target.cs.hlsl_cs_main_0.json](../../evidence/render-passes/recovery/final-post-painterly/_painterly_convolve_target.cs.hlsl_cs_main_0.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/painterly/debug_target.raster.hlsl` | [reconstruction/shader-source/shaders/painterly/debug_target.raster.hlsl](../../reconstruction/shader-source/shaders/painterly/debug_target.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | [evidence/render-passes/recovery/final-post-painterly/_painterly_debug_target.raster.hlsl_b903e43ffb3da915.json](../../evidence/render-passes/recovery/final-post-painterly/_painterly_debug_target.raster.hlsl_b903e43ffb3da915.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/painterly/generate_strokes.cs.hlsl` | [reconstruction/shader-source/shaders/painterly/generate_strokes.cs.hlsl](../../reconstruction/shader-source/shaders/painterly/generate_strokes.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/recovery/final-post-painterly/_painterly_generate_strokes.cs.hlsl_cs_main_0.json](../../evidence/render-passes/recovery/final-post-painterly/_painterly_generate_strokes.cs.hlsl_cs_main_0.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/painterly/stroke.inc.hlsl` | [reconstruction/shader-source/shaders/painterly/stroke.inc.hlsl](../../reconstruction/shader-source/shaders/painterly/stroke.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/painterly/raster_strokes.raster.hlsl` | [reconstruction/shader-source/shaders/painterly/raster_strokes.raster.hlsl](../../reconstruction/shader-source/shaders/painterly/raster_strokes.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | [evidence/render-passes/recovery/final-post-painterly/_painterly_raster_strokes.raster.hlsl_b903e43ffb3da915.json](../../evidence/render-passes/recovery/final-post-painterly/_painterly_raster_strokes.raster.hlsl_b903e43ffb3da915.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/path_pebble.raster.hlsl` | [reconstruction/shader-source/shaders/path_pebble.raster.hlsl](../../reconstruction/shader-source/shaders/path_pebble.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | [evidence/render-passes/integration/f-r-recheck/_path_pebble.hlsl_b903e43ffb3da915_recheck.json](../../evidence/render-passes/integration/f-r-recheck/_path_pebble.hlsl_b903e43ffb3da915_recheck.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>[evidence/render-passes/integration/f-r-recheck/_path_pebble.hlsl_f0adff76827e73ff_recheck.json](../../evidence/render-passes/integration/f-r-recheck/_path_pebble.hlsl_f0adff76827e73ff_recheck.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>[evidence/render-passes/recovery/f-r/path_pebble_f0ad_report.json](../../evidence/render-passes/recovery/f-r/path_pebble_f0ad_report.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>[evidence/render-passes/recovery/f-r/path_pebble_report.json](../../evidence/render-passes/recovery/f-r/path_pebble_report.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/path_pebbles.cs.hlsl` | [reconstruction/shader-source/shaders/path_pebbles.cs.hlsl](../../reconstruction/shader-source/shaders/path_pebbles.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/f-r-recheck/_path_pebbles.cs.hlsl_recheck.json](../../evidence/render-passes/integration/f-r-recheck/_path_pebbles.cs.hlsl_recheck.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>[evidence/render-passes/recovery/f-r/path_pebbles_report.json](../../evidence/render-passes/recovery/f-r/path_pebbles_report.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/paths/combine_path_exclusions.cs.hlsl` | [reconstruction/shader-source/shaders/paths/combine_path_exclusions.cs.hlsl](../../reconstruction/shader-source/shaders/paths/combine_path_exclusions.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/f-r-recheck/_paths_combine_path_exclusions.cs.hlsl_recheck.json](../../evidence/render-passes/integration/f-r-recheck/_paths_combine_path_exclusions.cs.hlsl_recheck.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>[evidence/render-passes/recovery/f-r/combine_path_exclusions_report.json](../../evidence/render-passes/recovery/f-r/combine_path_exclusions_report.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/paths/directional_contract.cs.hlsl` | [reconstruction/shader-source/shaders/paths/directional_contract.cs.hlsl](../../reconstruction/shader-source/shaders/paths/directional_contract.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/f-r-recheck/_paths_directional_contract.cs.hlsl_recheck.json](../../evidence/render-passes/integration/f-r-recheck/_paths_directional_contract.cs.hlsl_recheck.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>[evidence/render-passes/recovery/f-r/directional_contract_report.json](../../evidence/render-passes/recovery/f-r/directional_contract_report.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/paths/path_mask_visual.cs.hlsl` | [reconstruction/shader-source/shaders/paths/path_mask_visual.cs.hlsl](../../reconstruction/shader-source/shaders/paths/path_mask_visual.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/f-r-recheck/_paths_path_mask_visual.cs.hlsl_recheck.json](../../evidence/render-passes/integration/f-r-recheck/_paths_path_mask_visual.cs.hlsl_recheck.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>[evidence/render-passes/recovery/f-r/path_mask_visual_report.json](../../evidence/render-passes/recovery/f-r/path_mask_visual_report.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/prefix_scan/inclusive_prefix_scan.cs.hlsl` | [reconstruction/shader-source/shaders/prefix_scan/inclusive_prefix_scan.cs.hlsl](../../reconstruction/shader-source/shaders/prefix_scan/inclusive_prefix_scan.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/prefix_scan/inclusive_prefix_scan_merge.cs.hlsl` | [reconstruction/shader-source/shaders/prefix_scan/inclusive_prefix_scan_merge.cs.hlsl](../../reconstruction/shader-source/shaders/prefix_scan/inclusive_prefix_scan_merge.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/prefix_scan/inclusive_prefix_scan_subtotals.cs.hlsl` | [reconstruction/shader-source/shaders/prefix_scan/inclusive_prefix_scan_subtotals.cs.hlsl](../../reconstruction/shader-source/shaders/prefix_scan/inclusive_prefix_scan_subtotals.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/primary_rt_debug.cs.hlsl` | [reconstruction/shader-source/shaders/primary_rt_debug.cs.hlsl](../../reconstruction/shader-source/shaders/primary_rt_debug.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/rain_particles.raster.hlsl` | [reconstruction/shader-source/shaders/rain_particles.raster.hlsl](../../reconstruction/shader-source/shaders/rain_particles.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | [evidence/render-passes/recovery/f-v/rain_particles.r1.json](../../evidence/render-passes/recovery/f-v/rain_particles.r1.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>[evidence/render-passes/recovery/f-v/rain_particles.r2.json](../../evidence/render-passes/recovery/f-v/rain_particles.r2.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>[evidence/render-passes/recovery/f-v/rain_particles.r3.json](../../evidence/render-passes/recovery/f-v/rain_particles.r3.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/rasterize_buildings.cs.hlsl` | [reconstruction/shader-source/shaders/rasterize_buildings.cs.hlsl](../../reconstruction/shader-source/shaders/rasterize_buildings.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/recovery/f-t/rasterize_buildings.cs.hlsl.report.json](../../evidence/render-passes/recovery/f-t/rasterize_buildings.cs.hlsl.report.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/rasterize_buildings_animate.cs.hlsl` | [reconstruction/shader-source/shaders/rasterize_buildings_animate.cs.hlsl](../../reconstruction/shader-source/shaders/rasterize_buildings_animate.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/recovery/f-t/rasterize_buildings_animate.cs.hlsl.report.json](../../evidence/render-passes/recovery/f-t/rasterize_buildings_animate.cs.hlsl.report.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/rdi/temporal.cs.hlsl` | [reconstruction/shader-source/shaders/rdi/temporal.cs.hlsl](../../reconstruction/shader-source/shaders/rdi/temporal.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/read_mouse_depth_value.cs.hlsl` | [reconstruction/shader-source/shaders/read_mouse_depth_value.cs.hlsl](../../reconstruction/shader-source/shaders/read_mouse_depth_value.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/read_terrain_stroke.cs.hlsl` | [reconstruction/shader-source/shaders/read_terrain_stroke.cs.hlsl](../../reconstruction/shader-source/shaders/read_terrain_stroke.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/reed.raster.hlsl` | [reconstruction/shader-source/shaders/reed.raster.hlsl](../../reconstruction/shader-source/shaders/reed.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/resolve_to_swapchain.cs.hlsl` | [reconstruction/shader-source/shaders/resolve_to_swapchain.cs.hlsl](../../reconstruction/shader-source/shaders/resolve_to_swapchain.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/rocky_terrain/displace_rocky_terrain.cs.hlsl` | [reconstruction/shader-source/shaders/rocky_terrain/displace_rocky_terrain.cs.hlsl](../../reconstruction/shader-source/shaders/rocky_terrain/displace_rocky_terrain.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/f-r-recheck/_rocky_terrain_displace_rocky_terrain.cs.hlsl_recheck.json](../../evidence/render-passes/integration/f-r-recheck/_rocky_terrain_displace_rocky_terrain.cs.hlsl_recheck.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>[evidence/render-passes/recovery/f-r/displace_rocky_terrain_report.json](../../evidence/render-passes/recovery/f-r/displace_rocky_terrain_report.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/rocky_terrain/inc_mesh.inc.hlsl` | [reconstruction/shader-source/shaders/rocky_terrain/inc_mesh.inc.hlsl](../../reconstruction/shader-source/shaders/rocky_terrain/inc_mesh.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/rocky_terrain/rocky_terrain.raster.hlsl` | [reconstruction/shader-source/shaders/rocky_terrain/rocky_terrain.raster.hlsl](../../reconstruction/shader-source/shaders/rocky_terrain/rocky_terrain.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | [evidence/render-passes/integration/f-r-recheck/_rocky_terrain_rocky_terrain.hlsl_b903e43ffb3da915_recheck.json](../../evidence/render-passes/integration/f-r-recheck/_rocky_terrain_rocky_terrain.hlsl_b903e43ffb3da915_recheck.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>[evidence/render-passes/integration/f-r-recheck/_rocky_terrain_rocky_terrain.hlsl_f0adff76827e73ff_recheck.json](../../evidence/render-passes/integration/f-r-recheck/_rocky_terrain_rocky_terrain.hlsl_f0adff76827e73ff_recheck.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>[evidence/render-passes/recovery/f-r/rocky_terrain_f0ad_report.json](../../evidence/render-passes/recovery/f-r/rocky_terrain_f0ad_report.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>[evidence/render-passes/recovery/f-r/rocky_terrain_report.json](../../evidence/render-passes/recovery/f-r/rocky_terrain_report.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/rocky_terrain/triangle_adjacency.inc.hlsl` | [reconstruction/shader-source/shaders/rocky_terrain/triangle_adjacency.inc.hlsl](../../reconstruction/shader-source/shaders/rocky_terrain/triangle_adjacency.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/rocky_terrain/stairs/stairs.cs.hlsl` | [reconstruction/shader-source/shaders/rocky_terrain/stairs/stairs.cs.hlsl](../../reconstruction/shader-source/shaders/rocky_terrain/stairs/stairs.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/f-r-recheck/_rocky_terrain_stairs_stairs.cs.hlsl_recheck.json](../../evidence/render-passes/integration/f-r-recheck/_rocky_terrain_stairs_stairs.cs.hlsl_recheck.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>[evidence/render-passes/recovery/f-r/stairs_report.json](../../evidence/render-passes/recovery/f-r/stairs_report.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/rocky_terrain/stairs/place_stairs.inc.hlsl` | [reconstruction/shader-source/shaders/rocky_terrain/stairs/place_stairs.inc.hlsl](../../reconstruction/shader-source/shaders/rocky_terrain/stairs/place_stairs.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/rocky_terrain/stairs/marching_squares.inc.hlsl` | [reconstruction/shader-source/shaders/rocky_terrain/stairs/marching_squares.inc.hlsl](../../reconstruction/shader-source/shaders/rocky_terrain/stairs/marching_squares.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/rocky_terrain/stairs/isolines.inc.hlsl` | [reconstruction/shader-source/shaders/rocky_terrain/stairs/isolines.inc.hlsl](../../reconstruction/shader-source/shaders/rocky_terrain/stairs/isolines.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/rocky_terrain/stairs/stairs_counter.cs.hlsl` | [reconstruction/shader-source/shaders/rocky_terrain/stairs/stairs_counter.cs.hlsl](../../reconstruction/shader-source/shaders/rocky_terrain/stairs/stairs_counter.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/f-r-recheck/_rocky_terrain_stairs_stairs_counter.cs.hlsl_recheck.json](../../evidence/render-passes/integration/f-r-recheck/_rocky_terrain_stairs_stairs_counter.cs.hlsl_recheck.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>[evidence/render-passes/recovery/f-r/stairs_counter_report.json](../../evidence/render-passes/recovery/f-r/stairs_counter_report.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/rocky_terrain/stairs/stairs_indirect.raster.hlsl` | [reconstruction/shader-source/shaders/rocky_terrain/stairs/stairs_indirect.raster.hlsl](../../reconstruction/shader-source/shaders/rocky_terrain/stairs/stairs_indirect.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | [evidence/render-passes/integration/f-r-recheck/_rocky_terrain_stairs_stairs_indirect.hlsl_b903e43ffb3da915_recheck.json](../../evidence/render-passes/integration/f-r-recheck/_rocky_terrain_stairs_stairs_indirect.hlsl_b903e43ffb3da915_recheck.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>[evidence/render-passes/integration/f-r-recheck/_rocky_terrain_stairs_stairs_indirect.hlsl_f0adff76827e73ff_recheck.json](../../evidence/render-passes/integration/f-r-recheck/_rocky_terrain_stairs_stairs_indirect.hlsl_f0adff76827e73ff_recheck.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>[evidence/render-passes/recovery/f-r/stairs_indirect_f0ad_report.json](../../evidence/render-passes/recovery/f-r/stairs_indirect_f0ad_report.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>[evidence/render-passes/recovery/f-r/stairs_indirect_report.json](../../evidence/render-passes/recovery/f-r/stairs_indirect_report.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/roof_snow.raster.hlsl` | [reconstruction/shader-source/shaders/roof_snow.raster.hlsl](../../reconstruction/shader-source/shaders/roof_snow.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/roof_tiles_gravity.cs.hlsl` | [reconstruction/shader-source/shaders/roof_tiles_gravity.cs.hlsl](../../reconstruction/shader-source/shaders/roof_tiles_gravity.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/recovery/f-v/roof_tiles_gravity.r1.json](../../evidence/render-passes/recovery/f-v/roof_tiles_gravity.r1.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>[evidence/render-passes/recovery/f-v/roof_tiles_gravity.r2.json](../../evidence/render-passes/recovery/f-v/roof_tiles_gravity.r2.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>[evidence/render-passes/recovery/f-v/roof_tiles_gravity.r3.json](../../evidence/render-passes/recovery/f-v/roof_tiles_gravity.r3.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/screenshot/capture_screenshot_buffer.cs.hlsl` | [reconstruction/shader-source/shaders/screenshot/capture_screenshot_buffer.cs.hlsl](../../reconstruction/shader-source/shaders/screenshot/capture_screenshot_buffer.cs.hlsl) 存在 | sample1/compute, sample4/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/screenshot/capture_screenshot.inc.hlsl` | [reconstruction/shader-source/shaders/screenshot/capture_screenshot.inc.hlsl](../../reconstruction/shader-source/shaders/screenshot/capture_screenshot.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/screenshot/capture_screenshot_texture.cs.hlsl` | [reconstruction/shader-source/shaders/screenshot/capture_screenshot_texture.cs.hlsl](../../reconstruction/shader-source/shaders/screenshot/capture_screenshot_texture.cs.hlsl) 存在 | sample1/compute, sample4/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/session_selection/background.raster.hlsl` | [reconstruction/shader-source/shaders/session_selection/background.raster.hlsl](../../reconstruction/shader-source/shaders/session_selection/background.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/session_selection/new_glade_thumbnail.raster.hlsl` | [reconstruction/shader-source/shaders/session_selection/new_glade_thumbnail.raster.hlsl](../../reconstruction/shader-source/shaders/session_selection/new_glade_thumbnail.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/session_selection/thumbnail.raster.hlsl` | [reconstruction/shader-source/shaders/session_selection/thumbnail.raster.hlsl](../../reconstruction/shader-source/shaders/session_selection/thumbnail.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/session_selection/thumbnail_painterly.raster.hlsl` | [reconstruction/shader-source/shaders/session_selection/thumbnail_painterly.raster.hlsl](../../reconstruction/shader-source/shaders/session_selection/thumbnail_painterly.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/shader_refactoring.cs.hlsl` | [reconstruction/shader-source/shaders/shader_refactoring.cs.hlsl](../../reconstruction/shader-source/shaders/shader_refactoring.cs.hlsl) 存在 | capture/compute, compare/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/shadow_mask_mean.cs.hlsl` | [reconstruction/shader-source/shaders/shadow_mask_mean.cs.hlsl](../../reconstruction/shader-source/shaders/shadow_mask_mean.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/sheep.raster.hlsl` | [reconstruction/shader-source/shaders/sheep.raster.hlsl](../../reconstruction/shader-source/shaders/sheep.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | [evidence/render-passes/recovery/f-v/sheep.r1.json](../../evidence/render-passes/recovery/f-v/sheep.r1.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>[evidence/render-passes/recovery/f-v/sheep.r2.json](../../evidence/render-passes/recovery/f-v/sheep.r2.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>[evidence/render-passes/recovery/f-v/sheep.r3.json](../../evidence/render-passes/recovery/f-v/sheep.r3.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/skydome.raster.hlsl` | [reconstruction/shader-source/shaders/skydome.raster.hlsl](../../reconstruction/shader-source/shaders/skydome.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/snow_particles.raster.hlsl` | [reconstruction/shader-source/shaders/snow_particles.raster.hlsl](../../reconstruction/shader-source/shaders/snow_particles.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/solid_vertex_color.raster.hlsl` | [reconstruction/shader-source/shaders/solid_vertex_color.raster.hlsl](../../reconstruction/shader-source/shaders/solid_vertex_color.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/solid_vertex_color_albedo_map.raster.hlsl` | [reconstruction/shader-source/shaders/solid_vertex_color_albedo_map.raster.hlsl](../../reconstruction/shader-source/shaders/solid_vertex_color_albedo_map.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/solid_vertex_color_boat.raster.hlsl` | [reconstruction/shader-source/shaders/solid_vertex_color_boat.raster.hlsl](../../reconstruction/shader-source/shaders/solid_vertex_color_boat.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/solid_vertex_color_garden_stone.raster.hlsl` | [reconstruction/shader-source/shaders/solid_vertex_color_garden_stone.raster.hlsl](../../reconstruction/shader-source/shaders/solid_vertex_color_garden_stone.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/solid_vertex_color_waterwheel.raster.hlsl` | [reconstruction/shader-source/shaders/solid_vertex_color_waterwheel.raster.hlsl](../../reconstruction/shader-source/shaders/solid_vertex_color_waterwheel.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/solid_vertex_color_wooden_gate.raster.hlsl` | [reconstruction/shader-source/shaders/solid_vertex_color_wooden_gate.raster.hlsl](../../reconstruction/shader-source/shaders/solid_vertex_color_wooden_gate.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/sparkles.raster.hlsl` | [reconstruction/shader-source/shaders/sparkles.raster.hlsl](../../reconstruction/shader-source/shaders/sparkles.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/splash_particle.raster.hlsl` | [reconstruction/shader-source/shaders/splash_particle.raster.hlsl](../../reconstruction/shader-source/shaders/splash_particle.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/ssgi/ssgi.cs.hlsl` | [reconstruction/shader-source/shaders/ssgi/ssgi.cs.hlsl](../../reconstruction/shader-source/shaders/ssgi/ssgi.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/ssgi/XeGTAO.hlsl` | [reconstruction/shader-source/shaders/ssgi/XeGTAO.hlsl](../../reconstruction/shader-source/shaders/ssgi/XeGTAO.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>public_dependency_boundary；GPU等价未验收 |
| `shaders/ssgi/vaShared.hlsl` | [reconstruction/shader-source/shaders/ssgi/vaShared.hlsl](../../reconstruction/shader-source/shaders/ssgi/vaShared.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>public_dependency_boundary；GPU等价未验收 |
| `shaders/ssgi/temporal_filter.cs.hlsl` | [reconstruction/shader-source/shaders/ssgi/temporal_filter.cs.hlsl](../../reconstruction/shader-source/shaders/ssgi/temporal_filter.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/ssgi/upsample.cs.hlsl` | [reconstruction/shader-source/shaders/ssgi/upsample.cs.hlsl](../../reconstruction/shader-source/shaders/ssgi/upsample.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/stairs_foliage_exclusion.raster.hlsl` | [reconstruction/shader-source/shaders/stairs_foliage_exclusion.raster.hlsl](../../reconstruction/shader-source/shaders/stairs_foliage_exclusion.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | [evidence/render-passes/integration/f-r-recheck/_stairs_foliage_exclusion.hlsl_b903e43ffb3da915_recheck.json](../../evidence/render-passes/integration/f-r-recheck/_stairs_foliage_exclusion.hlsl_b903e43ffb3da915_recheck.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>[evidence/render-passes/integration/f-r-recheck/_stairs_foliage_exclusion.hlsl_f0adff76827e73ff_recheck.json](../../evidence/render-passes/integration/f-r-recheck/_stairs_foliage_exclusion.hlsl_f0adff76827e73ff_recheck.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>[evidence/render-passes/recovery/f-r/stairs_foliage_exclusion_f0ad_report.json](../../evidence/render-passes/recovery/f-r/stairs_foliage_exclusion_f0ad_report.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>[evidence/render-passes/recovery/f-r/stairs_foliage_exclusion_report.json](../../evidence/render-passes/recovery/f-r/stairs_foliage_exclusion_report.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/star.raster.hlsl` | [reconstruction/shader-source/shaders/star.raster.hlsl](../../reconstruction/shader-source/shaders/star.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/taa.cs.hlsl` | [reconstruction/shader-source/shaders/taa.cs.hlsl](../../reconstruction/shader-source/shaders/taa.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-dof-aa-merge/check__taa.cs.hlsl_cs_main_0.bin.json](../../evidence/render-passes/integration/wave2-dof-aa-merge/check__taa.cs.hlsl_cs_main_0.bin.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/unjitter_taa.hlsl` | [reconstruction/shader-source/shaders/inc/unjitter_taa.hlsl](../../reconstruction/shader-source/shaders/inc/unjitter_taa.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/terrain_derivative.cs.hlsl` | [reconstruction/shader-source/shaders/terrain_derivative.cs.hlsl](../../reconstruction/shader-source/shaders/terrain_derivative.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/recovery/f-t/terrain_derivative.cs.hlsl.report.json](../../evidence/render-passes/recovery/f-t/terrain_derivative.cs.hlsl.report.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/terrain_editing/cache_stroke.cs.hlsl` | [reconstruction/shader-source/shaders/terrain_editing/cache_stroke.cs.hlsl](../../reconstruction/shader-source/shaders/terrain_editing/cache_stroke.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/recovery/f-t/cache_stroke.cs.hlsl.report.json](../../evidence/render-passes/recovery/f-t/cache_stroke.cs.hlsl.report.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/inc/smooth_max.hlsl` | [reconstruction/shader-source/shaders/inc/smooth_max.hlsl](../../reconstruction/shader-source/shaders/inc/smooth_max.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/terrain_editing/calc_highlight_hill_mask.cs.hlsl` | [reconstruction/shader-source/shaders/terrain_editing/calc_highlight_hill_mask.cs.hlsl](../../reconstruction/shader-source/shaders/terrain_editing/calc_highlight_hill_mask.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/recovery/f-t/calc_highlight_hill_mask.cs.hlsl.report.json](../../evidence/render-passes/recovery/f-t/calc_highlight_hill_mask.cs.hlsl.report.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/terrain_editing/floor.raster.hlsl` | [reconstruction/shader-source/shaders/terrain_editing/floor.raster.hlsl](../../reconstruction/shader-source/shaders/terrain_editing/floor.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | [evidence/render-passes/recovery/f-t/floor.raster.hlsl.report.json](../../evidence/render-passes/recovery/f-t/floor.raster.hlsl.report.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/clearing/inc/forest_floor.hlsl` | [reconstruction/shader-source/shaders/clearing/inc/forest_floor.hlsl](../../reconstruction/shader-source/shaders/clearing/inc/forest_floor.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/terrain_editing/path_cobblestone_surface.inc.hlsl` | [reconstruction/shader-source/shaders/terrain_editing/path_cobblestone_surface.inc.hlsl](../../reconstruction/shader-source/shaders/terrain_editing/path_cobblestone_surface.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/terrain_editing/rasterize_terrain_stroke.cs.hlsl` | [reconstruction/shader-source/shaders/terrain_editing/rasterize_terrain_stroke.cs.hlsl](../../reconstruction/shader-source/shaders/terrain_editing/rasterize_terrain_stroke.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/recovery/f-t/rasterize_terrain_stroke.cs.hlsl.report.json](../../evidence/render-passes/recovery/f-t/rasterize_terrain_stroke.cs.hlsl.report.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/terrain_editing/inc_terrain_stroke.inc.hlsl` | [reconstruction/shader-source/shaders/terrain_editing/inc_terrain_stroke.inc.hlsl](../../reconstruction/shader-source/shaders/terrain_editing/inc_terrain_stroke.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/terrain_editing/sdf_utils.inc.hlsl` | [reconstruction/shader-source/shaders/terrain_editing/sdf_utils.inc.hlsl](../../reconstruction/shader-source/shaders/terrain_editing/sdf_utils.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/terrain_editing/read_terrain_height.cs.hlsl` | [reconstruction/shader-source/shaders/terrain_editing/read_terrain_height.cs.hlsl](../../reconstruction/shader-source/shaders/terrain_editing/read_terrain_height.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/recovery/f-t/read_terrain_height.cs.hlsl.report.json](../../evidence/render-passes/recovery/f-t/read_terrain_height.cs.hlsl.report.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/terrain_editing/read_terrain_rock_mask.cs.hlsl` | [reconstruction/shader-source/shaders/terrain_editing/read_terrain_rock_mask.cs.hlsl](../../reconstruction/shader-source/shaders/terrain_editing/read_terrain_rock_mask.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/recovery/f-t/read_terrain_rock_mask.cs.hlsl.report.json](../../evidence/render-passes/recovery/f-t/read_terrain_rock_mask.cs.hlsl.report.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/terrain_lighting_texture.cs.hlsl` | [reconstruction/shader-source/shaders/terrain_lighting_texture.cs.hlsl](../../reconstruction/shader-source/shaders/terrain_lighting_texture.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/recovery/f-t/terrain_lighting_texture.cs.hlsl.report.json](../../evidence/render-passes/recovery/f-t/terrain_lighting_texture.cs.hlsl.report.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/text.raster.hlsl` | [reconstruction/shader-source/shaders/text.raster.hlsl](../../reconstruction/shader-source/shaders/text.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/texture_test.raster.hlsl` | [reconstruction/shader-source/shaders/texture_test.raster.hlsl](../../reconstruction/shader-source/shaders/texture_test.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/tgsr.cs.hlsl` | [reconstruction/shader-source/shaders/tgsr.cs.hlsl](../../reconstruction/shader-source/shaders/tgsr.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-dof-aa-merge/check__tgsr.cs.hlsl_cs_main_3ee6a953408e4c4c.bin.json](../../evidence/render-passes/integration/wave2-dof-aa-merge/check__tgsr.cs.hlsl_cs_main_3ee6a953408e4c4c.bin.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>[evidence/render-passes/integration/wave2-dof-aa-merge/check__tgsr.cs.hlsl_cs_main_7a0af18450cdbcd1.bin.json](../../evidence/render-passes/integration/wave2-dof-aa-merge/check__tgsr.cs.hlsl_cs_main_7a0af18450cdbcd1.bin.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>[evidence/render-passes/integration/wave2-dof-aa-merge/check__tgsr.cs.hlsl_cs_main_b303f36f0dbdd5f6.bin.json](../../evidence/render-passes/integration/wave2-dof-aa-merge/check__tgsr.cs.hlsl_cs_main_b303f36f0dbdd5f6.bin.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>[evidence/render-passes/integration/wave2-dof-aa-merge/check__tgsr.cs.hlsl_cs_main_f7efabb81dfe256b.bin.json](../../evidence/render-passes/integration/wave2-dof-aa-merge/check__tgsr.cs.hlsl_cs_main_f7efabb81dfe256b.bin.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/tgsr_common.inc.hlsl` | [reconstruction/shader-source/shaders/tgsr_common.inc.hlsl](../../reconstruction/shader-source/shaders/tgsr_common.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/tgsr_blur_lum.cs.hlsl` | [reconstruction/shader-source/shaders/tgsr_blur_lum.cs.hlsl](../../reconstruction/shader-source/shaders/tgsr_blur_lum.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-dof-aa-merge/check__tgsr_blur_lum.cs.hlsl_cs_main_0.bin.json](../../evidence/render-passes/integration/wave2-dof-aa-merge/check__tgsr_blur_lum.cs.hlsl_cs_main_0.bin.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/tgsr_reproject.cs.hlsl` | [reconstruction/shader-source/shaders/tgsr_reproject.cs.hlsl](../../reconstruction/shader-source/shaders/tgsr_reproject.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-dof-aa-merge/check__tgsr_reproject.cs.hlsl_cs_main_0.bin.json](../../evidence/render-passes/integration/wave2-dof-aa-merge/check__tgsr_reproject.cs.hlsl_cs_main_0.bin.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/timed_terrain_gizmo.raster.hlsl` | [reconstruction/shader-source/shaders/timed_terrain_gizmo.raster.hlsl](../../reconstruction/shader-source/shaders/timed_terrain_gizmo.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | [evidence/render-passes/recovery/f-t/timed_terrain_gizmo.raster.hlsl.report.json](../../evidence/render-passes/recovery/f-t/timed_terrain_gizmo.raster.hlsl.report.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 1, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/timer_circle.raster.hlsl` | [reconstruction/shader-source/shaders/timer_circle.raster.hlsl](../../reconstruction/shader-source/shaders/timer_circle.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/ui_2d.raster.hlsl` | [reconstruction/shader-source/shaders/ui_2d.raster.hlsl](../../reconstruction/shader-source/shaders/ui_2d.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/ui_2d_color_button.raster.hlsl` | [reconstruction/shader-source/shaders/ui_2d_color_button.raster.hlsl](../../reconstruction/shader-source/shaders/ui_2d_color_button.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/ui_2d_ws.raster.hlsl` | [reconstruction/shader-source/shaders/ui_2d_ws.raster.hlsl](../../reconstruction/shader-source/shaders/ui_2d_ws.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/upload_cpu_raster.cs.hlsl` | [reconstruction/shader-source/shaders/upload_cpu_raster.cs.hlsl](../../reconstruction/shader-source/shaders/upload_cpu_raster.cs.hlsl) 存在 | r16f/compute, rgba16f/compute | [evidence/render-passes/recovery/f-t/upload_cpu_raster.cs.hlsl.report.json](../../evidence/render-passes/recovery/f-t/upload_cpu_raster.cs.hlsl.report.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/vermeer/acquire_target.cs.hlsl` | [reconstruction/shader-source/shaders/vermeer/acquire_target.cs.hlsl](../../reconstruction/shader-source/shaders/vermeer/acquire_target.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/recovery/final-post-painterly/_vermeer_acquire_target.cs.hlsl_cs_main_0.json](../../evidence/render-passes/recovery/final-post-painterly/_vermeer_acquire_target.cs.hlsl_cs_main_0.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/vermeer/combine.raster.hlsl` | [reconstruction/shader-source/shaders/vermeer/combine.raster.hlsl](../../reconstruction/shader-source/shaders/vermeer/combine.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | [evidence/render-passes/recovery/final-post-painterly/_vermeer_combine.raster.hlsl_b903e43ffb3da915.json](../../evidence/render-passes/recovery/final-post-painterly/_vermeer_combine.raster.hlsl_b903e43ffb3da915.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/vermeer/stroke.inc.hlsl` | [reconstruction/shader-source/shaders/vermeer/stroke.inc.hlsl](../../reconstruction/shader-source/shaders/vermeer/stroke.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/vermeer/convolve_target.cs.hlsl` | [reconstruction/shader-source/shaders/vermeer/convolve_target.cs.hlsl](../../reconstruction/shader-source/shaders/vermeer/convolve_target.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/recovery/final-post-painterly/_vermeer_convolve_target.cs.hlsl_cs_main_0.json](../../evidence/render-passes/recovery/final-post-painterly/_vermeer_convolve_target.cs.hlsl_cs_main_0.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/vermeer/generate_strokes.cs.hlsl` | [reconstruction/shader-source/shaders/vermeer/generate_strokes.cs.hlsl](../../reconstruction/shader-source/shaders/vermeer/generate_strokes.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/recovery/final-post-painterly/_vermeer_generate_strokes.cs.hlsl_cs_main_2f7da8dc0793466b.json](../../evidence/render-passes/recovery/final-post-painterly/_vermeer_generate_strokes.cs.hlsl_cs_main_2f7da8dc0793466b.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/vermeer/raster_strokes.raster.hlsl` | [reconstruction/shader-source/shaders/vermeer/raster_strokes.raster.hlsl](../../reconstruction/shader-source/shaders/vermeer/raster_strokes.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | [evidence/render-passes/recovery/final-post-painterly/_vermeer_raster_strokes.raster.hlsl_b903e43ffb3da915.json](../../evidence/render-passes/recovery/final-post-painterly/_vermeer_raster_strokes.raster.hlsl_b903e43ffb3da915.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/vertex_color.raster.hlsl` | [reconstruction/shader-source/shaders/vertex_color.raster.hlsl](../../reconstruction/shader-source/shaders/vertex_color.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/wall/generate_draw_lists.cs.hlsl` | [reconstruction/shader-source/shaders/wall/generate_draw_lists.cs.hlsl](../../reconstruction/shader-source/shaders/wall/generate_draw_lists.cs.hlsl) 存在 | cs_pass1/compute, cs_pass2/compute | [evidence/render-passes/integration/wave2-nani-cull-merge/reports/wall_generate_draw_lists_cs_pass1.json](../../evidence/render-passes/integration/wave2-nani-cull-merge/reports/wall_generate_draw_lists_cs_pass1.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>[evidence/render-passes/integration/wave2-nani-cull-merge/reports/wall_generate_draw_lists_cs_pass2.json](../../evidence/render-passes/integration/wave2-nani-cull-merge/reports/wall_generate_draw_lists_cs_pass2.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/wall/inc/brick_instance.hlsl` | [reconstruction/shader-source/shaders/wall/inc/brick_instance.hlsl](../../reconstruction/shader-source/shaders/wall/inc/brick_instance.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/wall/inc/draw_lists.hlsl` | [reconstruction/shader-source/shaders/wall/inc/draw_lists.hlsl](../../reconstruction/shader-source/shaders/wall/inc/draw_lists.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/wall/reset_pass2_inst_counts.cs.hlsl` | [reconstruction/shader-source/shaders/wall/reset_pass2_inst_counts.cs.hlsl](../../reconstruction/shader-source/shaders/wall/reset_pass2_inst_counts.cs.hlsl) 存在 | cs_main/compute | [evidence/render-passes/integration/wave2-nani-cull-merge/reports/wall_reset_pass2_inst_counts.json](../../evidence/render-passes/integration/wave2-nani-cull-merge/reports/wall_reset_pass2_inst_counts.json)：{'checked': 1, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 1}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/wall/wall_brick_lod0.raster.hlsl` | [reconstruction/shader-source/shaders/wall/wall_brick_lod0.raster.hlsl](../../reconstruction/shader-source/shaders/wall/wall_brick_lod0.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | [evidence/render-passes/integration/wave2-nani-cull-merge/reports/wall_wall_brick_lod0_b903.json](../../evidence/render-passes/integration/wave2-nani-cull-merge/reports/wall_wall_brick_lod0_b903.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/wall/inc/brick_vert.hlsl` | [reconstruction/shader-source/shaders/wall/inc/brick_vert.hlsl](../../reconstruction/shader-source/shaders/wall/inc/brick_vert.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/wall/wall_brick_lod1.raster.hlsl` | [reconstruction/shader-source/shaders/wall/wall_brick_lod1.raster.hlsl](../../reconstruction/shader-source/shaders/wall/wall_brick_lod1.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | [evidence/render-passes/integration/wave2-nani-cull-merge/reports/wall_wall_brick_lod1_b903.json](../../evidence/render-passes/integration/wave2-nani-cull-merge/reports/wall_wall_brick_lod1_b903.json)：{'checked': 2, 'pass_normalized': 0, 'pass_structural': 0, 'fail': 2}<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/water/calc_deferred_rt_args.cs.hlsl` | [reconstruction/shader-source/shaders/water/calc_deferred_rt_args.cs.hlsl](../../reconstruction/shader-source/shaders/water/calc_deferred_rt_args.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/water/deferred_rt.cs.hlsl` | [reconstruction/shader-source/shaders/water/deferred_rt.cs.hlsl](../../reconstruction/shader-source/shaders/water/deferred_rt.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/water/draw_water_mask.cs.hlsl` | [reconstruction/shader-source/shaders/water/draw_water_mask.cs.hlsl](../../reconstruction/shader-source/shaders/water/draw_water_mask.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/water/ice_cracks.raster.hlsl` | [reconstruction/shader-source/shaders/water/ice_cracks.raster.hlsl](../../reconstruction/shader-source/shaders/water/ice_cracks.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/water/ice_deferred_rt.cs.hlsl` | [reconstruction/shader-source/shaders/water/ice_deferred_rt.cs.hlsl](../../reconstruction/shader-source/shaders/water/ice_deferred_rt.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/water/ice_refl_march.cs.hlsl` | [reconstruction/shader-source/shaders/water/ice_refl_march.cs.hlsl](../../reconstruction/shader-source/shaders/water/ice_refl_march.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/water/ice_refl_resolve.cs.hlsl` | [reconstruction/shader-source/shaders/water/ice_refl_resolve.cs.hlsl](../../reconstruction/shader-source/shaders/water/ice_refl_resolve.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/water/ice_refr_resolve.cs.hlsl` | [reconstruction/shader-source/shaders/water/ice_refr_resolve.cs.hlsl](../../reconstruction/shader-source/shaders/water/ice_refr_resolve.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/water_plane.raster.hlsl` | [reconstruction/shader-source/shaders/water_plane.raster.hlsl](../../reconstruction/shader-source/shaders/water_plane.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/water/water_surface_frag.inc.hlsl` | [reconstruction/shader-source/shaders/water/water_surface_frag.inc.hlsl](../../reconstruction/shader-source/shaders/water/water_surface_frag.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/water_plane_refraction_copy.raster.hlsl` | [reconstruction/shader-source/shaders/water_plane_refraction_copy.raster.hlsl](../../reconstruction/shader-source/shaders/water_plane_refraction_copy.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/water/read_water_mask.cs.hlsl` | [reconstruction/shader-source/shaders/water/read_water_mask.cs.hlsl](../../reconstruction/shader-source/shaders/water/read_water_mask.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/water_snow_plane.raster.hlsl` | [reconstruction/shader-source/shaders/water_snow_plane.raster.hlsl](../../reconstruction/shader-source/shaders/water_snow_plane.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/water/spawn_boats.cs.hlsl` | [reconstruction/shader-source/shaders/water/spawn_boats.cs.hlsl](../../reconstruction/shader-source/shaders/water/spawn_boats.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/water/spawn_lilypads.cs.hlsl` | [reconstruction/shader-source/shaders/water/spawn_lilypads.cs.hlsl](../../reconstruction/shader-source/shaders/water/spawn_lilypads.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/water/spawn_reeds.cs.hlsl` | [reconstruction/shader-source/shaders/water/spawn_reeds.cs.hlsl](../../reconstruction/shader-source/shaders/water/spawn_reeds.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/water/spawn_waterwheels.cs.hlsl` | [reconstruction/shader-source/shaders/water/spawn_waterwheels.cs.hlsl](../../reconstruction/shader-source/shaders/water/spawn_waterwheels.cs.hlsl) 存在 | cs_main/compute | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/water_x_paths.raster.hlsl` | [reconstruction/shader-source/shaders/water_x_paths.raster.hlsl](../../reconstruction/shader-source/shaders/water_x_paths.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/water/waterfall_vert.inc.hlsl` | [reconstruction/shader-source/shaders/water/waterfall_vert.inc.hlsl](../../reconstruction/shader-source/shaders/water/waterfall_vert.inc.hlsl) 存在 | include/嵌入源 | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/waterfall.raster.hlsl` | [reconstruction/shader-source/shaders/waterfall.raster.hlsl](../../reconstruction/shader-source/shaders/waterfall.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |
| `shaders/waterfall_refraction_copy.raster.hlsl` | [reconstruction/shader-source/shaders/waterfall_refraction_copy.raster.hlsl](../../reconstruction/shader-source/shaders/waterfall_refraction_copy.raster.hlsl) 存在 | vs_main/vertex, ps_main/fragment | 未建立直接文件级报告；消费main/族报告另查<br>shader_game_candidate_or_shared_helper；GPU等价未验收 |

## 重建源码函数定义清单

### shaders/apply_screen_shadows.cs.hlsl

SHA256：`13de15ad744bd888cf6ee1ae8e559a5886800ccc6ed99dd798ca59526b11bb0a`。

sample_shadow_pcss@L71, cs_main@L172

### shaders/inc/common.hlsl

SHA256：`5a4bdff119ca0e3691c9e70caad6a1a8bf203ddacd27e8659cdb641df834f297`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/inc/samplers.hlsl

SHA256：`21a0c4a3e8d78a0235d2ad7bbb71129995cbee5c798b8b1d1cc0182c1b93ef51`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/inc/debug_utils.hlsl

SHA256：`0572fcb305e3bf1120085e220ffe6afec476faef68492a4fd84869fceca1fc46`。

debug_heatmap@L13

### shaders/inc/tracked_buffer.hlsl

SHA256：`e4dc5ea68bc4ae17302c6a6a632c6f83eee4fc5f71f5492d5a9b8071d1a37cb7`。

tracked_buffer_reserve@L14, tracked_buffer_store_float@L22

### shaders/inc/pack_unpack.hlsl

SHA256：`cac515f0d0ccdeb0b9be0381993f047da2b4b24b67739c43397cbc0d28ed4e7d`。

pack_rgba8@L10, pack_rgb8@L20, unpack_rgb8@L29, unpack_rgba8@L39, unpack_normal_rgb8@L45

### shaders/inc/math.hlsl

SHA256：`bd8ff2a266cf5e02e5edf193048da2e1f86d5a7279e525c5065102f04608bc8d`。

clamp01@L11, clamp01@L16, length_squared@L22, safe_normalize@L28, safe_rcp@L36, rcp_keep_sign@L42

### shaders/inc/math_const.hlsl

SHA256：`8ce35cf4835885b79b29eff6204783df12f638e42cc0eb3a757f999f0a2bf5a5`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/inc/uv.hlsl

SHA256：`ca6968fd453e7d6f8c23ab4462a0f225051776968be3b975ce72149325970726`。

pixel_to_uv@L12, uv_to_sample@L20, sample_to_clip@L26, sample_to_view@L33, view_to_sample@L39, view_to_clip@L45, world_to_view@L52, world_to_sample@L58

### shaders/inc/frame_constants.hlsl

SHA256：`5c9c356c140167ff01eaa16cf5b1fe0ee814198bbfed8c0fa169328555099146`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/inc/ray_cone.hlsl

SHA256：`25eecf310a21639fd18502c1f35c2514d253688cd3829dfb5b9f2a5b68011fb1`。

ray_cone_half_width@L22, ray_cone_mip@L29

### shaders/inc/sun.inc.hlsl

SHA256：`9fe24e7a1ec8167bbaa866d7260e7655d17a2410e18ec4e1b9703a8cc7238a96`。

get_sun_color@L10, get_sky_low_color@L15, get_sky_high_color@L20, get_sun_direction@L25, get_sky_color@L31

### shaders/inc/color_library.hlsl

SHA256：`b6684a2282965d56929b9d443103513ba5064d84ec0abaa4776d94b713bf4650`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/inc/color/srgb.hlsl

SHA256：`31e7e2173441dc8dda1a61de1a1422da2c292e831e04c06e1fe7b4bf5c67f773`。

srgb_to_linear@L14, srgb_to_linear@L24, srgb_to_linear@L29, linear_to_srgb@L35, linear_to_srgb@L45, linear_to_srgb@L50

### shaders/inc/utils.hlsl

SHA256：`02cacaf49a76b3857f1247795ebdd00757be134506d2dbef486aced631f848db`。

affine_basis@L27, affine_translation@L41, affine_scale@L51, pack_affine@L62, unpack_affine@L73, compose_affine_packed@L84

### shaders/inc/consts.hlsl

SHA256：`27a6d2f400717ae83dcc42be89783dec0390eb3ccc3821da48005b88e4d1ba7c`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/inc/hash.hlsl

SHA256：`32d1ae5fe1c8c4a7a927b9c31215ad9e6f7fd8dca2b7f09766a473d02ec26885`。

hash@L15, rand01@L26, hash_angle@L32, hash01@L38

### shaders/gi/gi_settings.inc.hlsl

SHA256：`6e374b8224c9ba583e8a6770dfcbd97b6b4f5b819d548054e6168c20fc0069b0`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/inc/history_valid.hlsl

SHA256：`a44d9d48a7dd361eb163009f3345dee42a634fac454a375cb7434a52d0d9e79c`。

incb_frexp_scalar@L22, history_depth_valid@L40, is_history_valid@L52

### shaders/inc/shadow.hlsl

SHA256：`55167cdadb5c56ffdb71f14d57c557c2ad8ad3ae4ebbfbcfea8b627cbc49d17a`。

sample_shadow@L18

### shaders/inc/hammersley.hlsl

SHA256：`17be2f0f2a9ceb30586793ed28a1f334a5d9fb0fa6f816b0838776172187fe7e`。

radicalInverse_VdC@L12, hammersley@L22

### shaders/inc/cascades.hlsl

SHA256：`5646c96ca40dede2772625b321e210827c5e6724ac3fef0be1bef5c6f84efa36`。

select_shadow_cascade@L17

### shaders/apply_volumetrics.cs.hlsl

SHA256：`f26f48200cd252a01c9af61d2eff63c1b1020bec0f429dfd1c101c5b25f41a89`。

fake_light_occlusion@L58, cs_main@L80

### shaders/inc/current_view.hlsl

SHA256：`b6a3293c7844af856214f5123120dd04a8ccdbc9964bdfcbb3c605425dc786ec`。

GetCurrentViewConstants@L15

### shaders/inc/perlin.hlsl

SHA256：`b12ea5a201eaa7e269addfbc4c18d10ae09f90d2c5b1397c4cfb4f11b037a228`。

hash@L20, incb_perlin_hash01@L35, perlin@L40

### shaders/inc/blue_noise.hlsl

SHA256：`900bc60759cddde33419a018e8fafc5659d926866d38d581bda21a989852935e`。

blue_noise@L19

### shaders/billboard_forest.raster.hlsl

SHA256：`91d1324ff6694baf92715119758120ba7c7dba372893f48b9fe82c13a2830739`。

vs_main@L103, ps_main@L195, vert_main@L378, vs_main@L489, frag_main@L629, ps_main@L740, vert_main@L847, vs_main@L958, frag_main@L996, ps_main@L1033

### shaders/inc/simplex_noise.hlsl

SHA256：`1da23332bc5e2cabfcb6c275636fa2d5ce7ab5240a74edcfdd544053ba2130ee`。

incb_mod289@L13, incb_mod289@L18, incb_permute@L23, incb_permute@L28, snoise@L33

### shaders/inc/material/solid_material.inc.hlsl

SHA256：`c538e7c119f6962f67d6edc58b8cc4879b1c727c6010c16fa854ecb51f24784b`。

init_solid_material@L15

### shaders/inc/material_eval.inc.hlsl

SHA256：`fd3535089b5463fba31916bc392a0af8f8c3301fbcc5d41efdd24241d4e708e9`。

unpack_gbuffer_normal@L19, init_material_eval_data@L29

### shaders/inc/brdf.hlsl

SHA256：`e2f8ea31e167244924c913d90fff030927c8379580db364e67c1bd9532a97ec9`。

distribution_ggx@L9, visibility_smith_ggx@L16, fresnel_schlick@L26, diffuse_ggx@L34, cook_torrance_specular@L44, evaluate_brdf@L55

### shaders/inc/lighting.hlsl

SHA256：`a3dba2074c7b3f960edc1da467da78720ac8c2eee14ef268bc0ee524b40a962d`。

evaluate_sun_light@L13, evaluate_sky_ambient@L27, evaluate_lighting@L36

### shaders/inc/sh.hlsl

SHA256：`2f236c5b9ef688a9ddb82bbb96da6c9a1113bf09eb6979b6fbaf531816d4ee93`。

sh_basis@L12, sh_eval@L17

### shaders/billboard_plant.raster.hlsl

SHA256：`a1e84ee39a10d0daf00617967c2766c6ec6ebadffe410fee9eaf2e71fd57c419`。

vs_main@L76, ps_main@L106, frag_main@L252, ps_main@L321, frag_main@L412, ps_main@L456

### shaders/blit/blit_stretch_decode_srgb.raster.hlsl

SHA256：`5511be4059422af3dc427a0a3af5ea78c69b430a5c689dcd2ccaacb15ca8236d`。

ps_main@L16, frag_main@L46, ps_main@L50

### shaders/fullscreen_triangle_vert.inc.hlsl

SHA256：`f0f08881b737c6fdd0b2c4fabaa2ae349636c1ba97f5942422958e0048766080`。

vs_main@L16

### shaders/resolve_to_swapchain.inc.hlsl

SHA256：`cd205197f26d1445e9caead7a08cb47661de8073eb409f0ba63426c8c3ddfa5f`。

resolve_to_swapchain_coord@L18

### shaders/blit/resolve_msaa_to_swapchain.raster.hlsl

SHA256：`39dcad5a8a58323fba8af5cc16b633c16e96a8f42090e84bbe4dd3f92f26bed3`。

ps_main@L21, frag_main@L59, ps_main@L63

### shaders/blit/resolve_to_swapchain.raster.hlsl

SHA256：`66773cd46b58b622d014ce31e374fb4ee58f89b84d299cde6edd3f14c2c6dea7`。

ps_main@L21, frag_main@L48, ps_main@L52

### shaders/blur.cs.hlsl

SHA256：`3498e43652f414bc20ad043e374400c27d8b16704d6f77dc7633b5771c27ed58`。

cs_main@L22

### shaders/blur_downsample.cs.hlsl

SHA256：`f62c18592fd9690a40b10ab6c66dbe854644ea5d14e191a6732435fb15e1adb9`。

cs_main@L24, cs_main_multisampled@L62

### shaders/blur_pyramid/blur.cs.hlsl

SHA256：`9aada9bff852f18e4b1a04049c7bac42fc421b6da4f61351c9d282e8c5940c37`。

cs_main@L26

### shaders/blur_pyramid/rev_blur.cs.hlsl

SHA256：`eb4347498c23dd6fc977c8aeabf6426c95a513343307acf3ad820c6941a9d3ee`。

cs_main@L24

### shaders/calculate_reprojection_map.cs.hlsl

SHA256：`c5b74e043e81f970a957bf05245eed6c30d455210ecc4ae9945919fe0ae9e540`。

cs_main@L39

### shaders/inc/bilinear.hlsl

SHA256：`96b7810e3d18bc98a49380743e6274cf586e470caf57c0ed5110847cdb17ab99`。

bilinear_corners@L19, bilinear_corners_mask@L30

### shaders/capture_panic_screenshot.cs.hlsl

SHA256：`57920d5aedce21b2582aa1452a5f32960c8235d2a48fd3125ef874000a32f27f`。

cs_main@L29

### shaders/inc/color/ycbcr.hlsl

SHA256：`41b133c4980ab9cbdd8eb0d2e48df20bfefb86b882bb049d970b95f3af20f973`。

rgb_to_ycbcr@L10, ycbcr_to_rgb@L20

### shaders/clearing/distant_billboard.raster.hlsl

SHA256：`6f8c08f6c9063ef986f0df6ad0ddcd0f812e79d1acbd9550348753775193f767`。

vs_main@L42, ps_main@L77

### shaders/clearing/instanced_background_tree.raster.hlsl

SHA256：`8489f66266668c19c56e8514ff549b556c31a4edad4f0baec616ef3b8b621929`。

vs_main@L92, ps_main@L132, frag_main@L223, ps_main@L231, frag_main@L251, ps_main@L255

### shaders/clearing/inc/instanced_placement.hlsl

SHA256：`a5576cec82b2dd743228e0a0df41a4253a398f097210aa7a9f1af361788e7862`。

placement_world@L33

### shaders/clearing/meadow_flower.raster.hlsl

SHA256：`a957aa71b135f55c822600b76cef21829bed284da59e441f7424255d8e6e56b5`。

vs_main@L92, pack_normal8@L186, ps_main@L207, vert_main@L368, vs_main@L577, frag_main@L704, ps_main@L742, vert_main@L878, vs_main@L1087, frag_main@L1197, ps_main@L1213

### shaders/paths/path_mask_sampler.inc.hlsl

SHA256：`aac6ba19a75f6750ff9c1974d1b958bf23e78f4156c14cce5e27b5e296362c3b`。

get_path_mask@L15, is_path@L23

### shaders/inc/texture_utils.hlsl

SHA256：`94787bbfa0835586cf76346f4f61a10c4a99f2e8f0bb744e3eb1cbb790b0357d`。

sample_lod_bias@L10, sample_lod@L16, load_texel@L22

### shaders/inc/terrain_heightmap.inc.hlsl

SHA256：`4a711b4526bd7bb1ce28c65ef57c8e5c6e346b7432811e45c5edfade25720977`。

terrain_heightmap_uv@L19, get_terrain_height@L25, terrain_height@L41, get_terrain_normal@L48

### shaders/inc/rocky_terrain.inc.hlsl

SHA256：`52d030717e6d401e23162d0b5ef1e7ea6a0f0940d2a00f48fb4c9dd286270fb7`。

get_rocky_terrain_height@L12, is_rocky_terrain@L21

### shaders/water/water_mask_sampler.inc.hlsl

SHA256：`effeaaa358a3d41a6495be94ba637e545cfa733e7b435edda8162fdb0337d67c`。

get_water_mask@L21, is_water@L28

### shaders/inc/material/plant_material.inc.hlsl

SHA256：`03c50136aa1be752f3134931f02031c39c670ee973fab00aa68b4b74cba9b327`。

init_plant_material@L18

### shaders/clearing/meadow_plant.raster.hlsl

SHA256：`e6ef78fa1a427c6ddc7e4be7f0d5796e053b4dfecff957be67b6fbac8ad9d1ce`。

vs_main@L133, ps_main@L281, frag_main@L489, ps_main@L546

### shaders/buildings/building_mask.inc.hlsl

SHA256：`a06b3180fd31df7a9d77781b9d47a708ab20762a8cc4ebc59c21f528438a9dfa`。

is_building_circle@L20, is_building_box@L28, is_building@L39, get_building_mask@L54

### shaders/inc/foliage_exclusion.hlsl

SHA256：`ee8943e15791c65e744c090881fab43f8f7d0080bc4da91ab913df4b30e8643c`。

get_foliage_exclusion@L11, is_foliage_excluded@L18

### shaders/inc/wind.hlsl

SHA256：`5502530eda638105fc4d56e2a4f19c2e8887769d3526e52ff627187dec8cdf96`。

wind_noise@L9, evaluate_wind@L28

### shaders/clearing/tree_canopy.raster.hlsl

SHA256：`36971206568c6bbd57ce53278b4860381296863a112f244b416ea134d4f3a5c3`。

spvPackHalf2x16@L142, spvUnpackHalf2x16@L148, vert_main@L153, vs_main@L380, frag_main@L566, ps_main@L867, spvPackHalf2x16@L1027, spvUnpackHalf2x16@L1033, vert_main@L1038, vs_main@L1193, frag_main@L1230, ps_main@L1234

### shaders/clearing/canopy_vert.inc.hlsl

SHA256：`7d485bc691b201be3e9ac0993edf4941c5acd9b42c7d200ef7d73ec79fc9f008`。

canopy_vertex@L14

### shaders/clearing/inc/canopy_common.inc.hlsl

SHA256：`c88707c437b0a09f627f6b4f79599180d4dfe9e4996dbea96f4fa16cbc61b890`。

canopy_age@L14, canopy_bend@L20, canopy_place@L26

### shaders/nani/inc/user.hlsl

SHA256：`107bc7a1eaee95abe679403f1a917dae0130b95cbbcd9a363e297b5ccf887eef`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/inc/material/canopy_material.inc.hlsl

SHA256：`fea4176511f47b3587e9fe5000c81ebf3acf0c3be248fd1756b61ec142ff50a5`。

init_canopy_material@L18

### shaders/clearing/tree_trunk.raster.hlsl

SHA256：`27434a4c50c9a3f8c751f49bf1cae5517de3a9b295acf7e1ab41653cf805aeeb`。

vs_main@L85, ps_main@L151, spvPackHalf2x16@L405, spvUnpackHalf2x16@L411, vert_main@L416, vs_main@L444, frag_main@L569, ps_main@L863, spvPackHalf2x16@L1004, spvUnpackHalf2x16@L1010, vert_main@L1015, vs_main@L1043, frag_main@L1080, ps_main@L1088

### shaders/inc/texture_blend.hlsl

SHA256：`e6d7482e2b14db64f5c64d16abe30d304ab80e8dc716607304249072bb92d587`。

blend_layers@L10, blend_layers_height@L16

### shaders/cloud.raster.hlsl

SHA256：`ea0edd376c71d369a9815865170089c3df38c7324dcf2313e1266ef0b1f8b1ed`。

vs_main@L51, ps_main@L107, vert_main@L275, vs_main@L387, vert_main@L497, vs_main@L609, frag_main@L647, ps_main@L683

### shaders/copy_depth_to_r.cs.hlsl

SHA256：`3f8e19ab62a230c54041259ec07d11de53abd225b2d367aef8d6d95cf0b3ff8c`。

cs_main@L15

### shaders/critters/butterfly.raster.hlsl

SHA256：`8e384b7930a893963f69fb4b2db14bfc3ec4200865a77cbc706d12a5f21c670d`。

butterfly_shadow@L40, vs_main@L146, ps_main@L238, vert_main@L355, vs_main@L527, vert_main@L649, vs_main@L821, frag_main@L843, ps_main@L847

### shaders/critters/duck_animated.raster.hlsl

SHA256：`a8eac91424f7e37a0a0a2d127da533968281b70cbffb51c1ee293d09b453cf7d`。

vs_main@L47, ps_main@L134

### shaders/critters/firefly.raster.hlsl

SHA256：`d7921e616de87378a14396146690f206fbaf9ca5e7781f8a19c0cda6be0e7c69`。

vs_main@L43, ps_main@L60, frag_main@L78, ps_main@L82

### shaders/critters/firefly_glow.raster.hlsl

SHA256：`26640d43d484cdb75d317c71af5f4debc1a80d9b5cbece4b2aaf92e8b02ab78b`。

vs_main@L48, ps_main@L121, frag_main@L140, ps_main@L144

### shaders/inc/coc.hlsl

SHA256：`c128dcaaf683b794c027430bae72f29240d3e184fc70ab749b5c64dba8b77c0e`。

compute_coc@L36

### shaders/critters/firefly_vis_query.cs.hlsl

SHA256：`4f5043f165aec9a36d5a1dcde3bdfaa878baf0a2ddc855007736685a2e2345cd`。

cs_main@L33

### shaders/critters/nani_wren.raster.hlsl

SHA256：`ae54737dc74fcf6393979d711b45bc8f25431eb93bcd667597c34ff082db8c22`。

vs_main@L57, ps_main@L144, vert_main@L282, vs_main@L346, frag_main@L450, ps_main@L461, vert_main@L595, vs_main@L659, frag_main@L688, ps_main@L692

### shaders/custom_cursor.raster.hlsl

SHA256：`3c66d806109f197a3dbf33c9bec5fc61cb59deecdd68dcaf4c039d5a9c6890a5`。

vs_main@L35, ps_main@L63, vert_main@L107, vs_main@L170, vert_main@L218, vs_main@L281, frag_main@L299, ps_main@L303

### shaders/inc/quad_mesh.hlsl

SHA256：`d22203645072211df94c7d6fe286da4deff71d2b2fec3bcfa514123f95231724`。

quad_corner@L10, fullscreen_triangle_clip@L17, unit_quad_clip@L24

### shaders/dashed_line.raster.hlsl

SHA256：`f074627607260a4410ded67971287dd5ad4ff1c9e0cdde89fdc8e4689bc0a09b`。

vs_main@L32, ps_main@L42

### shaders/deferred/light_sun_sky_prelude.inc.hlsl

SHA256：`b9a3ed51bdcfb00e0b3e09eae7665c53e332023b872b5a08c3b91c124371522c`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/inc/halfres.hlsl

SHA256：`548b87b292b89a1389d77ac2a592bd3f9c437e934b2f400a6627e52d794f6156`。

to_halfres_pixel@L10, from_halfres_pixel@L16, halfres_dims@L22

### shaders/terrain_editing/inc_highlight_active_terrain_chunk.inc.hlsl

SHA256：`7a2449ce675921f0d96d27b0ed8e88f776cf894dbeebfd4ffc816eab919ab33b`。

highlight_active_terrain_chunk@L10

### shaders/inc/lighting_indirect.hlsl

SHA256：`a105b1003518fc2f4ce5ef0b293ee61faa23cca1d2ee16b4c3ba5f6c52c5a865`。

get_light_grid_cell@L23, get_light_grid_index@L31, get_light_cell_range@L38, get_light_grid_lantern_index@L45

### shaders/inc/lanterns.hlsl

SHA256：`60b89c3bca0f982381fa7f2303c86c31b62e7de91c40a118e2c2ef9eb676d1ac`。

get_lantern_position@L14, get_lantern_intensity@L20, get_lantern_color_id@L26, get_lantern_color@L32, evaluate_lantern_light@L49

### shaders/deferred/canopy_light_sun_sky.raster.hlsl

SHA256：`2b7071e22ff81e86b790fc17f667bbba99032ddcee479eca4603b39b1fd4f7fc`。

spvUnpackHalf2x16@L134, ps_main@L138, ps_main@L1588

### shaders/inc/material/eval/canopy_material.inc.hlsl

SHA256：`3b9b58e934f1b2d9b7466873c1291420e05f6b7e38b3cf5453448055d90e9f3b`。

eval_canopy_material@L11

### shaders/inc/material/eval/../canopy_material.inc.hlsl

SHA256：`fea4176511f47b3587e9fe5000c81ebf3acf0c3be248fd1756b61ec142ff50a5`。

init_canopy_material@L18

### shaders/inc/raymarch.hlsl

SHA256：`e391cc33ef5ff065fdb57bd2b0bde833d1d08261f35b01fab4ce060d5164fd0a`。

raymarch_depth_texture@L12, raymarch_opacity@L56

### shaders/deferred/light_sun_sky.inc.hlsl

SHA256：`97dcbcb62388559e00f60f4d002cb20117d7079f7b17d0898c4c948d541ba101`。

reconstruct_view_position@L39, decode_gbuffer_normal@L50, evaluate_deferred_sun_light@L61, sample_indirect_radiance@L74

### shaders/inc/rt.hlsl

SHA256：`996410873faa0dd097ca21f5adcb1797a0862e1363e9da24545ada355b84660b`。

load_packed_triangle@L48, ray_triangle_intersect@L61, rt_unpack_half2x16@L89, trace_software_bvh@L106

### shaders/inc/material/eval/solid_material.inc.hlsl

SHA256：`f27de577d273c4091d4aa45f4e59288d4602c134118ba4f01bac559c5c0794a4`。

eval_solid_material@L11

### shaders/inc/material/eval/../solid_material.inc.hlsl

SHA256：`c538e7c119f6962f67d6edc58b8cc4879b1c727c6010c16fa854ecb51f24784b`。

init_solid_material@L15

### shaders/inc/brdf_lut.hlsl

SHA256：`9f94b64f7cb90760aabbadb73af6f27fbcde54cbcfc776f6d5f64d86a4b27130`。

sample_brdf_fg_lut@L9

### shaders/inc/highlight_color.hlsl

SHA256：`110008c1231d0fd01acf2aa58c592ec923a7c32cd4a246ab8720bb35b1b8ba33`。

get_highlight_color@L15

### shaders/inc/wall_color_ids.hlsl

SHA256：`dedc715433d8039f5420bc78cd5502481fd9c287fde4dbfaa8ce4bd014c3d114`。

get_wall_color_id@L10

### shaders/inc/reservoir.hlsl

SHA256：`bfcbb5882e85514dcf85ce925cca9eccc8347caedc8f4dd06df3919769567ff4`。

reservoir_init@L22, reservoir_update@L36, reservoir_combine_streams@L50, reservoir_resolve@L57, pack_reservoir1spp@L64, unpack_reservoir1spp@L70

### shaders/light_binning/grid.inc.hlsl

SHA256：`d5e1d21440903a952c49adf99e819c1aef1c9a592a1927adde702ca3db64fd35`。

grid_world_pos_to_cell@L24, grid_cell_index@L32, grid_cell_min_world@L38, grid_cell_max_world@L43, grid_point_to_cell_dist2@L50

### shaders/deferred/grass_light_sun_sky.raster.hlsl

SHA256：`0c0f1b3421cb76f6318a796d5fb86a72d37d56b8eb1da7d4d805b3ac539f1cb4`。

spvUnpackHalf2x16@L135, ps_main@L139, ps_main@L1521

### shaders/inc/material/eval/grass_material.inc.hlsl

SHA256：`6ef9d410f501f7a61ad6ae43ef1e8c681cf7ff3bf85b14e3ba0e377bec221e2c`。

eval_grass_material@L14

### shaders/inc/material/eval/../grass_material.inc.hlsl

SHA256：`2b2ba6fab8a3ed6da852f9c883f18c2a87120895e1c586597ac88f453c228c2b`。

init_grass_material@L18

### shaders/deferred/plant_light_sun_sky.raster.hlsl

SHA256：`74efcb7ec17e56a6eb912d1b678f0629ae77dd65b6cc98ca3ccb837bb4186909`。

spvUnpackHalf2x16@L133, ps_main@L137, ps_main@L1511

### shaders/inc/material/eval/plant_material.inc.hlsl

SHA256：`002926cac07c07c6025b789114402614ffb9d0b48f1c6197357439309bd69111`。

eval_plant_material@L14

### shaders/inc/material/eval/../plant_material.inc.hlsl

SHA256：`03c50136aa1be752f3134931f02031c39c670ee973fab00aa68b4b74cba9b327`。

init_plant_material@L18

### shaders/deferred/solid_light_sun_sky.raster.hlsl

SHA256：`828a44d0f64348b12ba25816bc0fe49e03fc696c0817170f23a7ef64c2d3dda0`。

spvUnpackHalf2x16@L150, ps_main@L154, ps_main@L2009

### shaders/demo_terrain_overlay.raster.hlsl

SHA256：`8569cb49f5f6efd673ce45ac883e8ff29366209457c1287b15dfac7e5d565655`。

vs_main@L46, ps_main@L69, frag_main@L93, ps_main@L101

### shaders/dof/analyze.cs.hlsl

SHA256：`cb526a201d042358b417f336f0e6364f507b1e2a45544c13f366f6ccb67220de`。

cs_main@L33, comp_main@L186, cs_main@L393

### shaders/inc/morton.hlsl

SHA256：`800837db2997b48f5f23c792105807212724972644dc861f4f4156a46a615704`。

morton_encode@L10, morton_decode@L25

### shaders/dof/tiles.inc.hlsl

SHA256：`7f89fe62602d4bb8c5d1d0b49e979e1dfbaba1a70909b92322e6c48b8ebec72c`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/dof/autofocus.cs.hlsl

SHA256：`246f8c162e9ec7f8340c2e8c595fe6127dadbc98b556812893c2aedfe19db3c8`。

cs_main@L41, comp_main@L346, cs_main@L607

### shaders/inc/bitonic_sort.hlsl

SHA256：`bb240594827bda5868af53d2d70ebb6d885c925eec284bde5b2b0a78a44ba2ee`。

bitonic_sort_pair@L9

### shaders/dof/clear_halfres.cs.hlsl

SHA256：`954e94c5bd60e3c9ae496171e348aa81d18103f7884e8e6c28cdef1f06fb922b`。

cs_main@L13

### shaders/dof/fullres.cs.hlsl

SHA256：`9fea46b1b872f15c6cd45b88baf455bcca7887938cf642b4eee51c1b82a32760`。

cs_main@L58, spvPackHalf2x16@L432, spvUnpackHalf2x16@L438, comp_main@L443, cs_main@L741, spvPackHalf2x16@L853, spvUnpackHalf2x16@L859, comp_main@L864, cs_main@L1159

### shaders/dof/multires.inc.hlsl

SHA256：`37df4e1c16a6ecfc689cd18182960b3cb667ffd52be05000efdc0acf100afb32`。

dof_pack_half2@L14, dof_unpack_half2@L21

### shaders/dof/upsample_halfres.inc.hlsl

SHA256：`f626ee42389be4b954001518fb528b63a572be0e6f51b972598c972e47dac20d`。

upsample_halfres@L16

### shaders/dof/fullres_all_lod0.cs.hlsl

SHA256：`aab143916cf66499af54f6da5fd10ce3dd66c21d69db1360e3cb6d4c939475b8`。

cs_main@L49, spvPackHalf2x16@L373, spvUnpackHalf2x16@L379, comp_main@L384, cs_main@L633, spvPackHalf2x16@L738, spvUnpackHalf2x16@L744, comp_main@L749, cs_main@L995

### shaders/dof/fullres_all_lod1.cs.hlsl

SHA256：`87c17e197dba8703694ecb46c6cd6eb73e0fb0c3bd065fc4632667b4ec405656`。

PackRingPair@L46, UnpackRingPair@L51, TileOrigin@L56, BokehDir@L62, SampleRing@L78, cs_main@L104, spvPackHalf2x16@L366, spvUnpackHalf2x16@L372, comp_main@L377, cs_main@L614, spvPackHalf2x16@L719, spvUnpackHalf2x16@L725, comp_main@L730, cs_main@L964

### shaders/dof/fullres_upsample.cs.hlsl

SHA256：`5c592aabb68a921b5344acf11b044ee4e543566870690a2e582b422e81d7fd54`。

cs_main@L27

### shaders/dof/fused.cs.hlsl

SHA256：`c14df59a6f63e1cdd21e004a996178e7d36532a3a37ecb9fbca114c6a6a80d20`。

PackRingPair@L37, UnpackRingPair@L42, cs_main@L48

### shaders/dof/halfres.cs.hlsl

SHA256：`34a8f9ca5736290fb0ef2f8a57b02d665e2b61da71b51b2f9b2ab953c146fc8e`。

halfres_core@L53, cs_halfres@L343, cs_halfres_all@L351, spvPackHalf2x16@L453, spvUnpackHalf2x16@L459, comp_main@L464, cs_halfres@L751, spvPackHalf2x16@L856, spvUnpackHalf2x16@L862, comp_main@L867, cs_halfres@L1151, spvPackHalf2x16@L1256, spvUnpackHalf2x16@L1262, comp_main@L1267, cs_halfres_all@L1539, spvPackHalf2x16@L1644, spvUnpackHalf2x16@L1650, comp_main@L1655, cs_halfres_all@L1924

### shaders/dof/init_buffers.cs.hlsl

SHA256：`1d907b0fb98bc8583711af8c78adad1f61de3d0d60582f78e7c412a827bb1493`。

cs_main@L20

### shaders/dof/prepare.cs.hlsl

SHA256：`6aab8336b22437cfe5c7f39414ad70ef4f9e983bc9ce4475e0caa24e379d7d74`。

cs_main@L30

### shaders/egui.raster.hlsl

SHA256：`d9bb9320b0d99c1704638ae2b1fd207f6976c8e04931c3ac87aa93cef63028ba`。

vs_main@L33, ps_main@L74, ps_main@L78, frag_main@L92, ps_main@L96

### shaders/extract_half_res_depth_normals.cs.hlsl

SHA256：`e162fc3430063b8d6c8c0c87753fbf93f9b067169c9e8f807789635bd4fe7aee`。

cs_main@L28

### shaders/fill_buffer.cs.hlsl

SHA256：`c6aa96cc736c418fb9a1b212f7ef8aa4ec16f64a61a36a28123139f7ebd7dfe0`。

cs_main@L23

### shaders/final_post.raster.hlsl

SHA256：`49523a4f488287dba348d742d819e95275b80163250274bcf0ccc0db3f5f9c52`。

bicubic_upsample@L79, easu_tap_weight@L110, easu_upsample@L121, final_post_sample_uv@L235, ps_main@L244, frag_main@L424, ps_main@L459

### shaders/inc/image.hlsl

SHA256：`ad5b36b990b8fa202c068bda576755b736c46dbd7cafcc39ac1916a1342c10ab`。

texture_dims@L12, sample_mask@L20

### shaders/inc/curve.hlsl

SHA256：`6976927bcfc99146253f454ccbf1de9f7153e5d3fb2b02b4d506c56a4e765d7e`。

incb_frexp@L25, curve@L45

### shaders/inc/bicubic.hlsl

SHA256：`9c607c8becccca84d4c30c59a78437a2f5ef734ca909dbc69548ea86e47f1f96`。

bicubic_weights@L12, bicubic_sample@L25

### shaders/inc/grading_lut.hlsl

SHA256：`e73dde7ba061c801ac39561f5a17d9601cb95f6014b4164f73989b97f3c76317`。

sample_grading_lut@L8, apply_tony_mc_mapface@L14, apply_grading_lut@L20

### shaders/inc/color/xyz.hlsl

SHA256：`133c710471ed511d7f8aa0e909362cb9afc9cb62ce7f3dfaa44ab369d3c1ae9e`。

srgb_xyz_to_xyz@L11, xyz_to_srgb_xyz@L21

### shaders/inc/color/luv.hlsl

SHA256：`fab351800207726fd080eb8545b5ef1ce26387e9c85b9d7b7fdba570664ad476`。

xyz_to_luv@L18, luv_to_xyz@L32

### shaders/inc/color/helmholtz_kohlrausch.hlsl

SHA256：`73977cf74f97d68cce37c105b5d7ea41f4a3cdc922a68d02e369ac2cbbf9f6ed`。

helmholtz_kohlrausch_lightness@L14, helmholtz_kohlrausch_perceived_lightness@L22

### shaders/inc/color/math.hlsl

SHA256：`7689c12d780022e934cb177b50a594d420c56bd25c3bf5e635f174eeeb4ced87`。

luma@L10

### shaders/inc/upsample/easu.hlsl

SHA256：`289d9e3fd8d37f098c9ca2af4e96e4f34e70957a3b49fda4bc924783c8359428`。

easu_sample@L14

### shaders/inc/upsample/lanczos2.hlsl

SHA256：`7a5e48d647bf3c095cbadaf701ea030023503cc65102027aaea433d7d5272ad4`。

lanczos2_kernel@L12, lanczos2_sample@L26

### shaders/flag_roof_cloth.raster.hlsl

SHA256：`63eb851ed421a96eb4a55f61af4abdb7732082671c9aedf86f8a6245fe99a123`。

flag_noise@L46, vs_main@L83, ps_main@L127, vert_main@L289, vs_main@L358, frag_main@L480, ps_main@L516, vert_main@L645, vs_main@L714, frag_main@L744, ps_main@L748

### shaders/flag_small.raster.hlsl

SHA256：`1fc42d2acfc8df4300d5f34d346a432c9be3bf120566974e4d4c8c6eb71ea7e6`。

flag_noise@L40, vs_main@L72, ps_main@L115, vert_main@L258, vs_main@L373, frag_main@L485, ps_main@L521, vert_main@L633, vs_main@L748, frag_main@L773, ps_main@L777

### shaders/flag_wall_cloth.raster.hlsl

SHA256：`cff48f40b4d01ff9ed202a02299e8ed1b7df136848059d168911a5101d5d4f2b`。

flag_noise@L43, vs_main@L79, ps_main@L152, vert_main@L315, vs_main@L469, frag_main@L593, ps_main@L640, vert_main@L766, vs_main@L920, frag_main@L949, ps_main@L953

### shaders/fsr_copy_history.cs.hlsl

SHA256：`d12e313fbb4c598f613bbd3057975f3e89f40879369c6630066384dbc212bc04`。

cs_main@L11

### shaders/fsr_init_reactive.cs.hlsl

SHA256：`e9e5a1f9deb2f33d1ff819c6a92740f2b351a19f6a65c96c30c27bc03f882db9`。

cs_main@L16

### shaders/fxaa.cs.hlsl

SHA256：`7cd4f276793a174502c0b78a26803204c3d2bb05169562421a563eab1a913fa9`。

comp_main@L31, cs_main@L580

### shaders/fxaa.inc.hlsl

SHA256：`e84f94c3601ea5dcf645f29ec21f9afd22d911bb7806b75c74c6d05d77c88df1`。

fxaa@L13

### shaders/garden/exclude_other_masks.cs.hlsl

SHA256：`a785d8145d821ca879aae496e9863e38ff7978ebcdc91bad098d47ff665afc30`。

cs_main@L14

### shaders/garden_lowpoly_flower.raster.hlsl

SHA256：`4d75a3769ea75f78fda54f852681d81e9eb33a32b76a453c8cb95f39fbdfb286`。

vs_main@L84, ps_main@L123, ps_main@L135, vert_main@L248, vs_main@L321, frag_main@L411, ps_main@L419, vert_main@L536, vs_main@L609, frag_main@L632, ps_main@L636

### shaders/garden/marching_squares.cs.hlsl

SHA256：`40b7bc2b87e6d58c618a377294991dca5508443c9179d414ff9cfc16f0eea534`。

garden_iso@L59, bisect_iso@L66, place_stones@L87, cs_main@L140, comp_main@L344, cs_main@L2227

### shaders/inc/draw_indirect.hlsl

SHA256：`f801f97d10af05ec94914892d72216c303b68e9616d5414900cb2af799d1169f`。

increment_draw_indexed_indirect_instance_count@L19, reserve_draw_indexed_indirect_instances@L32

### shaders/inc/draw_indirect_types.hlsl

SHA256：`35359d187687ab9b774539998392da96eb5ffb6d9b9d6cc97fd9439ef72e017a`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/inc/find_root.hlsl

SHA256：`4368a63ba93e0c4c2456c9870ae6a48c70e52d08e7ed18af367fd189d95ecff8`。

find_root@L15

### shaders/garden/read_bush_positions_for_cpu.cs.hlsl

SHA256：`279077e5a8f79d64161952661a66e05cbb39098aa3749152e328a0bf9ba07d8c`。

cs_main@L21

### shaders/inc/draw_indirect_readonly.hlsl

SHA256：`1dc8d5c850fbc4a1ee2d5d1845887b0ffb9953fef5fc6fd5eca767a81e89ce61`。

load_draw_indexed_indirect_instance_count@L21, load_draw_indexed_indirect_command@L30

### shaders/garden/defs.inc.hlsl

SHA256：`f0544206fc883d8a5e216d9970d1268a1bd6d04fef8b63dd8303a7bf080cca8f`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/garden/read_garden_mask.cs.hlsl

SHA256：`47a9c1392f7499d962fba0218310d9379e41c2ba5407b81cbe78c5d4fe083b8e`。

cs_main@L14

### shaders/garden/spawn_bushes_buildings.cs.hlsl

SHA256：`ae89d2eb9fe24cc996c84e782cd336b0535058aacd2e863af453f4d947627bad`。

cs_main@L52, comp_main@L237, cs_main@L475

### shaders/garden/placement_utils.inc.hlsl

SHA256：`bedd7b230292adb0bec27a2ccc8680d6dca00fda8793cf879747e00cd9761bb7`。

grid_world_pos@L17, placement_uv@L23, rotate_axis@L30, rotate_y@L46, placement_transform@L53, store_flower03@L69, placement_excluded@L89, band_falloff_garden@L112

### shaders/garden/spawn_bushes_garden.cs.hlsl

SHA256：`7ee1fbcb4223b991237e47b94f44f1400083f01fb721760c760a2a7467d2d740`。

cs_main@L44, comp_main@L168, cs_main@L306

### shaders/garden/spawn_flowers_buildings.cs.hlsl

SHA256：`2095e3c8677ea9fd69a2ed18250c481a448fd80c8c1a13301b94b8afc80de04f`。

cs_main@L46, comp_main@L211, cs_main@L643

### shaders/garden/spawn_flowers_garden.cs.hlsl

SHA256：`59c48f0b9ab4a494168f645bbd40e69f50caedfef5db9777e1b06ca9e215d1d7`。

cs_main@L43, comp_main@L189, cs_main@L603

### shaders/garden/spawn_small_flowers_garden.cs.hlsl

SHA256：`b9a05c72c79ca17a1f658fc38a645eb7010289ed8e0e5dd4abe1f4a0b2ad2d0c`。

cs_main@L40, comp_main@L114, cs_main@L202

### shaders/gauss_blur.cs.hlsl

SHA256：`b5531598688239ae29306d92c0a3d776114d71adac31c1cec4941cf4409fa12a`。

comp_main@L29, cs_main@L81, comp_main@L112, cs_main@L152

### shaders/gauss_blur_building_mask.cs.hlsl

SHA256：`84ee847ad10d680ff874b7ef4687ca0cf6fddbc2512e3386bfa8d2a0bd37e01f`。

cs_main@L26

### shaders/gauss_blur_mask.cs.hlsl

SHA256：`3d53ae5431214fbda749d2bff5de24fcccc85701f6adb0864142b2a029c8542c`。

cs_main@L26

### shaders/gauss_blur_r16f.cs.hlsl

SHA256：`af0c21f83891b03580d845fa1776d44900175a45b7e80ae975eca4269a6fe621`。

cs_main@L25

### shaders/generate_grass.cs.hlsl

SHA256：`accab511e709d726c029a01061434855ff02322aeee379f0c704fbb9cda6e3ef`。

pack_half2@L136, cs_main@L145, spvPackHalf2x16@L606, spvUnpackHalf2x16@L612, comp_main@L617, cs_main@L1314

### shaders/inc/gaussian_rand.hlsl

SHA256：`67f44af0c295133b27a30e44159ac185b9a2609bb771f17685ccf938c29dff0d`。

gaussian_rand@L14

### shaders/inc/scatter.hlsl

SHA256：`e6b30ba7e2447e041b14dd79c416a5ade25ef0adc59255cacf4ae6fa9cbf0790`。

scatter_point@L13, scatter_cell_point@L27

### shaders/inc/grass_blade_data.hlsl

SHA256：`dddab1fa478ae8fdf24e281c7cadf6ec838348d49a09d2d58df60aa6d38b6c6c`。

grass_blade_color@L19, grass_blade_widths@L27, grass_blade_bend_xz@L33, grass_blade_dir_x@L39

### shaders/inc/atomics.hlsl

SHA256：`1945bc463202ab8410b92bda0191d55918ebffa11aa24f86e35042213876ac1d`。

atomic_append@L10, atomic_add@L18

### shaders/inc/fbm_noise.hlsl

SHA256：`b09c60e11731ec548212e942afef221291a2a2c35c1cbc35c34f4e5283d968ef`。

fbm@L10

### shaders/inc/frustum.hlsl

SHA256：`d958877db5ea63c21fb96c8186dba4c7285fccd1a940c3b63ad2146be109c321`。

extract_frustum@L15, frustum_plane_distance@L28, frustum_intersects_sphere@L34, frustum_intersects_aabb@L45

### shaders/inc/lod.hlsl

SHA256：`c0846bd2f5fcfd9d3c61a3b74405077297db0be6b54eef0ac6dd59f568532703`。

select_lod@L17, select_grass_lod@L29

### shaders/grass/grass_common.inc.hlsl

SHA256：`302f544e17a30ca7e3642810b9629213abd64f0ed5c91b62ad9d9d4753fd90d0`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/generate_hzb.cs.hlsl

SHA256：`6ade2b62e489bf3a5cd75f449956945227f9919f132b4681812ae3950d350917`。

cs_main@L40

### shaders/generate_mips.cs.hlsl

SHA256：`d01a93882420c6794752c1565c2e10a4ebec549808ec3d806b3be0713bcfb24f`。

cs_main@L42

### shaders/gi/blur_screen_radiance.cs.hlsl

SHA256：`2198540e3141dec78fd7f4dfc4d34f9acd214e9d43c3252c38f70bba8f6d188f`。

comp_main@L89, cs_main@L137

### shaders/gi/restir.inc.hlsl

SHA256：`422ecc5f28615e19a6477e459fa2442cd6f91a36f2c3c94d35c86e4cc376c104`。

pack_restir_payload@L16, unpack_restir_payload@L22, pack_restir_weights@L28, unpack_restir_weights@L34

### shaders/gi/eval_screen_radiance.cs.hlsl

SHA256：`e29fea2eae76674746696e168820f3e7922d12e0c0084bd77cf48c386df15ad6`。

unpack_half2x16@L55, cs_main@L61

### shaders/gi/reproject_screen_radiance.cs.hlsl

SHA256：`b0fb70262efab2666d3dd7bbc99ac823fafb57cbdf76efefaf5a1235275fbeee`。

cs_main@L48

### shaders/gi/trace_gi_candidates.cs.hlsl

SHA256：`8bd34ef83f503c04867f002b9c0b48056b89df5cc5a8fef6800ffddb35c5e35a`。

spvPackHalf2x16@L180, spvUnpackHalf2x16@L186, comp_main@L191, cs_main@L1870

### shaders/gi/update_gi_radiance.cs.hlsl

SHA256：`67d85d60f236eaf938589d8461d4d99c7f6c18e245eedbc1dc6d0d21f35d9b99`。

pack_half2x16@L55, cs_main@L62

### shaders/grass.raster.hlsl

SHA256：`9d7fc31bab73c8022ada273ff705e397c3185009daab5834a1df65cb1eb69b80`。

blade_color_rgb@L57, pack_normal8@L65, vs_main@L88, ps_main@L225, spvPackHalf2x16@L386, spvUnpackHalf2x16@L392, vert_main@L397, vs_main@L532, frag_main@L645, ps_main@L689, spvPackHalf2x16@L819, spvUnpackHalf2x16@L825, vert_main@L830, vs_main@L965, frag_main@L1003, ps_main@L1011

### shaders/inc/material/grass_material.inc.hlsl

SHA256：`2b2ba6fab8a3ed6da852f9c883f18c2a87120895e1c586597ac88f453c228c2b`。

init_grass_material@L18

### shaders/grass/bundles_prefix.cs.hlsl

SHA256：`fd2547a6b51af0f9e7a3c66f9030475b29a26e55bb8afecc376087f82d6dcaf0`。

cs_main@L20

### shaders/inc/prefix_scan.hlsl

SHA256：`017da1e4e6fae662d66a3b80bf0861357ef7486a024098a5f9023d0511aa975c`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/grass/calc_dispatch_args.cs.hlsl

SHA256：`47d0f061865610591bc816f773658a03107910dd2e10af626a6807197e8bd195`。

cs_main@L20

### shaders/grass/calc_grasslet_bundle_ranges.cs.hlsl

SHA256：`f09b144b6d257e194bedaaea0eea71ed8df551085936434cf687295a0bcccc26`。

cs_main@L20

### shaders/grass/calc_tiles.cs.hlsl

SHA256：`145a46db638f9e8ef45666eaab80418f515c9fe4480708daa620333af84e1048`。

cs_main@L50

### shaders/inc/projection.hlsl

SHA256：`558b8e9fb6974239ea49a6f26f08fa1472472eadb60e26c3c61796f52f2cd8f4`。

perspective_projection@L12, orthographic_projection@L26, unproject_to_view@L38

### shaders/grass_lod1.raster.hlsl

SHA256：`6b048b17a85566f502e43a9d04a9790f78e95ec9a225a9bf74b0f9ef6f374be4`。

vs_main@L32, ps_main@L176, spvPackHalf2x16@L331, spvUnpackHalf2x16@L337, vert_main@L342, vs_main@L482, frag_main@L592, ps_main@L636, spvPackHalf2x16@L763, spvUnpackHalf2x16@L769, vert_main@L774, vs_main@L914, frag_main@L949, ps_main@L957

### shaders/grass_object_push.raster.hlsl

SHA256：`81c432278f23d6c027b3eab167da6fb5e36248f4133b2c0ddfb75c86f8f4f15f`。

vs_main@L25, ps_main@L69

### shaders/grass/tile_bundle_count.cs.hlsl

SHA256：`1d440c53e2d6e9aa0d43da64ff6f0e1dc6e41650e61cabe0b92ceb11a480adb2`。

cs_main@L27

### shaders/grass/tile_bundle_gen.cs.hlsl

SHA256：`d2ee20460c7818ef234bb263da09cdeb245d9a6dd8ecab3db5b37b9c03b0ef59`。

cs_main@L26

### shaders/grass/tiles_prefix.cs.hlsl

SHA256：`ce2799e570bf794375c8ff8e684e7f1799a61379f701a2442578b4840dfe4435`。

cs_main@L19

### shaders/ice_plane.raster.hlsl

SHA256：`cd64e3ca782933b8cc4684bdb2bb6a62ce9bf02c61f088fe31948e11cf5c6977`。

vs_main@L84, swap_yz@L98, cell_hash01@L106, depth_march@L124, ps_main@L210, spvPackHalf2x16@L620, spvUnpackHalf2x16@L626, frag_main@L631, ps_main@L1396, spvPackHalf2x16@L1525, spvUnpackHalf2x16@L1531, frag_main@L1536, ps_main@L2301

### shaders/inc/stochastic_tiling.hlsl

SHA256：`e2bc35400a44ba6feb6e9252531f451e59b7c3781372b9c029323425a874ce67`。

stochastic_tile_sample@L15

### shaders/water/ice_shared.inc.hlsl

SHA256：`8696c9230e82caa79e7f95c3a7a570ab0ffe12fbdaa0f1b4f732b186f4e75fe2`。

pack_half2x16@L27, unpack_half2x16@L34, ice_frexp@L51

### shaders/instanced_roof_beams.raster.hlsl

SHA256：`06320ff78dde7cd0f424283476e8c09ba5c71b9f10a32cd1b455e9bfbd62eea8`。

vert_main@L147, vs_main@L219, frag_main@L256, ps_main@L266, vert_main@L419, vs_main@L491, frag_main@L513, ps_main@L517

### shaders/instanced_roof_spire.raster.hlsl

SHA256：`2c726f5d90070ba0ad28566e18e3769f02fbe998deee5f48b1220bf17ba12566`。

spire_cell_hash01_2@L62, spire_noise3@L95, vs_main@L130, ps_main@L182, vert_main@L303, vs_main@L541, vert_main@L659, vs_main@L897

### shaders/inc/wall_distortion.hlsl

SHA256：`dfd1f015093242b4b1df779951d7a6a6e9fe0386b6bafbb703c884d6e800fa4f`。

wall_distortion_uv@L14

### shaders/ivy/instanced_ivy_branch.raster.hlsl

SHA256：`04c9f2b8ef38594c240a00f17a304e4169abcef7b5ccde17f92bc8f9c9bf6882`。

vs_main@L73, ps_main@L179, vert_main@L313, vs_main@L692, frag_main@L784, ps_main@L802, vert_main@L921, vs_main@L1300, frag_main@L1321, ps_main@L1325

### shaders/ivy/arch_noise_discard.inc.hlsl

SHA256：`265f99b90464bc7616279bdb2b0eaf095dbb8abf0f716a7c15cfdce16e6f5c1f`。

hash@L29, hash01@L45, perlin@L51, hash3@L66, hash3_01@L79, perlin3@L113, arch_noise_discard@L142

### shaders/ivy/instanced_ivy_leaf.raster.hlsl

SHA256：`9cc6a1aca9fafbe9f86c6aaee1a255f731b61dacb7604283fa0b4415e67795ff`。

vs_main@L77, ps_main@L187, vert_main@L324, vs_main@L716, frag_main@L822, ps_main@L837, vert_main@L963, vs_main@L1355, frag_main@L1448, ps_main@L1460

### shaders/ivy/ivy.raster.hlsl

SHA256：`fc2456a6f4c8a596017fc280a7219afaeb2b5bd954fd1e494f04a354700b1bea`。

vs_main@L52, ps_main@L80

### shaders/leaf_particle.raster.hlsl

SHA256：`46a53a1e4ede86946f63d2bca3d551bae54a48d89032a0bda75eda734c54f825`。

vs_main@L67, ps_main@L112, vert_main@L242, vs_main@L255, frag_main@L361, ps_main@L385, vert_main@L499, vs_main@L512, frag_main@L545, ps_main@L553

### shaders/leaf_particles.cs.hlsl

SHA256：`89071962e660d81cd6a72a2240aa353edc5ebece792378ce516a0bf8e3a63eee`。

leaf_noise@L64, cs_main@L70, comp_main@L314, cs_main@L796

### shaders/water/water_object_push.inc.hlsl

SHA256：`95c4a385814244f61d81d64e5c94af46a857a545bffdc7118c13fcf9f09869d5`。

water_object_push@L43

### shaders/water/waves.inc.hlsl

SHA256：`1f52d49bc779cf7e1db31aab938b0ec9bf2b6698d230d6d28afcd056217cdbf9`。

waves_hash01_2@L35, water_wave_offset@L60, water_wave_displacement@L74, waves_ripple_ring@L93, water_ripple_displacement@L106

### shaders/light_binning/bin_lights_to_cells.cs.hlsl

SHA256：`d08fb5df357c88c599d66370a07db4ab4e5622fd506d2eb0c6b15ee5bb44a319`。

wang_hash@L57, cs_main@L67, spvPackHalf2x16@L225, spvUnpackHalf2x16@L231, comp_main@L236, cs_main@L444, spvPackHalf2x16@L478, spvUnpackHalf2x16@L484, comp_main@L489, cs_main@L599

### shaders/light_binning/cell_prefix_merge.cs.hlsl

SHA256：`172e549af7e7d4ce96e6b280150e9df19085cf56d5a98c7fa3b73e1754f278cf`。

cs_main@L20

### shaders/light_binning/cell_prefix_sum.cs.hlsl

SHA256：`1913960f5890b9b93771703f82b05725f5ce81e90b268058960b2ee43ec71e91`。

cs_main@L28

### shaders/light_binning/cell_subtotal_prefix_sum.cs.hlsl

SHA256：`3bd72258f517ff483eee9707b45bee59b5a1137a74aa64247101576773bdac8d`。

cs_main@L27

### shaders/lilypad.raster.hlsl

SHA256：`2b2f5f0ab2b1b55e915953f4307ba54e19b70602c9534182482aba2757594516`。

water_displacement@L87, vs_main@L153, ps_main@L235, vert_main@L386, vs_main@L601, frag_main@L700, ps_main@L729, vert_main@L862, vs_main@L1077, frag_main@L1164, ps_main@L1172, vert_main@L1303, vs_main@L1518, frag_main@L1617, ps_main@L1646, vert_main@L1781, vs_main@L1996, frag_main@L2083, ps_main@L2091

### shaders/lilypad_flower.raster.hlsl

SHA256：`51b5665018464571790d9d753c84e9282186f30eb982dd059d3489c330f4eb95`。

water_displacement@L65, vs_main@L148, ps_main@L194, vert_main@L344, vs_main@L454, frag_main@L562, ps_main@L589, vert_main@L720, vs_main@L830, frag_main@L854, ps_main@L858

### shaders/loading_screen.raster.hlsl

SHA256：`f87a066bfbbf423723b082bd92aa3a976587c981fae9755d1a08188a8b490ce9`。

vs_main@L30, ps_main@L45, frag_main@L60, ps_main@L64

### shaders/logo_animation.raster.hlsl

SHA256：`0509e4302dfb213c281a982d88652c264fa0dbdc4d0c267d58a1fa3316d28823`。

vs_main@L39, ps_main@L49, frag_main@L66, ps_main@L70

### shaders/logo_branches_animation.raster.hlsl

SHA256：`07c8eb34e7b093a6979f01d8d7123adf4f2ceb0d0c0e2e8f894b2cff79ae7000`。

vs_main@L46, ps_main@L76, vert_main@L133, vs_main@L196, vert_main@L254, vs_main@L317, frag_main@L339, ps_main@L343

### shaders/logo_overlay.raster.hlsl

SHA256：`9df014e8e93d67dbc4600916544d1c7f9cc3eb1d017acbacf51a589aac11e651`。

vs_main@L33, ps_main@L55, frag_main@L69, ps_main@L73

### shaders/lut/brdf_fg.cs.hlsl

SHA256：`4ea256ddf7fb561a7a355d96fb91f7e9b462edab19f527af9b4ee234f920ca3b`。

comp_main@L20, cs_main@L137

### shaders/lut_gallery/thumbnail.raster.hlsl

SHA256：`c332f3301b95604c18f8930f0e734fdd358d15452abed680bff3d0453b1c192f`。

vs_main@L45, ps_main@L64, vert_main@L168, vs_main@L228, vert_main@L276, vs_main@L336, frag_main@L354, ps_main@L358

### shaders/lut/water_noise.cs.hlsl

SHA256：`4a2fbe2c40ef50ae8e51b28ee0692b9d4ad593960d9214a3a1ac3b4286cd3314`。

sample_noise_bicubic@L28, cs_main@L67

### shaders/mei_shadow.raster.hlsl

SHA256：`a98414a8649f086f6d162ea39e97eb29c32c012ea8af15d30dedf887bba339f1`。

ps_main@L53, frag_main@L184, ps_main@L211

### shaders/mouse_terrain_raycast.cs.hlsl

SHA256：`25cc927d7872f60d105cd360de60b0674fed1356ac836e1ecfed98ce25fc7611`。

cs_main@L31

### shaders/nani/allocate_draw_commands.cs.hlsl

SHA256：`a42a94bd0214242c4136fee862defe1f0f52328979cde8077056870818d7dceb`。

cs_main@L34

### shaders/nani/draw_lists.inc.hlsl

SHA256：`6e8cfdc5d697e41910e2fdf759af9b0871f5b0128fd886222bf8ddff1afc7fa3`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/nani/draw_list_offsets.inc.hlsl

SHA256：`cdeecf91a08f50804cd4bce9f69dc5268ba5d530d45df0fb1ac5ceae12b0a18a`。

instance_list_offset_at@L19

### shaders/nani_balcony_door.raster.hlsl

SHA256：`0d61e8a9b40d898fdaa3fcd672a3d8dd2a6323dfb86a2d2c4ba982970406b3f0`。

hash3@L51, corner_rand@L80, vs_main@L114, ps_main@L171, vert_main@L340, vs_main@L525, vert_main@L665, vs_main@L850, frag_main@L948, ps_main@L970

### shaders/inc/window_color_customization.hlsl

SHA256：`0926b7b3603750dc6f096bb1ec92d3a33fb5b7973e2c64baedfae3d3e920a505`。

sample_window_color@L8, get_window_color@L15

### shaders/nani_bush.raster.hlsl

SHA256：`06711cacbf6fd16b42c77bddd6a8eb9b28ca3596955c2ccd294c608f54e95d94`。

hash2@L57, hash2_rand@L72, vs_main@L97, ps_main@L156, vert_main@L313, vs_main@L396, vert_main@L533, vs_main@L616, frag_main@L719, ps_main@L732

### shaders/nani/clear_draw_lists.cs.hlsl

SHA256：`d9b3482688c3cc308e55dbb3d8a0f109d69d1517bb148377e3a6940b2cf734f9`。

cs_main@L21

### shaders/nani/clear_histogram.cs.hlsl

SHA256：`92c92e08db8047e3c1771fe700b116712536db8d489bd4cb20c7279db223a39f`。

cs_main@L16

### shaders/nani/clear_subtotals.cs.hlsl

SHA256：`ce05a5134ec5a2e985ae014fe8041488473bc16ea002906abf91580e2e40a523`。

cs_main@L24

### shaders/nani_cloth.raster.hlsl

SHA256：`bf2db9e6d5beb154e23f22d91d659abd638c18472666116726139a3efd997375`。

hash2@L51, hash2_rand@L66, noise2@L72, vs_main@L108, ps_main@L173, frag_main@L263, ps_main@L267

### shaders/inc/hsv.hlsl

SHA256：`a2d2278c156f0c7ae4a6ff6c36765b4d310e80ae90fa7ab6d97aaf263d374100`。

rgb2hsv@L12, hsv2rgb@L22

### shaders/nani/cull_and_bucket.cs.hlsl

SHA256：`5ba63062d5fdaff507c3e369e14977cd06d7057b3e89cda79c2950a7fe04e6eb`。

spvPackHalf2x16@L112, spvUnpackHalf2x16@L118, comp_main@L123, cs_main@L237, spvPackHalf2x16@L345, spvUnpackHalf2x16@L351, comp_main@L356, cs_main@L500

### shaders/nani/cull_and_bucket.inc.hlsl

SHA256：`f74fd0b2b9d213899f5c332677e752e765db9a32326f87c314c0473306372b5b`。

decode_instance_meta_size@L48, decode_instance_mesh_index@L54, cull_frustum_side_planes@L60, cull_append_visible@L74

### shaders/nani/atlas_part_info.inc.hlsl

SHA256：`bfec5b919ce1f38cc18b2f7b9f84b435ddcade920c7684d4837859054995d2bf`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/nani/cull_and_bucket_bush.cs.hlsl

SHA256：`3c82e3eacdb3a1f334af98a551cd869dd1df4f6a1fdf3fd92c2822d1d242cde8`。

comp_main@L124, cs_main@L239, comp_main@L359, cs_main@L498

### shaders/nani/inc/cull_and_bucket_owned_custom.hlsl

SHA256：`3277adbe7dbf256d3a8cf875b7eff203c5eef7629383c221f2dd2caad8ee2334`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/nani/cull_and_bucket_curved_halftimber.cs.hlsl

SHA256：`227886bbe89c9600bbd3df7085bd3fa1c5f70b9b1106c0dfc42330040ae72d47`。

comp_main@L122, cs_main@L230, comp_main@L348, cs_main@L489

### shaders/buildings/curved_halftimber.inc.hlsl

SHA256：`be03fc1e34f6a5d94218c7a2cc954b7b785f60eff211587d58e7f8b80a7f9e47`。

evaluate_curved_halftimber@L21

### shaders/nani/cull_and_bucket_flowers.cs.hlsl

SHA256：`cbe1a7529b768b94d84ea9b19c43522e1f52335b16e36c4592553d2b8577f4d2`。

comp_main@L124, cs_main@L241, comp_main@L361, cs_main@L503

### shaders/nani/cull_and_bucket_plaster.cs.hlsl

SHA256：`c114b090c7ff0982579a25ca41d01e6f740748dd4373130dbb003121c873c851`。

comp_main@L116, cs_main@L229, comp_main@L341, cs_main@L428

### shaders/plaster.inc.hlsl

SHA256：`4d5f29fab52fcc846990b40f9a31fa81fccb1fedf583fcfaa1a36660ee7b9c6d`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/nani/cull_and_bucket_roof_tile.cs.hlsl

SHA256：`3012f586b216029539388f5442574757b3657f6aa08b7f488586d5395cdedc23`。

spvPackHalf2x16@L119, spvUnpackHalf2x16@L125, comp_main@L130, cs_main@L241, spvPackHalf2x16@L358, spvUnpackHalf2x16@L364, comp_main@L369, cs_main@L501

### shaders/roof/roof_tile.inc.hlsl

SHA256：`db0b25ee2b412ef57b81b9ad6291614771596cc8da8e7506577bef07f4d402d4`。

pack_half2x16@L46, unpack_half2x16@L53, quat_to_matrix@L59

### shaders/nani/cull_and_bucket_smoke.cs.hlsl

SHA256：`c8b8e334d47662d40e4dfe03919cc08640be31d47e1b6d67302d6c5bcac70690`。

comp_main@L103, cs_main@L216, comp_main@L315, cs_main@L458

### shaders/smoke.inc.hlsl

SHA256：`c51f853d94ed40934d9f3b3f9c4314b29c32e4ef4136aec33bb3c02a679e3ac9`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/nani/cull_and_bucket_stairs_floor.cs.hlsl

SHA256：`46f39b75335f66674e8584b368c386f401edbfcf2381206e94809b28d7b1139f`。

comp_main@L121, cs_main@L234, comp_main@L351, cs_main@L494

### shaders/stairs_floor.inc.hlsl

SHA256：`afdc7b312576c979009adf988b1f91b7e1f40b389cf55f3bcc25c6907a075df8`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/nani/inc/trimesh.hlsl

SHA256：`8d1e01119b24b871f9327ee4bc23a579290e9ef96f0508d0df86ea348ec00052`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/nani/cull_and_bucket_wooden_plank.cs.hlsl

SHA256：`f5821e6b1caa3667f6bd37de18e123eb85a3c71bab7a187b42192d7c7e11c6c5`。

comp_main@L118, cs_main@L254, comp_main@L370, cs_main@L530

### shaders/wooden_plank.inc.hlsl

SHA256：`9a238316f4f9dec88f84b27b40ecba8e0143be70d832a1332532352be23de493`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/nani_curved_halftimber.raster.hlsl

SHA256：`d02c588ae113d92e4c3a7d6feacc546133cf50a68faeeb3b31c169ed4236f52b`。

vs_main@L79, ps_main@L119, vert_main@L288, vs_main@L549, frag_main@L676, ps_main@L789, vert_main@L929, vs_main@L1190, frag_main@L1217, ps_main@L1221

### shaders/inc/parallax_mapping.hlsl

SHA256：`eb0d89325ca1853700e5aa019a400365cbbeb00b36cd69f896de5d8eebdb96f8`。

parallax_mapping@L13

### shaders/nani/draw_indexed_indirect_to_dispatch.cs.hlsl

SHA256：`8d8482d41b63d1904dd4c1654b5500fd9a8b51d9671b5e09a5fd200226b1d47d`。

cs_main@L26

### shaders/nani/fill_sorted_instance_list.cs.hlsl

SHA256：`6741fc1c48c05564324cb7539a335cfb3018b1050311033b58dce0fcd6339483`。

cs_main@L34

### shaders/nani/fill_unsorted_instance_indirection.cs.hlsl

SHA256：`542c05855bac0796f480f65550c50164b41847f8136541d745984a4ddf7c439a`。

cs_main@L31

### shaders/nani/fill_unsorted_instance_list.cs.hlsl

SHA256：`e766715d8eca8447fb5eae381125dd34cd004f6d4641039690f3013d8405373f`。

cs_main@L36

### shaders/nani_flower.raster.hlsl

SHA256：`10121b8df641e12b1f4e45d65155d32390055241fdcae1a2710d55c7d497e7fd`。

hash2@L41, hash2_rand@L56, vs_main@L80, ps_main@L178, vert_main@L319, vs_main@L502, vert_main@L634, vs_main@L817, frag_main@L843, ps_main@L847

### shaders/nani/generate_dispatch_args.cs.hlsl

SHA256：`2a5bbd3193111c968045c7db3960352384477b2928facf467b513344edad8780`。

cs_main@L19

### shaders/nani/generate_draw_lists.cs.hlsl

SHA256：`5f7e1d420a6a24c2e654e81fb483e8e7fc063ee9547cc10cb83aad926f63722d`。

cs_main@L43

### shaders/nani_gothic_window_bricks.raster.hlsl

SHA256：`e84fede1c2527431b42a0cf3072940973952dfd9c12c9dabb54fb6752f9f520a`。

hash3_rand@L68, vs_main@L96, ps_main@L142, spvPackHalf2x16@L344, spvUnpackHalf2x16@L350, vert_main@L355, vs_main@L622, frag_main@L822, ps_main@L1197, spvPackHalf2x16@L1356, spvUnpackHalf2x16@L1362, vert_main@L1367, vs_main@L1586, frag_main@L1638, ps_main@L1682

### shaders/wall/inc/brick_vs_out.hlsl

SHA256：`493d989ccac898390781adfb246f507929ad2e8a20204eb5f82b25f5a3e8212b`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/wall/inc/brick_instance_flags.hlsl

SHA256：`22d5540d347d60ce7b29173116b3c5a5c8805a9126dba84ef5d2df9c93c550a2`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/wall/inc/brick_frag.hlsl

SHA256：`5a058d3c7f059cf221c90e37921eeca24617f18715dff67b7594fc1821bd533a`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/inc/wall_color_customization.hlsl

SHA256：`b8d239fb171a1906f0f4a84a0d3e73e2c8fa8d1972512474f7ba7040a55d1854`。

get_wall_custom_color@L9, get_wall_default_color@L23

### shaders/inc/roof_colors_include.hlsl

SHA256：`9cb1ef0e3b97c7436c07b5aee7f6cbcdafee802768bfe35990d45ece8a2a5dc3`。

get_roof_color@L8, get_roof_default_color@L17

### shaders/wall/inc/highlight_mode.hlsl

SHA256：`2e99cb336ba52e7fb2f9c0140ec7f091da32861759b0f0e21a0a6cd077f48aa2`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/nani/group_draw_cmd_prefix_sum.cs.hlsl

SHA256：`f343b1c596f42b1cb76935799fcc86afc3ba9b3f9f3bba60ef30c03670b62e89`。

cs_main@L33

### shaders/nani/histogram_prefix_sum.cs.hlsl

SHA256：`acd38151a9e59435e3523fc94c714a2a32d3fe423df45e35d314b09832273d30`。

cs_main@L17

### shaders/nani_instanced_roof.raster.hlsl

SHA256：`408258c3e935db38e1c63277a81b8d9a0cb9cd89d24c879b9c321945c06c54b6`。

vs_main@L81, ps_main@L130, spvPackHalf2x16@L299, spvUnpackHalf2x16@L305, vert_main@L310, vs_main@L891, frag_main@L1047, ps_main@L1410, spvPackHalf2x16@L1551, spvUnpackHalf2x16@L1557, vert_main@L1562, vs_main@L2143, frag_main@L2213, ps_main@L2307

### shaders/roof/dormer_hole.inc.hlsl

SHA256：`e43e435eb7a25763f5511980bd6c7c4c4aff746c7fdd30258628e82a1b15c4a0`。

is_inside_dormer_hole@L27

### shaders/nani_lantern_emissive.raster.hlsl

SHA256：`f697533a124928dccdd5bd88a1610641d6d7a23393243d0d9b63732536ed56e4`。

vs_main@L63, ps_main@L105

### shaders/nani_plant.raster.hlsl

SHA256：`46e232b1a529daf6801c55b65506e647020f42afa6ea1434a640cd0136bca642`。

hash2@L53, hash2_rand@L68, vs_main@L89, ps_main@L146, vert_main@L271, vs_main@L361, frag_main@L399, ps_main@L406, vert_main@L524, vs_main@L614

### shaders/nani_plaster.raster.hlsl

SHA256：`5c556ca2204d538942d9150215edde1c502d2cd8792cc22b1cbf63332a7aaf4d`。

vert_main@L125, vs_main@L425, frag_main@L517, ps_main@L1547, vert_main@L1686, vs_main@L1986, frag_main@L2045, ps_main@L2334

### shaders/nani/prefix_sum_unsorted_instances_per_atlas_entry.cs.hlsl

SHA256：`2bf2dca1e74347071b2f01f259a6f5965680979449677fab3e0c976123dcccda`。

cs_main@L35

### shaders/nani/prepare_unsorted_draw_lists.cs.hlsl

SHA256：`f91bdd41f139808cb39335d8febca700734410df00c233c85b556d2425803f24`。

cs_main@L30

### shaders/nani_preplaced_flower.raster.hlsl

SHA256：`df9d7eb0ba81e850329b1dcf69348af6e5412454765560e7956603ccac705e2a`。

hash2@L57, hash2_rand@L72, vs_main@L95, ps_main@L171, vert_main@L320, vs_main@L423, vert_main@L558, vs_main@L661, frag_main@L686, ps_main@L690

### shaders/nani_roof_floor.raster.hlsl

SHA256：`a687f0271497437cc126e08b0601c24b13fc8ce9ff481f4cf88abfbb6b91e98c`。

vs_main@L100, ps_main@L130, vert_main@L389, vs_main@L423, frag_main@L625, ps_main@L2536, vert_main@L2697, vs_main@L2731, frag_main@L2906, ps_main@L3877

### shaders/roof_material_id.inc.hlsl

SHA256：`c0fcdc89acac15b440ea5a6f57b9a688801cd5438e4291e148041c75dbaba76e`。

get_roof_material_params@L34

### shaders/roof/wooden_floor.inc.hlsl

SHA256：`19a331ee2af8762e3ac6163c36ee4ba72adcb271933e0b8eb79dd6d8fc8e218c`。

wooden_floor_sample@L13

### shaders/roof/stone_floor.inc.hlsl

SHA256：`fb40035446635f0e22f3b9d7d3d0252970fa0e49cfa0e1c0bd73dab8c9bd597a`。

stone_floor_march_height@L15, stone_floor_sample@L47

### shaders/nani_smoke.raster.hlsl

SHA256：`736d7c3aae655ccc69caffffc75c39d048719830eb1284e40354dfcf7191c71f`。

vert_main@L109, vs_main@L555, frag_main@L647, ps_main@L795, vert_main@L912, vs_main@L1358, frag_main@L1440, ps_main@L1583

### shaders/nani_solid_albedo_map.raster.hlsl

SHA256：`e5caca4a7e81e8bfb48d7d78c03d3c604e6a8279f9c3561aa1b7391b224bbad6`。

vs_main@L72, ps_main@L108, vert_main@L259, vs_main@L279, vert_main@L419, vs_main@L439, frag_main@L468, ps_main@L472

### shaders/nani_solid_albedo_map_emissive.raster.hlsl

SHA256：`d1d0e5da15d46ef3cccec3473d43ca040cb034e9852efc6c48d2d7592725419d`。

vs_main@L77, ps_main@L114, vert_main@L272, vs_main@L291, vert_main@L427, vs_main@L446, frag_main@L532, ps_main@L570

### shaders/nani_solid_vertex_color.raster.hlsl

SHA256：`394343dd78ca8fc40a40f73e04a833eab816be1a3e858e6bba55c6d4d18dbe31`。

vs_main@L66, ps_main@L101, vert_main@L260, vs_main@L278, vert_main@L409, vs_main@L427, frag_main@L522, ps_main@L544

### shaders/nani_solid_vertex_color_bricks.raster.hlsl

SHA256：`017d43aeb158bade2f2733879112ab65951d58363cc98e844e33f6928a08925a`。

spvPackHalf2x16@L132, spvUnpackHalf2x16@L138, vert_main@L143, vs_main@L375, frag_main@L566, ps_main@L941, vert_main@L1088, vs_main@L1278, frag_main@L1327, ps_main@L1371

### shaders/nani_solid_vertex_color_window.raster.hlsl

SHA256：`d82809eacbd2807f78ea892e56df510382f0fc5bc78d3665582a9d6f09c3fac1`。

window_hash@L53, window_corner_rand@L78, vs_main@L110, ps_main@L164, frag_main@L300, ps_main@L344, frag_main@L435, ps_main@L457

### shaders/nani_solid_vertex_color_window_glass.raster.hlsl

SHA256：`6e87980a4d53a0a83f6b06489270c56cb44471a3a09cf891edfd819d5492ddf3`。

spvBitfieldUExtract@L127, spvBitfieldUExtract@L133, spvBitfieldUExtract@L139, spvBitfieldUExtract@L145, spvBitfieldSExtract@L151, spvBitfieldSExtract@L159, spvBitfieldSExtract@L167, spvBitfieldSExtract@L175, vert_main@L183, vs_main@L369, spvBitfieldUExtract@L513, spvBitfieldUExtract@L519, spvBitfieldUExtract@L525, spvBitfieldUExtract@L531, spvBitfieldSExtract@L537, spvBitfieldSExtract@L545, spvBitfieldSExtract@L553, spvBitfieldSExtract@L561, frag_main@L569, ps_main@L927, spvBitfieldUExtract@L1068, spvBitfieldUExtract@L1074, spvBitfieldUExtract@L1080, spvBitfieldUExtract@L1086, spvBitfieldSExtract@L1092, spvBitfieldSExtract@L1100, spvBitfieldSExtract@L1108, spvBitfieldSExtract@L1116, vert_main@L1124, vs_main@L1310, spvBitfieldUExtract@L1426, spvBitfieldUExtract@L1432, spvBitfieldUExtract@L1438, spvBitfieldUExtract@L1444, spvBitfieldSExtract@L1450, spvBitfieldSExtract@L1458, spvBitfieldSExtract@L1466, spvBitfieldSExtract@L1474, frag_main@L1482, ps_main@L1504

### shaders/nani_stairs_floor.raster.hlsl

SHA256：`1d8292c8722205b8529ccc5a978724cb52eb11c4a9b68aac25cd31e6a8e0a0e7`。

vert_main@L134, vs_main@L207, frag_main@L338, ps_main@L640, vert_main@L784, vs_main@L857, frag_main@L878, ps_main@L882

### shaders/nani/subtotal_prefix_sum.cs.hlsl

SHA256：`003e7394e3aefe2e2ed51105ce137163fc8f688dd64c7c59d827c3d57a56a6de`。

cs_main@L28

### shaders/nani_uber.raster.hlsl

SHA256：`601f59f301df3f8f78382a4018e8f6037653ef09c6945be2488b59c335e14ba7`。

vs_main@L70, ps_main@L105, vert_main@L268, vs_main@L288, vert_main@L421, vs_main@L441, spvBitfieldUExtract@L535, spvBitfieldUExtract@L541, spvBitfieldUExtract@L547, spvBitfieldUExtract@L553, spvBitfieldSExtract@L559, spvBitfieldSExtract@L567, spvBitfieldSExtract@L575, spvBitfieldSExtract@L583, frag_main@L591, ps_main@L613

### shaders/nani_velocity_only.raster.hlsl

SHA256：`e05143c3230f98a7f8102bbde534884a93319277f12a49b078368c99fb63b378`。

vert_main@L103, vs_main@L123, frag_main@L155, ps_main@L160, vert_main@L264, vs_main@L284, frag_main@L303, ps_main@L307

### shaders/nani_wooden_plank.raster.hlsl

SHA256：`41fe5d4b60c606f6f9a8b8249311c8a97a3077500ab8f09164f7d3b7df69034a`。

vert_main@L132, vs_main@L396, frag_main@L524, ps_main@L592, vert_main@L737, vs_main@L1001, frag_main@L1032, ps_main@L1036

### shaders/normal_debug.raster.hlsl

SHA256：`46f9e90a2b2a5a50ce82fda7b7dd14a84e9873f4e4171fcb6b82aa64569e1013`。

vs_main@L33, ps_main@L45

### shaders/painterly/acquire_target.cs.hlsl

SHA256：`8fcf62cfbdb99c6827deb7362ce92b0fdff09ccee13943db1d5aa6a35357988e`。

cs_main@L31

### shaders/painterly/convolve_target.cs.hlsl

SHA256：`ccf25371cf66ae59058503cab5a1c52227f3368dfc20b200166bc0801e6eb47e`。

cs_main@L26

### shaders/painterly/debug_target.raster.hlsl

SHA256：`933d8d6746f0b17d2c35ffef2e063b8aa0e63fd19ca16d325ea55825b07678c2`。

ps_main@L12

### shaders/painterly/generate_strokes.cs.hlsl

SHA256：`3a5e53cfefc047018c164d69708df2a167468b6d1167973ccd1a44b442a4f0b1`。

cs_main@L39

### shaders/painterly/stroke.inc.hlsl

SHA256：`8ead34efa09449982c2ad9b77cb8e4608c1dd15c46f666807554ae8bbfce55c7`。

pack_half2@L23, unpack_half_x@L30, unpack_half_y@L36, unpack_stroke_pos@L42, unpack_stroke_size_rotation@L48

### shaders/painterly/raster_strokes.raster.hlsl

SHA256：`ff08089f47f22d8471313ff9f64a5f0f0642299b9f4b188c714de91f88a48d1d`。

vs_main@L39, ps_main@L81, frag_main@L107, ps_main@L111

### shaders/path_pebble.raster.hlsl

SHA256：`d920f18b6f0b699c85ed330b1404c9b714a47ec0497dfbdb27eed379e4aecdc3`。

vs_main@L83, ps_main@L110, vert_main@L234, vs_main@L249, frag_main@L288, ps_main@L302, vert_main@L411, vs_main@L426, frag_main@L447, ps_main@L451

### shaders/path_pebbles.cs.hlsl

SHA256：`9a76fe19f0820aecedfea117901babe40fca824488f5a5c5ddaedcc7cdccc242`。

cs_main@L58, comp_main@L169, cs_main@L342

### shaders/paths/combine_path_exclusions.cs.hlsl

SHA256：`38bb71858d50192521f3abd622eefa8e52afae6fa6247b39535dd5058459dd64`。

cs_main@L21

### shaders/paths/directional_contract.cs.hlsl

SHA256：`f16ce2d485477d2cd4cdf131cd8ee078bb7545b090f203c82ab143d8d172205b`。

cs_main@L32

### shaders/paths/path_mask_visual.cs.hlsl

SHA256：`ddfaf81052d1e74b4dcef7f7e1c0d77dadd09aef49d8fb67fb517d966b0f0400`。

vnoise_fbm@L50, cs_main@L81

### shaders/prefix_scan/inclusive_prefix_scan.cs.hlsl

SHA256：`20d75e1b3417da80fb80ce0aa9c88fa00da29996d381407508a63fd493480efd`。

cs_main@L18

### shaders/prefix_scan/inclusive_prefix_scan_merge.cs.hlsl

SHA256：`8c01400d9aa9145e02607bbb23b9f562695e3654e5c090ca856cc5ea8156c130`。

cs_main@L16

### shaders/prefix_scan/inclusive_prefix_scan_subtotals.cs.hlsl

SHA256：`f42050247642ee9046b163823aef97d7c4ae3b5ef1618e753a9700ed481f6aeb`。

cs_main@L19

### shaders/primary_rt_debug.cs.hlsl

SHA256：`ac8afed9d678ade34dded5960e722ffb23a984e3bec421fe4e3c7ccaabeaf96d`。

decode_slot_xor_key@L110, decode_slot_tri_count@L116, unpack_half2x16@L122, cs_main@L141

### shaders/rain_particles.raster.hlsl

SHA256：`3e43be2d81a2d6196c3fd3026150f748869f19c9ebd3d89997d372dec551d81a`。

vs_main@L41, ps_main@L75, vert_main@L168, vs_main@L198, vert_main@L296, vs_main@L326

### shaders/rasterize_buildings.cs.hlsl

SHA256：`849a40942f47a72904dd4c543a0795d906fdcb2804176dabe058c2ac8d6ce8be`。

cs_main@L21

### shaders/rasterize_buildings_animate.cs.hlsl

SHA256：`e97afe9cd3f766b666b0dec74931493efa21acce2eaef0503e7e4e87e03f7733`。

cs_main@L16

### shaders/rdi/temporal.cs.hlsl

SHA256：`107edb079f81cf4fc42a2323ebc8afac81371a9b122efb68334324a9c8735663`。

unpack_half2x16@L72, pack_half2x16@L77, cs_main@L87

### shaders/read_mouse_depth_value.cs.hlsl

SHA256：`295016fa51558f109a7504eeff28f043b59784405c637d1c1a17dee88905815c`。

cs_main@L36, comp_main@L183, cs_main@L257

### shaders/read_terrain_stroke.cs.hlsl

SHA256：`7d3e43f4a86da2bcb7ddef2c74f96f43363d24701a265bb723ff4b2a3b54799a`。

cs_main@L35

### shaders/reed.raster.hlsl

SHA256：`e7c01232afb8e9e4881bbeefc67a5ea04d951235764203897d4128c0c1f72f5d`。

water_displacement@L66, vs_main@L148, ps_main@L226, vert_main@L369, vs_main@L582, frag_main@L674, ps_main@L699, vert_main@L826, vs_main@L1039, frag_main@L1062, ps_main@L1066, vert_main@L1191, vs_main@L1404, frag_main@L1496, ps_main@L1521, vert_main@L1650, vs_main@L1863, frag_main@L1886, ps_main@L1890

### shaders/resolve_to_swapchain.cs.hlsl

SHA256：`394c375d0a2d6cb336d0b3a72fb2dd07f356c1e0983a3a3d4f56ff411b76492c`。

cs_main@L19

### shaders/rocky_terrain/displace_rocky_terrain.cs.hlsl

SHA256：`b5523547bd149e065a078b582a02b5aa6727256aff2af14008e327d131dce573`。

sample_rocky_terrain_channel@L74, value_noise@L87, cs_main@L103

### shaders/rocky_terrain/inc_mesh.inc.hlsl

SHA256：`e4c2afc5339c49d15d6607275e2bbe4a355133bbd3bc1703f7a9454e902e8c84`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/rocky_terrain/rocky_terrain.raster.hlsl

SHA256：`248221629a691f398fb02d6bf6b88c52f13d277c18e1ba1a01b659a349106861`。

vs_main@L87, ps_main@L122, frag_main@L244, ps_main@L248

### shaders/rocky_terrain/triangle_adjacency.inc.hlsl

SHA256：`a85090d91eaec114b940997f3ebdd5a251f347f1cf6837a8222342de32fe7f49`。

nearest_edge_index@L24, distance_to_edge@L36

### shaders/rocky_terrain/stairs/stairs.cs.hlsl

SHA256：`72210186cfd8dc9276b70ea7329d49f56be316b824c244674ca9743d74601d52`。

sample_rocky@L71, water_mask_value@L80, cs_main@L93, comp_main@L185, cs_main@L3728

### shaders/rocky_terrain/stairs/place_stairs.inc.hlsl

SHA256：`37ec0af51151490b44b6766c6323611e00193a298395ccd39db2b16a2f0d1271`。

place_stairs@L28

### shaders/rocky_terrain/stairs/marching_squares.inc.hlsl

SHA256：`2130d79c0157f34a90a2c0ca6ca04706fabc96e24dd85fcbf76739c595273e91`。

place_stair_case@L23

### shaders/rocky_terrain/stairs/isolines.inc.hlsl

SHA256：`9ce1060a3eb07b5413c1a09bb46bac72d802fc0e07e3dea9816d93dcbaf83ab8`。

place_stair_isolines@L22

### shaders/rocky_terrain/stairs/stairs_counter.cs.hlsl

SHA256：`09ff048def55c8d0e81fe1bb9c87b22ee6a7c0c3fe0c982a92bb10273f5468dc`。

cs_main@L19

### shaders/rocky_terrain/stairs/stairs_indirect.raster.hlsl

SHA256：`cd313b5b0724f1094de2189ec72f8fcf2667216013b8741c0b7aebb24ac50077`。

stairs_hash_corner@L79, stairs_noise_octave@L98, vs_main@L194, ps_main@L292, frag_main@L343, ps_main@L347

### shaders/roof_snow.raster.hlsl

SHA256：`52fec6faeaec5ae845cef4de62a85afdd55bb89de48acd9ba37462696f6eac11`。

snow_hash4@L33, snow_corner@L43, snow_noise3@L63, flag_noise@L81, vs_main@L111, ps_main@L155, frag_main@L200, ps_main@L214, frag_main@L240, ps_main@L248

### shaders/roof_tiles_gravity.cs.hlsl

SHA256：`4edec6823c7bf9b44dc86867ef647c1b64a3113ca4aca732192123b27ecac156`。

cs_main@L70, spvPackHalf2x16@L257, spvUnpackHalf2x16@L263, comp_main@L268, cs_main@L364

### shaders/screenshot/capture_screenshot_buffer.cs.hlsl

SHA256：`727d9930cd3fbf3293016e3a40aab5c57246305ebacf38b6067474baf12661c1`。

encode_srgb@L25, pack_rgba8_opaque@L31, sample1@L43, sample4@L59

### shaders/screenshot/capture_screenshot.inc.hlsl

SHA256：`83306bc65338790d50cf241e16d2a0e9c84cb1bbae22eb34252d250abc6063b3`。

capture_direct@L11, capture_gaussian@L17, capture_gaussian_msaa@L52

### shaders/screenshot/capture_screenshot_texture.cs.hlsl

SHA256：`79440d2a5609ca3ac0f002506d23ca43e47ac79590726bee97915926730ef1cb`。

encode_srgb@L24, encode4@L30, sample1@L36, sample4@L51

### shaders/session_selection/background.raster.hlsl

SHA256：`e80812b9a78188f0592a2e7a808d13c2d0400761e83be2aa34b3dc1bf32ef628`。

ps_main@L30, frag_main@L104, ps_main@L162, frag_main@L180, ps_main@L184

### shaders/session_selection/new_glade_thumbnail.raster.hlsl

SHA256：`b4e350ea2ca09528dae17e9ec67cacd4ebe211620d94927ba518ff5829d03f10`。

vs_main@L38, ps_main@L57, vert_main@L96, vs_main@L158, vert_main@L201, vs_main@L263, frag_main@L281, ps_main@L285

### shaders/session_selection/thumbnail.raster.hlsl

SHA256：`9ddb4716a51fd1413785165b47ead58a7143ab4bcdcf28debf6fbf0fab431168`。

vs_main@L39, ps_main@L61, vert_main@L113, vs_main@L177, vert_main@L221, vs_main@L285, frag_main@L303, ps_main@L307

### shaders/session_selection/thumbnail_painterly.raster.hlsl

SHA256：`5a952aeeb873b56ede4e998fbb3ac5480e0e66b41d59ecf72373c01fbc8c2031`。

vs_main@L44, ps_main@L130, vert_main@L182, vs_main@L311, vert_main@L360, vs_main@L489, frag_main@L508, ps_main@L512

### shaders/shader_refactoring.cs.hlsl

SHA256：`4b0b366c9510f39a25dac739c00cda6e52a37c267d84224903bac9c0d959c8aa`。

capture@L19, compare@L25

### shaders/shadow_mask_mean.cs.hlsl

SHA256：`5e6ccd4ecfe822a1f10f7d74887cd8d5aa6b71c987451069d0944a7af2c2b55f`。

cs_main@L23

### shaders/sheep.raster.hlsl

SHA256：`f9b4ac23f8cd482a28303aead629b850b7c96266b6418495a3340c6bec1ec9f8`。

vs_main@L67, ps_main@L113, vert_main@L232, vs_main@L256, vert_main@L372, vs_main@L396

### shaders/skydome.raster.hlsl

SHA256：`64f91fbdab1fc136163ede0c512e5098ce6ae63e895ea230630d556df4bc4101`。

vs_main@L50, ps_main@L62, vert_main@L200, vs_main@L210, frag_main@L309, ps_main@L346, vert_main@L448, vs_main@L458, frag_main@L480, ps_main@L484

### shaders/snow_particles.raster.hlsl

SHA256：`d5674f8ec59edb3be50c523bee8b9ca611b6dc47e174235750042763def40a3d`。

get_autofocus@L37, get_camera0@L42, snow_camera_cutoff@L53, vs_main@L79, ps_main@L194

### shaders/solid_vertex_color.raster.hlsl

SHA256：`ba46924f4bb04ca5b741954e0e373db5af49eee2e969547b83e6c7c09b3e516e`。

vs_main@L50, ps_main@L75, frag_main@L210, ps_main@L258, frag_main@L348, ps_main@L370

### shaders/solid_vertex_color_albedo_map.raster.hlsl

SHA256：`6d2827550b0ba722b6547555b8b9aed0893c4aa7f1dcf5f62bb6e2489b4e4ed7`。

vs_main@L56, ps_main@L81, frag_main@L201, ps_main@L223

### shaders/solid_vertex_color_boat.raster.hlsl

SHA256：`e09ddcd1dc0819e5d212c70c1586315eaa26252f6f76ebf2de91135813219ff8`。

vert_main@L106, vs_main@L122, frag_main@L160, ps_main@L170, vert_main@L282, vs_main@L298, frag_main@L321, ps_main@L325

### shaders/solid_vertex_color_garden_stone.raster.hlsl

SHA256：`95299cd94d8caf40e84172c9f7b605b8ea053dc5d426efd46857766d0199290d`。

vert_main@L109, vs_main@L125, frag_main@L164, ps_main@L174, vert_main@L289, vs_main@L305, frag_main@L329, ps_main@L333

### shaders/solid_vertex_color_waterwheel.raster.hlsl

SHA256：`3448ef2cb3d533ed16611d0a0beaf8f69a2b5dcfbbb44d18e0498e642d1c3325`。

vert_main@L110, vs_main@L145, frag_main@L184, ps_main@L194, vert_main@L310, vs_main@L345, frag_main@L369, ps_main@L373

### shaders/solid_vertex_color_wooden_gate.raster.hlsl

SHA256：`d91791a0370c7ca5ae4751d8c56e21a8a7020b04bf29ab7e8ef56b459294d10a`。

vs_main@L59, ps_main@L91, frag_main@L141, ps_main@L145

### shaders/sparkles.raster.hlsl

SHA256：`f1a37e57ea949db0b7a34f5d6435c26daf92b03a985aaa30b825fa497fbbde5f`。

sparkle_noise@L36, vs_main@L86, ps_main@L157

### shaders/splash_particle.raster.hlsl

SHA256：`1e6ec6ddccdaae75f9fa5e2648b99c0c4690f8d43638fe9ab728a1c114d4ab5a`。

vs_main@L57, drop_ring@L84, ps_main@L94, frag_main@L240, ps_main@L366

### shaders/ssgi/ssgi.cs.hlsl

SHA256：`7e33e1f84520eac055105284e400ddec6234a87febac65ced25ac1da63cd648c`。

cs_main@L42, comp_main@L256, cs_main@L403

### shaders/ssgi/XeGTAO.hlsl

SHA256：`7d29e59f1e8cd897ab4b50a3978e110ae8ba40a9a50833af1b21c315b69b0e57`。

XeGTAO_Internal_FastACos@L20, XeGTAO_Internal_ProjectedNormalLen@L28

### shaders/ssgi/vaShared.hlsl

SHA256：`421d9b24c9a5dc7cb395a3e873266cf99ece1dd796f7966f5aa7986d30342955`。

va_depth_to_view_z@L19

### shaders/ssgi/temporal_filter.cs.hlsl

SHA256：`f84481b051f351d10539aaf58e614be958afbdd92c68d80cce4a4faff24f9d1b`。

cs_main@L46, comp_main@L195, cs_main@L253

### shaders/ssgi/upsample.cs.hlsl

SHA256：`12e459d8686ec059415edcab1a9873a96b3d6378c1767da36745add4d78ebfc7`。

cs_main@L33, comp_main@L161, cs_main@L226

### shaders/stairs_foliage_exclusion.raster.hlsl

SHA256：`e316d7532e51ae9341d797a2a6c364cfe990e41108eb1f3091096dd835cca40f`。

vert_main@L29, vs_main@L35, frag_main@L69, ps_main@L78, vert_main@L112, vs_main@L118, frag_main@L146, ps_main@L154

### shaders/star.raster.hlsl

SHA256：`9bc97cd03560488f76f29a15cd21e0f60e9e232babb91d4d49c572e54bff4642`。

vs_main@L45, ps_main@L84, vert_main@L194, vs_main@L242, frag_main@L341, ps_main@L375, vert_main@L477, vs_main@L525, frag_main@L548, ps_main@L552

### shaders/taa.cs.hlsl

SHA256：`c6a0385501f85a78b19912aa0f34b6e2ddac6feba93ce19dab4f6b8706a13960`。

cs_main@L44, incb_frexp@L204, comp_main@L309, cs_main@L484

### shaders/inc/unjitter_taa.hlsl

SHA256：`ce4a795800f004e4804835d86a1409bb05e8870d1d56400a461cbac1e5fba875`。

unjitter_taa_pixel@L8, unjitter_taa_uv@L14, taa_history_uv@L21

### shaders/terrain_derivative.cs.hlsl

SHA256：`4f14681e0341d37c0f416a061f7768db8415b5b01f9722dac1bf6c5f295903ec`。

sample_terrain_height@L23, cs_main@L33

### shaders/terrain_editing/cache_stroke.cs.hlsl

SHA256：`0495b773da7f68768904ef8ef7915c311ed5d9d637515215edb27292d37ea2cf`。

cs_main@L11

### shaders/inc/smooth_max.hlsl

SHA256：`43beb814c78c45e64205d1b9e389599bac1546cf2671e01f9c4d0af79daccb18`。

smooth_max@L8

### shaders/terrain_editing/calc_highlight_hill_mask.cs.hlsl

SHA256：`3640161881a3c0af38407a7d51066de0678c9d67213ef6434b1a373970c93ac9`。

cs_main@L20

### shaders/terrain_editing/floor.raster.hlsl

SHA256：`07631377e141a54df118e0d84572ff3184bc4b7ba67a86e0e7547c519138538a`。

vert_main@L120, vs_main@L256, frag_main@L452, ps_main@L1108, vert_main@L1237, vs_main@L1373, frag_main@L1392, ps_main@L1396

### shaders/clearing/inc/forest_floor.hlsl

SHA256：`3a26287e1329618f755af8150022dd636d9be5e3b08e353741af822d00ce6ca8`。

forest_floor_blend@L14

### shaders/terrain_editing/path_cobblestone_surface.inc.hlsl

SHA256：`f2d3673ec355364e5cb35265fd2c0f2023832e713eb0ebddcece64c8cd78c961`。

evaluate_cobblestone_surface@L26

### shaders/terrain_editing/rasterize_terrain_stroke.cs.hlsl

SHA256：`8926728c207872b33404cab3992a02896a68311a62ccb14bfbc6f8d22fc6dcff`。

spvPackHalf2x16@L44, spvUnpackHalf2x16@L50, comp_main@L55, cs_main@L1529

### shaders/terrain_editing/inc_terrain_stroke.inc.hlsl

SHA256：`c318e330284b4ff212ed02e8953d5c36a7bf37744e7244d808e0a3807b8caf2b`。

stroke_distance@L23, stroke_influence@L29

### shaders/terrain_editing/sdf_utils.inc.hlsl

SHA256：`22065602113f71ab6a388aa6a6569bb5316e292908972c2c3e01b1e036e9f681`。

sd_segment@L8, sd_box@L17, sd_circle@L24, sd_smooth_union@L30, sd_smooth_intersect@L37

### shaders/terrain_editing/read_terrain_height.cs.hlsl

SHA256：`f1e03c4338bcb1836a237b65868d8c6fd081b055618e4f5304ba6b923bd05fe3`。

pack_height@L19, cs_main@L25

### shaders/terrain_editing/read_terrain_rock_mask.cs.hlsl

SHA256：`c92763f6c3ab914cc5ece8e37ff12442ce773be5146eac35d808d722363b34e9`。

cs_main@L16

### shaders/terrain_lighting_texture.cs.hlsl

SHA256：`fc98ea3e3be29821cccff477453065647efb4ff82764445020e6c8951bbb38ce`。

cs_main@L91, comp_main@L318, cs_main@L468

### shaders/text.raster.hlsl

SHA256：`160daa92747e2d984d861f98c8c2165298c6c4c741465bf1af58543c7a9591cb`。

vs_main@L62, decode_rgba8@L116, hash_noise_octave@L135, glyph_sample@L180, apply_sprite@L204, ps_main@L290, vert_main@L395, vs_main@L513, spvTextureSize@L569, frag_main@L576, ps_main@L1048, vert_main@L1112, vs_main@L1230, frag_main@L1251, ps_main@L1255

### shaders/texture_test.raster.hlsl

SHA256：`328cffc389bf0effea804554d6f1f9ab0ed6b589087cb7d9752d4a8140a9c4f4`。

vs_main@L44, ps_main@L54, frag_main@L67, ps_main@L71

### shaders/tgsr.cs.hlsl

SHA256：`210dee924949782310c19830e14c9d46bab22bcc469e2c44eda003b24f8ab7fc`。

cs_main@L57, incb_frexp@L314, comp_main@L431, cs_main@L749, incb_frexp@L766, comp_main@L879, cs_main@L1146, incb_frexp@L1162, comp_main@L1275, cs_main@L1542, incb_frexp@L1558, comp_main@L1675, cs_main@L1993

### shaders/tgsr_common.inc.hlsl

SHA256：`b25e8c66d28251ed1abf8b85b4a44dc47d04678676e365adabac4448a94d4efe`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/tgsr_blur_lum.cs.hlsl

SHA256：`7cbd9e9bded80cac1927247aad3c83acc6a9b10eede2e094b19498976163b69c`。

cs_main@L25

### shaders/tgsr_reproject.cs.hlsl

SHA256：`8c922d9265d4a7da1bd9657b48fb19bfa23ed7f2834d9e14066c3314575ad21d`。

cs_main@L35

### shaders/timed_terrain_gizmo.raster.hlsl

SHA256：`6742e21fc532b5412e8dfaea837059a2ea6d7392a3f9f759a6f24b01dc53d8f0`。

vs_main@L40, ps_main@L60, vert_main@L150, vs_main@L174, frag_main@L206, ps_main@L211, vert_main@L305, vs_main@L329

### shaders/timer_circle.raster.hlsl

SHA256：`2e2ad17ba47ca0c7144337fd81988bb17b249b1292000890acfce36a7cc003b6`。

vs_main@L29, ps_main@L47, frag_main@L69, ps_main@L73

### shaders/ui_2d.raster.hlsl

SHA256：`961a9ecd1e9997ac1339547fde9254062c2375c767245d28525f705b3fbe5d87`。

vs_main@L31, ps_main@L50, frag_main@L75, ps_main@L79

### shaders/ui_2d_color_button.raster.hlsl

SHA256：`a444d891faffa26e81bf435bd74c3b50f353414bbee5859acc02e91ee6f6f676`。

vs_main@L31, ps_main@L50, frag_main@L74, ps_main@L78

### shaders/ui_2d_ws.raster.hlsl

SHA256：`a8d24e94637c1232fcdbe760384ddc39db4ff9a05b6593d1258b4e0a4905ac9a`。

vs_main@L32, ps_main@L54, frag_main@L68, ps_main@L72

### shaders/upload_cpu_raster.cs.hlsl

SHA256：`cee24e5186a0090f77de6c10d56964b8b6a5cbf285e83fc0012cc3d1ed9b28af`。

decode_rgba8@L24, load_pixel_word@L36, r16f@L47, rgba16f@L58

### shaders/vermeer/acquire_target.cs.hlsl

SHA256：`1c0e6b27d1189f89905e2675ab610d0aed36639e8748abfe75ca3f6a7907fc8e`。

cs_main@L27

### shaders/vermeer/combine.raster.hlsl

SHA256：`65018e47b18ab292bbb6d5dd0d8999fde0966a49ae52ab950696b56efbce4a7b`。

ps_main@L36, frag_main@L71, ps_main@L75

### shaders/vermeer/stroke.inc.hlsl

SHA256：`5b5bc226490e6d1cb3c3c2480ca366a16103d3376db2c202bbca7f1d3497e4cf`。

pack_half2@L21, unpack_half_x@L28, unpack_half_y@L34, unpack_stroke_pos@L40, unpack_stroke_size_rotation@L46

### shaders/vermeer/convolve_target.cs.hlsl

SHA256：`e8241f91fd82e9c0544febc483d3506a406457ea9cc39fbb4d5ff96b3e7461eb`。

cs_main@L25

### shaders/vermeer/generate_strokes.cs.hlsl

SHA256：`186c08bf74112dd019733e89aee8c6b81676f0f7c4e3e2a8e860dc5739ddb297`。

spvPackHalf2x16@L42, spvUnpackHalf2x16@L48, comp_main@L53, cs_main@L353, spvPackHalf2x16@L398, spvUnpackHalf2x16@L404, comp_main@L409, cs_main@L1548

### shaders/vermeer/raster_strokes.raster.hlsl

SHA256：`f5d71e87bba30c8b3d11e240815427055383c507371c5f048f4a11f5c1dbba8f`。

vs_main@L45, ps_main@L105, frag_main@L141, ps_main@L145

### shaders/vertex_color.raster.hlsl

SHA256：`be554cb11ce0336f6ed814bb5cf630e73214cfe55725b87da4a674ea42adb9fb`。

vs_main@L30, ps_main@L39

### shaders/wall/generate_draw_lists.cs.hlsl

SHA256：`80bf4e14bc9ab94fb396dbcb9929603c06f8feb8f3876e92c5c6251cbc1f622a`。

wall_brick_draw_list_slot@L106, load_brick_transform@L112, cs_pass1@L133, cs_pass2@L250, comp_main@L504, cs_pass1@L636, comp_main@L754, cs_pass2@L925

### shaders/wall/inc/brick_instance.hlsl

SHA256：`b3b6ca250e6f2cd7784ec24ea79456c87dcfcd4fe97a77370de57c26dd8744f9`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/wall/inc/draw_lists.hlsl

SHA256：`df3842ed17b27c7469820e1a4e43154fc08773af8881262cdaeda32720dacb9b`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/wall/reset_pass2_inst_counts.cs.hlsl

SHA256：`8aef3932f0413228da8519bf5c53223136c0e1bb8bdf4cd1dcee04dd55978862`。

cs_main@L21

### shaders/wall/wall_brick_lod0.raster.hlsl

SHA256：`9cd465f521917b0c888e89b3169e90f6431e3a4a0a20de527d77fb0290005de8`。

vert_main@L136, vs_main@L599, frag_main@L769, ps_main@L1135, vert_main@L1284, vs_main@L1661, frag_main@L1702, ps_main@L1732

### shaders/wall/inc/brick_vert.hlsl

SHA256：`31adb06555bc7063292bbeb8ee1523a5f141224560c7e9af610076c13e026e5d`。

无可扫描函数定义（宏、include或数据声明）；不据此判空实现。

### shaders/wall/wall_brick_lod1.raster.hlsl

SHA256：`246e12d8e85e49f1bd1ba2580b12c9748978c37f1cb4273bc7adf0ec2294597d`。

vert_main@L131, vs_main@L475, frag_main@L667, ps_main@L1212, vert_main@L1357, vs_main@L1624, frag_main@L1663, ps_main@L1693

### shaders/water/calc_deferred_rt_args.cs.hlsl

SHA256：`28671a57177ff1434f7e25dd734b3786f15f06a810ac3f9eb8f94ee614a565fe`。

cs_main@L28

### shaders/water/deferred_rt.cs.hlsl

SHA256：`1e096744c4d81baa297cc64aa37eabf6377419ff1a3bd365b6ac472e37592741`。

sample_blue_noise@L90, pack_half2x16@L98, unpack_half2x16@L104, trace_bl_bvh@L121, water_terrain_march@L304, resolve_quad_pixel@L382, cs_main@L442, spvPackHalf2x16@L679, spvUnpackHalf2x16@L685, comp_main@L690, cs_main@L1721

### shaders/water/draw_water_mask.cs.hlsl

SHA256：`9545ae0e498c79e75b0782a49dd80e9fec330aa3f1a26b9c5dbdfc6fa0530f30`。

cs_main@L19

### shaders/water/ice_cracks.raster.hlsl

SHA256：`73451be4a210b41b1731dcb26b372f3f9ee744315726aac0e547ac41a6391f2d`。

vs_main@L69, ps_main@L86, ps_main@L107, frag_main@L120, ps_main@L124

### shaders/water/ice_deferred_rt.cs.hlsl

SHA256：`ee0ac1a03dfdbdd5b1690c342fe6f8a5d2c298778e153bf298fe7f019e08b110`。

sample_blue_noise@L92, trace_bl_bvh@L111, water_terrain_march@L295, cs_main@L370, spvPackHalf2x16@L640, spvUnpackHalf2x16@L646, comp_main@L651, cs_main@L1296

### shaders/water/ice_refl_march.cs.hlsl

SHA256：`62529b00e0ed7b454d5430a8391b3b8ef10eac23ef3e8fa8a9dc8099215e636f`。

swap_yz@L89, hilbert_index@L97, cs_main@L120, spvPackHalf2x16@L471, spvUnpackHalf2x16@L477, comp_main@L482, cs_main@L919

### shaders/water/ice_refl_resolve.cs.hlsl

SHA256：`0025fcd61115d52b3ec000c2339236f628b48a74f01301427209d1542eb16325`。

swap_yz@L68, cs_main@L76, incb_frexp@L215, comp_main@L325, cs_main@L451

### shaders/water/ice_refr_resolve.cs.hlsl

SHA256：`8b935b904289daa858b2d83cfdad6b609cafb42e2e4253c26f300666c7e0cb03`。

swap_yz@L51, normal_z_from_xy@L60, cs_main@L66, spvPackHalf2x16@L211, spvUnpackHalf2x16@L217, comp_main@L222, cs_main@L285

### shaders/water_plane.raster.hlsl

SHA256：`3683b2b12fddfc1dae90c3761a54f6fc611da31f373e3fb1176a462a38259352`。

vert_main@L103, vs_main@L114, spvPackHalf2x16@L276, spvUnpackHalf2x16@L282, frag_main@L287, ps_main@L1498, vert_main@L1608, vs_main@L1619, spvPackHalf2x16@L1745, spvUnpackHalf2x16@L1751, frag_main@L1756, ps_main@L2223

### shaders/water/water_surface_frag.inc.hlsl

SHA256：`1d9ea0db6ee4156f53c438b66d6c50a9ebd82533f3f841214fcb4296cca9978b`。

pack_half2x16@L25, unpack_half2x16@L31, halfres_depth_march@L39, clip_to_slab@L111, ps_main@L136, ps_main@L374

### shaders/water_plane_refraction_copy.raster.hlsl

SHA256：`3d9ecf84d03ae2f7d74f31461091910c12664f78c5e85018d9598e7aef5df2c3`。

vs_main@L72, ps_main@L96, ps_main@L106, frag_main@L119, ps_main@L123

### shaders/water/read_water_mask.cs.hlsl

SHA256：`f453eaf2a8e804f84ec4d71e9ef65e6c05434dbe221bcd954d83831e2000fb33`。

cs_main@L17

### shaders/water_snow_plane.raster.hlsl

SHA256：`ffed3eda266035921c32bf6f67df1128ef7d2574c81b289b10b2210c72296109`。

vs_main@L67, ps_main@L85, ps_main@L119, vert_main@L230, vs_main@L240, frag_main@L274, ps_main@L337, vert_main@L429, vs_main@L439, frag_main@L467, ps_main@L529

### shaders/water/spawn_boats.cs.hlsl

SHA256：`75e7d96942da083914e383676eb4c3c4d7697076ec1cfc18c244ed6fc0d5ab54`。

rotate_axis@L22, cs_main@L62, comp_main@L235, cs_main@L331

### shaders/water/spawn_lilypads.cs.hlsl

SHA256：`41227523ab0f271254a5c9120e4c09c1c245e7b37ec66e542fd13c85d4e00c88`。

rotate_axis@L25, band_falloff@L43, cs_main@L72, comp_main@L282, cs_main@L718

### shaders/water/spawn_reeds.cs.hlsl

SHA256：`c5563e85dbb61d778569eaa05710d09a034a6e6c9b9a527cd2ac47470899ed1f`。

rotate_axis@L23, band_falloff@L42, cs_main@L64, comp_main@L163, cs_main@L342

### shaders/water/spawn_waterwheels.cs.hlsl

SHA256：`b90d54065a6d0f05535949c94b475c8e3316b4225e2c69ad3d2ae76e78675110`。

cs_main@L19

### shaders/water_x_paths.raster.hlsl

SHA256：`c438d4a134b0ae2bcb049128305765ba38b3201db7ed04dc5cd89c2c4e6cb102`。

vs_main@L82, ps_main@L140, ps_main@L219, frag_main@L305, ps_main@L419, frag_main@L438, ps_main@L442

### shaders/water/waterfall_vert.inc.hlsl

SHA256：`f1ef965df5057b947fde86c23bee52bfd1ebf5f6a14d376daaf8f70726aa2cad`。

waterfall_hash3@L36, vs_main@L63

### shaders/waterfall.raster.hlsl

SHA256：`ae5294405a37bebaadfda997e87470d2646082e9610acfd54ca289b18204f930`。

vert_main@L113, vs_main@L336, spvPackHalf2x16@L498, spvUnpackHalf2x16@L504, frag_main@L509, ps_main@L1720, vert_main@L1840, vs_main@L2063, spvPackHalf2x16@L2189, spvUnpackHalf2x16@L2195, frag_main@L2200, ps_main@L2667

### shaders/waterfall_refraction_copy.raster.hlsl

SHA256：`4a9f8e590e8b516298eb1c44e7e1d269296b8b813363c08955717fe3d71b6f47`。

ps_main@L52, ps_main@L63, frag_main@L76, ps_main@L80
