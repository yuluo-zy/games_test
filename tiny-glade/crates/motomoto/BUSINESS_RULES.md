> 历史原作对照记录。2026-10-17 源码已工程化重构，修正了本文件记录的坏输入与 I/O 故障行为；以下实现名、测试数、源码哈希与零差异结论仅适用于历史快照。当前合同以 [README.md](README.md) 为准。本次未重新逆向或升级原进程验收。

# motomoto：原作 B 版业务规则（历史记录）

## 当前修正规则

发现的错误不会再因“历史行为”或“兼容测试”而保留。2026-10-17 已完成以下修正：

- 空 chunk 在末尾正常返回；残缺头和 payload 返回一次 `UnexpectedEof`，随后永久终止。
- 任意 `u64` payload 长度都经过输入范围校验，取消 signed cast、向后 seek 和坏输入 panic。
- 写入位置采用受检算术；I/O 故障永久标记 writer 失败，不消费或清除错误后继续写入。
- `ChunkWriter::finish()` 显式报告回填错误；Drop 仅兜底，不重试失败回填。
- 固化旧错误的生成用例已删除，当前十个测试验证修正后的合同。

下文只保留原作观察和历史验证出处，不作为当前实现要求。

2026-10-13 历史状态：可调用 chunk 写入→长度回填→slice 读取链已经实现并编译验证。Reader 的 173 个受控原指令案例与候选零差异；Writer/Drop 目前为原指令静态规则对照与候选格式/失败测试，未称为原 Writer 入口运行验收。完整游戏存档消费链接线仍未完成。

## 来源和证据身份

- 原锁 motomoto 0.1.0 无 source，唯一依赖 byteorder 1.5.0；local 身份本身不是私有证明。
- PDB 定义路径为 /home/h3/code/pouncelight/country-slice-private/crates/motomoto/src/lib.rs，源码校验和 47bf8eb6b797a4bc4169386dd82cfdc8a32cf5b9c582a2cc27b014b0006ed058。游戏自有的 chunk 协议为恢复范围。
- 发行 B RNE SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03 是指令及受控执行 oracle。
- Windows 专属 worker small-motomoto-20261013 的独立持久 clone 绑定 patched EXE SHA256 cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323。未修改函数签名/注释，也未同步原库。
- PDB 骨架只有 lib.rs；骨架注释所指 .pdb.jsonl.gz / _pdb/B 本地缺失。此次直接 LLVM 提取原 PDB module 485 定位 Drop 宿主，不把缺失档当实际读到的证据。

## Observed（观察事实）

| 原规则 | 依据 | 历史实现与对照（已过期） |
|---|---|---|
| u64 tag 原值返回；字符串取 UTF-8 原字节、零补足 8 字节、LE 解释；strict 版本长度超过 8 bytes panic，trunc 版本仅保留前 8 bytes，可以截断多字节字符 | tags-rne-disassembly.txt：0x142080ca0、0x142080d00；Writer u64 调用 0x1400885d0 | AsTag/tag_from_str/tag_from_str_trunc/tag_from_le_bytes_truncated；候选 tag 边界测试 |
| chunk 无额外 magic/padding：tag:u64 LE、payload_length:u64 LE，随后原 payload bytes | writer-rne-disassembly.txt 中当前 Cursor 位置写 tag、位置+8 写零；Reader 0x142080de6/0x142080dea 取两 word | Writer::add_chunk + byteorder；黄金字节格式与连续写读测试 |
| add_chunk 首先 take 并清空前一次 Drop 的错误；有错误则不进行 tag 转換、不写新 header；成功后读当前位置，再转换 tag，再写 tag/零长度 | 0x1415526aa..0x1415526d7；u64 实例同序 | deferred_error.take → stream_position → tag.as_tag → write_u64 → write_u64；失败先于非法长 tag 的候选测试 |
| 写 payload 的成功 byte 数与 Cursor 位置由 std IO 更新；协议不自行压缩或编码 payload | PDB impl$3::write 在 std Write / serde_json / flate2 宿主内联；Cursor 原机器码展开 | 直接委托 std::io::Write；partial-write/write_all 采用 std 实现 |
| Drop 记录 end；seek start+8；写 end-start-16；seek end；把 error slot 替换为 Result.err() | pdb-module-485.txt record 150376/150588/150620；drop-rne-disassembly.txt 0x14087abf0；成功路径清空 slot | seek_back_and_write_length + Drop；显式 wrapping 采用原 release 地址算术；回填位置与末尾恢复测试 |
| Reader::iter 从零创建独立 Cursor，不延续旧 iterator | 0x142080d50 复制 slice ptr/len并写 position=0 | Reader::iter |
| position+16 >= input length 时返回 None，不消费尾部。恰好 16 字节的最后一个空 chunk 被忽略；不足 16 的尾部也被忽略 | Reader RNE 0x142080d7f..0x142080d86 为 unsigned >= | try_next/next；原173case含尾部0..16与17字节最小非空chunk、空chunk后接非空chunk |
| header 先LE读 tag/length，然后 SeekFrom::Current(length as i64)，更新位置后再检查 payload end | 0x142080de3..0x142080e0e；PDB byteorder::ReadBytesExt + std Cursor::seek | byteorder ReadBytesExt + std Seek |
| end > input_len 返回 UnexpectedEof 且位置已更新为 end；无EOF fusion标记 | 0x142080e07..0x142080e1a；0x2500000003 为原 Simple ErrorKind | 原码案例验证返回分类及 cursor.position；不会把错误改成截短成功 |
| 超大 length 的 signed cast可能产生 InvalidInput；若合法向后seek到起点之前，则 slice start>end panic | 0x142080df5..0x142080e05 signed/carry检查及0x142080e1c..0x142080e48顺序检查 | InvalidInput 路径有受控原执行；length=u64::MAX 的 panic 分支只静态对照+候选 catch_unwind，未执行原panic外部call |
| Iterator::next 是 try_next().transpose()，返回 Option<Result<Chunk,io::Error>> | PDB Result::transpose内联 + 0x142080dd3/0x142080dcc 输出区布局 | 独立 try_next/Iterator API，非原 ABI复刻 |

## 真实业务消费者

原 B country-core/src/systems/save/snapshot.rs 的 capture_game_snapshot（0x1417a1d00）生产该格式；其 Writer::add_chunk 的已观察调用为 0x1417a31fa、0x1417a35fa、0x1417a3a50。

- 0x1417a31e2 传固定 8-byte 字符串 Snapshot，0x1417a31fa 调字符串 add_chunk。
- 动态资源段在 0x1417a35e7 调 tag_from_str_trunc，随后 0x1417a35fa 调 u64 add_chunk；因此资源名相同前8byte发生tag碰撞的可能性来自原规则，库不做去重。
- 0x1417a3a38 传固定 7-byte 字符串 Terrain，0x1417a3a50 调字符串 add_chunk。
- 固定字符串实际 RNE .rdata读取见 consumer-tags.json；调用片段见 consumer-chunk1/2/3-rne.txt。
- PDB 在该 capture 函数记录 serde_json::to_writer、flate2::DeflateEncoder::new/finish、ChunkWriter::drop 的内联；读取侧 validate_and_load_closest_snapshot_data（0x1417a6e50）调用 Reader::iter/next，且含 DeflateDecoder。压缩和 JSON 属于消费者策略，复用原库；本协议 reader仅交付payload slice。
- 具体所有动态tag的生产/消费表、Snapshot/Terrain每段的完整schema、压缩设置、迁移/回退规则尚未在本小库范围完成，不以依赖内联证据推断完整存档语义。

## Inferred / reconstruction choices（推断和自主表示）

- 公共接口选择 Writer<W> 持有 W、ChunkWriter 借用父 Writer、Chunk { tag, data }，较清楚且避免复制payload。作者准确泛型签名/公开字段名/trait可见性没有恢复；不声称ABI相等。
- 新增 Writer::finish 用于显式获取最后Drop错误与底层流，是重建扩展，未计原接口恢复。
- ChunkWriter::flush 委托底层流是标准Write适配选择；原具体flush实现没有独立证据，列入Unknown，不计原接口闭合。
- 证据里的 Writer 是 Cursor<&mut Vec<u8>>，无法触发普通IO失败。泛型 W 的 seek/write 失败传播按 ? 顺序和deferred error槽推断，并用自定义失败流候选测试；不能称原所有流实现验收。

## Unknown / 未闭合

- 原整体Writer::new在非零初始流位置及IO失败能力不同的泛型实例未原入口执行；候选支持当前位置。
- 原 drop 在写错误/seek错误/恢复末尾错误时的原实际动态案例，最后 deferred error 的最终消费者处理。
- 原 flush 合同；通用流的错误文本，不保证std版本的Display字符串逐字相同。
- 全存档consumer接线、真实游戏产生的存档文件跨版读取。此次目录内限定搜索没有可身份绑定的原B存档样本，不能将合成fixture称真实存档。
- PDB Reader::new 0x14067dd80共享地址别名多；未从它恢复独立ABI或给它执行覆盖数字。

## 验证与差异处理

6 个 Rust tests 通过，含173原Reader受控指令案例、格式黄金bytes、连续chunk、UTF-8长度/trunc、partial Write、回填末尾恢复及deferred error先后。原Reader oracle仅执行已审查不会external call/panic的分支；未启动游戏。详细结果 reader-oracle.json、cargo-test-msvc.log。

首次 cargo test 因child进程未带MSVC PATH/LIB而缺link.exe（cargo-test.log），改用已安装BuildTools/SDK的child环境后通过；未安装工具或改全局配置。模型环境可见ReVa namespace没有加载程序；SDK worker读取Reader成功但后续Writer反编译请求卡住，read-result.json保留为部分报告，不能把其ok=false称全批成功。Writer采用原RNE disassembly核对。

历史回看记录：当时将 Drop 错误槽替换和 wrapping 算术调整为原 release 行为，并保留了末尾 16 字节空 chunk 被忽略的原边界。2026-10-17 已取消这些错误行为：错误槽永久保留失败，位置使用受检算术，末尾空段正常读取。


## 2026-10-13 独立模块验收补充

2026-10-13 当时登记了 11 项候选测试；本历史记录中的测试数和“核心规则未改”结论不再适用于当前源码。当前模块合同与验证以 [FUNCTIONALITY.md](FUNCTIONALITY.md) 为准。原 flush 及原泛型失败动态合同仍属历史未验范围，游戏消费者接线仍另列。
