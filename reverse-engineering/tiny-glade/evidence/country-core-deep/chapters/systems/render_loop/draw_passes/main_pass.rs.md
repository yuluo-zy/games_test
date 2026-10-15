# systems/render_loop/draw_passes/main_pass.rs

> 文件作用：主不透明 pass 族：`main_pass`（W25，G-buffer 几何 pass）、`main_pass_oc2`（W30，HZB 后二次遮挡判定的 OC/2 分辨率 pass）、`late_forward_and_deferred_pass`（W34，延迟光照 + nani 前向组合）。均用 `RasterPass::new` + `CommonDrawParams::new` + 遍历 `DrawListItem` → `to_draw_packet` + `with_stencil_write`（主 pass 写 stencil）。

函数总计 4：业务 3 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::draw_passes::main_pass::main_pass` | 0x140833bc0 | 31 | W25 主不透明 pass：puffin scope + RasterPass::new + CommonDrawParams::new + 遍历 DrawListItem + to_draw_packet + DrawBuilder::stencil_ref + with_stencil_write；渲染目标名（串证据）：G-buffer / G-buffer emissive / G-buffer velocity / Forward / Velocity | O | render-loop\decomp\decomp-0x140833bc0.txt（串：G-buffer、G-buffer emissive、G-buffer velocity、Forward、Velocity）；wp5 draw-passes §2 |
| `country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2` | 0x140835a40 | 248 | W30 二次遮挡 pass（OC/2 半分辨率）：同 main_pass（G-buffer OC/2、G-buffer emissive OC/2、G-buffer velocity OC/2、Forward OC/2、Velocity OC/2），无 ShaderRefactoringState 判定 | O | render-loop\decomp\decomp-0x140835a40.txt（串：\* OC/2）；wp5 draw-passes §2 |
| `country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass` | 0x1408373b0 | 458 | W34：`impl$0::is_empty` 守卫 + 同组装（nani_* raster + /deferred/*_light_sun_sky.raster 组合）；目标名：Late Forward / Late Deferred | O | render-loop\decomp\decomp-0x1408373b0.txt（串：Late Forward、Late Deferred）；wp5 draw-passes §2 |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `with_stencil_write`（行 22）— `RasterPipelineOpts::set_stencil_test_enable` + `set_stencil_pass_op`（主 pass 写 stencil，wp5 O）

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### 1. `main_pass`（0x140833bc0）—— 6 子 pass（d_main §1.3 完整解码）
| 子 pass | 目标名 | 颜色 tag | 深度 | 列表 |
|---|---|---|---|---|
| G-buffer | "G-buffer" | **tag2**（首个） | 写 | 1 |
| emissive | "G-buffer emissive" | tag0 | 写 | 2 |
| velocity | "G-buffer velocity" | tag0 | 写 | 3 |
| Forward | "Forward" | tag0 | 写 | 4（**ShaderRefactoringState disc==0 门控**） |
| ForwardVelocity | "Forward Velocity" | tag0 | 写 | 5（同上门控） |
| Velocity | "Velocity" | tag0 | 写 | 6 |
组装：RasterPass::new（每个子 pass 一个）+ `CommonDrawParams::new` + 遍历 DrawListItem → `to_draw_packet`（head=[1,vao]）→ `DrawBuilder::stencil_ref`（None → unwrap panic）→ **`with_stencil_write`（byte1=(b&0x8e)|0x21）** → bind_common → 回调。访问码 0x36/0x2e/0x2f/0x17。

### 2. `main_pass_oc2`（0x140835a40）—— 5 子 pass
同 main_pass（G-buffer OC/2、emissive OC/2、velocity OC/2、Forward OC/2、Velocity OC/2），**无 ShaderRefactoringState 门控**。HZB 后二次遮挡判定（帧序 W30，`freeze_occlusion_culling` 门控 round2 绘制链，render_world 章）。

### 3. `late_forward_and_deferred_pass`（0x1408373b0）—— W34
`impl$0::is_empty`（**列表 12/13 都空则不建 pass**）守卫 → 同组装；nani_* raster + `/deferred/{solid,grass,plant,canopy}_light_sun_sky.raster` 组合；目标名 "Late Forward"/"Late Deferred"（列表 12/13）。
- 数据流：G-buffer（W25）→ HZB（W26）→ OC/2（W30）→ GI 链（W32-W43）→ **W34 光照组合**：延迟着色 + nani 前向，之后 W44 透明链。
- 资源：main 深度（写）、反照率/速度（写）、frame_constants（读）。

## 关键事实

- 资源：main 深度（写）、反照率/速度（写）、`frame_constants`（读，wp5 O）。
- 帧序：W25→（compute 链）→W30→W34（frame-wiring）。
- Shader URI：nani_* raster（uber/albedo_map/vertex_color/velocity_only…）+ `/deferred/{solid,grass,plant,canopy}_light_sun_sky.raster`（W34）。
