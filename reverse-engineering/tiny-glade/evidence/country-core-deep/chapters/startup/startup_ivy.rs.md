# startup/startup_ivy.rs

> 文件作用：**常春藤资源载入：ivy 分支/叶/花网格 + instanced shader。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::startup_ivy::startup_ivy` | 0x140a21a30 | 15 | `startup_ivy`：载入 ivy_branch, ivy_leaf, ivy_flower 网格 + `instanced_ivy_branch`/`instanced_ivy_leaf` shader + ivy_debug_cube。 | **O** | decomp-0x140a21a30.txt |

## 内联/无独立符号函数

- （无）

## 派生胶水

- 0 项： 等（derive 直接复用，不计业务）。

## 实现方式（算法级，2026-10-15 深挖）

- **startup_ivy 0x140a21a30（O）**：4x 名式网格（ivy_branch、ivy_leaf、ivy_flower、ivy_debug_cube）
  + instanced_ivy_branch/instanced_ivy_leaf shader + 3 个 spawn 记录（FUN_1412dd460）+ 3x unwrap。
