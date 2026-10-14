# systems/render_loop/draw_passes/wireframe_pass.rs

> 文件作用：线框调试 pass：`wireframe_pass`（世界图线框叠加）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::draw_passes::wireframe_pass::wireframe_pass` | 0x1409ccb20 | 17 | 线框 pass：RasterPass::new + CommonDrawParams::new + set_polygon_mode/set_depth_test_enable（wp5 draw-passes §10）；图层 WIREFRAME=20 | O | render-loop\decomp\decomp-0x1409ccb20.txt；wp5 draw-passes §10 |

## 关键事实

- 调试线框（draw_lists 图层 20）。
