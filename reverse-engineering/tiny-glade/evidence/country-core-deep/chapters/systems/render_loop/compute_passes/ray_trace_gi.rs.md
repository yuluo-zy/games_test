# systems/render_loop/compute_passes/ray_trace_gi.rs

> 文件作用：GI 光线追踪：`ray_trace_gi`（W38，软件 BVH GI 候选追踪 + 辐射度更新，供 eval_gi_screen_radiance 滤波）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::ray_trace_gi::ray_trace_gi` | 0x140a01cf0 | 24 | W38 GI 光线追踪：串证据 `restir_ray_dir`（GI 光线方向纹理）；`/gi/{trace_gi_candidates, update_gi_radiance}.cs.hlsl`；软件 BVH（rt_bl_bvh/rt_triangles/rt_triangle_meta，I：无 RT capability）；bind_shadowmap + bind_terrain_heightmap + bind_terrain_lighting + read_texture + write_texture + dispatch_2d + Temporal::into_inner；GI history pair（extent 按 param_6[0xab]）创建后无条件执行 | O | render-loop\decomp\decomp-0x140a01cf0.txt（串：restir_ray_dir）；wp5 compute-passes §11 |

## 关键事实

- 输出 GI 候选（eval_gi_screen_radiance W39 消费）。
- 帧序 W38（frame-wiring，半分辨率深度后）。
