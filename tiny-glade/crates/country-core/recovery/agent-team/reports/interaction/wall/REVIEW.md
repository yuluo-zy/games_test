# REVIEW — systems/wall(卡A2 终验 → 闭合·核对+登记)

审查员: reviewer-interaction | 日期: 2026-10-16
基线: restorer RESTORE.md(reports/interaction/wall/,本卡未改 src,仅核对+登记)。

## 核对

### ① wall 现状覆盖 — PASS
`systems/wall/` 已有 construct_wall_bricks/registration/adjust_wall_height/delete_wall/
wall_highlight/calc_wall_bend/history_recording/wall_state/{publish_public_wall_state,
wall_spatial_hash} —— 与 README"墙/屋顶/历史已具备编辑/发布/回放链,不在本批加深"一致 ✓。
本卡未改 src(核对+登记),不存在新增代码引入风险。

### ② wall_compound_edit 覆盖 — PASS(map 侧)
- `recovered_building/wall_history_edit.rs::WallCompoundHistoryEdit`(结构 + 构造器,
  原 resources/history/wall_compound_edit.rs 的等价实现)✓。
- `resources/history/replay_cmd_buffer.rs` 消费 `expand_wall_compound`(wall_history_replay)
  做回放展开(line 94)✓。
- 完整 serde/new/闭包族(原 79 函数)待取证登记,不冒充 ✓。

### ③ GAP 登记诚实 — PASS
`valid_enclosure`(45)、`systems/wall/snapping/*`、`systems/wall/eraser/*`(wall_eraser/
ui_bulldozer/archivist/animate_wall_bulldoze)、`systems/wall/shapes/*`、wall_compound_edit
完整族——实测 src 无对应文件,与 wall 组台账一致,诚实登记不冒充闭合 ✓。
"未做 45+79 大件 = 诚实预算判定(避免低质填充)"与 README 一致 ✓。

## 结论与 DISPATCH 建议

- 卡A2 复验通过:**核对+登记闭合**;GAP(valid_enclosure/snapping/eraser/shapes/
  wall_compound_edit 完整族)归 wall 组/后续卡。
- 建议:卡A2 → 闭合。本卡未改 src,编译状态不受本卡影响(现全 lib 唯一 error 为
  audio_resources.rs FMOD panic 格式,audio 组在途,不属本卡)。
