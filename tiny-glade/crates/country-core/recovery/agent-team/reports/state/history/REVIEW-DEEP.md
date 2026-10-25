# REVIEW — impl-history-deep 复验（组 state）

> 审查员：reviewer-state，2026-10-17。复核 impl-history-deep（planning/impl-history-deep/REBUILD-DEEP.md，
> 14 份 fresh 证据 + 两表 read_memory + ReVa 定点）。
> 编译基线：`cargo check -p country-core --offline --lib` **通过**（10 警告，含他组在途）。

## 逐项复核

| 项 | verdict | 说明 |
|---|---|---|
| replay_wall 17-op（0x142adf408/0x142adf44c 两跳表各 17 项，目标全解） | **OK（登记）** | 瞬时/定时双表 read_memory 各 160B 全解；op 号 = niche 解码；真业务分类；**登记**（需 recovered_building WallHistoryEdit 全变体，红线不改他模块；wall_history_transition 已实现 op 0/1/2）。 |
| replay_pop_bucket / DecoratorHistoryState::from / replay_edit Color-Decorator 展开 | **OK（登记）** | 跨模块登记（需 DecoratorInfo+0x41 style 字节、Archivist extract_diff、RaycastWorld::flush_colliders）——诚实不实现他模块业务。 |
| ColorEdit 8 类载荷 | **OK（结构对齐）** | fresh 核实 serialize 逐 entry 类型序（WallId/TerrainStrokeId/FlatRoofStyle/DecoratorId/bool/f32/StairRailingType/StairSupportType）+ visit_str 14 键名；src 平面模型维持 target 键 + 平面字段子集结构对齐 serde，互异载荷类型面 U 登记。 |
| history_process case 0xb+ | **OK（部分，U 诚实）** | case 0-3 已实装；0xb..0x18 halt_baddata（0x140af7d6a 指令重叠）ReVa 不收敛标 U。 |
| get_param @+0x58/+0x5c 载荷语义绑定 | **OK** | 4 组 × 0x20B `{data,p58,p5c,flag,u1c}` 布局 + 稀疏表槽 0x60B 取参 + panic 文案逐字（GameTimeResource/History/AssetSsboLibrary/ReplayBuffer）。 |
| 磁盘清理（15G incremental）+ rhapsody 在途 E0599 | OK（注明） | 环境/工作区备注，非业务。 |

## 结论与 DISPATCH 建议
- **history 深收口 → 闭合（实现层面）**。replay_wall 17-op 跳表全解登记、ColorEdit 8 类载荷 fresh 定案、
  get_param 载荷绑定、case 0-3 实装；跨模块项诚实登记。
- 残留登记（不伪造）：replay_wall 17-op 应用（需 WallHistoryEdit 全变体）、replay_pop_bucket/
  DecoratorHistoryState::from/replay_edit 跨模块、ColorEdit 互异载荷类型面、第二事件流 0xb..0x18。
- history → **review_passed（限定已实现范围）**；完整 core 验收仍 blocked（跨模块 + 全局调度 +
  rhapsody 收口后重验全 crate 绿窗口）。
