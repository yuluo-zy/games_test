# systems/render_loop/draw_passes/feedback_report_bg.rs

> 文件作用：反馈报告背景：`render_feedback_report_bg`（反馈报告 UI 背景渲染）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::draw_passes::feedback_report_bg::render_feedback_report_bg` | 0x1408a5a70 | 22 | 反馈报告背景：串证据 `Feedback report bg` + `logo_overlay.raster.hlsl` + `logo_overlay`（复用 logo 光栅 shader）；RasterPass::new + DrawPacket::new + read_texture + set_depth_test_enable | O | render-loop\decomp\decomp-0x1408a5a70.txt（串：Feedback report bg、logo_overlay.raster.hlsl）；wp5 draw-passes §10 |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

- `render_feedback_report_bg`（0x1408a5a70）：RasterPass::new + DrawPacket::new + read_texture（背景图）+ set_depth_test_enable；着色器复用 `/logo_overlay.raster.hlsl`。
- 三个门控（d_main §1.7 提到 feedback 三个门）：反馈报告 UI 可见才渲染（I）。
- 帧序：世界图闭包尾部（render_world 后、render_menu_bg 附近，compile 0x140161cd0 内 0x148a8 区）。

## 关键事实

- 复用 `/logo_overlay.raster.hlsl`（与 logo_overlay 域共享 shader）。
