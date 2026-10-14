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
