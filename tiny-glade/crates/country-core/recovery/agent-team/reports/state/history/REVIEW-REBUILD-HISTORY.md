# REVIEW — rebuild-history 复验（组 state）

> 审查员：reviewer-state，2026-10-17。复核 rebuild-history（22 份 fresh 反编译）。
> 编译基线：`cargo check -p country-core --offline --lib` **通过**（10 警告，含他组在途）；
> 本模块 `cargo check --tests` 0 错误（全 lib test 目标 55 错误全在他组：
> backend/input/mod_key、asset_libraries、render_loop——**零个在 state 文件**）。

## 逐项复核

| 项 | verdict | 说明 |
|---|---|---|
| 分类：21 全真业务、0 宏胶水 | **OK** | serde 族（Color/Decorator/Raster/Tree/Stairs）均手写 `Serialize`/`Deserialize`（真实字段策略/状态输出），非 derive；依据 fresh decomp（rebuild-history/decomp/）。 |
| History::add tag-7 Wall 校验 | **OK** | `HistoryRecordingValidation` trait + `EditType` impl；`EditType::Wall` 逐 group/edit：Create/Delete source 缺失 panic "None is not allowed in `CreateWall`/`DeleteWall` history command recording"、StateChange op_tag<2 时 start>=end panic "Slice start_u must be smaller than end_u. Otherwise an invalid curve will be created!"（文案 0x142ad8118/8170/81c8）；no-op impl（RasterStroke/RoofRim/BuildingEdit）；新测试 4 项。 |
| replay_stairs 下限 0.05 | OK | `REPLAY_DURATION_FLOOR=0.05`（fresh `fVar44<=0.05 → 0.05`）；先判后加；与 round22 对齐。 |
| finish_replay 实装 | **OK** | RestoreAppTo 写回 AppMode + Toolbar::switch_mode（CloneClutter→Editing，原 +0x18=7）+ MetaCommander 哨兵 0x8000000000000001 弹栈 + log（回放/非回放两路径都查哨兵）。 |
| replay_mode 实装 | **OK** | HoldToFastReplay **阈值 0.2s**（秒制，旧 500ms 证伪）+ AppMode 进出 Replay 检测（进入→ClearCanvasCmd）+ 回放中 fast→HistoryCommand::REDO；time_continuous_lerp/TimeOfDayScreen 写回由宿主。 |
| RasterEdit | **OK（纠偏）** | points **Vec<Vec3>**（旧 Vec2 证伪）+ 字段序 points→is_additive→brush_size→blend_mode（旧序错）+ StrokeBlendMode 手写 serde + From<Arc<[Vec3]>>。 |
| TreeEditData | **OK（纠偏）** | id **i32** 且移至第 4 位（旧 u64/首字段两处证伪）；字段序 position→scale→rot→id→prefab→is_preplaced。 |
| ColorEdit / DecoratorHistoryEdit | **OK（结构对齐）** | 手写 serde；ColorEdit target 键单 entry map + 平面字段子集（互异载荷类型面 U 登记）；DecoratorHistoryEdit 单 entry map {"<id>":{before,after}}（__SerializeWith）。 |
| 未重建登记 | **OK（诚实）** | StairsHistoryEdit 原生单边枚举 vs diff-Vec 不同构（维持 derive，原生形状登记 U）；history_process 第二事件流（byte[+9] 0x142ae5420）；replay_wall 17 操作跳表；replay_edit/replay_pop_bucket 完整版。 |

## 结论与 DISPATCH 建议
- **history 需重做项 → 闭合**。21 全真业务分类确认；History::add tag-7 校验、finish_replay/replay_mode 实装、
  RasterEdit/TreeEditData 载荷纠偏、edit serde 手写——均与 fresh decomp 一致。
- 残留登记（不伪造）：StairsHistoryEdit 原生枚举形状、history_process 第二事件流、replay_wall 17 操作、
  replay_edit/replay_pop_bucket 完整版、ColorEdit 互异载荷类型面。
- history → **review_passed（限定已实现范围）**；完整 core 验收仍 blocked（外部依赖 + 全局调度 +
  并行组 in-flight 收口后重验全 crate 绿窗口）。
