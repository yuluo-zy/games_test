# systems/wall/shapes/rectangle/ui_edit_rectangle_dims.rs

> 文件作用：矩形尺寸编辑 UI：get_distance_to_edge 拖边到新 dims 到 EditRectangleCmd；decorator_cull_allow_any。

函数总计 5：业务 2 / 内联无独立符号 1 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::do_ui_edit_rectangle_dims` | 0x140ad6e00 | 0 | ui_edit_rectangle_dims（变体） | O | ui-input\decomp\decomp-0x140ad6e00.txt, wall\decomp\decomp-0x140ad6e00.txt |
| `country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::ui_edit_rectangle_dims` | 0x140ad6250 | 43 | ui_edit_rectangle_dims（get_distance_to_edge 拖边） | O | wall\decomp\decomp-0x140ad6250.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_dims::get_distance_to_edge`（行 301）— 查询/计算

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`closure` 等 2 项
