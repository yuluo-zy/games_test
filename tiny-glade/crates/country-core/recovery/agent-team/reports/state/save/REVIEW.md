# REVIEW — systems/save（组 state）

> 审查员：reviewer-state，2026-10-15。独立核对，以原二进制证据为准。
> 范围：`src/systems/save/{mod,save_game,save_thread,shadow_file,save_backup_list,save_backup_metronome,snapshot_gc}.rs`
>（restorer 尚未新增 load_game/save_file_identity/snapshot/load_completion_checker；无 RESTORE.md，DISPATCH 仍 queued）。
> 证据：session-save/DIGEST.md、ALGORITHMS.md、decomp/*、round3/decomp-0x14171ece0.txt、
> 本审查 fresh 反编译（ghidra shared）+ RNE 机器码/串直读（tiny-glade.rne）。

## 逐函数判定

### save_game.rs
| 函数/类型 | verdict | 说明 |
|---|---|---|
| `write_json_sections` | **OK** | 逐字节核对：每节 `"KEY":VALUE,<pad>\n`。格式化件 `"`(0x142cd3da8)+`":`(0x142cd3da1) 拼键、载荷 to_json_string、后缀 `,`(0x142cd3dd0)+空格 repeat(padding)(0x142cd3dc8)+`\n`(0x142cd3de0)，与 src 完全一致。 |
| `save_game_to_memory` | **OK** | 结构逐字节核对 save_game_to_file\<Cursor\> 0x141750c90：`{`(0x7b) → 7 节 → `"History":{\n`(12B，0x79726f7473694822+0xa7b3a22) → 3 节 → `"edit_chain":[\n`(0x142cd3d58 len 0xf) → edit 间 `,\n`(0xa2c) → `\n]}}`(0x142cd3d67 len 4)；edit 偏移在分隔符前 push。 |
| `SerializedJson`/`GameSaveSections`/`SaveFileStructure` | OK | 编解码层；未伪造节载荷（节载荷由各域模块串化，诚实分层）。 |
| `SaveFileSection`（resources/save_file.rs 投影） | **OK** | json_key 双表实测：len [7,8,21,6,11,5,6,11,13,9]、ptr、padding [0,16,0,128,32,128,64,8,8,2048]；**第 9 节键 = "snapshots"(9B)**。src 正确；ALGORITHMS/DIGEST 的 "snapshot" 过期。 |
| 缺失：`save_game` 系统 0x140a3f470、`AutoSaveState`、`to_json_string` 各载荷 | GAP | 合同 remaining 已披露；未实现不冒充。节拍器内联逻辑已抽出为独立类型（见 metronome）。 |

### save_thread.rs（CurrentSaveFile）
| 函数 | verdict | 说明 |
|---|---|---|
| `create` | OK | save_game_to_memory + ShadowFile::create + 空锁集合。 |
| `change_section` | **OK** | fresh 核对 0x14171ece0：required=值长+2、new_length=max(2·old,required)、扩容迁移全部 section_ranges 与 edit_offsets（+growth 至 >old.start）、替换 `VALUE,<pad>\n`、write_at+mark_dirty。panic 串与截断语义吻合。 |
| `replace_edits` | **PARTIAL** | 断言串 `truncate <= …history_edit_offsets.len()` 一致；truncate=len 时回退位 data.len()-4（隐含 `\n]}}` 尾占 4B，与 save_game_to_memory 一致）；但原版 case 6 的逐 edit 追加细节（`Edit appended to save file` 等日志、`Error appending edit`）未在 src；截断/尾部/尾部迁移为重建推断，无独立 decomp 复核。 |
| `sync_all` | OK | flush→断言打开（"ShadowBackendFile is not open"）→OS sync（"Asking the OS to sync writes to disk"）；+0x78 按 Option<Error> 语义（src 修正 DIGEST 的 flushed_block_count 说法）。 |
| `start_snapshot_gc`/`consume_snapshot_gc_release`/`is_backup_locked` | OK | 锁备份集合 + 释放通道；consume 断言 `locked_backups.remove` 成功，对应 save_thread 主循环 0x141720680 每轮断言（panic @0x142cca978）。 |
| 缺失：`save_thread` 主循环、`execute_save_cmd`(3-11)、`flush_deferred_and_sync`、`backup_save`、`perform_manual_backup`、`UnsavedFileList`、`AutosaveErrorState::is_ok`、`execute_open_file_cmd`、`full_write_history_file`、`capture_snapshot`、`nuke_unreferenced_shapshots` | GAP | 合同 remaining 已披露。 |

### shadow_file.rs
| 函数 | verdict | 说明 |
|---|---|---|
| `create`/`rewrite` | **OK** | fresh 核对 flush 0x1409193d0：未开→`FUN_140ac5e70(.tmp, 4, 1, 0, 6)`=prefix ".tmp"+rand 6 同目录临时件→写全量→sync→persist；src 用 tempfile::Builder::prefix(".tmp").rand_bytes(6).tempfile_in().persist。src 注释纠正了"<path>.tmp 字面后缀"说法，正确。 |
| `flush_impl` | **OK**（微差） | 原版逐脏块 seek/write（位图迭代，块 0x1000）；src 合并相邻脏块一次 seek/write。语义等价（同一文件），属轻微优化偏离，非业务错误。 |
| `sync_all` | **OK** | flush→断言→OS sync；panic/context 串逐一命中。 |
| `mark_dirty`（resources/save_file.rs） | **OK** | 4KB 块位图，`end<=start return`、ceil(end/4096)；与 0x14091a450 机器码一致（round3 已修正 DIGEST "8 槽"）。 |
| `write_at`/`truncate`/`data`/`path`/`last_error`/`is_open`/`dirty_blocks`/`ShadowFileWriter` | OK | 追加偏移/零填充/截断语义与 ensure_current_snapshot 0x140a439b0 的 +0x38 追加偏移协议一致。 |

### save_backup_list.rs
| 函数 | verdict | 说明 |
|---|---|---|
| `BackupSaveFileKind::is_corrupted`/`is_auto` | OK | tag==4/tag==0（DIGEST 0x140ae8320 机器码）。 |
| `BackupSaveFileKind::from_path`（Manual/Recovered/Other 分类） | **PARTIAL/待取证** | 分类前缀（history-manual-/history-recovered-/corrupted-）声称来自 filter-map 闭包 0x14166df80，本审查未独立反编译该闭包；DIGEST 只确认 Auto/Corrupted。 |
| `iter_backup_files`（.json/.bak 过滤、跳过非文件） | PARTIAL/待取证 | 扩展名过滤与元数据跳过为 src 断言；原版 0x140ae8330 只确认"迭代备份文件"。 |
| `new_from_directory`/`new_from_session` | **PARTIAL** | 计数器累加：src 只对 `history-auto-` 前缀取 max(n+1)；**new_from_session 0x140ae8470 机器码对每个入选条目都解析数字后缀（closure 输出含前缀偏移）并 `uStack_74=max(prev, parsed+1)`，未限定 history-auto**。若手动/恢复备份亦带数字尾，原版会计入，src 不会。需反编译 0x14166df80 裁决。 |
| `backup_metronome_counter`/`into_files`/`files` | OK | |

### save_backup_metronome.rs
| 函数 | verdict | 说明 |
|---|---|---|
| `add_active_game_time` | **PARTIAL** | 断言 `delta>=0` 一致；但**缺阈值门**：原版（save_game 0x140a3f470 行 507-508）`阈值(+0x148) < 60.0 || NaN` 才累计。src 无条件累计，且结构无阈值字段。src 注释主张 +0x148 属 AutoSaveState；机器码显示该门就在累计块内。因 save_game 系统未实现，此门当前不可达，属潜伏缺口。建议在类型上建模该门（或让调用方按阈值门控），并保留注释。 |
| `consider_backing_up` | **OK** | 逐字节核对（纠正 ALGORITHMS 常量表错误）：clamp 上界 **8.0**(0x41000000，非 1.0)、步进 **-1.0**(0xbf800000，非 1.0)；`while 累计>=1.0 { 累计-=1; counter++ }` 扫除数桶 {300,600,3600,7200} 从大到小（rposition）；remove_counter 回退 `candidate-=interval 直至前值不再被更粗档整除`。测试用例 598→600/remove None 与 1500→remove 900 均复算吻合。 |
| `from_backup_counter`/`new_from_backup_list`/`counter`/`active_game_time` | OK | |

### snapshot_gc.rs
| 函数 | verdict | 说明 |
|---|---|---|
| `parse_save_file_snapshots` | **OK** | 0x141286730/闭包 0x1413e4830 语义：JSON 路径 visitor 首键命中即停，永不读 edit_chain；`let _=…` 丢周边解析错误。src 实现与测试（`{...,"edit_chain":BROKEN` 仍成功）一致。 |
| `snapshot_gc_thread` | **OK** | 0x14172b280：扩展名=="snapshot"(u64 常量 0x746f687370616e73，len 8) 才处理；解析十进制 stem（含 '+/-'）；未引用→"Deleting unreferenced snapshot"；非法数字名也删除（0x14172bb94）；每备份经通道发送释放（drop 接收端即取消，竞态守卫串一致）。 |

## FIX 清单（交 restorer）
1. **FIX（中）`SaveBackupMetronome::add_active_game_time` 缺阈值门**：原版仅在 `阈值(+0x148) < 60.0` 时累计（含 NaN）。要么在类型上加阈值字段并实现门，要么由调用方（将来的 save_game 系统）按 AutoSaveState 阈值门控；当前无门会令活跃时间在任何情况下都累计。
2. **FIX（低）`SaveBackupList::new_from_directory` 计数器口径**：原版对每个入选备份的数字后缀取 `max(prev, n+1)`，不限于 history-auto-。请反编译 0x14166df80 确认前缀语义后修正（或标注待取证）。
3. **PARTIAL 复核（低）`replace_edits`**：补充原版 case 6 逐 edit 追加的日志串（`Edit appended to save file` / `Error appending edit to save file: `）与尾部细节；目前实现可调用但与原版行为边界未独立闭合。
4. **微差（不必须）shadow flush** 逐块 vs 合并写；语义等价，可保留，注明偏离。

## 核对要点（src 优于既有证据）
- `SaveFileSection` 第 9 键 = "snapshots"；ALGORITHMS/DIGEST 的 "snapshot" 过期。
- metronome clamp=8.0 / 步进=-1.0；ALGORITHMS 常量表写 1.0/1.0 过期。
- ShadowFile +0x78 = Option<anyhow::Error>；DIGEST 的 flushed_block_count 过期。
- write_json_sections/save_game_to_file/change_section 的字节级格式（含空格 padding 位置）本审查已逐一实证，与 src 一致。

## 过程问题
- contracts.json 的 `evidence/core-recovery-20261015/save/REPORT.md`（save 及 shadow/save_thread/snapshot_gc/metronome 条目引用）**在磁盘不存在**；完成度声明引用缺失证据。建议重锚。
- 模块无 RESTORE.md，DISPATCH 仍 queued；本 REVIEW 针对现状。

## 结论
实现层面：代码c层（write_json_sections/save_game_to_memory/change_section/ShadowFile/GC/节拍器算法）为**真实可调用实现且多数字节级核对通过**，非空壳。验收层面：完整异步捕获（save_game 系统、save_thread 主循环、execute_save_cmd、快照采集）仍未实现，属合同已披露 GAP。整体状态：`in_review → fix_pending`（FIX-1、FIX-2 提交 restorer；FIX-3 待补充证据）。
