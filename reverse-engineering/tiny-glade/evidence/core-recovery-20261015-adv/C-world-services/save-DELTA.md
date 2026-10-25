# save-DELTA — 用户批改核对（2026-10-16，对照 save-SPEC + 原机器码）

> 核对 agent（C 世界）。逐 SPEC 项对照当前源码 `D:/game/tiny-glade/crates/country-core/src/systems/save/*`。
> 判定：**真实实现**（真实业务代码+真实消费者）/ **有水分**（伪实现/桩/错误语义冒充）/ **仍缺**（未实现）/ **待取证**。
> 原侧证据：`evidence/core-recovery-20261015/save/0x141720680-normalized.txt`、`metronome-a/b.asm`、`claude-metronome.result.json`、`evidence/country-core-deep/session-save/decomp/decomp-0x14146d030.txt`、RNE 常量直读（CAP=8.0、THRESH=1.0、STEP=-1.0、gate=60.0）。

## 1. 主后台保存循环 save_thread 0x141720680

| SPEC 项 | 当前状态 | 证据 | 备注 |
|---|---|---|---|
| ≥1s tick 逐文件 flush 首个脏条目 → 失败入 0x40B 错误列表 | 真实实现 | save_thread.rs:453-468；asm 行 148-193 | 0x40B=每条目 0x40 字节（asm `pppppvuVar1*0x40`）。本实现条目为 `(PathBuf, AutoSaveError)` 语义等价 |
| condvar wait_timeout(1s, 1e9ns) | 真实实现（替代） | save_thread.rs:487 `recv_timeout(1s)` | SPEC 认可"recv_timeout 等价等待命令"。与原版每轮先 wait 后 recv 的时序等价 |
| 错误列表 diff（字段比较）→ 空=info / 非空=warn + 回传游戏线程 | 真实实现 | save_thread.rs:471-482；asm 行 214-307（`FUN_140b0d840`+0x20/+0x38/+0x30 memcmp） | 当前用 Vec `!=` 近似逐字段比较，语义一致 |
| state!=3 → drop 当前存档（0/1/2 三函数） | 部分（顺序差异） | save_thread.rs:490-492；asm 行 364-373 | 原版每轮循环 drop；当前仅在收到命令时消费一个 GC 释放 |
| 断言 `locked_backups.remove(&path)` 成功（panic @0x142cca978 line 0x44） | 真实实现 | save_thread.rs:319-326 `consume_snapshot_gc_release`；asm 行 379-383（`FUN_1428d9430(&UNK_142cca978,0x44,..)`） | panic 串位置/行号与 asm 一致；消费点（每命令 vs 每轮）有差异 |
| 变体 11 Shutdown：需 flush → flush_deferred_and_sync；失败 warn "Error flushing save file on game thread disconnect."；确认回传；无活动档睡 10ms(0x989680) | 真实实现 | save_thread.rs:511-531；asm 行 406-479 | 10ms 与 0x989680 一致。**差异**：无 flush 且有活动档时，原版走 0x141721104 未恢复 jumptable，当前直接 break 退出——此分支语义当前标为 O 但实为待取证 |
| OpenFile 前置 `contains_file(history.json)` → panic "Was asked to load an unsaved file" | 真实实现 | save_thread.rs:496-501；asm 行 491-521（`FUN_1417201a0` + join "history.json" 0xc 串） | panic 串语义一致 |
| execute_save_cmd 0x141721aa0 switch 3..11 | 真实实现（分发骨架） | save_thread.rs:338-403 | ChangeSection/ReplaceEdits/CaptureSnapshot/Flush 的 No-active 分支日志与 SPEC 一致；**见 §2 字段名与 default 区差异** |

## 2. execute_save_cmd / AsyncSaveCmd 变体

| SPEC 项 | 当前状态 | 证据 | 备注 |
|---|---|---|---|
| AsyncSaveCmd 变体 tag 3..11 | 真实实现 | save_game.rs:127-166 | tag↔命令映射与 SPEC §1.1 表一致 |
| AsyncSaveCmd **全字段名/精确字节布局**（SPEC：先实现可见载荷偏移，**不编造字段名**） | 有水分（字段名编造） | save_game.rs:129-165（session/sections/edits/section/value/truncate/path/kind/snapshot/flush） | 原版载荷偏移已分段、字段名 U。当前给全部变体命名了语义字段名，违反 SPEC"不编造字段名"约定。需逐字段核对 RNE 后修正或标待取证 |
| 变体 0/1/2 | 待取证（诚实省略） | save_game.rs:124-125 注释"0/1/2 未在 switch 出现（I：死/历史变体）" | 符合 SPEC §6（不把"不存在"当"已恢复"） |
| 变体 5 ChangeSection / 6 ReplaceEdits No-active panic 串 | 真实实现 | save_thread.rs:351,357（"No active save file, but AsyncSaveCmd::ChangeSection/ReplaceEdits has been issued"） | panic 语义一致 |
| 变体 8 ManualBackup（default 区）：文件身份字节比对 + "snapshot"(0x746f687370616e73) 常量 | 仍缺 | save_thread.rs:261-267 `perform_manual_backup` 仅转发 backup_save | 原版 default 区身份比对/串常量未实现。待取证 |
| 变体 7 BackupSave：history-auto 前缀 + 删除旧备份日志族 | 部分 | save_thread.rs:240-258 | "Error making an auto-save backup"/"Error flushing history file for backup" 有；"Backup limit reached; removing "/"Error deleting stale backup " 无（retention 删除未接，见 §4） |

## 3. save_game 主系统 0x140a3f470 / 节拍器 / GameSaveData

| SPEC 项 | 当前状态 | 证据 | 备注 |
|---|---|---|---|
| 备份节拍器并入 AutoSaveState（counter@+0x140/累计@+0x144/阈值@+0x148） | 真实实现 | save_backup_metronome.rs 全文 | 常量直读：**CAP=8.0@0x142934604、THRESH=1.0@0x142925904、STEP=-1.0@0x14295a0e4、gate=60.0@0x1429d18cc 全部核实一致**。阈值门、delta>=0 断言、counter 递增、{300,600,3600,7200} 桶、从大到小整除匹配均与 asm 一致 |
| 节拍器命中 → "Capturing a game snapshot" → capture_game_snapshot_save_data → History::set_current_snapshot → AsyncSaveCmd::CaptureSnapshot(9) | 部分（触发真、捕获链未接） | save_game.rs:195-198 | 只发本地 `CaptureSnapshotRequested` 事件，宿主需响应；真实 capture/Arc/set_current_snapshot/send 均未接。诚实标注 |
| remove_counter（residual 删除候选）精确语义 | 待取证 | save_backup_metronome.rs:83-105 | 原版 0x140a4104c 起 residual 算法证据自评"unresolved"；当前向后回退候选为自创读法，语义未与原 asm 对齐 |
| GameSaveData 240B 完整字段布局（~9 资源 +0x28..+0x100，字段名 U） | 有水分（字段名自定，未标待取证） | save_game.rs:232-235 `{sections, edits}` | 注释诚实说明"域资源未齐、组装留待接入"，但**未显式标注 sections/edits 为自定名**，与 SPEC"字段名 U"不符 |
| save_game 节拍器系统在游戏调度注册 | 仍缺 | save/mod.rs:52-54 `add_save_load_systems_impl` 无任何生产调用方 | 节拍器仅测试/纯函数层可用，未注册进生产 Schedule |

## 4. 快照/备份/GC

| SPEC 项 | 当前状态 | 证据 | 备注 |
|---|---|---|---|
| capture_game_snapshot_save_data 0x1417a0430（Arc + 节序） | 部分（语义入口真、域采集器未接） | snapshot.rs:107-119 | 返回 Arc<SnapshotSaveData>，但域节载荷由调用方提供，真实采集器（wall/roof/decorator）未接。诚实标注 |
| validate_and_load_closest_snapshot_data 0x1417a6e50（节序 Version→Walls→…→Raster、Version 不符 panic、Terrain flate2、Raster 白名单） | 部分 | snapshot.rs:48-58,151-188 | 节序 ✓；Version 不符返回 Err（非 panic）；flate2 deflate/inflate ✓（Cargo `flate2=1.0.24`）；`is_known_raster_key` 恒 false（白名单待取证） |
| snapshot_gc / retention 删除队列 | 部分 | save_thread.rs:290-315 `start_snapshot_gc` | GC 线程存在但生产无构造点（无调用方）；"Backup limit reached" 删除路径未接 |
| save_thread 生产 spawner | 仍缺 | 全库仅 tests.rs:268 调 `save_thread(...)` | 独立线程真实实现但未接入游戏（无生产启动方） |

## 5. load_game 0x14146d030

| SPEC 项 | 当前状态 | 证据 | 备注 |
|---|---|---|---|
| 状态字节 +9 门控（0→进度记录+置2；2→主流程；其他 panic @0x142c641d0） | 部分 | load_game.rs:328-332 | Loading→Loaded 两态推进真实；panic 分支未实现；app_mode==0xe 门未接线 |
| NextSessionKind 解码 `decoded=kind^0x8000000000000000; min(·,2)` | 真实实现 | load_game.rs:334 | 与 SPEC/ALGORITHMS 一致 |
| parse_saved_game_data（节序 + upgrade 备份 + "Couldn't load save file from"） | 真实实现（纯函数） | load_game.rs:93-125 | 节序 Version/BackwardCompatibility/Camera/Time of day ✓；upgrade 备份命名 `history-recovered-`（I，原版精确策略待取证） |
| MetaCommand/PostGameLoadCommand 两遍回放 + fence 0x8000000000000003 终止 | 真实实现 | load_game.rs:233-314；decomp-0x14146d030.txt 行 383/743（`==0x8000000000000003 break`）、行 387/744（`^0x8000000000000000`） | fence 常量与变体解码核实一致 |
| 版本 → 全局开关 0x14362c308=(v>0x19)/0x14362c011=(v>0x1a) | 真实实现 | load_game.rs:154-156 + save_file.rs:239-256；decomp 行 860-861 | `0x19<v` / `0x1a<v` 与 decomp 一致 |
| 旧版颜色 ID 升级（位 48 window 999 / 位 40 arrow slit 映射） | 真实实现 | save_file.rs:203-233 + load_game.rs:158-170 | 映射表/999 常量一致（save_file.rs 已有） |
| extract_max_wall_id_from_history | 部分 | load_game.rs:174-198 | 遍历 WallHistoryEdit 取最大 id 真实；原版写计数器资源+0xc0，当前仅返回 Option，调用方写——诚实 |
| load_game 本体 ECS 系统注册 / init_game_state_from_snapshot 应用 / MetaCommand 应用 | 仍缺 | save/mod.rs:24-31 桩；snapshot.rs:208-210 空 `apply_loaded_snapshot` | 门控、loading.tmp 写、事件（LoadingGameComplete/StartupReveal）全部未接生产 |

## 6. save_file_identity / load_completion_checker

| SPEC 项 | 当前状态 | 证据 | 备注 |
|---|---|---|---|
| try_approx_parse_save_file_identity 0x141e58f20（0x800 缓冲 + "Identity" 字面量） | 真实实现 | save_file_identity.rs:33-56 + 3 测试 | 缓冲/字面量一致 |
| derive_fallback_save_file_identity 0x141673140（字段序 U） | 部分 | save_file_identity.rs:62-85 | 时间戳折叠为自创策略，注释标"原版字段序 U 未决"——诚实 |
| check_load_completion / is_session_ok_to_load / SessionLoadReadiness | 真实实现 | load_completion_checker.rs:42-78 + 测试 | loading.tmp 探测/unsaved 列表语义与 0x140a9f810/0x140a9fa80 一致；完整 SystemParam 面未接（注释注明） |

## 合成 agent 应关闭的真实剩余缺口

1. **save_thread / save_game / snapshot / GC 无生产接线**：`save_thread` 仅测试调用；`save_game_system` 节拍器未注册；GC 无构造点。需在 register_core_startup 挂载真实消费者（会话切换/读档时构造 GC、App 装配 save_thread spawn + command channel）。
2. **AsyncSaveCmd / GameSaveData 字段名**：按 SPEC"不编造字段名"，需对全变体载荷偏移做 RNE 取证后改名或标待取证；GameSaveData 需显式标注字段名 U。
3. **capture/load 域采集器**：capture_game_snapshot_save_data 的真实域节采集（wall/roof/decorator 三采集器 FUN_141305170/141308b40/141309840）与 `History::set_current_snapshot`→AsyncSaveCmd::CaptureSnapshot(9) 发送链未接。
4. **retention 删除队列**（"Backup limit reached; removing"）与 BackupSave 的 `history-auto` 前缀生产未接。
5. **save_thread 无 flush 且有活动档的 Shutdown 分支**：原版 0x141721104 未恢复 jumptable，当前直接 break——补取证或诚实标待取证。
6. **load_game 系统本体**：app_mode==0xe 门、loading.tmp、init_game_state_from_snapshot 应用、MetaCommand/PostGameLoadCommand 的实际应用（commander_update 已写入 NextSession，但无系统消费回放结果）。
