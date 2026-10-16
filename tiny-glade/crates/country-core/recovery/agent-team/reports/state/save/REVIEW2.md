# REVIEW2 — systems/save（组 state 二轮复验）

> 审查员：reviewer-state，2026-10-15。restorer 已交付 RESTORE.md 并新增实现；本报告复核
> RESTORE 声明与机器码/RE-ANALYSIS 的一致性。编译基线实测：`cargo check -p country-core --offline` 通过；
> `--lib systems::save resources::session audio::music meta_commander` **16/16 通过**（含
> execute_save_cmd 落盘、No-active panic、save_thread Shutdown ack、on_load 两遍回放）。

## 逐项复核

### 1. save_thread 主循环 0x141720680
| RESTORE 声明 | verdict | 说明 |
|---|---|---|
| 每 ≥1s tick flush 第一个脏影子文件 | OK | `last_tick.elapsed()>=1s` → `dirty_blocks().next().is_some()` → sync_all，失败入错误列表。与原 §2.1 一致。 |
| 错误列表 diff/日志/回传 | OK | `error_list != previous_errors` → "Auto-save I/O error resolved"/"Save thread I/O error; affected file count: " → error_sender 回传 → 交换。 |
| Shutdown flush+确认+退出 / 10ms 等待 | OK | need_flush → flush_deferred_and_sync（失败 warn "Error flushing save file on game thread disconnect."）→ ack → break；无活动档睡 10ms。 |
| OpenFile unsaved 前置 panic | OK | `unsaved.contains_file(history.json)` → panic "Was asked to load an unsaved file"。 |
| 每轮"drop 当前存档"（ALGORITHMS §2.1 step4） | **PARTIAL/待取证** | src 未建模原版每迭代 `state!=3` 时的存档 drop（ALGORITHMS 描述）。保留文件存活更合理，但与原记录行为不一致；疑 ALGORITHMS 对该步语义描述不精确。建议 re-analyst 对 0x141720680 step4 段定点澄清。 |

### 2. execute_save_cmd 0x141721aa0
| 项 | verdict | 说明 |
|---|---|---|
| ChangeSection/ReplaceEdits No-active panic | OK | 串与 panic 语义命中（"No active save file, but AsyncSaveCmd::… has been issued"）。测试覆盖。 |
| CaptureSnapshot/Flush No-active 日志 | OK | 仅日志，符合 §2.2（case 9/10 为日志非 panic）。 |
| OpenFile/CloseFile/BackupSave/ManualBackup | PARTIAL(低) | OpenFile = execute_open_file_cmd（旧态 flush warn + create + "Rewriting history file" 日志）✓；BackupSave/ManualBackup 无活动档时 src 用 warn（原 case7/8 的 No-active 行为 §2.2 未明确列出，微差）。 |
| 公共 truncate 断言 | OK | replace_edits 内置，串一致。 |

### 3. save_file_identity（0x141e58f20 / 0x141673140）
| 项 | verdict | 说明 |
|---|---|---|
| try_approx_parse（0x800B + "Identity" 字面量 len 10 + 解析数字） | OK | 与章节一致；`SaveFileIdentity(u64)` = to_json_string<HistoryIdentity> 十进制载荷。 |
| derive_fallback（SystemTime + 二次尝试 + 元数据派生） | OK（字段序 U） | 折叠 mtime 进身份值，诚实标注 U；稳定且不伪造。 |

### 4. snapshot.rs（0x1417a0430 / 0x1417a6e50 / 0x1417a9c50）
| 项 | verdict | 说明 |
|---|---|---|
| 节序 Version→…→Raster | OK | SNAPSHOT_SECTION_ORDER 与跳表一致。 |
| Terrain flate2 deflate（1.0.24 原锁） | OK | deflate/inflate 双向；"decompressing terrain data" 串一致。 |
| Version 校验 / Raster 键白名单 | PARTIAL | Version 校验 ✓；`is_known_raster_key` 恒 false 且未在 validate 中应用（"Ignoring unknown raster data" 行为未实现），诚实标注待取证。 |
| init_game_state_from_snapshot 应用 | PARTIAL | `apply_loaded_snapshot` 显式 no-op，不伪造"已应用"；域应用留待（诚实）。 |
| capture_game_snapshot_save_data Arc | OK | 返回 Arc<SnapshotSaveData>；不代造 Walls/Roofs。 |

### 5. UnsavedFileList / AutosaveErrorState / flush_deferred_and_sync / backup
| 项 | verdict | 说明 |
|---|---|---|
| UnsavedFileList::add/contains_file | OK | 0x14171ff80 / 0x1417201a0 语义（Mutex<Vec<PathBuf>>）。 |
| AutosaveErrorState::is_ok（列表空） | OK | 0x140b92210 `[+0x10]==0` 折叠。 |
| flush_deferred_and_sync（[Option<DeferredSection>;10]→change_section→sync） | OK | 0x14171f7f0 语义；PDB DeferredSection 合同。 |
| backup_save/perform_manual_backup | PARTIAL(低) | flush 影子 + 复制到目标，串族命中；"Copying inactive save file to backup"/"Writing backup from shadow copy" 精确文案未逐字比（语义一致）。 |
| capture_snapshot（`<id>.snapshot` 落盘） | OK | snapshots 目录 + `<snapshot_id>.snapshot`；与 GC stem 解析互证。 |

### 6. SaveBackupList 计数器口径（round1 FIX-2 收口）
- **分类映射已机器级闭合**：fresh 反编译 filter-map 闭包 0x14166df80 证实：
  - 扩展名过滤 = `.json`(0x6e6f736a) / `.bak`(0x6162+'k') —— src 的 `matches!("json"|"bak")` ✓；
  - 非文件/metadata 失败 → None ✓；
  - 前缀 → kind：`history-auto-`=0、`history-manual-`=1、`history-recovered-`=2、
    `corrupted-`=4（`cVar7=(uVar4==0)+'\x03'` 判 4）、其余=3 —— src `from_path` **全对**。
- **计数器口径仍待取证**：new_from_session 0x140ae8470 对每个入选条目解析数字后缀
  `uStack_74=max(prev, parsed+1)`，未显式限定 history-auto；src 只对 `history-auto-` 取。
  数字后缀的起始偏移来源（closure 输出不含前缀长）未钉死。保持**待取证**（低）。

### 7. load_game on_load 两遍回放（fence）
| 项 | verdict | 说明 |
|---|---|---|
| OnLoadEntry 0x20B {tag,payload} / ON_LOAD_VARIANT_MASK / ON_LOAD_FENCE | OK | 与 RE-ANALYSIS §2 一致；fence 检查 tag||payload。 |
| 两遍回放 + fence 终止（测试 2 MetaCommand + 2 Apply） | OK（自洽） | 测试通过；fence 后不再回放。 |
| **extend_on_load tag** | **FIX/待取证** | 本审查反汇编 append_on_load<[MetaCommand;3]> 0x1403b5150：`mov [rbp-0x38],3` —— 批量条目 tag = **3**（RE-ANALYSIS §2.2 同）。src 用 `ON_LOAD_VARIANT_MASK|1`。原 tag=3 在回放 `tag^mask; min(·,2)` 下会落到变体 2（Arc drop），与"push MetaCommand"语义冲突——RE-ANALYSIS §5.6 残留。src 自洽但**与机器 tag 不符**；建议 restorer 在 load_game 回放字节级复核后再定（fence 双 tag 关系同为此项）。 |
| 变体 1 回放产出 MetaCommand | PARTIAL | src 产出 `MetaCommand{tag: entry.tag, payloads:[entry.payload,0×5]}`（混合形态）；原版为 Box<[MetaCommand;3]> 的 6 qword 载荷。commander_update 未建前为诚实占位。 |
| 变体 2（Arc unblock/drop）/ 二遍变体 2+ | PARTIAL | src `_ => {}` 跳过；原版有 unblock+drop / push{tag 0}。未建模，诚实标注。 |
| kind 解码 / 状态机 | OK | `decoded().min(2)` 一致；Loading→Loaded；panic-on-其他状态因 Rust 枚举不可表示而省略（可接受）。 |

### 8. meta_commander（只建数据结构）
- `MetaCommand` 0x38B=7 qword、`EMPTY_SENTINEL=0x8000000000000014`、`SESSION_LOAD_MARKER=0x8000000000000000`、
  peek/consume/undo_consume/extend_script、`MetaCommandFence=Arc<Mutex<(bool,bool)>>`、new_pair_blocked 0x14091ef90 ——
  与 RE-ANALYSIS §1 一致。commander_update 分派表**未建**（依赖游戏资源，诚实披露）。**未伪造加载成功**。
  unblock 偏移（+2 vs +0x10）与 fence 双 tag 残留已登记。verdict：**OK（数据结构范围）**。

## 跨组依赖卡点（登记，不阻塞本次）
1. save_game 0x140a3f470 GameSaveData 组装 → CameraRigParametersCache/HistoryIdentity/GladeColorCustomization 等域资源（RE-ANALYSIS §1：16+ SystemParam；src 未建组装系统，未伪造载荷）。
2. capture_game_snapshot 0x1417a1d00（184KB）全 Query 串化 + init_game_state_from_snapshot 0x1417a9c50 应用 → 域 Query。
3. MetaCommand 应用 → commander_update 0x14091f180。
4. load_completion_checker 完整系统 → GameProfile/MetaCommander。

## 结论与 DISPATCH 建议
- round1 FIX-1（阈值门）已吸收（实测：`!(threshold<60.0)` 阻断，与 `(a<=b && b!=a)` 机器语义一致，含 NaN 阻断）；
  FIX-2 收口：分类全对，计数器口径待取证。
- 新增小项：**extend_on_load tag=3 对机器码**（FIX/待取证，低）；save_thread step4 drop 待取证（低）。
- 完整 save 验收仍被域组装/全局接线阻塞（跨组依赖）。
- **建议：save → fix_pending（低）**：唯一可行动项 = 按机器码把 extend_on_load 批量条目 tag 定为 3 并登记 fence 双 tag 关系；其余为已披露 GAP/待取证。
