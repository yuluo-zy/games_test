# session-save 域深挖 DIGEST（tg_a3，2026-10-14）

批量反编译 66 目标 / 57 成功、9 NotFound（已全部手解）。产物在 `decomp/`，
函数清单 `fnlist-full.txt`（batch PDB 名）、`sysres-domain.txt`（sysres-procedures.json 权威清单）。
**只写文档，未改 reconstruction 源码。**

## 1. 模块清单

| 文件 | 关键过程（地址） | 状态 |
|---|---|---|
| resources/session.rs | SessionId::{new 0x140916420, save_folder_path 0x140b46000, backup_folder_path 0x140b46180, session_file_list 0x140b46300}, SessionMeta::{try_load_from_path 0x140b46480, default 0x140b468f0, is_halloween2024 0x140b46860, is_halloween_any 0x140b46890, is_xmas2024 0x140b468c0}, NextSession::{load 0x140b46920, ignore_snapshot 0x140b46a20, append_on_load<MetaCommandFence> 0x141328ac0}, SavedSession::loading_background_path 0x140b46a50, delete_session 0x140b46d60 | 主体 O |
| resources/loading_screen.rs | LoadingScreenPlugin::build（实际位于 update_loading_screen 邻域）、startup_loading_screen 0x140a1a1e0、update_loading_screen 0x140a1a190（hashbrown UUID 键 0xd0B 条目管理） | O+U |
| resources/logo_animation.rs | play 0x140b19d00 / is_waiting 0x140b19d10 / is_finished 0x140b19d22 / finish_wait 0x140b19d33 / Plugin::build 0x140b19d40 / startup_animated_logo 0x140b19d90 / animate_logo 0x140b1a6b0 | O |
| systems/save/save_game.rs | save_game 0x140a3f470、save_game_to_file<Cursor> 0x141750c90 / <File> 0x140241ca0、write_json_sections×2 0x141751c90 / 0x140242b40、SaveFileSection::json_key 0x140a43810、AutoSaveState::close_file 0x140a43840、ensure_current_snapshot 0x140a439b0、save_session_meta_to_file 0x1417522e0、to_json_string×7（0x14085ee90 Colors / 0x140b38810 HistoryChainSnapshots / 0x140b385e0 HistoryIdentity / 0x1408aa7a0 HistoryChainIndex / 0x140b389b0 TimeOfDay / 0x1416f8470 CameraRigParametersCache / 0x140a51d40 Option<SheepControlSnapshot>） | O |
| systems/save/save_thread.rs | save_thread 0x141720680、execute_save_cmd 0x141721aa0（AsyncSaveCmd 3..11 分发）、CurrentSaveFile::{flush_deferred_and_sync 0x14171f7f0, backup_save 0x14171ff80}、UnsavedFileList::contains_file 0x1417201a0、save_thread::closure$1 0x14172b280（快照 GC 线程） | O |
| systems/save/shadow_file.rs | ShadowFile::{flush_impl 0x14091a230, mark_dirty 0x14091a450}、ShadowBackendFile::unwrap_open_mut 0x1409193d0 | O |
| systems/save/snapshot.rs | capture_game_snapshot_save_data 0x1417a0430、capture_game_snapshot 系统体 0x1417a1d00（184KB closure）、get_param 0x1417ab290 / validate_param 0x1417a6e50（**后者实为 validate_and_load_closest_snapshot_data**）、init_game_state_from_snapshot 0x1417a9c50 | O（get_param 为 Bevy 胶水，未逐行） |
| systems/save/snapshot_gc.rs | parse_save_file_snapshots<BufReader<File>> 0x141286730 | O（供 GC 线程） |
| systems/save/load_game.rs | load_game 0x14146d030、parse_saved_game_data 0x141470d00（内联于 load_game 内） | O |
| systems/save/load_completion_checker.rs | check_load_completion 0x140a9f810、is_session_ok_to_load 0x140a9fa80、SessionLoadReadiness::is_ok 0x140833230、deny_load 0x140a9fa70 | O（部分语义 I） |
| systems/save/save_backup_list.rs | BackupSaveFileKind::is_corrupted 0x140ae8320、iter_backup_files 0x140ae8330、new_from_session 0x140ae8470（未在批内，串证据） | O |
| systems/save/save_file_identity.rs | try_approx_parse_save_file_identity 0x141e58f20 + closure 0x141673140 | I |
| systems/save/save_backup_metronome.rs | （内联于 save_game 0x140a3f470 行 ~507-591 与 load_game） | O |
| systems/prefab.rs | PrefabPlugin::build 0x140840430、clone_behavior 0x140002370、register_required_components 0x1408446b0、create_prefab_rt 0x140841550、create_prefab_audio_trigger 0x140841b00、create_prefab_gpu_instances::closure$0 0x140841d60、despawn_prefabs 0x140842d60、despawn_prefab_contents 0x140843390、hot_reload_prefabs 0x140843740 | O |
| resources/prefab.rs | PrefabLibrary::get 0x141552d70 / has 0x141552ff0（hashbrown 查表，机器码核验） | O |
| render_loop/session_selection（邻域） | build 0x1408f68c0 / thumbnails 0x1408f6c00 | 未在本批（渲染域），仅登记 |

## 2. 布局表（机器码/反编译证据）

### 2.1 NextSession（Resource，0xa0=160B，O）
| 偏移 | 含义 | 证据 |
|---|---|---|
| +0x00..+0x18 | 待应用 on_load 命令 Vec（**cap-first**：cap@+0、ptr@+8、len@+0x10） | NextSession::load 机器码：`[rcx]=0;[rcx+8]=8;[rcx+0x10]=0`（cap=8 空 Vec） |
| +0x18 | NextSessionKind（enum niche：变体 i 存 0x800000000000000(i+1)） | load 写 `[rcx+0x18]=0x8000000000000002`；load_game 以 `x^0x8000000000000000` 解码 |
| +0x20..+0x30 | SessionId（uuid 16B） | load：`movups xmm0,[rdx]; movups [rcx+0x20],xmm0` |
| +0x98 | ignore_snapshot bool | 0x140b46a20：`mov byte[rdx+0x98],r8b; memcpy 0xa0` |
| 其余 | 会话参数/模板 | U |

- append_on_load<MetaCommandFence> 0x141328ac0：元素 **0x20B**，push 记录 `{u64 0x8000000000000000, fence_ptr}`（**reanalyst 2026-10-15：RNE 原字节证实单 u64 niche，旧"{u32 0, u32 0x80000000, u64 payload}"修正**；+0x10 区为 alloc 残留），`memcpy 0xa0` 返回副本（builder 模式）。append_on_load<[MetaCommand;3]> 0x1403b5150 = alloc 0xa8+memcpy Box 化 → push `{3, Box<[MetaCommand;3]>}`（调用方 run_game_thread 0x1405a3440）。
- on_load 列表以 **0x8000000000000003** 为 **fence 哨兵**（load_game 回放循环两遍的终止条件，O；元素步长 0x20 已证实）。

### 2.2 LogoAnimation（Resource，≥0x28B，O）
| 偏移 | 含义 | 证据 |
|---|---|---|
| +0x00..+0x14 | 5 阶段进度 f32（stage[0..5]） | animate_logo：`pfVar1[0..4]` 逐阶段比较/推进；play() 清零 +0..+0x14 |
| +0x0c | 第 4 阶段（等待）进度；is_waiting = `(+0x0c) > 0.0` | 0x140b19d10 机器码：`movss xmm0,[rcx+0xc]; ucomiss 0.0; seta` |
| +0x10 | 第 5 阶段（完成揭示）进度；is_finished = `(+0x10) >= 1.0`（常量 @0x142ae8b10） | 0x140b19d22 |
| +0x14..+0x28 | 5 阶段时长 f32 | animate_logo：`pfVar1[5..9]`；build 默认值 = {const×4, 0.5f} |
| 方法 | play 0x140b19d00（清零 +0..+0x14）；finish_wait 0x140b19d33（`[rcx+0xc]=1.0f`）；build 0x140b19d40（insert_resource + Startup startup_animated_logo + Update animate_logo） | O |

- animate_logo（Update 系统 `(ResMut<LogoAnimation>, Res<Time>, Res<AudioCore>)`，O：param[2]=param+0x1c tick 回写）：
  顺序找第一个 progress<1.0 的阶段 i，`stage[i] = clamp(stage[i] + dt/duration[i], 0, 1)`；
  **阶段 0 起始（stage[0]==0.0）播 FMOD `event:/UI/Logo/Scribble`，阶段 2 起始播 `event:/UI/Logo/Branches`**（O，串 @0x142ae945b/0x142ae9472，代码内联拼 23 字符事件名）。

### 2.3 SessionMeta（meta.json 内容，O+I）
- 季节谓词（**机器码为准**，全部为“存在字节匹配”扫描）：
  - is_halloween2024 0x140b46860：`bytes.iter().any(|b| *b==0)`
  - is_halloween_any 0x140b46890：`any(|b| *b<=1)`（`ja` 跳过 `>1`；**合同写 “>1” 方向相反，纠错**）
  - is_xmas2024 0x140b468c0：`any(|b| *b==2)`
  - 季节字节编码（I）：0=halloween2024、1=其他万圣节、2=xmas2024。字节字段 @+0x08(ptr)/+0x10(len)。
- SessionMeta::default 0x140b468f0（机器码）：`{+0x00:1, +0x08:1(dangling ptr), +0x10:0(len=0), +0x18:0x8000000000000000(Option None 哨兵), +0x58:byte 0}`。
- try_load_from_path 0x140b46480：读文件 → serde 反序列化。**serde 字段名（O）**：`"state"`（首字节分发表 @0x142aef526）、`"glade"`、`"mutators"`、`"image"`（串块 @0x142c3a870/0x142c3a890）；字段 4 载荷为 serde_json::Value（serde_json-1.0.85 value/mod.rs 证据）。6 个 visit 分支：0=unit、1=byte、2=(u64,u32,u32)、3=拷贝blob、4=json Value、5=String。
- 失败 panic：`a Display implementation returned an error unexpectedly`（@0x142aef358，session.rs）。

### 2.4 会话目录协议（O，串块 @0x142aef4f7 起）
- 会话目录文件：**history.json（存档主文件）、screenshot.jpg、meta.json、loading_background.jpg**，子目录 **assets/glade**（assets_root 叶名）。
- session_file_list 0x140b46300：对 {history.json, screenshot.jpg, meta.json} 三叶名迭代构建完整路径（机器码：3 个 (ptr,len) 对 + collect；**PDB 名 snapshots_folder_path 与本行为不符，判为错位**；合同已把 0x140b46300 记作 session_file_list ✓）。
- SavedSession::loading_background_path 0x140b46a50：save_folder_path + `"loading_background.jpg"`（**PDB 名 screenshot_path 错位**）。
- backup_folder_path 0x140b46180：saves()/uuid 基础上再拼接（Display 格式化，失败 panic “a Display implementation returned an error unexpectedly”）；叶名常量 @0x140aef328 为 {fn ptr 0x1405d45f0, 0x18, 8} 描述符（未解出明文）。
- delete_session 0x140b46d60：`deleted_saves()/uuid` 目标路径 → 创建标记文件并写入 → rename 移动会话目录 → 失败日志 `Error deleting glade: `（@0x142aef5a0）。两段删除（移入 deleted_saves，后清）。

### 2.5 SaveFileSection → 存档载荷映射（O+I）
| 节 | json_key | padding | 载荷类型（to_json_string 实例证据） |
|---|---|---|---|
| 0 Version | `Version` | 0 | 版本号（u64） |
| 1 Identity | `Identity` | 16 | **HistoryIdentity**（0x140b385e0） |
| 2 BackwardCompatibility | `BackwardCompatibility` | 0 | 兼容节（Value） |
| 3 Camera | `Camera` | 128 | **CameraRigParametersCache**（0x1416f8470；**reanalyst 2026-10-15 schema 全解 = 6 字段** `{yaw@+0, pitch@+4, position@+8(Vec3), camera_rig_arm_length@+0x14, pitch_offset@+0x1c, yaw_offset@+0x20}`，0x24B；"position_cache"串在 blob 0x1429cca3b 但 save 序列化不含——DIGEST 旧 7 字段表修正，见 state/save/RE-DOMAIN-TYPES §1） |
| 4 TimeOfDay | `Time of day` | 32 | **TimeOfDay**（0x140b389b0） |
| 5 Sheep | `Sheep` | 128 | **Option<SheepControlSnapshot>**（0x140a51d40） |
| 6 Colors | `Colors` | 64 | **GladeColorCustomization**（0x14085ee90） |
| 7 chain_index | `chain_index` | 8 | **HistoryChainIndex**（0x1408aa7a0） |
| 8 history_start | `history_start` | 8 | u64 |
| 9 snapshot | `snapshot` | 2048 | **HistoryChainSnapshots**（0x140b38810） |

- save_game_to_file：**手工拼 JSON**（写 `'{'` 0x7b、`"History":{` 字面量、历史 edit 间 `",\n"`（0xa2c）分隔符），节名走 json_key；两个实例（Cursor<Vec<u8>> 0x141750c90 / File 0x140241ca0）。

### 2.6 快照 schema（O，0x1417a6e50/0x1417a1d00 串证据）
- 顶层 JSON，节序（“parsing …” 日志序）：**Version → Walls → Roofs → Decorators → WallColors → GateColors → Terrain → Trees → Raster**。
- `Version mismatch; expected , got ` + `Failed to load history snapshot `（@0x142cdc688/0x142cdc840）。
- Terrain 数据 **flate2-1.0.24 deflate 压缩**（`decompressing terrain data`，zio.rs 证据）。
- `Ignoring unknown raster data in snapshot: ` — Raster 键白名单外忽略。
- 已知 serde 结构：`StairsSnapshot {edges, checksum}`、`StairsRemovedSupports {edges}`、`TerrainEditingData {strokes, next_stroke_id, next_stroke_index}`、`DecoratorStorageSnapshot {states}`。
- capture_game_snapshot_save_data 0x1417a0430：从 ~9 组世界数据构建 ~0x400B 快照结构，返回 **Arc<SnapshotSaveData>**（alloc 0x18 {strong=1,weak=1} + `LOCK inc` 克隆，O）。
- 巨型 capture 系统体 0x1417a1d00（184KB closure）串化全 Query（wall/roof/decorator/…）。

## 3. 方法/系统行为

### save_game（0x140a3f470，Update 系统，6 段 SystemParam，O）
1. **备份节拍器（内联 SaveBackupMetronome，O）**：结构位于某资源尾部 {+0x140: u32 counter, +0x144: f32 累计活动秒, +0x148: f32 阈值}。仅当阈值 `< 60.0`（或 NaN）时累计（`active_game_delta_seconds >= 0.0` 否则 panic @save_backup_metronome.rs）；累计值 clamp [0,1]；达到 1.0 时 counter++ 并查除数桶 **{300, 600, 3600, 7200}**（表 @0x142a9f348，即 5min/10min/1h/2h 备份档位），可一次消耗多个间隔。
2. 备份触发时：日志 `Capturing a game snapshot`（info，模块 country_core::systems::save::save_game）→ **capture_game_snapshot_save_data** → Arc 克隆 → `history.set_current_snapshot`（0x140a84760，PLAYBOOK 已证）→ **FUN_1408da280 枚举分发发送 AsyncSaveCmd（变体 9=CaptureSnapshot）**。
3. 同步保存路径：错误串 `Error while performing a synchronous save: ` / `Sync response from save thread` / `Failed writing session meta: `（+`called Result::unwrap() on an Err value`）。元数据落盘经 save_session_meta_to_file 0x1417522e0。
4. 构建 GameSaveData：从 ~9 资源拷贝字段至栈上大结构（历史链/时间/相机/羊/glade 字节等，+0x28..+0x100 密集字段写）。

### save_thread（0x141720680，线程主循环，O）
- 循环体：脏 shadow 文件逐 tick flush（ShadowFile::flush_impl）→ **mutex + condvar 等待 ≤1s** → 与上次列表 diff（0x40B 条目，memcmp）→ 差异记日志（`Save thread I/O error; affected file count: `（warn）或 `Auto-save I/O error resolved`（info））→ 错误列表回传游戏线程 → recv 命令（FUN_14152d950）。
- **AsyncSaveCmd 变体表（debug 串证据，O）**：3=OpenFile(`AsyncSaveCmd::OpenFile(`)、4=CloseFile、5=ChangeSection、6=ReplaceEdits、7=BackupSave、8=ManualBackup（`Copying inactive save file to backup`）、9=CaptureSnapshot、10=Flush、**11=Shutdown**（`(int)cmd == 0xb` → flush_deferred_and_sync + 退出，失败告警 `Error flushing save file on game thread disconnect.`）。0/1/2 未在本函数 switch 中出现（U）。
- execute_save_cmd 0x141721aa0：变体 3..10 大 switch。BackupSave（7）：`history-auto` 前缀、`Backup limit reached; removing ` / `Error deleting stale backup ` / `Error reading backup file list: `；ManualBackup：`Writing backup from shadow copy`；CaptureSnapshot（9）：创建 `snapshots` 目录（`creating snapshot folder`）、`No active save file, but AsyncSaveCmd::CaptureSnapshot has been issued`；Flush（10）：`Error flushing changes to disk`。增量历史写：`Edit appended to save file`、断言 `truncate <= self.save_file_structure.history_edit_offsets.len()`、`Rewriting history file`、`Saving the game to a temp file`、`Snapshot GC thread finished in `。
- **快照 GC 线程**（closure$1 0x14172b280，O）：parse_save_file_snapshots 解析存档内快照引用 → 与 snapshots 目录比对 → `Deleting unreferenced snapshot `；竞态守卫 `Save file changed before snapshot GC could finish. Shutting down the GC thread`；错误 `Error opening backup file ` / `Error parsing history snapshots fromfile `。

### shadow_file（O）
- ShadowBackendFile 内联字段：+0x28/+0x30 数据缓冲 (ptr,len)；+0x40/+0x48 文件状态（MIN 哨兵 + 句柄；None=0/负值 → `ShadowBackendFile is not open` panic）；+0x68 脏块计数（persist 后 &7 复位）。
- unwrap_open_mut：未开 → 创建 **`<path>.tmp`**（串 `.tmp` @0x142aaca18）→ 写数据（`Write data`）→ `Setting file length` → `Sync data` → 成功日志 `Save file successfully opened and written to disk`；错误 `Could not create temp file for save rewrite: ` / `Error writing save data to temp file: `。
- flush_impl（=Persist temp file）：写盘 + `Asking the OS to sync writes to disk`、`flushed_block_count: `。
- mark_dirty(start,end)：`Marking block range dirty: ..`（8 槽脏块区间跟踪）。
- ensure_current_snapshot 0x140a439b0：历史文件缓冲（cap-first Vec @+0x20..）追加当前 edit 计数入待写列表；追加 edit 间 `",\n"`（0xa2c）+ 序列化 JSON（0x80B 缓冲，FUN_140b39b80）→ 每次追加后 **mark_dirty(start,end)** 标记脏区间 → 增量写协议实锤。

### load_game（0x14146d030，主读档系统，13+ 段 SystemParam，O）
1. 状态日志 `LoadGameState::LoadGameData`（info）。写入 `save_folder/loading.tmp`（Windows HANDLE 路径，`loading.tmp` 串 @0x142c6416e）；新 glade 分支日志 `Loading a new glade. Spawn pre-placed trees` + 事件 push（Bevy 0.16.0 事件队列 grow）。
2. **NextSessionKind 解码**：`kind ^ 0x8000000000000000` 取 min(·,2)。==1 → parse_saved_game_data（含 `Making save file backup for upgrade`→`Reading file data`→`Parsing JSON`→`Version`→`BackwardCompatibility`→`Camera`→`parsing camera`→`Time of day`→`This save file had no 'Time of day' data`）；失败 `Couldn't load save file from `。==2（NextSession::load 设置）→ 走 on_load 命令回放 + 内存快照路径（`[+9]=2` 状态、`[+0xd]=0xffffffff` 无快照标记）。**（I）语义映射：1=解析存档文件、2=常规加载。**
3. 后处理：**PostGameLoadCommand 回放**（on_load 列表 4B 条目，fence 0x8000000000000003；变体 0=SwitchAppMode → `Ignoring PostGameLoadCommand::SwitchAppMode` 跳过；`Command skipping must not be used together with snapshots` / `Can't skip ` 断言）；**旧版颜色 ID 升级**（`Detected old version of window color IDs. Upgrading...` / `...arrow slits color IDs...`）：映射 {1→2, 2→5, 4→8, 5→3, 8→4}（0/3/6/7 不变、999 透传），应用于 0x58 步长结构（+0x52 字节 ∈ {0x38..0x3a} 者 +0x30 字段）与 0x10 步长表；**extract_max_wall_id_from_history**：遍历 MultiEdit（变体 7/8/9 载荷，niche 解码 `x-0x7fffffffffffffff`）取最大 wall id 写入计数器资源 +0xc0；`Cloning game state for save rewrite took `；`init_game_state_from_snapshot`（0x1417a9c50）应用数据。
4. Query 契约：`Query<&mut SheepControl>` 缺失 panic 串（bevy_ecs-0.16.0）。事件：LoadingGameComplete / StartupRevealStarted/Finished 等在系统内发送。

### load_completion_checker / save_backup_list / save_file_identity
- is_session_ok_to_load 0x140a9fa80（tag==2 判定候选，同址折叠域）、deny_load 0x140a9fa70（tag==1）、is_ok 0x140833230（is_initial 折叠）、check_load_completion 0x140a9f810（待处理计数，不足走 save_folder_path 补查）。
- BackupSaveFileKind::is_corrupted 0x140ae8320 = `tag==4`（机器码；**batch PDB 名 is_auto 错位**）；iter_backup_files 0x140ae8330 紧随其后；new_from_session 0x140ae8470 错误串 `Error reading save backup directory` / `Error creating backup save folder: `（save_backup_list.rs）。
- try_approx_parse_save_file_identity 0x141e58f20：近似解析（扫描文件头/JSON 前缀，I；细节 U）。

### prefab（O）
- PrefabLibrary::get/has：标准 hashbrown 查表（SIMD group probe；键 hash 0x1405b1a70；memcmp 0x142923bf0；桶 0x28B = {hash, PrefabName(16B), Prefab(16B)}——**Prefab 值 16B（I：Box<[PrefabPart]> 类 {ptr,len}）**）。
- PrefabPlugin::build：注册 2 个 Bevy 类型（type-id 0xa73802c6af7eaf33/0x188dddb3c672d859 与 0x98f655b4397350da/0x107a31ef8cad0cb1；I：PrefabLibrary + PrefabsModifiedCmd）+ 系统配置。
- create_prefab_rt 0x140841550：Prefab（位置查表 @0x142a94358 按变体）→ 四元数旋转矩阵 + 缩放 → GPU 实例缓冲 push（FUN_1415634c0）+ 打包颜色 u32（pfVar11[0x16] = b<<16|g<<8|r）→ 条件 spawn 实体（+0x84 标志 byte：&1 spawn、==2 跳过）。
- create_prefab_audio_trigger 0x140841b00：查 audio_trigger_library（0x140a382d0）→ 注册/生成（0x1408b9f20）。
- despawn_prefabs：逐实例（0x58 步长）despawn_prefab_contents → Commands::despawn（0x140d14770）。
- hot_reload_prefabs 0x140843740：watch 事件 → 加载 JSON 资产 → **despawn_prefab_contents → create_prefab_audio_trigger → create_prefab_rt** 全量重建实例链。
- clone_behavior 0x140002370 = `xor eax,eax; ret`（返回 0/false，O）。
- register_required_components 0x1408446b0：Bevy 组件注册胶水（ComponentId/registry），非业务。

### loading_screen（O+I）
- startup_loading_screen 0x140a1a1e0：once 初始化 → `load_mesh_into_library("loading_screen", hash 0x42ac8c61)` + `load_shader_into_library("loading_screen/loading_screen.raster.hlsl")` → spawn 0x88B bundle {DrawableMeshBundle(vt 0x1412da5d0), +0x48=0x12 渲染 pass, +0x58=0 DontCastShadow, 单位变换}。
- update_loading_screen 0x140a1a190：以 16B UUID（常量 @0x1429503e0）为键对两张 hashbrown 表（+0x70/+0xd8）做插入/删除（0x80 控制字节/0xff 墓碑，0xd0B 值条目，+0x58 字段写进度值 u32 from +0x5b0）。业务规则（何时增删）U。

## 4. panic / 日志串清单（原文 + 地址）

| 原文 | 地址 | 来源文件 |
|---|---|---|
| `assertion failed: active_game_delta_seconds >= 0.0` | 0x142a9f2c0 | save_backup_metronome.rs |
| `Capturing a game snapshot` | 0x142ace2b8 | save_game.rs |
| `Error while performing a synchronous save: ` | 0x142ace1f0 | save_game.rs |
| `Sync response from save thread` | 0x142ace188 | save_game.rs |
| `Failed writing session meta: ` | 0x142ace148 | save_game.rs |
| `Error appending edit to save file: ` | 0x142cca5b0 | save_thread.rs |
| `Edit appended to save file` | 0x142cca538 | save_thread.rs |
| `Save thread I/O error; affected file count: ` | 0x142cca908 | save_thread.rs |
| `Auto-save I/O error resolved` | 0x142cca950 | save_thread.rs |
| `assertion failed: current_save_file.locked_backups.remove(&path_buf)` | 0x142cca978 | save_thread.rs |
| `assertion failed: truncate <= self.save_file_structure.history_edit_offsets.len()` | 0x142cca4ab | save_thread.rs |
| `Was asked to load an unsaved file` | 0x142ccaa00 | save_thread.rs |
| `Error flushing save file on game thread disconnect.` | 0x142ccaa60 | save_thread.rs |
| `Rewriting history file` / `Saving the game to a temp file` / `creating snapshot folder` / `Snapshot GC thread finished in ` | 0x142ccaf31 / 0x142ccafd8 | save_thread.rs |
| `Error flushing history file for backup` / `Writing backup file from shadow copy` / `Backup limit reached; removing ` / `Error deleting stale backup ` / `Error reading backup file list: ` / `Copying inactive save file to backup` / `Error making an auto-save backup: ` / `Error capturing game snapshot: ` / `Error flushing changes to disk` / `Flush error while closing previous save file (upon open request)` / `No active save file, but AsyncSaveCmd::{CaptureSnapshot,Flush,ChangeSection,ReplaceEdits} has been issued` | 0x142cca730…0x142ccaf00 | save_thread.rs |
| `AsyncSaveCmd::{OpenFile(,CloseFile,ChangeSection,ReplaceEdits,BackupSave,ManualBackup,CaptureSnapshot,Flush}` | 0x142ccaaa0 等 | save_thread.rs（Debug 名） |
| `ShadowBackendFile is not open` / `Persist temp file` / `Error writing save data to temp file: ` / `Write data` / `Could not create temp file for save rewrite: ` / `Setting file length` / `Sync data` / `Save file successfully opened and written to disk` / `Asking the OS to sync writes to disk` / `Marking block range dirty: ..` / `flushed_block_count: ` | 0x142aace80…0x142aad1d8 | shadow_file.rs |
| `assertion failed: new_len <= buf.len() as u64` | 0x142aad048 | shadow_file.rs |
| `Loading a new glade. Spawn pre-placed trees` | 0x142c641a8 | load_game.rs |
| `Couldn't load save file from ` | 0x142c64250 | load_game.rs |
| `LoadGameState::LoadGameData` | 0x142c64120 | load_game.rs |
| `Ignoring PostGameLoadCommand::SwitchAppMode` | 0x142c642e0 | load_game.rs |
| `Command skipping must not be used together with snapshots` / `Can't skip ` | 0x142c64378 / 0x142c643d8 | load_game.rs |
| `Making save file backup for upgrade` / `Reading file data` / `Parsing JSON` / `Version` / `BackwardCompatibility` / `Camera` / `parsing camera` / `Time of day` / `This save file had no 'Time of day' data` | 0x142c64450 块 | load_game.rs |
| `Detected old version of arrow slits color IDs. Upgrading...` / `Detected old version of window color IDs. Upgrading...` | 0x142c64a90 / 0x142c64af0 | load_game.rs |
| `Cloning game state for save rewrite took ` | 0x142c64b48 | load_game.rs |
| `Failed to load history snapshot ` / `Version mismatch; expected , got ` / `decompressing terrain data` / `Ignoring unknown raster data in snapshot: ` | 0x142cdc688 / 0x142cdc840 / 0x142cdc800 / 0x142cdc7c0 | snapshot.rs |
| `parsing Version/Walls/Roofs/Decorators/WallColors/GateColors/Terrain/Trees/Raster` | 0x142cdc709 块 | snapshot.rs |
| `Error deleting glade: ` | 0x142aef5a0 | session.rs |
| `a Display implementation returned an error unexpectedly` | 0x142aef358 | session.rs（uuid 格式化 panic） |
| `Error reading save backup directory` / `Error creating backup save folder: ` | 0x142ae367c | save_backup_list.rs |
| `event:/UI/Logo/Scribble` / `event:/UI/Logo/Branches` | 0x142ae9472 / 0x142ae945b | logo_animation.rs |
| `loading.tmp` / `history.json` / `screenshot.jpg` / `meta.json` / `assets/glade` / `loading_background.jpg` / `state` / `.tmp` / `history-auto` / `snapshots` / `snapshot` | 见 §2.4、§3 | session.rs / save_thread.rs / shadow_file.rs |
| 依赖版本锚：`bevy_ecs-0.16.0`、`serde_json-1.0.85`、`flate2-1.0.24`、`puffin-0.14.1`、`ron`（git checkout 2787d8c）、rustc `05f9846f893b09a1be1fc8560e33fc3c815cfecb` | .rdata | — |

## 5. 与现有合同/代码的差异（逐条）

| # | 位置 | 现状（代码/合同） | 证据（机器码/串） |
|---|---|---|---|
| 1 | wp13 session.md：is_halloween_any “扫描字节 >1” | **方向相反**：实为“存在字节 ≤ 1”（`cmp [rdx+rcx],1; ja` 跳过 >1） | 0x140b46890 capstone |
| 2 | reconstruction session/mod.rs：`SavedSession::screenshot_path` 与 `loading_background_path` 均指向 0x140b46a50 | 0x140b46a50 实为 **loading_background_path**（拼接 `"loading_background.jpg"`，len 0x16）；screenshot_path 另址未证 | 0x140b46a50 反编译 + 串 0x142aef557 |
| 3 | reconstruction logo_animation.rs：`finish_wait` 置 +0x0c=1.0；`is_finished`（注释 “+0x0c >= 1.0”）地址/偏移混乱；`Plugin::build 0x140b19d40` | 实际：play=0x140b19d00；is_waiting=0x140b19d10（**+0x0c > 0.0**）；is_finished=0x140b19d22（**+0x10 >= 1.0**，常量 @0x142ae8b10）；finish_wait=0x140b19d33（+0x0c=1.0f）；build=0x140b19d40 ✓（init 结构：progress 全 0 + durations {const×4, 0.5}） | capstone 全程 |
| 4 | wp13 save-systems.md：json_key 0x140a43840 | 0x140a43840 实为 **AutoSaveState::close_file**（清 +0x120 + 清列表）；json_key 在 0x140a43810（wp13 已另列 ✓） | 0x140a43840 capstone（枚举分发 0x1408da280、0xb 分支） |
| 5 | wp13：BackupSaveFileKind::is_corrupted 0x140ae8320 ✓ 但 batch PDB 名 “is_auto” | 机器码 = tag==4 判定 → is_corrupted ✓（batch 名错位，登记） | capstone |
| 6 | wp13：`SessionId::snapshots_folder_path`（sysres PDB 名）绑定 0x140b46300 | 行为 = 构建 {history.json, screenshot.jpg, meta.json} 三条完整路径 → **session_file_list** 语义（wp13 合同本身也已把 0x140b46300 记给 session_file_list） | 0x140b46300 反编译 + 串块 |
| 7 | wp13：`init_game_state_from_snapshot` 0x1417a9c50 vs batch PDB 名 `try_load_snapshot_data` | 同址二名；行为 = 应用解析数据到世界（被 load_game 0x14146d030 调用） | 0x14146d030 行 1148 |
| 8 | wp13：`validate_and_load_closest_snapshot_data` 0x1417a6e50 vs batch PDB 名 `_::impl$0::validate_param` | 同址；行为 = 快照 JSON 解析器（parsing … 全串在此） | 0x1417a6e50 反编译 |
| 9 | reconstruction save_backup_metronome.rs：active_game_time 字段/阈值 U | 已解：{counter u32 @+0x140, 累计 f32 @+0x144（clamp [0,1]）, 阈值 f32 @+0x148}，档位表 {300,600,3600,7200}，仅阈值<60 时累计 | save_game 0x140a3f470 行 507-591 |
| 10 | reconstruction save_game.rs：save_game 骨架 `fn save_game(_history: &History)` | 实为 6 段 SystemParam 系统（含 Metronome/History/相机/时间/羊/颜色 等），节拍器内联其中；`add_history_edit_to_save_file` 同址 PDB 名另有 `ensure_current_snapshot` | 0x140a3f470 / 0x140a439b0 |
| 11 | reconstruction save_thread.rs：AsyncSaveCmd/命令面 U | 已解变体表 3..11（见 §3）+ Shutdown=0xb + 线程同步模型（condvar 1s） | 0x141720680 / 0x141721aa0 |
| 12 | reconstruction shadow_file.rs：“写目录下临时文件后 rename/sync” 泛述 | 精确：`<path>.tmp` 后缀、Option<File> @+0x40（MIN 哨兵）+句柄 @+0x48、数据 @+0x28/+0x30、脏块计数 @+0x68（8 槽 &7）、mark_dirty(start,end) 区间协议、ensure_current_snapshot 追加 `",\n"` + 序列化后 mark_dirty | §3 |
| 13 | wp13 save-systems.md：snapshot 载荷 “rkyv/ron 编码” 未知 | **JSON 明文 + flate2 deflate**（ron 库仅出现在 parse.rs 路径，非载荷编码主载体；serde_json Value 确认） | snapshot.rs 串 + flate2-1.0.24 路径 |
| 14 | reconstruction session/mod.rs：`SessionMeta` 季节字节 `Vec<u8>` @+0x08/+0x10 | 布局补充：+0x00=1、+0x18=Option None 哨兵、+0x58=byte 0；serde 字段名 {state, glade, mutators, image}（reconstruction 无） | 0x140b468f0 / 0x140b46480 |
| 15 | reconstruction load_game.rs：`load_game(_manager: &mut LoadGameManager)` | 13+ 段 SystemParam；主状态机 LoadGameState::LoadGameData；写 loading.tmp；颜色 ID 升级映射 {1→2,2→5,4→8,5→3,8→4}；max wall id 恢复写 +0xc0 | 0x14146d030 |
| 16 | reconstruction prefab/mod.rs：`Prefab { parts: Vec<PrefabPart> }` | 哈希表内 Prefab 值仅 **16B**（Vec 24B 不符）→ 实际为 {ptr,len} 胖指针类（Box<[PrefabPart]>？），reconstruction 的 `Prefab.parts: Vec` 布局存疑 | 0x141552d70 桶步长 0x28 |

## 6. 跨领域线索

- **camera-time 域（协调员同步，2026-10-14；reanalyst 2026-10-15 修正）**：会话相机存档 schema =
  **6 字段** `{yaw, pitch, position, camera_rig_arm_length, pitch_offset, yaw_offset}`（0x24B；
  **旧 7 字段表含 position_cache 修正**——position_cache 仅反序列化接受/内部缓存，save 序列化
  0x1416f8470 只写 6 字段）。本域对应存档节 **Camera（tag 3，json_key "Camera"，padding 128）**，
  载荷 = `CameraRigParametersCache`（to_json_string 实例 0x1416f8470，save_game.rs）。请对照相机
  字段顺序/序列化 tag 与本域 SaveFileSection::Camera 节序。
- **history 域**：save_game 调 `0x140a84760`（=set_current_snapshot，PLAYBOOK 已证）挂接新快照 Arc；save_game_to_file 序列化 HistoryIdentity/HistoryChainIndex/HistoryChainSnapshots；load_game 遍历 MultiEdit（变体 7/8/9）恢复 max wall id；ensure_current_snapshot 以 ShadowFile::mark_dirty 增量写历史 JSON。
- **meta_commander**：NextSession::append_on_load<MetaCommandFence>（crates/country-core/src/meta_commander.rs 路径串 0x142c5ae38）。
- **render 域**：loading_screen/logo 经 `load_mesh_into_library`（0x140b1aca0）/`load_shader_into_library`（0x140b3fe40）挂 AssetMeshLibrary/AssetShaderLibrary；`*.raster.hlsl` 命名（loading_screen、logo_animation、logo_branches_animation）；DrawableMeshBundle(vt 0x1412da5d0)+DesignatedRenderPass(0x12)+DontCastShadow；session_selection（0x1408f68c0/0x1408f6c00）缩略图渲染留待渲染域。
- **audio 域**：animate_logo 播 FMOD `event:/UI/Logo/Scribble|Branches`（AudioCore @+0x10 句柄路径）；prefab 音频触发经 audio_trigger_library（0x140a382d0）+ 0x1408b9f20。
- **uuid/paths**：SessionId::new=0x140916420→Uuid 生成（0x140c102e0）；paths::saves()=0x140b9db90、paths::deleted_saves()=0x140b9dbf0（0x140b9db30 族）。
- **bevy_ecs-0.16.0**：事件队列/component.rs/registry 路径串多处（Bevy 分叉仍标 0.16.0）。

## 7. 错位/纠错登记

| 地址 | PDB/batch 名 | 实际（机器码） |
|---|---|---|
| 0x140b19d30 | finish_wait | is_finished（+0x10 >= 1.0） |
| 0x140b19d33 | （未命名） | finish_wait（+0x0c = 1.0f） |
| 0x140ae8320 | BackupSaveFileKind::is_auto | is_corrupted（tag==4；与 GladeType::is_winter 同址折叠） |
| 0x140a43840 | SaveFileSection::json_key | AutoSaveState::close_file |
| 0x140b46a50 | SavedSession::screenshot_path | loading_background_path |
| 0x140b46300 | SessionId::snapshots_folder_path | session_file_list 行为 |
| 0x140b468c0 | try_load_from_path::closure$0 | is_xmas2024（字节==2 扫描） |
| 0x140b46920 | is_xmas2024::closure$0 | NextSession::load |
| 0x1417a6e50 | _::impl$0::validate_param | validate_and_load_closest_snapshot_data（快照 JSON 解析器） |
| 0x1417a9c50 | try_load_snapshot_data | init_game_state_from_snapshot（应用数据到世界） |
| 0x140a439b0 | ensure_current_snapshot | （同址另有 wp13 名 add_history_edit_to_save_file；行为 = 追加 edit + mark_dirty，两义兼容） |

## U — 遗留未知

- NextSessionKind 变体 1/2 的完整语义映射（1=解析存档、2=常规加载为 I）；**reanalyst 2026-10-15：
  on_load 条目已解（0x20B {tag,payload1..3}，见 §2.1 与 state/session RE-ANALYSIS §2.3）**。
- backup_folder_path 的叶名明文（描述符 @0x140aef328 未解）；"state" 串在 meta.json/快照中的角色。
- SessionMeta 字段 0/2/5（unit/双 qword/String）与 "state" 键的对应。
- execute_save_cmd 变体 0/1/2 归属（**reanalyst：维持"死/历史变体无 case"**）；快照 GC 的触发频率。
- update_loading_screen 的 UUID 键业务含义与增删时机。
- try_approx_parse_save_file_identity 的近似解析规则。
- **GameSaveData 逐字段 layout**（消费边界已闭合：16+ 项 SystemParam 数据源 + AutoSaveState 序列化缓冲；字段级需对 save_game 0x140a3f470 分段逐槽比对）。

## 9. 算法级深挖补充/纠错（2026-10-15，见 ALGORITHMS.md）

- **mark_dirty = 4KB 块位图**（0x14091a450）：start_block = start>>12、end_block =
  ceil(end/4096)；位图 {ptr@+0x60, 位计数@+0x68}；"8 槽脏块区间 &7"描述修正（&7
  为字内位偏移）。ShadowFile 数据 Vec cap-first {cap@+0x20, ptr@+0x28, len@+0x30}。
- **ensure_current_snapshot 0x140a439b0**：+0x38 是影子缓冲的**追加偏移**
  （round2 CLOSURE 的"History+0x38 快照 id 域"描述修正）；协议 = push offset 进
  edit_offsets → ", \n"(0x0a2c) → MultiEdit::serialize（0x80B 缓冲，FUN_140b39b80）
  → mark_dirty(旧端, 新端)。
- **备份节拍器除数桶语义**（save_game 0x140a3f470 行 ~507-591 全文）：counter % 
  {7200,3600,600,300} 从大到小匹配，命中后 counter -= 除数，内层升序重校验可一次
  消耗多个间隔（桶内容 0 → panic 除零 @0x142c6e9d8）。
- **load_game 版本开关**：`0x14362c308 = (v > 0x19)`、`0x14362c011 = (v > 0x1a)`
  （后者供 history 域 add_decorator_diff 楼梯 checksum 校验——跨域互证闭合）。
- **execute_save_cmd 公共断言**：case 3-10 头部共用 truncate <=
  history_edit_offsets.len()（@0x142cca478，51 字 + 源路径反向扫描取文件名）；
  Shutdown 分支 = flush_deferred_and_sync 0x14171f7f0 + 发回确认 + 线程退出（state==3
  时睡 10ms 继续）。
