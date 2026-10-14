# systems/wall/shapes/rectangle/create_rectangle.rs

> 文件作用：矩形墙创建：CreateRectangleCmd 到 Rectangle2d::as_curve2 + smoothstep 角 + ridge_length_01_from_rectangle_ratio + roof_height_01_from_wall_shape 到 CreateRoofCmd、历史。

函数总计 6：业务 2 / 内联无独立符号 0 / 派生胶水 4

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::rectangle::create_rectangle::create_rectangle` | 0x140a93790 | 26 | create_rectangle（smoothstep 角 + roof + 历史） | O | wall\decomp\decomp-0x140a93790.txt |
| `country_core::systems::wall::shapes::rectangle::create_rectangle::get_building_and_roof_colors` | 0x140a94880 | 173 | 访问器/转换 | I | wall\decomp\decomp-0x140a94880.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 4 项
