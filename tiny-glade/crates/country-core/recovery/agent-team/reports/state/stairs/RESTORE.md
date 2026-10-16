# RESTORE — resources/stairs（卡A4，组 state，stairs 收尾核对）

> restorer-state，2026-10-16。核对现状 + 缺口登记；未做大件=诚实预算判定（同 A2/A5/A3 原则）。

## 现状核对（inventory ↔ src ↔ 章节）

| inventory 文件 | src 现状 | 覆盖 | 缺口 |
|---|---|---|---|
| resources/stairs/stairs_state.rs（58 函数/38 业务） | **EXISTS**（366L） | **权威图/快照/差分链全核**：StairsState（add_node/add_edge/disconnect_edge/remove_node/get_all_connected_nodes/contains_edge_or_stub/collect_edges/drain_dirty_edges + endpoint 设置）+ 备份/恢复（make_backup/restore_backup/clear_backup）+ 快照（take_snapshot/StairsSnapshot/compute_checksum）+ 差分（extract_diff_and_make_backup/reduce_diff/apply_diff/StairsHistoryDiff）+ TransientStairEdgeId/Allocator | 逐函数 38 业务大多覆盖；剩余 serde/derive 胶水不计业务 |
| resources/stairs/stairs_assembly_params.rs（16/8 业务） | **EXISTS**（112L） | StairsAssemblyParams 组装参数 | — |
| resources/stairs/invalidate.rs（3/1） | **MISSING** | — | 缺（失效标记；行 invalidate） |
| resources/stairs/stairs_collider.rs（1 胶水） | **MISSING** | — | 缺（碰撞组件；标准 required-components 胶水） |
| resources/stairs/stairs_removed_supports.rs（9/1） | **MISSING** | — | 缺（移除支撑边集；replay_stairs undo 消费） |
| resources/stairs/structural_graph.rs（8/7） | **MISSING** | — | 缺（结构图；可复用 stairs_state 图操作） |
| resources/stairs/visual.rs（11/5） | **MISSING** | — | 缺（视觉/网格生成） |
| resources/stairs_walls.rs（5/1） | **MISSING** | — | 缺（墙-楼梯衔接） |
| resources/history/stairs_history_edit.rs（12/2） | **EXISTS**（卡C 新增：StairsHistoryEdit/Diff + StairsEdge=(u32,u32)） | ✓ | — |
| systems/history/replay_stairs.rs（3/1） | **MISSING** | — | 缺（undo 插边集 StairsRemovedSupports/!=0 remove_entry + PlayStairsAudioCmd；卡C 登记阻塞） |
| systems/audio/playermade_stairs_audio / terrain_stairs_audio | 缺 | — | audio 组在途 |
| systems/render_loop/* stairs | 缺 | — | render 组在途 |

## recovered_building 算法复核边界

- **wall_platform_stairs.rs**（recovered_building）：平台楼梯生成/物化（StepRange 局部于前掩码、
  manual-stair blockers 移除水平邻域；共享物化与墙实例生产者属共享 owner）——**生成/物化算法
  在共享 recovered_building，不重挖**。
- **stairs_state.rs** 是 country-core 权威运行时图（petgraph UnGraphMap<u32,()> + endpoint +
  快照/差分/备份链）；与章节 stairs_state.rs.md 一致。
- 视觉/碰撞（visual.rs/stairs_collider.rs）与音频/渲染衔接为缺口（其他组在途或未做）。

## 编译/测试状态

- `cargo check -p country-core --offline`：零错误；我的文件零警告。
- 全库 `cargo test --lib`：**1634/1**（唯一失败=pre-existing geometry/sphere 浮点精度）。

## 合同/状态建议

- inventory：stairs → `partial_implemented`（权威图/快照/差分链全核；缺 invalidate/
  stairs_collider/stairs_removed_supports/structural_graph/visual/stairs_walls/
  replay_stairs——视觉/碰撞/音频衔接缺口）。
- DISPATCH：卡A4 交 reviewer 终验。
