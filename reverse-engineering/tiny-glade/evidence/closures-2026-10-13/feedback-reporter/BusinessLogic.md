# feedback-reporter — B 业务逻辑描述（逐函数）

来源：`reconstruction/shared-foundation/crates/feedback-reporter/src/lib.rs` + `skeleton-enriched/B/.../lib.rs.enriched.json.gz`。
本次 fresh 反编译：`impl$0::build` @ 0x1406901f0、`report_player_feedback` @ 0x1406903a0、
`zip_dir<Cursor<Vec<u8>>>` @ 0x141002650（PDB RVA 0x1002650 同源）；内联 send_report /
zip_benchmark_data / prepare_report 经 report_player_feedback 调用点核对。

## 数据模型
- `ReportPlayerFeedbackCmd`：text / meta（String）、screenshot（Option<Vec<u8>>）、
  save_files（Option<PlayerFeedbackReportSaveFiles>）、benchmark（Option<FeedbackBenchmarkInfo>）。
- `PlayerFeedbackReportSaveFiles`：root_dir（PathBuf）、files（Vec<RelativePathBuf>）、
  ready_files（[PathBuf;2]，生产者给出的两个“就绪文件”）。
- `FeedbackBenchmarkInfo`：benchmark_file + gpu_utilization_file。
- `PendingFeedback`：report（Option<cmd>）+ waited_frames（u32）；系统参数为 `Local<PendingFeedback>`。

## impl$0::build（Bevy Plugin build）@ 0x1406901f0
- 注册事件 `ReportPlayerFeedbackCmd`；注册事件 `PlayerFeedbackReportSendingFinished`（各带资源已存在检查）。
- `add_systems(Update, report_player_feedback)`（SchedulesRequested 检查 + panic 文案一致）。
- 原版不 init_resource 任何 HTTP 传输；`report_player_feedback` 内每次调用新建 reqwest Client。
  还原版的 `FeedbackTransport` Resource 为可注入 HTTP 边界（测试隔离用），属文档化重建差异。

## report_player_feedback（Update 系统）@ 0x1406903a0
1. 读全部 `ReportPlayerFeedbackCmd` 事件；逐个 clone 进 `Local<PendingFeedback>`，每次重置 waited_frames=0
   （“后写覆盖”语义）。
2. 若 report 为 None → 直接返回。
3. 若 save_files 存在：对 ready_files[0] 与 ready_files[1] 各做一次 `is_file()`（短路）：
   - 两个都就绪 → 走“发送”。
   - 未就绪且 waited_frames < 601（0x259）：waited_frames += 1 并返回（等待）。
   - 未就绪且 waited_frames >= 601：强制释放（发送）。
4. 发送路径：waited_frames 再 +1、report.take()，然后：
   - `Client::builder().connect_timeout(5s).build().unwrap()`（每次新建 Client）。
   - 反混淆 `REPORT_ENDPOINT`（obfstr 常量，既证）。
   - 组 multipart：text、meta、screenshot、save、benchmark；screenshot/save/benchmark 附带 MIME
     （image/jpg / application/zip），text/meta 无显式 MIME。
   - save 附件：调 `zip_dir`；打包失败向上传播（anyhow），发送整体失败。
   - benchmark 附件：内联 `zip_benchmark_data`；失败仅 log + 省略该附件。
   - HTTP 规则：send 失败 → log "Error sending feedback report: {e}" + Err("Connection failed")；
     2xx → 丢弃响应成功；非 2xx → `text()`，失败 → log "Error while receiving feedback service response: {e}"
     + Err("Error while waiting for a response")，成功 → Err(正文)。
5. 结束：成功 log "Feedback successfully sent" 并写 `PlayerFeedbackReportSendingFinished{error:None}`；
   失败 log "Error sending feedback: {e}" 并写 `{error:Some(e.to_string())}`。

## zip_dir<Cursor<Vec<u8>>> @ 0x141002650
- 前缀 = root_dir 的 UTF-8 file_name（`file_name().unwrap().to_str().unwrap()`）。
- 遍历 save.files（保序）：`relative.to_path(&root_dir)` 得磁盘路径；`prefix.join(relative)` 得归档名。
- 存在性判定：对路径先做一次 metadata（stat）；仅当错误 kind == NotFound 时跳过该条目；
  其它错误（含成功）落到 `is_file()`：非文件 → panic "not yet implemented"（常量 @ 0x142bc8c38）。
- 记录日志 "feedback save: adding file {} as {} ..."。
- `start_file(name, FileOptions)`：压缩方式取参数（调用方传 Deflated），unix_permissions=0o755。
- `File::open + read_to_end + write_all`（io 错误传播）；`finish()` 返回 Result<W, ZipError>。

## send_report / prepare_report（内联于 report_player_feedback）
- 附件组装：text、meta（String→Part）、screenshot（bytes+image/jpg）、
  save（zip_dir→bytes+application/zip）、benchmark（zip_benchmark_data→bytes+application/zip）。
- save 打包失败传播；benchmark 打包失败 log "Failed zipping benchmark data: {e}" 并省略。
- HTTP 四条规则与上面一致。

## zip_benchmark_data（内联）
- `fs::read(benchmark_file)` 失败 → anyhow Err（调用方 log 后省略附件）。
- `fs::read_to_string(gpu_utilization_file)`：Ok → `format!("GPU_UTILIZATION: Ok\n{text}")`；
  Err → `format!("GPU_UTILIZATION: Err({error})")`（不传播）。
- ZipWriter：先 `start_file("benchmark.puffin")` 写原始字节，再 `start_file("info.txt")` 写诊断文本；
  Deflated 压缩；finish 返回 bytes。start_file/write_all/finish 失败 → anyhow Err（省略附件）。
