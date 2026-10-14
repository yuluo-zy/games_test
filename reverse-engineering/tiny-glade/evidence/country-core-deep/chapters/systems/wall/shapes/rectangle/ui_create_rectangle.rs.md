# systems/wall/shapes/rectangle/ui_create_rectangle.rs

> 文件作用：矩形绘制 UI：rectangle_dims_from_cursor_pos + get_rectangle_orientation 到 PreviewRectangle。

函数总计 10：业务 2 / 内联无独立符号 2 / 派生胶水 5

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::rectangle::ui_create_rectangle::ui_create_rectangle` | 0x14155d410 | 0 | ui_create_rectangle（dims_from_cursor_pos + orientation） | O | ui-input\decomp\decomp-0x14155d410.txt, wall\decomp\decomp-0x14155d410.txt |
| `country_core::systems::wall::shapes::rectangle::ui_create_rectangle::rectangle_signifier` | 0x14155e460 | 347 | rectangle_signifier（构建 cursor_rectangle_ 预览 signifier：FUN_141559ce0 矩形预览 + signifier 流） | O |  |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::shapes::rectangle::ui_create_rectangle::PreviewRectangle::new`（行 62）— 构造
- `country_core::systems::wall::shapes::rectangle::ui_create_rectangle::rectangle_dims_from_cursor_pos`（行 332）— UI 交互

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 5 项
