# systems/render_loop/draw_passes/foliage_exclusion_mask.rs

> 文件作用：植被排除掩码光栅：`rasterize_foliage_exclusion_mask`（W7，把建筑/墙体光栅到排除掩码，供植被避开）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::draw_passes::foliage_exclusion_mask::rasterize_foliage_exclusion_mask` | 0x1409cb7b0 | 19 | W7 植被排除掩码：串证据 `Foliage exclusion`（pass 名）+ 系统 `::f` 名；RasterPass::new + DrawPacket::new + set_cull_mode + bind_terrain_heightmap | O | render-loop\decomp\decomp-0x1409cb7b0.txt（串：Foliage exclusion）；wp5 draw-passes §10 |

## 关键事实

- 输出排除掩码供植被生成（garden/water spawn 避开建筑墙）。
- 帧序 W7（frame-wiring）。
