# startup/startup_terrain.rs

> 文件作用：**地形启动：载入地形/岩石/楼梯/鹅卵石/植被网格与 shader，生成地形网格（cw/ccw），spawn 地形实体。**

函数总计 5：业务 3 / 内联无独立符号 2 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::startup_terrain::startup_terrain` | 0x140926bf0 | 0 | `startup_terrain`：载入 terrain, rocky_terrain, stairs_step, path_pebble, stairs_pebble, billboard_plants 网格（8× load_mesh_into_library）+ 6 shader（demo_terrain_overlay, rocky_terrain, stairs_step, path_pebble, solid_vertex_color{albedo_map}, billboard_plant）+ 调 grid_gen_cw（`grid cw`）+ spawn 地形实体。 | **O** | decomp-0x140926bf0.txt |
| `country_core::startup::startup_terrain::grid_gen_cw` | 0x140928100 | 281 | `grid_gen_cw`：递归生成地形 quad 网格三角形索引（CW 缠绕；param_1 = cap-first Vec，元素 4B，FUN_1416a83a0 扩容）。 | **O** | decomp-0x140928100.txt |
| `country_core::startup::startup_terrain::grid_gen_ccw` | 0x140928350 | 328 | `grid_gen_ccw`：同族 CCW 缠绕变体。 | **O** | decomp-0x140928350.txt |

## 内联/无独立符号函数

- `country_core::startup::startup_terrain::generate_grid_cw`（行 264）— **I**：`generate_grid_cw` — 生成函数包装（内联）。
- `country_core::startup::startup_terrain::generate_grid_ccw`（行 311）— **I**：`generate_grid_ccw` — 生成函数包装（内联）。

## 派生胶水

- 0 项： 等（derive 直接复用，不计业务）。
