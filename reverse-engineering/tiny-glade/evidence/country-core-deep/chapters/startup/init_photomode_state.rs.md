# startup/init_photomode_state.rs

> 文件作用：**照片模式状态初始化与启用。**

函数总计 3：业务 1 / 内联无独立符号 1 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::init_photomode_state::PhotomodeState::on_enable_photomode` | 0x140870570 | 36 | `PhotomodeState::on_enable_photomode`：机器码 — 写 float 到字段0、字段+4=-1、+8/+0xa/+0xc 清零 = 初始化 PhotomodeState 默认值（视全局标志分支）。 | **O** | decomp-0x140870570.txt |

## 内联/无独立符号函数

- `country_core::startup::init_photomode_state::init_photomode_state`（行 15）— **I**：`init_photomode_state` — 注册 PhotomodeState 资源/初始化（内联）。

## 派生胶水

- 1 项：default 等（derive 直接复用，不计业务）。
