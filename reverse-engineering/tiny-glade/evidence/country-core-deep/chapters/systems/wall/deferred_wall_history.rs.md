# systems/wall/deferred_wall_history.rs

> 文件作用：延迟墙历史记录：PendingWallChanged::try_new 把前一墙首/末端点投影到下一曲线（approximate_curve_u：分母下限 1e-8、严格更小、并列取首段、clamp01、严格 low<high，否则 i64::MIN 哨兵）到 12B StateChange range；history_record_wall_changed/merge 记 ConcurrentEdits 组；CompoundEditBuilder::wall_deleted 记 Delete 组。

函数总计 5：业务 4 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::deferred_wall_history::PendingWallChanged::try_new` | 0x1408dc9f0 | 48 | PendingWallChanged::try_new（投影/clamp01/严格 low<high/i64::MIN 哨兵） | O | wall\decomp\decomp-0x1408dc9f0.txt |
| `country_core::systems::wall::deferred_wall_history::history_record_wall_changed` | 0x1408dccf0 | 96 | history_record_wall_changed（pending → ConcurrentEdits 组） | O | wall\decomp\decomp-0x1408dccf0.txt |
| `country_core::systems::wall::deferred_wall_history::history_record_merge_walls` | 0x1408dd1b0 | 118 | history_record_merge_walls | O | wall\decomp\decomp-0x1408dd1b0.txt |
| `country_core::resources::history::wall_compound_edit::CompoundEditBuilder::wall_deleted` | 0x140a876a0 | 165 | CompoundEditBuilder::wall_deleted（Delete 组 3 分支） | O | wall\decomp\decomp-0x140a876a0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 1 项
