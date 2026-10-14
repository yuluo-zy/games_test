# systems/wall/finish_wall.rs

> 文件作用：墙收尾系统：消费 FinishWall，取进行中墙（无则 panic 'wall_manager has no temp curve'），mark_as_non_enclosed + update_wall_state_enclosure，CompoundEditBuilder 记历史（Create(seed) 到 StateChange）；5 ResMut（含 History）。

函数总计 3：业务 1 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::finish_wall::finish_wall` | 0x140a800a0 | 0 | finish_wall（5 ResMut；no temp curve/doesn't exist panic；CompoundEditBuilder 历史） | O | wall\decomp\decomp-0x140a800a0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 2 项
