# systems/render_loop/compute_passes/spawn_vegetation_around_buildings.rs

> 文件作用：建筑周围植被生成：`spawn_vegetation_around_buildings`（W14，沿建筑周边生成灌木/花）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::spawn_vegetation_around_buildings::spawn_vegetation_around_buildings` | 0x1408e4cc0 | 19 | W14 建筑周边植被：串证据 `/garden/spawn_flowers_buildings.cs.hlsl` + `Spawn flowers around buildings/garden/spawn_flowers_buildings.cs.hlsl` + `/garden/spawn_bushes_buildings.cs.hlsl`；uniform + bind_terrain_heightmap + write_draw_indirect_cmd_buffer + dispatch | O | render-loop\decomp\decomp-0x1408e4cc0.txt（串：Spawn flowers around buildings、/garden/spawn_flowers_buildings.cs.hlsl、/garden/spawn_bushes_buildings.cs.hlsl）；wp5 compute-passes §20 |

## 关键事实

- 帧序 W14（frame-wiring；W15 read_bush_positions_for_cpu 消费）。
