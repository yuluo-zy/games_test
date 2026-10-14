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
