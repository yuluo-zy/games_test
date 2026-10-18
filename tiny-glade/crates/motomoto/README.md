# motomoto

`motomoto` 提供同步、无压缩的二进制 chunk 容器，用于按 tag 顺序写入和读取独立 payload。读取端借用输入切片，写入端通过可定位的输出流回填 payload 长度。

本包只负责容器边界和 I/O 生命周期。payload 的 schema、版本迁移、tag 唯一性、压缩、校验和、加密以及文件原子替换由调用方负责。

## 模块结构

| 文件 | 职责 |
|---|---|
| [`src/lib.rs`](src/lib.rs) | 公开 API 导出与内部协议常量。 |
| [`src/tag.rs`](src/tag.rs) | 数值及字符串 tag 转换。 |
| [`src/reader.rs`](src/reader.rs) | 零拷贝 chunk 读取、长度校验及迭代终止。 |
| [`src/writer.rs`](src/writer.rs) | 输出流所有权、chunk 写入、长度回填及失败状态。 |
| [`src/tests.rs`](src/tests.rs) | 协议字节、tag、截断输入及部分写入测试。 |
| [`src/tests/io_contract.rs`](src/tests/io_contract.rs) | 真实文件、缓冲输出、I/O 故障及资源生命周期测试。 |

## 二进制协议

每个 chunk 由固定的 16 字节头和紧随其后的 payload 组成；多个 chunk 直接拼接。

| 相对偏移 | 长度 | 编码与含义 |
|---|---|---|
| `0` | 8 字节 | 小端 `u64` tag。 |
| `8` | 8 字节 | 小端 `u64` payload 长度，单位为字节，不包括头部。 |
| `16` | 指定长度 | payload 原始字节。 |

容器没有总头、magic、对齐填充或结束标记。空输入表示零个 chunk；零长度 chunk 可以出现在任何位置，包括文件末尾。所有 `u64` tag 均有效，重复 tag 不会被合并或拒绝。协议不能检测 payload 内部损坏。

字符串 tag 使用 UTF-8 原始字节，右侧补零后按小端解释为 `u64`。因此限制按字节计算，而非字符数；`"abcdefgh"` 恰好八字节，`"é"` 占两字节。补零和截断可能让不同输入得到相同 tag，调用方需要自行设计名称空间。

## 公开 API

### Tag 转换

| API | 合同 |
|---|---|
| `AsTag::as_tag(&self) -> u64` | `Writer::add_chunk` 接受的转换接口，内置实现为 `u64` 和 `&str`。 |
| `tag_from_str(&str) -> u64` | 接受最多八个 UTF-8 字节；超过限制会 panic。 |
| `tag_from_str_trunc(&str) -> u64` | 只保留前八个 UTF-8 字节，可能在字符编码内部截断；输出是数值 tag，无需保持有效 UTF-8。 |
| `tag_from_le_bytes_truncated(&[u8]) -> u64` | 只保留前八字节，较短输入补零；空输入返回零。 |

### 读取

| API | 合同 |
|---|---|
| `Reader::new(&'a [u8]) -> Reader<'a>` | 借用输入，不复制或预解析内容。 |
| `Reader::iter(&self) -> ChunkIterator<'a>` | 创建从输入起点开始的独立迭代器。 |
| `ChunkIterator::try_next(&mut self) -> io::Result<Option<Chunk<'a>>>` | 返回下一个 chunk、结束或格式错误。 |
| `ChunkIterator` 的 `Iterator` 实现 | `Item = io::Result<Chunk<'a>>`；支持 `for`、`collect` 等标准迭代操作。同时实现 `FusedIterator`。 |
| `Chunk<'a>` | 公开字段 `tag: u64`、`data: &'a [u8]`，payload 借用原输入，生命周期不依赖 `Reader` 本身。 |

每次读取先检查头部长度，再将编码的长度转换为可寻址长度，验证其不超过剩余输入，最后切分 payload 和余下数据。空输入正常结束；残缺头、残缺 payload 以及超出可寻址或输入范围的长度均返回 `UnexpectedEof`。错误只报告一次；之后 `try_next()` 返回 `Ok(None)`，`next()` 返回 `None`。迭代器不会尝试重新同步或跳过坏段。

### 写入

| API | 合同 |
|---|---|
| `Writer::new(W) -> Writer<W>` | 接管 `W: Write + Seek`，从其当前位置开始使用；构造本身不写入。 |
| `Writer::add_chunk<T: AsTag>(&mut self, T)` | 返回 `io::Result<ChunkWriter<'_, W>>`，写入 tag 和零长度占位头；字符串过长时在写入前 panic。 |
| `ChunkWriter` 的 `Write` 实现 | 通过 `write`、`write_all` 等方法向当前 payload 顺序写入；`flush` 刷新底层流。 |
| `ChunkWriter::finish(self) -> io::Result<()>` | 消耗当前段，回填长度并报告错误。 |
| `Writer::finish(self) -> io::Result<W>` | 消耗容器 writer，检查失败或未完成状态，成功时交回底层流；不执行 `flush`。 |

## 写入流程与资源生命周期

1. `add_chunk` 记录当前位置，检查头部位置加法，写入 tag 和长度占位。返回的 `ChunkWriter` 独占借用父 writer，因此正常使用时不能同时创建多个段。
2. 调用 `Write` 方法写入 payload。`ChunkWriter` 不提供 `Seek`，payload 按顺序写入。
3. `ChunkWriter::finish()` 读取末尾位置，计算长度，seek 到长度字段并覆盖，核对实际回填位置，最后恢复 payload 末尾位置。成功后可以创建下一段。
4. 所有段完成后调用 `Writer::finish()` 取回输出流；缓冲输出仍应显式检查 `flush()` 或 `into_inner()` 的结果。

未显式完成的 `ChunkWriter` 在 Drop 时执行相同的回填。Drop 无法返回错误，失败会保存在父 writer 中，由后续操作或 `Writer::finish()` 报告；应优先使用显式 `finish()`。回填失败后不会由 Drop 重试，因为此时流位置可能已位于段头。

使用 `mem::forget()` 等方式遗失 `ChunkWriter` 会留下未完成状态；后续 `add_chunk()` 和 `Writer::finish()` 均返回 `InvalidData`。直接丢弃 `Writer` 不能替代显式完成和错误检查。

## I/O 错误与输出要求

输出流必须支持在 seek 指定的位置覆盖写入。append 模式 `File` 虽然实现 `Write + Seek`，写入仍会追加，因此不受支持。完成段时会核对实际回填位置；发现追加写入后返回 `InvalidData` 并使 writer 永久失效。位置检查使用会刷新 `BufWriter` 的 seek 操作，不能仅依赖其未刷新的逻辑位置。

| 情况 | 行为 |
|---|---|
| 头部位置加法溢出 | 返回 `InvalidInput`，使 writer 失效。 |
| 完成段时末尾位置小于 payload 起点 | 返回 `InvalidData`，使 writer 失效。 |
| 头部、payload、flush、seek 或长度回填失败 | 记录错误，使 writer 永久失效；本包之后的操作不再主动读写底层流。 |
| `write` 返回 `Interrupted` | 向调用方返回该错误，保持可重试状态。 |
| `write_all` 遇到 `Interrupted` | 委托底层 `write_all`，按标准 `Write` 合同重试。 |
| `write_all` 无法推进，返回 `WriteZero` | 记录错误，使 writer 失效。 |
| `ChunkWriter::flush()` | 只刷新流，不回填长度、不关闭当前段。 |
| `Writer::finish()` | 只检查并交回流；不会主动刷新，也不表示数据已经持久化到磁盘。 |

writer 不截断已有文件、不回滚部分写入，也不移除失败输出。调用方应以正确的打开选项准备文件，并在任何写入错误后丢弃该输出。底层缓冲流自身的 Drop 行为仍由其类型决定。需要文件替换或持久化保证时，调用方负责临时文件、原子替换及 `File::sync_all` 等策略。

## 使用示例

下面的程序写入一个有数据的 chunk 和一个空 chunk，显式检查长度回填、容器完成与缓冲刷新，再读取并验证结果。

```rust
use motomoto::{Reader, Writer, tag_from_str};
use std::io::{self, BufWriter, Cursor, Write};

fn main() -> io::Result<()> {
    let output = BufWriter::new(Cursor::new(Vec::new()));
    let mut writer = Writer::new(output);

    let mut chunk = writer.add_chunk("Snapshot")?;
    chunk.write_all(b"payload")?;
    chunk.finish()?;
    writer.add_chunk("empty")?.finish()?;

    let mut output = writer.finish()?;
    output.flush()?;
    let cursor = output.into_inner().map_err(|error| error.into_error())?;
    let bytes = cursor.into_inner();

    let mut chunks = Reader::new(&bytes).iter();
    let first = chunks.try_next()?.expect("Snapshot chunk");
    assert_eq!(first.tag, tag_from_str("Snapshot"));
    assert_eq!(first.data, b"payload");
    let empty = chunks.try_next()?.expect("empty chunk");
    assert_eq!(empty.tag, tag_from_str("empty"));
    assert!(empty.data.is_empty());
    assert!(chunks.try_next()?.is_none());
    Ok(())
}
```

## 性能特征与限制

读取正常路径不分配内存、不复制 payload；每段只解析固定长度头并检查边界。创建迭代器的成本为常数，多个迭代器可以独立扫描同一输入。错误路径会构造 `io::Error`。

写入直接委托底层输出，不在包内收集整段 payload。每段需要头部写入、位置查询、长度回填和游标恢复；这些 seek 可能触发缓冲刷新，因此适合较大 payload，微小段过多时应由调用方合并。输出可以是 `File`、`Cursor` 或具备正确覆盖语义的缓冲流，不能是仅支持顺序写入的 socket 或 stdout。

Reader 要求输入已存在于一块连续切片中，不提供流式 `Read` 解码、随机访问索引或 tag 查询索引。该格式也不携带总长度或校验信息；截断为完整 chunk 边界后的前缀仍是有效容器，需要调用方通过 schema 或完整性信息判断业务数据是否齐全。

## 验证

在 workspace 根目录 `games_test/tiny-glade` 执行：

```sh
cargo test -p motomoto --offline --all-targets
cargo test -p motomoto --offline --release --all-targets
cargo clippy -p motomoto --offline --all-targets -- -D warnings
cargo fmt -p motomoto -- --check
```

测试覆盖协议字节、UTF-8 tag、截断位置、极端长度、迭代终止、部分写入、真实 `File` / `BufWriter`、I/O 故障、位置运算、`Interrupted` / `WriteZero`、遗失 chunk 及 append 输出拒绝。

## 来源与历史材料

模块合同以上述文档与当前源码为准。历史恢复和审查资料仅描述各自快照：

- [BUSINESS_RULES.md](BUSINESS_RULES.md)：历史行为快照，不作为当前错误处理合同的依据。
- [来源台账](../../provenance/foundation-audit.json)与[原恢复入口](../../../reverse-engineering/tiny-glade/reconstruction/shared-foundation/crates/motomoto/)。
- [源码审查报告](../../reports/source-review-2026-10-17/README.md)、[重构报告](../../reports/foundation-refactor-2026-10-17/README.md)与[复审报告](../../reports/foundation-rereview-2026-10-17/README.md)。
