# REVIEW — R6+R7（组 state：roofs 视觉 + stairs 结构/视觉/replay）终验

> 审查员：reviewer-state，2026-10-16。复核 restorer-roofs-stairs 的 R6+R7（RESTORE-R6R7.md）。
> **编译：`cargo check -p country-core --offline` 通过（零错误）。**
> **测试：R6/R7 模块自带单元测试 5 例失败**（roof_visual_shape×1 / roof_ridge×1 / structural_graph×1 /
> replay_stairs×2），与 RESTORE 声称"逻辑经临时集成测试验证后删除"矛盾——临时 harness（已删）通过
> 不代表**保留的单元测试**通过。全 crate 当前 1703/7（geometry::sphere 既知 + wall 组在途 3 + 本模块 3~5）。

## 逐项核对

### R6：roof_visual_shape / roof_ridge
| 项 | verdict | 说明 |
|---|---|---|
| get_height_01_unclamped_from_ws（0x1408e4790） | **FIX** | 公式实现；但 `flat_roof_height_is_zero_cap` 测试失败（159 行 `h.is_finite()`）——flat roof（height_01≤0.1 阈值、cap=0、floor=0）时分母 `(cap+floor)-floor`=0 → 0/0=NaN。**待裁决**：原版 0x1408e4790 是否确产出 inf/NaN（若然则测试断言错；若否则实现需处理 0 分母）。需对 decomp 做定点核对并修正 实现或测试。 |
| get_roof_shape_from_pos_ws_clamped / get_roof_shape（0x1408e4750/0x1408e4920 委托 system_roof） | OK | clamp → section 委托共享。 |
| get_roof_expanded_top_and_bottom（0x140a880c0） | **OK** | 常量（0.28/eave_ws 1.5/圆 1.2×+0.5/顶 0.2/脊内缩 (1−ridge)·0.1）与反编译一致；测试 expanded_rect/circle 通过。 |
| RoofBoundsDims 纠偏 {tag,bottom,top}（核对点①） | **OK** | Circle{bottom_radius,top_radius}/Rectangle{bottom,top}；纠正旧章"圆心/半径"误读；as_circle/as_rectangle 手解一致。 |
| roof_ridge 7 业务（委托 system_roof::ridge，14,776 案例） | **FIX** | 主链 4 测试通过（world_space/from_rect/from_circle/from 轴）；但 `inverse_ridge_clamps_and_rejects_nan` 失败（113 行 `ok > 1.0`）。需核对 inverse_ridge_01_param 的夹取/NaN 语义（NaN→panic 是否应为 unwrap 而非夹取）。 |

### R7：structural_graph / visual / removed_supports / stairs_walls / replay_stairs
| 项 | verdict | 说明 |
|---|---|---|
| StairsStructuralGraph 7/7（clear/upsert_span/upsert_support_source/add_edge_by_span|collider/remove_node_by_collider/disconnect_node_by_collider） | **FIX** | 结构实现全（表分布 +0x90/+0xc0/+0xf0/+0x150；StairColliderHandle {index,generation,slot} 排序 {slot,index,generation}）；但 `disconnect_keeps_node_removes_edges` 失败（145 行 add_edge_by_span 的 `expect("edge index entry exists")` panic）——edge_index 未预置。原版"边索引表命中写 0，未命中 panic"，实现忠实，但**测试未预置 edge_index**（或 add_edge_by_span 的 expect 语义需复核）。 |
| visual.rs / stairs_collider / stairs_removed_supports（HashMap<u32,u32>）/ stairs_walls（StairsPillarProposals::add 0x140901270） | **OK** | removed_supports record/erase/contains 与 replay_stairs remove_entry 语义一致；pillar add 反编译一致。 |
| replay_stairs（0x140af75b0） | **FIX** | 结构实现全（前桶 tag 过滤/动画门控/undo 分支/decorator 过滤/音频/逆序移除）；**2 测试失败**：①`redo_records_and_undo_erases_support_edge`（130 行 buffer.is_empty()）——处理后仅移除 bucket.commands，bucket 仍在（pop_empty_replay_bucket 未纳入测试）；②`animation_gating_waits_a_frame`（170 行）——门控"先判后加 dt"使第 3 帧 elapsed=0.4<0.5 仍未就绪，需第 4 帧；测试期望第 3 帧（ceil 语义）。**待裁决**：原版门控是"先判后加"还是"先加后判"（off-by-one），以及完成命令后 bucket 是否由 replay_stairs 直接弹出。 |
| ReplayCommand::Stairs 共享 infra 变体（核对点⑥） | OK（登记） | replay_cmd_buffer 最小增改（expand EditType::Stairs → before−after 差集边 + make_animations_instant）；归属已由协调者确认，报告中显式标注。 |
| U 项登记（②+0x120/+0x180/+0x48 类型 / ④StairsReplayCmd 时长/下限原值 / ⑤完成命令移除索引语义） | OK（诚实） | 全部标注 U，未伪造。 |

## 结论与 DISPATCH 建议
- **R6+R7 → fix_pending（非闭合）**。主实现结构完整且多数核对通过，但**模块自带单元测试 5 例失败**，
  与"验证后删除临时测试"声明不一致。需 restorer 对 5 例逐一定点（实现 bug 或测试 bug 以原机器码为准）：
  1. roof_visual_shape flat 0/0→NaN（实现 vs 测试断言）；
  2. roof_ridge inverse 夹取/NaN 语义；
  3. structural_graph edge_index 预置（测试 setup 或 expect 语义）；
  4. replay_stairs 门控 off-by-one（先判后加 vs 先加后判）；
  5. replay_stairs 完成后 bucket 弹出归属（replay_stairs 直接 pop 或依赖 pop_empty_replay_bucket，测试补该系统）。
- 修复后重跑本模块 + 全 crate，绿窗口通过再交复验。
- roofs/stairs → 保持 `partial_implemented`；完整 core 验收仍 blocked（视觉 RtWorld/mesh 接线、
  replay_pop_bucket 完整楼梯 diff 应用、全局调度）。
