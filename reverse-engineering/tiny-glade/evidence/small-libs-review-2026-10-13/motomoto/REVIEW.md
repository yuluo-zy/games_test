# motomoto 独立代码审查（2026-10-13）

结论：本轮在已恢复的 chunk 协议范围内未发现 confirmed implementation bug、重复手写公共依赖或已确认性能退化。核心写入、Drop 回填及 Reader 的非典型边界与原 B 指令匹配。完整业务链仍未闭合：当前没有真实存档 producer/consumer 接线，不能将库级测试计为完整存档兼容或整库原作功能验收。

本review没有修改 crate、shared Cargo、全局tracking或任何数据库；仅写本目录。未启动MCP worker，所以不存在本轮需保存/停止的DB后端。继承项目约束和 reverse-engineer-anything 技能，优先复用原身份绑定证据，再独立运行有具体歧义的原指令片段。未重复未变化的173 Reader旧案例或6旧tests。

## 输入与独立证据

- 原 B RNE SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03；PDB私有源文件 crates/motomoto/src/lib.rs，源校验和 47bf8eb6b797a4bc4169386dd82cfdc8a32cf5b9c582a2cc27b014b0006ed058。本轮每个oracle重新验证RNE身份。
- [input-hashes.json](input-hashes.json)冻结实现/测试/Cargo/骨架/原锁；本轮源码没有变动。
- fresh LLVM指令切片：[writer](writer-fresh.txt)、[drop](drop-fresh.txt)、[Snapshot调用](consumer-snapshot-fresh.txt)、[动态资源调用](consumer-resource-fresh.txt)、[Terrain调用](consumer-terrain-fresh.txt)。这些重新从发行RNE读取，不把原作者报告作为结论。
- [writer-oracle.json](writer-oracle.json)：20个独立原Writer+Drop成功fixtures，起始位置0/3/31/1024、payload 0/1/7/64/2048；另pending-error早返回fixture。成功验证header、payload、end position、clear error。仅执行预分配Cursor<Vec>且不扩容；u64 AsTag调用位移重定位到同一RNE的4-byte原identity leaf。没有外部调用、panic/allocator执行或整游戏启动。
- [reader-resume-oracle.json](reader-resume-oracle.json)：3组新原Reader错误后继续next，验证InvalidInput保留position16后继续产出3个空chunk，以及UnexpectedEof seek到116后None。专门补旧oracle遇Err即停止的覆盖边界；没有执行原panic路径。
- [candidate-review-run.json](candidate-review-run.json)：独占scratch以原byteorder 1.5.0和被审motomoto编译，exit0。对照相同20 Cursor header/length/end、3组reader恢复、4个Drop失败位置、payload panic unwind、4096 chunks零复制指针与局部计数/计时。stderr中的injected payload panic被catch_unwind正常捕获，并非测试失败。初版harness cast编译失败已修正，保留 [failure](candidate-review-harness-compile-failure.json)，不是crate缺陷。

## 正确性与完整性

| 规则 | 本轮审查结果 | 证据范围 |
|---|---|---|
| u64直接tag、UTF-8 byte tag、8-byte trunc/strict panic | 静态规则与实现一致；LittleEndian由byteorder负责 | 既有tag RNE指令 + 本轮identity leaf鲜读；未扩未变旧cases |
| pending error先take，当前位置→tag→零length header | 符合；pending error早返回不碰stream/tag | fresh Writer + 原u64入口新fixture；字符串tag顺序静态 |
| payload委托Write；回填长度后恢复end；Drop替换error | 符合；新原Cursor运行验证正常回填、0 payload、非零起点 | 无需复刻编译器Vec/Cursor展开 |
| 原泛型IO失败 | 候选四故障点短路/finish上报正确，原泛型行为仍Unknown | 本发行只观察到Cursor实例，不能用候选故障流证明原File/BufWriter |
| Reader尾部 <=16 bytes忽略；终端empty忽略 | 原规则保留，属于 inherited behavior | 原unsigned >= 与旧identity-bound oracle证据 |
| 高位length signed seek、InvalidInput后继续、向前越界EOF | 新resume原/候选匹配；length=u64::MAX的slice panic为继承原静态规则 | 原panic外部调用未运行；不能声称所有畸形输入动态对照 |
| finish/flush | finish是明确扩展；flush委托是合理std适配，原flush未知 | 不把扩展计原恢复；finish没有durability承诺 |
| 真实snapshot闭合 | 未完成 | 原capture有Snapshot/资源/Terrain三调用，当前重建真实save消费者缺失；外层serde_json/flate2/schema/回退/迁移另需闭合 |

故障流显示PatchWrite失败后位置8、Restore失败后位置16；候选正确留下deferred error，且finish返回Err。库没有回滚损坏header的承诺，也没有原Cursor可失败证据，不能把该现象误报原规则偏差。payload panic仍由Drop回填已写partial bytes；这与原释放策略的静态控制流一致，调用者需丢弃失败事务。

Reader的高位长度panic、末尾空chunk丢弃、truncated tail忽略及资源tag前8字节碰撞都不能擅自改成通常解析器规则。如果产品要更严格/安全解析，应新增明确API并保留兼容入口，先核实真实consumer需求。

## 骨架函数参考对齐

骨架只有一个原lib.rs。按业务分组核对而非把单态化数量当功能分母：

| 观察线索 | 当前实现 |
|---|---|
| impl0/impl1 as_tag、tag_from_le_bytes_truncated/tag_from_str/tag_from_str_trunc | AsTag u64/&str及3 tag helpers |
| Writer::new、add_chunk<Cursor,&str>和<Cursor,u64> | 泛型Writer new/add_chunk统一策略；两个实例不重复造两个实现 |
| impl3 write、ChunkWriter::seek_back_and_write_length、impl5 drop | std Write委托、回填private helper及Drop |
| Reader::new/iter、impl9 next、try_next | slice Reader、独立Cursor迭代器及transpose |

PDB inline实例不是独立公共API，公开可见性/原字段名/准确ABI未恢复。as_tag u64及Reader::new的共享地址别名不算额外业务函数；Reader::new地址0x14067dd80的大量无关调用不能算真实motomoto消费者。Write::flush和Writer::finish为当前候选适配/扩展，骨架未列出也不自动判缺少原接口。

## 依赖与造轮子审查

原Cargo motomoto 0.1.0只依赖byteorder；原byteorder 1.5.0 registry checksum 1fd0f2584146f6f2ef48085050886acf353beff7305ebd1ae69500e27c67f64b。被审Cargo显式=1.5.0，与原锁一致；scratch只引入此相同依赖。

- read_u64/write_u64/LittleEndian复用byteorder；Cursor、Read/Write/Seek、write_all、Vec、io::Error复用std。
- 8-byte零补齐、tag截断、16-byte chunk framing、deferred Drop slot是原私有协议策略，不是重复实现通用序列化器。
- 没有手写压缩、JSON、哈希、IO retry、Vec缓冲或字节序转换器。存档外层应直接接原serde_json/flate2，不能在此补造。
- 本目录Python/PE节定位/ctypes原码调用是分析工具，不能计为重建交付或通用库替身。

## 性能

Reader为O(chunk数量) header工作，payload只返回借用slice：本轮4096 chunks ×100遍共409600条，assert payload pointer直接落在原buffer；release单次局部用时2.301ms（输入98304bytes）。这个小样本只说明本机限定路径的成本，不能外推原游戏存档速度，也没有原/候选benchmark比值。

正常计数流每chunk共4个Seek调用（add位置+drop位置+回填位置+恢复位置）、4个Write调用（两个header、一次payload、一次length）、0个flush。原应用Cursor是内存常量操作；直接File/BufWriter可能增加syscall/缓冲flush，是选流方式风险而非已确认回归。payload Write委托std，由底层决定分配/复制；motomoto本身没有额外payload Vec/copy。默认保留原Cursor聚合→存档consumer文件写入策略，无依据时不新增手写缓冲器。

## 最终处置

[findings.json](findings.json)包含0个confirmed implementation bug，1个confirmed functionality scope gap，以及泛型合同Unknown、继承异常规则、限定性能风险和文档化扩展。当前适合保留核心实现，下一工作应补真实save consumer而非增加函数计数或大量重复oracle。完整库/存档功能、泛型flush/IO失败原等价性与原侧性能仍未验收。
