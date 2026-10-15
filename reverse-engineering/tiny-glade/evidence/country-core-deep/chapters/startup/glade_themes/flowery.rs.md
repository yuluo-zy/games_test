# startup/glade_themes/flowery.rs

> 文件作用：**Flowery 主题资源载入：主题植物（lavender/低模花）+ 树配置。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::glade_themes::flowery::startup_flowery` | 0x141553120 | 17 | `startup_flowery`：载入 billboard_plants_2, plants/lavender.json, meadow_lowpoly_flowers 网格 + trees/{rocks,clearing_edge_rocks,stumps,clearing_edge_tree_stumps}.json 树配置。 | **O** | decomp-0x141553120.txt |

## 内联/无独立符号函数

- （无）

## 派生胶水

- 0 项： 等（derive 直接复用，不计业务）。

## 实现方式（算法级，2026-10-15 深挖）

- **startup_flowery 0x141553120（O）**：2x 名式网格 + 2x 文件式网格（meadow_lowpoly_flowers、
  'plants/lavender.json'）+ 2x shader（FUN_140937aa0：'billboard_plant_shader' 0x142c6c862、
  'meadow_flower' 0x142c6c8f3）+ 树配置 trees/{rocks,clearing_edge_rocks,stumps,
  clearing_edge_tree_stumps}.json（FUN_140d62810 拼接）+ 2 个注册记录（FUN_1412dabb0/1412d9de0）
  + 2x unwrap。
- 跨模块：init_clearing 直接调用本函数（0x140a78aa0 → 0x141553120）。
