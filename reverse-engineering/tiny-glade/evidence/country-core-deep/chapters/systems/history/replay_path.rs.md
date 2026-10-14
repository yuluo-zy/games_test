# systems/history/replay_path.rs

> 文件作用：路径/鹅卵石光栅回放：replay_raster 的 4 个领域实例（DirtPath/Cobblestone1-3）+ generate_path_replay_cmds（0x1416286d0，niche 判别 0..3 过滤后尾调共享核心 0x1414433xx）。

函数总计 5：业务 2 / 内联无独立符号 3 / 派生胶水 0（已并入业务表）

## 业务函数

| 函数 | 地址 | 源码行 | 作用（有依据推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::history::replay_path::generate_path_replay_cmds` | 0x1416286d0 | 44 | Path/Cobblestone 编辑 → ReplayCmd：niche 判别 0..3 过滤后尾调 0x1414433xx 共享核心 | O | history\decomp\decomp-0x1416286d0.txt |
| `country_core::systems::history::replay_path::generate_path_replay_cmds::closure$0` | 0x1414475a0 | 50 | Path/Cobblestone 编辑 → ReplayCmd：niche 判别 0..3 过滤后尾调 0x1414433xx 共享核心 | O | history\decomp\decomp-0x1414475a0.txt |

## 内联/无独立符号函数（名称揭示意图；逻辑并入调用点）

- `country_core::systems::history::replay_path::replay_path`（行 18）— replay_raster<Path> 领域包装（I；单态 0x1409febf0）
- `country_core::systems::history::replay_path::replay_path::closure$0`（行 30）— replay_raster<Path> 领域包装（I；单态 0x1409febf0）
- `country_core::systems::history::replay_path::replay_path::closure$1`（行 35）— replay_raster<Path> 领域包装（I；单态 0x1409febf0）

<!-- 未覆盖函数数：0 -->
