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
