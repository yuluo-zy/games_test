# systems/render_loop/compute_passes/taa.rs

> 文件作用：时域抗锯齿：`calculate_taa`（W52，TAA 时域积累）、`calculate_tgsr`（TGSR 升采样超分，替代 TAA）。

函数总计 2：业务 2 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::taa::calculate_taa` | 0x140a3b2f0 | 28 | W52 TAA：串证据 `TAA/taa.cs.hlsl` + `/taa.cs.hlsl`；utils::get_res + uniform + read_texture + write_texture + dispatch_2d | O | render-loop\decomp\decomp-0x140a3b2f0.txt（串：TAA、/taa.cs.hlsl）；wp5 compute-passes §3 |
| `country_core::systems::render_loop::compute_passes::taa::calculate_tgsr` | 0x140a3ba60 | 63 | TGSR（超分升采样）：`/tgsr.cs.hlsl:cs_main`（+`/tgsr_reproject.cs.hlsl`、`/tgsr_blur_lum.cs.hlsl`，wp5 O）；Temporal::split_pair/into_inner + Option::is_some_and；min_subgroup_size；dispatch_2d | O | render-loop\decomp\decomp-0x140a3ba60.txt；wp5 compute-passes §3 |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### 1. `calculate_taa`（0x140a3b2f0，W52）
处理链（O，B_c1 §2）：`"TAA"`，`history_valid = valid ? !G : 0`（历史可用标志），5 个纹理访问序，`bypass_neighborhood_clamping`、`sharpen_amount` uniform，`dispatch_2d(history_output extent)`。

### 2. `calculate_tgsr`（0x140a3ba60）
处理链（O，B_c1 §2）：`tgsr_aux` temporal 对（Err → **Result::unwrap panic**）→ 3 个 RG 纹理（0x4c/0x53/9）→ **`TGSR blur lum` → `TGSR reproject` → `TGSR`（10 个纹理访问）**；`min_subgroup_size`（`wave32 = is_some_and(<0x21)`）；dispatch_2d。
- `TgsrMain{wave32, sharpen}` 的 `PERM_wave32`/`PERM_sharpen` defines 经 `pipeline_defines()`（B_i_api §3.3）；source_hash = `calculate_hash(&("uri:entry", &self))`。
- **选择**：`param_6[0xab]==0 → TAA，!=0 → TGSR`（互斥，render_world 章）；TGSR 比 taa 多一个深度参数（B_s_h）。
- 历史对：`taa_history`（尺寸取 param_8.desc extent）/`tgsr_history`（尺寸取分辨率资源 +8），均 `new_2d(..,0x7a).sampled().storage()`；输出 "taa_output"（0x7a 无用法位）。

## 关键事实

- 选择：`param_6[0xab]==0 → TAA，!=0 → TGSR`（frame-wiring §3）。
- `TgsrMain` 实现 AsComputeShaderSource（O）。
