# systems/wall/shapes/circle/ui_create_circle.rs

> 文件作用：圆形绘制 UI：CameraCursorRay::intersect_with_xz_plane 拖拽半径到 PreviewCircle；circle_signifier（custom shader）。

函数总计 6：业务 2 / 内联无独立符号 1 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::circle::ui_create_circle::ui_create_circle` | 0x14165bed0 | 0 | ui_create_circle（射线拖拽半径 → PreviewCircle） | O | ui-input\decomp\decomp-0x14165bed0.txt, wall\decomp\decomp-0x14165bed0.txt |
| `country_core::systems::wall::shapes::circle::ui_create_circle::circle_signifier` | 0x14165cd40 | 288 | circle_signifier（构建 cursor_circle_ 预览 signifier：FUN_14165bd40 圆圈预览 + signifier 流） | O |  |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::shapes::circle::ui_create_circle::PreviewCircle::new`（行 50）— 构造

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 2 项
