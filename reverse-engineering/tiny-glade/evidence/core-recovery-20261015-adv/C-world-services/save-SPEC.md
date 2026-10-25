# save-SPEC — 完整 save-session 生命周期（C 世界，2026-10-15 第二轮）

> 历史分析结果，供同组实时复合 agent 直接落地。本文件只列**剩余缺口**的实现契约。
> 已实现边界见 `evidence/core-recovery-20261015/save/REPORT.md`（ShadowFile、JSON 信封/节表、备份节拍器、SaveBackupList、snapshot_gc、CurrentSaveFile 锁集均已就位且经中央 fixture 复核）。
> 证据源：`evidence/core-recovery-20261015/save/*.asm|*.txt`、`evidence/country-core-deep/session-save/ALGORITHMS.md`、`sysres-procedures-v2.json`。
> 原 B/RNE SHA256：`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。

## 状态判定

- **可直接落地**：AsyncSaveCmd 枚举骨架+主后台保存循环（0x141720680 全文已机器码证实）；UnsavedFileList；错误列表 0x40B 条目与通道回传；GC 构造/运行接线（复用已实现 snapshot_gc_thread）。
- **需先取证**：各 AsyncSaveCmd 变体负载的**全字段名/精确布局**（switch 体已读，字段名未恢复）；AutoSaveState 的完整 owner 结构（+0x140/+0x144/+0x148 已实，其余槽位 U）；capture/load 的每域采集器（分属其他域）。

---

## 1. 完整 AsyncSaveCmd 变体 + deferred 命令 + 主后台保存循环 / unsaved-file 重试上报

### 1.1 AsyncSaveCmd 枚举契约（O，部分字段）

原分发函数 `execute_save_cmd` = **FUN_141721aa0**（`evidence/.../save/0x141721aa0-normalized.txt`，3858 行）。判别在 `*param_1`（**tag@+0**，qword）。switch 面：`case 3,4,5,6,7,9,10` + default（含 8=ManualBackup）；`0/1/2` 无专属 case（主循环收包时按变体 drop 分派 `FUN_14167c8a0/141677330/1416d64e0`，I：死/历史变体或 Arc 负载变体）。`11`（Shutdown）在**主循环**处理，不进分发器。

| tag | 命令 | 负载（O 分段） | 关键行为 / 常量 |
|---|---|---|---|
| 3 | OpenFile | path（经 save_folder_path join） | 公共断言 `truncate <= history_edit_offsets.len()`；state!=3 → 拷 0x2a0B 旧态 → `flush_deferred_and_sync 0x14171f7f0`（**开新档前 flush 旧档**）；失败 → warn "Flush error while closing previous save file (upon open request)"；`UnsavedFileList::add`（FUN_14171ff80）；join "snapshots"；全量 `save_game_to_file<Cursor> 0x141750c90` 重写；`ShadowFile::create("history.json")`；0x270B 状态组装为新 CurrentSaveFile |
| 4 | CloseFile | — | debug log "AsyncSaveCmd::CloseFile("；flush 旧态；UnsavedFileList::add；清锁备份/通道 |
| 5 | ChangeSection | param_1[1..2]（+8/+0x10）载荷，[3..4]（+0x18/+0x20） | state==3 → panic **"No active save file, but AsyncSaveCmd::ChangeSection has been issued"**（@0x142ccab90/0x142ccaba0）；否则 change_section：节记录+offset+mark_dirty×2 |
| 6 | ReplaceEdits | {byte 标志@+3, edit 列表 ptr/len} | 同 No-active panic（@0x142ccac38）；逐 edit 追加（"Edit appended to save file" @0x142cca538 / "Error appending edit to save file: " @0x142cca5b0） |
| 7 | BackupSave | {path, kind} | `history-auto` 前缀备份；日志 "Backup limit reached; removing " / "Error deleting stale backup " / "Error reading backup file list: " / "Error making an auto-save backup: "（@0x142ccaf00 区） |
| 8 | ManualBackup | path 等 | 在 default 区：载荷比对文件身份字节（+0x53/+0x299 等）与 "snapshot"（0x746f687370616e73 u64 常量）；"Copying inactive save file to backup" / "Writing backup from shadow copy" / "Error flushing history file for backup" |
| 9 | CaptureSnapshot | {Arc<SnapshotSaveData>?, …} | state==3 → "No active save file, but AsyncSaveCmd::CaptureSnapshot has been issued"；创建 `snapshots` 目录（"creating snapshot folder"）→ 快照 JSON 落盘（节序 §2.6） |
| 10 | Flush | — | state==3 → 同 No-active 日志；否则 flush 当前档（"Error flushing changes to disk"） |
| 11 | Shutdown | 需 flush 标志字节 | 主循环：需 flush → state=3 → `flush_deferred_and_sync`；失败 → warn "Error flushing save file on game thread disconnect."（@0x142ccaa60）→ channel 发回确认 → drop 全部 → return（线程退出）；state==3 → 睡 10ms（0x989680ns）继续 |

公共前置（case 3..10 头部共用）：从 0x142cca478 区（51 字）反向扫 `/` `\` 取源文件路径（panic 消息）；断言串 "assertion failed: truncate <= self.save_file_structure.history_edit_offsets.len()"（@0x142cca478/0x142cca410）。

### 1.2 主后台保存循环 save_thread（O，全文已读）

**`save_thread::save_thread` = FUN_141720680**（`0x141720680-normalized.txt`）。每轮：

1. **时间前进 ≥1s**（无符号比较）且 `state != 3` 且脏列表非空 → 第一个脏条目 `ShadowFile::sync_all 0x14091a230` → 结果组装 **0x40B 错误条目 {路径, 状态}**（FUN_140d61b00 + FUN_141720490）→ push 失败列表（grow @0x142cca8c0）；
2. **condvar 等待**：mutex LOCK；未通知 → `wait_timeout(1s)`（FUN_1428cd210, 1e9 ns）；UNLOCK；
3. **失败列表 diff**（与上轮）：0x40B 条目逐字段比较（FUN_140b0d840 结构比较 + `+0x20` int + `+0x38` qword + `+0x30` 串 memcmp），首个差异处断开 → 新列表空 → log **"Auto-save I/O error resolved"**（info，级别>2）；非空 → log **"Save thread I/O error; affected file count: "**（warn，级别!=0）→ 经 channel 回传游戏线程（FUN_1412f86a0 + FUN_1408da430）→ 交换新旧列表；
4. `state != 3` → drop 当前存档（枚举 0/1/2 → FUN_14167c600/1416777a0/1416d6a60）；
5. **断言 `locked_backups.remove(&path)` 成功**（panic "assertion failed: current_save_file.locked_backups.remove(&path_buf)" @0x142cca978, line 0x44）——现有 `CurrentSaveFile::consume_snapshot_gc_release` 已实现此断言；
6. **recv 命令**（FUN_14152d950）→ 按命令变体解码（0/1/2 → 三 drop 函数）；
7. 变体 11 走 §1.1 表；变体 3 → `save_folder_path`+join+"history.json" → `UnsavedFileList::contains_file 0x1417201a0` → 命中 → panic "Was asked to load an unsaved file"（@0x142ccaa00）；
8. `execute_save_cmd 0x141721aa0`（§1.1）。

**UnsavedFileList**：`contains_file` = 0x1417201a0（sysres v2：`save_thread::UnsavedFileList::contains_file`）；成员文件登记 `add` = FUN_14171ff80（main 循环/OpenFile 内联）。**AutoSaveErrorState::is_ok** = 0x140b92210（sysres v2；错误状态访问器）。

### 1.3 CurrentSaveFile owner 升级

现有 `CurrentSaveFile`（systems/save/save_thread.rs）只含 shadow_file + structure + locked_backups + release receiver。原版 state 是 **枚举 0/1/2/3**（3=无活动存档；0/1/2=三种存档形态，drop 分派三函数）。OpenFile 组装 0x270B 状态。**落地建议**：把 `CurrentSaveFile` 扩展为能表达 state 判别（0..3）与快照目录字段的 owner；保留现有字段作 0x270B 的部分成员。**快照目录**：`save_folder_path().join("snapshots")`（@0x142ccaf28）。

### 1.4 待取证（save）

- AsyncSaveCmd 各变体**全字段名/精确字节布局**（tag@0 已实；负载偏移已分段但字段名 U）。落地时**先实现可见载荷偏移，不编造字段名**。
- 变体 0/1/2 的负载语义（I：Arc 变体，drop 三函数已见；无调试串）。
- `flush_deferred_and_sync` 0x14171f7f0 的完整语义（OpenFile/Shutdown 均调用；建议落地时复用现有 flush/truncate 协议实现）。

---

## 2. 游戏线程捕获/序列化 + 快照二进制捕获/load

### 2.1 save_game 主系统（O，节拍器内联）

**`save_game::save_game` = 0x140a3f470**（6 段 SystemParam，主游戏线程 Update 系统）：

1. **备份节拍器（内联，行 ~507-591）**：字段 {counter u32@+0x140, 累计 f32@+0x144, 阈值 f32@+0x148}；仅当 `阈值 < 60.0`（或 NaN）累计；断言 `delta >= 0.0`（"assertion failed: active_game_delta_seconds >= 0.0" @0x142a9f2c0）；累计 clamp[0,1]；`累计 >= 1.0` → counter++ 并扫**除数桶 {300,600,3600(0xe10),7200(0x1c20)}**（@0x142a9f348，从大到小整除匹配）→ 命中后 `counter -= 除数`（**可一次消耗多个间隔**，内层按升序重新校验；桶内容 0 → panic 除零）。**注意**：与当前 `SaveBackupMetronome::consider_backing_up` 的差异——原版命中后 `counter -= divisor`（不是 wrapping_add 后保留），且消耗逻辑不同。现有实现已按指令复核（`consider_backing_up` 返回 `BackupDecision{counter, remove_counter}`），**落地时把节拍器并入 AutoSaveState 字段布局**。
2. **触发备份**：log "Capturing a game snapshot"（info，模块 country_core::systems::save::save_game）→ `capture_game_snapshot_save_data 0x1417a0430` → Arc 克隆 → `History::set_current_snapshot 0x140a84760`（断言当前快照 none 后有序插入）→ `FUN_1408da280` 枚举分发发送 `AsyncSaveCmd::CaptureSnapshot(9)`。
3. **同步保存**：GameSaveData 组装（~9 资源 +0x28..+0x100 密集字段）→ `save_game_to_file`；错误串 "Error while performing a synchronous save: " / "Sync response from save thread" / "Failed writing session meta: "。
4. **增量写**：`add_history_edit_to_save_file 0x140a439b0`（= ensure_current_snapshot）——已有 `ShadowBuffer::append_edit` 实现此协议。

### 2.2 快照捕获（O）

- **`capture_game_snapshot_save_data 0x1417a0430`**：串匹配 @0x142cdc3b0（0x46 字，panic @0x142cdc480/0x142cdc498）+ 路径 join（FUN_140c16d40）→ 从 ~9 组世界数据（**采集器 FUN_141305170/141308b40/141309840**，分属墙/屋顶/装饰域）构建 ~0x400B 快照结构（JSON 键串表 @0x142a93bfe 起：walls/roofs/…）→ 返回 Arc（alloc 0x18 {strong=1,weak=1} + LOCK inc）。
- **`capture_game_snapshot 0x1417a1d00`**（184KB 系统体）：全 Query 串化；`StairsSnapshot{edges, checksum}` / `TerrainEditingData{strokes, next_stroke_id, next_stroke_index}` / `DecoratorStorageSnapshot{states}`。

### 2.3 快照 load（O）

- **`validate_and_load_closest_snapshot_data 0x1417a6e50`**：按节名键分派（FUN_1414744d0 匹配 + 跳表 @0x142cdca80/0x142cdca98/0x142cdcab0）：**节序 Version → Walls → Roofs → Decorators → WallColors → GateColors → Terrain → Trees → Raster**；每节 log "parsing X"（@0x142cdc709 块）；Version 不符 → panic "Version mismatch; expected , got "（@0x142cdc840）；Terrain 数据 **flate2 deflate 解压**（"decompressing terrain data" @0x142cdc800）；Raster 键白名单外 → "Ignoring unknown raster data in snapshot: "（@0x142cdc7c0）。
- **`init_game_state_from_snapshot 0x1417a9c50`**：load_game 拷贝 0x1b0B 解析数据 → 应用各域资源（颜色/时间/羊/地形…）。
- **`SnapshotLoadResources::init_state` 0x1417ab290**：SystemParam 初始化（sysres v2）。

### 2.4 load_game 主系统（O，读档流核心）

**`load_game::load_game` = 0x14146d030**（13+ 段 SystemParam）：

- **门控**：`*app_mode == 0xe`（LoadGameData 态）才运行；
- **状态字节 +9**：`0` → 按 +0x4c 计数（可负）发 16B 加载进度记录 {计数器, tag 0x3010001 或 0x3010101} 入 Vec<16B>，置状态 2，return（多帧加载节拍）；`2` → 主流程；其他 → panic @0x142c641d0（0x28 字）；
- **NextSessionKind 解码**：`decoded = kind ^ 0x8000000000000000; sel = min(decoded, 2)`；
  - sel==1 → `parse_saved_game_data 0x141470d00`（内联于行 ~413）：节序 "Making save file backup for upgrade" → "Reading file data" → "Parsing JSON" → "Version" → "BackwardCompatibility" → "Camera" → "parsing camera" → "Time of day"（缺失 → "This save file had no 'Time of day' data"）；失败 → "Couldn't load save file from "；**upgrade 前先做存档备份**；
  - sel==2 → 内存路径：manager 状态字节 +9=2、无快照标记 +0xd=0xffffffff、on_load 命令回放；
- **MetaCommand 回放**（on_load 列表，条目 4B tag）：`tag == 0x8000000000000003` → **fence 终止**；`tag ^ 0x8000000000000000` 解码（>2 钳 1）：变体 0 → 写 manager 状态 + push 24B 事件；变体 1 → `FUN_140b5a180` push MetaCommand（meta_commander.rs 路径串 @0x142c5ae38）；变体 2+ → Arc LOCK dec drop；
- **PostGameLoadCommand 回放**（第二轮 on_load 遍历）：变体 0（SwitchAppMode）→ log "Ignoring PostGameLoadCommand::SwitchAppMode"（级别>1，@0x142c642e0）跳过；变体 1 → push 24B {tag, 载荷×2}（grow @0x142c64308）；变体 2+ → 同上；fence 终止；断言 "Command skipping must not be used together with snapshots" / "Can't skip "（@0x142c64378/0x142c643d8 区）；`FUN_1412e0b90` 应用命令；
- **版本 → 全局开关**：`0x14362c308 = (v > 0x19)`；`0x14362c011 = (v > 0x1a)`（后者 = history 域 add_decorator_diff 楼梯 checksum 校验开关——**跨域移交点**）；
- **旧版颜色 ID 升级**（"Detected old version of window color IDs. Upgrading..." / "Detected old version of arrow slits color IDs. Upgrading..."，@0x142c64a90/0x142c64af0）——`resources/save_file.rs` 的 `LoadUpgradeFlags/upgrade_*` 已实现此协议，**load_game 需调用**；
- **extract_max_wall_id_from_history**：遍历 MultiEdit 取最大 wall id → 计数器资源 +0xc0；
- **init_game_state_from_snapshot 0x1417a9c50** 应用 → 清理（快照 Arc 列表 LOCK dec 循环）；
- 事件：LoadingGameComplete / StartupRevealStarted/Finished；"Cloning game state for save rewrite took "（@0x142c64b48）；
- 后置：写 `save_folder/loading.tmp`（@0x142c6416e）；新 glade 分支 log "Loading a new glade. Spawn pre-placed trees"（@0x142c641a8）+ 事件 push。

### 2.5 待取证（save）

- GameSaveData 240B 的**完整字段布局**（owner 在 save 域；~9 资源 +0x28..+0x100 已见，字段名 U）。
- capture 的 3 个世界数据采集器（FUN_141305170/141308b40/141309840）的采集协议（分属 wall/roof/decorator 域，落地时先留接口）。
- `parse_saved_game_data` 0x141470d00 的节解析精确策略（已在 load_game 内联描述层，全字段待取证）。

---

## 3. 自动/手动备份生产 + sync 失败回退 shadow 内存 + retention 删除队列与锁交互

### 3.1 备份生产（O）

- **`AutoSaveState::create_backup_of 0x141e59860`**（sysres v2；备份目录创建）。
- BackupSave 命令（tag 7）见 §1.1：`history-auto` 前缀；日志族 "Backup limit reached; removing " / "Error deleting stale backup " / "Error reading backup file list: " / "Error making an auto-save backup: "。
- **保留策略**：现有 `SaveBackupMetronome::consider_backing_up` 返回的 `remove_counter` 即 retention 删除候选（`BackupDecision{remove_counter: Option<u32>}`）；删除走 "Backup limit reached; removing " 日志路径。**落地**：把 `BackupDecision` 接到 AsyncSaveCmd::BackupSave 生产（counter→`history-auto-{counter}.json`，remove_counter→删除旧备份）。
- **manual 备份**（tag 8 default 区）：`history-manual-` 前缀（SaveBackupList::from_path 已实现分类）。

### 3.2 sync 失败回退（O）

`ShadowFile::sync_all` 失败 → save_thread 每 1s 逐文件 flush 失败条目（0x40B）→ 回传游戏线程 → 下轮 diff "Auto-save I/O error resolved"。现有 `ShadowFile` 在 `create` 失败时已保留全内存镜像并全块标脏、flush 重试（`last_error@+0x78` 为 `Option<anyhow::Error>`）——**回退 shadow 内存的机制已就位，缺的是主循环的 1s 轮询接线**（§1.2）。

### 3.3 retention 删除队列与锁交互

现有 `CurrentSaveFile` 的 `locked_backups` 集合 + `snapshot_gc_releases` receiver + `consume_snapshot_gc_release`（断言 remove）已实现 GC 锁协议。**剩**：备份 retention 删除队列（`remove_counter` 触发 `remove_file`）与 locked_backups 的交互（删除一个锁定的备份要如何避免 GC 误删——待取证，原版 `BackupSave` case 内嵌）。

### 3.4 待取证（save）

- retention 删除队列的确切结构（I：逐文件删除，无独立队列；与 locked_backups 的竞态策略 U）。

---

## 4. session 切换/打开自动构造并运行 GC 片 + 完整 save-session 生命周期

### 4.1 GC 片的构造点（O）

`snapshot_gc_thread 0x14172b280` 已实现。**调用方**（构造 backup_paths/snapshot_paths/referenced 并 `start_snapshot_gc`）是 session 切换系统 / load_game：见 session-SPEC §3（会话切换系统读 `Res<AutoSaveState>` + `ResMut<NextSession>` + `ResMut<LoadGameManager>` + 事件 NewSessionStartedCmd/SessionLoadedCmd）。原版会话流：
- `refresh_session_list 0x140b46b00`：清空后 paths::saves() readdir 重建 SavedSessionList；
- 会话选择 → `load_game_session 0x14166e7d0`（startup_session，见 session-SPEC §4）；
- `snapshot_gc_thread` 的 `parse_save_file_snapshots 0x141286730` 读每个锁定的备份 → release channel → 主循环断言 remove。

### 4.2 完整 save-session 生命周期（I/O 混合）

见 `session-save/ALGORITHMS.md §1.1/§1.2` 处理链图。**可直接落地**：save_thread 主循环 + AsyncSaveCmd 分发 + GC 接线使能"存档线程作为独立 owner"；**需先取证**：session 切换系统（归属 session 域）与 load_game 的完整时序。

---

## 5. 真实消费者接线清单

| 生产者 | 消费者 | 状态 |
|---|---|---|
| save_game 0x140a3f470（节拍器触发） | History::set_current_snapshot 0x140a84760 → AsyncSaveCmd::CaptureSnapshot(9) | 未接线（节拍器已实现，发送缺失） |
| save_thread 0x141720680（1s 轮询） | ShadowFile::sync_all 失败 0x40B 错误列表 → 游戏线程 channel | 未接线 |
| execute_save_cmd 0x141721aa0 | CurrentSaveFile::change_section/replace_edits/flush（已实现） | 未接线 |
| session 切换（session 域） | CurrentSaveFile::start_snapshot_gc（已实现）→ snapshot_gc_thread | 未接线 |
| load_game 0x14146d030 | save_file.rs LoadUpgradeFlags / 版本开关 / extract_max_wall_id | 未接线 |
| AutoSaveState（save_game.rs owner） | SaveBackupMetronome（已实现）| 需并入 +0x140/0x144/0x148 布局 |

## 6. 明确排除项

- 不重写 std/Bevy/crossbeam 通道/bitvec；直接复用。
- GPU 执行、场景回放不属本 SPEC。
- 不编造快照/存档领域数据；codec 需调用方提供真实序列化节值（现有 GameSaveSections 语义保持）。
- 不把 AsyncSaveCmd 变体 0/1/2 的"不存在"当"已恢复"——标记待取证。
