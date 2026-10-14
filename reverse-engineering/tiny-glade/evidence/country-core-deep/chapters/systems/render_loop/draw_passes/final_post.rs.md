# systems/render_loop/draw_passes/final_post.rs

> 文件作用：最终后处理（最终图）：`final_post`（色调映射/grading LUT/ICC LUT）、`resolve_to_swapchain`（MSAA 解析到交换链）。

函数总计 4：业务 2 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::draw_passes::final_post::final_post` | 0x14090d000 | 44 | 最终后处理：串证据 `Final blit` + `default`/`assets/luts/.png`（LUT 资产默认名）+ msaa color out；ImageFormationConfigs::current + ColorGradingState::flattened + RenderGraphRecorder::create；闭包绑定 LUT（GradingLut::lut_name_to_asset_name）+ read_texture + uniform；`_final_post.raster`（色调映射/grading_lut/icc_lut/tony_mc_mapface_lut/shadow_mask_mean） | O | render-loop\decomp\decomp-0x14090d000.txt（串：Final blit、assets/luts/.png、msaa color out）；wp5 draw-passes §6 |
| `country_core::systems::render_loop::draw_passes::final_post::resolve_to_swapchain` | 0x14090d850 | 195 | 解析到交换链：串证据 `Resolve to swapchain` + `raster_resolve_to_swapchain` + `raster_resolve_msaa_to_swapchain`；Handle::desc 对比 + get_resource；`_blit_resolve_to_swapchain`/`_blit_resolve_msaa_to_swapchain`（MSAA 档）/`_blit_blit_stretch_decode_srgb` | O | render-loop\decomp\decomp-0x14090d850.txt（串：Resolve to swapchain、raster_resolve_to_swapchain、raster_resolve_msaa_to_swapchain）；wp5 draw-passes §6 |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 2 项（closure$0/closure$1 行 120/244：read_texture + uniform）

## 关键事实

- 最终图属 `tiny_glade::render_loop`（R12–R14），country-core 仅提供 final_post 组装（wp5 O 名）。
- LUT 默认资产：`assets/luts/.png`（占位名，ColorGrading 域）。
