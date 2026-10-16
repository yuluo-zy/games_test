# RESTORE — systems/save（组 state）

> restorer-state，2026-10-15。实现按"业务逻辑 → 骨架 → Rust"；关键逻辑标注证据
> （session-save/DIGEST.md、ALGORITHMS.md、chapters/systems/save/*、meta_commander.rs.md、
> re-analyst RE-ANALYSIS.md、reviewer REVIEW.md）。

## 本批新增实现

| 原函数/类型 | Rust 位置 | 证据 |
|---|---|---|
| `AsyncSaveCmd`（save_game.rs，mpsc 命令枚举） | `save_game.rs::AsyncSaveCmd`（3=OpenFile/4=CloseFile/5=ChangeSection/6=ReplaceEdits/7=BackupSave/8=ManualBackup/9=CaptureSnapshot/10=Flush/11=Shutdown） | ALGORITHMS §2.2 变体表；`should_trigger_debug_log` 原方法名 |
| `AutoSaveState`/`AutoSaveSectionState`/`DeferredJsonSerialize`/`GameSaveData`/`SaveGameCmd`/`DeleteSessionCmd`/`SavingGameComplete` | `save_game.rs`（Resource + 事件 + 载荷） | PDB structures（save_game.rs.structures.json.gz）；close_file 0x140a43840/create_backup_of 0x141e59860/last_error_state 0x140a439a0 |
| `save_game_to_file<Cursor/File>` | `save_game_to_writer`（字节布局与 save_game_to_memory 一致） | 0x140241ca0 / 0x141750c90 |
| `save_session_meta_to_file` | `save_game.rs::save_session_meta_to_file` | 0x1417522e0（tmp+sync+rename） |
| `add_history_edit_to_save_file`/`ensure_current_snapshot` | `save_thread.rs::CurrentSaveFile::add_history_edit_to_save_file` | 0x140a439b0（复用 ShadowBuffer::append_edit 节偏移协议） |
| `execute_save_cmd` 0x141721aa0 | `save_thread.rs::execute_save_cmd`（switch 3-11 + No-active panic/日志门控） | ALGORITHMS §2.2 |
| `save_thread` 0x141720680 主循环 | `save_thread.rs::save_thread`（1s tick flush、错误 diff/日志/回传、Shutdown flush 确认、OpenFile unsaved 前置 panic、10ms 等待） | ALGORITHMS §2.1 + decomp-0x141720680.txt |
| `execute_open_file_cmd` | `save_thread.rs::execute_open_file_cmd`（旧态 flush→全量重写→ShadowFile::create） | ALGORITHMS §2.2 case 3 |
| `CurrentSaveFile::flush_deferred_and_sync` | 0x14171f7f0（deferred_sections[10] → change_section → sync_all） | PDB `DeferredSection`/`[Option<DeferredSection>;10]` |
| `CurrentSaveFile::backup_save`/`perform_manual_backup` | "Copying inactive save file to backup"/"Writing backup from shadow copy" | ALGORITHMS §2.2 default/7 |
| `CurrentSaveFile::capture_snapshot` | snapshots 目录 + `<snapshot_id>.snapshot` 落盘 | ALGORITHMS §2.2 case 9 |
| `UnsavedFileList::add`/`contains_file` | 0x14171ff80 / 0x1417201a0 | ALGORITHMS §2.2 |
| `AutosaveErrorState::is_ok`（+channel） | 0x140b92210 | ALGORITHMS |
| `nuke_unreferenced_shapshots` | 委托 `start_snapshot_gc`（snapshot_gc_thread 0x14172b280） | PDB 名拼写即如此 |
| `try_approx_parse_save_file_identity`/`derive_fallback_save_file_identity` | `save_file_identity.rs`（0x800B 缓冲 + `"Identity"` 字面量搜索 + 元数据回退） | 0x141e58f20 / 0x141673140 |
| `SessionLoadReadiness::is_ok`/`deny_load`、`check_load_completion`/`is_session_ok_to_load` | `load_completion_checker.rs` | 0x140833230 / 0x140a9fa70 / 0x140a9f810 / 0x140a9fa80 |
| `SnapshotSaveData`/`capture_game_snapshot_save_data`/`validate_and_load_closest_snapshot_data`/`try_chunk`/`LoadedSnapshotData`/`SnapshotLoadResources` | `snapshot.rs`（节序 Version→Walls→…→Raster；Terrain flate2 deflate；Version 校验） | 0x1417a0430 / 0x1417a6e50；flate2 1.0.24（原锁） |
| `LoadGameManager`/`LoadGameSource`/`ParsedGameData`/`PostGameLoadCommand`/事件 | `load_game.rs` | PDB structures |
| `parse_saved_game_data`（upgrade 备份 + 节序解析） | `load_game.rs` | ALGORITHMS §2.7 |
| on_load 两遍回放（MetaCommand → PostGameLoadCommand，fence 终止） | `load_game.rs::advance_load_game`/`replay_meta_commands`/`replay_post_game_load_commands` | re-analyst（0x20B 条目、fence=0x8000000000000003）+ ALGORITHMS §2.7 |
| `extract_max_wall_id_from_history`/`upgrade_window_color`/`upgrade_arrow_slit_color`/`version_gates` | `load_game.rs`（复用 resources::save_file 映射） | ALGORITHMS §2.7 + save_file.rs 既有实现 |
| `add_save_load_systems`/`add_save_load_events_and_resources` | `mod.rs`（事件/资源注册；系统注册阻塞面如实记录） | 0x1409443b0 / 0x1409447e0 |
| **MetaCommand/MetaCommander/MetaCommandFence** | `meta_commander.rs`（0x38B=7 qword；空哨兵 0x8000000000000014；peek/consume/undo_consume/extend_script） | meta_commander.rs.md + RE-ANALYSIS |

## 吸收的 reviewer FIX

- `SaveBackupMetronome::add_active_game_time` 补阈值门（+0x148 < 60.0 才累计；`active_game_threshold` 字段）。
- `replace_edits`/`add_history_edit_to_save_file` 补原版日志串（"Edit appended to save file"/"Error appending edit to save file: "）。

## 二轮修正（reviewer REVIEW2）

- `extend_on_load` 批量条目按机器码 0x1403b5150 改为 **tag=3 + Box 批量结构**
  （`OnLoadEntry::Batch { commands: Vec<MetaCommand> }`），不再用 `ON_LOAD_VARIANT_MASK|1`
  单条 payload；load_game 两遍回放按 `OnLoadEntry` 枚举（Command/Batch）分派。
- 登记 **fence 双 tag 关系**：append_on_load<MetaCommandFence> 写
  `{tag=0x8000000000000000, payload=0x8000000000000003}`；load_game 回放以
  `tag==FENCE || payload==FENCE` 终止（RE-ANALYSIS 标注，未伪造）。

## 域类型落地（re-analyst RE-DOMAIN-TYPES.md，GameSaveData 组装解锁）

- **CameraRigParametersCache**（`systems/camera_rig/mod.rs`，0x24B 6 字段：
  yaw/pitch/position(Vec3)/camera_rig_arm_length/pitch_offset/yaw_offset；Camera 节载荷
  to_json_string 0x1416f8470；position 缺省 (0,5,0)）。
- **HistoryIdentity**（`resources/history/mod.rs`，u64 透明新类型；Identity 节 = 裸 u64；
  generate 0x140a84220 = fastrand）。
- **GladeColorCustomization**（新 `resources/glade_color_customization.rs`，15 字段全表；
  Colors 节载荷 to_json_string 0x14085ee90；visit_str 0x14085f550 名/序闭合）。
- **GameProfile**（新 `game_profile.rs`，0x208B ron 5 序列化字段；player_id
  `#[serde(skip_serializing)]`；default = Uuid::new_v4）。
- **`assemble_game_save_sections`**（`save_game.rs`）：Version/Identity/Camera/Colors 由
  上述类型直接串化；其余节由所属域提供序列化载荷，缺失即 panic（原版每节必写）。
- **`SaveFileIdentity` = `HistoryIdentity` 同值**（save_file_identity.rs 类型别名）。
- **load_completion_checker 澄清**：check_load_completion = (Res<AppMode>,
  EventReader<LoadingGameComplete>, Local<LoadingCompletionState>, Res<ActiveSession>)；
  is_session_ok_to_load = (SessionId, &UnsavedFileList)——**均不含 GameProfile**；
  GameProfile 真实消费方 = load_game / save_game / startup。纯函数签名已匹配。

新增测试：game_profile 2、glade_color_customization 1、save_file_identity 用 HistoryIdentity。

## 依赖复用边界

- flate2 1.0.24（原锁已有）新增为 country-core 直接依赖，供快照 Terrain deflate。
- crossbeam-channel（原锁）作命令/错误通道；shadow/snapshot_gc 既有实现不动。
- resources::save_file.rs 的 `LoadVersionGates`/`upgrade_*`/`DirtyBlocks` 直接复用。

## 仍缺失/待取证

- **save_game 系统 0x140a3f470**：GameSaveData 组装（~9 资源 + AutoSaveState 序列化缓冲
  {ptr,len}@+0x60/+0x68）依赖 CameraRigParametersCache/HistoryIdentity/GladeColorCustomization
  等域资源，留待对应域恢复后接入；不伪造节载荷。
- **capture_game_snapshot 0x1417a1d00（184KB 系统体）**：全 Query 串化依赖域 Query；
  本批仅落 SnapshotSaveData 结构 + 节序/解压基础设施。
- **init_game_state_from_snapshot 0x1417a9c50**：应用逻辑留待域恢复。
- **MetaCommand 变体 1 应用**（commander_update 0x14091f180 case 0..0x13 分派表）依赖众多
  游戏资源，未建；`extend_script` 已就绪。
- **load_completion_checker 完整系统**：SystemParam 含 GameProfile/MetaCommander（未建），
  纯函数已落地，系统注册阻塞。
- **CurrentSaveFile 原 3 变体枚举（0/1/2）**：本实现以 struct + Option 建模；ABI 形态差异登记。
- **SaveBackupList 计数器口径**（reviewer FIX-2，低）：0x14166df80 前缀语义待反编译裁决。
- **fence 双 tag 关系**（append 写 0x8000000000000000 vs load_game 终止 0x8000000000000003）
  与变体 2 unblock 偏移（+2 vs +0x10）：RE-ANALYSIS 标注，未伪造。

## 卡X4（save_game 本体 ECS 组装 0x140a3f470）追加

- **`save_game_system`**（save_game.rs）：①备份节拍器推进（AutoSaveState.metronome
  add_active_game_time，阈值门在累计块内）；②触发备份 → log "Capturing a game snapshot" +
  `CaptureSnapshotRequested` 事件（宿主构建快照 Arc 后经 AsyncSaveCmd::CaptureSnapshot
  发送）；③同步保存的 OpenFile/Flush 发送由宿主在事件响应中经 `auto.send_command` 完成。
- `register_save_game_events` + `add_save_load_systems_impl` 注册。
- **仍阻塞**：20+ SystemParam 域资源收集（InnerWalls/Roof/TimeOfDay/Terrain/
  DecoratorStorage/三色 ID/Prefab…）由宿主在事件响应中完成；不伪造节载荷。

## 卡X5/X7（capture/init 快照链 + run_game_thread 3 命令）追加
- **X5 capture 链**：`SnapshotTreesEntry` + `collect_trees`（snapshot.rs；Trees =
  Query<(&PrefabInstanceData,&PrefabInstance,&TreeComponent)> → (TreeStorageIndex,
  TreeData)，TreeData 以 tree_edit::TreeEditData 登记）；节序 Version→Walls→Roofs→
  Decorators→WallColors→GateColors→Terrain→Trees→Raster 已核（RNE 键串）。**各节
  域序列化器（Walls/Roofs/Decorators/三色/Terrain）留待域恢复**（capture_game_snapshot
  0x1417a1d00 184KB 域 Query）；init_game_state_from_snapshot 写回链（FUN_14004f570 +
  各节 helper）登记待域恢复。
- **X7 run_game_thread 3 命令**：`new_session_on_load_commands` + `build_new_session_on_load`
  （startup/run_game.rs；cmd2 = 变体 8 0x8000000000000008，载荷 [rbx+0x2230]/[rbx+0x2240]/
  rdi 三源；cmd0/cmd1 tag/载荷需 ReVa p-code 定点残留）。

## 编译/测试状态

- `cargo check -p country-core --offline`：我的文件零错误零警告（全 crate 2 警告在
  interaction 组文件）。
- `cargo test -p country-core --offline --lib systems::save`：**11/11 通过**（含新增
  execute_save_cmd 落盘、无活动档 panic、save_thread Shutdown ack、on_load 两遍回放）。

## 合同/状态建议

- inventory 建议：save_game/save_thread/snapshot/snapshot_gc/shadow_file/load_game/
  save_file_identity/load_completion_checker 状态 → `partial_implemented`（加载/回放链闭合，
  域组装与 commander_update 待接入）；本批 RESTORE 已记录新增实现与证据。
- contracts.json 的 `evidence/core-recovery-20261015/save/REPORT.md` 引用在磁盘不存在，
  reviewer 已提示；建议重锚到本 RESTORE.md 或 chapters。
- DISPATCH：save 仍 `fix_pending`（reviewer FIX 已吸收），完整模块验收被域组装/全局接线阻塞。
