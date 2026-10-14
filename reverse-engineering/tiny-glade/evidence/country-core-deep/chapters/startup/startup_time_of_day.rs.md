# startup/startup_time_of_day.rs

> 文件作用：**ToD 启动：载入默认时刻。**

函数总计 2：业务 1 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::startup_time_of_day::startup_time_of_day` | 0x14089d6f0 | 6 | `startup_time_of_day`：启动时初始化时间（load_default_tod 内联）。 | **O** | decomp-0x14089d6f0.txt |

## 内联/无独立符号函数

- `country_core::startup::startup_time_of_day::load_default_tod`（行 10）— **I**：`load_default_tod` — 载入默认 ToD（内联）。

## 派生胶水

- 0 项： 等（derive 直接复用，不计业务）。
