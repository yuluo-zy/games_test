# motomoto

存档使用的带 8 字节 tag 和长度头的 chunk 容器；这里只处理容器业务，不重写 IO 或压缩器。

## 功能与目录

src/lib.rs（Writer/ChunkWriter/Reader、tag 编码）；src/tests.rs、src/tests/io_contract.rs。

## 来源

- 恢复源码：[shared-foundation/motomoto](../../../reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/motomoto/)，完整源路径为 `reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/motomoto/`。
- 原 B 包/PDB 归属：`crates/motomoto/src/`；这是行为恢复源码，不是取得作者原始 Rust 文本。
- 本次选用提交：`3e0387cd`、`c05fad96`；每文件完整提交与输入/输出 SHA256 见 [foundation-audit.json](../../provenance/foundation-audit.json)。
- 逆向/验收报告：[REPORT.md](../../../reverse-engineering/tiny-glade/evidence/small-libs-batch-02-2026-10-13/REPORT.md)；[BUSINESS_RULES.md](../../../reverse-engineering/tiny-glade/evidence/small-libs-batch-02-2026-10-13/motomoto/BUSINESS_RULES.md)。

## 已完成

tag 长度检查/截断、add_chunk 错误槽顺序、Drop 长度回填、Reader iter/next 的截断尾部/UnexpectedEof/溢出边界均实现并有原入口复核；存档 capture/read 消费者已定位。

## 验收边界

- **泛型 IO：**候选实现支持满足 `Write + Seek` 的流。`File`、`BufWriter` 及可注入故障的流已验证；header/payload 写入失败会返回错误，Drop 回填失败会延迟到后续 `add_chunk` 或重建扩展 `finish()` 报告，`flush()` 则直接委托底层流。原 B 版目前只确认了 `Cursor<&mut Vec<u8>>` 单态化，原作的非 Cursor 失败路径和 `flush()` 合同尚无独立证据，因此候选流测试不代表原作泛型实例也已验收。详见 [功能验收](FUNCTIONALITY.md) 和 [业务规则及证据边界](BUSINESS_RULES.md)。
- **`ChunkWriter` 布局：**原作 `add_chunk` 的反编译显示返回值占三个机器字；当前候选结构保存 `parent` 与 `start` 两个逻辑字段，并通过 `parent` 访问底层流。候选实现已覆盖 chunk 写入和长度回填行为，但原三字段各自的类型、含义及偏移尚未全部确认，不能声称内存布局/ABI 相同。此项只影响要求复现原作内部二进制布局或调用约定的场景，不影响当前 Rust API 的 chunk 读写格式。
- **更大范围验收：**实际世界存档 schema 与压缩快照消费者接线、发行进程整链、A/C 版本仍未验收；这些属于消费者集成或跨版本验证，不是本 crate 的 chunk 基本读写功能。

在 workspace 根执行 `cargo test -p motomoto --all-targets`。本说明列的是实现与证据范围，当前集成验证结果以根工程报告为准。
