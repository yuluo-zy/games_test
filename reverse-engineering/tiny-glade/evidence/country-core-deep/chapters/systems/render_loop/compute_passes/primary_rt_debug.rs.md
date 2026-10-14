# systems/render_loop/compute_passes/primary_rt_debug.rs

> 文件作用：主光线追踪调试视图：`primary_rt_debug`（调试用：把 RT 结果/地形/灯光绑定到调试输出）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::primary_rt_debug::primary_rt_debug` | 0x1409beca0 | 15 | RT 调试视图：debug 族（wp5）；write_texture + bind_terrain_heightmap/lighting + bind_shadowmap + dispatch | I（wp5 记名，无独立串证据） | render-loop\decomp\decomp-0x1409beca0.txt；audio-misc\decomp\decomp-0x1409beca0.txt；wp5 compute-passes §22 |

## 关键事实

- 调试用途 pass（primary ray tracing debug view），非默认帧路径（wp5 §22 原语/调试族）。
