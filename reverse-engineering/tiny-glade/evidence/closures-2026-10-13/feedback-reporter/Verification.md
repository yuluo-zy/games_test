# feedback-reporter — B 验证记录

## 已对照函数（fresh 反编译）
- `feedback_reporter::impl$0::build` @ 0x1406901f0（compiled，callees 核对）
- `feedback_reporter::report_player_feedback` @ 0x1406903a0（compiled，含内联 send_report / zip_benchmark_data / prepare_report 调用点）
- `feedback_reporter::zip_dir<Cursor<Vec<u8>>>` @ 0x141002650（compiled；PDB rva 0x1002650=16787024 一致，PDB va 字段偏移 0x01000000 为记录误差）
- 内联函数经 report_player_feedback 调用点 + rodata 常量逐字节核验：send_report、zip_benchmark_data、prepare_report、send_report::closure$0/1/2

## 发现的差异
| 差异 | 地址/证据 | 状态 |
|---|---|---|
| `zip_dir` 缺失条目跳过条件：Rust 用 `path.exists()`（任何 metadata 错误都跳过）；原版仅 NotFound（Windows err==2）跳过，其它错误落入 `is_file()` 并 panic | 0x141002650 循环：`stat()` 后 `if err != 2` 分支走 is_file/panic，err==2 分支跳过 | **已修复** |
| `build` 额外 `init_resource::<FeedbackTransport>()` | 0x1406901f0 callees 无任何传输初始化（原版每次新建 reqwest Client） | 保留（文档化重建差异：可注入 HTTP 边界，测试隔离用；游戏侧接线超范围） |
| `zip_dir` 返回类型 anyhow::Result vs 原版 Result<W, ZipError> | 反编译返回槽按 Result 打包；PDB tpi 为 void 占位不权威 | 保留（内部差异，调用端行为一致） |

## 修复
- `zip_dir` 缺失跳过语义：`reconstruction/shared-foundation/crates/feedback-reporter/src/lib.rs:100-108`
  （注释含原地址 0x141002650；`fs::metadata` 仅 `ErrorKind::NotFound` 跳过，其它错误落到 `is_file()` panic）。
  原实现为 `if !path.exists() { continue; }`（对非 NotFound 的 metadata 错误也会跳过，与原版分支不符）。

## 测试
- 同步：`rm -rf .../native/crates/feedback-reporter && cp -R reconstruction/.../crates/feedback-reporter .../native/crates/feedback-reporter`
- `cargo test -p feedback-reporter --offline --locked`：9 passed（既有 8 项 + 新增 1 项 `non_notfound_metadata_error_panics_not_skips`，覆盖非 NotFound metadata 错误 → panic 而非跳过）
- `cargo clippy -p feedback-reporter --offline --locked --all-targets -- -D warnings`：clean（曾触发 `collapsible_if`，已按 let-chain 折叠）

## 逐函数结论
- build：注册两事件 + Update 系统 MATCH；init_resource 为文档化重建差异。
- report_player_feedback：等待/替换 601 计数、is_file 双就绪、HTTP 四规则、multipart 顺序与 MIME、日志文案全部 MATCH。
- zip_dir：前缀/保序/0755/Deflated/panic 文案 MATCH；缺失跳过分支已修复。
- zip_benchmark_data：benchmark.puffin→info.txt、GPU_UTILIZATION Ok/Err 前缀、失败省略 MATCH。
- send_report/prepare_report：响应与附件策略 MATCH（拆分设计文档化）。

## 剩余缺口
- 无完整计时证据证明 601 语义（600 等待帧 + 601 释放）在真实运行被触发；由 decompiled 0x259 常量 + Rust 边界测试共同确认。
- screenshot 分支因 `Option<Vec<u8>>` niche 表示与 Ghidra 寄存器复用，单段反编译难逐行追踪；部分名/MIME 常量逐字节核验，顺序依既有发射面清单，不作完整性声明。
- 反编译结果来自本机 Ghidra 只读 DB（B patched）；未做 A/C。
- 无 live HTTP（离线构造/桩传输测试）。
