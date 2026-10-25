# REVIEW2 — systems/terrain(二轮复验)

审查员: reviewer-interaction | 日期: 2026-10-15
基线: restorer-interaction RESTORE.md + 新增 5 文件;AppMode 裁决落地。
本轮核对点:0x421 门、deferred_terrain_history 哨兵、ui_draw/ui_edit 门、ui_terrain_focus_mode。

## 逐项复验

### 1. terrain_hover_idx @0x140aff720 — OK(门)/FIX(光标门缺)

- AppMode 0x421 掩码门(bits {0,5,10} = All/Garden/Terrain):`disc >= 0xb || ((0x421>>disc)&1)==0
  → return`。0x421 = 0b10000100001 = 位 0/5/10 ✓(章节 + re-analyst 裁决,修正 {0,5,9})。
- -1 哨兵 / 水位 ≤ 0 / 距离 ≤ 0.5 不更新、写 `{tag=1, id}` ✓。
- **缺**:章节"光标无效"门当前 `_cursor: Res<CursorPositionSS>` 被忽略(未检查光标有效性)。
  MouseTerrainRaycastSsbo/WaterMaskSsbo 布局为宿主适配(Gap,已标注)。
- 结论:门与哨兵/水位/距离/写出**与章节一致**;光标门待补或取证。

### 2. deferred_terrain_history @0x140ac9770 — OK(状态机)/待取证(字段)

- 哨兵 `0x8000000000000002` = i64::MIN+2 ✓(章节)。
- 每帧置哨兵;上一状态非哨兵才构建 TerrainEdit(pending 4 qword + previous)→ History::add +
  DebugHistoryMsg;之后不重复 ✓(章节状态机一致)。
- `EditType::Terrain` native_tag **占位 14 待取证**(原 0xd=AutoClutter,0xe=Wall 等之后地形
  tag 需 RNE 串表核对)。
- TerrainEdit 0x38 B 字段语义 U(已标注)。

### 3. ui_draw_terrain @0x140a9fdc0 — PARTIAL(门 + 生产面,消费面缺)

- 门 `mode.native_tag() != 10 → return`(AppMode::Terrain)✓。
- 光标世界位置 + 悬停 → `new_stroke` + `EvTerrainStrokeChanged` ✓(生产面)。
- **缺**:章节消费面 Res<AssetSsboLibrary, WaterRaster, UiState, GladeBorder> 的高度采样门
  (FUN_140a14df0 bilinear,130.0×2/0x100 常量)、"高度≤0 → 不注册画布"、界外反馈
  (DisplayOutOfBorder/FeedbackInputOutOfBorder)、UI 画布 widget 注册(名字 hash、UiState::add)。
  当前以 TerrainHoverInput 适配,真实 UiState 接线缺。

### 4. ui_edit_terrain @0x140aef1f0 — PARTIAL + **FIX(点集路径死代码)**

- 门 mode==10 ✓;MouseActivityTimer 维护 ✓;缓存比较 → EditTerrainStrokeCmd ✓(结构)。
- **FIX**:`DIRTY_MASK_POINTS` 从未被置位(`dirty` 只 OR 入 HEIGHT/FALLOFF),下方
  `if dirty & DIRTY_MASK_POINTS != 0` 的 `point_sets.write` 是**不可达死代码**。
  原 16 位脏掩码含点变化位;当前实现不比较 points,`SetTerrainStrokePtsCmd` 永不发出。
  修法:比较 `stroke.points != 上帧缓存 points` 时置 POINTS 位并发出点集命令。
- 原事件组(CursorCaptureCmd/EnableArrowGizmoStretch/AddTerrainHighlightCmd)、widget
  (add<String>@0x1409231f0)、130.0×2/0x100 常量、panic@0x142ae3acf(行 370)待取证(已标注)。

### 5. ui_terrain_focus_mode @0x1409a4fb0 — GAP(仅门存根,主体未恢复)

- 当前 = `mode==1` 门 + 空 trace 日志。原 171KB 近全内联状态机(4 闭包:0x9a3620 曲线采样、
  0x9a8640/0x9a88c0/0x9a8b40 状态转换,OutlineWallCmd/PublicWalls 相关)全部缺失。
- 属真实 GAP(依赖 wall 模块,restorer 已注明"待取证")。**不可**把门存根计作函数闭合。

## 结论与对 DISPATCH 建议

- 0x421 门、deferred 哨兵状态机、ui_draw/ui_edit 门+生产面核对通过;既有 3 个 CPU 系统
  仍 OK。
- 需 FIX:ui_edit_terrain 点集路径死代码;terrain_hover_idx 光标门。
- 需 re-analyst 补挖:EditType::Terrain 真 tag、ui_terrain_focus_mode 状态机主体、
  ui_draw 消费面、ui_edit widget/常量。
- 建议:状态 `in_progress`,待 FIX(点集路径)+ re-analyst 补挖后再交复验;ui_terrain_focus_mode
  单独标 GAP(不应计入本批闭合)。
