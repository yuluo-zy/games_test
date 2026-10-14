# systems/render_loop/compute_passes/ssgi.rs

> 文件作用：屏幕空间全局光照：`calculate_ssgi`（W37，SSGI 求值 + 时域滤波 + 上采样）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::ssgi::calculate_ssgi` | 0x1408743b0 | 18 | W37 SSGI：串证据 `SSGI/ssgi/ssgi.cs.hlsl` + `ssgi_upsampled` + `SSGI upsample/ssgi/upsample.cs.hlsl` + `ssgi_temporal`/`ssgi_history` + `/ssgi/temporal_filter.cs.hlsl`；read_texture + write_texture + dispatch_2d + uniform | O | render-loop\decomp\decomp-0x1408743b0.txt（串：SSGI、/ssgi/ssgi.cs.hlsl、ssgi_upsampled、SSGI upsample、/ssgi/upsample.cs.hlsl、ssgi_temporal、ssgi_history、/ssgi/temporal_filter.cs.hlsl）；wp5 compute-passes §10 |

## 关键事实

- 输出 ssgi_tex / ssgi_history（时域对），供 W43 deferred_raster 读 `ssgi_tex`（wp5 draw-passes §5）。
- 帧序 W37（frame-wiring）。
