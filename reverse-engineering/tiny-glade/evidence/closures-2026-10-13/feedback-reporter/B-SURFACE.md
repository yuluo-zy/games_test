# feedback-reporter — B 发射面原函数清单（骨架提取）

来源：`skeleton-enriched/B/crates/feedback-reporter/src/lib.rs.enriched.json.gz`。

## lib.rs（原函数，扣除 serde derive / zip / std）

| 原函数 | RVA | VA | compiled | callers |
|---|---|---|---|---|
| `feedback_reporter::impl$0::build`（Bevy Plugin build） | 6881776 (0x6901F0) | 0x1406901F0 | 是 | — |
| `feedback_reporter::report_player_feedback` | 6882208 (0x6903A0) | 0x1406903A0 | 是 | — |
| `feedback_reporter::zip_dir<Cursor<Vec<u8>>>` | 16787024 (0x1002650) | 0x141002650 | 是 | — |
| `feedback_reporter::send_report` | — | — | 无独立实例（内联） | — |
| `feedback_reporter::zip_benchmark_data` | — | — | 无独立实例（内联） | — |
| `send_report::closure$0/1/2` | — | — | 无 | — |
| `impl$1/2/3::clone` | — | — | derive | 不计 |

## 对照结论（当前 Rust coverage）

Rust `lib.rs`：`FeedbackReporterPlugin`（build）、`report_player_feedback`、`send_report`、
`zip_dir`、`zip_benchmark_data`、`prepare_report`、`selected_save_files`、`ReportPlayerFeedbackCmd`、
`PlayerFeedbackReportSendingFinished`、`PendingFeedback`、`ReportPart`/`PreparedReport`/`ReportResponse`。

库内函数名一一对应；待 fresh 反编译核对 `build`/`report_player_feedback`/`zip_dir` 及内联
send_report/zip_benchmark_data 的字段构造与分支语义。

## 关键数值/边界（差分核对点）

`REPORT_ENDPOINT`、`CONNECT_TIMEOUT=5s`、`WAIT_FOR_FILES_FRAMES=600`（601 强制释放）、
等待/替换 601 计数、选定文件 ZIP（缺文件跳过/目录 panic/root-prefix/0755/Deflated）、
benchmark ZIP（benchmark.puffin + info.txt、GPU_UTILIZATION Ok/Err 前缀、失败省略）、
multipart 顺序 text,meta,screenshot,save,benchmark + MIME 规则、HTTP 规则（2xx 无 body /
`Connection failed` / 非 2xx body / `Error while waiting for a response`）。
