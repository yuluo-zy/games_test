# systems/ui/signifiers/signifier_mark_for_delete.rs

> 文件作用：指示符删除：signfier_mark_for_delete（PDB 拼写如此）按 signifier_config_hash + UiState 标记删除；despawn_marked 销毁 SignifierMarkedForDeletion 实体。

函数总计 3：业务 2 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ui::signifiers::signifier_mark_for_delete::signfier_mark_for_delete` | 0x140929160 | 11 | 标记删除 | O（wp15 b7）：2 callers | wp15/analysis/b7_signifiers/signfier_mark_for_delete_0x140929160.c |
| `country_core::systems::ui::signifiers::signifier_mark_for_delete::despawn_marked` | 0x140929a90 | 63 | 销毁已标记实体 | O（wp15 b7）：3 callers | wp15/analysis/b7_signifiers/despawn_marked_0x140929a90.c |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项
