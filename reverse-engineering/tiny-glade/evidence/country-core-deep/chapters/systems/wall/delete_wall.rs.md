# systems/wall/delete_wall.rs

> 文件作用：墙删除系统：消费 OnWallDeleted（16B 行），从 PublicWalls.wall_entities_map(+0x30)/WallEntities/WallPlasters 三 map remove_entry，Commands::despawn 墙实体与逐石膏实体（两个类型 id）。

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::delete_wall::delete_wall` | 0x140aac1e0 | 7 | delete_wall（三 map remove + Commands::despawn 双类型） | O | wall\decomp\decomp-0x140aac1e0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项

## 实现方式（算法级）（O，decomp-0x140aac1e0）

**处理链**：5 参 `(EventReader<OnWallDeleted>, ResMut<PublicWalls>, ResMut<WallPlasters>,
ResMut<WallEntities>, Commands)`；事件行 16B `{id, WallId}`；双缓冲 drain 逐事件：

1. **PublicWalls**：find 经 `*param_2+0x50` 区（哈希种子/控制区），`remove_entry` 于 **`*param_2+0x30`**
   （FUN_140b76f90：map/哈希态指针 + hint + key）。
2. **WallEntities**（param_4）：find@`*param_4+0x20`，remove（FUN_140b76f90）；返回值 = 墙实体
   `Option<Entity>`，**高 32 位（generation）== 0 → 跳过 despawn**（None=0 编码，I）。
3. **墙实体 despawn**：`Commands::despawn` 类型 id = **UNK_142add9c8**（FUN_140d14770；
   Commands 队列字段 = param_5[0..3]）。
4. **WallPlasters**（param_3）：find@`*param_3+0x20`，remove 用**另一 erase 助手 FUN_140b777f0**；
   返回石膏 `Vec<Option<Entity>>`（8B 条目，cap-first `{cap@0, ptr@8, len@0x10}`）。
5. **石膏逐实体 despawn 循环**：迭代 Vec，`while(entry>>0x20==0)` 跳过空位，命中 `>>0x20!=0` →
   `Commands::despawn` 类型 id = **UNK_142add9e0**；循环结束 `free(ptr, cap*8, 8)`（func_0x000140613c20）。

三个 ResMut tick 回写内联（param_2/3/4 `param[n][2]=*(param+n+0x1c)`）。无专用 panic 串。
