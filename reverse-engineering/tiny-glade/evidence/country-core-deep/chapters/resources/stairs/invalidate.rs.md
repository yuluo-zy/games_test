# resources/stairs/invalidate.rs

> 文件作用：**脏楼梯伪影失效系统：脏边→撤碰撞体/RT 网格/视觉/装配缓存**

函数总计 3：业务 1 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts` | 0x140b37180 | 0 | 系统:失效 | O：invalidate_dirty_stair_artifacts（3455B）——脏边 drain；每边 remove_collider<StairGraphEdge>×3+Arc drop+RtMesh 删+StairsVisualState::remove_edge；contains_edge_or_stub 决定撤视觉 | collision-eco\decomp\decomp-0x140b37180.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 2 项
