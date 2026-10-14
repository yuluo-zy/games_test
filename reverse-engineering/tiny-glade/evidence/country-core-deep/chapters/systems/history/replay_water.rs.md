# systems/history/replay_water.rs

> 文件作用：水面光栅回放：replay_raster<WaterRasterStagingSsbo, WorldRaster<512>, WaterFilter> 薄包装 + 命令生成闭包（宿主 0x141444c70）。

函数总计 5：业务 2 / 内联无独立符号 3 / 派生胶水 0（已并入业务表）

## 业务函数

| 函数 | 地址 | 源码行 | 作用（有依据推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::history::replay_water::replay_water::closure$1::closure$0` | 0x141447640 | 38 | replay_raster<Water> 薄包装 | I | history\decomp\decomp-0x141447640.txt |
| `country_core::systems::history::replay_water::replay_water::closure$2` | 0x140a01560 | 43 | replay_raster<Water> 薄包装 | I | history\decomp\decomp-0x140a01560.txt |

## 内联/无独立符号函数（名称揭示意图；逻辑并入调用点）

- `country_core::systems::history::replay_water::replay_water`（行 19）— replay_raster<Water> 薄包装（I；单态 0x1409f4650）
- `country_core::systems::history::replay_water::replay_water::closure$0`（行 29）— replay_raster<Water> 薄包装（I；单态 0x1409f4650）
- `country_core::systems::history::replay_water::replay_water::closure$1`（行 36）— replay_raster<Water> 薄包装（I；单态 0x1409f4650）

<!-- 未覆盖函数数：0 -->
