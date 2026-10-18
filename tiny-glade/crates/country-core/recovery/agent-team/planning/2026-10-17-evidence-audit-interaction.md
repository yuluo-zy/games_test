# 2026-10-17 证据等级逐函数审计 — interaction 四模块 + integration 轮 + tiny-glade

> 角色：auditor-interaction（只读汇总；未改 src、未反编译、未改库）。
> 范围：
> - 上层 interaction 四模块：**clutter / onboarding / wall-constructor / toolbar**
> - integration 轮：coregap-1/2、purelogic、external-dep、unlock、consume、remaining、tests
> - tiny-glade 批次 4：A（启动/主循环/配置）、B（窗口/崩溃/GUI/截图）
>
> 证据来源：`reports/interaction/{clutter,onboarding,wall-constructor,toolbar}/{RESTORE,REVIEW}.md`、
> `reports/{wave2,wave2b,wave3,wave4,gap,consume,remaining,unlock,integration,tinyglade,tests}/**`。
> 时间线以 mtime 为准；**后轮报告覆盖前轮**，但只有独立 REVIEW 的 verdict 才计入 C。
> 关键时间线：interaction（03:28–05:14）→ wave2（08:27）→ wave2b（09:07）→ wave3（11:27–11:44）
> → gap（21:50）→ unlock（21:50–22:51）→ consume（22:29–22:58）→ remaining（22:28–22:36）
> → wave4 reviews（22:26–22:31）→ tests（22:13）→ tinyglade（22:38–22:45）。

## 0. 口径与证据等级

| 等级 | 定义 | 本审计记号 |
|---|---|---|
| 1 未验证 | 无实现/空壳/todo | **D** |
| 2 restorer 自判 | 只有 RESTORE 声明判等/重建，无对应 REVIEW verdict | **A2** |
| 3 独立复核判等 | REVIEW 明确对该函数给出 判等 / fix_verified | **C** |
| 4 数值断言 | `tests/numeric-assertions` 或模块单测直接断言（附测试名） | **A4**（无独立复核时） |
| 5 差分案例 | 原指令/RNE/冻结用例（G04 等） | 计入 **C** 的最高证据 |

判定规则：
- **A = 判等但无独立复核**。本报告把 A 分为 **A2（仅 restorer 自判）** 与 **A4（有单测/数值断言但无独立 REVIEW）**。
- **B = 重建/部分实现但未判等**（含 REVIEW 判“需修”且修复后未见独立复验）。
- **C = 有独立 REVIEW 判等/fix_verified**（含 G04 差分、RNE 机器码复核）。
- **D = 未实现/空壳/todo/core-type-gap 阻塞**。
- 胶水（derive/Bevy 注册/纯转发）与归属他 crate 的条目单列，不计入业务函数。

---

## 1. 结论摘要（四模块为主口径）

| 模块 | 业务函数 | A2 | A4 | B | C | D | A 合计占比 | C 占比 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| clutter | 31 | 0 | 0 | 1 | 8 | 22 | 0% | 25.8% |
| onboarding | 54 | 0 | 1 | 25 | 22 | 6 | 1.9% | 40.7% |
| wall-constructor | 97 | 0 | 0 | 4 | 12 | 81 | 0% | 12.4% |
| toolbar | 121 | 0 | 8 | 12 | 32 | 69 | 6.6% | 26.4% |
| **四模块合计** | **303** | **0** | **9** | **42** | **74** | **178** | **3.0%** | **24.4%** |

> **2026-10-17/18 更新（reviewer-B-interaction，§14）**：clutter/onboarding/wall-constructor
> 的 30 项 B 已独立复核（fresh 反编译→逐语句）：判等 5（升 C，均附限定）/ 需修 25 / 未实现 0。
> 三模块 B/C 由 30/42 调整为 **25/47**（onboarding 25/22→22/25、wall 4/12→2/14、clutter 不变）。
> 上表为复核前口径；四模块合计随之变为 B 37 / C 79（toolbar 12/32 不变）。详见 §14。

> 关键结论：四模块内**没有仅靠 restorer 自判（A2）的判等函数**——所有“判等”宣称要么已被独立 REVIEW 复核（C），
> 要么至少有模块单测/数值断言（A4）。真正缺独立复核的是 **A4 的 9 项**（onboarding 1 + toolbar 8）。
> 但 B（42）和 D（178）仍占 72.6%，与上层进度文档“行级判等 24.8%”口径一致。

**integration + tiny-glade 另列口径**（含类型/API/合同条目，且与四模块有重叠，不并入上表）：

| 轮次 | 条目/函数 | A2 | A4 | B | C | D/合同 |
|---|---:|---:|---:|---:|---:|---:|
| coregap-1 | 10 | 0 | 0 | 2 | 8 | 0（+1 冲突跳过） |
| coregap-2 | 12 | 0 | 0 | 2 | 10 | 0（+1 冲突跳过） |
| purelogic（21+5） | 26 | 1 | 0 | 6 | 4 | 15（含 146 函数族另计） |
| unlock（7 线） | 7 线 / 约 40 条目 | 8 | 12 | 6 | 1 | 13（含 2 线纯合同） |
| consume（6 目录） | 约 25 函数/子项 | 2 | 1 | 9 | 9 | 4 |
| remaining（2 目录） | 约 14 | 0 | 0 | 5 | 4 | 5 |
| tests | 31 测试 / 188 断言 | — | 覆盖 16 函数 | — | — | — |
| tiny-glade A | 55 | 0 | 0 | 1 | 5 | 44 |
| tiny-glade B | 73 | 0 | 0 | 32 | 6 | 26 |

> tiny-glade A/B 在审计时**完全没有独立 REVIEW**：A 的 6 个判等、B 的 18 个判等全部是 restorer 自判。
> **2026-10-17 已由 reviewer-A-tinyglade 独立复核这 24 项**（fresh 反编译→独立复述→逐语句判等）：
> 判等 11 项升 **C**（A 5 + B 6），需修 13 项仍计 **B**（A 1 + B 12）；未实现 0。
> 报告：`reports/tinyglade/reviews/A/REVIEW.md`；FIX-A1..A13 见其 §2。上表为复核后口径。

---

## 2. A 清单 — 判等但无独立复核

### 2.1 A2（最高证据 = restorer 自判，无单测/数值断言）

| # | 模块/轮次 | 函数/条目 | VA | 来源文件 | 自判内容 |
|---|---|---|---|---|---|
| 1 | purelogic | `FishSpawnLocations::new` | 0x140f909f0 | `reports/integration/purelogic/RESTORE.md` | 公式/行列序判等；无 REVIEW |
| 2 | purelogic/water | `draw_water` 注册（add_water_cpu_systems + register_water_events） | — | 同上 §C | 判等/已验证；自足接线，无独立复核 |
| 3 | unlock/debugline | `DebugColor` 手写 `Debug` / `as_array` | 0x140b05c70 | `reports/unlock/debugline-signifier/RESTORE.md` | 六分支跳表判等；无 REVIEW |
| 4 | unlock/debugline | `SignifierFactory::new` | 0x14089d8b0 | 同上 | 57 指令判等；consume 仅 restorer 复核 |
| 5 | unlock/debugline | `UiSignifierStream` 闭包承接/唤醒（write/write_factory/wake） | 0x140b2cf90 / 0x140b2cfa0 | 同上 | 重建后自称语义等价；无 REVIEW（gap REVIEW 只覆盖 write_raw 接口面） |
| 6 | consume/debugline | water `draw_water::closure$0` | 0x141ba9700 | `reports/consume/debugline/RESTORE.md` | 85 指令判等；无 REVIEW |
| 7 | consume/debugline | `SignifierEntityBuilder::{position,scale}` | 0x14089dc00 / 0x14089dc60 | 同上 | 逐指令判等；无 REVIEW |
| 8 | unlock/mei-toolbar | `mei_pre_update` 9 参签名 | 0x1406842e0 | `reports/unlock/mei-toolbar/RESTORE.md` | 签名判等；主体未实现 |
| 9 | unlock/mei-toolbar | `mei_post_update` 9 参签名 | 0x1406884c0 | 同上 | 签名判等；主体未实现 |
| 10 | unlock/mei-toolbar | `render_taffy_ui_to_widgets` 13 参接线 | 0x142338990 | 同上 | 签名/接线判等；主体未实现 |
| 11 | tinyglade A | `render` | 0x1404e8010 | `reports/tinyglade/A/RESTORE.md` | 委托 country-core 判等；无 REVIEW |
| 12 | tinyglade A | `render_world` | 0x1404ec110 | 同上 | 同上 |
| 13 | tinyglade A | `handle_screenshot_error` | 0x1404ebfb0 | 同上 | 同上 |
| 14 | tinyglade B | `EguiBackend::end_frame` | 0x1400dd910 | `reports/tinyglade/B/RESTORE.md` | 判等；无 REVIEW |
| 15 | tinyglade B | `calculate_max_resolution_scale` | 0x1404f7f20 | 同上 | 判等；常量 .rdata 直读 |
| 16 | tinyglade B | `NvAftermathDelegate::default` | 0x140244460 | 同上 | 判等 |
| 17 | tinyglade B | `NvAftermathDelegate::shader_debug_info` | 0x140245340 | 同上 | 判等 |
| 18 | tinyglade B | `crash_payload` Serialize 字段面（6 单态） | 0x14003e250 / e7c0 / eb80 / ec80 / e350 / eda0 | 同上 | 字段名 .rdata 直读判等 |
| 19 | tinyglade B | `ClientHandle::send_message` | 0x14015ac60 | 同上 | 判等 |
| 20 | tinyglade B | `server::on_message` | 0x1402ae1f0 | 同上 | 判等 |
| 21 | tinyglade B | `dump_log_file` | 0x1403acc70 | 同上 | 判等 |
| 22 | tinyglade B | `dump_settings_file` | 0x1403ae050 | 同上 | 判等 |
| 23 | tinyglade B | `dump_gpu_crash_dump_file` | 0x1403ae570 | 同上 | 判等 |
| 24 | tinyglade B | `dump_rhapsody_log` | 0x1403ae770 | 同上 | 判等 |
| 25 | tinyglade B | `dump_screenshot` | 0x14058a060 | 同上 | 判等 |
| 26 | tinyglade B | `spawn_hang_watchdog_thread` | 0x140365be0 | 同上 | 判等 |
| 27 | tinyglade B | `write_image_with_callback_impl` | 0x1403206a0 | 同上 | 判等 |
| 28 | tinyglade B | `CoasterAudioInstance::new` | 0x1403b6880 | 同上 | 判等 |
| 29 | tinyglade B | `confirm_incoming_panic_repro` | 0x1400ed280 | 同上 | 判等 |

**A2 合计 29 项。** 模块分布：tinyglade 19（A 3 + B 16）、unlock 7、consume 2、purelogic 2。

### 2.2 A4（有单测/数值断言，但无独立 REVIEW）

| # | 模块/轮次 | 函数/条目 | VA | 来源文件 | 断言证据 |
|---|---|---|---|---|---|
| 1 | onboarding | `hint_orchestrator` | 0x14227fc30 | `reports/consume/hint/RESTORE.md` | 新增 9 单测（排序/文本/门控/同 id/不同 id/冷却/菜单/Rollercoaster/工具栏） |
| 2 | unlock/hint | `DisplayHintRequest` 布局（0x50B / stride 0x58） | — | `reports/unlock/hint-displayrequest/RESTORE.md` | 4 单测（布局/flags、placement tags、排序稳定、id 18） |
| 3 | unlock/hint | `sort_display_hint_requests`（closure$0 排序键） | 0x1425e5b70 | 同上 | 单测覆盖稳定性 |
| 4 | unlock/hint | `HintType::Rollercoaster = 18` | — | 同上 | 单测 |
| 5 | unlock/debugline | `DebugLine::show` | 0x140b04f70 | `reports/unlock/debugline-signifier/RESTORE.md` | `show_packs_the_record_and_wakes_the_dashed_line_closure` |
| 6–13 | toolbar | 8× `taffy_mut_style` setter：toolbar_env0/2/10/11/12/13、undo_redo_env0/1 | 0x142375c10 / 0x142375eb0 / 0x142375b40 / 0x142375ce0 / 0x142376050 / 0x142376300 / 0x142376230 / 0x142375f80 | `reports/unlock/mei-toolbar/RESTORE.md` | `toolbar_systems::tests` 逐字段断言 |
| 14 | toolbar/unlock | `UiNav` 0x40B 布局 | 0x141e36510 | 同上 | `shared_ui_component_sizes_match_fresh_descriptors` |
| 15 | toolbar/unlock | `HistoryButton` 1B 布局 | 0x141e34b90 | 同上 | 同上 |
| 16 | toolbar/unlock | 20× `register_required_components` 组件定义 | 0x142383770.. | 同上 | 20 项 size 断言 |
| 17 | unlock/nani | `NaniBundle<T>` 0x48B 布局 | 0x141a07f20 / 0x14267a9e0 | `reports/unlock/nani/RESTORE.md` | 2 个布局测试（测试因 ENOSPC 未跑成功） |
| 18 | unlock/nani | `DrawableMeshBundle` 0x40B 布局 | 0x141fe7aa0 / 0x14267c2a0 | 同上 | 同上 |
| 19 | tinyglade A | `log_time_to_first_frame` | 0x1403db200 | `reports/tinyglade/A/RESTORE.md` | 模块单测 |
| 20 | tinyglade A | `FpsLimiter::frame` | 0x140270850 | 同上 | 3 个单测 |
| 21 | tinyglade A | `is_supported_steam_language` 白名单 | 0x140323230 内 | 同上 | 模块单测（15 语言名） |
| 22 | tinyglade B | `winit_to_bevy_keycode` | 0x1400fb850 | `reports/tinyglade/B/RESTORE.md` | `key_code_mapping_matches_b_table` |
| 23 | tinyglade B | `fs_util::copy_folder` | 0x140355800 | 同上 | `copies_nested_tree` |

**A4 合计 23 项。** 模块分布：tinyglade 5、toolbar 11、unlock 5、onboarding 1、consume 1（与 unlock 项有交叠）。
**A 总计 52 项（A2 29 + A4 23）。**

> 注：`hint_orchestrator` 的 9 单测由 consume/hint restorer 编写，非独立 REVIEW；因此计 A4 而非 C。
> `DisplayHintRequest` 布局虽有 4 单测，但单测与实现同源，仍无独立复核。

---

## 3. B 清单 — 重建/部分实现但未判等

### 3.1 四模块

| 模块 | 函数 | VA | 来源 | 说明 |
|---|---|---|---|---|
| clutter | `populate_autoclutter_regions` | 0x1422d6ed0 | wave2 RESTORE + wave2 review | 分派/槽表重建；REVIEW 判“结构判等 + 需修”（权重/算子/active 路径 3 分歧），未复验 |
| onboarding | `resolve_steam_impl` | 0x14227e5e0 | wave3 RESTORE/REVIEW | 重建；Steam 载荷/Graphic U |
| onboarding | `hint_highlight_shape` | 0x141c50120 | wave3 | 重建；+0x11/WallId U |
| onboarding | `hint_position_update` | 0x141c50510 | consume/hint | 重建 case 6/7/8/10；Y 占位 U |
| onboarding | `exponential_back_off` | 0x142284800 | wave3 | 重建；对 A 读者事件类型 U、多事件取首 vs 末 |
| onboarding | `reset_hints` | 0x141c1c8d0 | wave3 | 重建；触发事件类型 U |
| onboarding | `hint_camera_display_condition` | 0x14227c910 | wave3 | 重建；duration 递减/PlayUiSound seam、写面未复现门控 |
| onboarding | `camera_pan_hint` | 0x14227d000 | wave3 | 重建；Steam index/键码 U |
| onboarding | `camera_rotate_hint` | 0x14227d2f0 | wave3 | 重建；同上 |
| onboarding | `camera_zoom_hint` | 0x14227d680 | wave3 | 重建；同上 |
| onboarding | `hint_focused_edit_condition` | 0x14227da70 | wave3 | 重建；焦点字节/PlayUiSound id U |
| onboarding | `hint_focused_edit_hint` | 0x14227dd60 | consume/geo-src | 重建；I/U 7 项登记 |
| onboarding | `hint_clutter_focused_edit_condition` | 0x141c3d890 | wave3 | 重建；同上 |
| onboarding | `hint_clutter_focused_edit_hint` | 0x141c3db80 | consume/geo-src | 重建；I/U 7 项登记 |
| onboarding | `make_focus_or_camera_binding_hint` | 0x141c3de00 | wave3 | 重建；Steam index/Graphic U |
| onboarding | `hint_roof_shape` | 0x142284ed0 | wave3 | 重建（部分）；完成路径已按 REVIEW 修正，展示门控 seam 未触发 |
| onboarding | `make_lbm_drag_binding_hint` | 0x142285270 | wave3 | 重建；Steam index U |
| onboarding | `hint_stairs` | 0x142285a20 | wave3 | 重建（骨架）；reader 映射 U |
| onboarding | `hint_terrain_layering` | 0x142283cc0 | wave3 | 重建；载荷字节 seam |
| onboarding | `hint_terrain_focus` | 0x142283980 | wave3 | 重建；载荷字节 seam |
| onboarding | `hint_terrain_steepness` | 0x142283640 | wave3 | 重建；载荷字节 seam |
| onboarding | `hint_undo` | 0x142284220 | wave3 | 重建；按键/键槽 seam |
| onboarding | `hint_unlink_windows` | 0x14227d760 | wave3 | 重建；载荷/键码 U |
| onboarding | `hint_deselect_tool` | 0x1422853c0 | wave3 | 重建；reader B/C/D/E U |
| onboarding | `hint_disable_snapping` | 0x14227e140 | wave3 | 重建；载荷/键槽/Steam U |
| onboarding | `prompt_rollercoaster` | 0x14227bb70 | wave3 | 重建；Steam index/键码/hint id U |
| wall-constructor | `ArchWalker::new` | 0x14074c2d0 | interaction REVIEW | 字段语义未定（🟡 合理） |
| wall-constructor | `calculate_wall_path_segments_impl` | 0x14122ff90 | interaction RESTORE | 136B→104B materializer，复用冻结链 |
| wall-constructor | `circle_categorize` | 0x14124af60 | remaining + wave4 geometry | 重建；REVIEW FIX-G1（NaN/∞ 语义）已由 restorer 落库，未见独立复验 |
| wall-constructor | `BrickRowSine::new` | 内联 | wave2 | 构造函数，装配 random_f 4 值；无独立复核（若按 trivial 构造函数排除，则 B=3） |
| toolbar | `MeiContext::reuse_or_create_node` | 0x140683ec0 | interaction REVIEW | 重建；身份回收未复刻 |
| toolbar | `process_input_action_handlers` | 0x140687eb0 | interaction REVIEW | 重写后重建（I）；on_input 二/三参状态线程 U |
| toolbar | `MeiContext::load_configs` | 0x140682e70 | interaction REVIEW | 重建；已存 self.configs |
| toolbar | `config::load_font_table` | 0x14068fc60 | interaction REVIEW | 重建（I）；ron Options 标志 U |
| toolbar | `ComputedLayout::center` | 0x14068ea00 | interaction REVIEW | 重建（I）；getter 内联 |
| toolbar | `Node::hover` | 0x140683c90 | interaction REVIEW | 重建（I）；位语义未在原函数验证 |
| toolbar | `Node::has_focus` | 0x140683ca0 | interaction REVIEW | 重建（I） |
| toolbar | `place_childen_impl<XY/YX>` | 0x140fe12a0 / 0x140fe19a0 | wave2b + interaction REVIEW | 核心重建；尾区 justify>8/RON/返回容器 U |
| toolbar | `toolbar_alignment` | 0x142344560 | wave3 + wave3 review | 前置/分派判等；closure_env setter 目标未实现 |
| toolbar | `tod_v2_button` | 0x142342f30 | gap review | 值判等；REVIEW FIX-3（+2 写序）未复验 |
| toolbar | `ui_animation_state_update` | 0x14233b3f0 | wave3 + gap review | 公式判等；分派/符号（enter/leave、appear sign）未实现 |
| toolbar | `link_up_nav` | 0x141e4d880 | unlock/mei-toolbar | 重建（I）；tag=3 语义 U |

### 3.2 integration / tiny-glade（不并入四模块）

| 轮次 | 函数/条目 | 来源 | 说明 |
|---|---|---|---|
| coregap-1 | `SignifierEntityBuilder` 链（new/set_ui_pass/animation_duration/custom_shader/build） | coregap-1 RESTORE/REVIEW | REVIEW 需修 3 项；restorer 已修，无二次复核 |
| coregap-1 | `ReplayBucket.advance_timing` | coregap-1 | REVIEW 位极性反转；已修，无二次复核 |
| coregap-2 | 装饰/窗族事件载荷 5 条（SetDecoratorColorIdCmd 等） | coregap-2 | REVIEW 需修 `_gap4`；已修，无二次复核 |
| coregap-2 | `GameProfile` serde 形状（player_id/game_installation_id） | coregap-2 | REVIEW 需修；已改，无二次复核 |
| purelogic | `DuckSpawnLocations::new` | purelogic | 实现（精确网格+timer），无 REVIEW |
| purelogic | `create_or_restore_decorator_backup` | purelogic + decorator REVIEW | 状态机判等；恢复叶近似 |
| purelogic | `window_lights` | purelogic + decorator/wave3 REVIEW | 常量/灯态/帧末判等；收集主循环未实现 |
| purelogic | `detect_inter_roof_merlons` 等 4 项 | purelogic | dirty_walls 接线完成；几何逐语句未完成 |
| unlock | `geo_src_to_edit_subject` / `_w_deco_clutter` | unlock/geo-src | 实现 3（非判等）；5 单测（A4），无独立 REVIEW |
| unlock | `signifier_create` | unlock/debugline | 重建 + UiState 配置表克隆 GAP |
| consume | `process_tree_cmds`（Insert） | consume/types + wave4 geometry | 重建；T2/T4(is_corrupted)/T5 缺口 |
| consume | `replay_tree` | consume/types + wave4 geometry | 重建；Move-undo 整条路径未实现 |
| consume | `ui_move_tree` | consume/types + wave4 geometry | 重建；候选源代理/T3 |
| consume | `maintain_dormer_tiles` | consume/types + wave4 geometry | REVIEW G3 push→单槽 store 已修，无二次复核 |
| remaining | `handplaced_tree_animation` / `forest_tree_animation` | remaining/misc | 算法判等；GPU 写面未实现 |
| remaining | `update_butterfly_states` | remaining/misc + wave4 misc | REVIEW 需修 4 项，restorer 已修，无二次复核 |
| remaining | `update_wrens` | 同上 | 同上 |
| remaining | `move_critters` | 同上 | 同上 |
| remaining | `critters_avoid_collisions` | 同上 | 同上 |
| tinyglade B | 20 重建函数 | tinyglade/B | 见 §7 |

---

## 4. C 清单 — 有独立复核且判等

### 4.1 clutter（8）

| 函数 | VA | 复核来源 |
|---|---|---|
| `AutoClutterState::add_region` | 0x140ee3fd0 | interaction/clutter REVIEW（判等-附U分歧） |
| `AutoClutterState::mark_dirty` | 0x140ee48c0 | 同上 |
| `AutoClutterRegion::is_equivalent` | L408 | 同上（判等） |
| `ClutterCandidateIndex::{new,get,is_none}` | — | 同上（哨兵 0xffff） |
| `AutoClutterState::remove_stale_regions` | L233 | 同上（内联 retain） |
| `WallAndRoofChanges::dirty_walls` | 0x141cd16b0 | 同上（判等） |
| `add_autoclutter_on_windows` | 0x1422cd7d0 | wave2/reviews/geometry（判等，r2 全函数） |
| `load_clutter_meshes` | 0x1422c0760 | 同上（结构判等，55 closure） |

### 4.2 onboarding（22）

组合子/显示系统（interaction/onboarding REVIEW，19）：
`alternate<Option<KeyCode>>` 0x141c4e4f0、`alternate<MouseButton>` 0x141c4e6d0、
`alternate<HintGraphicParams>` 0x141c4e8b0、`alternate<[Option<KeyCode>;4]>` 0x141c4ead0、
`alternate<BindingHintDisplay>` 0x141c4ecb0、`adjacent<Option<KeyCode>>` 0x141c4ee80、
`adjacent<BindingHintDisplay>` 0x141c4f060、`together<BindingHintDisplay>` 0x141c4f230、
`filter_to_mode_impl` 0x1422808d0、`cleanup_impl` 0x14227f4a0（fix_verified）、
`is_some`、`is_row`（inline）、`hint_animation_system` 0x141c4f400、
`hint_deletion_cooldown_system` 0x141c4f790、`hint_audio` 0x141c4fb00（fix_verified）、
`hint_deletion_system` 0x141c4fea0、`progress_bar_update_system` 0x141c530f0、
`hint_camera_progress_update` 0x14227bd30、`hint_camera_progress_reset` 0x14227c070。

wave3 独立复核（3）：`tutorial_completion` 0x14227c230、`hints_startup` 0x14227b450、
`hint_camera_enable_condition` 0x14227c780。

### 4.3 wall-constructor（12）

`debug_force_update_arches` 0x14003bb80（RNE ret 存根）、
`resolve_hole_overlap` 0x1412411c0（G04 320 例 + REVIEW，残留 split_x 定向用例）、
`trim_rows` 0x14123eef0（G04 200 例）、`above_to_below` 0x14123fab0（G04）、
`below_to_above` 0x14123fb70（G04）、`split_into_random_rows` 0x14123e110（G04 160 例）、
`perturb_row_ends` 0x141240d30（RNE 逐指令 + REVIEW）、`split_horizontally` 0x141240ec0（同上）、
`BrickRowSine::random_f`（宿主 0x140769760 内联，wave2 geometry REVIEW）、
`BrickRowSine::sine_wave_deform`（同上）、
`merge_nearby_proposals`（宿主 0x140178a70 步骤 2，wave2 geometry REVIEW）、
`find_stone_column_start_y` 0x141250720（wave4 geometry REVIEW 判等）。

### 4.4 toolbar（32）

interaction/toolbar REVIEW 复核的 29 项（含 8 FIX fix_verified）：
Node 方法 7（set_sounds 0x140683c20 / ignore_input 0x140683c30 / force_interactive 0x140683c40 /
default_focus 0x140683c50 / with_nav 0x140683c60 / block_input 0x140683cd0 / used 0x140683c80）、
NodeContext 11（hover 0x140683ce0 / interaction 0x140683dd0 / node 0x140681d50 / node_mut 0x140681e50 /
child_with_global_id 0x1406861b0 / into_child_with_global_id 0x140686300 / has_global_id 0x140686450 /
expect_child_with_global_id 0x140686530 / layout 0x140686550 / ignore_input 0x140686690 / with_nav 0x140686790）、
MeiContext 6（safe_frame 0x140f99230 / new_at_pos<&str> 0x140f992e0 / new_at_pos<String> 0x142289f80 /
on_key_event 0x140685fc0 / on_clipboard_paste 0x1406860d0 / load_font_table 0x140683a20）、
LayoutStyle 3（default 0x14068e850 / horizontal 0x14068e880 / place_childen 0x14068ea20）、
LogSlider::param_to_value 0x142383730、read_node_config_from_file 0x14068fe40。

wave3/gap 独立复核（3）：`toolbar_button_keep_focus` 0x1423414b0、
`switch_toolbar_category_button` 0x1423427f0、`add_toolbar_item` 0x141e4dc40（spawn 面；kind_handler 另计 D）。

> 注：`toolbar_alignment` 的分派 case 被 wave3 REVIEW 判等，但 setter 目标未实现，整体计入 B；
> `ui_animation_state_update` 公式判等但分派/符号未实现，整体计入 B。

---

## 5. D 统计 — 未实现

### 5.1 四模块 D 清单

**clutter（22）**：
`remove_region_from_lazy_grid` 0x140ee4a70、`flush_candidates_pending_removal` 0x140ee4bf0（FIX 已验但 REVIEW 最终仍计未实现）、
`set_current_tick`、`reset_autoclutter_for_new_frame`、`autoclutter_subscribe_to_tabloids` 0x1422d6930、
`process_autoclutter_candidates` 0x1422ce260、`manage_autoclutter_entities` 0x1422d04e0、
`read_bush_positions_from_gpu` 0x140eebaa0、`autoclutter_animation` 0x1422ccef0、
`add_autoclutter_around_gates` 0x1422cc390、`add_autoclutter_around_windows` 0x1422cefe0、
`add_birdnests` 0x1422c9c70、`add_lantern_water_setdresssing` 0x1422d2fa0、
`add_preplaced_autoclutter` 0x141ce5900、`add_tree_setdressing` 0x1422d3800、
`detect_inter_roof_merlons` 0x141ce3a70、`construct_inter_roof_merlons` 0x1422d4870、
`request_inter_roof_merlon_audio` 0x1422d4230、`detect_intra_shape_corners` 0x141cd1a90、
`add_hole_at_shape_intersection` 0x141cd3ce0、`spawn_stitches` 0x141ce65b0、
`spawn_stitch_bricks` 0x141ce7190。

**onboarding（6）**：
`hint_draw` 0x141c51a60、`hint_draw::closure$2` 0x14228c260、`render_hints` 0x141c52700、
`render_hints::closure$1` 0x14228c870、`closure$2` 0x14228ca20、`closure$4` 0x14228cc00。

**wall-constructor（81）**：
主干 5（from_curve 0x140769760、construct_walls 0x140757800、
init_wall_height_and_arch_segment_height 0x14078b550、gpu_clear_wall_gate_pieces 0x140773c00、
gpu_flush_plank_buffers 0x140774010）；
ArchWalker walk 4（0x141232bb0/0x141233380/0x141231ea0/0x14048ab50）；
分段 7（0x14074f310/0x14074f7a0/0x140750420/0x140751240/0x140751610/0x14074c720/0x140751960）；
拱/过梁 8（0x14075f680/0x1407645f0/0x140765300/0x14076d9f0/0x141242530/0x141243740/0x141243c20/0x141fe09c0）；
立面支撑 22（0x1407867d0/0x140788160/0x140788a80/0x1407896b0/0x14124b2c0/0x141fe1de0/0x14124ba30/0x14124cc30/0x14124eb80/0x14124fcb0/0x141253fc0/0x141250da0/0x141248620/0x141248300/0x14078a6e0/0x141fe2150/0x141fe3240/0x142418f50/0x140753670/0x1407534d0/0x14120d670/0x14120d340）；
门/栅栏/地板/楼梯 14（0x140756370/0x140755690/0x141210110/0x140765830/0x1412388f0/0x141238d80/0x141fdf9e0/0x1407661e0/0x1407744d0/0x140496800/0x140780e20/0x1407834f0/0x14077a2f0/0x14077d520/0x140178a70）；
半木/抹灰 6（0x1407706d0/0x141244310/0x14075d460/0x14075b810/0x14075dc60/0x1412269b0）；
UI 3（0x140766e70/0x140767b70/0x140771a60）；
utils 近似体 15（0x141223d10/0x141215540/0x141217a20/0x1412221f0/0x141218e30/0x141217be0/0x1412218a0/0x141221da0/0x141221f70/0x141fd1c30/`wrap_around` 等）。

**toolbar（69 具名/族 + kind_handler 子项）**：
`Node::prev_computed_layout`、`switch_app_mode_button` 0x142341fe0、`toolbar_auto_hide` 0x142343a10、
`ui_sounds` 0x14233a7f0、`ui_slider` 0x1423486d0、`render_taffy_ui_to_widgets` 0x142338990、
`mei_pre_update` 0x1406842e0（主体）、`mei_post_update` 0x1406884c0（主体）、
6× 未实例化 alignment closure（1/3/4/5/6/7/8/9 中的 6 个）、
`taffy_update_drawables` 0x142340d20、`taffy_finalize_element_size` 0x142340890、
`setup_screen_and_toolbar_nodes` 0x141e4ccb0、`add_contextual_menu` 0x141e4d1e0、
`UndoRedoLayout::setup` 0x140fad870、`toolbar_setup` 0x142345890、
`toolbar_steal_mouse_input` 0x142344df0、`toolbar_button_size_and_padding_update` 0x141eaec50、
`ui_send_focus_begin_events`、`ui_focus_animation`、`ui_appear_animation`、`ui_select_animation`、
`ui_locked_animation`、`ui_show_if_parent_in_focus`、`ui_notify_hints_if_toolbar_in_focus`、
`ui_focus_parent_if_self_in_focus`、`sort_contextual_buttons_draw_order`、`debug_hide_ui`、`hide_ui`、
`ui_nav_update`、`iter_subtree`、`demo_show_full_glade_size`、`demo_show_max_elevation`、
`menu::advance_menu_state` 0x14068e5b0、`welcome::ui`、`new_session::ui`、`daily_welcome::ui`、
`splash::ui`、`load::ui`、`thumbnail_scroll`、`display_save_file_load_error`、`load_session_meta`、
`show_contextual_buttons`、`corrupted_save_file::ui`、`save_error::ui`、`credits::ui`、
`spawn_third_party_license_http_server`、`scroll`、`report_bug::ui`、`try_remove_file`、`text_edit`、
`toggle_icon`、`photomode::do_photomode`、`photomode_buttons`、`setup_photomode_toolbar`、
`ui_slider_reset`、`slider_val_01`、`lut_gallery::ui`、`text_startup_system`、
`update_text_renderer_scale`、`prepare_text_fragments`、`render_text_fragments`、`flush_text_render`、
`SingleFileServer::http`、`SingleFileServer::run`、`kind_handler`（add_toolbar_item 子项）、
5 个族条目（widgets 构建器/视图、tweaker 族、decorate_hovered、mei audio、config instantiate/template）。

### 5.2 integration / tiny-glade D（摘要）

- coregap-1/2：0 实现未做；InsertTreeCmd 0x48、PublicWalls 密集表为冲突跳过/待证（合同）。
- purelogic：hide_trees、despawn_fish、plan_duck_trajectories、move_ducks_along_trajectories、
  update_decorator_bird_spots、update_roof_bird_spots、calculate_roof_trigger_volumes、
  ~146 decorator 业务函数族、manage_autoclutter_entities、autoclutter_animation、
  gates/windows/preplaced/tree 4 项、request_inter_roof_merlon_audio 等。
- unlock：publicwalls-tables（12 合同 / 0 实现）、replay-decorator（4 合同 / 0 实现）。
- consume：Ui2dImage/Ui2dExtent 载荷、kind_handler、hint_draw/render_hints 等。
- remaining：hide_trees、DebugLine::show（后由 unlock/consume 闭环，见 A4/C）。
- tinyglade A：44；tinyglade B：26。

---

## 6. integration 轮逐项摘要

### 6.1 coregap-1（10 项）
- C（8）：PlayUiSound 变体 #4、SignifierFactory、MoveTreeCmd 0x20、HighlightColor+HighlightRoofCmd、
  GraphicsDeviceResource、WoodenPlankSsbo、AutoClutter 布局、utils::calculate_hash。
- B（2）：SignifierEntityBuilder 链（需修 3 项已改，无二次复核）、ReplayBucket.advance_timing（位极性已修，无二次复核）。
- 冲突跳过：InsertTreeCmd 0x48（REVIEW 确认布局合同准确）。
- 待证：PublicWalls 密集表。

### 6.2 coregap-2（12 项）
- C（10）：GameProfile 提示区、TutorialCooldowns、16 NotifyHintSystem*、TutorialOp、
  WriteSettingsFileCmd/SaveGameProfileCmd、DisplayRollercoasterPrompt、ModeJustChanged 载荷、
  StructureType::is_fence、ColorId Ssbo 族、RoofColorIdSsbo、ChimneyExtra。
- B（2）：装饰/窗族事件载荷 5 条（`_gap4` 已删，无二次复核）、GameProfile serde（已改，无二次复核）。
- 冲突跳过：SteamInput（rebuild-steam 覆盖）。

### 6.3 purelogic（26）
- C（4）：clear_out_visual_entities（decorator REVIEW fix_verified 后升判等）、
  clear_out_wall_holes（decorator REVIEW 判等）、dirty_walls（clutter REVIEW）、
  draw_water 注册（A2，严格说不是 C；此处按“已验证”计 C 会与 A 冲突——**本报告把 draw_water 计入 A2**）。
  修正：C=3（两个 clear_out + dirty_walls）。
- A2（1）：FishSpawnLocations::new；draw_water 注册（A2，另列）。
- B（6）：DuckSpawnLocations::new、create_or_restore_decorator_backup、window_lights、
  detect_inter_roof_merlons 等 4 项部分。
- D（15）：hide_trees、despawn_fish、duck trajectories 2、bird spots 2、roof trigger volumes、
  ~146 decorator 族（按 1 族计）、clutter 放置/动画族 5、request audio 等。

### 6.4 unlock（7 线）
- debugline-signifier：A4（DebugLine::show、DebugColor、SignifierFactory/UiSignifierStream 部分）、
  B（signifier_create）、D（build mesh/shader 解析面、UiState 配置表克隆）。
- geo-src-raycast：B（geo_src 两函数，5 单测但非判等）、A（legacy_raycast 修正）。
- hint-displayrequest：A4（DisplayHintRequest 布局、排序键、HintType 18）。
- mei-toolbar：A4（8 setters、UiNav、HistoryButton、20 components）、A2（mei 签名、render_taffy 签名）、
  B（link_up_nav）、D（6 closure、case 4/6）。
- nani：A4（NaniBundle/DrawableMeshBundle 布局；布局测试因 ENOSPC 未跑）。
- publicwalls-tables：D（12 合同，0 实现）。
- replay-decorator：D（4 合同，0 实现）。

### 6.5 consume（6 目录）
- C：display_glade_border 0x141be0ad0（wave4 misc）、closure$1 0x141be1080（wave4 misc）、
  add_toolbar_item spawn 面（wave4 visual）、switch_roof_type 5 子项（wave4 visual）、
  build<Transient/Persistent> + impl$4/impl$5（wave4 visual）、gpu_flush_buffers 0x140de50a0（wave4 geometry）。
- A2/A4：water closure$0、SignifierFactory::new、builder position/scale、hint_orchestrator（A4）。
- B：geo-src 两 hint（重建）、hint_position_update、process_tree_cmds、replay_tree、ui_move_tree、
  maintain_dormer_tiles。
- D：hint_draw/render_hints、Ui2dImage/Ui2dExtent 载荷、kind_handler。

### 6.6 remaining（2 目录）
- C：manage_tree_prefabs（wave4 misc）、convert_preplaced_trees（wave4 misc）、
  display_glade_border perimeter（wave4 misc）、find_stone_column_start_y（wave4 geometry）。
- B：handplaced/forest_tree_animation、butterfly、wren、move_critters、critters_avoid_collisions、
  circle_categorize（wave4 geometry FIX-G1 已落，无复验）。
- D：hide_trees、DebugLine::show（后由 unlock/consume 闭环）、wall-constructor todo 族。

### 6.7 tests（numeric-assertions）
- 31 测试 / 188 断言 / 31 通过。覆盖函数：rotation_from_normal、Rectangle2d::orientation_mat3、
  timberframe 圆/矩形计数、timberframe_beam_count_change_audio、random_tree_prefab、
  time_continuous_lerp、水下重映射、scale/rotation、mesh_slots_for_kind、select_mesh、
  weighted_mesh_landing、circular_position、sheep_walk_animation、RoofAnimation::advance、
  quantize、DELETION_COOLDOWN_TABLE。
- 这些是 **A4** 级证据；其中 rotation_from_normal 的 gap REVIEW 判“需修”（掩码等价声明错误），
  数值断言未覆盖 inv==0 退化输入。

---

## 7. tiny-glade 批次 4

### 7.1 tiny-glade A（55）
- 审计时判等 6（全 A，无 REVIEW）：log_time_to_first_frame（A4）、FpsLimiter::frame（A4）、
  is_supported_steam_language 白名单（A4）、render（A2）、render_world（A2）、
  handle_screenshot_error（A2）。
- **reviewer-A-tinyglade 复核后**：判等 5 升 C（log_time_to_first_frame、FpsLimiter::frame、
  is_supported_steam_language、render_world、handle_screenshot_error）；`render` 需修（仍 B，
  FIX-A1：OOM panic 的 `Option<(vk::Format,[u32;2])>` Debug 形状）。
- 胶水 5；未实现 44。
- 关键风险：render/render_world/handle_screenshot_error 是“委托 country-core 已判等实现”，
  本轮已逐分支/常量/文案复核；宿主设备调用不在 A-list 范围。

### 7.2 tiny-glade B（73）
- 审计时判等 18（全 A）：winit_to_bevy_keycode（A4）、copy_folder（A4）、其余 16 为 A2
  （EguiBackend::end_frame、calculate_max_resolution_scale、NvAftermathDelegate::default、
  shader_debug_info、crash_payload Serialize 6 单态、ClientHandle::send_message、server::on_message、
  dump_log_file/settings/gpu/rhapsody、dump_screenshot、spawn_hang_watchdog_thread、
  write_image_with_callback_impl、CoasterAudioInstance::new、confirm_incoming_panic_repro）。
- **reviewer-A-tinyglade 复核后**：判等 6 升 C（winit_to_bevy_keycode、copy_folder、
  calculate_max_resolution_scale、dump_log_file、dump_gpu_crash_dump_file、
  spawn_hang_watchdog_thread）；12 项需修仍 B（FIX-A2..A13；其中 4 项涉及文件格式/崩溃载荷/
  坏包处理/UI 状态机等实质偏差）。
- 重建 20（B）：calculate_resolution_scale_mem_estimate_text、dumped、spawn_server、client::start、
  server::run、create_minidump_file、on_minidump_created、crash_trace_to_sentry_event、
  setup_panic_hook、closure、recover_game_state_after_crash、create_sentry_attachment、
  hang_watchdog、verify_package_integrity_and_show_report、gather_screenshot_requests、
  handle_screenshot_opportunity、finish_screenshot_capture、hurry_active_screenshots_on_shutdown、
  run_vulkan_configuration_app、date_spinner。
- 胶水 9；未实现 26。

---

## 8. 最值得优先补独立复核的 top 10

排序依据：影响面（被多少模块消费）× 是否基础类型/入口 × 当前是否仅有自判/单测。

| # | 条目 | VA | 当前等级 | 理由 |
|---|---|---|---|---|
| 1 | `hint_orchestrator` | 0x14227fc30 | A4 | onboarding 提示总入口；9 单测与实现同源；门控/文本/实体迁移任何偏差影响全部 hint |
| 2 | `DisplayHintRequest` 布局 + 全部写面 | 0x50B / stride 0x58 | A4 | 所有 hint_* 的事件载体；placement/id/flags 偏移错一位即全错 |
| 3 | `geo_src_to_edit_subject` / `_w_deco_clutter` + legacy hit code | 0x140977f50 / 0x140978000 / 0x1408cc680 | A/B | 被 onboarding 3 函数、tree-tool、hint_roof_shape 消费；跳表/命中码是跨模块入口 |
| 4 | `DebugLine::show` + `SignifierFactory`/`UiSignifierStream` 唤醒 | 0x140b04f70 / 0x14089d8b0 / 0x140b2cf90 | A4/A2 | glade-border/water/signifier 共用；闭包槽/唤醒错则虚线/橡皮全部不显示 |
| 5 | `NaniBundle<T>` / `DrawableMeshBundle` 布局 | 0x141a07f20 / 0x141fe7aa0 | A4 | decorator/roof/toolbar 三类 spawn 的基础；布局测试因 ENOSPC 未跑 |
| 6 | `UiNav` / `HistoryButton` 布局 + link_up_nav | 0x141e36510 / 0x141e34b90 / 0x141e4d880 | A4/B | toolbar 导航/撤销重做的基础；tag=3/槽语义 U |
| 7 | 8× taffy setter closure | 0x142375c10..0x142376300 | A4 | toolbar_alignment 的语义核心；仅 setter 字段断言，无独立 REVIEW |
| 8 | tiny-glade A `render` / `render_world` / `handle_screenshot_error` | 0x1404e8010 / 0x1404ec110 / 0x1404ebfb0 | A2 | 主循环渲染入口；仅“委托 country-core”自判，无本 crate REVIEW |
| 9 | tiny-glade B `winit_to_bevy_keycode` + `crash_payload` Serialize 6 单态 | 0x1400fb850 / 0x14003e250.. | A4/A2 | 输入映射与崩溃载荷 schema；B 报告整批无 REVIEW |
| 10 | `FishSpawnLocations::new` + `draw_water` 注册 | 0x140f909f0 / — | A2 | purelogic 唯一两项“实现/判等”但无复核；鱼群与水面系统入口 |

备选：`rotation_from_normal`（gap REVIEW 已判需修，数值断言未覆盖退化输入）、
`DuckSpawnLocations::new`（B）、`hint_position_update`（B，4 单测但与实现同源）。

---

## 9. 方法与文件索引

- 四模块主证据：
  - `reports/interaction/clutter/{RESTORE,REVIEW}.md`
  - `reports/interaction/onboarding/{RESTORE,REVIEW}.md` + `reports/wave3/onboarding/RESTORE.md` + `reports/wave3/reviews/onboarding/REVIEW.md`
  - `reports/interaction/wall-constructor/{RESTORE,REVIEW}.md` + `reports/wave2/wall-constructor/RESTORE.md` + `reports/wave2/reviews/geometry/REVIEW.md` + `reports/remaining/wall-constructor/RESTORE.md` + `reports/wave4/reviews/geometry/REVIEW.md`
  - `reports/interaction/toolbar/{RESTORE,REVIEW}.md` + `reports/wave2b/toolbar/RESTORE.md` + `reports/wave3/toolbar/RESTORE.md` + `reports/wave3/reviews/toolbar-decorator/REVIEW.md` + `reports/gap/review/REVIEW.md` + `reports/unlock/mei-toolbar/RESTORE.md` + `reports/wave4/reviews/visual/REVIEW.md`
- integration：`reports/integration/{coregap-1,coregap-2,purelogic,external-dep}/**`、
  `reports/{gap,unlock,consume,remaining,tests}/**`。
- tiny-glade：`reports/tinyglade/{A,B}/RESTORE.md`。
- 口径复核：`planning/2026-10-17-upper-final-progress.md`、`planning/2026-10-17-domain-quickref.md`、
  `planning/2026-10-17-unimplemented-ledger.md`。
- 计数口径：族条目按组计；四模块 D 中含族（toolbar 5 族、wall-constructor utils 近似体 15 等），
  因此 D 的函数面展开大于本表具名数。工具栏/墙构造的 D 计数以具名符号为准，族按 1 计；
  若按函数面展开，wall-constructor 约 +10、toolbar 约 +40（与 quickref “~69/~90”一致）。

---

## 10. 回执摘要

- **四模块**：A2=0、A4=9、B=42、C=74、D=178（总 303）。
  - A 合计占比 3.0%；C 占比 24.4%；B+D 占比 72.6%。
  - A4 分布：onboarding 1（hint_orchestrator）、toolbar 8（taffy setters）。
- **A 清单（含 integration/tiny-glade）**：审计时 A2=29、A4=23，合计 52。
  - 模块分布：tinyglade 24（A 3 + B 19 + A4 5，其中 B 的 16 个 A2 已计入）、
    unlock 12、toolbar 8、purelogic 2、onboarding 1、consume 2（与 unlock 交叠）。
  - **已复核**：unlock 12（reviewer-A-unlock）与 tinyglade 24（reviewer-A-tinyglade）均已独立复核；
    tinyglade 复核结果 判等 11（升 C）/ 需修 13（仍 B）/ 未实现 0。
  - 最集中区已消除：tiny-glade A/B 24 项不再是“判等无独立 REVIEW”。
- **最值得优先补复核 top 10**：hint_orchestrator、DisplayHintRequest 布局、geo_src 链、
  DebugLine/show+signifier 唤醒、NaniBundle/DrawableMeshBundle、UiNav/HistoryButton/link_up_nav、
  8 taffy setters、tiny-glade A render 三函数、tiny-glade B 键码映射+crash_payload、FishSpawnLocations+draw_water 注册。

---

## 11. 复核回执 — reviewer-A-unlock（2026-10-17）

> 只读独立复核；fresh Ghidra/radare2 反编译/反汇编 + 独立 taffy 探针；未改 src。
> 报告与证据：`reports/unlock/reviews/A/REVIEW.md`、`reports/unlock/reviews/A/evidence/`。

### 11.1 主清单（unlock 轮 12 项）：**判等 11 / 需修 1 / 未实现 0**

| # | 条目 | 原等级 | 复核后 |
|---|---|---|---|
| 1 | `DebugColor` Debug/as_array 0x140b05c70 | A2 | **C** |
| 2 | `SignifierFactory::new` 0x14089d8b0 | A2 | **C** |
| 3 | `UiSignifierStream` write/wake 0x140b2cf90/0x140b2cfa0 | A2 | **C（限定：既有 UiState+0x20 GAP）** |
| 4 | `mei_pre_update` 9 参签名 0x1406842e0 | A2 | **C（签名；主体仍 D）** |
| 5 | `mei_post_update` 9 参签名 0x1406884c0 | A2 | **C（签名；主体仍 D）** |
| 6 | `render_taffy_ui_to_widgets` 13 参 0x142338990 | A2 | **C（签名；主体仍 D）** |
| 7 | `DisplayHintRequest` 布局 0x50B/stride 0x58/placement/flags/id | A4 | **C** |
| 8 | `sort_display_hint_requests` 排序键 0x1425e5b70 | A4 | **C** |
| 9 | `HintType::Rollercoaster = 18` | A4 | **C** |
| 10 | `DebugLine::show` 0x140b04f70 | A4 | **需修（FIX-A1：payload[3] 应 colors.capacity()）** |
| 11 | `NaniBundle<T>` 0x48B 布局 | A4 | **C** |
| 12 | `DrawableMeshBundle` 0x40B 布局 | A4 | **C** |

### 11.2 任务点名附加项：**判等 6 / 需修 0 / 未实现 0**

| 条目 | 原状态 | 复核后 |
|---|---|---|
| `geo_src_to_edit_subject` / `_w_deco_clutter` 跳表链 | B | **C** |
| `legacy_raycast` 命中码 0x1408cc680 | B/A 修正 | **C** |
| `UiNav` 0x40 布局 / `HistoryButton` 1B 布局 | A4 | **C**（toolbar A4 中 2 项） |
| `link_up_nav` 0x141e4d880 | B | **C（重建，语义等价）** |
| 8× taffy setter closure | A4 | **C**（toolbar A4 中 8 项） |
| `RoofTileSsbo` `SparseSsbo` | wave3 core-type-gap/D | **C（gap 已闭合）** |

### 11.3 账目影响与剩余

- 本轮复核范围内：unlock 11 项、toolbar A4 10 项（8 setters + UiNav + HistoryButton）、
  geo-src 2 函数 + legacy_raycast、RoofTileSsbo gap **由 A/B/D 升 C**；
  `DebugLine::show` 保留 **需修**（FIX-A1）。
- toolbar A4 剩余 **1 组**未独立复核：20× `register_required_components` 组件定义
  （本复核只覆盖 UiNav/HistoryButton 两项尺寸）。
- **FIX-A1（唯一）**：`crates/country-core/src/utils/debug_viz.rs` `DebugLine::show` 闭包
  payload 第 4 qword 由 `points.capacity()` 改为 `colors.capacity()`（原生 build 默认分支
  以该槽作 colors Vec cap；colors 与 points 不同长时暴露，属字段级不等）。
- 结论：**需修 1 / 判等 17**。冷却表 `[f32;19]` 与原生 0x143038ae8 的
  19×f32 + 4B padding（+0x50 起 u64 表）一致，无需调整。

## 12. 复核回执 — reviewer-A-toolbar（2026-10-17）

> 只读独立复核；fresh Ghidra 反编译/反汇编（`--worker reviewer-a-toolbar`）；未改 src。
> 报告与证据：`reports/reviews/A-toolbar/REVIEW.md`、`work/reviewer-A-toolbar/`。
> 范围：toolbar 4（keep_focus/switch_category/tod_v2/ui_animation）+ purelogic 3
> （FishSpawnLocations::new/DuckSpawnLocations::new/draw_water 注册）+ consume 5
> （replay_tree/gpu_flush_buffers/maintain_dormer_tiles/process_tree_cmds Insert/ui_move_tree）= **12 项**。

### 12.1 逐项 verdict：**判等 3 / 需修 9 / 未实现 0**

| # | 条目 | 原等级 | 复核后 | 关键结论 |
|---|---|---|---|---|
| 1 | `toolbar_button_keep_focus` 0x1423414b0 | C（gap REVIEW） | **需修** | 四步算法判等；keep/selected 属原生 `UiInteractionState`(3B) 本身，Rust 拆出的 `WidgetByteState` 从未 spawn → Query 空集、系统 no-op |
| 2 | `switch_toolbar_category_button` 0x1423427f0 | C（gap REVIEW） | **需修** | 门/载荷判等；同上空集，`query.get` 恒 Err → 类别事件不发射 |
| 3 | `tod_v2_button` 0x142342f30 | B（FIX-3） | **需修** | 清 visible 事件源错误：原生读 `ModeJustChanged` 首事件 tag≠0xd 且在 click 之后；Rust 读 click 非空且在 click 之前；UiNavState 写面缺 |
| 4 | `ui_animation_state_update` 0x14233b3f0 | C（gap REVIEW） | **需修** | 公式/state 机判等；appear sign 源应为 interaction+0；Query 空集 no-op |
| 5 | `FishSpawnLocations::new` 0x140f909f0 | A2 | **需修** | 行列转置（原生外层→x）；公式 7/25 档差 1 ULP；used 字节缺（已登记） |
| 6 | `DuckSpawnLocations::new` 0x140f8ee90 | 实现 | **需修** | 同行列转置；行公式逐位一致/timer 25.0/used 正确 |
| 7 | `draw_water` 注册 | A2 | **C（限定）** | 原生 build 判等（2 系统+3 事件+WaterRaster）；draw_water 为已登记自足追加，非原生注册面 |
| 8 | `replay_tree` 0x141bc34f0 | B | **需修** | 时序/逆序/各分支判等；Move-undo 旧位路径整条未实现（G2/T3 登记属实） |
| 9 | `gpu_flush_buffers` 0x140de50a0 | A4 | **C** | 5 行逐语句一致 |
| 10 | `maintain_dormer_tiles` 0x1407ce920 | A4 | **C** | 游标预写/每瓦四步/drop 判等 |
| 11 | `process_tree_cmds` Insert 0x142267430 | B | **需修** | 名称门/门控/α/组件/声音判等；SpawnPrefabCmd 0x50 载荷只发 default（T5） |
| 12 | `ui_move_tree` 0x14226b1a0 | B | **需修** | 门禁/状态机（state 0/1 亦钳 case3）判等；T3 旧位缺；MoveTreeCmd 原生 payload 0x18 vs Rust 0x20 |

### 12.2 账目影响

- **升级 C**：`gpu_flush_buffers`、`maintain_dormer_tiles`（A4→C）；`draw_water` 注册
  （A2→C，限定：原生 build 判等 + draw_water 自足追加已登记）。
- **降级 需修**：toolbar 3 项原 C（gap REVIEW 判等）因 `WidgetByteState` 未挂载的
  Query 空集问题成立而不可维持判等；`tod_v2` 原 B 的 FIX-3 之外新增事件源/顺序分歧。
- **purelogic 两构造器**：原 A2/“实现/判等”改判需修（转置为跨消费面实证：Fish/Duck
  consumer 均以候选 Vec2 首分量作世界 x）。
- **consume**：3 项维持 B（需修），本次补 `MoveTreeCmd` payload 0x18 布局纠错。
- **FIX 清单**：FIX-T1/T2、FIX-P1/P2、FIX-C1..C4（详见 REVIEW.md §4）。其中 FIX-T1/C4
  为 core 类型改动，需集成 owner 裁决。

---

## 13. 复核回执 — reviewer-A-tinyglade（2026-10-17）

> 只读独立复核；Ghidra shared fresh 反编译 + 汇编 + .rdata 读回；未改 src/库。
> 报告与证据：`reports/tinyglade/reviews/A/REVIEW.md`、`reports/tinyglade/reviews/A/evidence/`。

### 13.1 主清单（tinyglade A/B 24 项）：**判等 11 / 需修 13 / 未实现 0**

| # | 条目 | VA | 原等级 | 复核后 |
|---|---|---|---|---|
| 1 | `render` | 0x1404e8010 | A2 | **需修**（FIX-A1：OOM panic Debug 形状） |
| 2 | `render_world` | 0x1404ec110 | A2 | **C** |
| 3 | `handle_screenshot_error` | 0x1404ebfb0 | A2 | **C(limited)**（日志交宿主，调用点未接线 U） |
| 4 | `log_time_to_first_frame` | 0x1403db200 | A4 | **C** |
| 5 | `FpsLimiter::frame` | 0x140270850 | A4 | **C** |
| 6 | `is_supported_steam_language` | 0x140323230 内 | A4 | **C** |
| 7 | `winit_to_bevy_keycode` | 0x1400fb850 | A4 | **C**（256 表 0 mismatch；194 变体逐名） |
| 8 | `fs_util::copy_folder` | 0x140355800 | A4 | **C** |
| 9 | `EguiBackend::end_frame` | 0x1400dd910 | A2 | **需修**（FIX-A2） |
| 10 | `calculate_max_resolution_scale` | 0x1404f7f20 | A2 | **C**（budget getter 契约 U） |
| 11 | `NvAftermathDelegate::default` | 0x140244460 | A2 | **需修**（FIX-A3） |
| 12 | `NvAftermathDelegate::shader_debug_info` | 0x140245340 | A2 | **需修**（FIX-A4） |
| 13 | `crash_payload` Serialize 6 单态 | 0x14003e250/e7c0/eb80/ec80/e350/eda0 | A2 | **需修**（FIX-A5：breadcrumbs 类型） |
| 14 | `ClientHandle::send_message` | 0x14015ac60 | A2 | **需修**（FIX-A6：.unwrap 文案） |
| 15 | `server::on_message` | 0x1402ae1f0 | A2 | **需修**（FIX-A7：坏包 panic 非继续） |
| 16 | `dump_log_file` | 0x1403acc70 | A2 | **C** |
| 17 | `dump_settings_file` | 0x1403ae050 | A2 | **需修**（FIX-A8：.unwrap） |
| 18 | `dump_gpu_crash_dump_file` | 0x1403ae570 | A2 | **C** |
| 19 | `dump_rhapsody_log` | 0x1403ae770 | A2 | **需修**（FIX-A9） |
| 20 | `dump_screenshot` | 0x14058a060 | A2 | **需修**（FIX-A10：jpg/RGB8/文案） |
| 21 | `spawn_hang_watchdog_thread` | 0x140365be0 | A2 | **C** |
| 22 | `write_image_with_callback_impl` | 0x1403206a0 | A2 | **需修**（FIX-A11） |
| 23 | `CoasterAudioInstance::new` | 0x1403b6880 | A2 | **需修**（FIX-A12） |
| 24 | `confirm_incoming_panic_repro` | 0x1400ed280 | A2 | **需修**（FIX-A13：状态机/描述/标题） |

### 13.2 账目影响

- tinyglade A：A2 3→0、A4 3→0、**C 0→5**、**B 0→1**（D 44、胶水 5 不变）。
- tinyglade B：A2 16→0、A4 2→0、**C 0→6**、**B 20→32**（D 26、胶水 9 不变）。
- 最高影响 FIX：A5（崩溃载荷 breadcrumbs 类型）、A7（坏包 unwrap 语义）、
  A10（截图文件名/编码）、A11（图片写盘格式分派）、A13（panic-repro UI 状态机）。
- FIX 清单：FIX-A1..A13（详见 REVIEW.md §2）；修后需二次独立复核方可升 C。

## 14. 复核回执 — reviewer-B-interaction（2026-10-17/18）

> 只读独立复核；Ghidra `--worker shared` fresh 反编译（30 函数）+ disassemble_function/radare2
> 逐指令（8 函数）+ 跳表字节读回；未改 src。
> 报告与证据：`reports/reviews/B-interaction/REVIEW.md`、`reports/reviews/B-interaction/evidence/`。
> 范围：§3.1 B 清单 clutter 1 + onboarding 25 + wall-constructor 4 = **30 项**。

### 14.1 逐项 verdict：**判等 5 / 需修 25 / 未实现 0**

| # | 模块 | 条目 | VA | 复核后 | 关键结论 |
|---|---|---|---|---|---|
| 1 | clutter | populate_autoclutter_regions | 0x1422d6ed0 | **需修** | select_mesh off-by-one；live 权重源/逐记录→逐格近似（FIX-BI-01..03） |
| 2 | onboarding | resolve_steam_impl | 0x14227e5e0 | **C（限定）** | 分支/递归/空-单-Alternate 逐句一致；0xcd 哨兵未建模 |
| 3 | onboarding | hint_highlight_shape | 0x141c50120 | **需修** | 缺 +0x11 门；WallId 源应为组件+8 |
| 4 | onboarding | hint_position_update | 0x141c50510 | **需修** | 6/10、8 max-x、夹取/lerp 判等；7/8 的 Y 与 default 跳表未实现 |
| 5 | onboarding | exponential_back_off | 0x142284800 | **需修** | 公式/写面判等；reader 应取末事件+耗尽 |
| 6 | onboarding | reset_hints | 0x141c1c8d0 | **需修** | 资源集/掩码写面判等；state u32 2/0 vs bool；reader 语义 |
| 7 | onboarding | hint_camera_display_condition | 0x14227c910 | **需修** | 逐 id duration/progress 门与 PlayUiSound 未复现 |
| 8 | onboarding | camera_pan_hint | 0x14227d000 | **需修** | 单 Graphic 三图标+0.7/100；Row 四键自 profile；默认键 +0x1c2 |
| 9 | onboarding | camera_rotate_hint | 0x14227d2f0 | **需修** | 三图标单 Graphic；Steam 索引 1/1；尾项两 Key adjacent+alternate |
| 10 | onboarding | camera_zoom_hint | 0x14227d680 | **需修** | 索引 0x10/0x11；尾项 Graphic("hint/scroll",100) |
| 11 | onboarding | hint_focused_edit_condition | 0x14227da70 | **需修** | 门控/id1/焦点/声音判等；reader 末事件+耗尽 |
| 12 | onboarding | hint_focused_edit_hint | 0x14227dd60 | **C（限定）** | 跳表 1→Wall(0x258≥2)/2→Roof；写面 0x0101/0x0100 逐指令一致 |
| 13 | onboarding | hint_clutter_focused_edit_condition | 0x141c3d890 | **需修** | 门控/id9 判等；reader 末事件+耗尽 |
| 14 | onboarding | hint_clutter_focused_edit_hint | 0x141c3db80 | **C（限定）** | UiState+0x18/raycast/geo_src 3/写面 0x0100 逐句一致 |
| 15 | onboarding | make_focus_or_camera_binding_hint | 0x141c3de00 | **需修** | 双图标+0.7/100 判等；默认键 +0x1c6 |
| 16 | onboarding | hint_roof_shape | 0x142284ed0 | **需修** | 展示门缺 raycast；geo_src seam 恒假 |
| 17 | onboarding | make_lbm_drag_binding_hint | 0x142285270 | **需修** | 第二 Steam set(1)；双图标+0.7/100 |
| 18 | onboarding | hint_stairs | 0x142285a20 | **需修** | 阶段 5 极性反；counter/状态条件/timer/载荷门 |
| 19 | onboarding | hint_terrain_layering | 0x142283cc0 | **需修** | count 仅 byte==0 时+1；载荷 seam |
| 20 | onboarding | hint_terrain_focus | 0x142283980 | **需修** | 完成依赖 byte==2（seam） |
| 21 | onboarding | hint_terrain_steepness | 0x142283640 | **需修** | 完成依赖 byte!=0（seam） |
| 22 | onboarding | hint_undo | 0x142284220 | **需修** | just_pressed/两键槽 seam |
| 23 | onboarding | hint_unlink_windows | 0x14227d760 | **需修** | 完成依赖 byte!=0（seam） |
| 24 | onboarding | hint_deselect_tool | 0x1422853c0 | **需修** | 缺 reader C 门；D/E count++ 未实现 |
| 25 | onboarding | hint_disable_snapping | 0x14227e140 | **需修** | reader 末事件；键槽 seam |
| 26 | onboarding | prompt_rollercoaster | 0x14227bb70 | **需修** | Steam 0x14；键取自 profile+0x15c |
| 27 | wall | ArchWalker::new | 0x14074c2d0 | **需修** | 字段偏移对；实参 ABI 不匹配（R8D/XMM3/XMM1） |
| 28 | wall | calculate_wall_path_segments_impl | 0x14122ff90 | **需修** | NaN min/bounds panic/u 投影空间周期 |
| 29 | wall | circle_categorize | 0x14124af60 | **C** | 逐指令一致；FIX-G1 已落实 |
| 30 | wall | BrickRowSine::new | 内联 0x140769760 | **C（限定）** | random_f 四值公式逐句一致；无调用点 |

### 14.2 账目影响

- **升级 C（5）**：onboarding `resolve_steam_impl`、`hint_focused_edit_hint`、
  `hint_clutter_focused_edit_hint`（B→C）；wall `circle_categorize`、`BrickRowSine::new`（B→C）。
- **维持 B（25）**：均已独立复核并给出具体 FIX；不得再以“结构判等”计入判等。
- 四模块表更新（本三模块）：clutter B/C **1/8**（不变）；onboarding **25/22 → 22/25**；
  wall-constructor **4/12 → 2/14**；三模块合计 B/C **30/42 → 25/47**。
- **FIX 清单**：FIX-BI-01..22（详见 REVIEW.md §5）；其中 FIX-BI-04/05/06 为跨函数/核心事件类型
  改动，需集成 owner 裁决；修后须二次独立复核方可升 C。
- 关键反例：`populate_autoclutter_regions::select_mesh` 返回后移一格且末槽误 panic；
  `hint_stairs` 阶段 5 门控极性反；camera_pan/rotate 三图标被拆成 3 个单图标元素。

## 15. 复核回执 — reviewer-B-toolbar（2026-10-17/18）

> 只读独立复核；Ghidra `--worker shared` fresh 反编译（41 项函数/43 VA）+ B patched EXE
> radare2 直读小函数机器码 + .rdata 格式 pieces/字符串实读；未改 src、未改库。
> 报告与证据：`reports/reviews/B-toolbar/REVIEW.md`、`reports/reviews/B-toolbar/evidence/`。
> 范围：§3.1 B 清单 **toolbar 12 项** + §3.2/§7.2 **tinyglade B 20 重建**；
> 并按任务补读/补审 tinyglade **D 重建 4 项**、**E 重建 5 项**（D/E 不在审计 B 表内）。合计 **41 项**。

### 15.1 逐项 verdict：**判等 5 / 需修 36 / 未实现 0**

> 下表 40 行；第 21 行合并 `setup_panic_hook` 与 `setup_panic_hook::closure$0` 两个函数，
> 故条目数 = 41。

| # | 组 | 条目 | VA | 复核后 | 关键结论 |
|---|---|---|---|---|---|
| 1 | toolbar | ComputedLayout::center | 0x14068ea00 | **C** | 机器码 0.5*size+pos 逐指令（原“物理 return”为 void 签名假象） |
| 2 | toolbar | Node::hover | 0x140683c90 | **C** | `state_bits & 1` 逐指令 |
| 3 | toolbar | Node::has_focus | 0x140683ca0 | **C** | `(state_bits & 0x40)>>6` 逐指令 |
| 4 | toolbar | reuse_or_create_node | 0x140683ec0 | **B** | 身份回收（remove_entry 旧 flags/id/载荷）未复刻（FIX-BT-1） |
| 5 | toolbar | process_input_action_handlers | 0x140687eb0 | **B** | 门用 parents 应为 nodes；反向仅 [0,stop]；on_input 2B 状态实参（FIX-BT-2） |
| 6 | toolbar | load_configs | 0x140682e70 | **B** | 缺 relative_to；panic 缺路径；log target（FIX-BT-3） |
| 7 | toolbar | config::load_font_table | 0x14068fc60 | **B** | IO panic→Err；from_ron 多 validate/Options（FIX-BT-4） |
| 8 | toolbar | place_childen_impl<XY/YX> | 0x140fe12a0/19a0 | **B** | YX sum 读 min 非 size；justify 6+ 未实现；输出 {f32,f32,Vec2}（FIX-BT-5） |
| 9 | toolbar | toolbar_alignment | 0x142344560 | **B** | case 4/6、8、0xc..0x13 未实现；未接回分派（FIX-BT-6） |
| 10 | toolbar | tod_v2_button | 0x142342f30 | **B** | 缺 UiNavState+0x10=0 写面（FIX-BT-7） |
| 11 | toolbar | ui_animation_state_update | 0x14233b3f0 | **B** | 两条 state 只置 2 路径缺无条件 select clamp（FIX-BT-8） |
| 12 | toolbar | link_up_nav | 0x141e4d880 | **B（限定）** | payload 写面一致；缺 Commands insert；ids 源 stride 0x20@+8（FIX-BT-9） |
| 13 | tinyglade B | calculate_resolution_scale_mem_estimate_text | 0x1404f8240 | **B** | 数值链判等；格式串“数字与 GiB”间多空格（FIX-BT-10） |
| 14 | tinyglade B | NvAftermathDelegate::dumped | 0x140244790 | **B** | zip 结构/名字/文案判等；FileOptions 立即数未锚定（FIX-BT-11） |
| 15 | tinyglade B | CrashReporter::spawn_server | 0x14015aeb0 | **B** | to_str panic→Err；Child 未保存（FIX-BT-12） |
| 16 | tinyglade B | client::start | 0x14048a170 | **B** | 缺 reader spawn + CrashHandler::attach（FIX-BT-13） |
| 17 | tinyglade B | server::run | 0x1402aee80 | **B** | 原生 Server::with_name/run；Rust 自造 channel 循环（FIX-BT-14） |
| 18 | tinyglade B | create_minidump_file | 0x1402ad880 | **B** | 自造 open_minidump/next_minidump（FIX-BT-15） |
| 19 | tinyglade B | on_minidump_created | 0x1402adae0 | **B** | 缺 open/read_to_end + ResolvedCrash +0x200 写面（FIX-BT-16） |
| 20 | tinyglade B | crash_trace_to_sentry_event | 0x1404b17d0 | **B** | Exception/Mechanism/Stacktrace 被扁平化（FIX-BT-17） |
| 21 | tinyglade B | setup_panic_hook + closure$0 | 0x1400db5b0/6b0 | **B** | Mutex/panic_count 双 panic/client 捕获近似（FIX-BT-18） |
| 22 | tinyglade B | recover_game_state_after_crash | 0x1400dbc30 | **B** | 目录 Context、breadcrumb via logging_map_error 未复刻（FIX-BT-19） |
| 23 | tinyglade B | create_sentry_attachment | 0x1404b4d50 | **B** | limit off-by-one（原生 <0x1400001）；stat context（FIX-BT-20） |
| 24 | tinyglade B | hang_watchdog | 0x140365e20 | **B** | 缺 thread_priority set + 两条日志（FIX-BT-21） |
| 25 | tinyglade B | verify_package_integrity_and_show_report | 0x1402552b0 | **B** | 缺 EventLoop/RedrawWindow；GUI 非无条件（FIX-BT-22） |
| 26 | tinyglade B | gather_screenshot_requests | 0x14031af10 | **B** | 缺热键/文件命名/Steam/MetaCommander/fold 主体（FIX-BT-23） |
| 27 | tinyglade B | handle_screenshot_opportunity | 0x14031d140 | **B** | 匹配三条件简化；缺 dispatch/readback（FIX-BT-24） |
| 28 | tinyglade B | finish_screenshot_capture | 0x14031ea20 | **B** | 两处 read_back_screenshot 缺失；state 应 u32@+0x50（FIX-BT-25） |
| 29 | tinyglade B | hurry_active_screenshots_on_shutdown | 0x14031eed0 | **B** | 缺 JoinHandle join 与预算计算（FIX-BT-26） |
| 30 | tinyglade B | run_vulkan_configuration_app | 0x1403e74b0 | **B** | 缺 EventLoop/write_game_config_to_disk；层映射自造（FIX-BT-27） |
| 31 | tinyglade B | date_spinner | 0x140148b40 | **B** | speed/range 描述符未在函数体锚定（FIX-BT-28） |
| 32 | tinyglade D | NvAftermathDelegate::default | 0x140244460 | **C** | FIX-A3 落库后与 fresh 一致（布局 I） |
| 33 | tinyglade D | capture_image | 0x140589c90 | **B** | 尺寸算法判等；w/h==0 提前返回与原生 h==0 行为不同（FIX-BT-29） |
| 34 | tinyglade D | report_crash_event_to_sentry | 0x1404b3e30 | **B** | 9 项数组顺序（Minidump 应首项）；缺 GameProfile 读取（FIX-BT-30） |
| 35 | tinyglade D | ReverseBacktrace::fmt | 0x1402cfc30 | **B** | 文本行反转替代逐 frame 逆序（FIX-BT-31） |
| 36 | tinyglade E | find_best_fullscreen_video_mode | 0x1400453c0 | **B** | 评分键判等；D1/D2/D3 平台面与 sort 方向未锚定（FIX-BT-32） |
| 37 | tinyglade E | setup_window | 0x1400436b0 | **B** | D4–D6；None 分支缺 “Desired monitor…” 日志（FIX-BT-33） |
| 38 | tinyglade E | gather_pending_video_setting_changes | 0x1404e3380 | **C** | 逐语句一致（队列/consumed/锁/标志字节；E1 类型名 I） |
| 39 | tinyglade E | extract_available_monitors_info | 0x1404e2470 | **B** | get_monitor_info 失败 warn 分支与 sort 方向（FIX-BT-34） |
| 40 | tinyglade E | EguiEventIo::new | 0x14026de50 | **B** | 24B 外壳与 {1,1,0}@+0x120/+0x128/+0x130 未建模（FIX-BT-35） |
| 41 | tinyglade E | EguiEventIo::take_egui_input | 0x14026dfd0 | **C** | Mutex/poison/puffin/State 转发逐句一致 |

### 15.2 账目影响

- toolbar（B 12）：**B 12 → C 3 / B 9**；升 C：`ComputedLayout::center`、`Node::hover`、
  `Node::has_focus`（原“物理 return”是 Ghidra `void()` 签名假象，B EXE radare2 直读可判等）。
- tiny-glade B（20）：**B 20 → C 0 / B 20**；全部有实现但存在逐语句分歧，不得按“重建诚实”升 C。
- tiny-glade D 重建（4）：**C 1 / B 3**；E 重建（5）：**C 2 / B 3 条目**
  （`take_egui_input` 判等但同条目的 `new` 未闭合 → EguiEventIo 条目仍计 B）。
- 合计 **判等 5 / 需修 36 / 未实现 0**。
- **FIX 清单**：FIX-BT-1..35（详见 REVIEW.md §5）；P1 集中在
  `reuse_or_create_node`、`process_input_action_handlers`、`load_font_table`、
  `place_childen_impl`、`toolbar_alignment`、`on_minidump_created`、截图四函数、
  `report_crash_event_to_sentry`、`find_best`/`setup_window`。修后须二次独立复核方可升 C。
- **附带 C 清单纠正**：`Node::used` 0x140683c80 的 B EXE 机器码为
  `test byte [rcx+0xb6],0x30; setne al; ret`（有返回值），当前 Rust 为空体；
  `Node::prev_computed_layout` 依赖 `+0x90/+0x94` 而非纯空。二者均无调用点，但应按
  “需修/未实现”重判并修签名。
- 关键反例：`create_minidump_file` 自造 `open_minidump`；`finish_screenshot_capture`
  两处 readback 缺失；`create_sentry_attachment` 20MiB 边界差 1 字节；
  `vram` 文本每处 “数字 GiB” 多一个空格（pieces 实读已证）。
