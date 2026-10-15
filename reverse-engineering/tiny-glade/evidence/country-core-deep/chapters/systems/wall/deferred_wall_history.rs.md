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

## 实现方式（算法级）

### PendingWallChanged::try_new 0x1408dc9f0（O）
- 入参：`(out, WallId?, 前一墙状态 param_3, 后一墙状态 param_4, 方向 char)`；前后墙均含曲线（`points Vec2@+8、count@+0x10、u 数组@+0x20`）。
- 断言：`!before.curve.is_empty()`（line 42，UNK_142aa4bd8/142aa4c08）、`!after.curve.is_empty()`（line 41，UNK_142aa4b90/142aa4bc0）。
- **投影**：方向==0 → 取前墙**末点**（`puVar2+len*2` 的 {x,y}），否则取**首点**（`*puVar2, puVar2[1]`）→ `FUN_141469a20(后墙曲线, x, y)`（approximate_curve_u：线段查找+lerp，NaN/inf panic）→ **clamp01**。
- **严格 low<high**：比较两 u（`bVar7 = cmp`），不满足 → `*out = 0x8000000000000000`（**i64::MIN 哨兵=None**）+ 释放两墙曲线数组。
- 有效 → `func_0x000142923c00(out, 前墙, 0xe0)`（拷贝前墙状态）+ `out+0xe0 = WallId`、`+0xe8 = 0`、`+0xec = low u f32`、`+0xf0 = high u f32`（PendingWallChanged = 0xf8B）。

### history_record_wall_changed 0x1408dccf0（O）
- ResMut<ConcurrentEdits>（param_2，+0x1c tick）：`pending = self+0x120`（取 PendingWallChanged），`self+0x120 = i64::MIN`（置 None）；`pending < 0` → 早退（无挂起）。
- 快照 ConcurrentEdits 当前状态（self+0x128..0x210 多处 u64/f32）→ 分配 0x108B 编辑项（func_0x000140613c10(0x108,8)，Arc {1,ptr,1} 包装）→ 状态 0xe0 拷贝 → `FUN_141696f50`（**build**）→ `FUN_140a84ec0(editor, built, &UNK_142aa4c20)`（提交）。无挂起/提交为空 → 早退。

### history_record_merge_walls 0x1408dd1b0（O）
- ResMut<ConcurrentEdits>：`self 状态 tag=2`（置 2），`原本==2` → 早退（merge 幂等）。`原本==1` → 先 `FUN_140a83fb0+FUN_140a84ec0` 记前墙 changed（UNK_142aa4c38）。
- 对**两条被合并墙**各做：hash InnerWalls（param_3，0x78 步长）→ `FUN_140a876a0(&builder, wall_id1, &状态, ty1)`（**wall_deleted**）→ 拷贝墙状态（points 8B 元素、u 4B 元素、shape tag）构建编辑项 `variant 0`（0x8000000000000000）→ `FUN_140a82b40`（builder add）→ `FUN_141696f50`（build）+ `FUN_140a84ec0(editor, &built, &UNK_142aa4c68)`（提交）。
- 未命中 → `no entry found for key`（UNK_142aa4c50 域，deferred_wall_history.rs）。

### CompoundEditBuilder::wall_deleted 0x140a876a0（O）
- 入参：`(builder, WallId?, 墙状态 param_3{points@+8/count@+0x10、u 数组@+0x20/count@+0x28、shape tag@+0x38、+0x54 u32、+0x58..+0x68}, ty)`。
- 拷贝两条曲线数组（`count1×8`、`count2×4`，悬空哨兵 {4,0} 处理）→ **tag 3 分支**：`tag-2` clamp ≤2 → 0→variant 2、1→variant 3、else→读 +0x3c/+0x44/+0x4c 原始 tag（预置 vs freehand 形状编码）→ 组装 `uStack_1a0 = 0x8000000000000001`（**enum variant 1 = Delete**）→ `FUN_140a82b40(builder, &item)` 入 Delete 组。
- **纠错**：wall_deleted 返回 void 且经 builder.add 入组；"3 分支"= 形状 tag 的变体选择（0/1/else），非删除组的三种命令。
