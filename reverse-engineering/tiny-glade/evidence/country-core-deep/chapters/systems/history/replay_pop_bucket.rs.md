# systems/history/replay_pop_bucket.rs

> 文件作用：空桶弹出系统：前桶 cmds 空时 ReplayBuffer::pop_front，应用桶的装饰 diff（restore_decorators，FUN_14155ba30）与楼梯 diff（restore_backup/apply_diff 链 1408ff210→140900ba0→14157cec0→140b50cc0→140b50d60→1408fefe0），Arc drop 收尾并 flush colliders/archivist。

函数总计 2：业务 1 / 内联无独立符号 1 / 派生胶水 0（已并入业务表）

## 业务函数

| 函数 | 地址 | 源码行 | 作用（有依据推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::history::replay_pop_bucket::replay_pop_bucket` | 0x140b38250 | 15 | 见文件作用 | O | decomp-0x140b38250.txt |

## 内联/无独立符号函数（名称揭示意图；逻辑并入调用点）

- `country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure$0`（行 43）— 见文件作用（O；decomp-0x140b38250.txt）

<!-- 未覆盖函数数：0 -->
## 实现方式（算法级，2026-10-15 深挖）

- **replay_pop_bucket 0x140b38250**（109 行全文）：
  1. `active(+0x18) != 0` 且**前桶 cmds len(+0x10) == 0** → `FUN_1409f18f0` pop_front
     取 (decorators Arc, stairs Arc)；
  2. decorators Arc 非空 → `FUN_14155ba30(storage, arc+2, …)` = 装饰 diff 恢复
     （DecoratorStorage::restore_decorators）→ LOCK dec（归零 FUN_140b5cb60 drop）；
  3. stairs Arc 非空 → 楼梯应用链：FUN_1408ff210（restore_backup）→ FUN_140900ba0
     （apply_diff）→ FUN_14157cec0(+0xb8, +0x28, +0x148) → FUN_140b50cc0(+0xb8) →
     FUN_140b50d60(+0x100) → `(+0x178)=0` → FUN_1408fefe0（
     StairsState::restore_backup/apply_diff/iter_edges_and_stubs 链）→ LOCK dec
     （归零 FUN_140b5c420 drop）；
  4. 收尾（两分支共用）：`FUN_140b2cb30(…, 1)` + `FUN_140b743e0(…, 0x70, 0x10)` +
     `FUN_1408cd5c0(*param_7)`（RaycastWorld flush / archivist extract）。

