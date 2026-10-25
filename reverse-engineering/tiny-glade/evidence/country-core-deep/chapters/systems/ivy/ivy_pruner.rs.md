# systems/ivy/ivy_pruner.rs

> 文件作用：**常春藤修剪：墙变更后点有效性校验+按位置移除**

函数总计 4：业务 4 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ivy::ivy_pruner::ivy_pruner` | 0x140997410 | 20 | 系统:修剪 | O：ivy_pruner（70KB）——墙空间哈希校验（"Spatial Hash returned a wall id "）+逐点有效性 | collision-eco\decomp\decomp-0x140997410.txt |
| `country_core::systems::ivy::ivy_pruner::is_point_still_valid` | 0x140998dc0 | 133 | 查询/计算 | O：is_point_still_valid——点仍有效检查（含 closest_segment_index 调用） | collision-eco\decomp\decomp-0x140998dc0.txt |
| `country_core::systems::ivy::ivy_pruner::ivy_pruner_cmd` | 0x140999330 | 213 | 系统:命令 | O：ivy_pruner_cmd——RemoveIvyCmd 消费 | collision-eco\decomp\decomp-0x140999330.txt |
| `country_core::systems::ivy::ivy_pruner::remove_ivy_at_location` | 0x140999860 | 264 | 系统:命令 | O：remove_ivy_at_location——按位置移除 | collision-eco\decomp\decomp-0x140999860.txt |

## 实现方式（算法级）

- ivy_pruner（0x140997410，70KB）：墙空间哈希校验（"Spatial Hash returned a wall id " 0x142abb028）→ 逐点有效性（callee 证实：`is_point_still_valid` 0x140998dc0 ×2 + 共享 ivy helper FUN_140992af0 ×2）。
- is_point_still_valid（0x140998dc0）：内含 closest_segment_index 复检 —— 墙段变更后点是否仍在有效段/高度带内。
- ivy_pruner_cmd（0x140999330）：消费 RemoveIvyCmd 事件 → remove_ivy_at_location（0x140999860）按世界位置移除。
