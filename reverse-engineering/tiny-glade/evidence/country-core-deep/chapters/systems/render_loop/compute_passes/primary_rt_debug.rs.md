# systems/render_loop/compute_passes/primary_rt_debug.rs

> 文件作用：主光线追踪调试视图：`primary_rt_debug`（调试用：把 RT 结果/地形/灯光绑定到调试输出）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::primary_rt_debug::primary_rt_debug` | 0x1409beca0 | 15 | RT 调试视图：debug 族（wp5）；write_texture + bind_terrain_heightmap/lighting + bind_shadowmap + dispatch | I（wp5 记名，无独立串证据） | render-loop\decomp\decomp-0x1409beca0.txt；audio-misc\decomp\decomp-0x1409beca0.txt；wp5 compute-passes §22 |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `primary_rt_debug`（0x1409beca0）—— 调试视图
处理链（O，B_c3 §2，纯反汇编恢复）：
1. output 写（调试输出纹理）；
2. 预设 9/10/8（terrain_heightmap / visual_terrain_heightmap / terrain_lighting）+ `blue_noise`（真实 TextureFromDisk，i_infra 后外部绑定不登记 RG 访问）+ `bool_render_shadows=true` + `shadow_map`（预设 0x11）；
3. `bind_ray_tracing`（软件 BVH）；
4. `current_view_index=0`；`dispatch_2d`。
- 调试用途 pass（primary ray tracing debug view），非默认帧路径；i_infra 后访问数 10→9。

## 关键事实

- 调试用途 pass（primary ray tracing debug view），非默认帧路径（wp5 §22 原语/调试族）。
