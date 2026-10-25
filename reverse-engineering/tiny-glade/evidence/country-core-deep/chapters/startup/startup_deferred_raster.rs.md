# startup/startup_deferred_raster.rs

> 文件作用：**deferred raster 阶段 shader 载入。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::startup::startup_deferred_raster::startup_deferred_raster` | 0x140a97a30 | 11 | `startup_deferred_raster`：载入 4 个 shader 并注册句柄（param_4 数组记录 {key,handle}）。 | **O** | decomp-0x140a97a30.txt |

## 内联/无独立符号函数

- （无）

## 派生胶水

- 0 项： 等（derive 直接复用，不计业务）。

## 实现方式（算法级，2026-10-15 深挖）

- **startup_deferred_raster 0x140a97a30（O）**：4x load_shader_into_library<&str>（FUN_140b3fe40）
  → 句柄对 {key, handle} 写入 param_4 数组（每项 2x8B，供 deferred raster pass 消费）。
