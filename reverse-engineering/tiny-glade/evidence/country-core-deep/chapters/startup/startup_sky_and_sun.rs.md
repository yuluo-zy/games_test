# startup/startup_sky_and_sun.rs

> 文件作用：**天空/太阳资源载入：skydome mesh + shader。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::startup_sky_and_sun::startup_sky_and_sun` | 0x140864430 | 14 | `startup_sky_and_sun`：载入 skydome（`skydome_shader`，来自 systems/backend/vao_update.rs 的 VAO 构建路径）。 | **O** | decomp-0x140864430.txt |

## 内联/无独立符号函数

- （无）

## 派生胶水

- 0 项： 等（derive 直接复用，不计业务）。
