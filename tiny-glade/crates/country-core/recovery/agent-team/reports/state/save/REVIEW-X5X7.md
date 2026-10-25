# REVIEW — X5+X7（组 state：快照收集骨架 + 新会话命令数组）终验

> 审查员：reviewer-state，2026-10-16。复核 restorer X5（snapshot 链）+ X7（run_game_thread 3 命令）。
> 编译基线实测：`cargo check -p country-core --offline` **零错误**（6 警告，含他组在途）；全 crate 测试
> **1709/1**（唯一败 geometry::sphere，既知）。

## 逐项复核

### X5 snapshot.rs
| 项 | verdict | 说明 |
|---|---|---|
| `SnapshotTreesEntry` + `collect_trees` | **OK** | Trees = Query<(&PrefabInstanceData,&PrefabInstance,&TreeComponent)> → `(TreeStorageIndex, TreeData)`；`TreeData` 以 `tree_edit::TreeEditData`（id/position/scale/rotation/prefab_name/is_preplaced）登记，原类型待域恢复（诚实）。 |
| 节序 Version→…→Raster | **OK** | SNAPSHOT_SECTION_ORDER 与 RNE 键串一致（0x1417a6e50）。 |
| 各节域序列化器（Walls/Roofs/Decorators/三色/Terrain） | GAP（登记） | 留待域恢复（capture_game_snapshot 0x1417a1d00 184KB 域 Query）。 |
| capture_game_snapshot_save_data 9 组收集器 → ~0x400B → Arc | OK（骨架） | Arc 语义既有；收集器骨架登记。 |
| init_game_state_from_snapshot 写回链（FUN_14004f570 + 各节 helper） | GAP（登记） | 字段映射残留待域恢复。 |

### X7 run_game.rs
| 项 | verdict | 说明 |
|---|---|---|
| `new_session_on_load_commands` | **OK** | cmd2 = 变体 8（0x8000000000000008，0x1405a340c 直写），载荷三槽 = [rbx+0x2230] 16B / [rbx+0x2240] 16B / rdi（RNE 原字节一致）。 |
| `build_new_session_on_load` | **OK** | 压入 3 命令 + `append_replay_terminator`（回放终止哨兵 0x8000000000000003）。 |
| cmd0/cmd1 tag/逐字段载荷 | GAP（登记） | 需 ReVa p-code 定点（残留），由调用方按已定位源字段传入，不伪造。 |

## 结论与 DISPATCH 建议
- **X5+X7 → 闭合（快照收集骨架 + 新会话命令数组）**。Trees 收集器/节序/变体 8 命令与 RNE 原字节一致；
  终止哨兵接线正确。
- 残留登记（不阻塞）：各节域序列化器 + init_game_state_from_snapshot 写回字段映射（域恢复）、
  cmd0/cmd1 ReVa p-code 载荷。
- save → **review_passed（限定已实现范围）**；完整 core 验收仍 blocked（域资源收集/组装 + 快照域 Query +
  全局调度 + 各域系统批量并入）。
