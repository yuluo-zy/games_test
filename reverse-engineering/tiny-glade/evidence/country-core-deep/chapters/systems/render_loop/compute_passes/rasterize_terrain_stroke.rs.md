# systems/render_loop/compute_passes/rasterize_terrain_stroke.rs

> 文件作用：地形笔触光栅：`rasterize_terrain_stroke`（W3，把 CPU 地形笔触光栅到地形掩码/高度 + 高斯模糊 + 高度图重算）。

函数总计 5：业务 1 / 内联无独立符号 1 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke` | 0x1408c7120 | 65 | W3 地形笔触光栅：串证据 `Blur Terrain Stroke` + `Terrain Derivative/terrain_derivative.cs.hlsl` + blur temp + `assertion`（多段）；Events::is_empty 守卫 + DynamicConstantsChunk::with_data + DynamicConstants::push_slice + uniform + write_texture + dispatch_2d + bind_terrain_heightmap；GaussBlur（`/gauss_blur.cs.hlsl:cs_main`）+ RecalculateHeightMapCmd（U） | O | render-loop\decomp\decomp-0x1408c7120.txt（串：Blur Terrain Stroke、Terrain Derivative、/terrain_derivative.cs.hlsl、blur temp）；terrain-water\decomp\decomp-0x1408c7120.txt；wp5 compute-passes §21 |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `impl$0::as_shader_def_value`（行 43）— shader def 值转换（terrain stroke 类型 → def）

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`hash` 等 3 项

## 关键事实

- 输出地形高度/掩码（供 W4 displace_rocky_terrain、地形网格跟随等）。
- 帧序 W3（frame-wiring）。
