# startup/spawn_preplaced_trees.rs

> 文件作用：**预置树生成：deserialize trees.json，spawn clearing 外围预置树。**

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::spawn_preplaced_trees::spawn_preplaced_trees` | 0x140862270 | 25 | `spawn_preplaced_trees`：载入 `trees.json`（deserialize trees）+ `clearing/tree/bush` 网格，spawn 预置树实体。 | **O** | decomp-0x140862270.txt |

## 内联/无独立符号函数

- （无）

## 派生胶水

- 1 项：closure$0 等（derive 直接复用，不计业务）。

## 实现方式（算法级，2026-10-15 深挖）

- **spawn_preplaced_trees 0x140862270（27KB，O）**：3x 文件操作（FUN_140d76ed0 + FUN_140d62810，
  读 trees.json + clearing/tree/bush 网格）+ deserialize trees（'deserialize trees' 0x142ad6e18）
  → 4 个 spawn 记录（FUN_1428a9a60/FUN_141564d30 三件套）+ 3x unwrap。
