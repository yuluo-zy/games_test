# systems/render_loop/draw_passes/logo_overlay.rs

> 文件作用：Logo 叠加层：`LogoOverlayPlugin::build`、`startup`（启动建资源）、`render_logo_overlay`（启动/加载屏 logo 渲染）。

函数总计 3：业务 2 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::draw_passes::logo_overlay::impl$0::build` | 0x1408a5f40 | 26 | LogoOverlayPlugin::build：插件注册（sub rsp 0x900 框架，存 args；NotFound 手解为真实函数；wp5 §10 LogoOverlayPlugin） | I（插件 build；名 wp5 O） | 手解：`rne-read.py --hex 0x1408a5f40 40`（`48 89 d1 e9 …` thunk 型） |
| `country_core::systems::render_loop::draw_passes::logo_overlay::render_logo_overlay` | 0x1408a5f50 | 47 | 渲染 logo 叠加：串证据 `logo_overlay`；RasterPass::new + DrawPacket::new + read_texture + set_depth_test_enable/blend（wp5 draw-passes §10） | O | render-loop\decomp\decomp-0x1408a5f50.txt（串：logo_overlay）；wp5 draw-passes §10 |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `startup`（行 33）— 启动时加载 logo 纹理（生命周期控制）

## 关键事实

- shader：`/logo_overlay.raster.hlsl`（feedback_report_bg 复用）。
