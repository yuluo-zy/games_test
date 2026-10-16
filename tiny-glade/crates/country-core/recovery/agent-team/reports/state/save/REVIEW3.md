# REVIEW3 — systems/save（组 state 三轮快速复验）

> 审查员：reviewer-state，2026-10-15。复核 restorer 按 REVIEW2 FIX×2 的修正。

## FIX 吸收核对

| REVIEW2 FIX | 吸收情况 | verdict |
|---|---|---|
| extend_on_load 批量条目 tag=3（机器码 0x1403b5150） | `OnLoadEntry` 改为枚举 `Command{tag,payload}` / `Batch{commands:Vec<MetaCommand>}`；`extend_on_load` 入队 `Batch`（0x20B = {tag=3, Box<0xa8B>} 语义，注释引机器码）；`append_fence` 仍 `Command{tag=ON_LOAD_VARIANT_MASK, payload=ON_LOAD_FENCE}`。load_game 两遍回放按枚举分派：Command → fence 检查（`tag||payload == ON_LOAD_FENCE` 终止）+ 变体解码；Batch → 展开为 MetaCommand（一遍）/ Apply（二遍）。**与机器码的 tag=3 批量结构一致**。 | **OK** |
| fence 双 tag 登记（append …0000 vs 终止 …0003） | 回放以 `tag==FENCE || payload==FENCE` 终止：append_fence 条目（payload=FENCE）在变体解码前 break，不作为变体 0 处理；另可收任何 tag=0x8000000000000003 条目。双解释均登记，RE-ANALYSIS §5.6 残留合理收口。 | **OK** |

## 复核要点
- 变体 1（Command）回放仍产出混合 MetaCommand{tag: entry.tag, payloads:[payload,0×5]}——原版为 Box<[MetaCommand;3]> 6 qword 载荷；当前无该形态生产者，commander_update 未建前为诚实占位。PARTIAL（已披露）。
- 一遍变体 2（Arc unblock/drop）与二遍变体 2+ 仍 `_ => {}` 跳过。PARTIAL（已披露）。
- SaveBackupList 计数器口径：维持**待取证**，未伪造（分类已在 REVIEW2 机器级闭合）。
- 跨组卡点（域组装/快照 Query/commander_update/load_completion_checker 系统）不变，完整 save 验收仍被其阻塞。

## 编译基线（重要）
- **状态组文件内部一致**（OnLoadEntry 枚举 refactor 全仓 grep 无残留 `.tag`/`.payload` 直接访问；derive 满足 Vec<MetaCommand>）。
- **当前 `cargo check -p country-core --offline` 失败**：唯一 error = `systems/ivy/ivy_spawner.rs:242 unclosed delimiter`（eco 组并发编辑，mtime 22:20，非 state 文件）。因此本三轮无法独立重跑 16/16 测试与"全 crate 1584/3"基线；state 组修正的正确性已由代码一致性核对确认。**建议主会话协调 eco 组收口 ivy_spawner.rs 后重验全 crate。**

## 结论与 DISPATCH 建议
- **建议：save → review_passed（限定已实现范围）**。REVIEW2 两项 FIX 正确吸收；剩余为已披露 GAP/待取证（变体 1/2 形态、计数器口径、save_thread step4 drop、跨组域组装/commander_update/load_completion_checker）。完整 save 验收仍 blocked（跨组依赖）。
