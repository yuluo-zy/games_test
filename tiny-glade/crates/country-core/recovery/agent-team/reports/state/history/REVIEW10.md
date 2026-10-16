# REVIEW10 — wall_compound_edit 完整族（组 state 卡R3 终验）

> 审查员：reviewer-state，2026-10-16。复核 restorer 卡R3 落地（CompoundEditBuilder +
> panic_deprecated_operation + 操作变体族登记）。
> 编译基线实测：`cargo check -p country-core --offline` **零错误**（6 警告，含他组在途）；全 crate 测试
> **1671/1**（唯一败 geometry::sphere::tests::uv_formula_matches_the_closure，components-geometry 组，
> 既知）。

## 逐项复核

### 1. CompoundEditBuilder（recovered_building/wall_history_edit.rs）
| 函数 | verdict | 说明 |
|---|---|---|
| `new_edit_sequence` 0x140a82b20 | **OK** | 开始新编辑序列（started=true + current.clear）。 |
| `next` 0x140a82b40 | OK | 非空当前组 push 进 groups 并取走（std::mem::take）。 |
| `build` 0x140a82d50 | **OK** | 先 next()，未 started 或 groups 空 → `None`，否则 `Some(WallCompoundHistoryEdit)`。 |
| `next_empty` / `with_concurrent` | OK | 当前组判空 / 加入并发组（0x140a82b2x 内联族）。 |
| 章节一致性 | OK | 地址注释（new_edit_sequence/next/build）与章节 wall_compound_edit.rs.md O 一致；`groups: Vec<Vec<..>>` 不拍平（ordered ConcurrentEdits groups）。 |

### 2. panic_deprecated_operation 0x140a82e20
| 项 | verdict | 说明 |
|---|---|---|
| 已弃用操作 panic | **OK**（文案待取证） | `panic!("attempted to use a deprecated wall history operation")`；注释明确"原文案待取证"。诚实。 |

### 3. WallHistoryEdit 操作变体族登记
| 项 | verdict | 说明 |
|---|---|---|
| 4 结构变体（Create/Delete/EnclosureChange/StateChange） | **OK** | StateChange 含 `observed_operation: [u8; 12]`（操作 tag + 两 float 原字节），注释"由真实 replay 消费者解释，不从序列化枚举名猜测"——**不伪造操作载荷**。 |
| ~40 操作变体（ChangeFreehandHeightType/PlasterPeelAmount/…） | GAP（登记） | 逐变体名/载荷待 E1（re-analyst 40 变体映射）落地后吸收；当前未扩变体，无假载荷。 |
| serde（deserialize 0x141401960 ~40 visitor / serialize 0x140314a00）+ EditType tag 对齐 | GAP（登记） | 待 E1 变体名落地 + MultiEdit deserialize 跳表 0x142aee31c 对齐（卡C 残留）。 |

## 结论与 DISPATCH 建议
- **卡R3 → 闭合（结构变体 + Builder）**。CompoundEditBuilder 三函数 + next_empty/with_concurrent 与
  章节地址一致；panic_deprecated_operation 落地（文案待取证）；4 结构变体覆盖，操作变体不伪造。
- 残留登记（不阻塞）：~40 操作变体逐名/载荷 + serde/tag 对齐归 E1（re-analyst 40 变体映射）落地后补。
- 模块状态建议：history → **review_passed（限定已实现范围）**；完整 core 验收仍 blocked
  （E1 40 变体映射 + 全局调度 + 各域系统批量并入）。
