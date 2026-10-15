# startup/startup_nani.rs

> 文件作用：**nani（GPU 实例渲染后端）启动与热重载：创建世界子集、检查运行时限制。**

函数总计 5：业务 2 / 内联无独立符号 2 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::startup_nani::startup_nani` | 0x140a38610 | 23 | `startup_nani`：Bevy 系统入口（间接跳转分发到实际系统函数）。 | **O** | decomp-0x140a38610.txt |
| `country_core::startup::startup_nani::reload_nani` | 0x140a398b0 | 206 | `reload_nani`：重载 nani 配置（遍历实例/world subset 重建；内联 runtime_check_limits / create_world_subset）。 | **O** | decomp-0x140a398b0.txt |

## 内联/无独立符号函数

- `country_core::startup::startup_nani::runtime_check_limits`（行 153）— **I**：`runtime_check_limits` — 运行时限制检查（内联）。
- `country_core::startup::startup_nani::create_world_subset`（行 166）— **I**：`create_world_subset` — 创建 nani 世界子集（内联）。

## 派生胶水

- 1 项：closure$0 等（derive 直接复用，不计业务）。

## 实现方式（算法级，2026-10-15 深挖）

- **startup_nani 0x140a38610（O）**：Bevy 系统入口（间接跳转分发到实际系统函数）。
- **reload_nani 0x140a398b0（O）**：遍历 nani 实例/world subset 重建 —
  FUN_140ae6750（subset 重建，索引 &0xffffffff）→ FUN_140beefa0（重建结果接线）→
  记录 push（FUN_140b724f0/FUN_140b761c0/FUN_1428c08b0）→ `.unwrap()` panic 行 43，
  文件 `crates/country-core/src/render/nani/gpu_world.rs`（0x142aba4d8）。
  内联 runtime_check_limits（行 153）/ create_world_subset（行 166）。
