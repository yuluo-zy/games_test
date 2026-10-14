# systems/wall/wall_state/wall_roof_hashmap.rs

> 文件作用：墙到屋顶映射维护：Query<(Entity,&Roof,Option<...>)> 遍历，维护 PublicWalls.roof_entity 与 WallEntities.wall_entity；预置形状无 roof 到清除。

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::wall_state::wall_roof_hashmap::wall_roof_hashmap` | 0x140a94cd0 | 40 | wall_roof_hashmap（Query 遍历 + 双向映射维护） | O | wall\decomp\decomp-0x140a94cd0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项
