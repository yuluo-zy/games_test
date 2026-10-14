# systems/render_loop/compute_passes/taa.rs

> 文件作用：时域抗锯齿：`calculate_taa`（W52，TAA 时域积累）、`calculate_tgsr`（TGSR 升采样超分，替代 TAA）。

函数总计 2：业务 2 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::taa::calculate_taa` | 0x140a3b2f0 | 28 | W52 TAA：串证据 `TAA/taa.cs.hlsl` + `/taa.cs.hlsl`；utils::get_res + uniform + read_texture + write_texture + dispatch_2d | O | render-loop\decomp\decomp-0x140a3b2f0.txt（串：TAA、/taa.cs.hlsl）；wp5 compute-passes §3 |
| `country_core::systems::render_loop::compute_passes::taa::calculate_tgsr` | 0x140a3ba60 | 63 | TGSR（超分升采样）：`/tgsr.cs.hlsl:cs_main`（+`/tgsr_reproject.cs.hlsl`、`/tgsr_blur_lum.cs.hlsl`，wp5 O）；Temporal::split_pair/into_inner + Option::is_some_and；min_subgroup_size；dispatch_2d | O | render-loop\decomp\decomp-0x140a3ba60.txt；wp5 compute-passes §3 |

## 关键事实

- 选择：`param_6[0xab]==0 → TAA，!=0 → TGSR`（frame-wiring §3）。
- `TgsrMain` 实现 AsComputeShaderSource（O）。
