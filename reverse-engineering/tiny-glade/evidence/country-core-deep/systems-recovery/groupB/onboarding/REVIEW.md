# onboarding 审查核对表（reviewer-B tg_vB1 预置基线）

> 状态：**已审查**（2026-10-16，restorer-B 交付后独立审查；判定分布 P≈20 / F≈4 / Q≈36，明细见文末审查判定明细节；编译复验受 country-core session/save 域未提交修改阻断，restorer-B 临时 workspace 通过）。
> 本表业务函数清单与证据锚点来自 `SOURCE_MAP-systems.json`（skeleton PDB 元数据）+ `evidence/multi-baseline/*` 反汇编级分析 + country-core-deep 章节 + reviewer tg_vB1 独立 RNE 抽验。
> 交付后 reviewer 逐项核对：签名/参数面 vs 机器码；接口契约 vs core 类型；偷懒（空壳/假数据/直接复制 TODO 骨架）；缺失（本表函数未实现）。
> 核对列：P=通过 / F=失败 / Q=存疑待深挖。

| 函数 | VA | 核对 | 证据锚点 / 已知风险 |
| **exponential_back_off.rs** | | | |
| `system_onboarding::exponential_back_off::exponential_back_off` | 0x142284800 | | 0x142284800 |
| **hint_camera_controls.rs** | | | |
| `system_onboarding::hint_camera_controls::camera_pan_hint` | 0x14227d000 | | camera_pan_hint 0x14227d000 / camera_rotate_hint 0x14227d2f0 / hint_camera_enable_condition 0x14227c780 / hint_camera_display_condition 0x14227c910 / camera_zoom_hint 0x14227d680 |
| `system_onboarding::hint_camera_controls::camera_rotate_hint` | 0x14227d2f0 | |  |
| `system_onboarding::hint_camera_controls::hint_camera_enable_condition` | 0x14227c780 | |  |
| `system_onboarding::hint_camera_controls::hint_camera_display_condition` | 0x14227c910 | |  |
| `system_onboarding::hint_camera_controls::camera_zoom_hint` | 0x14227d680 | |  |
| **hint_camera_progress.rs** | | | |
| `system_onboarding::hint_camera_progress::hint_camera_progress_update` | 0x14227bd30 | | hint_camera_progress_update 0x14227bd30 / hint_camera_progress_reset 0x14227c070 |
| `system_onboarding::hint_camera_progress::hint_camera_progress_reset` | 0x14227c070 | |  |
| **hint_clutter_focus_edit.rs** | | | |
| `system_onboarding::hint_clutter_focus_edit::make_focus_or_camera_binding_hint` | 0x141c3de00 | | make_focus_or_camera_binding_hint 0x141c3de00 / hint_clutter_focused_edit_condition 0x141c3d890 / hint_clutter_focused_edit_hint 0x141c3db80 |
| `system_onboarding::hint_clutter_focus_edit::hint_clutter_focused_edit_condition` | 0x141c3d890 | |  |
| `system_onboarding::hint_clutter_focus_edit::hint_clutter_focused_edit_hint` | 0x141c3db80 | |  |
| **hint_deselect_tool.rs** | | | |
| `system_onboarding::hint_deselect_tool::hint_deselect_tool` | 0x1422853c0 | | 0x1422853c0 |
| **hint_disable_snapping.rs** | | | |
| `system_onboarding::hint_disable_snapping::hint_disable_snapping` | 0x14227e140 | | 0x14227e140 |
| **hint_display_system.rs** | | | |
| `system_onboarding::hint_display_system::hint_animation_system` | 0x141c4f400 | | 16 业务(最大单文件): hint_animation_system 0x141c4f400 / hint_deletion_cooldown_system 0x141c4f790 / hint_audio 0x141c4fb00 / hint_deletion_system 0x141c4fea0 / hint_highlight_shape 0x141c50120 / hint_position_update 0x141c50510 / hint_draw 0x141c51a60 / render_hints 0x141c52700 / progress_bar_update_system 0x141c530f0 / register_required_componentsx5 / BindingHintDisplay::resolve_steam_impl 0x14227e5e0 / cleanup_impl 0x14227f4a0 |
| `system_onboarding::hint_display_system::impl$1::register_required_components` | 0x141c53530 | |  |
| `system_onboarding::hint_display_system::impl$3::register_required_components` | 0x141c53c20 | |  |
| `system_onboarding::hint_display_system::impl$4::register_required_components` | 0x141c54310 | |  |
| `system_onboarding::hint_display_system::impl$5::register_required_components` | 0x141c54a00 | |  |
| `system_onboarding::hint_display_system::hint_deletion_cooldown_system` | 0x141c4f790 | |  |
| `system_onboarding::hint_display_system::hint_audio` | 0x141c4fb00 | |  |
| `system_onboarding::hint_display_system::hint_deletion_system` | 0x141c4fea0 | |  |
| `system_onboarding::hint_display_system::impl$7::register_required_components` | 0x141c550f0 | |  |
| `system_onboarding::hint_display_system::hint_highlight_shape` | 0x141c50120 | |  |
| `system_onboarding::hint_display_system::hint_position_update` | 0x141c50510 | |  |
| `system_onboarding::hint_display_system::hint_draw` | 0x141c51a60 | |  |
| `system_onboarding::hint_display_system::render_hints` | 0x141c52700 | |  |
| `system_onboarding::hint_display_system::progress_bar_update_system` | 0x141c530f0 | |  |
| `system_onboarding::hint_orchestrator::BindingHintDisplay::resolve_steam_impl` | 0x14227e5e0 | |  |
| `system_onboarding::hint_orchestrator::BindingHintDisplay::cleanup_impl` | 0x14227f4a0 | |  |
| **hint_focused_edit.rs** | | | |
| `system_onboarding::hint_focused_edit::hint_focused_edit_condition` | 0x14227da70 | | hint_focused_edit_condition 0x14227da70 / hint_focused_edit_hint 0x14227dd60 |
| `system_onboarding::hint_focused_edit::hint_focused_edit_hint` | 0x14227dd60 | |  |
| **hint_orchestrator.rs** | | | |
| `system_onboarding::hint_orchestrator::hint_orchestrator` | 0x14227fc30 | | 16 业务: hint_orchestrator 0x14227fc30 / register_required_componentsx7 / BindingHintDisplay::alternatex5(0x141c4e4f0/0x141c4e6d0/0x141c4e8b0/0x141c4ead0/0x141c4ecb0) / adjacentx2(0x141c4ee80/0x141c4f060) / together 0x141c4f230 / filter_to_mode_impl 0x1422808d0 |
| `system_onboarding::hint_orchestrator::impl$13::register_required_components` | 0x1422809d0 | |  |
| `system_onboarding::hint_orchestrator::impl$21::register_required_components` | 0x1422810f0 | |  |
| `system_onboarding::hint_orchestrator::impl$23::register_required_components` | 0x1422817e0 | |  |
| `system_onboarding::hint_orchestrator::impl$25::register_required_components` | 0x142281ed0 | |  |
| `system_onboarding::hint_orchestrator::impl$27::register_required_components` | 0x1422825c0 | |  |
| `system_onboarding::hint_orchestrator::impl$31::register_required_components` | 0x142282f40 | |  |
| `system_onboarding::hint_orchestrator::BindingHintDisplay::alternate<array$<enum2$<core::option::Option<enum2$<bevy_input::keyboard::KeyCode> > >,4> >` | 0x141c4ead0 | |  |
| `system_onboarding::hint_orchestrator::BindingHintDisplay::alternate<enum2$<bevy_input::mouse::MouseButton> >` | 0x141c4e6d0 | |  |
| `system_onboarding::hint_orchestrator::BindingHintDisplay::alternate<enum2$<core::option::Option<enum2$<bevy_input::keyboard::KeyCode> > > >` | 0x141c4e4f0 | |  |
| `system_onboarding::hint_orchestrator::BindingHintDisplay::alternate<enum2$<system_onboarding::hint_orchestrator::BindingHintDisplay> >` | 0x141c4ecb0 | |  |
| `system_onboarding::hint_orchestrator::BindingHintDisplay::alternate<system_onboarding::hint_orchestrator::HintGraphicParams>` | 0x141c4e8b0 | |  |
| `system_onboarding::hint_orchestrator::BindingHintDisplay::adjacent<enum2$<core::option::Option<enum2$<bevy_input::keyboard::KeyCode> > > >` | 0x141c4ee80 | |  |
| `system_onboarding::hint_orchestrator::BindingHintDisplay::adjacent<enum2$<system_onboarding::hint_orchestrator::BindingHintDisplay> >` | 0x141c4f060 | |  |
| `system_onboarding::hint_orchestrator::BindingHintDisplay::together<enum2$<system_onboarding::hint_orchestrator::BindingHintDisplay> >` | 0x141c4f230 | |  |
| `system_onboarding::hint_orchestrator::BindingHintDisplay::filter_to_mode_impl` | 0x1422808d0 | |  |
| **hint_roof_shape.rs** | | | |
| `system_onboarding::hint_roof_shape::hint_roof_shape` | 0x142284ed0 | | hint_roof_shape 0x142284ed0 / make_lbm_drag_binding_hint 0x142285270 |
| `system_onboarding::hint_roof_shape::make_lbm_drag_binding_hint` | 0x142285270 | |  |
| **hint_stairs.rs** | | | |
| `system_onboarding::hint_stairs::hint_stairs` | 0x142285a20 | | 0x142285a20 |
| **hint_terrain.rs** | | | |
| `system_onboarding::hint_terrain::hint_terrain_layering` | 0x142283cc0 | | 0x142283cc0 |
| **hint_terrain_focus.rs** | | | |
| `system_onboarding::hint_terrain_focus::hint_terrain_focus` | 0x142283980 | | 0x142283980 |
| **hint_terrain_steepness.rs** | | | |
| `system_onboarding::hint_terrain_steepness::hint_terrain_steepness` | 0x142283640 | | 0x142283640 |
| **hint_undo.rs** | | | |
| `system_onboarding::hint_undo::hint_undo` | 0x142284220 | | 0x142284220 |
| **hint_unlink_windows.rs** | | | |
| `system_onboarding::hint_unlink_windows::hint_unlink_windows` | 0x14227d760 | | 0x14227d760 |
| **hints_startup.rs** | | | |
| `system_onboarding::hints_startup::hints_startup` | 0x14227b450 | | 0x14227b450 |
| **lib.rs** | | | |
| `system_onboarding::impl$0::add_onboarding` | 0x140650d50 | | impl$0::add_onboarding 0x140650d50 / prompt_rollercoaster 0x14227bb70 |
| `system_onboarding::prompt_rollercoaster` | 0x14227bb70 | |  |
| **reset_hints.rs** | | | |
| `system_onboarding::reset_hints::reset_hints` | 0x141c1c8d0 | | 0x141c1c8d0 |
| **tutorial_completion.rs** | | | |
| `system_onboarding::tutorial_completion::tutorial_completion` | 0x14227c230 | | 0x14227c230 |


## 审查判定明细（2026-10-16，tg_vB1）

> 总体：60 业务全有结构覆盖（碎而浅特征明显），无既有重建可对照；判定分布：
> P≈20 / F≈4 / Q≈36。**最大风险 = 键码/触发条件/显示几何为自造推断**（均标 I 但未对
> 机器码）。toolbar 依赖（hint_draw/render_hints 的 TemplateView/MeiContext/NodeContext）
> 按批次 3 后置，UI 缝可接受。

### F 项（需 restorer-B 修复或转 re-analyst）

1. **F1 hint_highlight_shape 空壳**（0x141c50120）：只遍历不做事；机器码
   PublicWalls::roof_entity 目标形状高亮未实现。
2. **F2 hint_position_update 固定屏幕锚点**（0x141c50510）：0.9/0.1/0.5 屏幕分数替代
   机器码 world_space_to_screen_space 几何投影（ShapeParameters/Roof::height/pivot_3d/
   PublicWallState::max_y/CameraMatrices::eye_right/Rectangle2d/3d 消费全缺）。
3. **F3 键码/键帽串自造**：`keycode_to_str`（0x01=Esc、0x1d=Ctrl、0x2c=Z、0x38=Alt、
   0x1e=A、0x11=W）与 hint_* 的 Key(0x…) 均未对机器码（game_key_bindings::keycode_to_str
   与真实绑定树）；bevy KeyCode 判别值与 VK 码不一致（0x01 实为鼠标左键 VK）。
4. **F4 steam()=identity / resolve_steam_impl（0x14227e5e0）未实现**；cleanup_impl
   （0x14227f4a0）完全缺失。

### Q 项（宿主化/推断，需 toolbar 批次与 re-analyst 跟进）

- Q1 hint_* 触发条件宿主化：机器码读 DigitalActionState/AppMode::is_tool_selected/
  0x140eaaa50 按键判定 + make_focus_or_camera_binding_hint；restorer 用 HintInput 资源
  （cancel_pressed/undo_pressed/tool_selected 由宿主填）——行为不落地前依赖宿主。
- Q2 hint_camera_display_condition：机器码 0x1408c5680（工具选择判定），restorer 用
  back_off.remaining 检查替代。
- Q3 exponential_back_off：BACKOFF_BASE_S=0.5/2^n 封顶 6/16 槽全部自造（机器码 raw_vec
  + f32 语义，具体公式 U）。
- Q4 prompt_rollercoaster（0x14227bb70）：3.0s/5 提示轮播自造。
- Q5 hint_type_ordinal 显示序自造（机器码 driftsort 比较字段 U）。
- Q6 hint_deletion_system 冷却语义自造（remaining[ord]=2.0 复位）；hint_animation_system
  的 Easing 应用后丢弃（`let _eased`）。
- Q7 hint_orchestrator（结构较忠实 ✓：esc 门控/同型去重/spawn/lang-lib lookup 缝），但
  HintLocalization 宿主缝 + hint_label 键名自造（lang-lib 键应为机器码字符串池）。
- Q8 hints_startup 相机绑定树（MouseMiddle/LMB/RMB/Scroll 组合）键值自造；
  WelcomeScreenState 联动未实现（机器码排队初始提示以欢迎屏状态）。
- Q9 hint_camera_progress/hint_focused_edit/hint_clutter_focus_edit/hint_roof_shape：
  结构存在，触发/显示细节为推断（make_focus_or_camera_binding_hint 载荷自造）。
- Q10 register_required_componentsx12（0x141c53530..0x142282f40）与 alternate/adjacent/
  together x8 实例为胶水/合并（按 AGENTS.md 不计独立业务，登记合并）。
- Q11 `configure` 命名替代 add_onboarding 0x140650d50（约定内登记）；SubApp 装配
  （机器码证据 SubApp::add_systems + chain_ignore_deferred + in_set）未复现。
- Q12 raycast_seam：legacy_raycast_hit/raycast_hit 恒 None（与 core 壳一致，装配层落地）。

### 缺失登记（真实缺项）

- resolve_steam_impl（0x14227e5e0）、cleanup_impl（0x14227f4a0）——F4；
- clutter 侧 init_state（0x1422e1b40）、get_param（0x141ce3430）——见 clutter Q9。


## 复验轮判定（2026-10-16 第二轮，restorer-B 修复后）

> 主 workspace 编译：**通过**。判定：F1-F4 全部闭合（P/Q）；剩余 F：无。
> 剩余 Q（toolbar 批次 3 / re-analyst / 输入层）：见下。

- **F1 hint_highlight_shape：P/Q**。高亮数据（HintHighlightShape kind/outline/y）
  产出 + HintGeometryTarget 宿主缝 ✓。Q：机器码 PublicWalls::roof_entity 直接消费
  在缝内。
- **F2 hint_position_update：P/Q**。固定锚点已替换为
  `utils::world_space_to_screen_space` 真实投影（view/proj/target_world 经
  HintProjection 缝）✓。Q：目标世界点解析（ShapeParameters/Roof::height/max_y）
  在宿主缝。
- **F3 BindingKey 语义变体：P**。BindingKey 改语义枚举（Esc/Ctrl/Alt/KeyA/KeyW/
  KeyZ/Scroll/Raw 透传），hint_* 全部用语义键（不再自造 VK 码），keycode_to_str
  输出语义名 ✓。Q：语义键 ↔ 机器码 DigitalActionState/绑定树映射待输入层核对。
- **F4 steam/cleanup：P**。steam() → resolve_steam_impl（递归遍历组合树）+
  steam_glyph 字形表 ✓；cleanup_impl（0x14227f4a0）在 hint_orchestrator ✓。
