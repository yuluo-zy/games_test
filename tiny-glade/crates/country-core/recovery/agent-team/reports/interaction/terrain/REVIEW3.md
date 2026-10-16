# REVIEW3 — systems/terrain(三轮 FIX 复验)

审查员: reviewer-interaction | 日期: 2026-10-15
基线: restorer 已按 REVIEW2 修复 ui_edit_terrain 点集死代码;`cargo check --lib` 0 error。

## 三轮 FIX 复核

### ui_edit_terrain 点集路径死代码 — **PASS**

- `TerrainStrokeParamsCache` 增加 `points_stamp: u64`(len 基 + 逐点 `f32::to_bits` ×3 的
  wrapping hash 指纹)。
- `points_stamp != cache.points_stamp` → `dirty |= DIRTY_MASK_POINTS`(位 2)✓。
- 点变化时 `point_sets.write(SetTerrainStrokePtsCmd { points: stroke.points.clone(), ... })`
  现在**可达**(不再是死代码),并回写 `cache.points_stamp` 防重复 ✓。
- 高度/falloff 位(0/1)路径不变,EditTerrainStrokeCmd 仍按脏位发 Absolute/Keep ✓。
- 指纹机制为 I(推断),非原 16 位脏掩码逐位语义;但 FIX 目标(点集命令可达)已达成,
  脏掩码精确位序仍待取证(已登记)。

## 结论与对 DISPATCH 建议

- ui_edit 点集死代码 FIX **复验通过**。
- 维持 `in_progress`;待 re-analyst 补证据后闭合:
  1) EditType::Terrain 真 native_tag(占位 14);
  2) ui_terrain_focus_mode 状态机主体(现为门存根,不计数闭合);
  3) ui_draw_terrain 消费面(高度采样门、UI 画布 widget、界外反馈);
  4) 16 位脏掩码精确位序与 widget 常量。
- 其余既有(terrain_hover_idx 0x421 门、deferred 哨兵、3 个 CPU 系统)维持前轮结论。
