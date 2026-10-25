# systems/history/replay_auto_clutter.rs

> 文件作用：自动杂物回放系统：消费前桶判别 9 命令，undo==0 插墙集（HashMap<WallId,()>），!=0 从 DeletedAutoClutter erase (DecoratorId,DecoratorId)；instant/动画节奏。

函数总计 2：业务 1 / 内联无独立符号 1 / 派生胶水 0（已并入业务表）

## 业务函数

| 函数 | 地址 | 源码行 | 作用（有依据推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::history::replay_auto_clutter::replay_auto_clutter` | 0x140aa4a60 | 0 | 见文件作用 | O | decomp-0x140aa4a60.txt |

## 内联/无独立符号函数（名称揭示意图；逻辑并入调用点）

- `country_core::systems::history::replay_auto_clutter::replay_auto_clutter::closure$0`（行 38）— 见文件作用（O；decomp-0x140aa4a60.txt）

<!-- 未覆盖函数数：0 -->
## 实现方式（算法级，2026-10-15 深挖；骨架见 history/ALGORITHMS.md §2.6）

- **replay_auto_clutter 0x140aa4a60**：前桶迭代骨架（speed = ReplayBuffer+0x28）；
  每命令：`undo(+0x1c)==0 → HashMap<WallId,()> insert`（墙派生装饰恢复）；
  `!=0 → DeletedAutoClutter.erase((DecoratorId, DecoratorId))`；instant/动画节奏
  （elapsed += dt*speed，完成后索引入列表）；收尾逆序移除（tag 9 → 0x1402e5d80
  drop）。

