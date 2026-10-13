# WP13 — `systems::history` + `systems::save` 业务逻辑合同

B 版本。恢复提交/撤销/重做/存档系统。实现见 `src/systems/{history,save}/`。

## systems::history

### history_process（0x140af5b30，O：building-history 已有反编译 + 本包核验）
- 消费 `HistoryCmd` 事件队列（4 字节字）。字布局：byte[0]=命令族，byte[1]=子命令。
  - `byte[1]==0` → Undo：`cursor>0 && cursor>history_start && cursor<len` 时 `cursor-1`。
  - `byte[1]==1` → Redo：`cursor+1 < len` 时 `cursor+1`；`cursor<-1` panic。
  - `byte[1]==4` 或 `byte[0]==2` → Interrupt（中断批量命令消费）。
  - 每次游标移动后 `replay_edit()` 展开该条 MultiEdit 为回放命令。
- `HistoryCmd::interrupt` 0x140af5b20：构造 `0x02000200`（候选 TG-c4cdc0aa5e62db9266，
  B 地址核实）。
- `PendingHistoryCommands::new`/`is_empty`；`schedule_replay_commands`（+闭包）；
  `history_clear_previous_state`；`raster_edit_to_replay_cmd`；
  `generate_replay_cmds_from_edit`（4 个实例化：replay_edit / path / water / garden）。
- 与 `resources::history::ReplayBuffer`（replay_cmd_buffer.rs）交互：`front_mut_bucket`
  0x1409f1a80、`pop_front` 0x1409f18f0、`make_animations_instant` 0x1409f1ab0、`len`
  0x140949170（O：地址；语义 U）。

### history_recording
- `history_start_recording` / `history_finish_recording` 0x1409dd790：
  驱动 `History::start_input_recording` / `finish_input_recording` 的 ECS 系统
  （O：PDB 名 + 地址；记录/完成编排 U）。

### 回放族（O：地址已定位，语义按 domain 各自恢复）
`finish_replay` 0x1408b36a0、`replay_mode` 0x140a7c550、`replay_pop_bucket`
0x140b38250（每调用最多弹出一个外层桶）、`signifier_replay_active` 0x140b1b860，
及各 domain 回放：wall（`replay_wall` 0x140951b80、`WallReplayEditCmd::process`
0x140abfb30）、roof、terrain（`TerrainReplayCmd::forward/backward` 0x140994f30/
0x140995040）、path、water、garden、stairs、auto_clutter、raster。

### mod.rs
`add_history_systems` / `insert_history_resources` / `add_history_events`（PDB 名）。

## systems::save

### save_game（0x140a3f470，主存档系统，O：反编译 + 字符串）
- 读取 GameSaveData 相关资源；`capture_game_snapshot` 前日志
  “Capturing a game snapshot”；同步保存出错 “Error while performing a synchronous
  save”；`write_to_disk` 失败 “Failed writing session meta”。
- `save_game_to_file<W,Iter>` 0x141750c90 + `write_json_sections<W>`：按
  `SaveFileSection` 枚举逐节写 JSON（Cursor<Vec<u8>> 与 File 两个实例化）。
- `add_history_edit_to_save_file` 0x140a439b0：增量写历史 edit（把当前 edit
  计数值压入待写列表；节 `HistoryChainIndex`/`HistoryStart`/`HistorySnapshot`）。
- `save_json_through_tmp_file`（+闭包）：临时文件写回。

### SaveFileSection 枚举（10 变体，O：json_key 0x140a43810 机器码 + .rdata 表读取）

| tag | json_key（rdata 0x142ace218 串块） | padding_bytes（0x142ace3c8 表） |
|---|---|---|
| 0 | `Version`（len 7） | 0 |
| 1 | `Identity`（len 8） | 16 |
| 2 | `BackwardCompatibility`（len 21） | 0 |
| 3 | `Camera`（len 6） | 128 |
| 4 | `Time of day`（len 11） | 32 |
| 5 | `Sheep`（len 5） | 128 |
| 6 | `Colors`（len 6） | 64 |
| 7 | `chain_index`（len 11） | 8 |
| 8 | `history_start`（len 13） | 8 |
| 9 | `snapshot`（len 9） | 2048 |

另有 `save_cooldown_seconds` / `should_trigger_backup` / `should_trigger_debug_log`
（PDB 名；语义 U）。`json_key` 读取 tag 字节 → 两张 8B 表（len 表 + ptr 表）。

### AutoSaveState / CurrentSaveFile / save_thread
- `AutoSaveState`：`close_file` 0x140a43840、`last_error_state` 0x140a439a0、
  `create_backup_of<PathBuf>` 0x141e59860。
- `save_thread` 0x141720680：异步写盘循环（execute_open_file_cmd / execute_save_cmd /
  full_write_history_file / capture_snapshot / nuke_unreferenced_shapshots）。
- `CurrentSaveFile`：`backup_save` / `replace_edits` / `perform_manual_backup` /
  `change_section` / `flush_deferred_and_sync`。
- `UnsavedFileList::add` / `contains_file` 0x1417201a0；`AutosaveErrorState::is_ok`
  0x140b92210（is_empty 折叠）。

### snapshot（0x1417a* 族，O：反编译）
- `capture_game_snapshot` 0x1417a1d00（日志 “Capturing a game snapshot”）。
- `capture_game_snapshot_save_data` 0x1417a0430。
- `try_chunk` 0x1417a6d70：`Result<Option<chunk>, anyhow>`。
- `validate_and_load_closest_snapshot_data` 0x1417a6e50（失败日志 “Failed to load
  history snapshot”）。
- `init_game_state_from_snapshot` 0x1417a9c50（日志 “Loading game state from a
  snapshot”）；`try_load_snapshot_data`；SystemParam `get_param`/`validate_param`。

### load_game（0x14146d030，O：反编译 + 字符串）
- `LoadGameManager` / `LoadGameSource` / `ParsedGameData` / `PostGameLoadCommand` /
  `LoadingGameComplete` / `StartupRevealStarted/Finished`。
- `parse_saved_game_data` 0x141470d00：反序列化各节。
- `apply_parsed_game_data` / `apply_backward_compatibility_overrides` /
  `extract_max_wall_id_from_history` / `upgrade_window_color`（旧版 window color /
  arrow slots 升级日志；“Ignoring PostGameLoadCommand Switch…”）。

### save_backup_list / save_file_identity / shadow_file / load_completion_checker / snapshot_gc
- `SaveBackupList::new_from_session` 0x140ae8470（错误日志 “Error reading save backup
  directory”；文件名前缀 `history_auto`）；`iter_backup_files` 0x140ae8330；
  `into_files`；`backup_metronome_counter`。`BackupSaveFileKind::is_corrupted`
  0x140ae8320（与 GladeType::is_winter 同址折叠，tag==4 判定）/`is_auto`。
- `save_file_identity`：`derive_fallback_save_file_identity` 0x141673140、
  `try_approx_parse_save_file_identity` 0x141e58f20（+闭包）。
- `shadow_file`：`ShadowFile::create` 0x1409184d0、`flush` 0x1409193d0、
  `sync_all` 0x14091a230、`mark_dirty` 0x14091a450、`write_at`/`truncate`/
  `path`/`data`；`ShadowFileWriter`（write_all/stream_position）——影子文件
  原子写。
- `load_completion_checker`：`SessionLoadReadiness::is_ok` 0x140833230（is_initial
  折叠）、`deny_load` 0x140a9fa70、`is_session_ok_to_load` 0x140a9fa80、
  `check_load_completion` 0x140a9f810。
- `snapshot_gc`：`parse_save_file_snapshots<BufReader<File>>` 0x141286730。

## I — 推断

- 存档文档节序/布局与 tg-history 观察的保存 JSON 一致（Version/History/Camera/
  Sheep/Time of day/…/Identity/BackwardCompatibility/Colors）。
- 增量存档按 `chain_index`/`history_start`/`snapshot` 三个历史子节维护
  （SaveFileSection 变体 7/8/9 即历史字段）。

## U — 未知（typed gap）

- save_game / load_game 的完整资源清单与精确 JSON 字段序（void() 占位 ABI；
  反编译为 serde 胶水噪声）。
- `SaveFileSection::save_cooldown_seconds`/`should_trigger_backup` 数值。
- save_thread 消息协议、影子文件路径命名、backup 节流（metronome）节奏。
- snapshot 负载的 rkyv/ron 编码（save schema 恢复仅到 json_key/节级）。
