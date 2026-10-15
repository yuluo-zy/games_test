# systems/render_loop/compute_passes/dof_autofocus.rs

> 文件作用：DoF 自动对焦：`calculate_dof_autofocus`（W32，按主相机焦点/窗口宽高比计算对焦参数写回 SSBO）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::dof_autofocus::calculate_dof_autofocus` | 0x140a17880 | 59 | W32 DoF 自动对焦：串证据 `Mouse Depth Write to SSBO/dof/autofocus.cs.hlsl` + `/dof/autofocus.cs.hlsl` + `Delete`；MainCamera::focus + WindowSize::aspect_ratio/as_vec2 计算 focus 参数 → uniform + read_texture + dispatch_2d；写 `DofFocusReadbackSsbo`/`DofAutofocusSsbo` | O | render-loop\decomp\decomp-0x140a17880.txt（串：Mouse Depth Write to SSBO、/dof/autofocus.cs.hlsl）；wp5 compute-passes §4 |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `calculate_dof_autofocus`（0x140a17880，W32）
处理链（O，B_c1 §2）：
1. 5 个 ECS 资源（MainCamera 焦点 + WindowSize）；
2. **`point_uv` 三分支**：mode 2 鼠标 / 3 / 其余；门控 `mode<2 && view_kind==0x13`；
3. **7 个 uniform 顺序**（focus 参数 = 相机焦点 × 宽高比计算）；
4. 2 SSBO + `depth_tex`（0x19）；pass 名 `"Mouse Depth Write to SSBO"`；
5. `dispatch_2d(8,8)`（`.rdata 0x142ac7e70 = {1,8,8,1}` dispatch 描述符，c1 纠错"全零向量"误读）。
- 数据流：`DofAutofocusSsbo`（写）→ `dof_prepare`（W49 读）→ CoC。
- 帧序 W32（早于 W49/W51 DoF）。`use_dual_focus` 语义 I（HLSL 字段名）。

## 关键事实

- 结构体（U 布局）：`DofAutofocusSsbo`（写；读于 dof_prepare）、`DofFocusReadbackSsbo`。
- 帧序 W32（frame-wiring，早于 W49/W51 DoF）。
