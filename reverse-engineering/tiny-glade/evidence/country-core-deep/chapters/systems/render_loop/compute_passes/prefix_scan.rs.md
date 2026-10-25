# systems/render_loop/compute_passes/prefix_scan.rs

> 文件作用：前缀扫描原语：`inclusive_prefix_scan_u32_1m`（≤1M u32 的 inclusive 前缀和，grass/nani 复用）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::prefix_scan::inclusive_prefix_scan_u32_1m` | 0x14098bf60 | 8 | u32 前缀和（≤1M）：串证据 `_Prefix merge/prefix_scan/inclusive_prefix_scan_merge.cs.hlsl` + `/prefix_scan/inclusive_prefix_scan_subtotals.cs.hlsl` + `/prefix_scan/inclusive_prefix_scan_merge.cs.hlsl`；div_ceil + max + next_multiple_of + dispatch_1d；grass/nani 复用 | O | render-loop\decomp\decomp-0x14098bf60.txt（串：_Prefix merge、/prefix_scan/inclusive_prefix_scan_merge.cs.hlsl、/prefix_scan/inclusive_prefix_scan_subtotals.cs.hlsl）；wp5 compute-passes §22 |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `inclusive_prefix_scan_u32_1m`（0x14098bf60）
处理链（O，B_c3 §2）：三段（"_Prefix 1/2/merge"）：
1. `assert_eq!(n & 0x3ff, 0)`（元素数须 1024 倍数）；
2. 子和缓冲 `max(blocks,1)` 补到 **1024 的倍数 ×4B**；
3. dispatch：`n - n/2`、`blocks - blocks/2`、`n - n/2`（三段线程数）。
- shader：`/prefix_scan/{inclusive_prefix_scan_subtotals, inclusive_prefix_scan_merge}.cs.hlsl`。
- 复用点（O）：grass 瓦片前缀（`SizedRgBuffer{bundle_index, 0x80000}`，i_infra §5）、nani draw-list 直方图前缀（gpu_generate_draw_lists 章）、`prefix_sum_unsorted_instances_per_atlas_entry`（c1 系，wp5 I）。

## 关键事实

- wp5 注 `/nani/prefix_sum_unsorted_instances_per_atlas_entry.cs.hlsl`（I）为本模块复用链。
- 复用点：grass 瓦片前缀、nani draw-list 直方图前缀。
