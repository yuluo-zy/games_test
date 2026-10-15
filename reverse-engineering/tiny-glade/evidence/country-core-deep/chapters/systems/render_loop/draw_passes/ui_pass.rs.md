# systems/render_loop/draw_passes/ui_pass.rs

> 文件作用：UI pass（世界图内 UI 层）：`ui_pass`、`loading_screen_pass`（加载屏光栅）。

函数总计 2：业务 2 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::draw_passes::ui_pass::ui_pass` | 0x140837c40 | 17 | UI pass：RasterPass::new + to_draw_packet + set_blend_mode（wp5 draw-passes §10）；图层 UI=19（draw_lists） | O | render-loop\decomp\decomp-0x140837c40.txt；ui-input\decomp\decomp-0x140837c40.txt；wp5 draw-passes §10 |
| `country_core::systems::render_loop::draw_passes::ui_pass::loading_screen_pass` | 0x140837ff0 | 47 | 加载屏 pass：RasterPass::new + to_draw_packet + set_blend_mode（wp5 §10）；图层 LOADING_SCREEN=18 | O | render-loop\decomp\decomp-0x140837ff0.txt；ui-input\decomp\decomp-0x140837ff0.txt；wp5 draw-passes §10 |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

- `ui_pass`（0x140837c40）与 `loading_screen_pass`（0x140837ff0）**同一名字 "UI"**（d_main §2），仅图层不同（UI=19 / LOADING_SCREEN=18）：RasterPass::new + to_draw_packet + set_blend_mode；无状态改写。
- 帧序：世界图闭包尾部（compile 0x140161cd0 内，render_world 之后、EguiBackend::draw 前后）；加载图实例（0x1401647b0）用 loading_screen_pass。
- 与 `ui_2d_pass`/`mei_pass` 的区别：本文件是世界图内 UI 层（draw_lists 图层 18/19），ui_2d_pass 是最终图 UI（egui/2D 几何）。

## 关键事实

- 与世界图内 UI 层（draw_lists 图层 18/19）对应。
