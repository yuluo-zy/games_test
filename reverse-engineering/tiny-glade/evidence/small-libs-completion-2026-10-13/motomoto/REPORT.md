# motomoto 模块功能与独立验收

本轮完成 B 版已观察 motomoto 业务的独立源码/实际 IO 验收，不包括 country-core 存档消费者。核心可执行实现没有修改；未发现需要修复的确认业务缺陷。新增五个测试把泛型 File/BufWriter 和错误合同变成可重复验收。

## 全功能清单

| 业务 | 实现/合同 | 验收范围 |
|---|---|---|
| tag | u64 原值；UTF-8 字节零填；strict 八字节限制；trunc 前八字节 | 原指令静态证据及候选边界测试 |
| 创建 Writer / add_chunk | 当前流位置；先取 deferred error；LE tag+零长度；成功返回借用 ChunkWriter | 原 Cursor Writer/Drop 限定运行证据；候选实际 File/BufWriter |
| payload / flush | std Write 委托；flush 只委托底层，不提前回填 | payload 原内联依赖证据；flush 候选适配合同验证，原 flush 独立证据未知 |
| Drop / length backfill | end→start+8→长度→end；短路并替换 deferred error | 原 Cursor 成功路径运行证据；候选四个 IO 故障阶段 |
| Reader / iter | 借用 slice；每次迭代独立 Cursor | 候选零复制/独立迭代及原静态规则 |
| try_next / next | LE header；signed seek；原尾部/EOF/错误后位置；transpose | 173 既有原受控案例、review 三组原恢复案例 |
| finish | 返回流或最后 Drop 错误 | 明确重建扩展；候选失败测试，不计原接口 |

impl/closure/泛型单态化按业务合并。private 回填 helper 已由 Drop 覆盖。Reader::new 同地址 ABI不影响 Rust 业务实现，不追求 ABI1:1。

## 新验证

- 实际 File 与容量31的 BufWriter：非零起点、非终端空段、131073字节跨缓冲 payload、段内 flush 后继续写、多个段、终端空段，显式 flush/关闭/重开后内容和最终位置吻合。Reader直接借用重读buffer。
- header position/tag/length 失败及时返回；payload失败由调用者处理，Drop只回填已经写成功的部分。这没有回滚或 durability 承诺。
- Drop 的 position/patch seek/patch write/restore 四处故障均短路并由 finish 上报。
- 底层 flush 失败直接返回；不写长度、不占 deferred Drop error；恢复后仍能继续写入。
- 本轮11 tests、all-targets Clippy -D warnings、限定 rustfmt check通过。没有原新函数运行，也没有原游戏运行、原泛型 File/BufWriter 执行或新原性能比较。

原审查的20 Cursor Writer/Drop成功fixtures+pending error、三组Reader恢复以及173 Reader原受控案例可复用：核心source未改，不把候选IO测试增加到原oracle数量。

## 依赖、性能和环境

仅 byteorder 1.5.0（原checksum 1fd0f2584146f6f2ef48085050886acf353beff7305ebd1ae69500e27c67f64b）和 std IO，没有新增压缩/schema依赖或自制缓冲器。Reader保持零复制；泛型 File/BufWriter成本由std和OS决定，本轮只验正确性，不宣称原性能等价。

共享workspace test尝试受到其他模块新增 theme-fetcher / cache_control offline依赖缺失阻塞，原日志 cargo-test.log 保留。隔离scratch直接以绝对路径编译同一生产lib.rs与同一tests.rs，并使用锁定byteorder源/version/checksum；未复制或修改核心算法。scratch仅验收工具，不计游戏恢复源码。构建仅child-local MSVC环境，未改全局配置。

## 状态和限制

独立候选库功能：全部已观察业务已实现，实际IO/失败合同验收通过。原B scoped行为：既有局部证据成立。完整原B所有泛型/flush合同、A/C行为尚未验收；真实游戏consumer后续单列，不阻塞本轮独立库交付，也不能因为本轮通过就报告其完成。

原flush补查限定于B骨架函数符号和已提取PDB module485，未找到可绑定motomoto flush的明确记录；这种无匹配不能证明原flush不存在。保持当前std代理适配并公开证据不足，不臆造原实现。

历史证据保持不变；当前报告以 input-hashes.json 绑定新增tests与同一lib。没有开启或修改数据库worker。
