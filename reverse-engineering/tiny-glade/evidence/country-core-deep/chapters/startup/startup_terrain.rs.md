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

## 实现方式（算法级，2026-10-15 深挖）

- **startup_terrain 0x140926bf0（28KB，O）**：
  1. grid_gen_cw(Vec, 0, 0x5a, 0x5a, 0x5a) — **90x90 地形 quad 索引网格（cw）**（行 104）
  2. 8 网格 + 6 shader 以**嵌套 Option 链**顺序载入（任一失败整体中止；每对成功 →
     压 spawn 记录 {fn-ptr 类型标识, mesh, shader, scale 1.0 (0x3f800000)}，cap-first grow FUN_1428a9a60）：
     floor(5) + floor_shader → demo_text_floor(15) + demo_terrain_overlay →
     rocky_terrain（**文件式** 'assets/data/rocky_terrain.json'，FUN_140b0eeb0）+ rocky_terrain shader →
     stairs_step_mesh(16) + stairs_step → path_pebble_mesh(16) + path_pebble →
     stairs_pebble_mesh(18) → terrain(7) + solid_vertex_color{albedo_map} → billboard_plants(16)
     + billboard_plant_shader（串 0x142aaef28–0x142aaf2d4 区）
  3. spawn 记录 fn-ptr 族：FUN_1412d7aa0(0x10B) 1412d7c70(0x78B) 1412d8b00(0x18B) 1412dc9f0(0x88B)
     1412da440 1412d9790 1412dbb00 1412db2a0 1412dd190(0x88B) 1412d6640
  4. 7x unwrap panic 行 43（0x142aae510/0x142aae4f0）
- **grid_gen_cw 0x140928100（O）**：递归生成三角索引 — `while (5 < (to+1)-from)`: 分 <=4 列块递归；
  每 quad 6 索引 {a, b, a+1, a+1, b, c}（a=from+c+r*stride，b=a+stride+1，c=a+stride+2 族）；
  每行 param_4 行 x (to-from) quad；cap-first Vec<u32>（grow FUN_1416a83a0，4B）；from>to →
  assert panic（0x142aaf2f8 区）。**grid_gen_ccw 0x140928350 = 同族 CCW 缠绕变体**。
  复用：load_mesh_atlases 的 stone_floor_/timber_/smoke parts 也用它。
- 跨模块：load_prefabs_impl 内联引用 generate_grid_cw（canopy quad 重算，PDB inline）。
