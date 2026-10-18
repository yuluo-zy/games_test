# crates/tiny-glade 崩溃/上报 — RESTORE（restorer-tinyglade-D）

日期：2026-10-17。范围：`crash_pickup/{mod,client,server,minidump_to_sentry,crash_payload}.rs`、
`panic_reporter/{mod,panic_dump,panic_report,panic_screenshot,sentry_reporter,reverse_backtrace}.rs`、
`hang_watchdog.rs`、`integrity_report_main.rs`、`aftermath.rs`。

证据：Ghidra 持久 B 库 `tiny-glade.exe`（SHA256 `cf048e27…6323`），
`--worker shared` **只读**；每函数 fresh `get_functions` + `disassemble_function` +
`read_memory`。原始结果/反汇编落盘：
`recovery/agent-team/work/restorer-tinyglade-D/`（`result1/2.json`、`disasm_*.txt`、
`decomp/*.c`）。所有 VA 为 B。判等按 `planning/2026-10-16-logic-verify-protocol.md`
与 8 条分歧检查表逐条对照。

## 三态回执

| 类别 | 数量 | 函数 |
|---|---|---|
| 判等 | 13 | `ClientHandle::send_message`、`spawn_hang_watchdog_thread`、`dump_log_file`、`dump_settings_file`、`dump_gpu_crash_dump_file`、`dump_rhapsody_log`、`create_minidump_file`、`on_message`、`panic_info_to_message`、`create_sentry_attachment`、`shader_debug_info`、`dump_screenshot`、`crash_payload` serde 字段平面 |
| 重建（已还原主体，登记缺口） | 14 | `spawn_server`、`client::start`、`server::run`、`crash_trace_to_sentry_event`、`setup_panic_hook`、hook closure、`recover_game_state_after_crash`、`capture_image`、`report_crash_event_to_sentry`、`hang_watchdog`、`verify_package_integrity_and_show_report`、`NvAftermathDelegate::default`、`NvAftermathDelegate::dumped`、`ReverseBacktrace::fmt` |
| 胶水-legitimate | 5 | `create_graphics_resources`（设备 buffer 创建/存放）、`on_client_connected`/`on_client_disconnected`、`NvAftermathDelegate::description`（ICF 空体）、`send_message_impl`/`reader thread`/`CrashHandler::attach` 传输层 |
| 未实现-需重做 | 7 | `dump_inputs`、`dump_save`、`dump_profile_file`、`on_game_crashed`、`minidump_to_crash_trace`、`on_minidump_created`、`panic_info_to_sentry_event`（结构面） |

`cargo check -p tiny-glade`：**本组文件 0 error**。验证时整 crate 仍被并行 agent 的在途文件阻断：先前的 tiny-glade 检查只报
`app_config.rs`（`GraphicsInstanceLayerAllowlist`）、`logging.rs`
（`colored::control::set_virtual_terminal`）、`icc.rs`（私有 `MonitorHandle`）、
`egui_backend.rs`（`ClippedShape`）与 `JpegEncoder::write_image`；最终一轮改由
上游 `crates/tg-text/src/fragment.rs`（`cache_token`/`?` on Option）阻断，尚未进入
tiny-glade。以上均不在本组范围、未改动；本组文件在最后一次到达 tiny-glade 的检查中
0 error。
`cargo test -p tiny-glade --lib`：**11 passed / 0 failed**（含新增
`panic_screenshot::tests::{reduction_matches_16_by_9, yuy2_group_yields_two_rgb_pixels}`；
该次运行早于并行 agent 的后续破坏）。

## 逐函数表

| 文件 | 函数 | VA | verdict | 证据 / 分歧点 |
|---|---|---|---|---|
| `crash_pickup/mod.rs` | `ClientHandle::send_message` | 0x14015ac60 | 判等 | `serde_json::to_vec(...).unwrap()`：失败跳 `core::result::unwrap_failed`，消息为通用 `0x14295b9d0`（len 0x2b）；成功走 `minidumper::Client::send_message_impl`（胶水）。已把 `expect` 改回 `unwrap` |
| `crash_pickup/mod.rs` | `CrashReporter::spawn_server` | 0x14015aeb0 | 重建 | socket = `temp_dir().join(Uuid::new_v4().to_string())`；`Command::new(current_exe()).arg("--crash-reporter-server").arg(socket).arg("--crash-reporter-send-to-sentry").arg("true"/"false")`（字面量 0x14295bc50/0x14295bc67/0x14295ba65/0x14295ba60 逐字节解码）；非 UTF-8 错误串 0x14295bc2f。传输/Child 保存为构造面缺口 |
| `crash_pickup/client.rs` | `client::start` | 0x14048a170 | 重建 | 重试次序按汇编改正：失败 → `waited += 50ms` → `timeout <= waited` 判 Err → `sleep(50ms)` → 再连（0x14048a1de/1e2/1e5/1ed）。reader 线程 + `crash_handler::attach` 为 SDK 胶水 |
| `crash_pickup/server.rs` | `server::run` | 0x1402aee80 | 重建 | 两个 `Arc<Mutex<…>>`（`Option<ResolvedCrash>`、`CrashMiscInfo`）+ `Server::with_name` + `run` + trace 日志 `"Shutting down the crash server."` + `Option::expect`。`CrashMiscInfo`/`ResolvedCrash`/`CrashPayload` 运行时类型未建模（见 crash_payload 注释） |
| `crash_pickup/server.rs` | `create_minidump_file` | 0x1402ad880 | 判等 | `create_dir_all(dir)` → `format!("{}.dmp", Uuid::new_v4())`（pieces `["", ".dmp"]` 0x1429ad1e0/0x1429ad1d8）→ join → `OpenOptions` write/create/truncate → `Result<(PathBuf, File)>` |
| `crash_pickup/server.rs` | `on_message` | 0x1402ae1f0 | 判等 | `from_slice::<CrashPickupMessage>` 失败 **unwrap panic**（0x1402ae959 跳 unwrap_failed，消息 0x1429acfd0 `"crash pickup message"`）。已由“记录后继续”改为 `.expect` |
| `crash_pickup/server.rs` | `on_minidump_created` | 0x1402adae0 | 未实现 | 原版 `OpenOptions::open(path).unwrap()` + `read_to_end`，把 `MinidumpBinary` 写入 `ResolvedCrash.payload`（+0x200，含 file handle +0x238）；当前仅记录路径/回传。分歧：状态结构与载荷平面未接 |
| `crash_pickup/minidump_to_sentry.rs` | `minidump_to_crash_trace` | 0x1404b2fb0 | 未实现 | 调用链：`Minidump::read_path` → SystemInfo/MiscInfo/ModuleList/Exception → `exception.context()` → memory list → `get_stack_pointer` → `memory_at_address` → `minidump_unwind` → `get_crash_reason` → `Stacktrace::from_frames_reversed`。阻断：workspace 缺 `minidump-unwind`；HeaderOnlyParser 明示非原逻辑 |
| 同上 | `crash_trace_to_sentry_event` | 0x1404b17d0 | 重建 | 两次 `Event::default()`、异常类型字面量 `"crash"`（0x73617263+0x68）、modules→breadcrumbs。分歧：事件按 exception/stacktrace 结构构造，当前扁平成 message/frames |
| `crash_pickup/crash_payload.rs` | serde 字段平面 | 0x14003e250… | 判等 | 字段名/变体名保持 B `.rdata` 逐项。`CrashRecovered.breadcrumbs` 已按原类型改为 `sentry_types::protocol::v7::Breadcrumb` |
| `panic_reporter/mod.rs` | `setup_panic_hook` | 0x1400db5b0 | 重建 | `take_hook`→`Box`→`set_hook`；闭包 env 4 字 |
| 同上 | hook closure | 0x1400db6b0 | 重建 | 逐控制流：全局互斥字节 0x143634b78；双 panic 标志 0x143634b79 → `eprintln("Panic mutex poisoned! 😱 Aborting.\n")` + `__fastfail(7)`；已报告标志 0x143634b7a → 释放锁并尾调旧 hook；否则 `backtrace::Backtrace::new()` → error 日志 `"Reverse backtrace:\n{…}"` 与 `"PANIC: {…}"`（target `tiny_glade::panic_reporter`）→ `recover_game_state_after_crash()` → 有 client 时 `panic_info_to_message().to_owned()` 组 `CrashPickupMessage::Panic{BacktracePayload{backtrace,message}}` 发送并 `.expect("send message to crash handler")` → `abort()`。分歧：`panic_count` 以 `std::thread::panicking()` 等价近似；互斥用自旋 |
| 同上 | `recover_game_state_after_crash` | 0x1400dbc30 | 重建 | 目录 `tmp/panics/panic_{ts}`（格式串 0x14293aa80 区）、context `"Creating the bug report directory"`（0x14293aab8）、8 个采集器顺序 save/screenshot/rhapsody/inputs/log/settings/profile/gpu、错误→breadcrumb、`SessionId::save_folder_path`/`backup_folder_path`。分歧：breadcrumb 的 category 字面量未取证 |
| `panic_reporter/panic_dump.rs` | `dump_log_file` | 0x1403acc70 | 判等 | `tmp/log.txt` → `report/<log.txt>`，context `"Copying the log file"`，成功返回目标 PathBuf |
| 同上 | `dump_inputs` | 0x1403ace70 | 未实现 | 门：INFO tag==2→`"Game not yet initialized"`；mode@+0x40（`InputRecordReplayMode` niche 0x8000…+n）：0/1 序列化并写 `inputs.json`，1 再返回 `Err("Inputs saved to {path}")`，其它→`"Input record replay state was not InterHistoryRecord."`。阻断：`country-core InputStream` 缺 +0x40 mode 槽与 `Serialize(sync_points/frames)` |
| 同上 | `dump_save` | 0x1403ad640 | 未实现 | INFO tag==2 先返回；**先** `OpenOptions` 建 `save.json`，再判 history 空（`"History is empty"`）；调用链 `HistoryIdentity::generate`→`MainCamera::store_parameters`→`HistoryEditChain::iter`→`iter_resolved_snapshots`→`GladeColorCustomization::default`→`save_game_to_file`。阻断：后三者未在 country-core 恢复（已按原顺序补建文件与门） |
| 同上 | `dump_settings_file` | 0x1403ae050 | 判等 | `config/settings.ron` → `report/<file_name>`；`file_name().unwrap()`；context `"Copying the settings file"` |
| 同上 | `dump_profile_file` | 0x1403ae2f0 | 未实现 | 需要 `country_core::game_profile::profile_file_path`（未恢复）；context `"Copying the profile file"` |
| 同上 | `dump_gpu_crash_dump_file` | 0x1403ae570 | 判等 | `tmp/gpu-crash-dump.zip` → report，context `"Copying the GPU crash dump file"` |
| 同上 | `dump_rhapsody_log` | 0x1403ae770 | 判等 | `rhapsody::dump_log` 返回 Vec<u8> → `std::fs::write(report/render.log, …)`，context `"Writing rhapsody log"`（0x1429cbb5b，len 20）。已从 `File::create+write` 改为 `fs::write(dump_log_vec())` |
| `panic_reporter/panic_report.rs` | `on_game_crashed` | 0x1403ae940 | 未实现 | 56KB 内联编排。fresh 顺序：payload 检查(+0x1e8) → 字符串 clone → `TempDir::new` → `canonicalize` + `format_err` → `"Error creating a crash report folder"` → `rename`/`copy` 回退 → `load_last_used_locale` → 逐文件 copy + `date_time_string_for_filename` → `minidump_to_crash_trace` → `verify_package_manifest`。路径常量/打包结构未闭合 |
| `panic_reporter/panic_screenshot.rs` | `create_graphics_resources` | 0x1405898a0 | 胶水 | `BufferDesc::new_cpu_to_gpu` + `create_mapped_buffer`；成功存全局 Option，旧值非空 → panic；失败记录 |
| 同上 | `capture_image` | 0x140589c90 | 重建 | f32 `sqrt(230400/(w/h))`→trunc→饱和 u32::MAX→min 1；`230400/tw` 后 `&0x3fffe`；掩码高度为 0 时跳过 dispatch。compute pass 绑定/dispatch 为 glue |
| 同上 | `dump_screenshot` | 0x14058a060 | 判等 | 尺寸非零 / `w*h <= 0x38400`；两条文案逐字节：`"…Dimensions can't be zero."`、`"…Would be too many bytes."`；YUY2(4B=2px) → BT.709 full-range RGB8（系数 1.5748/1.8556/0.1873/0.4681，×255+0.5、clamp、截断）；父目录 + `save_buffer`，context `"saving image"` |
| `panic_reporter/sentry_reporter.rs` | `panic_info_to_message` | 0x1404b54c0 | 判等 | payload `TypeId` 对 `&str`/`String` 分支返回 (ptr,len)，否则 `"Box<Any>"`（0x1429e90a5, 8B）；已由 `info.to_string()` 改为 payload downcast |
| 同上 | `panic_info_to_sentry_event` | 0x1404b5550 | 未实现 | 大体内联：`Event::default()`、`"panic"` 异常类型、`sentry_backtrace::process::backtrace_to_stacktrace`、level 4(Fatal)、Exception/Mechanism/Stacktrace 逐字段写入。当前模型扁平化（返回结构不符），tags/fingerprint 键未取证 |
| 同上 | `create_sentry_attachment` | 0x1404b4d50 | 判等 | 5 个错误串（`No attachment file name`/`Corrupt attachment file name`/`No attachment file name extension`/`Corrupt attachment extension`/`Unrecognized attachment file name extension: {}`）；content-type：json→application/json、jpg/jpeg→image/jpeg、txt/log/ron→text/plain、dmp→application/binary、zip→application/zip；累计 `total + len >= 0x1400001` → `"Attachment limit reached."`；已整体重写 |
| 同上 | `report_crash_event_to_sentry` | 0x1404b3e30 | 重建 | 9 元组 `(name, Result<PathBuf,String>)`：Minidump/GPU crash dump/Rhapsody log/Inputs/Settings/Profile/Screenshot/Log/Save（名字 0x1429e8f07…0x1429e8f3e 逐字节）；共享累计字节数；`Vec::retain` 后 `sentry::with_scope(uuid)`。scope/transport 为 SDK 胶水 |
| `panic_reporter/reverse_backtrace.rs` | `ReverseBacktrace::fmt` | 0x1402cfc30 | 重建 | 纯格式化适配器（frame 逆序）；`backtrace::Backtrace`（crate）由 `Backtrace::new()` 产生；Debug 由并行 agent 补上以匹配 `{:?}` 日志 |
| `hang_watchdog.rs` | `hang_watchdog` | 0x140365e20 | 重建 | 20s 周期/1s 轮询/`TICK_IDX`（i32）同 tick → error `"The frame index has not changed in {:?}"` + info `"🔥 Triggering an intentional crash..."` + `abort`；`parking_lot::deadlock::check_deadlock()` 非空 → `"Deadlock detected"` + `"🛑 Deadlock :{}"` + `"Thread {} backtrace:\n{:?}"` + 同 abort。`thread_priority` 缺依赖 → 置零 glue（两条优先级日志未发） |
| 同上 | `spawn_hang_watchdog_thread` | 0x140365be0 | 判等 | `Builder::new().name("hang watchdog")`（`"hang wat"`+`"chdog"` 0x74617720676e6168/0x676f646863746177）`.spawn` |
| `integrity_report_main.rs` | `verify_package_integrity_and_show_report` | 0x1402552b0 | 重建 | `Instant::now`→`verify_package_manifest`；Ok 且 info 级 → `"Package manifest check passed. Took {:?}"`；错误分支 `"Could not read {}: {}"` / `"Incorrect files"` / `"Extra files"`；**无条件**建 EventLoop 并 `run_integrity_report_gui`。GUI crate 已删除 → trait seam；错误变体参数文案未逐项取证 |
| `aftermath.rs` | `NvAftermathDelegate::default` | 0x140244460 | 重建 | `tmp/gpu-crash-dump.zip` 存在则 unlink；失败日志文案未在 listing 中解码（现为登记近似） |
| 同上 | `dumped` | 0x140244790 | 重建 | info `"NV Aftermath dump data received."`；内存 zip（`Cursor<Vec<u8>>`）；debug-info 条目名 `shader-nvdbg/{:x}-{:x}.nvdbg`（pieces 0x1429a070d…）；主 dump `nv-aftermath.nv-gpudmp`；context `"writing debug info data"`/`"writing dump data"`；`fs::write(tmp/gpu-crash-dump.zip)`；info `"NV Aftermath dump file written."`；错误日志 `"Error writing NV Aftermath dump file: {…}"` 后吞掉 |
| 同上 | `shader_debug_info` | 0x140245340 | 判等 | trace 日志 `"Got NV shader debug info! Ident: {:?}"` + 加锁 `HashMap::insert` |
| 同上 | `description` | 0x14003bb80 | 胶水 | 单字节 RET（ICF 空体） |

## 本轮修正的关键分歧（8 条检查表命中）

1. **panic-vs-return 反**：`on_message` 由“记录后继续”改为 `.expect("crash pickup message")`；
   `send_message` 由 `expect(...)` 改为通用 `.unwrap()`。
2. **门控次序**：`client::start` 超时判断次序改为“先加 50ms 再判”；`dump_save`
   改为先创建 `save.json` 再判 history 空。
3. **载荷/结构面**：`create_sentry_attachment` 完整重写（错误串、扩展名表、20MiB 上限、
   共享累计字节）；`recover_game_state_after_crash` 由 `Result<()>` 改为返回
   `CrashRecovered`（0x1e8B，字段平面与原 serde 对齐）。
4. **callee 转发错**：`dump_rhapsody_log` 改回 `rhapsody::dump_log_vec()+fs::write`。
5. **返回结构不符**：`panic_info_to_message` 改回 payload downcast（`&str`/`String`/`Box<Any>`）；
   `report_crash_event_to_sentry` 改为 9 个命名字段而非 `read_dir` 收集。
6. **截断/取值语义**：`dump_screenshot` 由 RGBA8 改为 YUY2→RGB8（BT.709 full-range）。
7. 未发现自造事件/命令；未发现条件极性反向（另见未实现项）。

## 阻断（其他 owner，未越线修改）

1. `country-core::input::input_stream::InputStream`：缺 `+0x40 InputRecordReplayMode` 与
   `Serialize(sync_points/frames)` → `dump_inputs`。
2. `country-core::resources::history`：`HistoryEditChain::iter`、
   `History::iter_resolved_snapshots`、`save_game_to_file`（0x140241ca0）缺失 → `dump_save`。
3. `country_core::game_profile::profile_file_path` 缺失 → `dump_profile_file`。
4. workspace 缺 `minidump-unwind` → `minidump_to_crash_trace`；缺 `thread_priority` →
   watchdog 优先级设置。
5. `minidumper`/`crash-handler`/`sentry` git+registry 依赖身份已在 workspace 落库，但
   `tiny-glade` 未启用其真实类型；传输层以 trait 缝保留。
6. `on_game_crashed` 的路径常量/打包结构与 `panic_info_to_sentry_event` 的
   exception/tag/fingerprint 键仍需下一轮 ReVa 定点取证。

## 并发协作（同 crate 在途）

- 并行 `restorer-tinyglade-E` 在本组文件上做了三处增量，已保留并复核：
  `reverse_backtrace.rs` 增加 `Debug` 转发（panic hook 以 `{:?}` 记录）；
  `crash_payload.rs` 将 `CrashRecovered.breadcrumbs` 改为原类型
  `sentry_types::protocol::v7::Breadcrumb`（本组据此同步 `recover_game_state_after_crash`
  与 `report_crash_event_to_sentry`）；`aftermath.rs` 拆出显式 `Default` 并把
  deadlock `check_deadlock()` 按 parking_lot 0.12.1 的
  `Vec<Vec<DeadlockedThread>>` 返回值修正（已对照 `parking_lot_core-0.9.3` 源确认）。
- 最终 `cargo check` 失败点会随并行任务移动（最近一次为 `tg-text`），与本组文件无关。

## 红线

- Ghidra/ReVa 全部 `--worker shared` 只读，未开 `--write`、未改数据库。
- 只改 `crates/tiny-glade/src/` 下本组文件与 `crates/tiny-glade/Cargo.toml`
  （`parking_lot` 增加 `deadlock_detection`，B 原版使用
  `parking_lot_core::deadlock_impl::check_deadlock`）。
- 未改 `country-core`/`rhapsody`/其他模块；共享 helper 缺失项只登记。
