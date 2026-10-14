# systems/save/save_thread.rs

> 文件作用：存档线程：save_thread 主循环（脏 shadow 文件逐 tick flush → mutex+condvar 等待≤1s → 错误列表 diff/日志 → recv 命令）；execute_save_cmd = AsyncSaveCmd 分发（3=OpenFile/4=CloseFile/5=ChangeSection/6=ReplaceEdits/7=BackupSave/8=ManualBackup/9=CaptureSnapshot/10=Flush/11=Shutdown）；snapshot_gc_thread 清理未引用快照；CurrentSaveFile 增量历史写（Edit appended to save file + mark_dirty 协议）。

函数总计 30：业务 9 / 内联无独立符号 7 / 派生胶水 14

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| ``country_core::systems::save::save_thread::execute_save_cmd`` | 0x141721aa0 | 0 | 待推断 | O：execute_save_cmd = switch{3,4,5,6,7,9,10}+default(含 8=ManualBackup)；OpenFile 体内 join "snapshots"+Rewriting history file+Saving the game to a temp file+creating snapshot folder | session-save\decomp\decomp-0x141721aa0.txt  |
| ``country_core::systems::save::save_thread::save_thread`` | 0x141720680 | 0 | IO/解析 | O：save_thread 主循环（condvar 1s；cmd==0xb → flush_deferred_and_sync+退出；Save thread I/O error; affected file count 日志） | session-save\decomp\decomp-0x141720680.txt  |
| ``country_core::systems::save::save_thread::CurrentSaveFile::change_section`` | 0x14171ece0 | 115 | 设置/写入 | O：CurrentSaveFile::change_section = 修改当前节+ShadowFile::mark_dirty(0x14091a450)×2（脏区间协议互证） | round3\decomp-0x14171ece0.txt  |
| ``country_core::systems::save::save_thread::CurrentSaveFile::flush_deferred_and_sync`` | 0x14171f7f0 | 174 | 待推断 | O：flush_deferred_and_sync（游戏线程断开时落盘；错误日志 Error flushing save file on game thread disconnect.） | session-save\decomp\decomp-0x14171f7f0.txt  |
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
