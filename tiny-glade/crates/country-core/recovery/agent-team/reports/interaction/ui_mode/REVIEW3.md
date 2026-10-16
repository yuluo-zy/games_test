# REVIEW3 — systems/ui_mode(卡E-slice3 终验 → 闭合·mode_manager)

审查员: reviewer-interaction | 日期: 2026-10-16
基线: restorer RESTORE.md 三.卡E-slice3 节;mode_manager 6 测试通过。
编译说明:现全 lib 唯一 error 在 `resources/audio_resources.rs:183`(FMOD bank panic 格式,
**audio 组在途暂态,非本批**);本批文件(mode_manager/signifier/brush_preview/widget_params/
slider3d)零 error。

## 核对

### ① mode_manager 既有实现 — PASS(实质完整,非存根)
- `category_advance`(→toolbar.advance_category 类别索引推进 current+1 % len)→ mode=next ✓。
- `select_app_mode`(0x140afa2c0):mode=request;**Editing/CloneClutter 特判**走
  Editing 工具栏槽、其余 switch_mode ✓。
- `toggle_photomode`(0x140afa9f0):Photomode 快捷切回 Editing(None)、否则普通类别循环 ✓。
- `esc_menu_blocks`:`probe.hovered && widget_id==esc_menu_canvas_id && hit_tag!=2` 悬停
  早退 ✓。
- `send_mode_just_changed_event`:current≠previous → `AppModeChanged{current, previous}`
  双 AppMode 事件 + PreviousAppMode 更新 ✓。
- `mode_manager` 主系统:esc_menu 早退 → History 录音门(blocked → ModeManagerSound::
  SwitchBlocked)→ requested_history/undo/redo(槽 16/17)/Replay 早退/LoadingGameComplete
  → restore/requested_modes → select_app_mode/requested_categories → category_advance/
  工具跳表(槽 4..12,原序 eraser 先于 terrain,再 garden/tree/water)/photomode(13)/
  time_of_day(14)/clear(26 pressed)——**实质实现,与原序/分派一致** ✓。

### ② AppMode::is_tool_selected = {6..12,15..18} — PASS(机器码已验证)
卡E-slice1 已 fresh 0x140a86b40 验证 `0x3cfe >> (tag-5)` + `(tag-6)<0xf` 真集 =
{6..12,15..18},与 types.rs 一致 ✓(本轮复核未变)。

### ③ 待接登记 — PASS
spawn_brush_previews/path_brush_preview_size/signifier_animation/mark_for_delete/despawn
实体机制项全登记,无冒充。

## 结论与 DISPATCH 建议

- 卡E-slice3 复验通过:**mode_manager 主体闭合**,卡E 主体(UI/widget_params/slider3d/
  AppMode/signifiers/brush_preview/mode_manager)全闭合。
- 建议:slice3 → 闭合;卡E → 闭合(实体机制项登记待接);audio_resources FMOD 错误为
  audio 组在途,不属本批。
