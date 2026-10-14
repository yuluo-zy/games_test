# systems/wall/shapes/shape_placement.rs

> 文件作用：形状放置参数：光标地形位到 ShapePlacement::new_unsnapped；closest_segment_to_snap_to（WallSpatialHash 查询 + distance_segment_point）；snap_to_roof_ridge（Roof::pivot_3d/RoofRidgeDims::calc）。

函数总计 12：业务 2 / 内联无独立符号 2 / 派生胶水 8

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge` | 0x140980db0 | 0 | snap_to_roof_ridge（Roof::pivot_3d/RoofRidgeDims::calc） | O | wall\decomp\decomp-0x140980db0.txt |
| `country_core::systems::wall::shapes::shape_placement::shape_placement_params` | 0x14097f970 | 86 | shape_placement_params（光标位→new_unsnapped + 吸附） | O | wall\decomp\decomp-0x14097f970.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::shapes::shape_placement::ShapePlacement::new_unsnapped`（行 70）— 构造
- `country_core::systems::wall::shapes::shape_placement::closest_segment_to_snap_to`（行 456）— 查询/计算

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 8 项
