# resources/loading_screen.rs

> 文件作用：启动加载屏：LoadingScreenPlugin（注册 LoadingScreenProgress + Startup/Update 系统）+ 加载屏网格与着色器。startup_loading_screen 加载 mesh "loading_screen"(hash 0x42ac8c61) + shader "loading_screen/loading_screen.raster.hlsl"并 spawn (DrawableMeshBundle, DesignatedRenderPass(0x12), DontCastShadow)；update_loading_screen 为内联函数（无独立符号），逐帧推进加载进度。

函数总计 4：业务 2 / 内联无独立符号 1 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| ``country_core::resources::loading_screen::impl$0::build`` | 0x140a1a190 | 41 | 构造/注册/生成 | O：Plugin::build（v2 名；v1 名 update_loading_screen 错位）= Bevy 系统注册 + hashbrown 表维护（键=占位 UUID 33333333-3333-3333-3333-333333333333，0xd0B 条目 progress@+0x58） | session-save\decomp\decomp-0x140a1a190.txt, round2\session-save-CLOSURE.md  |
| ``country_core::resources::loading_screen::startup_loading_screen`` | 0x140a1a1e0 | 51 | 生命周期控制 | O：Startup 系统：once 初始化 → load_mesh_into_library("loading_screen") + load_shader_into_library("loading_screen/loading_screen.raster.hlsl") → spawn 0x88B bundle（vt 0x1412da5d0, +0x48=0x12 渲染 pass, +0x58=0 DontCastShadow） | session-save\decomp\decomp-0x140a1a1e0.txt  |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::loading_screen::update_loading_screen`（行 29）— 系统:周期性处理

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项
