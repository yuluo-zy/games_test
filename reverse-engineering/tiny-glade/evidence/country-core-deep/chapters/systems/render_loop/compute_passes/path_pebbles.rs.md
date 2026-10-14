# systems/render_loop/compute_passes/path_pebbles.rs

> 文件作用：路径卵石生成：`path_pebbles_compute`（W17，沿路径生成卵石实例）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::path_pebbles::path_pebbles_compute` | 0x140958f70 | 20 | W17 路径卵石：串证据 `Pebble Path/path_pebbles.cs.hlsl` + `/path_pebbles.cs.hlsl`；write_draw_indirect_cmd_buffer + uniform + read_texture + bind_terrain_heightmap + dispatch；`PebblePlacementSsbo` | O | render-loop\decomp\decomp-0x140958f70.txt（串：Pebble Path、/path_pebbles.cs.hlsl）；wp5 compute-passes §21 |

## 关键事实

- 结构体（U 布局）：`PebblePlacementSsbo`；输出由 path_pebble_draw（draw_shaders）消费。
- 帧序 W17（frame-wiring）。
