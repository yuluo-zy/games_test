# systems/render_loop/draw_passes/wireframe_pass.rs

> 文件作用：线框调试 pass：`wireframe_pass`（世界图线框叠加）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::draw_passes::wireframe_pass::wireframe_pass` | 0x1409ccb20 | 17 | 线框 pass：RasterPass::new + CommonDrawParams::new + set_polygon_mode/set_depth_test_enable（wp5 draw-passes §10）；图层 WIREFRAME=20 | O | render-loop\decomp\decomp-0x1409ccb20.txt；wp5 draw-passes §10 |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

- `wireframe_pass`（0x1409ccb20）：RasterPass::new + `CommonDrawParams::new` + **`set_polygon_mode`（byte3 bit7）**/`set_depth_test_enable(false)`（byte0 bit7 清除，I 位名）+ to_draw_packet；图层 WIREFRAME=20。
- 帧序：世界图闭包尾部（render_world 后、ui_pass 前）；`WireframePass` 组件的实体 → draw_lists 目标列表 `[20]`（build_draw_lists 分派，d_gap §1.1）。
- 调试用途：线框叠加由 `WireframePass` 组件门控（非每帧）。

## 关键事实

- 调试线框（draw_lists 图层 20）。
