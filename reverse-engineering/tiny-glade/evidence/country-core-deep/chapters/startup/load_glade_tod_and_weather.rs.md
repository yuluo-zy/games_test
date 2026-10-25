# startup/load_glade_tod_and_weather.rs

> 文件作用：**glade ToD/天气 与雪粒子载入（按 theme）。**

函数总计 3：业务 2 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::load_glade_tod_and_weather::load_glade_tod_and_weather` | 0x140ae7810 | 0 | `load_glade_tod_and_weather`：按 theme（byte@+0x58 ×8）载入 `assets/gladestate/<theme>` 的 ToD/weather 配置（FUN_140d76ed0 打开路径）。 | **O** | decomp-0x140ae7810.txt |
| `country_core::startup::load_glade_tod_and_weather::load_snow_particles` | 0x140ae7d90 | 45 | `load_snow_particles`：载入雪粒子配置/资源。 | **O** | decomp-0x140ae7d90.txt |

## 内联/无独立符号函数

- （无）

## 派生胶水

- 1 项：closure$0 等（derive 直接复用，不计业务）。

## 实现方式（算法级，2026-10-15 深挖）

- **load_glade_tod_and_weather 0x140ae7810（O）**：按 theme 路径（assets/gladestate/<theme>，0x142aef51a 区）
  → 2x 打开 + 拼接（FUN_140d76ed0/FUN_140d62810）→ **共用 ToD 载入器 FUN_140d59140 +
  FUN_1414051e0**（2 变体：theme 目录与 'assets/tod/default.json' 0x142a99540(23)）→ 3x unwrap panic。
- **load_snow_particles 0x140ae7d90（O）**：全局生成器/RNG 状态（FUN_140d4d740/FUN_140d64e00）
  + 记录注册（FUN_1412fa8e0 + FUN_1412dc470）+ assert panic（FUN_1428d9310）。
- 跨模块：startup_time_of_day 复用同一 ToD 载入器（FUN_140d59140 + FUN_1414051e0）。
