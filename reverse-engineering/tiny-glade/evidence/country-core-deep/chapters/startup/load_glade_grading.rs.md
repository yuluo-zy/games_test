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
