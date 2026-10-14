# systems/render_loop/compute_passes/visibility_queries.rs

> 文件作用：可见性查询：`run_visibility_queries`（W31，GPU 可见性查询回读）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::visibility_queries::run_visibility_queries` | 0x140a2f2e0 | 14 | W31 可见性查询：visibility query 族（U 具体 shader）；read_texture + bind_terrain_heightmap + dispatch_1d | I（wp5 记名；串仅 `\x01`） | render-loop\decomp\decomp-0x140a2f2e0.txt；wp5 compute-passes §16 |

## 关键事实

- 用途：场景对象可见性判定（回读供 CPU 端剔除/布置逻辑）。
