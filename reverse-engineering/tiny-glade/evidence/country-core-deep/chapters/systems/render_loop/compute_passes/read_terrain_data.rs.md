# systems/render_loop/compute_passes/read_terrain_data.rs

> 文件作用：地形数据回读：`read_terrain_data`（W5，组合读地形高度/岩质掩码/水面掩码/花园掩码四个回读 dispatch），`read_terrain_height`/`read_terrain_rock_mask`/`read_water_mask`/`read_garden_mask`（内联实现）。

函数总计 5：业务 1 / 内联无独立符号 4 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::read_terrain_data::read_terrain_data` | 0x14098c680 | 48 | W5 地形数据回读：串证据 `Visual terrain height` + `/terrain_editing/read_terrain_height.cs.hlsl` + `Read terrain rock mask/terrain_editing/read_terrain_rock_mask.cs.hlsl` + `Read water/water/read_water_mask.cs.hlsl` + `Read garden/garden/read_garden_mask.cs.hlsl` + rocky_terrain；`ScheduledTerrainReadback::on_gpu_copy_issued`；各 dispatch_1d 回读 | O | render-loop\decomp\decomp-0x14098c680.txt（串：Visual terrain height、Read terrain rock mask、Read water、Read garden、/terrain_editing/read_terrain_height.cs.hlsl、/water/read_water_mask.cs.hlsl、/garden/read_garden_mask.cs.hlsl）；terrain-water\decomp\decomp-0x14098c680.txt；wp5 compute-passes §21 |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `read_terrain_height`（行 162）— `/terrain_editing/read_terrain_height.cs.hlsl` 回读
- `read_terrain_rock_mask`（行 181）— `/terrain_editing/read_terrain_rock_mask.cs.hlsl` 回读（rocky_terrain）
- `read_water_mask`（行 196）— `/water/read_water_mask.cs.hlsl` 回读
- `read_garden_mask`（行 207）— `/garden/read_garden_mask.cs.hlsl` 回读

## 关键事实

- 回读结果经 `ScheduledTerrainReadback`（terrain-water 域）供 CPU 编辑/拾取。
