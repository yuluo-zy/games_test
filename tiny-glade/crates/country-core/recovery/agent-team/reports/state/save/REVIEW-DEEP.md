# REVIEW — impl-save-roofs-deep 复验（组 state）

> 审查员：reviewer-state，2026-10-17。复核 impl-save-roofs-deep（REBUILD-DEEP.md + 14 份 fresh 证据）。
> 编译基线：`cargo check -p country-core --offline --lib` **通过**（9 警告，含他组在途）。

## 逐项复核

| 项 | verdict | 说明 |
|---|---|---|
| capture_game_snapshot_save_data 收集器（0x198B/backup_accurate/~9 域） | **OK** | 0x198B 结构（fresh 0x1417a0430 memcpy 0x198）；`assert!(res.stairs.backup_accurate())` + `_DEV_SKIP_TREE_SERIALIZATION` 门控收集 Trees。 |
| SnapshotDomainData::sorted() 原生排序序 | OK | sorted 落原生排序序。 |
| is_known_raster_key 真白名单（ordered_names 6） | **OK** | 由 `AllWorldRasterWrite::ordered_names()` 实现；validate 对 Raster 节白名单保留/未知忽略。 |
| Terrain deflate 纠错 | **OK（纠偏）** | Terrain 节 JSON 明文 strokes（flate2 仅栅格尾，TerrainSectionJson）——修正旧"Terrain 全节 deflate"。 |
| try_chunk | OK | 分块读取（round4 已核）。 |
| init 写回登记 | OK（登记） | 域写回链待域。 |
| 内联快照捕获链 → capture_snapshot_system | OK | 快照捕获系统承载（域采集面宿主喂入）。 |
| add_game_startup_systems = 胶水 | OK | 注册入口胶水（round7 已核）。 |
| load_glade_audio 判等 | OK | 既有常量/判等。 |
| **⚠️ 备份机制关键纠错** | **OK（纠偏）** | `parse_saved_game_data` **无备份**（fresh 0x141470d00 无 copy_file_to_backup）；备份实为 `load_game` 0x14146d030 行 681 `copy_file_to_backup`（0x140c96a80）→ `generate_backup_file_path`（0x140c968b0）= `{源文件名}-{YYYY-MM-DD HH-MM-SS}.json`（同目录）；旧 `backup-saves/history-recovered-{ts}.json` 目标名已弃；调用已移 `load_game_system`（kind==1 parsed 路径）。**round1 曾核的旧 make_upgrade_backup 目标名/位置按 fresh 作废**。 |
| roofs：wall_platform_stairs×5 vs B-platform-stair | OK | 共享 recovered_building 公式逐条一致（round8/12 边界维持）。 |
| roof_tiles/dormer/buffers 胶水合法 | OK | glue_legitimate（round12 已核）。 |
| 剩余宿主接线面登记 | **OK（诚实）** | 域 Query 全链 section_values / History 资源 / Schedule 28 槽 / R1 碰撞注册表 / init Stairs-Raster 写回。 |

## 结论与 DISPATCH 建议
- **save+roofs 深收口 → 闭合（实现层面）**。快照域 5 项 + 备份机制关键纠错（目标名/位置）与 fresh
  decomp 一致；roofs 平台楼梯公式与共享 evidence 一致；胶水合法。
- 残留登记（不伪造）：域 Query 全链 section_values、History 资源、Schedule 28 槽、R1 碰撞注册表、
  init Stairs-Raster 写回。
- save/roofs → **review_passed（限定已实现范围）**；完整 core 验收仍 blocked（外部宿主接线 + 全局调度）。
