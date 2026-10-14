# systems/render_loop/draw_passes/draw_shaders/common_includes.rs

> 文件作用：draw/compute 共用绑定 include：`bind_terrain_heightmap`、`bind_terrain_lighting`、`bind_water_object_push`（ComputePass/DrawBuilder 两单态）、`bind_garden_exclusion`。

函数总计 5：业务 3 / 内联无独立符号 2 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `…common_includes::impl$2::bind_water_object_push<ComputePass>` | 0x14160e330 | 47 | 绑定水上物体推入缓冲（ComputePass 变体）：read_texture（水上物体 push buffer，供 compute 侧读） | I（名称 + 两单态结构） | render-loop\decomp\decomp-0x14160e330.txt |
| `…common_includes::impl$2::bind_water_object_push<DrawBuilder>` | 0x141631f90 | 47 | 绑定水上物体推入缓冲（DrawBuilder 变体）：read_texture（draw 侧读） | I（同型） | render-loop\decomp\decomp-0x141631f90.txt |
| `…common_includes::impl$3::bind_garden_exclusion<ComputePass>` | 0x14160e630 | 65 | 绑定花园排除掩码（ComputePass 变体）：read_texture（植被避开花园边界） | I（名称） | render-loop\decomp\decomp-0x14160e630.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `impl$0::bind_terrain_heightmap`（行 13）— 绑定地形高度纹理（全模块共享；各 compute/draw pass 复用）
- `impl$1::bind_terrain_lighting`（行 30）— 绑定地形光照纹理

## 关键事实

- 这两个 include 是水/花园/地形各 pass 的公共绑定（wp5 全模块引用 bind_terrain_heightmap/lighting）。
