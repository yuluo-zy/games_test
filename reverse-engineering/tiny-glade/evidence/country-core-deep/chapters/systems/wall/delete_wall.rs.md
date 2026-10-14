# systems/wall/delete_wall.rs

> 文件作用：墙删除系统：消费 OnWallDeleted（16B 行），从 PublicWalls.wall_entities_map(+0x30)/WallEntities/WallPlasters 三 map remove_entry，Commands::despawn 墙实体与逐石膏实体（两个类型 id）。

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::delete_wall::delete_wall` | 0x140aac1e0 | 7 | delete_wall（三 map remove + Commands::despawn 双类型） | O | wall\decomp\decomp-0x140aac1e0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项
