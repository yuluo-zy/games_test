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

## 实现方式（算法级，2026-10-15 深挖；全文见 session-save/ALGORITHMS.md）

- **ShadowFile 布局**（本域机器码新证）：数据 Vec cap-first {cap@+0x20, ptr@+0x28,
  len@+0x30}、追加偏移@+0x38、文件状态@+0x40（i64::MIN 哨兵 = Some/已开）、句柄
  @+0x48、脏位图 ptr@+0x60、位计数@+0x68、flushed_block_count@+0x78。
- **flush 0x1409193d0**（unwrap_open_mut 同址）：未开 → 建 `<path>.tmp`（.tmp
  @0x142aaca18）→ 写数据（"Write data"）→ "Setting file length" → "Sync data" →
  log "Save file successfully opened and written to disk"；错误 "Could not create
  temp file for save rewrite: " / "Error writing save data to temp file: "。
- **sync_all 0x14091a230**（flush_impl）：先 flush → 断言 +0x40 < 0（否则 panic
  "ShadowBackendFile is not open" @0x142aace80）→ OS sync → "Asking the OS to sync
  writes to disk"（@0x142aad1d8）→ flushed_block_count@+0x78 → 返回 ok-bool。
- **mark_dirty 0x14091a450**（脏区间协议）：`end <= start → return`；**4KB 块**：
  start_block = start>>12、end_block = ceil(end/4096)；位图不足 → grow
  （FUN_140915c20 @+0x60）；越界 → 格式化 panic @0x142aabe08；置位 [start, end)；
  debug log "Marking block range dirty: .."（级别>4，@0x142aad220）。
  **DIGEST 的"8 槽 &7"修正为 4KB 块位图**（&7 是字内位偏移运算）。
- create 0x1409184d0 = 未开状态初始化（"ShadowBackendFile is not open" 前置语义）。
