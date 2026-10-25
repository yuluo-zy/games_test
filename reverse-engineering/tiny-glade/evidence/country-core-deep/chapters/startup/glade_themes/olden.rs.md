# startup/glade_themes/olden.rs

> 文件作用：**Olden 主题资源载入：主题树配置 + clover 花。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::glade_themes::olden::startup_olden` | 0x1415539d0 | 18 | `startup_olden`：载入 trees/{rocks,clearing_edge_rocks,stumps,clearing_edge_tree_stumps,logs}.json 树配置 + clover_flowers 网格。 | **O** | decomp-0x1415539d0.txt |

## 内联/无独立符号函数

- （无）

## 派生胶水

- 0 项： 等（derive 直接复用，不计业务）。

## 实现方式（算法级，2026-10-15 深挖）

- **startup_olden 0x1415539d0（O）**：2x 名式网格 + 2x 文件式网格（'assets/meshes/clover.json'）+
  2x shader + **6 份树配置** trees/{rocks,clearing_edge_rocks,stumps,clearing_edge_tree_stumps,
  logs,clearing_edge_tree_logs}.json（串 0x142c6c918 区）+ 2 个注册记录（FUN_1412d9400/1412d9270）
  + 2x unwrap。
- 跨模块：init_clearing 直接调用本函数（0x140a78aa0 → 0x1415539d0）。
