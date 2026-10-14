# systems/render_loop/draw_passes/ui_2d_pass.rs

> 文件作用：最终图 UI 2D：`ui_2d_pass`（UI 2D 几何/文本光栅）、`mei_pass`（MEI 2D 后处理，UI 阴影合并）。

函数总计 4：业务 2 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::draw_passes::ui_2d_pass::ui_2d_pass` | 0x1409c4470 | 29 | UI 2D pass：串证据 `UI 2D` + `ui_2d` + `ui_2d_color_button` + 系统 `::f` 名；WindowSize::as_vec2 + World::query（UI 实体）+ sort_by + RasterPass::new + DrawPacket::new + uniform + read_texture_view/read_texture + set_depth_test_enable/set_blend_mode；shader `/ui_2d.raster.hlsl`、`/egui.raster.hlsl`、`/text.raster.hlsl` | O | render-loop\decomp\decomp-0x1409c4470.txt（串：UI 2D、ui_2d、ui_2d_color_button）；ui-input\decomp\decomp-0x1409c4470.txt；wp5 draw-passes §7 |
| `country_core::systems::render_loop::draw_passes::ui_2d_pass::mei_pass` | 0x1409c6700 | 205 | MEI 2D：串证据 `Mei 2D` + `mei temp` + `UI shadow combine` + `mei_shadow`；Handle::desc + RenderGraphRecorder::create + 同组装；读 GraphicsDeviceResource（panic 串 `Resource does not exist` + `country_core::render::GraphicsDeviceResource`） | O | render-loop\decomp\decomp-0x1409c6700.txt（串：Mei 2D、mei temp、UI shadow combine、mei_shadow）；ui-input\decomp\decomp-0x1409c6700.txt；wp5 draw-passes §7 |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 2 项（sort_by 键闭包）

## 关键事实

- 结构体（U 布局）：`ShapeStreamSsbo`、`TextPrimSsbo`。
- 最终图 execute（R12–R14 frame-wiring）。
