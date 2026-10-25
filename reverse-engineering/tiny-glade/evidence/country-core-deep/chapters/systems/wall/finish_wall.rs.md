# systems/wall/finish_wall.rs

> 文件作用：墙收尾系统：消费 FinishWall，取进行中墙（无则 panic 'wall_manager has no temp curve'），mark_as_non_enclosed + update_wall_state_enclosure，CompoundEditBuilder 记历史（Create(seed) 到 StateChange）；5 ResMut（含 History）。

函数总计 3：业务 1 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::finish_wall::finish_wall` | 0x140a800a0 | 0 | finish_wall（5 ResMut；no temp curve/doesn't exist panic；CompoundEditBuilder 历史） | O | wall\decomp\decomp-0x140a800a0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 2 项

## 实现方式（算法级）（O，decomp-0x140a800a0，59KB 结构级）

**处理链**：13 参；**6 个 +0x1c tick 回写槽**：param_3（plStack_78=ResMut<InnerWalls>）、param_4、param_6、
param_9、param_10、param_11（**DIGEST 列 5 漏 param_3 —— 纠错**）。事件 16B 行（u8 标志@+8）；
墙 id = InProgressWall+0x80。

1. **InnerWalls 查表**（0x78 步长）。命中：param_6 回写 + 读记录 ~0x78B 入栈 + **`*param_6 =
   0x8000000000000000`**（None，清 temp/历史游标）；复制墙 2D 点 Vec `{cap@value+8, ptr@value+0x10,
   len@value+0x18}`（8B 点）。
2. **历史记录**：`FUN_1408dc9f0`（history_record_wall_changed）→ 0xf8B 结构；**CompoundEditBuilder**
   （param_10）：`*param_10+0x120` 两段 **0xf8B 拷贝**（旧→新）。失败 → panic
   `assertion failed: pending_wall_history.wall_changed.replace(expand).is_none()`（0x142ad74bb，
   行 0x4d=77）—— **揭示 builder 字段 `pending_wall_history.wall_changed`**。
3. **Builder 族 ×3**：0x140a82b40（@744/1202/1305）、0x141696f50（@758/1215）、0x140a84ec0（@766/1223/1324）。
4. **InnerWalls 方法**：FUN_14085a560（get by id → 形状 tag@+0x38 变体分派）、FUN_14085aa60 ×2（采样）。
5. **围合失败 panic**：墙不存在 → fmt panic `Finish wall marked a wall as non-enclosed, but it doesn't
   exist`（0x142ad75d0；模块串与消息串相邻存储，FUN_1428d9390 带 WallId display）。
6. **freehand_ty@InProgressWall+0x9c** 选择**稠密 f32 数组**（param_12/13，以 wall id 为下标）；
   越界 → FUN_1428d9518。`*plStack_88 = -0x8000000000000000`（None 哨兵写回）。

panic 串：`Was asked to finish a wall, but wall_manager has no temp curve`（0x142ad75a8，panic_str）。
