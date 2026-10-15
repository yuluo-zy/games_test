# systems/render_loop/compute_passes/eval_gi_screen_radiance.rs

> 文件作用：GI 屏幕辐射度求值：`eval_gi_screen_radiance`（W39，GI 候选→屏幕辐射度 + 时域滤波）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::eval_gi_screen_radiance::eval_gi_screen_radiance` | 0x140a44cd0 | 19 | W39 GI 辐射度求值：串证据 `Eval screen radiance/gi/eval_screen_radiance.cs.hlsl` + `Filtered indirect radiance` + `Filtered indirect stats`；`/gi/{eval_screen_radiance, reproject_screen_radiance, blur_screen_radiance}.cs.hlsl`；ShaderRefactoringState::disabled 判定 + read/write/read_write_texture + dispatch_2d | O | render-loop\decomp\decomp-0x140a44cd0.txt（串：Filtered indirect stats、Filtered indirect radiance、Eval screen radiance、/gi/eval_screen_radiance.cs.hlsl）；wp5 compute-passes §12 |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `eval_gi_screen_radiance`（0x140a44cd0，W39）
处理链（O，B_c3 §2）：
1. 辐射度纹理宽 ×3（"Screen indirect radiance" 格式 0x61）；
2. **Eval / Reproject / Filter 三 pass**（`/gi/{eval_screen_radiance, reproject_screen_radiance, blur_screen_radiance}.cs.hlsl`）；
3. `allow_non_determinism = (ShaderRefactoringState disc == 0)`；
4. **两处写访问码 0x37**（i_infra 后 blue_noise 走真实 FromDisk，访问序 [F,T,F]）。
- 数据流：W38 `ray_trace_gi` 候选 → W39 求值 + 时域滤波（reproject）→ "Filtered indirect radiance" → W43 deferred 光照 + W42 screen shadows。

## 关键事实

- 输出 `Filtered indirect radiance`（W38 ray_trace_gi 候选 → 滤波后供 deferred 光照）。
- 帧序 W38→W39（frame-wiring）。
