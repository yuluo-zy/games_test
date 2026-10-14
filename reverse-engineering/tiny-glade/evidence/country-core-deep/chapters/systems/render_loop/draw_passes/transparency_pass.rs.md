# systems/render_loop/draw_passes/transparency_pass.rs

> 文件作用：透明/水面 pass 族：`refraction_copy_pass`（W44，折射拷贝）、`water_surface_pass`（W45 非冬季，水面 + 软件 BVH 光线步进 + deferred rt）、`ice_surface_pass`（W45 冬季，冰面）、`pre_taa_transparency_pass`（W46，TAA 前透明）、`post_taa_transparency_pass`（W53，TAA 后透明）。

函数总计 5：业务 5 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass` | 0x1409bf320 | 33 | W44 折射拷贝：RasterPass::new + CommonDrawParams::new + 遍历 + to_draw_packet + set_depth_write；条件 `\*0x14362c098==0`；串仅调试串（(knNOpening… 为 lottie/web 噪音，非业务） | O | render-loop\decomp\decomp-0x1409bf320.txt；wp5 draw-passes §4 |
| `country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass` | 0x1409bf800 | 72 | W45 非冬季水面：RenderGraphRecorder::create + Handle::desc + Iterator::product（extent）；raster（`_water_plane.raster` + to_draw_packet + set_blend_mode）+ compute（`/water/deferred_rt.cs.hlsl`、`/water/calc_deferred_rt_args.cs.hlsl`：bind_terrain_heightmap + bind_terrain_lighting + write_texture + dispatch_1d）+ `dispatch_indirect_raw` + `bind_ray_tracing`（软件 BVH rt_bl_bvh/rt_triangles/rt_triangle_meta）；串证据：Water deferred rt、ice roughness tex、ice refr hits/radiance、ice_refl_count_buffer、ice_rt_count_buffer | O | render-loop\decomp\decomp-0x1409bf800.txt；wp5 draw-passes §4 |
| `country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass` | 0x1409c08d0 | 174 | W45 冬季冰面：同水 + bind_shadowmap + Temporal::into_inner + read_texture_view + dispatch_2d；串：`/water/ice_{deferred_rt,refl_march,refl_resolve,refr_resolve}.cs.hlsl` + water refr temporal + `_ice_plane`/`_water_ice_cracks.raster` | O | render-loop\decomp\decomp-0x1409c08d0.txt；wp5 draw-passes §4 |
| `country_core::systems::render_loop::draw_passes::transparency_pass::pre_taa_transparency_pass` | 0x1409c3a80 | 448 | W46 TAA 前透明：RasterPass::new + to_draw_packet + set_depth_write + set_blend_mode（透明 nani raster：window_glass/cloth…）；串证据：Pre-TAA transparency | O | render-loop\decomp\decomp-0x1409c3a80.txt；wp5 draw-passes §4 |
| `country_core::systems::render_loop::draw_passes::transparency_pass::post_taa_transparency_pass` | 0x1409c40b0 | 494 | W53 TAA 后透明：`Vec::is_empty` 守卫 + 同组装；串证据：Post-TAA transparency | O | render-loop\decomp\decomp-0x1409c40b0.txt；wp5 draw-passes §4 |

## 关键事实

- Shader URI（O，wp5）：水 `/water/{deferred_rt,calc_deferred_rt_args}.cs.hlsl` + `_water_plane.raster`；冰 `/water/ice_{deferred_rt,refl_march,refl_resolve,refr_resolve}.cs.hlsl` + `_ice_plane` + `/water/ice_cracks.raster.hlsl`；折射 `_water_plane_refraction_copy`/`_waterfall_refraction_copy`。
- 资源：`rt_bl_bvh`/`rt_triangles`/`rt_triangle_meta`/`WallColorIdSsbo`（bind_ray_tracing）、`frame_constants`、透明颜色（写）。
- 帧序：W44→W45（水或冰，按 ActiveSession 冬季/季节）→W46（TAA 前）→TAA→W53（TAA 后）。
