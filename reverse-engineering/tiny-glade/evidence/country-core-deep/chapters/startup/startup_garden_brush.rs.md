# startup/startup_garden_brush.rs

> 文件作用：**花园笔刷资源载入：garden_stone + 低模花。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::startup_garden_brush::startup_garden_brush` | 0x1415f87c0 | 16 | `startup_garden_brush`：载入 `assets/meshes/garden_stone.json` + `solid_vertex_color_garden_stone.raster.hlsl` shader + garden_lowpoly_flower。 | **O** | decomp-0x1415f87c0.txt |

## 内联/无独立符号函数

- （无）

## 派生胶水

- 0 项： 等（derive 直接复用，不计业务）。

## 实现方式（算法级，2026-10-15 深挖）

- **startup_garden_brush 0x1415f87c0（O）**：2x 名式网格（garden_lowpoly_flower 等）+ 2x 文件式
  （'assets/meshes/garden_stone.json'）+ 2x shader（solid_vertex_color_garden_stone.raster.hlsl 等）
  + 2 个 spawn 记录 + 2x unwrap。
