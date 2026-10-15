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
- **round3 闭合（2026-10-15）**：`Flower03Ssbo` 库 SsboId = **`0xF78C9C2EB9E1F6FB`**（O：`add_new<Flower03Ssbo>` 0x1412c4480 `movabs rdx,-0x837363d1461e905`）；
  **NAME 仍 U**（全二进制串/SPIR-V 标识符/HLSL 标识符/花名组合穷尽无命中；reconstruction 占位 `"Flower03Ssbo"` 的 xxh3=0x5f3a187f0dd4b356≠键，确认错误）。
  同态 U：`IvyFlowerSsbo`=0x1d6de5e69ac50d2a、`LilypadFlowerSsbo`=0xAE37CBCAFBE88DA4（add_new 立即数 O）。详见 `round3/RENDER-U-CLOSURE.md` §8。
- 帧序 W12→W16（frame-wiring）。

## 实现方式（算法级）

### garden_rasterize_from_cpu（W12，0x140a22140）

同 water 模板：+0x29b 判 → ssbo.version(+0x5c) ← 库(+0x5b0)；清 +0x29b；memset(bitmap@+0x98, 0, 0x200)；建纹理（0x61）；on_texture_processed + write_texture + dispatch_2d。

### garden_spawn_vegetation（W16，0xa22b80）

"Garden Marching Squares" /garden/marching_squares.cs.hlsl（方格行进，CPU 掩码→SSBO）+ /garden/spawn_small_flowers_garden.cs.hlsl + /garden/spawn_{bushes_garden,flowers_garden}.cs.hlsl；bind_terrain_heightmap + write_draw_indirect_cmd_buffer + dispatch。

**round3 闭合**：Flower03Ssbo 库 SsboId = 0xF78C9C2EB9E1F6FB（add_new 立即数 O）；IvyFlowerSsbo=0x1d6de5e69ac50d2a、LilypadFlowerSsbo=0xAE37CBCAFBE88DA4。帧序 W12→W16。
