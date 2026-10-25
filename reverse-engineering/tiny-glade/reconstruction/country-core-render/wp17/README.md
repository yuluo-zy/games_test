# WP17 — country-core 历史回放域（history replay domain）恢复

B 版本（EXE sha256 `cf048e27…`，PDB sha256 `2d9686b8…`）。方法遵循项目工作流：
**(1) 分析业务逻辑（反编译 + 调用链 + PDB 骨架证据）→ (2) 整理完整业务逻辑描述 →
(3) 按源码骨架实现 Rust**。机器码差分仅裁决关键数值/边界歧义，不作为实现手段。
本包不 patch 共享 `reconstruction/country-core` crate；交付独立门禁，由集成 owner 整合。

## 交付内容

| 类别 | 路径 |
|---|---|
| 业务合同 | `contracts/{replay-orchestration,replay-systems,edit-history-types}.md`（O/I/U + typed gap） |
| 回放系统实现 | `src/systems/history/{replay_mode, finish_replay, signifier_replay_active, replay_pop_bucket, replay_wall, replay_terrain, replay_auto_clutter, replay_stairs, replay_raster, replay_path, replay_water, replay_garden}.rs` |
| 编辑历史资源实现 | `src/resources/history/{replay_cmd_buffer, wall_history, wall_compound_edit, auto_clutter_edit, color_edit, decorator_edit, path_edit, stairs_history_edit, terrain_edit, tree_edit}.rs` |
| 独立 crate 门禁 | `Cargo.toml` + `src/lib.rs`（offline `cargo check` 0 错误 / 0 警告） |
| 证据 | `evidence.json` + `evidence/`（3 批 Ghidra job/result + 逐函数反编译 `.c`） |

## 恢复要点

### 回放编排（replay-orchestration.md）
- **ReplayCmd / ReplayBucket / ReplayBuffer**：ReplayCmd 0x30B 枚举
  （判别 0/1/2 RasterEdit、3 Terrain、4 Wall、5 Roof、6/7 Decorator、8 Tree、
  9 AutoClutter、10 Stairs；动画头 instant/duration/elapsed/undo）；
  ReplayBuffer 环形缓冲（head/buf/cap/len/counter；pop_front 0x1409f18f0）。
- **replay_edit 0x140af6170**：把 MultiEdit.edits 经
  `generate_replay_cmds_from_edit` 展开为 ReplayCmd，undo 时反转命令列表，
  按判别分组为 ReplayBucket（附装饰 diff + cull allow-list + 楼梯 diff），
  push 到 ReplayBuffer。
- **replay_mode 0x140a7c550**：AppMode==Replay 时按输入动作发 redo HistoryCmd，
  `RestoreAppTo`（set/get）记回放前模式。
- **finish_replay 0x1408b36a0**：pending 空 && ReplayBuffer 空 → 恢复 AppMode、
  `Toolbar::switch_mode`、MetaCommander 哨兵日志；pending==1 redo → 发
  `CmdUnfreezeSheep`。
- **replay_pop_bucket 0x140b38250**：队首空桶弹出 + `restore_decorators` /
  `StairsState::restore_backup/apply_diff` / `RaycastWorld::flush_colliders`。
- **signifier_replay_active 0x140b1b860**：`ReplayModeElapsedTime`（active==1）、
  `SignifierReplayIndex` 组件、`ease_out_elastic_jiggle` 弹性抖动。

### 领域回放系统（replay-systems.md）
- replay_wall / replay_terrain / replay_auto_clutter / replay_stairs：
  消费前桶对应判别命令，instant 立即应用 / animated 按 duration+GameTime
  累加，完成索引移除；空桶交由 replay_pop_bucket。
- replay_raster 泛型系统：Path/Cobblestone1..3/Garden/Water 六实例
  （0x1409febf0/0x1409fc290/0x1409f1cf0/0x1409f9930/0x1409f6ff0/0x1409f4650）；
  undo 清缓冲重绘存储 stroke，redo rasterize_stroke + 事件，
  `DirtyTiles::rasterize_world_space_stroke` 写 dirty tiles。

### 编辑历史资源（edit-history-types.md）
- wall_history（ev_new_wall / ev_height_changed → EditType 判别 7）、
  wall_compound_edit（WallHistoryCompoundEdit + CompoundEditBuilder）、
  path_edit（RasterEdit）、terrain_edit（TerrainEdit revert/reapply/
  update_terrain_curve）、decorator_edit（DecoratorHistoryDiff +
  send_state_diff_events）、stairs_history_edit、tree_edit、auto_clutter_edit、
  color_edit —— 字段以 `reconstruction/history/observed-history-schema.json`
  观察模型为准（开放枚举，U 一律 typed gap）。

## 已复用边界
- wp13 的 `History` 核心（edit_chain/snapshots/pending/cursor/history_start/
  nonce）、`EditType`、`MultiEdit`、`HistoryMutationToken`、history_process
  undo/redo 游标规则（`systems/history/boundary.rs` facade）。
- `reconstruction/history/observed-history-schema.json` + findings.md +
  transaction.rs + `crates/tg-history`。

## 已知缺口（typed gap）
- ReplayCmd 判别 9/10+ 精确负载；ReplayBucket 字段偏移；HistoryCmd
  0x0300xxxx 命令字语义；WallReplayEditCmd 分派跳表子过程；编辑记录字段
  开放枚举；bevy SystemParam 接线（集成 owner）；四个无 Ghidra 函数体的
  目标（RasterReplayCmd forward/backward、generate_path_replay_cmds、
  send_state_diff_events）由调用侧推断。

全部 `implemented_unverified`：无 runtime/回放执行验收；合同是当前可审阅依据，
不因编译通过声称原作等价。
