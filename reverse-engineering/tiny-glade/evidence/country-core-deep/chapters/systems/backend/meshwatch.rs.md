# systems/backend/meshwatch.rs

> 文件作用：**网格热载系统。每帧锁 `MeshWatch` 互斥队列，日志 `'Meshwatch: detected changes: N'`，按位图索引遍历变更网格并调 `AssetMeshLibrary::reload_mesh`（`FUN_14092d8d0`，JSON 网格重载）；panic `Result::unwrap()`（meshwatch.rs:43）。**

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::backend::meshwatch::meshwatch` | 0x14085b4b0 | 5 | Bevy 系统（ResMut，+0x1c tick 回写）。锁 MeshWatch 互斥项（`*param+0x30 → +0x10` 标志）；日志 `'Meshwatch: detected changes: '`（0x142a95140）+ 计数；u16 位图迭代器遍历变更网格句柄（每项 0x20B 位图槽），逐项调 `FUN_14092d8d0(..., ResMut)` = `AssetMeshLibrary::reload_mesh`；引用 `cloud/cloud.raster.hlsl` / `cloud_shader`（0x142a95168，云网格重载亦经此路径）。 | O | decomp/decomp-0x14085b4b0.txt；rne-read --unks（0x142a95140 / 0x142a95168） |
| （助手）`FUN_14092d8d0` | 0x14092d8d0 | — | `AssetMeshLibrary::reload_mesh`：mesh_library.rs；读 `'json'` 网格文件，日志 `'Asset mesh library couldnt reload a mesh: '`（0x142aafd88）、`'Asset Mesh library was requested to reload a non-existing mesh path '`（0x142aafe20）。 | O | decomp/decomp-0x14092d8d0.txt；rne-read --unks |

## 实现方式（算法级）

**meshwatch @0x14085b4b0（系统，O）**：
1. LOCK MeshWatch 互斥项（`*param+0x30 → +0x10` 标志）；未空闲 → panic `Result::unwrap()`（meshwatch.rs:43，0x142a949f0）。（O）
2. 日志 `'Meshwatch: detected changes: '`（0x142a95140）+ 变更数。（O）
3. u16 位图迭代器遍历变更网格句柄（0x20B 位图槽），逐项调 `FUN_14092d8d0(..., ResMut)` 重载。（O）
4. 引用 `cloud/cloud.raster.hlsl`/`cloud_shader`（0x142a95168）—— 云网格亦经此路径。（O）

**FUN_14092d8d0 @0x14092d8d0（AssetMeshLibrary::reload_mesh，O）**：
- 读 `'json'` 网格文件；失败 log `'Asset mesh library couldnt reload a mesh: '`（0x142aafd88）；路径不存在 → `'Asset Mesh library was requested to reload a non-existing mesh path '`（0x142aafe20）。（O）

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

（无）

## 派生胶水（derive/宏生成；不计独立业务）

（无）

## 备注

- `cloud_shader` 字符串来自内联的 cloud 网格处理；`MeshWatch` 队列元素含 0x20B 条目（与 texture_watch 同构）。
