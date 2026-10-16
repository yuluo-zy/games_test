# REVIEW4 — systems/terrain(终验)

审查员: reviewer-interaction | 日期: 2026-10-15
基线: restorer 终轮落地 + RE-ANALYSIS2;`cargo check --lib` 0 error。

## 终验核对

1. **0x421 门(bits{0,5,10})**:terrain_hover_idx `disc<0xb && (0x421>>disc)&1`,位 0/5/10 =
   All/Garden/Terrain ✓(与章节/机器码一致,三轮未变)。
2. **ui_edit 点集死代码修复**:points_stamp 指纹 → DIRTY_MASK_POINTS → SetTerrainStrokePtsCmd
   可达(三轮 PASS,未变)。
3. **ui_terrain_focus_mode 行为骨架**:已从"仅门存根"提升为门(mode==1)+ raycast 距离门
   (f32::MAX 未命中 / >0.5 阈值)→ TerrainFocusTrigger + Replay 哨兵 0x8000000000000002 +
   0.5 阈值常量。对应 RE-ANALYSIS2 的 raycast_simple@0x1408ce180 行为;**内联 switch 表 U
   已如实标注为卡点**(167KB 内联体未拆)。
4. **待取证项登记诚实**:EditType::Terrain 真 tag(占位 14)、focus_mode switch 表、ui_draw
   消费面(高度采样门/UI 画布 widget/界外反馈)、16 位脏掩码位序、EditCommand 字段序——
   全部在 RESTORE 登记,无冒充。

## 结论与 DISPATCH 建议

- 已实现主体(terrain_hover_idx 门/哨兵/水位/距离、deferred_terrain_history 哨兵状态机、
  ui_draw/ui_edit 门+生产面、focus_mode 行为骨架、既有 3 个 CPU 系统)与章节/RE-ANALYSIS2
  **对得上**;待取证项诚实登记。
- 建议:terrain 模块给 **review_passed(限定已实现范围)**。边界明确:EditType::Terrain 真
  tag、focus_mode 内联 switch 表、ui_draw 消费面、16 位脏掩码位序、EditCommand 字段序
  不在本次验收内,由 re-analyst 补证/restorer 补实现后单独闭合。
