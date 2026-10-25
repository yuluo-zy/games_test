# RESTORE — wall systems 收尾（卡A2，order2-3）

实现者: restorer-interaction
日期: 2026-10-16

## 核对结论

`recovery/README.md` 明确：**"墙/屋顶/历史已具备编辑/发布/回放链，不在本批加深；其当前改动仅作编译收尾"**。本批核对 wall 模块现状并登记缺口，不做大件实现（45+79 函数超出本卡诚实预算，避免低质填充）。

## 现状核对

- `systems/wall/` 已有：construct_wall_bricks、registration、adjust_wall_height、
  delete_wall、wall_highlight、calc_wall_bend、history_recording、wall_state/（含
  publish_public_wall_state、wall_spatial_hash）。
- `recovered_building/wall_history_edit.rs`：`WallCompoundHistoryEdit` 结构 + 构造器
  （`resources/history/wall_compound_edit.rs` 原函数的等价实现，map 侧已有）。
- `resources/history/replay_cmd_buffer.rs`：`expand_wall_compound`（回放展开）。

## 登记缺口（待 wall 组/后续卡闭合）

- `resources/history/wall_compound_edit.rs`（原 79 函数：serde/new/clone/fmt + 复合编辑）：
  crate 内 `WallCompoundHistoryEdit` 已覆盖核心，完整 serde/new/闭包族待取证。
- `valid_enclosure`（45 函数：墙围合判定资源/检查）：crate 无对应文件，GAP。
- `systems/wall/snapping/*`（snapping_init/maintain/to_ends/audio）、
  `systems/wall/eraser/*`（wall_eraser/ui_bulldozer/archivist/animate_wall_bulldoze）：均
  缺，登记待闭合。
- `systems/wall/shapes/*`：登记。

## 编译状态

`cargo check -p country-core --lib` 0 error（本卡未改 src；仅核对+登记）。

## 留给 reviewer 的核对点

1. wall 既有实现覆盖（construct/delete/highlight/history_recording）与
   WallCompoundHistoryEdit 等价性。
2. valid_enclosure/snapping/eraser GAP 登记是否与 wall 组台账一致。
