# systems/render_loop/compute_passes/path_pebbles.rs

> 文件作用：路径卵石生成：`path_pebbles_compute`（W17，沿路径生成卵石实例）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::path_pebbles::path_pebbles_compute` | 0x140958f70 | 20 | W17 路径卵石：串证据 `Pebble Path/path_pebbles.cs.hlsl` + `/path_pebbles.cs.hlsl`；write_draw_indirect_cmd_buffer + uniform + read_texture + bind_terrain_heightmap + dispatch；`PebblePlacementSsbo` | O | render-loop\decomp\decomp-0x140958f70.txt（串：Pebble Path、/path_pebbles.cs.hlsl）；wp5 compute-passes §21 |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `path_pebbles_compute`（0x140958f70，W17）
处理链（O，B_c3 §2）：
1. **5 个纹理 + 3 个 i32 uniform（3/0x240/0x18）**；
2. `write_draw_indirect_cmd_buffer` + read_texture + `bind_terrain_heightmap`；
3. `dispatch_1d(0x240)`（576）；`PebblePlacementSsbo`（48B = [u32;12]，B_c3 §6）。
- 数据流：路径掩码（W10）→ 卵石放置（W17）→ `path_pebble_draw`（draw_shaders 章）draw_instanced。
- 帧序 W17（leaf_particles W18 前）。

## 关键事实

- 结构体（U 布局）：`PebblePlacementSsbo`；输出由 path_pebble_draw（draw_shaders）消费。
- 帧序 W17（frame-wiring）。
