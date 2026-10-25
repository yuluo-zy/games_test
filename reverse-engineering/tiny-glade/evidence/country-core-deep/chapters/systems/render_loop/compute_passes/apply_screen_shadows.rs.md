# systems/render_loop/compute_passes/apply_screen_shadows.rs

> 文件作用：屏幕空间阴影应用：`apply_screen_shadows`（W42，应用屏幕阴影到光照）、`compute_shadow_mask_mean`（W47，阴影掩码均值统计回读）。

函数总计 2：业务 2 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::apply_screen_shadows::apply_screen_shadows` | 0x140a39e70 | 20 | W42 应用屏幕阴影：pass "Screen shadows temporal" + `/apply_screen_shadows.cs.hlsl`；uniform + bind_shadowmap + read_texture + write_texture + dispatch_2d；写 shadow_mask_histogram | O | render-loop\decomp\decomp-0x140a39e70.txt（串：Screen shadows temporal、/apply_screen_shadows.cs.hlsl、Shadow mask mean、/shadow_mask_mean.cs.hlsl）；wp5 compute-passes §6 |
| `country_core::systems::render_loop::compute_passes::apply_screen_shadows::compute_shadow_mask_mean` | 0x140a3a6f0 | 58 | W47 阴影掩码均值：pass "Shadow mask mean" + `/shadow_mask_mean.cs.hlsl`；dispatch_1d；读 `ShadowMaskHistogramSsbo` → 写 `ShadowMaskMeanSsbo`（final_post 消费） | O | render-loop\decomp\decomp-0x140a3a6f0.txt；wp5 compute-passes §6 |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### 1. `apply_screen_shadows`（0x140a39e70，W42）—— 时域复用链
处理链（O，B_c2 §2）：`Screen shadows temporal` 纹理对（**R8_UNORM**）→ `history_valid` 来自对结果字节（复用历史）→ **9 个绑定 + 直方图**（写 shadow_mask_histogram）→ 预设 0x11 shadow_map + bind_shadowmap → `dispatch_2d`。
- 数据流：screen shadows 累积 → histogram（`ShadowMaskHistogramSsbo` u32[256]）→ `compute_shadow_mask_mean`（W47）。

### 2. `compute_shadow_mask_mean`（0x140a3a6f0，W47）
pass "Shadow mask mean" + `/shadow_mask_mean.cs.hlsl`；**256 线程 1D**（dispatch_1d）；读 `ShadowMaskHistogramSsbo` → 写 `ShadowMaskMeanSsbo`（1×f32）→ **final_post 消费均值**（`FinalPostEnv` 的 `ShadowMaskMean` 缓冲读取，d_gap §4）。
- 帧序：W42 → …（GI/deferred/W44..）→ W47。

## 关键事实

- 结构体（U 布局）：`ShadowMaskHistogramSsbo`、`ShadowMaskMeanSsbo`（final_post 消费均值）。
- 帧序：W42 → … → W47（frame-wiring）。
