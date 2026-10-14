# systems/render_loop/compute_passes/compute_garden.rs

> 文件作用：花园掩码/植被生成：`garden_rasterize_from_cpu`（W12，CPU 花园掩码 → GPU 光栅）、`garden_spawn_vegetation`（W16，花园植被程序化生成）。

函数总计 2：业务 2 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::compute_garden::garden_rasterize_from_cpu` | 0x140a22140 | 29 | W12 花园 CPU→GPU 光栅：garden 掩码上传（on_texture_processed 钩子）+ write_texture + dispatch_2d（wp5 统一模式 B1） | O | render-loop\decomp\decomp-0x140a22140.txt；terrain-water\decomp\decomp-0x140a22140.txt；wp5 compute-passes §20 |
| `country_core::systems::render_loop::compute_passes::compute_garden::garden_spawn_vegetation` | 0x140a22b80 | 67 | W16 花园植被生成：串证据 `Garden Marching Squares/garden/marching_squares.cs.hlsl`（方格行进，CPU 掩码→SSBO）+ `/garden/spawn_small_flowers_garden.cs.hlsl`（Spawn small flowers）+ `/garden/spawn_{bushes_garden,flowers_garden}.cs.hlsl`；bind_terrain_heightmap + write_draw_indirect_cmd_buffer + dispatch/2d | O | render-loop\decomp\decomp-0x140a22b80.txt（串：Garden Marching Squares、/garden/marching_squares.cs.hlsl、Spawn small flowers、/garden/spawn_small_flowers_garden.cs.hlsl）；wp5 compute-passes §20 |

## 关键事实

- 结构体（U 布局）：`BushSsbo/FlowerSsbo/Flower03Ssbo/GardenStoneSsbo`（wp5 记名）。
- 帧序 W12→W16（frame-wiring）。
