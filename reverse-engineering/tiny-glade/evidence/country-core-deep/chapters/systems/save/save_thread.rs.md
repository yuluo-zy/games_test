# systems/save/save_thread.rs

> 文件作用：存档线程：save_thread 主循环（脏 shadow 文件逐 tick flush → mutex+condvar 等待≤1s → 错误列表 diff/日志 → recv 命令）；execute_save_cmd = AsyncSaveCmd 分发（3=OpenFile/4=CloseFile/5=ChangeSection/6=ReplaceEdits/7=BackupSave/8=ManualBackup/9=CaptureSnapshot/10=Flush/11=Shutdown）；snapshot_gc_thread 清理未引用快照；CurrentSaveFile 增量历史写（Edit appended to save file + mark_dirty 协议）。

函数总计 30：业务 9 / 内联无独立符号 7 / 派生胶水 14

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| ``country_core::systems::save::save_thread::execute_save_cmd`` | 0x141721aa0 | 0 | 待推断 | O：execute_save_cmd = switch{3,4,5,6,7,9,10}+default(含 8=ManualBackup)；OpenFile 体内 join "snapshots"+Rewriting history file+Saving the game to a temp file+creating snapshot folder | session-save\decomp\decomp-0x141721aa0.txt  |
| ``country_core::systems::save::save_thread::save_thread`` | 0x141720680 | 0 | IO/解析 | O：save_thread 主循环（condvar 1s；cmd==0xb → flush_deferred_and_sync+退出；Save thread I/O error; affected file count 日志） | session-save\decomp\decomp-0x141720680.txt  |
| ``country_core::systems::save::save_thread::CurrentSaveFile::change_section`` | 0x14171ece0 | 115 | 设置/写入 | O：CurrentSaveFile::change_section = 修改当前节+ShadowFile::mark_dirty(0x14091a450)×2（脏区间协议互证） | round3\decomp-0x14171ece0.txt  |
| ``country_core::systems::save::save_thread::CurrentSaveFile::flush_deferred_and_sync`` | 0x14171f7f0 | 174 | 待推断 | O（round3 round2 修正串归属）：flush_deferred_and_sync（TLS 守卫+断言+vtable 调用，断开时落盘结构吻合）；“Error flushing save file on game thread disconnect.”（0x142ccaa28）唯一 xref 在 save_thread 主循环 0x141720680（0x1417212c0 处），不在本函数 | session-save\decomp\decomp-0x14171f7f0.txt |
| ``country_core::systems::save::save_thread::UnsavedFileList::add`` | 0x14171ff80 | 294 | 构造/注册/生成 | O：UnsavedFileList::add（v2 名；v1 名 backup_save 同址）= 锁+condvar 通知+内联实现 | session-save\decomp\decomp-0x14171ff80.txt  |
| ``country_core::systems::save::save_thread::UnsavedFileList::contains_file`` | 0x1417201a0 | 300 | 查表/访问 | O：contains_file = 列表查找（0x40B 条目 memcmp） | session-save\decomp\decomp-0x1417201a0.txt  |
| ``country_core::systems::save::save_thread::impl$2::from`` | 0x141720490 | 312 | 待推断 | O：impl$2::from（v1 名 contains_file::closure$0）= 文件条目比较辅助 | session-save\decomp\decomp-0x141720490.txt  |
| ``country_core::systems::save::save_thread::AutosaveErrorState::is_ok`` | 0x140b92210 | 328 | 谓词/校验 | O：AutosaveErrorState::is_ok = [rcx+0x10]==0 sete（is_empty 折叠，NotFound 手解） | session-save\DIGEST.md  |
| ``country_core::systems::save::save_thread::snapshot_gc_thread`` | 0x14172b280 | 817 | 待推断 | O：snapshot_gc_thread = parse_save_file_snapshots→目录比对→Deleting unreferenced snapshot；竞态守卫 Save file changed before snapshot GC could finish... | session-save\decomp\decomp-0x14172b280.txt  |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::save::save_thread::CurrentSaveFile::replace_edits`（行 65）— 待推断
- `country_core::systems::save::save_thread::CurrentSaveFile::backup_save`（行 187）— 待推断
- `country_core::systems::save::save_thread::CurrentSaveFile::perform_manual_backup`（行 266）— 待推断
- `country_core::systems::save::save_thread::execute_open_file_cmd`（行 670）— 待推断
- `country_core::systems::save::save_thread::full_write_history_file`（行 750）— 待推断
- `country_core::systems::save::save_thread::capture_snapshot`（行 793）— 待推断
- `country_core::systems::save::save_thread::nuke_unreferenced_shapshots`（行 868）— 删除/清理

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`closure`、`default`、`eq` 等 14 项

## 实现方式（算法级，2026-10-15 深挖；全文见 session-save/ALGORITHMS.md）

- **save_thread 0x141720680 主循环**：
  1. 每 ≥1s tick（time 基线比较）：state!=3 且脏列表非空 → 第一个脏条目
     ShadowFile::sync_all（0x14091a230）→ 结果组装 0x40B 条目（FUN_140d61b00 +
     FUN_141720490）→ 失败列表（grow FUN_1405627b0）；
  2. mutex + condvar 等待 ≤1s（FUN_1428cd210, 1e9ns）→ UNLOCK；
  3. 失败列表 diff（0x40B 条目逐字段：结构比较 + +0x20 int + +0x38 qword + +0x30
     memcmp）→ 新列表空：log "Auto-save I/O error resolved"（级别>2）；非空：log
     "Save thread I/O error; affected file count: "（级别!=0）→ FUN_1412f86a0 +
     FUN_1408da430 经 channel 回传游戏线程；
  4. state!=3 → drop 当前存档（枚举 0/1/2 → FUN_14167c600/1416777a0/1416d6a60）；
  5. 断言 locked_backups.remove(&path) 成功（panic @0x142cca978）；
  6. recv 命令（FUN_14152d950）→ 解码（0/1/2 → FUN_14167c8a0/141677330/1416d64e0）；
  7. **0xb Shutdown**：需 flush → state=3 → CurrentSaveFile::flush_deferred_and_sync
     0x14171f7f0（失败 warn "Error flushing save file on game thread disconnect."
     @0x142ccaa60）→ channel 发回确认（FUN_140052a30）→ 清理 → return 退出线程；
     否则 state==3 → 睡 10ms 继续；state!=3 → 直接 return；
  8. **3 OpenFile 前置**：save_folder_path + join("history.json") →
     UnsavedFileList::contains_file 0x1417201a0 → 命中 → panic "Was asked to load an
     unsaved file"（@0x142ccaa00）；
  9. execute_save_cmd 0x141721aa0。
- **execute_save_cmd**（switch {3,4,5,6,7,9,10}+default；公共断言 = truncate <=
  history_edit_offsets.len()，@0x142cca478）：
  - 3 OpenFile：debug log "AsyncSaveCmd::OpenFile(" → 旧态 flush（失败 warn "Flush
    error while closing previous save file (upon open request)" @0x142ccaf00）→
    UnsavedFileList::add → save_folder_path+join("snapshots") → **save_game_to_file
    <Cursor> 全量重写**（失败 log "Rewriting history file" @0x142ccaf31）→
    ShadowFile::create("history.json") → 0x270B 状态组装；
  - 4 CloseFile / 5 ChangeSection / 6 ReplaceEdits：**state==3（无活动档）→ panic
    "No active save file, but AsyncSaveCmd::X has been issued"**；5 改节 + mark_dirty
    ×2（0x14171ece0 互证）；6 逐 edit 追加（"Edit appended to save file"
    @0x142cca538 / "Error appending edit to save file: " @0x142cca5b0）；
  - 7 BackupSave：`history-auto` 前缀 + 备份限额（"Backup limit reached; removing " /
    "Error deleting stale backup " / "Error reading backup file list: "）；
  - 9 CaptureSnapshot：创建 `snapshots` 目录（"creating snapshot folder"）→ 快照
    JSON 落盘；10 Flush：flush 当前档（"Error flushing changes to disk"）；
  - default（含 8=ManualBackup）："Copying inactive save file to backup" /
    "Writing backup from shadow copy" / "Error flushing history file for backup"；
    按载荷比对文件身份字节（+0x53/+0x299）与 "snapshot"（0x746f687370616e73）；
  - 0/1/2 无专属 case（I：死/历史变体）。
- **snapshot_gc_thread 0x14172b280**：parse_save_file_snapshots（0x141286730）→
  snapshots 目录比对 → "Deleting unreferenced snapshot "（@0x142ccb088/0x142ccb0e0）
  → 结果经 channel（FUN_1408da040）；竞态守卫 "Save file changed before snapshot GC
  could finish. Shutting down the GC thread"；错误 "Error opening backup file " /
  "Error parsing history snapshots fromfile"。
