# systems/render_loop/compute_passes/bin_lights.rs

> 文件作用：灯光分箱：`bin_lights`（W40，把灯源 bin 到 3D 单元格网格，供延迟光照/ReSTIR 查询）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::bin_lights::bin_lights` | 0x1409e7770 | 37 | W40 灯光分箱：`BinLights` AsComputeShaderSource → `/light_binning/bin_lights_to_cells.cs.hlsl:cs_main`；+ `/light_binning/{cell_prefix_sum, cell_subtotal_prefix_sum, cell_prefix_merge}.cs.hlsl`；AssetSsboLibrary::get + InstanceBuffer::instance_count + uniform + push_constants + dispatch_1d；串仅 `\x01`/`\x08`（名串在 shader 反射表） | O | render-loop\decomp\decomp-0x1409e7770.txt；wp5 compute-passes §13 |

## 关键事实

- 输出：`light_indirection_grid` / `grid_light_indices`（deferred_raster 读，wp5 draw-passes §5）。
- 帧序 W40（frame-wiring，光源 bin 后供 W43 deferred 光照）。
