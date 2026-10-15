# systems/onboarding 任务清单

## 元信息

| 项 | 值 |
|---|---|
| B 原路径 | `crates/systems/onboarding` |
| package | `system-onboarding` |
| 规模 | 20 文件 / 66 本 ns 函数 / 4 闭包 / **48 业务函数** |
| 难度 | 中（数学简单，但系统数多、零证据） |
| 批次 | 2 |
| 上游依赖 | `country-core`、`toolbar`、`lang-lib`、`rhapsody`、`glam`、`utils` |
| 复现工程现状 | MODULE_PROGRESS：**未逐项评估**，无本机源码；仅索引 |

## 任务表（按 B src 层级）

### 编排核心

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-ONBOARD-01 | `src/hint_orchestrator.rs` | 恢复 `hint_orchestrator` 与 `BindingHintDisplay::{alternate, adjacent, together, filter_to_mode_impl}`：提示编排与按键提示组合逻辑。 | 输入映射 | 无 | 原入口对照 + 组合规则与原一致 | 未开始 |
| UM-ONBOARD-02 | `src/hint_display_system.rs` | 恢复显示系统：`hint_animation_system`、`hint_deletion_{system,cooldown_system}`、`hint_audio`、`hint_highlight_shape`、`hint_position_update`、`hint_draw`、`render_hints`、`progress_bar_update_system`、`BindingHintDisplay::{resolve_steam_impl, cleanup_impl}`。 | 渲染/UI | 无 | 原入口对照 | 未开始 |
| UM-ONBOARD-03 | `src/exponential_back_off.rs` | 恢复 `exponential_back_off`：提示重试退避。 | — | 无 | 原指令样例 | 未开始 |
| UM-ONBOARD-04 | `src/hints_startup.rs` | 恢复 `hints_startup`：提示系统启动注册。 | 全部提示 | 无 | 原注册顺序对照 | 未开始 |
| UM-ONBOARD-05 | `src/reset_hints.rs` | 恢复 `reset_hints`：提示重置。 | — | 无 | 原指令样例 | 未开始 |
| UM-ONBOARD-06 | `src/tutorial_completion.rs` | 恢复 `tutorial_completion`：教程完成判定。 | 全部提示 | 无 | 原入口对照 | 未开始 |

### 各提示（hint_*）— 条件 + 内容

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-ONBOARD-07 | `src/hint_camera_controls.rs` | 恢复 `camera_pan_hint`、`camera_rotate_hint`、`camera_zoom_hint`、`hint_camera_enable_condition`、`hint_camera_display_condition`。 | 相机 | 无 | 原指令样例 | 未开始 |
| UM-ONBOARD-08 | `src/hint_camera_progress.rs` | 恢复 `hint_camera_progress_update`、`hint_camera_progress_reset`。 | 相机 | 无 | 原指令样例 | 未开始 |
| UM-ONBOARD-09 | `src/hint_focused_edit.rs` | 恢复 `hint_focused_edit_condition`、`hint_focused_edit_hint`。 | 聚焦编辑 | 无 | 原指令样例 | 未开始 |
| UM-ONBOARD-10 | `src/hint_clutter_focus_edit.rs` | 恢复 `hint_clutter_focused_edit_{condition,hint}`、`make_focus_or_camera_binding_hint`。 | clutter | 无 | 原指令样例 | 未开始 |
| UM-ONBOARD-11 | `src/hint_roof_shape.rs` | 恢复 `hint_roof_shape`、`make_lbm_drag_binding_hint`。 | 屋顶 | 无 | 原指令样例 | 未开始 |
| UM-ONBOARD-12 | `src/hint_stairs.rs` | 恢复 `hint_stairs`。 | 楼梯 | 无 | 原指令样例 | 未开始 |
| UM-ONBOARD-13 | `src/hint_terrain.rs` | 恢复 `hint_terrain_layering`。 | 地形 | 无 | 原指令样例 | 未开始 |
| UM-ONBOARD-14 | `src/hint_terrain_focus.rs` | 恢复 `hint_terrain_focus`。 | 地形 | 无 | 原指令样例 | 未开始 |
| UM-ONBOARD-15 | `src/hint_terrain_steepness.rs` | 恢复 `hint_terrain_steepness`。 | 地形 | 无 | 原指令样例 | 未开始 |
| UM-ONBOARD-16 | `src/hint_undo.rs` | 恢复 `hint_undo`。 | 历史 | 无 | 原指令样例 | 未开始 |
| UM-ONBOARD-17 | `src/hint_unlink_windows.rs` | 恢复 `hint_unlink_windows`。 | decorator | 无 | 原指令样例 | 未开始 |
| UM-ONBOARD-18 | `src/hint_deselect_tool.rs` | 恢复 `hint_deselect_tool`。 | 工具状态 | 无 | 原指令样例 | 未开始 |
| UM-ONBOARD-19 | `src/hint_disable_snapping.rs` | 恢复 `hint_disable_snapping`。 | 吸附 | 无 | 原指令样例 | 未开始 |

### 插件

| ID | B 源文件 | 任务内容（关键函数） | 依赖 | 现状/证据 | 验收 | 状态 |
|---|---|---|---|---|---|---|
| UM-ONBOARD-20 | `src/lib.rs` | 恢复 `add_onboarding`、`prompt_rollercoaster`。 | 全部上面 | 仅索引 | 与原注册顺序对照 | 未开始 |

## 卡点

- 提示「条件」多依赖其他系统的状态（相机/工具/吸附/地形）；这些状态在 core 或其他上层模块，需用冻结接口。
- `hint_audio` 属音频跨切面。
- 按键提示依赖输入映射表（tiny-glade/输入层），需确认映射来源。

## 验收标准

- 每提示的 enable/display 条件 ≥ 原指令样例；编排组合规则与原一致；注册顺序一致。
