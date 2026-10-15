# systems/history/replay_terrain.rs

> 文件作用：地形回放系统：前桶判别 3 命令；TerrainReplayCmd::forward/backward 构造 duration=clamp(Σ点距*0.01,0.4,3.0)（无点 0.3，stroke_length 有限断言）；undo(+0x1c)==0→reapply，!=0→revert；动画 ease_out。

函数总计 5：业务 3 / 内联无独立符号 2 / 派生胶水 0（已并入业务表）

## 业务函数

| 函数 | 地址 | 源码行 | 作用（有依据推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::history::replay_terrain::replay_terrain` | 0x140995150 | 0 | 见文件作用 | O | decomp-0x140995150.txt |
| `country_core::systems::history::replay_terrain::TerrainReplayCmd::forward` | 0x140994f30 | 30 | 前向命令：duration=clamp(Σ点距*0.01,0.4,3.0)（空 0.3）+ stroke_length 有限断言 | O | decomp-0x140994f30.txt；DIGEST §2.4 |
| `country_core::systems::history::replay_terrain::TerrainReplayCmd::backward` | 0x140995040 | 40 | 反向命令（同构） | O | decomp-0x140995040.txt |

## 内联/无独立符号函数（名称揭示意图；逻辑并入调用点）

- `country_core::systems::history::replay_terrain::TerrainReplayCmd::new`（行 50）— 命令构造（I；内联）
- `country_core::systems::history::replay_terrain::ease_out`（行 168）— 缓出函数（系数歧义 U）（U；内联；reconstruction 用 1-(1-t)^3）

<!-- 未覆盖函数数：0 -->

## 实现方式（算法级，2026-10-15 深挖；骨架见 history/ALGORITHMS.md §2.6）

- **replay_terrain 0x140995150**（前桶迭代骨架 + 下列）：
  1. `active(+0x18)==0 → return`；speed = ReplayBuffer+0x28；dt = Time+0x28；
  2. 前桶 cmds（ptr@+0x08, len@+0x10）0x30 步长迭代，仅 tag 3；载荷 = TerrainEdit
     （niche @+0x10 == 0x8000000000000000 时变体字段在 +0x18）；
  3. **instant 路径**（`duration(+0x10)==0.0 且非 NaN 且 undo(+0x1c)==0`）：拷贝点集
     （12B/点，断言点数 < 0x3fffff @0x142aae990）→ 更新地形 stroke 计数（+0x38 =
     max(索引+1, 原值)）→ FUN_141526d10 直接插入 TerrainEditingData（+0x8 起点）→
     释放拷贝；构造默认命令 {ptr, 0.4f, 2, stroke_id}（0x3ecccccd = 0.4f）；
  4. **动画路径**：`t = clamp(elapsed/duration, 0, 1)`，`eased = 1-(1-t)^3`
     （ease_out_cubic；0.4 为 clamp 下限 @0x1429cd060、1.0 @0x142925904）；
     `undo==0 → reapply 0x1408592d0(payload+0x10, …, eased)`，`!=0 → revert
     0x140858d30`；`eased >= 1.0` 时 +0x18 计数--，<0 → 完成入列表；
  5. `elapsed += dt*speed`（+0x10）；
  6. 收尾同通用骨架（完成索引逆序移除：边界断言 → memmove 压缩 → 按判别字节 drop
     分派，tag 3 → FUN_140b5c2a0）。
- **TerrainReplayCmd::forward 0x140994f30 / backward 0x140995040**（同构 1533B）：
  duration = clamp(Σ点距*0.01, 0.4, 3.0)（无点 0.3）；"assertion failed:
  stroke_length.is_finite()"（terrain_edit.rs:43，@0x142a94c34）；输出 {data ptr,
  +0x8=0, duration f32@+0xc, +0x10=2}。
