# systems/history/finish_replay.rs

> 文件作用：回放自然完成/非活跃回放的收尾系统：检测 ReplayBuffer 与待处理命令双空后把 RestoreAppTo 记录的 AppMode 写回、解锁游戏指挥官（MetaCommander 哨兵弹栈）、恢复工具栏模式，并向绵羊系统发 CmdUnfreezeSheep。日志“History replay has naturally finished…”/“History replay was not active…”。

函数总计 2：业务 1 / 内联无独立符号 1 / 派生胶水 0（已并入业务表）

## 业务函数

| 函数 | 地址 | 源码行 | 作用（有依据推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::history::finish_replay::finish_replay` | 0x1408b36a0 | 0 | 见文件作用 | O | decomp-0x1408b36a0.txt |

## 内联/无独立符号函数（名称揭示意图；逻辑并入调用点）

- `country_core::systems::history::finish_replay::finish_replay::closure$1`（行 52）— 见文件作用（O；decomp-0x1408b36a0.txt）

<!-- 未覆盖函数数：0 -->
