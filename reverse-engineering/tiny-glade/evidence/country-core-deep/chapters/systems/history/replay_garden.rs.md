# systems/history/replay_garden.rs

> 文件作用：花园光栅回放：replay_raster<GardenRasterStagingSsbo, WorldRaster<512>, GardenFilter> 的薄包装 + 命令生成闭包（generate_replay_cmds_from_edit 宿主 0x14143d2c0）。

函数总计 5：业务 1 / 内联无独立符号 4 / 派生胶水 0（已并入业务表）

## 业务函数

| 函数 | 地址 | 源码行 | 作用（有依据推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::history::replay_garden::replay_garden::closure$1::closure$0` | 0x141447500 | 34 | replay_raster<Garden> 薄包装 | I | history\decomp\decomp-0x141447500.txt |

## 内联/无独立符号函数（名称揭示意图；逻辑并入调用点）

- `country_core::systems::history::replay_garden::replay_garden`（行 17）— replay_raster<Garden> 薄包装（I；单态 0x1409f6ff0）
- `country_core::systems::history::replay_garden::replay_garden::closure$0`（行 25）— replay_raster<Garden> 薄包装（I；单态 0x1409f6ff0）
- `country_core::systems::history::replay_garden::replay_garden::closure$1`（行 32）— replay_raster<Garden> 薄包装（I；单态 0x1409f6ff0）
- `country_core::systems::history::replay_garden::replay_garden::closure$2`（行 39）— replay_raster<Garden> 薄包装（I；单态 0x1409f6ff0）

<!-- 未覆盖函数数：0 -->
