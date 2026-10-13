# crates/motomoto/src

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

B chunk tag/Writer/ChunkWriter/Reader/Iterator业务链可调用；库IO/LE编码复用byteorder与std。

## 接线与验证

接线：已加入shared-foundation workspace；已定位原capture_game_snapshot/读取consumer，未接完整游戏存档。

既有6候选tests/173原Reader案例；独立review新增20限定原Cursor Writer/Drop+pending error与3原Reader错误后恢复组均匹配，候选四故障/unwind/零复制验证。无确认代码缺陷或性能回退，完整save consumer/泛型IO仍未验收。

batch-02（2026-10-13）：Ghidra 反编译 add_chunk×2/Drop/iter/next 与候选逐条一致；新增字节级 header 测试（12 测试过、clippy 过）；存档消费者映射深化（capture_game_snapshot add_chunk<&str> + validate_and_load Reader::iter→next + DeflateDecoder）。证据：[batch-02 BUSINESS_RULES](../../evidence/small-libs-batch-02-2026-10-13/motomoto/BUSINESS_RULES.md)、[delivery](../../evidence/small-libs-batch-02-2026-10-13/motomoto/delivery.json)、[native-oracle](../../evidence/small-libs-batch-02-2026-10-13/motomoto/native-oracle.json)。

完成轮（2026-10-14）：补 AsTag for &str（0x142080ca0）与 tag_from_str_trunc（0x142080d00）反编译一致；登记原仅 Cursor 单态化范围（泛型 IO 合同归候选边界）、ChunkWriter 三字段 ABI 观察、Reader::new 地址勘误。
完成度判定（2026-10-14）：库内业务逻辑全部实现并反编译验证（tag 编码/add_chunk/Drop/Reader 迭代边界）；未闭合仅消费者接线与 A/C（按用户保留）。
## 已恢复的函数和类型范围

- tag_from_str/tag_from_str_trunc/tag_from_le_bytes_truncated
- Writer::new/add_chunk、ChunkWriter write/drop/length backfill
- Reader::new/iter、ChunkIterator::try_next/next

| 原函数 | Rust接口 | 版本与限定范围 | 状态与证据 |
|---|---|---|---|
| motomoto::tag_from_str | [reconstruction/shared-foundation/crates/motomoto/src/lib.rs](../../reconstruction/shared-foundation/crates/motomoto/src/lib.rs)：`tag_from_str` | B：原strict UTF-8 bytes长度/tag规则；静态与候选边界对照 | scoped_business_comparison：[evidence/small-libs-batch-01/motomoto/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/motomoto/BUSINESS_RULES.md), [evidence/small-libs-batch-01/motomoto/delivery.json](../../evidence/small-libs-batch-01/motomoto/delivery.json), [evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json) |
| motomoto::tag_from_str_trunc | [reconstruction/shared-foundation/crates/motomoto/src/lib.rs](../../reconstruction/shared-foundation/crates/motomoto/src/lib.rs)：`tag_from_str_trunc` | B：原trunc规则；静态与候选边界对照 | scoped_business_comparison：[evidence/small-libs-batch-01/motomoto/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/motomoto/BUSINESS_RULES.md), [evidence/small-libs-batch-01/motomoto/delivery.json](../../evidence/small-libs-batch-01/motomoto/delivery.json), [evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json) |
| motomoto::Writer::add_chunk | [reconstruction/shared-foundation/crates/motomoto/src/lib.rs](../../reconstruction/shared-foundation/crates/motomoto/src/lib.rs)：`add_chunk` | B：str/u64泛型实例合并；原IO/error顺序静态合同与候选读写验证；独立review另有20限定Cursor原Writer/Drop成功运行，非泛型IO/完整consumer验收 | scoped_business_comparison：[evidence/small-libs-batch-01/motomoto/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/motomoto/BUSINESS_RULES.md), [evidence/small-libs-batch-01/motomoto/delivery.json](../../evidence/small-libs-batch-01/motomoto/delivery.json), [evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json), [evidence/small-libs-review-2026-10-13/motomoto/REVIEW.md](../../evidence/small-libs-review-2026-10-13/motomoto/REVIEW.md) |
| motomoto::ChunkWriter::seek_back_and_write_length | [reconstruction/shared-foundation/crates/motomoto/src/lib.rs](../../reconstruction/shared-foundation/crates/motomoto/src/lib.rs)：`seek_back_and_write_length` | B：原Drop宿主内联回填规则静态合同；未原Writer/Drop动态执行；独立review另有20限定Cursor原Writer/Drop成功运行，非泛型IO/完整consumer验收 | scoped_business_comparison：[evidence/small-libs-batch-01/motomoto/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/motomoto/BUSINESS_RULES.md), [evidence/small-libs-batch-01/motomoto/delivery.json](../../evidence/small-libs-batch-01/motomoto/delivery.json), [evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json), [evidence/small-libs-review-2026-10-13/motomoto/REVIEW.md](../../evidence/small-libs-review-2026-10-13/motomoto/REVIEW.md) |
| motomoto::ChunkIterator::try_next | [reconstruction/shared-foundation/crates/motomoto/src/lib.rs](../../reconstruction/shared-foundation/crates/motomoto/src/lib.rs)：`try_next` | B：173原RNE受控Reader指令案例；泛型IO/panic及完整save consumer未验 | scoped_business_comparison：[evidence/small-libs-batch-01/motomoto/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/motomoto/BUSINESS_RULES.md), [evidence/small-libs-batch-01/motomoto/delivery.json](../../evidence/small-libs-batch-01/motomoto/delivery.json), [evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json) |

## 明确缺口

- 完整Snapshot/Terrain存档schema/压缩/consumer接线
- 泛型非Cursor flush/IO失败原合同、完整writer消费者
- 原Reader::new共享地址ABI
- A/C行为验收

下一闭包：在真实save consumer恢复schema/压缩/失败事务，按需核实泛型IO合同；不重复未变孤立案例

## 证据

- [evidence/small-libs-batch-01/motomoto/delivery.json](../../evidence/small-libs-batch-01/motomoto/delivery.json)
- [evidence/small-libs-batch-01/motomoto/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/motomoto/BUSINESS_RULES.md)
- [evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json)
- [evidence/small-libs-batch-01/integration/dependency-validation.json](../../evidence/small-libs-batch-01/integration/dependency-validation.json)
- [evidence/small-libs-review-2026-10-13/motomoto/REVIEW.md](../../evidence/small-libs-review-2026-10-13/motomoto/REVIEW.md)
- [evidence/small-libs-review-2026-10-13/motomoto/findings.json](../../evidence/small-libs-review-2026-10-13/motomoto/findings.json)
- [evidence/small-libs-review-2026-10-13/integration/summary.json](../../evidence/small-libs-review-2026-10-13/integration/summary.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/motomoto/src/lib.rs` | A,B,C | [reconstruction/shared-foundation/crates/motomoto/src/lib.rs](../../reconstruction/shared-foundation/crates/motomoto/src/lib.rs)：original_pdb_rules_and_scoped_native_or_candidate_comparison<br>[skeleton/task-index/work/TG-27c9dd1e5f1a286560/abc-tools-52/crates/motomoto/src/lib.rs](../../skeleton/task-index/work/TG-27c9dd1e5f1a286560/abc-tools-52/crates/motomoto/src/lib.rs)：explicit_candidate_delivery_mapping | motomoto::impl$0::as_tag（implemented_unverified；业务等价以限定证据为准）<br>motomoto::tag_from_str（scoped_business_comparison；业务等价以限定证据为准）<br>motomoto::tag_from_str_trunc（scoped_business_comparison；业务等价以限定证据为准）<br>motomoto::Writer::add_chunk（scoped_business_comparison；业务等价以限定证据为准）<br>motomoto::ChunkWriter::seek_back_and_write_length（scoped_business_comparison；业务等价以限定证据为准）<br>motomoto::ChunkIterator::try_next（scoped_business_comparison；业务等价以限定证据为准） | candidate_present；完整可调用chunk候选链；173原Reader案例，Writer/Drop仅静态与候选对照。：[evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json)<br>motomoto::tag_from_str：原strict UTF-8 bytes长度/tag规则；静态与候选边界对照：[evidence/small-libs-batch-01/motomoto/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/motomoto/BUSINESS_RULES.md), [evidence/small-libs-batch-01/motomoto/delivery.json](../../evidence/small-libs-batch-01/motomoto/delivery.json), [evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json)<br>motomoto::tag_from_str_trunc：原trunc规则；静态与候选边界对照：[evidence/small-libs-batch-01/motomoto/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/motomoto/BUSINESS_RULES.md), [evidence/small-libs-batch-01/motomoto/delivery.json](../../evidence/small-libs-batch-01/motomoto/delivery.json), [evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json)<br>motomoto::Writer::add_chunk：str/u64泛型实例合并；原IO/error顺序静态合同与候选读写验证；独立review另有20限定Cursor原Writer/Drop成功运行，非泛型IO/完整consumer验收：[evidence/small-libs-batch-01/motomoto/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/motomoto/BUSINESS_RULES.md), [evidence/small-libs-batch-01/motomoto/delivery.json](../../evidence/small-libs-batch-01/motomoto/delivery.json), [evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json), [evidence/small-libs-review-2026-10-13/motomoto/REVIEW.md](../../evidence/small-libs-review-2026-10-13/motomoto/REVIEW.md)<br>motomoto::ChunkWriter::seek_back_and_write_length：原Drop宿主内联回填规则静态合同；未原Writer/Drop动态执行；独立review另有20限定Cursor原Writer/Drop成功运行，非泛型IO/完整consumer验收：[evidence/small-libs-batch-01/motomoto/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/motomoto/BUSINESS_RULES.md), [evidence/small-libs-batch-01/motomoto/delivery.json](../../evidence/small-libs-batch-01/motomoto/delivery.json), [evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json), [evidence/small-libs-review-2026-10-13/motomoto/REVIEW.md](../../evidence/small-libs-review-2026-10-13/motomoto/REVIEW.md)<br>motomoto::ChunkIterator::try_next：173原RNE受控Reader指令案例；泛型IO/panic及完整save consumer未验：[evidence/small-libs-batch-01/motomoto/BUSINESS_RULES.md](../../evidence/small-libs-batch-01/motomoto/BUSINESS_RULES.md), [evidence/small-libs-batch-01/motomoto/delivery.json](../../evidence/small-libs-batch-01/motomoto/delivery.json), [evidence/small-libs-batch-01/integration/source-review.json](../../evidence/small-libs-batch-01/integration/source-review.json) |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/shared-foundation/crates/motomoto/src/lib.rs

[reconstruction/shared-foundation/crates/motomoto/src/lib.rs](../../reconstruction/shared-foundation/crates/motomoto/src/lib.rs)

SHA256：`7289ac4504d65d0044c148eb420f52e925b8a3c891877508362367565e7d196c`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `as_tag` | 7 | 未由扫描证明；以证据限定范围为准 |
| `as_tag` | 10 | 未由扫描证明；以证据限定范围为准 |
| `as_tag` | 15 | 未由扫描证明；以证据限定范围为准 |
| `tag_from_le_bytes_truncated` | 19 | 未由扫描证明；以证据限定范围为准 |
| `tag_from_str` | 25 | 未由扫描证明；以证据限定范围为准 |
| `tag_from_str_trunc` | 29 | 未由扫描证明；以证据限定范围为准 |
| `new` | 39 | 未由扫描证明；以证据限定范围为准 |
| `add_chunk` | 45 | 未由扫描证明；以证据限定范围为准 |
| `finish` | 59 | 未由扫描证明；以证据限定范围为准 |
| `write` | 71 | 未由扫描证明；以证据限定范围为准 |
| `flush` | 74 | 未由扫描证明；以证据限定范围为准 |
| `seek_back_and_write_length` | 79 | 未由扫描证明；以证据限定范围为准 |
| `drop` | 92 | 未由扫描证明；以证据限定范围为准 |
| `new` | 106 | 未由扫描证明；以证据限定范围为准 |
| `iter` | 109 | 未由扫描证明；以证据限定范围为准 |
| `try_next` | 119 | 未由扫描证明；以证据限定范围为准 |
| `next` | 141 | 未由扫描证明；以证据限定范围为准 |
### skeleton/task-index/work/TG-27c9dd1e5f1a286560/abc-tools-52/crates/motomoto/src/lib.rs

[skeleton/task-index/work/TG-27c9dd1e5f1a286560/abc-tools-52/crates/motomoto/src/lib.rs](../../skeleton/task-index/work/TG-27c9dd1e5f1a286560/abc-tools-52/crates/motomoto/src/lib.rs)

SHA256：`692f74f641b1bbae2a224c35b5a4f1594fd74c446fe5d83690549f64068cdf8d`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `as_tag` | 15 | 未由扫描证明；以证据限定范围为准 |
