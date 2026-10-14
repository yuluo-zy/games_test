# systems/save/snapshot_gc.rs

> 文件作用：快照 GC 辅助：parse_save_file_snapshots 从存档文件解析快照引用集合，供 snapshot_gc_thread 判定未引用快照。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| ``country_core::systems::save::snapshot_gc::parse_save_file_snapshots<std::io::buffered::bufreader::BufReader<std::fs::File> >`` | 0x141286730 | 19 | 快照 | O：parse_save_file_snapshots<BufReader<File>> = 解析存档内快照引用（GC 线程消费；错误 Error parsing history snapshots fromfile） | session-save\decomp\decomp-0x141286730.txt  |
