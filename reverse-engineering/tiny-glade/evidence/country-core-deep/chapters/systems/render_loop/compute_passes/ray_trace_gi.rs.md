# systems/render_loop/compute_passes/ray_trace_gi.rs

> 文件作用：GI 光线追踪：`ray_trace_gi`（W38，软件 BVH GI 候选追踪 + 辐射度更新，供 eval_gi_screen_radiance 滤波）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::ray_trace_gi::ray_trace_gi` | 0x140a01cf0 | 24 | W38 GI 光线追踪：串证据 `restir_ray_dir`（GI 光线方向纹理）；`/gi/{trace_gi_candidates, update_gi_radiance}.cs.hlsl`；软件 BVH（rt_bl_bvh/rt_triangles/rt_triangle_meta，I：无 RT capability）；bind_shadowmap + bind_terrain_heightmap + bind_terrain_lighting + read_texture + write_texture + dispatch_2d + Temporal::into_inner；GI history pair（extent 按 param_6[0xab]）创建后无条件执行 | O | render-loop\decomp\decomp-0x140a01cf0.txt（串：restir_ray_dir）；wp5 compute-passes §11 |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `ray_trace_gi`（0x140a01cf0，W38）
处理链（O，B_c3 §2 + sequence.md）：
1. **GI history temporal pair**（`get_or_create_temporal_pair`，extent 按 param_6[0xab] 字节选）创建成功才无条件执行；
2. **两 pass**（`/gi/trace_gi_candidates.cs.hlsl` → `/gi/update_gi_radiance.cs.hlsl`）+ **三个 temporal**；
3. `update` 的 `history_valid` 需**三个标志位都为 1**；
4. `bind_ray_tracing(ComputePass)`（软件 BVH：RtTriangles/RtTriangleMeta/RtBvh/WallColorId，无 RT capability，I）+ bind_shadowmap + bind_terrain_heightmap/lighting；
5. `div_up_extent`；`restir_ray_dir` 纹理（GI 光线方向）。
- 数据流：W37 SSGI 之后 → W38 GI 候选 → W39 eval_gi_screen_radiance 滤波 → W43 deferred。
- 帧序 W38（半分辨率深度 W35/reprojection W36/SSGI W37 后）。

## 关键事实

- 输出 GI 候选（eval_gi_screen_radiance W39 消费）。
- 帧序 W38（frame-wiring，半分辨率深度后）。
