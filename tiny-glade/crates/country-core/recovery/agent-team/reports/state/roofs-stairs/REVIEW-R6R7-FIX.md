# REVIEW — R6+R7 修复复验（组 state）

> 审查员：reviewer-state，2026-10-16。复核 restorer-roofs-stairs 对前轮 REVIEW 5 例失败的修复。
> 编译基线实测：`cargo check -p country-core --offline` 通过；R6/R7 模块测试 **14/14 通过**
> （含原 5 失败例）；全 crate **1709/1**（唯一败 geometry::sphere，components-geometry 组，既知）。

## 5 项裁决复验

| # | 原失败 | 修复 | verdict |
|---|---|---|---|
| 1 | roof_visual_shape flat 0/0→NaN | 测试改断 `!h.is_finite()`（flat unclamped 必须 inf/NaN，与原 0x1408e4790 平顶分母恒 0 一致）；clamped 入口出截面不 panic。 | **OK** |
| 2 | structural_graph edge_index 未命中 panic | 新增 `register_edge_index`（写 1 预置，填充源 U）；add_edge_by_span 未命中 panic 忠实 @0x142a96bd0；测试预置索引。 | **OK** |
| 3 | replay_stairs 完成后 bucket 未弹 | pop 归属独立系统 `pop_empty_replay_bucket`（Replay 阶段 replay_stairs → Pop 阶段 pop_empty 顺序弹空桶），接线确认。 | **OK** |
| 4 | replay_stairs 门控 off-by-one | `advance` 判式改 `elapsed <= required` 且**先判后加**（对齐 decomp `if (elapsed<=required){elapsed+=dt;}`）；测试改 4 帧 0.0/0.2/0.4→0.6>0.5。 | **OK** |
| 5 | inverse_ridge ok>1.0 断言 | 夹紧 [0,1]：`(5−0.1)/(4−0.1)=1.256→1.0`、下界→0.0、NaN→Err；测试更新。 | **OK** |

## 结论与 DISPATCH 建议
- **R6+R7 → 闭合（roofs 视觉 + stairs 结构/视觉/replay + 共享 infra Stairs 变体）**。5 例全部修复且
  以原机器码为准（0x1408e4790 平顶非有限、@0x142a96bd0 索引 panic、decomp 先判后加）；本模块 14 测试
  通过，全 crate 仅余既知 geometry::sphere。
- U 项维持登记（structural_graph +0x120/+0x180/+0x48 精确类型、StairsReplayCmd 时长/下限原值、
  完成命令移除索引语义、edge_index 填充源）。
- roofs/stairs → **review_passed（限定已实现范围）**；完整 core 验收仍 blocked（视觉 RtWorld/mesh 接线、
  replay_pop_bucket 完整楼梯 diff 应用、全局调度 + 各域系统批量并入）。
