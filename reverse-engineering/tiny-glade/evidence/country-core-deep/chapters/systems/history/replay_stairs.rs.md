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
## 实现方式（算法级，2026-10-15 深挖；骨架见 history/ALGORITHMS.md §2.6）

- **replay_stairs 0x140af75b0**（前桶迭代骨架 + 下列）：
  1. 命令 {tag 10, flags byte@+0x08（bit0=instant）, duration f32@+0x0c, payload@+0x10,
     elapsed f32@+0x18, undo@+0x1c}；payload+0x10 = 边记录 {edge1 u32, edge2 u32,
     标志字节}；
  2. **动画门控**：`elapsed < max(duration/speed, 下限 _UNK_142930d70)` → 仅
     `elapsed += dt` 等下一帧；满足后（或 instant）→ 处理；
  3. **处理**：`undo==0 → hashbrown insert 边入 StairsRemovedSupports`（FUN_1405b1be0
     哈希 + FUN_140b76ce0，表基 +0x20）；`!=0 → FUN_1402c8640 remove_entry(edge1,
     edge2)`；查 DecoratorStorage::lookup_decorator_info（FUN_140a1c930；type 字节
     +0x41 | 0xf8 > 0xf9 → 跳过）+ CachedDecoratorTransforms（FUN_1405b0ed0 SIMD 探测，
     键 = edge1）→ **push PlayStairsAudioCmd**（Events<PlayStairsAudioCmd>，
     {edge id, decorator info u64 对}）；
  4. 收尾同通用骨架（逆序移除 + tag 10 → default drop）。

