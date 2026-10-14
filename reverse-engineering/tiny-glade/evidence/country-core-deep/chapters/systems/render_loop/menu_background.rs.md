# systems/render_loop/menu_background.rs

> 文件作用：主菜单背景：`MenuBackgroundPlugin::build` 注册插件，`render_menu_bg` 渲染菜单背景（高斯模糊链 `/gauss_blur.cs.hlsl:cs_main`）。

函数总计 3：业务 2 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::menu_background::impl$0::build` | 0x1408c99c0 | 24 | 插件 build：注册 MenuBackground 资源/系统（返 self，小插件） | I（插件 build 结构） | render-loop\decomp\decomp-0x1408c99c0.txt |
| `country_core::systems::render_loop::menu_background::render_menu_bg` | 0x1408c9a30 | 35 | 渲染菜单背景：串证据 "Menu bg" + `/gauss_blur.cs.hlsl:cs_main`（背景高斯模糊 pass） | O | render-loop\decomp\decomp-0x1408c9a30.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 1 项

## 关键事实

- 串证据："Menu bg"（pass 名）+ `/gauss_blur.cs.hlsl:cs_main`（高斯模糊）。
- 相关：rasterize_terrain_stroke（0x1408c7120）串里也出现 menu_background.rs 路径（跨文件引用，U 具体关联）。
