# systems/render_loop/lut_gallery.rs

> 文件作用：摄影模式 LUT 画廊：`LutGalleryPlugin::build` 注册插件，`render_photomode_lut_gallery` 渲染 LUT 缩略图画廊（每项 `lut_gallery_thumbnail` 纹理）。

函数总计 4：业务 2 / 内联无独立符号 1 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::lut_gallery::impl$0::build` | 0x1408b04c0 | 35 | 插件 build：注册 LUT 画廊系统/资源（小插件，返 self；wp5 未单列） | I（插件 build 结构，无独立强证据） | render-loop\decomp\decomp-0x1408b04c0.txt |
| `country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery` | 0x1408b0560 | 55 | 摄影模式 LUT 画廊渲染：串证据 `lut_gallery_thumbnail`；读 GradingLut 列表渲染缩略图到画廊 UI 目标 | O | render-loop\decomp\decomp-0x1408b0560.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `startup`（行 41）— 启动时建 LUT 画廊资源（生命周期控制）

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项（逐 LUT 缩略图构造闭包）

## 关键事实

- 串证据：`lut_gallery_thumbnail`（每张 LUT 的缩略图纹理名）。
- 跨域：依赖 `resources/grading_lut`（audio-misc 域已覆盖 GradingLut）。
