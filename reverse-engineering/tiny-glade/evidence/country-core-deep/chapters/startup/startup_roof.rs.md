# startup/startup_roof.rs

> 文件作用：**屋顶资源载入：roof_spire/support_beam 网格 + shader + 删除笔刷。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::startup_roof::startup_roof` | 0x1414547d0 | 14 | `startup_roof`：载入 roof_spire, roof_support_beam, roof_delete_scribble 网格 + `instanced_roof_spire` shader + roof_button。 | **O** | decomp-0x1414547d0.txt |

## 内联/无独立符号函数

- （无）

## 派生胶水

- 0 项： 等（derive 直接复用，不计业务）。
