# systems/render_loop/compute_passes/highlight_active_terrain_stroke.rs

> 文件作用：活跃地形笔触高亮：`highlight_active_terrain_stroke`（W8，当前地形笔触高亮掩码生成 + 高斯模糊）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::highlight_active_terrain_stroke::highlight_active_terrain_stroke` | 0x140a58450 | 20 | W8 地形笔触高亮：串证据 `Create a highlighted terrain stroke mask/terrain_editing/calc_highlight_hill_mask.cs.hlsl` + `Blur Terrain Highlight Mask/gauss_blur_r16f.cs.hlsl` + blur temp；EventCursor::read + last + uniform + write_texture + dispatch；`AddTerrainHighlightCmd` | O | render-loop\decomp\decomp-0x140a58450.txt（串：Create a highlighted terrain stroke mask、/terrain_editing/calc_highlight_hill_mask.cs.hlsl、Blur Terrain Highlight Mask、/gauss_blur_r16f.cs.hlsl）；terrain-water\decomp\decomp-0x140a58450.txt；wp5 compute-passes §21 |

## 关键事实

- 输出高亮掩码 + r16f 高斯模糊（blur temp）。
