# startup/init_color_library.rs

> 文件作用：**颜色库初始化：从 assets/colors.json 载入默认调色板。**

函数总计 2：业务 1 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::init_color_library::init_color_library` | 0x14082c000 | 12 | `init_color_library`：构造 0x12B 颜色条目（color 值 + 名字），注册进 color library 资源（FUN_1408ba290）；路径 `assets/colors.json`。 | **O** | decomp-0x14082c000.txt |

## 内联/无独立符号函数

- `country_core::startup::init_color_library::reload_color_library`（行 29）— **I**：`reload_color_library` — 重载颜色库 helper（内联）。

## 派生胶水

- 0 项： 等（derive 直接复用，不计业务）。
