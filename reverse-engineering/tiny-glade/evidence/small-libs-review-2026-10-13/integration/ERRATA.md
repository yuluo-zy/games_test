# 本轮勘误：tg-text 的字符串反序列化接口

确认项 T-DOC-1（低严重度，证据准确性）。当前实现 String::deserialize 正确；上一批源码注释和若干报告把原 API 写成 deserialize_str，这个推断来自共用地址的反编译器标签。

## 权威原记录

skeleton/B/crates/tg-text/src/shape_stream.rs 的 605–623 行记录同一 original module 1186 的内联链：

- record 2768：serde 1.0.200 de/impls.rs:700，String 的 Deserialize。
- record 2792：原 RON fork de/tag.rs:139，deserialize_string。
- record 2816：原 RON fork de/mod.rs:455，deserialize_string。

这条 source-level 内联链的归属优先于共享机器码地址 0x141e43360 的任意单一别名。源码定义位置和原 LLVM module 1186 提取也由独立 reviewer 复核。

## 修正

当前 shape_stream.rs 的说明已改为“原内联链委托 String::deserialize”，crate README 已链接本文件。库函数代码与行为没有改变。历史 batch-01 的 BUSINESS_RULES.md、DIFFERENCES_AND_REPAIRS.md 和 interface-map.json 保留原样，其 Deserialize_str 表述由此勘误取代；原 hash 与原失败报告不被覆盖。

上一批真实 RNE word/metric 数据和候选测试保持其已有范围；本次不因注释修正重跑未变的原机器码案例。原 serde 入口运行及完整罕见失败行为仍待验收。

[修正的前后输入哈希](documentation-fix.json)。
