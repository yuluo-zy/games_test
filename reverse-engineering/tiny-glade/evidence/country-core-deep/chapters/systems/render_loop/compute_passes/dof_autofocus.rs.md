# systems/render_loop/compute_passes/dof_autofocus.rs

> 文件作用：DoF 自动对焦：`calculate_dof_autofocus`（W32，按主相机焦点/窗口宽高比计算对焦参数写回 SSBO）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::dof_autofocus::calculate_dof_autofocus` | 0x140a17880 | 59 | W32 DoF 自动对焦：串证据 `Mouse Depth Write to SSBO/dof/autofocus.cs.hlsl` + `/dof/autofocus.cs.hlsl` + `Delete`；MainCamera::focus + WindowSize::aspect_ratio/as_vec2 计算 focus 参数 → uniform + read_texture + dispatch_2d；写 `DofFocusReadbackSsbo`/`DofAutofocusSsbo` | O | render-loop\decomp\decomp-0x140a17880.txt（串：Mouse Depth Write to SSBO、/dof/autofocus.cs.hlsl）；wp5 compute-passes §4 |

## 关键事实

- 结构体（U 布局）：`DofAutofocusSsbo`（写；读于 dof_prepare）、`DofFocusReadbackSsbo`。
- 帧序 W32（frame-wiring，早于 W49/W51 DoF）。
