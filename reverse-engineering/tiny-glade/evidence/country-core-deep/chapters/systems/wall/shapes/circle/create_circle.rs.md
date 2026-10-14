# systems/wall/shapes/circle/create_circle.rs

> 文件作用：圆形墙创建：CreateCircleCmd 到 Circle2d::as_curve2 到 crashy_new_from_points、enclosure=circle、style/elevation、RoofShapeParams::from_wall_shape + roof_height_01_from_wall_shape 到 CreateRoofCmd、历史。

函数总计 3：业务 1 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::circle::create_circle::create_circle` | 0x14097e560 | 38 | create_circle（曲线+enclosure+roof+历史） | O | wall\decomp\decomp-0x14097e560.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 2 项
