# systems/render_loop/compute_passes/ssgi.rs

> 文件作用：屏幕空间全局光照：`calculate_ssgi`（W37，SSGI 求值 + 时域滤波 + 上采样）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::ssgi::calculate_ssgi` | 0x1408743b0 | 18 | W37 SSGI：串证据 `SSGI/ssgi/ssgi.cs.hlsl` + `ssgi_upsampled` + `SSGI upsample/ssgi/upsample.cs.hlsl` + `ssgi_temporal`/`ssgi_history` + `/ssgi/temporal_filter.cs.hlsl`；read_texture + write_texture + dispatch_2d + uniform | O | render-loop\decomp\decomp-0x1408743b0.txt（串：SSGI、/ssgi/ssgi.cs.hlsl、ssgi_upsampled、SSGI upsample、/ssgi/upsample.cs.hlsl、ssgi_temporal、ssgi_history、/ssgi/temporal_filter.cs.hlsl）；wp5 compute-passes §10 |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `calculate_ssgi`（0x1408743b0，W37）
处理链（O，B_c3 §2）：
1. **三 pass**：SSGI（`/ssgi/ssgi.cs.hlsl`）→ upsample（`/ssgi/upsample.cs.hlsl`）→ temporal_filter（`/ssgi/temporal_filter.cs.hlsl`）；
2. **第一遍 `depth_tex` 绑半分辨率深度**（W35 的 half-res depth）；
3. `ssgi_temporal`/`ssgi_history`（时域对）；第三张 ssgi_temporal extent 来源 I（全分辨率）。
- 数据流：ssgi_tex → `deferred_raster`（W43）读 + GI 链（W38 ray_trace_gi 之后）。
- 帧序 W37（W36 reprojection 后、W38 前）。

## 关键事实

- 输出 ssgi_tex / ssgi_history（时域对），供 W43 deferred_raster 读 `ssgi_tex`（wp5 draw-passes §5）。
- 帧序 W37（frame-wiring）。
