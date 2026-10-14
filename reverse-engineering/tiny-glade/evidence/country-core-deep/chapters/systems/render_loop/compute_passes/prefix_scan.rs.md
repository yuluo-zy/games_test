# systems/render_loop/compute_passes/prefix_scan.rs

> 文件作用：前缀扫描原语：`inclusive_prefix_scan_u32_1m`（≤1M u32 的 inclusive 前缀和，grass/nani 复用）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::prefix_scan::inclusive_prefix_scan_u32_1m` | 0x14098bf60 | 8 | u32 前缀和（≤1M）：串证据 `_Prefix merge/prefix_scan/inclusive_prefix_scan_merge.cs.hlsl` + `/prefix_scan/inclusive_prefix_scan_subtotals.cs.hlsl` + `/prefix_scan/inclusive_prefix_scan_merge.cs.hlsl`；div_ceil + max + next_multiple_of + dispatch_1d；grass/nani 复用 | O | render-loop\decomp\decomp-0x14098bf60.txt（串：_Prefix merge、/prefix_scan/inclusive_prefix_scan_merge.cs.hlsl、/prefix_scan/inclusive_prefix_scan_subtotals.cs.hlsl）；wp5 compute-passes §22 |

## 关键事实

- wp5 注 `/nani/prefix_sum_unsorted_instances_per_atlas_entry.cs.hlsl`（I）为本模块复用链。
- 复用点：grass 瓦片前缀、nani draw-list 直方图前缀。
