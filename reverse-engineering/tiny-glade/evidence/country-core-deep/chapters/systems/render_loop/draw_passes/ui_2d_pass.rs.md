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

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### 1. `ui_2d_pass`（0x1409c4470）
- **查询**（O，PDB）= `(Ui2dExtent, Ui2dImage, Option<CustomizationIcon>, Option<UiInteractionState>, Option<UiDrawOrder>, Option<UiInactive>)`；元素元组 0x30B：`Ui2dExtent`（4×f32）、`Ui2dImage`（名字 &str@+8/+0x10、层号@+0x18）、tint（f32@+0x18、bool@+0x21）、button（bool@+2）、排序键 `Option<&u64>`@+0x20（I：None 最小）。
- 处理链：`WindowSize::as_vec2` → World query → **sort_by 排序键** → RasterPass::new → DrawPacket::new → uniform → read_texture_view/read_texture（**TextureViewDesc view_type 1 / base_layer / layer_count**）→ set_depth_test_enable/set_blend_mode。着色器 `/ui_2d.raster.hlsl`、`/egui.raster.hlsl`、`/text.raster.hlsl`。
- 组件真名（d_gap §6）：`Ui2dTint`→CustomizationIconRead、`Ui2dButton`→UiInteractionStateRead、`UiInactive` 存在性 → uniform U_INACTIVE；字段真名仍 U。

### 2. `mei_pass`（0x1409c6700）
`Handle::desc` + `RenderGraphRecorder::create` + 同组装；`create_raw_resource("mei temp")`（颜色 tag2，I）+ `TextRenderer::render`（tg_text）由调用方提供产出；读 `GraphicsDeviceResource`（panic 串 `Resource does not exist` + `country_core::render::GraphicsDeviceResource`）。两 pass："Mei 2D" + "UI shadow combine"（mei_shadow）。
- 帧序：最终图 execute（R12-R14）内，UI 2D 之后、custom_cursor 之前。

## 关键事实

- 结构体（U 布局）：`ShapeStreamSsbo`、`TextPrimSsbo`。
- 最终图 execute（R12–R14 frame-wiring）。
