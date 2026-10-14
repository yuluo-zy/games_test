# systems/render_loop/draw_passes/shadow_pass.rs

> 文件作用：阴影图光栅：`shadow_pass`（W23，太阳阴影 atlas 层光栅）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::draw_passes::shadow_pass::shadow_pass` | 0x1409cc270 | 20 | W23 阴影 pass：puffin scope + utils::get_res_mut + RasterPass::new + DepthAttachment::layer（shadow atlas 层）+ CommonDrawParams::new + 遍历 DrawListItem + to_draw_packet + RasterPipelineOpts::set_depth_compare_mode；shader `_mei_shadow.raster.*`（I：无同名文件，阴影图光栅） | O | render-loop\decomp\decomp-0x1409cc270.txt；wp5 draw-passes §3 |

## 关键事实

- 阴影 atlas 分层（DepthAttachment::layer）。
- 帧序 W23（frame-wiring，主 pass 前）。
