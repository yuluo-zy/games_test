# systems/wall/wall_state/publish_public_wall_state.rs

> 文件作用：公共墙状态发布（权威流）：14 系统参数；收集 lazy 更新源到逐墙重建 public 记录（源曲线点/形状/style halftimber/material_stats/SparseInstanceBuffer::allocate/哈希段维护）；AnimateWallBulldozeBuffer::contains 保留动画墙；发 OnWallChanged/OnWallDeleted（16B 行）；断言 !is_adding 与 w.simplified_curve_2d.is_valid()。

函数总计 12：业务 1 / 内联无独立符号 2 / 派生胶水 9

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::wall_state::publish_public_wall_state::publish_public_wall_state` | 0x140a8b530 | 51 | publish_public_wall_state（14 参；lazy 源→重建 public→SparseInstanceBuffer::allocate→OnWallChanged/Deleted） | O | wall\decomp\decomp-0x140a8b530.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources`（行 206）— 待推断
- `country_core::systems::wall::wall_state::publish_public_wall_state::process`（行 309）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 9 项
