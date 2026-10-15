# session-save 域算法级深挖（ALGORITHMS.md，2026-10-15）

> 本文件把 session-save 域 17 章升级到"算法级"：存档流/读档流处理链图、逐函数算法、
> 状态机、常量表。证据 = `session-save/decomp/decomp-<va>.txt` + rne-read + round2/
> session-save-CLOSURE.md。O = 机器码证实；I = 推断；U = 未决。与 DIGEST.md §3 互补。

---

## 1. 处理链图（存档流 / 读档流）

### 1.1 存档流

```
游戏线程                              存档线程（save_thread 0x141720680 主循环）
─────────                            ─────────────────────────────────────
save_game 0x140a3f470（Update 系统）
  ├─ SaveBackupMetronome（内联）:           ┌─ 每 1s tick：
  │   counter/累计/阈值@+0x140/0x144/0x148 │    脏 shadow 文件逐 tick flush
  │   档位 {300,600,3600,7200}s             │    （ShadowFile::sync_all 0x14091a230）
  ├─ 触发备份:                             │    → 失败入 0x40B 错误列表
  │   "Capturing a game snapshot" →        │    → 与上轮 diff → log
  │   capture_game_snapshot_save_data       │      "Save thread I/O error; affected
  │   0x1417a0430 → Arc →                    │       file count: " / "Auto-save I/O
  │   History::set_current_snapshot          │       error resolved"
  │   0x140a84760 →                          │    → 错误列表回传游戏线程（channel）
  │   AsyncSaveCmd::CaptureSnapshot(9)      │  ┌─ mutex+condvar 等待 ≤1s
  ├─ 同步保存:                             │  ┌─ drop 当前存档状态（枚举 0/1/2）
  │   GameSaveData 组装 →                   │  ┌─ recv 命令（FUN_14152d950）→ 解码
  │   save_game_to_file<Cursor/File>        │  ┌─ 3=OpenFile 前置检查
  │   （手工拼 JSON + write_json_sections） │  └─ execute_save_cmd 0x141721aa0
  ├─ 增量写:                               │       ├─ 3 OpenFile：旧态 flush → 全量
  │   add_history_edit_to_save_file          │       │   save_game_to_file<Cursor> →
  │   0x140a439b0 = ensure_current_snapshot  │       │   ShadowFile::create("history.json")
  │   （",\n"+序列化+mark_dirty(start,end)） │       ├─ 4 CloseFile / 5 ChangeSection /
  │                                          │       │   6 ReplaceEdits（No active save
  └─ save_session_meta_to_file 0x1417522e0  │       │   file panic 门控）
                                             │       ├─ 7 BackupSave（history-auto 前缀 +
                                             │       │   备份限额）／8 ManualBackup
                                             │       ├─ 9 CaptureSnapshot（snapshots 目录）
                                             │       ├─ 10 Flush
                                             │       └─ 11 Shutdown：flush_deferred_and_sync
                                             │          + 发回确认 + 退出线程
                                             └─ snapshot_gc_thread 0x14172b280：
                                                  parse_save_file_snapshots → 目录比对
                                                  → "Deleting unreferenced snapshot "
```

### 1.2 读档流

```
load_game 0x14146d030（主系统，13+ 段 SystemParam）
  ├─ 门控：*app_mode == 0xe（LoadGameData 态）才运行
  ├─ LoadGameManager 状态字节（+9）：0 → 发 N 条 16B 加载进度记录 {计数器, tag}，置 2
  ├─ 状态 2 → NextSessionKind 解码（kind ^ 0x8000000000000000，min(·,2)）：
  │    ==1 → parse_saved_game_data（内联 0x141470d00）：
  │          "Making save file backup for upgrade" → "Reading file data" →
  │          "Parsing JSON" → 节序 Version → BackwardCompatibility → Camera →
  │          "parsing camera" → Time of day（缺失 → "This save file had no
  │          'Time of day' data"）；失败 → "Couldn't load save file from "
  │    ==2 → on_load 命令回放（MetaCommand，fence 0x8000000000000003）+ 内存快照路径
  ├─ PostGameLoadCommand 回放（第二轮 on_load 遍历）：
  │    变体 0（SwitchAppMode）→ log "Ignoring PostGameLoadCommand::SwitchAppMode" 跳过；
  │    变体 1 → push 24B 命令 {tag, 载荷×2}；变体 2+ → Arc 负载 drop
  ├─ 存档版本 → 全局开关：0x14362c308 = (v>0x19)，0x14362c011 = (v>0x1a)
  │    （后者 = history 域楼梯 checksum 校验开关！）
  ├─ 旧版颜色 ID 升级：+0x52 ∈ {0x38..0x3a} 的 0x58 步长记录，+0x30 字段按
  │    {1→2, 2→5, 4→8, 5→3, 8→4}（0/3/6/7 不变，999 透传，default→1）
  ├─ extract_max_wall_id_from_history：遍历 MultiEdit 取最大 wall id → 计数器资源 +0xc0
  ├─ init_game_state_from_snapshot 0x1417a9c50（拷贝 0x1b0B 解析数据 → 应用）
  └─ 事件：LoadingGameComplete / StartupRevealStarted/Finished；写 loading.tmp
```

### 1.3 影子文件原子写协议

```
mark_dirty(start, end) 0x14091a450：4KB 块位图（+0x60 ptr / +0x68 位计数）置位
   start_block = start>>12；end_block = ceil(end/4096)；位图不足 → grow；越界 → panic
ensure_current_snapshot 0x140a439b0：
   push 当前追加偏移进 edit_offsets 列表 → 零填充扩容 → 写 u16 0x0a2c (",\n") →
   MultiEdit::serialize（0x80B 缓冲，FUN_140b39b80）→ 追加 JSON + mark_dirty(旧端, 新端)
flush 0x1409193d0：未开 → 建 <path>.tmp → Write data → Setting file length → Sync data
sync_all 0x14091a230：flush → 断言打开（+0x40 < 0）→ OS sync → flushed_block_count@+0x78
```

---

## 2. 逐函数算法

### 2.1 save_thread 0x141720680 主循环（O，全文）

```
Prologue：全局序列 FUN_1400d9b20；time 基线 FUN_140d635e0；state = 3（无当前存档）
do {
  1. time 快照：前进 ≥1s（无符号比较）且 state != 3 且脏列表非空 →
       第一个脏条目 ShadowFile::sync_all 0x14091a230
       → 结果组装 0x40B 条目 {路径, 状态}（FUN_140d61b00 + FUN_141720490）
       → push 进失败列表（grow FUN_1405627b0 @0x142cca8c0）
  2. condvar 等待：mutex LOCK；未通知 → wait_timeout(1s)（FUN_1428cd210, 1e9 ns）；UNLOCK
  3. 失败列表 diff（与上轮）：0x40B 条目逐字段比较（FUN_140b0d840 结构比较 +
       +0x20 int + +0x38 qword + +0x30 串 memcmp），首个差异处断开
       → 新列表空：log "Auto-save I/O error resolved"（info，级别 >2）
       → 非空：log "Save thread I/O error; affected file count: "（warn，级别 !=0）
       → FUN_1412f86a0 + FUN_1408da430 经 channel 回传游戏线程；交换新旧列表
  4. state != 3 → drop 当前存档（枚举 0/1/2 → FUN_14167c600/1416777a0/1416d6a60）
  5. 断言 locked_backups.remove(&path) 成功（panic "assertion failed:
       current_save_file.locked_backups.remove(&path_buf)" @0x142cca978, line 0x44）
  6. recv 命令（FUN_14152d950）→ 按命令变体解码（0/1/2 → FUN_14167c8a0/141677330/
       1416d64e0）
  7. 变体 0xb（Shutdown）：
       - 需 flush（标志字节）→ state=3 → CurrentSaveFile::flush_deferred_and_sync
         0x14171f7f0；失败 → log "Error flushing save file on game thread disconnect."
         （warn，级别 !=0，@0x142ccaa60）→ channel 发回确认（FUN_140052a30）
         → drop 全部资源 → return（线程退出）
       - 否则：state==3 → 睡 10ms（0x989680ns）继续；state!=3 → 直接 return
  8. 变体 3（OpenFile）→ save_folder_path + "history.json"（0x142cca6a8, len 0xc）
       → UnsavedFileList::contains_file 0x1417201a0 → 命中 → panic "Was asked to load
       an unsaved file"（@0x142ccaa00）
  9. execute_save_cmd 0x141721aa0
} while(true)
```

### 2.2 execute_save_cmd 0x141721aa0（O，switch 面 = {3,4,5,6,7,9,10}+default）

公共断言（case 3/4/5/6/7/9/10 头部共用）：从 0x142cca478 区（51 字）反向扫 `/` `\`
取源文件路径（panic 消息用）→ `truncate <= history_edit_offsets.len()` 断言
（panic "assertion failed: truncate <= self.save_file_structure.history_edit_offsets.
len()"，@0x142cca478/0x142cca410）。

- **case 3 OpenFile**：
  1. debug log "AsyncSaveCmd::OpenFile("（级别 >2，@0x142ccaaa0）；
  2. 取当前存档状态；state != 3 → 拷旧态（0x2a0B）→ `flush_deferred_and_sync
     0x14171f7f0`（**开新档前先 flush 旧档**）→ 失败 → warn "Flush error while closing
     previous save file (upon open request)"（@0x142ccaf00）→ 新文件登记
     UnsavedFileList::add（FUN_14171ff80）；
  3. save_folder_path → join "snapshots"（@0x142ccaf28）→ 快照目录路径；
  4. **全量重写**：组装 GameSaveData 参数面 → `save_game_to_file<Cursor<Vec<u8>>>
     0x141750c90` 生成完整 JSON → 失败 → log "Rewriting history file"（@0x142ccaf31，
     0x16 字）；
  5. join "history.json" → `ShadowFile::create 0x1409184d0` → 0x270B 状态组装 →
     存为新 CurrentSaveFile（state=3 变体），内含 shadow 文件、快照目录、offsets 等；
  6. 附：creating snapshot folder / Snapshot GC 编排串在本 case 域。
- **case 4 CloseFile**：debug log "AsyncSaveCmd::CloseFile("（@0x142ccab08）；同公共
  断言；state!=3 → flush 旧态 → UnsavedFileList::add；收尾（清锁备份/通道）。
- **case 5 ChangeSection**：参数 {section tag, offset}；**state==3（无活动档）→ panic
  "No active save file, but AsyncSaveCmd::ChangeSection has been issued"**
  （@0x142ccab90/0x142ccaba0）；否则 change_section：当前节记录 + offset + mark_dirty ×2
  （round3 decomp-0x14171ece0 互证）。
- **case 6 ReplaceEdits**：参数 {byte 标志@+3, edit 列表 ptr/len}；state==3 → panic
  "No active save file, but AsyncSaveCmd::ReplaceEdits has been issued"（@0x142ccac38，
  先 debug log 级别 >1）；否则逐 edit 追加（"Edit appended to save file" @0x142cca538 /
  "Error appending edit to save file: " @0x142cca5b0）。
- **case 7 BackupSave**：参数 {path, kind}；`history-auto` 前缀备份；日志族：
  "Backup limit reached; removing " / "Error deleting stale backup " / "Error reading
  backup file list: " / "Error making an auto-save backup: "（@0x142ccaf00 区）。
- **case 9 CaptureSnapshot**：参数 {Arc<SnapshotSaveData>?, …}；**state==3 → 日志
  "No active save file, but AsyncSaveCmd::CaptureSnapshot has been issued"**；创建
  `snapshots` 目录（"creating snapshot folder"）→ 快照 JSON 落盘（节序见 §2.6）。
- **case 10 Flush**：state==3 → 同 No-active 日志（"…AsyncSaveCmd::Flush…"）；否则
  flush 当前档（"Error flushing changes to disk"）。
- **default（含 8=ManualBackup）**："Copying inactive save file to backup" /
  "Writing backup from shadow copy" / "Error flushing history file for backup"；
  按载荷比对文件身份字节（+0x53/+0x299 等）与 "snapshot"（0x746f687370616e73 常量）。
- 变体 0/1/2：无专属 case、无 debug 串（I：死/历史变体或不存在）。

### 2.3 shadow_file（O）

- **ShadowFile 布局**（本域新证）：数据 Vec cap-first {cap@+0x20, ptr@+0x28,
  len@+0x30}、追加偏移@+0x38、文件状态@+0x40（i64::MIN 哨兵 = Some/已开）、句柄@+0x48、
  脏位图 ptr@+0x60、位计数@+0x68、flushed_block_count@+0x78。
  （DIGEST 的"+0x28/+0x30 (ptr,len)"为部分视图；cap@+0x20 由 ensure_current_snapshot
  的 grow 调用证实。）
- **flush 0x1409193d0**（unwrap_open_mut 同址）：未开 → 建 `<path>.tmp`（串 .tmp
  @0x142aaca18）→ 写数据（"Write data"）→ "Setting file length" → "Sync data" →
  log "Save file successfully opened and written to disk"；错误 "Could not create temp
  file for save rewrite: " / "Error writing save data to temp file: "。
- **sync_all 0x14091a230**（flush_impl）：先 flush；断言 `+0x40 < 0`（否则 panic
  "ShadowBackendFile is not open" @0x142aace80）；OS sync；"Asking the OS to sync writes
  to disk"（@0x142aad1d8, 0x24 字）；flushed_block_count@+0x78 落盘；返回 ok-bool。
- **mark_dirty 0x14091a450**（区间协议）：`end <= start → return`；
  `start_block = start>>12; end_block = ceil(end>>12)`（**4096B 块**）；
  位图不足 → grow（FUN_140915c20 @+0x60）；越界 → 格式化 panic @0x142aabe08；
  置位 [start_block, end_block)；debug log "Marking block range dirty: .."
  （级别 >4，@0x142aad220）。**DIGEST 的"8 槽 &7"描述修正为 4KB 块位图**（&7 是
  字内位偏移运算）。

### 2.4 ensure_current_snapshot 0x140a439b0（= add_history_edit_to_save_file，O）

1. `offset = 影子文件+0x38`（当前追加偏移）→ **push 进 edit_offsets 列表**
   （param_3 = Vec<u64> cap-first）。
2. **写分隔符**：容量检查（grow FUN_1428a9a60 @+0x20，零填充 gap）→
   `*(u16*)(buf+offset) = 0x0a2c`（**",\n"**）→ len = offset+2 → `mark_dirty(offset, offset+2)`。
3. **序列化追加**：alloc 0x80B（128B）缓冲 → `FUN_140b39b80(param_2, &mut 缓冲)` =
   **MultiEdit::serialize**（h63c9 单态，直接以 &MultiEdit 调用）→ 扩容 + 零填充 +
   memcpy JSON → len = offset+2+json_len → `mark_dirty(旧端, 新端)`。
4. 失败 → log "Error appending edit to save file: " 族（@0x142acdcf0 区, 0x2b 字）。
   （round2 CLOSURE 的"History+0x38 快照 id 域"描述不准确——+0x38 是影子缓冲的追加
   偏移；CLOSURE 的 edit_offsets 语义正确。）

### 2.5 快照捕获（O）

- **capture_game_snapshot_save_data 0x1417a0430**：串匹配 @0x142cdc3b0（0x46 字，
  panic @0x142cdc480/0x142cdc498）+ 路径 join（FUN_140c16d40）→ 从 ~9 组世界数据
  （FUN_141305170/141308b40/141309840 收集器，alloc 错误 @0x142cdc1a8）构建 ~0x400B
  快照结构（JSON 键串表 @0x142a93bfe 起：walls/roofs/…）→ 返回 Arc（alloc 0x18
  {strong=1,weak=1} + LOCK inc）。
- **capture_game_snapshot 0x1417a1d00**（184KB 系统体）：全 Query 串化；
  StairsSnapshot{edges, checksum} / TerrainEditingData{strokes, next_stroke_id,
  next_stroke_index} / DecoratorStorageSnapshot{states}。
- **validate_and_load_closest_snapshot_data 0x1417a6e50**（快照 JSON 解析器）：
  按节名键分派（FUN_1414744d0 匹配 + 各节跳表 @0x142cdca80/0x142cdca98/0x142cdcab0）：
  **节序 Version → Walls → Roofs → Decorators → WallColors → GateColors → Terrain
  → Trees → Raster**；每节 log "parsing X"（@0x142cdc709 块）；Version 不符 → panic
  "Version mismatch; expected , got "（@0x142cdc840）；Terrain 数据 **flate2 deflate
  解压**（"decompressing terrain data" @0x142cdc800）；Raster 键白名单外 →
  "Ignoring unknown raster data in snapshot: "（@0x142cdc7c0）。
- **init_game_state_from_snapshot 0x1417a9c50**：load_game 拷贝 0x1b0B 解析数据 →
  应用各域资源（颜色/时间/羊/地形…）。

### 2.6 save_game 0x140a3f470（O）

1. **备份节拍器（内联，行 ~507-591）**：
   - 字段 {counter u32@+0x140, 累计 f32@+0x144, 阈值 f32@+0x148}；
   - 仅当 `阈值 < 60.0`（或 NaN）时累计：断言 `delta >= 0.0`（"assertion failed:
     active_game_delta_seconds >= 0.0" @0x142a9f2c0）→ `累计 += delta` → clamp [0,1]；
   - `累计 >= 1.0` → **counter++ 并扫除数桶 {300, 600, 3600(0xe10), 7200(0x1c20)}**
     （@0x142a9f348 起）——按 **counter % 除数** 从大到小找匹配；命中后
     `counter -= 除数` 可**一次消耗多个间隔**（内层循环按升序 {300,600,3600,7200}
     重新校验整除，除数桶内容为 0 时 panic 除零）；
   - 备份触发标志 → 捕获流程。
2. **捕获**：log "Capturing a game snapshot"（info，模块 country_core::systems::
   save::save_game）→ capture_game_snapshot_save_data → Arc 克隆 →
   History::set_current_snapshot（0x140a84760）→ FUN_1408da280 枚举分发发
   AsyncSaveCmd::CaptureSnapshot(9)。
3. **同步保存**：错误串 "Error while performing a synchronous save: " /
   "Sync response from save thread" / "Failed writing session meta: "。
4. **GameSaveData 组装**：从 ~9 资源拷贝（历史链/时间/相机/羊/glade 字节等，
   +0x28..+0x100 密集字段）→ save_game_to_file。
- **save_game_to_file<Cursor 0x141750c90 / File 0x140241ca0>**：手工拼 JSON（写
  `'{'` 0x7b、`"History":{` 字面量、edit 间 `",\n"` 0xa2c）。
- **write_json_sections<Cursor 0x141751c90 / File 0x140242b40>**：遍历 0x28B 节条目；
  每节：json_key（FUN_140d79f90 格式化）+ 载荷 to_json_string（FUN_140b883f0）。
- **SaveFileSection 协议**：10 节（0 Version/"Version"/0、1 Identity/"Identity"/16、
  2 BackwardCompatibility/0、3 Camera/"Camera"/128、4 TimeOfDay/"Time of day"/32、
  5 Sheep/"Sheep"/128、6 Colors/"Colors"/64、7 chain_index/8、8 history_start/8、
  9 snapshot/"snapshot"/2048）；json_key 查表 @0x142ace328（len）+@0x142ace378（ptr），
  padding @0x142ace3c8 = [0,16,0,128,32,128,64,8,8,2048]。
- **save_session_meta_to_file 0x1417522e0**：SessionMeta serialize → File::create
  （FUN_140d59dc0）→ write_all（FUN_1415a55d0）→ sync（FUN_140d597e0）→ 错误日志
  @0x142cd3bb0/0x142cd3b90 族（"Failed writing session meta" 系）。

### 2.7 load_game 0x14146d030（O，重点区）

- **门控**：`*app_mode == 0xe`（LoadGameData 态）→ 否则 return。
- **状态字节 +9**：`0` → 按 +0x4c 计数（可负）发 16B 加载进度记录
  {计数器, tag 0x3010001 或 0x3010101} 入 Vec<16B>，置状态 2，return（多帧加载节拍）；
  `2` → 主流程；其他 → panic @0x142c641d0（0x28 字）。
- **主流程（状态 2）**：
  1. 拷走并清空 on_load 列表（{cap,ptr,len} = {0,8,0}）；
  2. **NextSessionKind 解码**：`decoded = kind ^ 0x8000000000000000; sel = min(decoded, 2)`；
     - sel==1 → `parse_saved_game_data`（FUN_141470d00 内联于本函数行 ~413 区）：
       节序 "Making save file backup for upgrade" → "Reading file data" →
       "Parsing JSON" → "Version" → "BackwardCompatibility" → "Camera" →
       "parsing camera" → "Time of day"（缺失 → "This save file had no 'Time of day'
       data"）；失败 → "Couldn't load save file from "；**upgrade 前先做存档备份**；
     - sel==2 → 内存路径：manager 状态字节 +9=2、无快照标记 +0xd=0xffffffff、
       on_load 命令回放；
  3. **MetaCommand 回放**（on_load 列表，条目 4B tag）：`tag == 0x8000000000000003`
     → **fence 终止**；`tag ^ 0x8000000000000000` 解码（>2 钳 1）：变体 0 → 写 manager
     状态 + push 24B 事件；变体 1 → `FUN_140b5a180` push MetaCommand（meta_commander.rs
     路径串 @0x142c5ae38）；变体 2+ → Arc LOCK dec drop；
  4. **PostGameLoadCommand 回放**（第二轮 on_load 遍历）：变体 0（SwitchAppMode）→
     log "Ignoring PostGameLoadCommand::SwitchAppMode"（级别 >1，@0x142c642e0）跳过；
     变体 1 → push 24B {tag, 载荷×2}（grow @0x142c64308）；变体 2+ → 同上；fence 终止；
     断言 "Command skipping must not be used together with snapshots" /
     "Can't skip "（@0x142c64378/0x142c643d8 区）；`FUN_1412e0b90(param_8, &列表, …)`
     应用命令；
  5. **版本 → 全局开关**：`0x14362c308 = (v > 0x19)`；`0x14362c011 = (v > 0x1a)`
     （**history 域 add_decorator_diff 楼梯 checksum 校验的启用条件**——存档版本
     ≥27 才校验）；
  6. **旧版颜色 ID 升级**（"Detected old version of window color IDs. Upgrading..." /
     "Detected old version of arrow slits color IDs. Upgrading..."，@0x142c64a90/
     0x142c64af0）：遍历 0x28B MultiEdit 元素内 0x58 步长记录，`+0x52 字节 ∈
     {0x38..0x3a}` 者其 `+0x30` 字段按 {0,3,6,7 → 原值; 1→2; 2→5; 4→8; 5→3; 8→4;
     999 → 999; default → 1} 改写；
  7. **extract_max_wall_id_from_history**：遍历 MultiEdit（变体 niche 解码），取最大
     wall id → 写计数器资源 +0xc0；
  8. **init_game_state_from_snapshot**（FUN_1417a9c50）：拷贝 0x1b0B 解析数据应用，
     随后清理（快照 Arc 列表 LOCK dec 循环）；
  9. 事件：LoadingGameComplete / StartupRevealStarted/Finished；
     "Cloning game state for save rewrite took "（@0x142c64b48）计时日志。
- 后置：写入 `save_folder/loading.tmp`（@0x142c6416e）；新 glade 分支 log "Loading a
  new glade. Spawn pre-placed trees"（@0x142c641a8）+ 事件 push；缺失 Query 资源 →
  bevy_ecs-0.16.0 panic 串。

### 2.8 load_completion_checker / save_backup_list / save_file_identity（O）

- check_load_completion 0x140a9f810：save_folder_path + join + 文件存在性探测
  （File::create×2 模式）；is_session_ok_to_load 0x140a9fa80：save_folder_path×2 +
  join×2 + File::create×4（会话就绪检查）。
- BackupSaveFileKind::is_corrupted 0x140ae8320：`tag == 4`；iter_backup_files 0x140ae8330；
  new_from_session 0x140ae8470：扫 `history-auto` 前缀（错误 "Error reading save backup
  directory" / "Error creating backup save folder: "，@0x142ae367c）。
- try_approx_parse_save_file_identity 0x141e58f20：0x800B 缓冲 → ReadFile → UTF-8 →
  搜字面量 `"Identity"`（带引号，@0x142f92a40 len 10）→ 解析其后 JSON 值为
  SaveFileIdentity；失败回退 derive_fallback 0x141673140（SystemTime + 二次尝试 +
  文件元数据派生，字段 U）。

### 2.9 快照 GC（O）

- **snapshot_gc_thread 0x14172b280**：`FUN_141286730` = parse_save_file_snapshots
  <BufReader<File>>（解析存档内快照引用集合）→ 与 `snapshots` 目录 read_dir 比对 →
  未引用 → 删除（"Deleting unreferenced snapshot " @0x142ccb088/0x142ccb0e0）→
  结果经 channel 回传（FUN_1408da040）；竞态守卫 "Save file changed before snapshot GC
  could finish. Shutting down the GC thread"；错误 "Error opening backup file " /
  "Error parsing history snapshots fromfile"（@0x142ccb168 区）。

### 2.10 session.rs 资源层（O，DIGEST 基础上补算法）

- SessionId::new 0x140916420 → Uuid 生成（0x140c102e0）；
  save_folder_path 0x140b46000 = paths::saves()/uuid（Display fmt 失败 panic "a Display
  implementation returned an error unexpectedly" @0x142aef358）；backup_folder_path
  0x140b46180 = paths::backups()/uuid（**独立顶级目录**，round2 闭合）；session_file_list
  0x140b46300 = {history.json, screenshot.jpg, meta.json} 三叶名路径；
  loading_background_path 0x140b46a50 = +"loading_background.jpg"。
- SessionMeta::try_load_from_path 0x140b46480：读文件 → serde（字段名 {state, glade,
  mutators, image}，首字节分发表 @0x142aef526；字段 4 载荷 = serde_json::Value）；
  is_halloween2024 = any(b==0)、is_halloween_any = any(b<=1)（方向纠错）、
  is_xmas2024 = any(b==2)；assets_root 0x140b46370 = "assets/glade" + 主题表
  {summer/olden/flowery/autumn/winter}[+0x58]；write_to_disk 0x140b463c0 =
  save_folder_path+"meta.json" + save_session_meta_to_file。
- NextSession::load 0x140b46920：{Vec(cap=8), kind=0x8000000000000002, uuid@+0x20,
  ignore_snapshot=false@+0x98}；create 0x140b46960：96B 模板 + 32B 尾部构造（kind 由
  调用方模板决定——变体 1 生产者在 UI/会话流 vtable，卡点）；ignore_snapshot
  0x140b46a20：写 +0x98 + memcpy 0xa0（builder）；append_on_load<MetaCommandFence>
  0x141328ac0：push 0x20B {u32 tag=0, u32 0x80000000, u64 payload} + memcpy 0xa0。
- delete_session 0x140b46d60：deleted_saves()/uuid 标记文件 → rename 移动 → 两段删除；
  失败 log "Error deleting glade: "（@0x142aef5a0）。
- loading_screen：startup 0x140a1a1e0（load_mesh/shader + spawn 0x88B bundle）；
  update 0x140a1a190（占位 UUID 33333333-3333-3333-3333-333333333333 @0x1429503e0
  键，两张 hashbrown 表 +0x70/+0xd8，0xd0B 条目）。
- logo_animation：animate_logo 0x140b1a6b0（5 阶段进度 + 时长表；阶段 0 起始播
  event:/UI/Logo/Scribble、阶段 2 起始播 event:/UI/Logo/Branches）。
- prefab：PrefabLibrary::get/has（hashbrown 0x28B 桶 = {hash, PrefabName 16B,
  Prefab 16B}——Prefab 值 16B = {ptr,len} 胖指针类）；create_prefab_rt 0x140841550
  （位置查表 @0x142a94358 → 旋转矩阵 + 缩放 → GPU 实例缓冲 + 打包颜色
  b<<16|g<<8|r → +0x84 标志 &1 spawn / ==2 跳过）；hot_reload 0x140843740
  （despawn → 重建链）；clone_behavior 0x140002370 = ret 0（false）。

---

## 3. 状态机

### 3.1 CurrentSaveFile（存档线程侧）

```
*state == 3 → 无活动存档（OpenFile 前置检查/ChangeSection/ReplaceEdits/CaptureSnapshot/
Flush 的 "No active save file, but AsyncSaveCmd::X has been issued" panic 条件）
*state ∈ {0,1,2} → 三种存档形态（drop 分派 FUN_14167c600/1416777a0/1416d6a60；
字段 U）
OpenFile：旧态 flush（若 != 3）→ 全量重写 → 新 state=3 形态装载
```

### 3.2 LoadGameManager（读档侧）

```
+9 == 0：发加载进度记录（16B {counter, tag}），置 2，return
+9 == 2：NextSessionKind 解码（1=解析存档文件+upgrade 备份 / 2=on_load 命令回放+内存快照）
+9 其他：panic
```

### 3.3 影子文件脏区间

```
位图（4KB 块粒度）：mark_dirty(start,end) 置位；sync_all 落盘后 flushed_block_count@+0x78
记录；save_thread 每 1s 逐文件 flush（失败条目 0x40B 回传游戏线程）
```

## 4. 常量表

| 常量/静态 | 值 | 用途 |
|---|---|---|
| 0x142cca6a8 | "history.json"（len 0xc） | OpenFile 影子档名 |
| 0x142ccaf28 | "snapshots" | 快照目录名 |
| 0x142cca560 / 0x142ccaac0 | log target（country_core::systems::save::save_thread 系） | 全函数日志 |
| 0x142a9f348..0x142a9f354 | {300, 600, 3600, 7200} | 备份档位秒表 |
| 0x142925904 | 1.0f | 节拍器触发阈值 |
| 0x142934604 | 1.0f | clamp 上界 |
| 0x14295a0e4 | 1.0f | 计数器步进 |
| 0x142a9f2c0 | "assertion failed: active_game_delta_seconds >= 0.0" | 节拍器断言 |
| 0x142c5ae38 | crates/country-core/src/meta_commander.rs | load_game MetaCommand push |
| 0x1429503e0 | 占位 UUID 33333333-3333-3333-3333-333333333333 | loading_screen 键 |
| 0x142aef526 / 0x142c3a870 | SessionMeta serde 字段分发表 / {state,glade,mutators,image} | meta.json |
| 0x142f92a40 | `"Identity"`（含引号 len 10） | 存档身份近似解析 |
| 0x142cdca80/0x142cdca98/0x142cdcab0 | 快照节跳表 | validate_and_load 分派 |
| 0x142a93bfe 起 | 快照 JSON 键串表（walls/roofs/…） | capture 串化 |
| 0x14362c011 / 0x14362c308 | 版本门控（>0x1a / >0x19） | load_game 写；history 楼梯校验开关 |
| 0x746f687370616e73 | "snapshot" u64 常量 | execute_save_cmd default 区身份比对 |

## 5. panic/断言算法级清单

| 串 | 地址 | 触发 |
|---|---|---|
| assertion failed: truncate <= …history_edit_offsets.len() | 0x142cca478/0x142cca410 | case 3-10 公共 |
| assertion failed: current_save_file.locked_backups.remove(&path_buf) | 0x142cca978 | save_thread 每轮 |
| No active save file, but AsyncSaveCmd::{ChangeSection,ReplaceEdits,CaptureSnapshot,Flush} has been issued | 0x142ccab90/0x142ccac38/0x142ccaf00 区 | state==3 |
| Was asked to load an unsaved file | 0x142ccaa00 | OpenFile 前置 |
| Error flushing save file on game thread disconnect. | 0x142ccaa60 | Shutdown flush 失败 |
| ShadowBackendFile is not open | 0x142aace80 | sync_all 断言 |
| assertion failed: new_len <= buf.len() as u64 | 0x142aad048 | shadow 扩容 |
| assertion failed: active_game_delta_seconds >= 0.0 | 0x142a9f2c0 | 节拍器 |
| Version mismatch; expected , got  | 0x142cdc840 | 快照解析 |
| a Display implementation returned an error unexpectedly | 0x142aef358 | uuid 格式化 |

## 6. 跨域移交

- 0x14362c011（stairs checksum 开关）由 load_game 按存档版本写 → history 域
  add_decorator_diff 消费（本文件互证闭合）。
- AsyncSaveCmd 0/1/2 存在性、NextSessionKind 变体 1 生产者、MetaCommand 头
  {0, 0x80000000} 语义 → UI/会话流（vtable 间接调用，卡点保留）。
