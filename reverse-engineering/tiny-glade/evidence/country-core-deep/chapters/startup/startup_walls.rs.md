# startup/startup_walls.rs

> 文件作用：**墙启动：载入 wall brick LOD shader，生成 half_cube 网格。**

函数总计 3：业务 1 / 内联无独立符号 2 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::startup_walls::glade_startup_walls` | 0x140a92670 | 30 | `glade_startup_walls`：载入 `wall_brick_lod0`/`wall_brick_lod1` shader（`/wall/wall_brick_lod*.raster.hlsl`）+ 生成 `half_cube`（brick half cube）网格（字节常量 0x01-0x07 为三角索引）。 | **O** | decomp-0x140a92670.txt |

## 内联/无独立符号函数

- `country_core::startup::startup_walls::once_startup_walls`（行 19）— **I**：`once_startup_walls` — 一次性墙初始化（内联）。
- `country_core::startup::startup_walls::create_half_cube_mesh`（行 120）— **I**：`create_half_cube_mesh` — 半立方体网格生成器（内联）。

## 派生胶水

- 0 项： 等（derive 直接复用，不计业务）。

## 实现方式（算法级，2026-10-15 深挖）

- **glade_startup_walls 0x140a92670（O）**：
  1. 2x load_shader_into_library<&str>（FUN_140b3fe40）：'/wall/wall_brick_lod0.raster.hlsl' +
     'wall_brick_lod0'、'/wall/wall_brick_lod1.raster.hlsl' + 'wall_brick_lod1'（串 0x142ada29e 区）
  2. 2x load_mesh_into_library（FUN_140b1aca0）：'half_cube'（0x142ada295）/'brick half cube'（0x142ada2fe）
  3. **7x 顶点/索引构造调用**（FUN_1428a9a60/FUN_141564d30/FUN_140d146a0 三件套 x7 =
     create_half_cube_mesh 内联：字节常量 0x01–0x07 三角索引族）
- 消费：wall_brick LOD shader → 墙渲染 LOD（systems/wall 域）。
