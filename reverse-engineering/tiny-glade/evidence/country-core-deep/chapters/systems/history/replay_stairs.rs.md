# systems/history/replay_stairs.rs

> 文件作用：楼梯回放系统：前桶判别 10 命令，undo==0 插边集（StairsRemovedSupports），!=0 从边集 remove_entry；查附挂装饰（DecoratorStorage::lookup_decorator_info + CachedDecoratorTransforms）并发 PlayStairsAudioCmd。

函数总计 3：业务 1 / 内联无独立符号 2 / 派生胶水 0（已并入业务表）

## 业务函数

| 函数 | 地址 | 源码行 | 作用（有依据推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::history::replay_stairs::replay_stairs` | 0x140af75b0 | 0 | 见文件作用 | O | decomp-0x140af75b0.txt |

## 内联/无独立符号函数（名称揭示意图；逻辑并入调用点）

- `country_core::systems::history::replay_stairs::replay_stairs::closure$0`（行 58）— 见文件作用（O；decomp-0x140af75b0.txt）
- `country_core::systems::history::replay_stairs::replay_stairs::closure$3`（行 88）— 见文件作用（O；decomp-0x140af75b0.txt）

<!-- 未覆盖函数数：0 -->
