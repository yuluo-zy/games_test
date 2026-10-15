# systems/render_loop/draw_passes/deferred_raster.rs

> 文件作用：延迟光照光栅：`deferred_raster`（W43，把 G-buffer 反照率/法线 + 光照累积到光照目标，读 SSGI/屏幕阴影/ReSTIR/灯光网格）。

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster` | 0x1409d20b0 | 42 | W43 延迟光照：串证据 "Apply deferred lighting"；Duration::as_secs_f32（时间 uniform）+ RasterPass::new + AssetSsboLibrary::get + InstanceBuffer::instance_count + DrawPacket::new + stencil_ref + set_blend_mode/depth_write/depth_test/stencil_test/compare_op/compare_mask；bind_shadowmap + bind_terrain_heightmap；shader `/deferred/{solid,grass,plant,canopy}_light_sun_sky.raster.hlsl`；draw_no_vao + bind_color_library + bind_ray_tracing；读 LanternSsbo/SpatialResolveOffsetsSsbo、ssgi_tex/screen_shadows_tex/rdi_reservoir_tex/light_indirection_grid/grid_light_indices | O | render-loop\decomp\decomp-0x1409d20b0.txt（串：Apply deferred lighting）；wp5 draw-passes §5 |

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 1 项（DeferredRasterResources 默认）

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `deferred_raster`（0x1409d20b0，6203B）—— 完整接线（d_gap §3）
- **ABI 表**（O）：`rcx` rg、`rdx` world、`r8` = `[Handle<Texture>;2]`（`[1]` = 颜色附件 tag1、`[0]` = 视口来源）、`r9` = 深度（tag0 可写、无清除）、栈参 5..13（逐个对应 binding hash）。
- **5 条目** `ENTRIES = [(1,F),(3,F),(0,F),(0,T),(2,F)]` × 着色器数组（资源 `0x78cac557…`，None → `Option::unwrap` panic）；`flag` 决定 `byte3=(b3&0x9f)|0x20` 与模板 `|0x80`；收尾状态 `byte0&=0x7f, byte1|=1, byte2=(b2&3)|0xe8, byte3=(b3&0xe0)|0x10`；`draw_no_vao(3)`。
- **绑定序列 33 步**（逐条 hash/来源见 `bind_all`）：`Time` = `ctx.elapsed.as_secs_f32()`；Lantern uniform = `PassHost::ssbo_buffer_word(LanternSsbo)`；`bind_ray_tracing::<Tri,Meta,Bvh,Wall>`（i_api 版）；读 ssgi_tex/screen_shadows_tex/rdi_reservoir_tex/light_indirection_grid/grid_light_indices + LanternSsbo/SpatialResolveOffsetsSsbo。
- 着色器 `/deferred/{solid,grass,plant,canopy}_light_sun_sky.raster.hlsl`；"Apply deferred lighting"。
- 帧序：GI 链（W37 ssgi → W40 bin_lights → W41 restir → W42 apply_screen_shadows）→ **W43 deferred_raster** → W34 late_forward（注：W43 在 W34 之后注册但同帧，deferred 光照目标）。

## 关键事实

- 结构体：`DeferredRasterResources`（U 字段）。
- 帧序 W43（frame-wiring，bin_lights W40 → ssgi W37 → 屏幕阴影 W42 → 延迟光照 W43 → late_forward W34 之后主 pass）。
