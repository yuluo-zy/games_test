# system-onboarding RESTORE.md（组 B restorer，2026-10-16）

crate 落点：`tiny-glade/crates/systems/onboarding/`（workspace 已挂载，`cargo check -p system-onboarding` 通过）。
证据基线：B patched EXE（skeleton PDB 元数据：函数名/行号/原生调用目标）+ chapters/resources/onboarding.rs.md。

## 1. 覆盖（SOURCE_MAP 60 业务，全部落位）

| 文件 | 业务函数（VA） | 本 crate 实现 |
|---|---|---|
| lib.rs | impl$0::add_onboarding 0x140650d50、prompt_rollercoaster 0x14227bb70 | configure（Hint 组件族/事件/32 系统链）+ prompt_rollercoaster |
| exponential_back_off.rs | 0x142284800 | 退避计时资源 + trigger/reset（2^n 加倍） |
| hints_startup.rs | 0x14227b450 | 启动提示队列（相机三连） |
| hint_camera_controls.rs | enable 0x14227c780、display 0x14227c910、pan 0x14227d000、rotate 0x14227d2f0、zoom 0x14227d680 | 全 5（组合子绑定树 + run_if 条件） |
| hint_camera_progress.rs | update 0x14227bd30、reset 0x14227c070 | 全 2 |
| hint_clutter_focus_edit.rs | make_focus_or_camera_binding_hint 0x141c3de00、condition 0x141c3d890、hint 0x141c3db80 | 全 3（实现经 hint_focused_edit 共享核心） |
| hint_deselect_tool.rs | 0x1422853c0 | 实现 + HintInput 资源 |
| hint_disable_snapping.rs | 0x14227e140 | 实现 |
| hint_display_system.rs | animation 0x141c4f400、deletion_cooldown 0x141c4f790、audio 0x141c4fb00、deletion 0x141c4fea0、highlight_shape 0x141c50120、position_update 0x141c50510、hint_draw 0x141c51a60、render_hints 0x141c52700、progress_bar 0x141c530f0、register_required_components×6 | 全 16（显示状态机 + 位置/进度 + hint_ui 缝） |
| hint_focused_edit.rs | condition 0x14227da70、hint 0x14227dd60 | 全 2 |
| hint_orchestrator.rs | hint_orchestrator 0x14227fc30、filter_to_mode_impl 0x1422808d0、BindingHintDisplay::alternate/adjacent/together/steam/is_some/filter_to_mode ×13、register×6 | 全 16（组合子代数 + 编排：lang-lib 文本/去重/esc 门控/排序） |
| hint_roof_shape.rs | hint 0x142284ed0、make_lbm_drag_binding_hint 0x142285270 | 全 2 |
| hint_stairs.rs | 0x142285a20 | 实现 |
| hint_terrain.rs | layering 0x142283cc0 | 实现 |
| hint_terrain_focus.rs | 0x142283980 | 实现 |
| hint_terrain_steepness.rs | 0x142283640 | 实现 |
| hint_undo.rs | 0x142284220 | 实现 |
| hint_unlink_windows.rs | 0x14227d760 | 实现 |
| reset_hints.rs | 0x141c1c8d0 | 实现 |
| tutorial_completion.rs | 0x14227c230 | 实现 |

## 2. 关键设计

- **组件/事件契约（PDB BundleInfo 单态化证据）**：提示实体 Bundle =
  (HintLayoutMemo, HintSpring, HintAnimation, HintText, HintType, HintPlacement)；
  事件 DisplayHintRequest / LingerHintRequest；显示查询 7 列
  (Entity, HintSpring, HintAnimation, HintPlacement, HintType, Option<HintText>,
  Option<HintProgressBar>, Option<BindingHintDisplay>)。
- **BindingHintDisplay 组合子代数**（alternate/adjacent/together 单态化实例 ×8 对照）；
  filter_to_mode（InputMode 过滤）。
- **触发条件 = run_if 条件系统**（本 bevy 分叉 tuple 链不支持 bool 返回系统，故
  camera enable/display、focused_edit、clutter_focused_edit 条件经 `.run_if()` 挂载——
  与证据 `add_systems(...).run_if()` 一致）。
- **exponential_back_off**：16 槽退避（2^n 加倍，基础 0.5s），全部 hint 触发门。
- **toolbar 缝**：hint_draw/render_hints 的 TemplateView/MeiContext/NodeContext
  （toolbar 批次 3）以 `HintUiContext` 表达消费面；本 crate 实现载荷准备
  （键帽串 keycode_to_str、lang-lib 文本、展平绑定树）。

## 3. 编译与验证

- `cargo check -p system-onboarding`：通过（3 警告，无错误；temp workspace 验证环境同
  critters 说明）。
- 与 core 交互：WelcomeScreenState（core resources/onboarding.rs 已恢复）、
  EscMenuState::is_open、AppMode::is_tool_selected、DigitalActionState、lang_lib::lookup、
  Localization（HintLocalization 资源包装）。

## 4. 未决（不伪造）

- hint_draw/render_hints 的 toolbar 布局数学（批次 3 接线）；相机进度事件源（宿主相机
  系统写 CameraMoved）；hint_position_update 的目标几何投影（ShapeParameters/Roof 计算
  由宿主 HintGeometryTarget 注入）；DisplayHintRequest 的重复抑制细节（driftsort 键）；
  WelcomeScreenState 欢迎屏流程（core 侧）。

## 5. FIX 交接（reviewer-B 21 条，2026-10-16 第二轮）

| # | 修复 | 落点 |
|---|---|---|
| F1 | hint_highlight_shape 真实：HintGeometryTarget 缝（PublicWalls::roof_entity 目标形状）→ HintHighlights 资源（kind/outline/y） | hint_display_system.rs |
| F2 | hint_position_update 真实投影：utils::world_space_to_screen_space（core 已恢复）+ HintProjection（view/proj/目标世界点）→ 屏幕锚点 + 弹簧 | hint_display_system.rs + lib.rs HintGeometryTarget/HintProjection |
| F3 | 键码/键帽自造移除：BindingKey 改语义变体（Esc/Ctrl/Alt/KeyA/KeyW/KeyZ/Raw），keycode_to_str 按语义名（Raw 透传） | lib.rs + hint_display_system.rs + hint_deselect_tool.rs + hints_startup.rs |
| F4 | steam() = resolve_steam_impl（0x14227e5e0：Steam 字形递归解析）；cleanup_impl（0x14227f4a0：解析为空/超时提示 despawn） | lib.rs + hint_orchestrator.rs |

编译：temp workspace 三 crate 全通过；已同步 `tiny-glade/crates/systems/onboarding`。
