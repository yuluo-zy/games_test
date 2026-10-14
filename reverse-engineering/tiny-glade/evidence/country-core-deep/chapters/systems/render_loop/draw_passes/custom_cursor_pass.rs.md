# systems/render_loop/draw_passes/custom_cursor_pass.rs

> 文件作用：自定义光标：`custom_cursor_pass`（最终图光标光栅 pass）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::draw_passes::custom_cursor_pass::custom_cursor_pass` | 0x1409d1710 | 39 | 自定义光标 pass：串证据 `Custom cursor` + `custom_cursor`（系统名 `::f`）+ `/custom_cursor.raster.hlsl`（wp5 draw-passes §7）；WindowSize::as_vec2 + RasterPass::new + DrawPacket::new + uniform + read_texture；`CustomCursorSsbo` | O | render-loop\decomp\decomp-0x1409d1710.txt（串：Custom cursor、custom_cursor）；wp5 draw-passes §7 |

## 关键事实

- 结构体（U 布局）：`CustomCursorSsbo`（光标位置/形状）。
- 最终图 execute（R12–R14 frame-wiring）。
