# systems/terrain/deferred_terrain_history.rs

> 文件作用：**延迟地形历史：PendingTerrainHistory 脏标记（0x8000000000000002 哨兵）→ 首次变化构建历史记录 History::add + DebugHistoryMsg。**

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::terrain::deferred_terrain_history::history_record_terrain` | 0x140ac9770 | 16 | PendingTerrainHistory 置脏标记；首次变化构建历史记录并 History::add，发 DebugHistoryMsg；之后不重复记录 | O | terrain-water\decomp\decomp-0x140ac9770.txt

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 1 项

## 实现方式（算法级）

`history_record_terrain`（0xac9770）：PendingTerrainHistory 脏标记（0x8000000000000002 哨兵）→ 首次变化构建历史记录 History::add + DebugHistoryMsg；之后不重复记录。
