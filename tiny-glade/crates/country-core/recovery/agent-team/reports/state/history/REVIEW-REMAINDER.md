# REVIEW — impl-history-remainder 复验（组 state）

> 审查员：reviewer-state，2026-10-17。复核 impl-history-remainder（REBUILD-REMAINDER.md）。
> 编译基线：`cargo check -p country-core --offline --lib` 通过；**全 lib test 目标当前被
> workspace 兄弟 crate `rhapsody` 在途 E0004 打断**（非 country-core 文件），故本批无法独立重跑
> 测试计数（restorer 记录的 history 5/5、replay_wall 3/3、replay_stairs 2/3 待 rhapsody 收口后复核）。

## 逐项复核

| 项 | verdict | 说明 |
|---|---|---|
| StairsHistoryEdit 重载单边枚举 {SupportsDelete/SupportsRecreate(StairGraphEdge)} | **OK** | 判别字节 bit0（TEST [RCX],0x1）；手写 serde `{"<变体>":{"edge":<StairGraphEdge>}}`（键串 0x1429a4c82/90/94）；expanded 单编辑→单命令。旧 diff-Vec 模型证伪已替换；运行时差集在 stairs_state。 |
| missing_prefab | **OK** | `"MISSING_TREE_PREFAB_NAME"`（0x1409e6210 三 qword 拼装）。 |
| get_param（0x140ae5830） | **OK（载荷面 U）** | `RasterReplayParams` 4 资源（GameTime/History/AssetSsboLibrary/ReplayBuffer）结构对齐；4 索引稀疏表判缺失 panic 语义保留；@+0x58/+0x5c 载荷 U 登记。 |
| history_process 第二事件流（0x142ae5420） | **OK（部分）** | `process_history_control_stream`：case 0-3 可解码（条件清队/`make_animations_instant`+写事件+清队）；产生端 U（历史控制命令上游未接线）登记；0xb..0x18+ 跳表分支 U。 |
| 登记 5 项诚实 | **OK（登记）** | DecoratorHistoryState::from 需 DecoratorInfo+0x41 布局；replay_pop_bucket 需 aux diff+archivist+RaycastWorld::flush_colliders；replay_wall 17 操作；replay_edit Color/Decorator 展开；ColorEdit 8 类载荷表 0x14292c628 fresh 定案。 |

## 结论与 DISPATCH 建议
- **history 余量 → 闭合（实现层面）**。单边枚举/手写 serde/missing_prefab/get_param/第二事件流
  case 0-3 与 fresh decomp 一致；跨模块项诚实登记。
- 残留登记（不伪造）：get_param 载荷面、第二事件流 0xb..0x18+、5 项跨模块（decorator 布局/pop_bucket/
  replay_wall 17 操作/replay_edit/replay_wall 展开、ColorEdit 8 类载荷表）。
- **编译告警**：全 lib test 目标被 workspace 兄弟 crate rhapsody 在途打断（非 state）；建议主会话协调
  收口后重验全 crate 绿窗口（含本批 history/replay_wall/replay_stairs 测试计数）。
- history → **review_passed（限定已实现范围）**；完整 core 验收仍 blocked（跨模块项 + 全局调度）。
