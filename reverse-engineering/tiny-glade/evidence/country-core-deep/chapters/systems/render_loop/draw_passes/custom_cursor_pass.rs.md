# systems/render_loop/draw_passes/custom_cursor_pass.rs

> 文件作用：自定义光标：`custom_cursor_pass`（最终图光标光栅 pass）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::draw_passes::custom_cursor_pass::custom_cursor_pass` | 0x1409d1710 | 39 | 自定义光标 pass：串证据 `Custom cursor` + `custom_cursor`（系统名 `::f`）+ `/custom_cursor.raster.hlsl`（wp5 draw-passes §7）；WindowSize::as_vec2 + RasterPass::new + DrawPacket::new + uniform + read_texture；`CustomCursorSsbo` | O | render-loop\decomp\decomp-0x1409d1710.txt（串：Custom cursor、custom_cursor）；wp5 draw-passes §7 |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

- `custom_cursor_pass`（0x1409d1710）：`WindowSize::as_vec2` → RasterPass::new → DrawPacket::new → uniform（光标位置/形状，`CustomCursorSsbo`）→ read_texture（光标图像）→ draw；着色器 `/custom_cursor.raster.hlsl`。
- 结构体 `CustomCursorSsbo`（SsboId 取自 `get_rg_buffer<T>` 0x1412c7550，d_main §2 注）字段布局 U。
- 帧序：最终图 execute（R12-R14）内 **最后一个** UI 层 pass（UI 2D → mei → custom_cursor）；窗口光标隐藏时可能 skip（I：光标可见性门控，d_main §7 提到 cursor 可见性）。

## 关键事实

- 结构体（U 布局）：`CustomCursorSsbo`（光标位置/形状）。
- 最终图 execute（R12–R14 frame-wiring）。
