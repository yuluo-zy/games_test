# systems/save/shadow_file.rs

> 文件作用：影子文件原子写协议：ShadowFile{+0x28/+0x30 数据 (ptr,len), +0x40/+0x48 文件状态（MIN 哨兵+句柄）, +0x68 脏块计数(8 槽 &7)}；create 初始化；flush = 打开 <path>.tmp→Write data→Setting file length→Sync data→Save file successfully opened and written to disk；sync_all = Persist temp file（Asking the OS to sync writes to disk / flushed_block_count）；mark_dirty(start,end) 记录脏字节区间。

函数总计 16：业务 4 / 内联无独立符号 8 / 派生胶水 4

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| ``country_core::systems::save::shadow_file::ShadowFile::create`` | 0x1409184d0 | 0 | 构造/注册/生成 | O：create = 初始化影子文件（未开状态；ShadowBackendFile is not open 前置） | round3\decomp-0x1409184d0.txt  |
| ``country_core::systems::save::shadow_file::ShadowFile::flush`` | 0x1409193d0 | 116 | 待推断 | O：flush（v2 名；v1 名 unwrap_open_mut 同址）= 未开则开：<path>.tmp 创建→写数据→set_length→sync→成功日志；错误 Could not create temp file for save rewrite: / Error writing save data to temp file:  | session-save\decomp\decomp-0x1409193d0.txt  |
| ``country_core::systems::save::shadow_file::ShadowFile::sync_all`` | 0x14091a230 | 211 | 待推断 | O：sync_all（v1 名 flush_impl）= Persist temp file：写盘+同步+flushed_block_count 复位 | session-save\decomp\decomp-0x14091a230.txt  |
| ``country_core::systems::save::shadow_file::ShadowFile::mark_dirty`` | 0x14091a450 | 227 | 设置/写入 | O：mark_dirty = 记录脏字节区间（Marking block range dirty: ..；被 change_section/ensure_current_snapshot 调用） | session-save\decomp\decomp-0x14091a450.txt  |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::save::shadow_file::ShadowBackendFile::unwrap_open_mut`（行 24）— 待推断
- `country_core::systems::save::shadow_file::ShadowFile::write_at`（行 93）— 设置/写入
- `country_core::systems::save::shadow_file::ShadowFile::truncate`（行 103）— 删除/清理
- `country_core::systems::save::shadow_file::ShadowFile::data`（行 112）— 待推断
- `country_core::systems::save::shadow_file::ShadowFile::flush_impl`（行 121）— 待推断
- `country_core::systems::save::shadow_file::ShadowFile::path`（行 247）— 待推断
- `country_core::systems::save::shadow_file::ShadowFileWriter::write_all`（行 281）— 设置/写入
- `country_core::systems::save::shadow_file::ShadowFileWriter::stream_position`（行 289）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`drop` 等 4 项
