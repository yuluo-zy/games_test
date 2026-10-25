# startup/load_glade_grading.rs

> 文件作用：**glade 分级 LUT 载入（按 theme）。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::load_glade_grading::load_glade_grading` | 0x1408bf9c0 | 10 | `load_glade_grading`：按 theme（byte@+0x58 ×8）从 gladestate 载入分级配置/LUT（`assets/gladestate` 路径族）。 | **O** | decomp-0x1408bf9c0.txt |

## 内联/无独立符号函数

- （无）

## 派生胶水

- 0 项： 等（derive 直接复用，不计业务）。

## 实现方式（算法级，2026-10-15 深挖）

- **load_glade_grading 0x1408bf9c0（O）**：按 theme（byte@+0x58 x8）选 `assets/gladestate/<theme>` 路径
  （0x142aef51a 区）→ 2x 打开（FUN_140d76ed0 + FUN_140d62810 拼接）→ 分级配置经 FUN_141405ec0
  注册（2 变体：初载/reload 路径各一）→ FUN_140d7bd90 清理 x4 + 1x unwrap。
