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

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### 1. `refraction_copy_pass`（0x1409bf320，W44）
RasterPass::new + CommonDrawParams::new + 遍历 + to_draw_packet + set_depth_write。**条件 `*0x14362c098（demo）==0`**（round3 O：非 0 整段跳过）。shader `_water_plane_refraction_copy`/`_waterfall_refraction_copy.raster.*`。

### 2. `water_surface_pass`（0x1409bf800，W45 非冬季）—— 完整接线（B_d_water §2）
- 读列表 **15**（WATER_SURFACE，`DesignatedRenderPass(0xf)` 实体）。
- 缓冲：`water_rt_count_buffer`（`new_gpu_only(0x10, 0x20)`，`clear_rg_buffer` 清零）、`water_rt_ray_buffer`（**size = `(n+(n>>4))<<4`，n = w*h** 32 位）。
- 绑定名（全部 FNV 反解，O）：A=`half_depth_tex`、B=`taa_history_tex`、C=`refraction_tex`、D=`reprojection_tex`、u8=`history_valid`（栈参顺序 A,B,u8,C,D,lists）。
- `bind_common` flag 恒 false；`Time` → CommonDrawParams.time。
- 栅格每项：A,B,history_valid,D,**write_rg(count),write_rg(ray)**,C；`state.byte3=(b3&0x9f)|0x40`（**没有** byte0&=0xbf，与 pre/post-TAA 不同）。
- 计算 pass：`Water deferred rt args`（`/water/calc_deferred_rt_args.cs.hlsl`，**dispatch_1d(1)**）、`Water deferred rt`（`/water/deferred_rt.cs.hlsl`，`dispatch_indirect_raw(count, 0)`）；`bind_ray_tracing`（软件 BVH：RtTriangles/RtTriangleMeta/RtBvh/WallColorId）。
- 格式：water refl hits 0x53、water refl hit radiance 0x62（半分辨率 = `max(v-v/2,1)`）、water refr temporal 0x7a(+sampled+storage)。

### 3. `ice_surface_pass`（0x1409c08d0，W45 冬季）—— 完整接线（B_d_water §2）
- 3 纹理 + 4 缓冲（2 个 `rg_buffer_copy_from_slice<u32>`）；`Ice shadows` temporal pair。
- 5 颜色：`[tag0 color, tag1 roughness(视图字=四个 1.0f32), tag1 refr hits(0), tag2 refr hit radiance, tag1 Ice shadows 当前帧(0)]`；上一帧 → `shadows_history_tex`。
- `history_valid`（栅格）= `u8!=0 ? pair标志 : 0`；march/trace = u8 原值；refl resolve = `(flag1 & flag2) & 1`。
- 计数缓冲初值 `[0,1,1,0]`（rodata 0x142abfce4）；射线尺寸：冰 px=`4*(n+(n>>4))`、ray=`(n+(n>>4))<<4`。
- 4 计算 pass：`Ice refl march/trace`、`Ice refr/refl resolve`（URI `/water/ice_*.cs.hlsl`）；两个 resolve 是 `dispatch_2d(color 的 extent)`；两处 `*_hit_radiance_tex` 以 `Some(TextureViewDesc{format 0x7b, usage 4})` 绑定；blue noise 在 march/trace/水 rt 各绑两次。
- 磁盘纹理键 `0x579f39ab91a01865 = xxh3("blue_noise")`（B_c3 已证）；预设号：9 terrain_heightmap、0xa visual_terrain_heightmap、8 terrain_lighting、0 brdf_fg_lut、0x11 shadow_map。

### 4. pre/post-TAA 透明（W46/W53）
RasterPass + to_draw_packet + set_depth_write + set_blend_mode（透明 nani raster：window_glass/cloth…）；post 有 `Vec::is_empty` 守卫（空列表不建 pass）。串 "Pre-TAA transparency"/"Post-TAA transparency"。
- 帧序：W44（demo 门控）→ W45（is_winter 选水/冰）→ W46 → TAA/TGSR（W52）→ W53。

## 关键事实

- Shader URI（O，wp5）：水 `/water/{deferred_rt,calc_deferred_rt_args}.cs.hlsl` + `_water_plane.raster`；冰 `/water/ice_{deferred_rt,refl_march,refl_resolve,refr_resolve}.cs.hlsl` + `_ice_plane` + `/water/ice_cracks.raster.hlsl`；折射 `_water_plane_refraction_copy`/`_waterfall_refraction_copy`。
- 资源：`rt_bl_bvh`/`rt_triangles`/`rt_triangle_meta`/`WallColorIdSsbo`（bind_ray_tracing）、`frame_constants`、透明颜色（写）。
- 帧序：W44→W45（水或冰，按 ActiveSession 冬季/季节）→W46（TAA 前）→TAA→W53（TAA 后）。
