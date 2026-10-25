# feedback-reporter — B fresh 反编译对 Rust 逐函数对照

反编译源：Ghidra `tiny-glade.exe`（B patched），本机 worker shared。
`zip_dir` 实际 VA 0x141002650（Ghidra 基址 0x140000000 + RVA 0x1002650，与 PDB rva=16787024 一致；
PDB 的 va 字段 0x1401002650 偏移 0x01000000，属 PDB 记录误差，RVA 相同）。

## impl$0::build @ 0x1406901f0
| 项 | 原 | Rust | 结果 |
|---|---|---|---|
| 注册 `ReportPlayerFeedbackCmd` 事件 | `register_event<ReportPlayerFeedbackCmd>` | `add_event::<ReportPlayerFeedbackCmd>()` | MATCH |
| 注册 `PlayerFeedbackReportSendingFinished` | `register_event<...Finished>` | `add_event::<...Finished>()` | MATCH |
| 注册 Update 系统 | `add_systems<Update, report_player_feedback>`（含 SchedulesRequested panic） | `add_systems(Update, report_player_feedback)` | MATCH |
| init_resource HTTP 传输 | 无（每次调用新建 reqwest Client） | `init_resource::<FeedbackTransport>()`（可注入边界） | DEVIATION（文档化重建差异，供测试注入；游戏侧接线超范围） |
| 系统参数 | `Local<Option<PendingFeedback>>`（PDB 签名串） | `Local<PendingFeedback>`（report: Option + waited_frames） | MATCH（行为等价，见 report_player_feedback） |

## report_player_feedback @ 0x1406903a0
| 项 | 原 | Rust | 结果 |
|---|---|---|---|
| 事件读入覆盖 Local，waited_frames 重置 0 | 每个事件 clone 进 Local，`*(uint*)(Local+0xd8)=0` | `replace()` 置 report 并 waited_frames=0 | MATCH |
| report None → 返回 | `SBORROW8(0,*Local)` None → return | `report.as_ref()?` | MATCH |
| 就绪判定 | ready_files[0].is_file() && ready_files[1].is_file()（短路） | `ready_files[0].is_file() && ready_files[1].is_file()` | MATCH |
| 等待分支计数 | `waited < 0x259(601)` → +1 并 return | `waited <= 600` → +1 并 None | MATCH |
| 发送分支计数 | take 前也 `+1` | `wrapping_add(1)` 后 take | MATCH |
| 强制释放阈值 | waited==601 时释放（601 次等待帧后） | waited==601 释放 | MATCH |
| 新建 Client + 5s 超时 | `Client::builder().connect_timeout(5).build().unwrap()` | `connect_timeout(CONNECT_TIMEOUT=5s).build().unwrap()` | MATCH |
| REPORT_ENDPOINT | obfstr XOR 反混淆（既证） | `REPORT_ENDPOINT` 常量 | MATCH |
| multipart 顺序 | text(0x142a18918) meta(0x142a1891c) screenshot(0x142a18929) save(0x142a18942) benchmark(0x142a18946) | 按此顺序 push | MATCH（顺序依既有发射面清单；常量名与长度逐字节核验） |
| MIME | screenshot=image/jpg(0x142a18920)、save/benchmark=application/zip(0x142a18933) | `mime_str("image/jpg"/"application/zip")` | MATCH |
| save 打包失败 | `from<ZipError>` → 发送整体失败 | `zip_dir(...)?` 传播 | MATCH |
| benchmark 打包失败 | log "Failed zipping benchmark data: {e}" + 省略附件 | `log::error!("Failed zipping benchmark data: {error}")` + 省略 | MATCH |
| send 失败 | log "Error sending feedback report: {e}" + Err("Connection failed") | 同 | MATCH |
| 2xx | 丢弃响应、不读 body、Ok | `(200..300)` → `Ok(String::new())` | MATCH |
| 非 2xx body | `Response::text()`；失败 → log "Error while receiving feedback service response: {e}" + Err("Error while waiting for a response")；成功 → Err(正文) | `response.body` Err/Ok 分支同文案 | MATCH |
| 结束事件 | 成功 log "Feedback successfully sent" + error=None；失败 log "Error sending feedback: {e}" + error=Some | 同 | MATCH |

## zip_dir<Cursor<Vec<u8>>> @ 0x141002650
| 项 | 原 | Rust | 结果 |
|---|---|---|---|
| 前缀 | root_dir.file_name().unwrap().to_str().unwrap() | 同 | MATCH |
| 遍历保序 | for relative in save.files | 同 | MATCH |
| 缺失跳过 | stat（fs::metadata）后**仅** NotFound（err==2）跳过；其它错误落到 is_file | 旧：`!path.exists()` 跳过一切 metadata 错误 | **FIXED**：改为仅 NotFound 跳过（lib.rs:99-104，注释含地址） |
| 非文件 panic | is_file false → panic "not yet implemented"（@0x142bc8c38） | `panic!("not yet implemented")` | MATCH |
| 日志 | "feedback save: adding file {} as {} ..."（@0x142bc8c68） | `log::trace!("feedback save: adding file {} as {} ...", ...)` | MATCH |
| 0755 权限 | FileOptions unix_permissions 0o755（0x1ed 位域） | `.unix_permissions(0o755)` | MATCH |
| 压缩 | 参数传入（调用方 Deflated，zip 0.6.6 Deflated=8） | `.compression_method(compression)`，调用方传 Deflated | MATCH |
| io 错误传播 | open/read_to_end/write_all 错误返回 Err | `?` 传播 | MATCH |
| 返回类型 | Result<W, ZipError>（PDB 为 void 占位，不权威） | anyhow::Result<W> | 内部差异（文档化，调用端行为一致） |

## zip_benchmark_data（内联于 report_player_feedback）
| 项 | 原 | Rust | 结果 |
|---|---|---|---|
| 读 benchmark 失败 | `fs::read` Err → anyhow，log+省略 | `fs::read(&info.benchmark_file)?` → log+省略 | MATCH |
| GPU Ok | `format!("GPU_UTILIZATION: Ok\n{text}")`（@0x142a18a18 起） | 同 | MATCH |
| GPU Err | `format!("GPU_UTILIZATION: Err({error})")`（@0x142a18a68 起） | 同 | MATCH |
| 条目顺序 | benchmark.puffin（@0x142a18a18）→ info.txt（@0x142a18a40 区） | 同 | MATCH |
| 压缩 | Deflated | Deflated | MATCH |
| 后续 start_file/write_all/finish 失败 | anyhow → 省略附件 | `?` → prepare_report log+省略 | MATCH |

## send_report / prepare_report（内联）
- 拆分设计：原版在 send_report 内直接组装 multipart 并发送；还原版拆为 `prepare_report`（组装 parts）+ `ReportTransport::post`（组装 reqwest form）+ `send_report`（响应规则）。四条 HTTP 规则、日志文案、MIME 与顺序均与反编译一致。
- 常量逐字节核验：text/meta/screenshot/save/benchmark（0x142a18918-0x142a1894e）、image/jpg（0x142a18920）、application/zip（0x142a18933）、"Failed zipping benchmark data: "（0x142a18960）、"Error sending feedback report: "（0x142a189a8）、"Connection failed"（0x142a189f0）、"Feedback successfully sent"（0x142a18860）、"Error sending feedback: "（0x142a188a8 区）、"Error while receiving feedback service response: "（0x142a18af9）、"Error while waiting for a response"（0x142a18b58 区）、"not yet implemented"（0x142bc8c38）。
