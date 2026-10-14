# systems/render_loop/draw_passes/deferred_raster.rs

> 文件作用：延迟光照光栅：`deferred_raster`（W43，把 G-buffer 反照率/法线 + 光照累积到光照目标，读 SSGI/屏幕阴影/ReSTIR/灯光网格）。

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster` | 0x1409d20b0 | 42 | W43 延迟光照：串证据 "Apply deferred lighting"；Duration::as_secs_f32（时间 uniform）+ RasterPass::new + AssetSsboLibrary::get + InstanceBuffer::instance_count + DrawPacket::new + stencil_ref + set_blend_mode/depth_write/depth_test/stencil_test/compare_op/compare_mask；bind_shadowmap + bind_terrain_heightmap；shader `/deferred/{solid,grass,plant,canopy}_light_sun_sky.raster.hlsl`；draw_no_vao + bind_color_library + bind_ray_tracing；读 LanternSsbo/SpatialResolveOffsetsSsbo、ssgi_tex/screen_shadows_tex/rdi_reservoir_tex/light_indirection_grid/grid_light_indices | O | render-loop\decomp\decomp-0x1409d20b0.txt（串：Apply deferred lighting）；wp5 draw-passes §5 |

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 1 项（DeferredRasterResources 默认）

## 关键事实

- 结构体：`DeferredRasterResources`（U 字段）。
- 帧序 W43（frame-wiring，bin_lights W40 → ssgi W37 → 屏幕阴影 W42 → 延迟光照 W43 → late_forward W34 之后主 pass）。
