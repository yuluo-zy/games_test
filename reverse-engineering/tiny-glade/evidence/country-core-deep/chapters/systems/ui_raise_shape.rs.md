# systems/ui_raise_shape.rs

> 文件作用：抬升形状：widget 名 'outline_wall_bottom_'/'arrow_raise_shape_'/'raise_wall_elevation_'；impl_raise_shape 用 ProportionalScreenSpaceCoord::project_onto_ray 算抬升量 + History::add 记录。

函数总计 8：业务 2 / 内联无独立符号 0 / 派生胶水 6

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ui_raise_shape::ui_raise_shape` | 0x140952e70 | 37 | 系统:周期性处理（ui_raise_shape） | O：8.7KB 主系统（wp20 + 本域复核） | ui-input\decomp\decomp-0x140952e70.txt |
| `country_core::systems::ui_raise_shape::impl_raise_shape` | 0x140955650 | 238 | 抬升实现 | O：impl_raise_shape（project_onto_ray + 高度 delta + History::add；wp20 既有） | wp20/analysis/0x140955650_impl_raise_shape.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 6 项
