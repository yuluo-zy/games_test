# systems/save/save_backup_list.rs

> 文件作用：备份存档列表：SaveBackupList::new_from_session 扫描备份目录（文件名前缀 history-auto）建列表；BackupSaveFileKind::is_corrupted（tag==4）。

函数总计 10：业务 3 / 内联无独立符号 3 / 派生胶水 4

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| ``country_core::systems::save::save_backup_list::BackupSaveFileKind::is_corrupted`` | 0x140ae8320 | 37 | 谓词/校验 | O：is_corrupted = tag==4 判定（与 GladeType::is_winter 同址折叠；v1 批名 is_auto 错位） | session-save\decomp\decomp-0x140ae8320.txt  |
| ``country_core::systems::save::save_backup_list::SaveBackupList::iter_backup_files`` | 0x140ae8330 | 54 | 迭代/变换 | O：iter_backup_files = 迭代备份文件 | round3\decomp-0x140ae8330.txt  |
| ``country_core::systems::save::save_backup_list::SaveBackupList::new_from_session`` | 0x140ae8470 | 97 | 构造 | O：new_from_session = 扫备份目录建列表；错误日志 Error reading save backup directory / Error creating backup save folder: ；前缀 history-auto | round3\decomp-0x140ae8470.txt  |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::save::save_backup_list::BackupSaveFileKind::is_auto`（行 28）— 谓词/校验
- `country_core::systems::save::save_backup_list::SaveBackupList::into_files`（行 136）— 待推断
- `country_core::systems::save::save_backup_list::SaveBackupList::backup_metronome_counter`（行 140）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`fmt` 等 4 项

## 实现方式（算法级，2026-10-15 深挖）

- **BackupSaveFileKind::is_corrupted 0x140ae8320**：`tag == 4`（机器码；batch PDB 名
  is_auto 错位）；iter_backup_files 0x140ae8330 = 迭代备份文件。
- **new_from_session 0x140ae8470**：扫备份目录（`history-auto` 前缀）建列表；错误
  "Error reading save backup directory" / "Error creating backup save folder: "
  （@0x142ae367c）。
