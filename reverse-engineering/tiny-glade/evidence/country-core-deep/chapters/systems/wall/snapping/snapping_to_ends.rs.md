# systems/wall/snapping/snapping_to_ends.rs

> 文件作用：端吸附系统：非 enclosure 公共墙；InProgressCurve::first_raw_input_point 投影墙曲线（get_pos_at_coord/get_pos_at_u/start_pos）到 find_closest_end_to_snap_to（u=0/1）到 mode=2。

函数总计 2：业务 1 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::snapping::snapping_to_ends::snapping_to_ends` | 0x140a6ad80 | 10 | snapping_to_ends（首输入点投影墙曲线→最近端→mode=2） | O | wall\decomp\decomp-0x140a6ad80.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::snapping::snapping_to_ends::find_closest_end_to_snap_to`（行 36）— 查询/计算
