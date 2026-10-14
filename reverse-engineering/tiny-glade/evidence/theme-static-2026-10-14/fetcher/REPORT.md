# theme-fetcher B 静态复核与补全

本轮完成已定位 B 库内业务面的逐项静态核查和偏差修复。交付目录为 `D:/game/tiny-glade/crates/theme-fetcher`，只修改此 crate 与本 evidence 目录；未调用真实主题服务，未执行原游戏，也未分析 tiny-garden。恢复的是可编译行为源码，未取得作者原始 Rust 文本。

## 原作身份和范围

- B patched EXE / Ghidra Windows worker：SHA256 `cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`，program `/tiny-glade.exe`，worker `themes-fetcher-20261014`。`clock-metadata.json`/`clock-symbols.json` 均 `ok=true`，SDK 脚本 success=true、timedOut=false，实际 MCP 连接；不是原游戏执行。
- 原发行 RNE：SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。原 PDB：SHA256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。先复用绑定该身份的已有函数/PDB证据。
- 4 个原独立发射实例：`new<PathBuf>` 0x14054aa00、`run` 0x140618310、`extract_with_checksums` 0x140d921c0、`verify_folder_checksums` 0x140d939a0。fetch_events/download_blob/trim_cas/validate_cas_cache_entry 内联在 run，不能据缺独立符号判作没有业务。
- 本轮新 worker 中 std clock/mtime 入口为一字节导入体，未补造 ABI 或声称其反编译恢复；新 PDB public 查询独立确认 `Metadata::modified`=0x140d59e60、`Instant::now`=0x140d635e0、`SystemTime::now`=0x140d63ba0，标准库实现仍直接复用。
- Windows 工具入口为 `scripts/reverse_tools/reverse_operator.py` + `D:/tools/reverse-engineering/venvs/reverse-engineering-assistant/Scripts/python.exe`。未使用说明中失效的 Mac路径，未修改共享/原数据库或声称同步 Mac overlay。

## 静态偏差及修复

| 项目 | 原静态证据 | 最终实现 |
|---|---|---|
| Date/ETag 错误副作用 | run.c lines 680–865：Date decode/parse/skew 在 ETag decode/store 之前；原 bytes 调用在必需长度之后 | Reqwest `get` 只拿 headers，`read_body` 延迟 bytes；Date/ETag错误透过默认 Transport hook。有效Date后坏ETag仍留下skew；有效ETag后body读错仍留下ETag；缺必需长度不读body |
| 真实响应生命周期 | 原 fetch返回释放blocking Response；304/错误状态不调用bytes | production仅对合法200/body路径暂存Response，其他分支立即drop；read_body用take消耗，包括body错误。不会将304保留到下次刷新 |
| Date本地时间 | 原 asm 0x14061959b 调用SystemTime::now，位于Date解析及Server date debug日志后，bytes前 | production response_local_time在fetch处理Date时实时采样；fixture默认使用给定时间 |
| 活动时间采样 | run.c LAB_14061a131 调用SystemTime::now，处在fetch结果收口后、逐blob下载前；native 0x14061a131 | production run用分阶段wall-clock回调；固定run_once保留冻时投影，避免一次可能60秒请求期间跨主题边界却仍用请求前时间 |
| CAS mtime错误 | run.c lines 430–447：Metadata::modified Err跳LAB_1406188a3；elapsed Err进入verify；秒=0x127500时还比较非零纳秒 | modified错误传播；future时间继续verify；严格age>14天并保留亚秒 |
| ZIP 父目录 | extract.c LAB_140d92410：parent存在时stat，仅stat Err调用DirBuilder | `parent().filter(!exists)`之后create_dir_all，保留公共exists/目录API |
| ZIP 清单写入 | extract.c 0x140d92bxx先创建清单，随后serde_json Vec缓冲，0x140d92cxx最终write_all；serialize Err不加Writing上下文 | File::create → serde_json::to_vec → write_all；只有最后IO错误加Writing上下文 |
| 原日志 | run.c的fmt pieces地址/数量和明确level词：1 Error，2 Warn，3 Info，4 Debug | 恢复21处原日志记录，target=theme_fetcher；log宏/过滤/回调由原log0.4.27复用，无公共库重写 |

日志精确字符串由 `read_log_formats.py` 直接读取已核SHA的RNE中的Rust fmt pieces数组，输出 `log-formats.json`（包括地址、长度、文本、原level值）。注意events_status使用单一空piece加已构造错误字符串的Display；corrupt日志保留原句式 `Removing corrupted CAS cache entry {path} is corrupted. Reason: ({error})`。以上级别来自记录内明确level词，未从文案猜测。读取脚本仅为取证，不能计作业务恢复。

## 已复核但无需改写的业务

Constructor建cache根、reqwest timeout60秒、无idle pooling、保留Sender；ETag条件请求及缺头保留、Date整数秒skew与负调整时间clamp0；CacheControl max_age默认36000；200替换列表并将每事件data初始化Pending，304保留列表且保留上次fetch error；失败backoff1→2→4…封顶300且成功不重置；所有Err的blob每轮重试，无论事件是否当前活动；已有目录跳过下载；blob可选长度、BLAKE3内容cid、zip原库；活动半开区间与最后命中胜出；无活动时取fetch错误；只发布变化、断开时退出、一秒sleep；CAS逐条文件/目录清理；ZIP归档顺序、enclosed_name、完整目的路径存清单、aggregate路径+hex按序hash、mmap失败读文件回退、忽略未列文件。

原有 2026-10-13 contract.json 中“下载错误等待新列表”是历史错误，已被 closures 的任意Err重试修复取代，不可引用作当前合同。原有 Compare 对mtime错误的过强等价表述，以及清单to_writer的等价表述也由本报告纠正。

ZIP 失败仍保留前面解出的文件。若已产生CAS目录，紧接的download_blob重试会通过is_dir直接命中这个不完整目录；校验/删坏目录在下次启动trim发生。没有擅自添加原作没有的回滚、额外目录扫描或每次cache-hit校验。

## 验证与界限

新增4个离线合同测试：Date成功→ETag错误的副作用、fetch后活动采时、部分解包后目录cache-hit、严格14天+Windows可表达100ns年龄边界。原11项保留。在本轮前段输入上 `cargo test -p theme-fetcher --offline --locked --lib` 15/15通过；该次运行早于最终log/延迟body/no-body生命周期补全，最终冻结源码测试与strict Clippy由整合owner统一运行，不能用此前输出覆盖最终输入。

所有fixture使用内存响应与临时目录，无主题服务请求。原锁版本reqwest/zip/serde/httpdate/cache_control/blake3/memmap2/arc-swap/log直接复用；标准库/serde派生/公共HTTP和ZIP实现不计游戏手写功能。Transport与run_once是重建测试接口，不宣称原Rust ABI。

完成度为**B库内已定位业务静态覆盖与实现补全，未通过完整原入口oracle**。HTTP body错误、系统文件错误、sender与日志回调的全部真实运行时组合未逐入口执行原机器码；不能说全原行为等价。真实tiny-glade主题plugin/内容consumer属于库外，整合owner负责界定接线；真实服务、A/C与画面未验收。全游戏consumer没有以“这是库外”换成“游戏已经完成”。

证据输入/冻结输出SHA索引见 `manifest.json`。日志/时钟/缓存策略之外的原库内部不再逆向。
