# crates/feedback-reporter/src

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

B等待/替换/601计数、选定文件ZIP、benchmark、multipart/HTTP失败规则与Bevy Update事件插件。

## 接线与验证

接线：shared_foundation_workspace_and_library_adapters_original_game_producer_consumer_pending。

共享 --offline --locked Cargo测试及all-targets Clippy -D warnings通过；原叶/静态/字段与真实后端范围分别见证据，不接受完整原模块。

## 已恢复的函数和类型范围

- text/meta与可选screenshot/save/benchmark按原顺序/MIME组织
- 只打包指定相对文件；missing跳过，目录原panic；root文件名前缀/0755/Deflated
- benchmark.puffin/info.txt固定名；失败非致命省略附件
- connect_timeout5秒，发送失败/非2xx body/读失败规则；真实reqwest adapter
- Bevy command→Local pending替换/两文件或计数>=601→SendingFinished；保留注入provider

| 原函数 | Rust接口 | 版本与限定范围 | 状态与证据 |
|---|---|---|---|
| feedback_reporter::zip_dir | [reconstruction/shared-foundation/crates/feedback-reporter/src/lib.rs](../../reconstruction/shared-foundation/crates/feedback-reporter/src/lib.rs)：`zip_dir` | B：原0x141002650字节/控制流与选定文件规则；候选测试，不含完整原ZIP入口oracle | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/feedback-cloud/delivery.json](../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/delivery.json), [evidence/foundation-unstarted-2026-10-13/feedback-cloud/validation.json](../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/validation.json) |
| feedback_reporter::impl$0::build | [reconstruction/shared-foundation/crates/feedback-reporter/src/lib.rs](../../reconstruction/shared-foundation/crates/feedback-reporter/src/lib.rs)：`build` | B：原0x1406901f0 Bevy Update注册与Local pending；重建Resource provider为接入glue | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/feedback-cloud/delivery.json](../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/delivery.json), [evidence/foundation-unstarted-2026-10-13/feedback-cloud/validation.json](../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/validation.json) |
| feedback_reporter::report_player_feedback | [reconstruction/shared-foundation/crates/feedback-reporter/src/lib.rs](../../reconstruction/shared-foundation/crates/feedback-reporter/src/lib.rs)：`report_player_feedback` | B：原0x1406903a0；48例仅等待分支对照，打包/发送静态与候选测试，完整入口未oracle | candidate_business_rule_mapping：[evidence/foundation-unstarted-2026-10-13/feedback-cloud/delivery.json](../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/delivery.json), [evidence/foundation-unstarted-2026-10-13/feedback-cloud/validation.json](../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/validation.json) |

## 明确缺口

- 公开字段与两ready路径语义原名/ABI
- 完整原report/zip入口oracle
- country-core/UI实际反馈producer和A/C适配

下一闭包：公开字段与两ready路径语义原名/ABI；完整原report/zip入口oracle；country-core/UI实际反馈producer和A/C适配

## 证据

- [evidence/foundation-unstarted-2026-10-13/feedback-cloud/delivery.json](../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/delivery.json)
- [evidence/foundation-unstarted-2026-10-13/feedback-cloud/validation.json](../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/validation.json)
- [evidence/foundation-unstarted-2026-10-13/integration/dependency-validation.json](../../evidence/foundation-unstarted-2026-10-13/integration/dependency-validation.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/feedback-reporter/src/lib.rs` | A,B,C | [reconstruction/shared-foundation/crates/feedback-reporter/src/lib.rs](../../reconstruction/shared-foundation/crates/feedback-reporter/src/lib.rs)：B_PDB_original_source_and_scoped_business_rules_candidate | feedback_reporter::zip_dir（candidate_business_rule_mapping；业务等价以限定证据为准）<br>feedback_reporter::impl$0::build（candidate_business_rule_mapping；业务等价以限定证据为准）<br>feedback_reporter::report_player_feedback（candidate_business_rule_mapping；业务等价以限定证据为准） | candidate_present；B等待/替换/601计数、选定文件ZIP、benchmark、multipart/HTTP失败规则与Bevy Update事件插件。：[evidence/foundation-unstarted-2026-10-13/feedback-cloud/delivery.json](../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/delivery.json), [evidence/foundation-unstarted-2026-10-13/feedback-cloud/validation.json](../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/validation.json)<br>feedback_reporter::zip_dir：原0x141002650字节/控制流与选定文件规则；候选测试，不含完整原ZIP入口oracle：[evidence/foundation-unstarted-2026-10-13/feedback-cloud/delivery.json](../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/delivery.json), [evidence/foundation-unstarted-2026-10-13/feedback-cloud/validation.json](../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/validation.json)<br>feedback_reporter::impl$0::build：原0x1406901f0 Bevy Update注册与Local pending；重建Resource provider为接入glue：[evidence/foundation-unstarted-2026-10-13/feedback-cloud/delivery.json](../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/delivery.json), [evidence/foundation-unstarted-2026-10-13/feedback-cloud/validation.json](../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/validation.json)<br>feedback_reporter::report_player_feedback：原0x1406903a0；48例仅等待分支对照，打包/发送静态与候选测试，完整入口未oracle：[evidence/foundation-unstarted-2026-10-13/feedback-cloud/delivery.json](../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/delivery.json), [evidence/foundation-unstarted-2026-10-13/feedback-cloud/validation.json](../../evidence/foundation-unstarted-2026-10-13/feedback-cloud/validation.json) |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/shared-foundation/crates/feedback-reporter/src/lib.rs

[reconstruction/shared-foundation/crates/feedback-reporter/src/lib.rs](../../reconstruction/shared-foundation/crates/feedback-reporter/src/lib.rs)

SHA256：`8571398f74ea0a26e52c026cab1f4452ced629b563129e227d95bc291fcb13e5`；缺口注释标记数：1。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `replace` | 59 | 未由扫描证明；以证据限定范围为准 |
| `take_ready` | 67 | 未由扫描证明；以证据限定范围为准 |
| `zip_dir` | 86 | 未由扫描证明；以证据限定范围为准 |
| `zip_benchmark_data` | 123 | 未由扫描证明；以证据限定范围为准 |
| `prepare_report` | 153 | 未由扫描证明；以证据限定范围为准 |
| `post` | 207 | 未由扫描证明；以证据限定范围为准 |
| `default` | 214 | 未由扫描证明；以证据限定范围为准 |
| `post` | 229 | 未由扫描证明；以证据限定范围为准 |
| `send_report` | 261 | 未由扫描证明；以证据限定范围为准 |
| `build` | 285 | 未由扫描证明；以证据限定范围为准 |
| `report_player_feedback` | 293 | 未由扫描证明；以证据限定范围为准 |
| `selected_save_files` | 319 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L28：/// author field names are unknown; callers supply the producer's two paths.
