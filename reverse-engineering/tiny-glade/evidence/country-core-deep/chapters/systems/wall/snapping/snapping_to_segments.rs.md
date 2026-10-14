# systems/wall/snapping/snapping_to_segments.rs

> 文件作用：段吸附系统：raw_input_curve_length_ws；closest_segment_to_snap_to（get_curve_segment_from_u/get_tangent_at_u + distance_segment_point）最近段到 mode=1。

函数总计 4：业务 1 / 内联无独立符号 1 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::snapping::snapping_to_segments::snapping_to_segments` | 0x140a6b5e0 | 21 | snapping_to_segments（raw 长度 + closest_segment_to_snap_to → mode=1） | O | wall\decomp\decomp-0x140a6b5e0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to`（行 53）— 查询/计算

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 2 项
